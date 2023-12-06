use std::marker::PhantomData;

pub trait TypedIndex: Copy {
    fn from_usize(value: usize) -> Self;
    fn into_usize(self) -> usize;
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
pub struct TypedIndexCollection<X, T> {
    _marker: PhantomData<X>,
    pub(crate) inner: T,
}

impl<X, T> TypedIndexCollection<X, T> {
    #[inline]
    pub fn new(inner: T) -> Self {
        Self {
            _marker: PhantomData,
            inner,
        }
    }
}

impl<X, T> std::default::Default for TypedIndexCollection<X, T>
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
