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
    {
        assert_future::<Self::Item, E, _>(map_err::new(self, f))
    }
}

fn assert_future<A, B, F>(t: F) -> F
where
    F: Future<Item = A, Error = B>,
{
    t
}
