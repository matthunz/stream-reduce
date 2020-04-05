//! This crate gives Streams a `reduce` function that is similar to
//! [`fold`](https://docs.rs/futures/0.3.4/futures/stream/trait.StreamExt.html#method.fold)
//! but without an initial value. The function returns a [`Future`](core::future::Future)
//! containing `None` if the stream is empty and `Some(value)` otherwise.
//!
//! Based on David Tolnay's [`reduce`](https://docs.rs/reduce/0.1.2/reduce/) crate for iterators.
//! 
//! # Examples
//! ```
//! use stream_reduce::Reduce;
//! use futures::stream;
//!
//! # futures::executor::block_on(
//! async {
//!     // Reduce a non-empty stream into Some(value)
//!     let v = vec![1usize, 2, 3, 4, 5];
//!     let sum = stream::iter(v).reduce(|a, b| async move { a + b }).await;
//!     assert_eq!(Some(15), sum);
//!
//!     // Reduce an empty stream into None
//!     let v = Vec::<usize>::new();
//!     let product = stream::iter(v).reduce(|a, b| async move { a * b }).await;
//!     assert_eq!(None, product);
//! }
//! # )
//! ```

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
