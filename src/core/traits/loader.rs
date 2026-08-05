pub trait Loader<T> {
    type Output;
    fn load(&self, source: T) -> Self::Output;
}
