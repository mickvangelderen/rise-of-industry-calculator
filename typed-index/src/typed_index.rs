use std::marker::PhantomData;

pub trait TypedIndex {
    fn from_usize(value: usize) -> Self;
    fn into_usize(self) -> usize;
}

impl<I> TypedIndex for I
where
    I: From<usize> + Into<usize>,
{
    #[inline]
    fn from_usize(value: usize) -> Self {
        value.into()
    }

    #[inline]
    fn into_usize(self) -> usize {
        self.into()
    }
}

#[macro_export]
/// $Inner must implement From<usize> + Into<usize>.
macro_rules! impl_typed_index {
    (pub struct $Index:ident($Inner:ty)) => {
        #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash)]
        pub struct $Index($Inner);

        impl $crate::TypedIndex for $Index {
            fn from_usize(value: usize) -> Self {
                Self(<$Inner>::from(value))
            }

            fn into_usize(self) -> usize {
                self.0.into()
            }
        }
    };
}

// TODO: Determine if we want this abstraction or just implement the distinct types directly.
pub struct TypedIndexCollection<I, T> {
    _marker: PhantomData<I>,
    pub(crate) inner: T,
}

impl<I, T> TypedIndexCollection<I, T> {
    #[inline]
    pub fn new(inner: T) -> Self {
        Self {
            _marker: PhantomData,
            inner,
        }
    }
}

impl<I, T> std::default::Default for TypedIndexCollection<I, T>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            _marker: Default::default(),
            inner: Default::default(),
        }
    }
}
