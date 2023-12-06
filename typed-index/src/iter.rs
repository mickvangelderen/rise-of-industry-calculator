use crate::TypedIndex;

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

pub struct Indexed<I> {
    inner: I,
}

impl<I> Indexed<I> {
    pub fn new(inner: I) -> Self {
        Self { inner }
    }
}

impl<I> Iterator for Indexed<I>
where
    I: IndexableIterator,
{
    type Item = (I::Index, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.indexed_next()
    }
}

impl<I> DoubleEndedIterator for Indexed<I>
where
    I: IndexableDoubleEndedIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.indexed_next_back()
    }
}

pub struct Indexed2<X, T> {
    inner: T,
    index: X,
}

impl<X, T> Indexed2<X, T>
where
    X: TypedIndex,
{
    pub fn new(inner: T, index: X) -> Self {
        Self { inner, index }
    }

    pub fn new_from_zero(inner: T) -> Self {
        Self::new(inner, X::from_usize(0))
    }
}

impl<X, T> Iterator for Indexed2<X, T>
where
    T: IndexableIterator<Index = X>,
    X: TypedIndex + Copy,
{
    type Item = (X, T::Item);

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.inner.next()?;
        let index = self.index;
        self.index = X::from_usize(self.index.into_usize() + 1);
        Some((index, item))
    }
}

impl<X, T> DoubleEndedIterator for Indexed2<X, T>
where
    T: IndexableDoubleEndedIterator<Index = X>,
    X: TypedIndex,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.indexed_next_back()
    }
}

type x = std::iter::Enumerate<()>;
