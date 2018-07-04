//! futures-rs copy
#![no_std]

#[cfg(feature = "use_std")]
extern crate std;

#[macro_use]
extern crate log;

pub mod poll;
use poll::{Async, Poll};

mod map;
use map::Map;

mod map_err;
use map_err::MapErr;

mod chain;

mod then;
use then::Then;

mod and_then;
use and_then::AndThen;

mod or_else;
use or_else::OrElse;

macro_rules! if_std {
    ($($i:item)*) => ($(
            #[cfg(feature = "use_std")]
            $i
            )*)
}

if_std!{
    pub type BoxFuture<T, E> = std::boxed::Box<Future<Item = T, Error= E> + Send>;

    impl <F: ?Sized + Future> Future for std::boxed::Box<F> {
        type Item = F::Item;
        type Error = F::Error;

        fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
            (**self).poll()
        }
    }
}

pub trait Future {
    type Item;
    type Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error>;

    fn wait(self) -> Result<Self::Item, Self::Error>
    where
        Self: Sized,
    {
        panic!("Not implet yet");
    }

    #[cfg(feature = "use_std")]
    fn boxed(self) -> BoxFuture<Self::Item, Self::Error>
    where
        Self: Sized + Send + 'static,
    {
        ::std::boxed::Box::new(self)
    }

    fn map<F, U>(self, f: F) -> Map<Self, F>
    where
        F: FnOnce(Self::Item) -> U,
        Self: Sized,
    {
        assert_future::<U, Self::Error, _>(map::new(self, f))
    }

    fn map_err<F, E>(self, f: F) -> MapErr<Self, F>
    where
        F: FnOnce(Self::Error) -> E,
        Self: Sized,
    {
        assert_future::<Self::Item, E, _>(map_err::new(self, f))
    }

    fn then<F, B>(self, f: F) -> Then<Self, B, F>
    where
        F: FnOnce(Result<Self::Item, Self::Error>) -> B,
        B: IntoFuture,
        Self: Sized,
    {
        assert_future::<B::Item, B::Error, _>(then::new(self, f))
    }

    fn and_then<F, B>(self, f: F) -> AndThen<Self, B, F>
    where
        F: FnOnce(Self::Item) -> B,
        B: IntoFuture<Error = Self::Error>,
        Self: Sized,
    {
        assert_future::<B::Item, Self::Error, _>(and_then::new(self, f))
    }

    fn or_else<F, B>(self, f: F) -> OrElse<Self, B, F>
    where
        F: FnOnce(Self::Error) -> B,
        B: IntoFuture<Item = Self::Item>,
        Self: Sized,
    {
        assert_future::<Self::Item, B::Error, _>(or_else::new(self, f))
    }
}

fn assert_future<A, B, F>(t: F) -> F
where
    F: Future<Item = A, Error = B>,
{
    t
}

pub trait IntoFuture {
    type Future: Future<Item = Self::Item, Error = Self::Error>;
    type Item;
    type Error;

    fn into_future(self) -> Self::Future;
}
