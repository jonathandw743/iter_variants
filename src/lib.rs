#![no_std]

use core::{marker::{PhantomData, PhantomPinned}, num::*};

pub use iter_variants_derive::IterVariants;
use iter_variants_derive::impl_iter_variants_tuple;

pub trait IterVariants {
    type IterVariantsInput;
    fn iter_variants<F: Fn(Self::IterVariantsInput)>(f: F);
}

impl<T: IterVariants> IterVariants for Wrapping<T> {
    type IterVariantsInput = Wrapping<T::IterVariantsInput>;
    fn iter_variants<F: Fn(Self::IterVariantsInput)>(f: F) {
        T::iter_variants(|value| f(Wrapping(value)));
    }
}
impl<T: ?Sized> IterVariants for PhantomData<T> {
    type IterVariantsInput = Self;
    fn iter_variants<F: Fn(Self::IterVariantsInput)>(f: F) {
        f(PhantomData)
    }
}
impl IterVariants for PhantomPinned {
    type IterVariantsInput = Self;
    fn iter_variants<F: Fn(Self::IterVariantsInput)>(f: F) {
        f(PhantomPinned)
    }
}
impl IterVariants for bool {
    type IterVariantsInput = Self;
    fn iter_variants<F: Fn(Self::IterVariantsInput)>(f: F) {
        f(false);
        f(true);
    }
}

impl<U: IterVariants> IterVariants for Option<U>
where
    <U as IterVariants>::IterVariantsInput: IterVariants,
{
    type IterVariantsInput = Option<<U as IterVariants>::IterVariantsInput>;
    fn iter_variants<F: Fn(Self::IterVariantsInput)>(f: F) {
        f(None);
        U::iter_variants(|v| f(Some(v)));
    }
}

macro_rules! impl_iter_variants_for_primitives {
    ( $t:ty ) => {
        impl IterVariants for $t {
            type IterVariantsInput = Self;
            fn iter_variants<F: Fn(Self::IterVariantsInput)>(f: F) {
                for i in <$t>::MIN..=<$t>::MAX {
                    f(i);
                }
            }
        }
    };
}
macro_rules! impl_iter_variants_for_nonzeros {
    ( $prim:ty, $t:ty ) => {
        impl IterVariants for $t {
            type IterVariantsInput = Self;
            fn iter_variants<F: Fn(Self::IterVariantsInput)>(f: F) {
                for i in <$prim>::MIN..=<$prim>::MAX {
                    if let Some(i) = <$t>::new(i) {
                        f(i);
                    }
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
impl_iter_variants_for_primitives!(usize);

impl_iter_variants_for_primitives!(i8);
impl_iter_variants_for_primitives!(i16);
impl_iter_variants_for_primitives!(i32);
impl_iter_variants_for_primitives!(i64);
impl_iter_variants_for_primitives!(i128);
impl_iter_variants_for_primitives!(isize);

impl_iter_variants_for_nonzeros!(u8,    NonZeroU8);
impl_iter_variants_for_nonzeros!(u16,   NonZeroU16);
impl_iter_variants_for_nonzeros!(u32,   NonZeroU32);
impl_iter_variants_for_nonzeros!(u64,   NonZeroU64);
impl_iter_variants_for_nonzeros!(u128,  NonZeroU128);
impl_iter_variants_for_nonzeros!(usize, NonZeroUsize);

impl_iter_variants_for_nonzeros!(i8,    NonZeroI8);
impl_iter_variants_for_nonzeros!(i16,   NonZeroI16);
impl_iter_variants_for_nonzeros!(i32,   NonZeroI32);
impl_iter_variants_for_nonzeros!(i64,   NonZeroI64);
impl_iter_variants_for_nonzeros!(i128,  NonZeroI128);
impl_iter_variants_for_nonzeros!(isize, NonZeroIsize);

impl_iter_variants_for_primitives!(char);

impl_iter_variants_tuple!();

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use core::num::{NonZeroU8, Wrapping};

    use super::IterVariants;

    #[derive(IterVariants, Clone, Copy)]
    enum A {
        B, C
    }

    #[derive(IterVariants)]
    struct Foo(Option<()>, u32, bool, Option<A>);

    #[derive(IterVariants)]
    struct Wraps(Wrapping<u32>, NonZeroU8, Wrapping<NonZeroU8>);

    #[derive(IterVariants)]
    enum Bar {
        A(bool),
        B(Option<bool>, usize),
        C,
        D {
            x: i32,
            y: Option<A>,
        }
    }
}
