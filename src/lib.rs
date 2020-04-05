#![no_std]

use core::future::Future;
use futures::Stream;

mod reducer;
pub use reducer::Reducer;

pub trait Reduce<T>: Stream {
    fn reduce<F, Fut>(self, f: F) -> Reducer<Self, T, F, Fut>
    where
        Self: Sized,
        F: FnMut(T, Self::Item) -> Fut,
        Fut: Future<Output = T>,
    {
        Reducer::new(self, f)
    }
}

impl<S, T> Reduce<T> for S where S: Stream<Item = T> {}
