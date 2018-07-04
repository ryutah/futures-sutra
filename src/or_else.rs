use chain::Chain;
use {Future, IntoFuture, Poll};

pub struct OrElse<A, B, F>
where
    A: Future,
    B: IntoFuture,
{
    state: Chain<A, B::Future, F>,
}

pub fn new<A, B, F>(future: A, f: F) -> OrElse<A, B, F>
where
    A: Future,
    B: IntoFuture<Item = A::Item>,
{
    OrElse {
        state: Chain::new(future, f),
    }
}

impl<A, B, F> Future for OrElse<A, B, F>
where
    A: Future,
    B: IntoFuture<Item = A::Item>,
    F: FnOnce(A::Error) -> B,
{
    type Item = B::Item;
    type Error = B::Error;

    fn poll(&mut self) -> Poll<B::Item, B::Error> {
        self.state.poll(|a, f| match a {
            Ok(item) => Ok(Ok(item)),
            Err(e) => Ok(Err(f(e).into_future())),
        })
    }
}
