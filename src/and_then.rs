use chain::Chain;
use {Future, IntoFuture, Poll};

pub struct AndThen<A, B, F>
where
    A: Future,
    B: IntoFuture,
{
    state: Chain<A, B::Future, F>,
}

pub fn new<A, B, F>(future: A, f: F) -> AndThen<A, B, F>
where
    A: Future,
    B: IntoFuture,
{
    AndThen {
        state: Chain::new(future, f),
    }
}

impl<A, B, F> Future for AndThen<A, B, F>
where
    A: Future,
    B: IntoFuture<Error = A::Error>,
    F: FnOnce(A::Item) -> B,
{
    type Item = B::Item;
    type Error = B::Error;

    fn poll(&mut self) -> Poll<B::Item, B::Error> {
        self.state
            .poll(|result, f| result.map(|e| Err(f(e).into_future())))
    }
}
