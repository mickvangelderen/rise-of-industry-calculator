use std::ops::{Add, AddAssign};

/// Used to implement `Add<UntypedOffset>` for `TypedIndex` instead of implementing `Add<usize>`. A
/// typed index should not be modified carelessly.
pub struct UntypedOffset(pub usize);

pub trait TypedIndex:
    Copy + From<usize> + Into<usize> + Add<UntypedOffset, Output = Self> + AddAssign<UntypedOffset>
{
}

impl<T> TypedIndex for T where
    T: Copy
        + From<usize>
        + Into<usize>
        + Add<UntypedOffset, Output = Self>
        + AddAssign<UntypedOffset>
{
}

#[macro_export]
macro_rules! impl_typed_index {
    (pub struct $Index:ident($Inner:ty)) => {
        #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash)]
        pub struct $Index($Inner);

        impl From<usize> for $Index {
            #[inline]
            fn from(value: usize) -> Self {
                Self(value.into())
            }
        }

        impl From<$Index> for usize {
            #[inline]
            fn from(value: $Index) -> Self {
                value.0.into()
            }
        }

        impl ::std::ops::Add<$crate::UntypedOffset> for $Index {
            type Output = Self;

            #[inline]
            fn add(self, rhs: $crate::UntypedOffset) -> Self::Output {
                Self(self.0 + rhs.0)
            }
        }

        impl ::std::ops::AddAssign<$crate::UntypedOffset> for $Index {
            #[inline]
            fn add_assign(&mut self, rhs: $crate::UntypedOffset) {
                self.0 += rhs.0
            }
        }
    };
}
