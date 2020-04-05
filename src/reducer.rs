use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};
use futures::ready;
use futures::Stream;
use pin_utils::{unsafe_pinned, unsafe_unpinned};

#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct Reducer<S, T, F, Fut> {
    stream: S,
    f: F,
    accum: Option<T>,
    future: Option<Fut>,
}

impl<S, T, F, Fut> Reducer<S, T, F, Fut>
where
    S: Stream,
    F: FnMut(T, S::Item) -> Fut,
    Fut: Future<Output = T>,
{
    unsafe_pinned!(stream: S);
    unsafe_unpinned!(f: F);
    unsafe_unpinned!(accum: Option<T>);
    unsafe_pinned!(future: Option<Fut>);

    pub(super) fn new(stream: S, f: F) -> Self {
        Self {
            stream,
            f,
            accum: None,
            future: None,
        }
    }
}

impl<S, T, F, Fut> Future for Reducer<S, T, F, Fut>
where
    S: Stream<Item = T>,
    F: FnMut(T, S::Item) -> Fut,
    Fut: Future<Output = T>,
{
    type Output = Option<T>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {
            if self.accum.is_none() {
                if self.future.is_none() {
                    let first = ready!(self.as_mut().stream().poll_next(cx));
                    if first.is_none() {
                        return Poll::Ready(None);
                    }
                    *self.as_mut().accum() = first;
                } else {
                    let accum = ready!(self.as_mut().future().as_pin_mut().unwrap().poll(cx));
                    *self.as_mut().accum() = Some(accum);
                    self.as_mut().future().set(None);
                }
            }

            let item = ready!(self.as_mut().stream().poll_next(cx));
            let accum = self
                .as_mut()
                .accum()
                .take()
                .expect("Reducer polled after completion");

            if let Some(e) = item {
                let future = (self.as_mut().f())(accum, e);
                self.as_mut().future().set(Some(future));
            } else {
                return Poll::Ready(Some(accum));
            }
        }
    }
}
