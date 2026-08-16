pub trait By {
    fn by_ref(&self) -> &Self {
        self
    }

    fn by_mut(&mut self) -> &mut Self {
        self
    }
}

impl<T: ?Sized> By for T {}
