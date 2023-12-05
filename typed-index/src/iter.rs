pub trait IndexableIterator: Iterator {
    type Index;

    fn indexed_next(&mut self) -> Option<(Self::Index, Self::Item)>;

    fn index(self) -> Indexed<Self>
    where
        Self: Sized,
    {
        Indexed { inner: self }
    }
}

pub trait IndexableDoubleEndedIterator: IndexableIterator {
    fn indexed_next_back(&mut self) -> Option<(Self::Index, Self::Item)>;
}

pub struct Indexed<T> {
    inner: T,
}

impl<T> Indexed<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> Iterator for Indexed<T>
where
    T: IndexableIterator,
{
    type Item = (T::Index, T::Item);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.indexed_next()
    }
}

impl<T> DoubleEndedIterator for Indexed<T>
where
    T: IndexableDoubleEndedIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.indexed_next_back()
    }
}
