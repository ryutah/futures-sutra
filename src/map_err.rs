use {Async, Future, Poll};

#[must_use = "futures do nothing unless polled"]
pub struct MapErr<A, F>
where
    A: Future,
{
    future: A,
    f: Option<F>,
}

pub fn new<A, F>(future: A, f: F) -> MapErr<A, F>
where
    A: Future,
{
    MapErr { future, f: Some(f) }
}

impl<U, A, F> Future for MapErr<A, F>
where
    A: Future,
    F: FnOnce(A::Error) -> U,
{
    type Item = A::Item;
    type Error = U;

    fn poll(&mut self) -> Poll<A::Item, U> {
        let e = match self.future.poll() {
            Ok(Async::NotReady) => return Ok(Async::NotReady),
            other => other,
        };
        e.map_err(self.f.take().expect("cannot poll MapErr twice"))
    }
}
