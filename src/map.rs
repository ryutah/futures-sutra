use {Async, Future, Poll};

#[must_use = "futures do nothing unless polled"]
pub struct Map<A, F>
where
    A: Future,
{
    future: A,
    f: Option<F>,
}

pub fn new<A, F>(future: A, f: F) -> Map<A, F>
where
    A: Future,
{
    Map { future, f: Some(f) }
}

impl<U, A, F> Future for Map<A, F>
where
    A: Future,
    F: FnOnce,
{
    type Item = U;
    type Error = A::Error;

    fn poll(&mut self) -> Poll<U, A::Error> {
        let e = match self.future.poll() {
            Ok(Async::NotReady) => return Ok(Async::NotReady),
            Ok(Async::Ready(e)) => Ok(e),
            Err(e) => Err(e),
        };

        e.map(self.f.take().expect("cannnot poll map twice"))
            .map(Async::Ready)
    }
}
