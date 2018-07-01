//! futures-rs copy
#![no_std]

#[macro_use]
extern crate log;

pub mod poll;
use poll::{Async, Poll};

macro_rules! if_std {
    ($($i:item)*) => ($(
        #[cfg(future = "use_std")]
        $i
    )*)
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
}
