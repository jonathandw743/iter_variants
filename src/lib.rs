use iter_variants_derive::impl_iter_variants_tuple;
pub use iter_variants_derive::IterVariants;

pub trait IterVariants {
    type T;
    fn iter_variants<F: Fn(Self::T)>(f: F);
}

impl IterVariants for bool {
    type T = Self;
    fn iter_variants<F: Fn(Self::T)>(f: F) {
        f(false);
        f(true);
    }
}

macro_rules! impl_iter_variants_for_primitives {
    ( $t:ty ) => {
        impl IterVariants for $t {
            type T = Self;
            fn iter_variants<F: Fn(Self::T)>(f: F) {
                for i in <$t>::MIN..=<$t>::MAX {
                    f(i);
                }
            }
        }
    };
}

impl_iter_variants_for_primitives!(u8);
impl_iter_variants_for_primitives!(u16);
impl_iter_variants_for_primitives!(u32);
impl_iter_variants_for_primitives!(u64);
impl_iter_variants_for_primitives!(u128);

impl_iter_variants_for_primitives!(i8);
impl_iter_variants_for_primitives!(i16);
impl_iter_variants_for_primitives!(i32);
impl_iter_variants_for_primitives!(i64);
impl_iter_variants_for_primitives!(i128);

impl_iter_variants_tuple!();