use std::num::{NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize};

macro_rules! impl_index_types {
    ($($N:ty => $I:ident($M:ident($Z:ty))),* $(,)?) => {
        $(
            #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash)]
            struct $M($Z);

            impl $M {
                #[inline]
                pub fn new(value: $N) -> Option<Self> {
                    <$Z>::new(value.wrapping_add(1)).map(Self)
                }

                #[inline]
                pub fn get(self) -> $N {
                    <$Z>::get(self.0).wrapping_sub(1)
                }
            }

            #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash)]
            pub struct $I($M);

            impl $I {
                #[inline]
                pub fn new(value: $N) -> Self {
                    Self(<$M>::new(value).expect("index too large"))
                }

                #[inline]
                pub fn get(self) -> $N {
                    self.0.get()
                }
            }

            impl From<usize> for $I {
                #[inline]
                fn from(value: usize) -> Self {
                    <$I>::new(value.try_into().expect("index too large"))
                }
            }

            impl From<$I> for usize {
                #[inline]
                fn from(value: $I) -> Self {
                    value.get().try_into().expect("index too large")
                }
            }
        )*
    };
}

impl_index_types! {
    u8 => IndexU8(NonMaxU8(NonZeroU8)),
    u16 => IndexU16(NonMaxU16(NonZeroU16)),
    u32 => IndexU32(NonMaxU32(NonZeroU32)),
    u64 => IndexU64(NonMaxU64(NonZeroU64)),
    usize => IndexUsize(NonMaxUsize(NonZeroUsize)),
}
