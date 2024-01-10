use crate::monad::Monad;

#[derive(Debug)]
pub enum Maybe<T> {
    Just(T),
    None,
}

use Maybe::{Just, None};

impl<T> Monad<T> for Maybe<T> {
    type Impl<U> = Maybe<U>;

    // Constructor
    fn new(value: T) -> Self {
        Self::Just(value)
    }

    fn map<U, F>(self, f: F) -> Self::Impl<U>
    where
        F: FnOnce(T) -> Self::Impl<U>,
    {
        match self {
            Just(value) => f(value),
            None => None,
        }
    }
}
