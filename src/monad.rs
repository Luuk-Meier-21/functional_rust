pub trait Monad<T> {
    type Impl<U>;

    fn new(value: T) -> Self;
    fn map<U, F>(self, f: F) -> Self::Impl<U>
    where
        F: FnOnce(T) -> Self::Impl<U>;
}
