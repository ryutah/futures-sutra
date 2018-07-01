#[macro_export]
macro_rules! try_ready {
    ($e:expr) => {
        match $e {
            Ok($crate::Async::Ready(t)) => t,
            Ok($crate::Async::NotReady) => return Ok($crate::Async::NotReady),
            Err(e) => return Err(From::from(e)),
        }
    };
}

pub type Poll<T, E> = Result<Async<T>, E>;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Async<T> {
    Ready(T),
    NotReady,
}

impl<T> Async<T> {
    pub fn map<F, U>(self, f: F) -> Async<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Async::NotReady => Async::NotReady,
            Async::Ready(t) => Async::Ready(f(t)),
        }
    }

    pub fn is_not_ready(&self) -> bool {
        match *self {
            Async::NotReady => true,
            Async::Ready(_) => false,
        }
    }

    pub fn is_ready(&self) -> bool {
        !self.is_not_ready()
    }
}
