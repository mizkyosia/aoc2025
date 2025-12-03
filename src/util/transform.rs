pub trait Transform {
    fn transform<U>(self, f: impl FnMut(&Self) -> U) -> U;
}

impl<T> Transform for T {
    fn transform<U>(self, mut f: impl FnMut(&Self) -> U) -> U {
        f(&self)
    }
}
