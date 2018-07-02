use chain::Chain;
use {Future, IntoFuture, Poll};

#[must_use = "futures do nothing unless polled"]
pub struct Then<A, B, F>
where
    A: Future,
    B: IntoFuture,
{
    state: Chain<A, B::Future, F>,
}

pub fn new<A, B, F>(future: A, f: F) -> Then<A, B, F>
where
    A: Future,
    B: IntoFuture,
{
    Then {
        state: Chain::new(future, f),
    }
}

impl<A, B, F> Future for Then<A, B, F>
where
    A: Future,
    B: IntoFuture,
    F: FnOnce(Result<A::Item, A::Error>) -> B,
{
    type Item = B::Item;
    type Error = B::Error;

    fn poll(&mut self) -> Poll<B::Item, B::Error> {
        self.state.poll(|a, f| Ok(Err(f(a).into_future())))
    }
}
