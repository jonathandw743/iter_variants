#![no_std]

use core::{
    marker::{PhantomData, PhantomPinned},
    num::*,
};

extern crate alloc;

pub use iter_variants_derive::IterVariants;

macro_rules! impl_iter_variants_tuple {
    () => {
        impl_iter_variants_tuple! {
            V0,
            V1,
            V2,
            V3,
            V4,
            V5,
            V6,
            V7,
            V8,
            V9,
            V10,
            V11,
        }
    };
    ($($t:ident),+ $(,)?) => {
        impl_iter_variants_tuple!(@each() $($t)*);
    };
    (@each($($c:ident)*)) => {
        impl_iter_variants_tuple!(@impl());
    };
    (@each($($c:ident)*) $f:ident $($t:ident)*) => {
        impl_iter_variants_tuple!(@impl() $($c)* $f);
        impl_iter_variants_tuple!(@each($($c)* $f) $($t)*);
    };
    (@impl() $($t:ident)*) => {
        impl<$($t),*> IterVariants for ($($t,)*)
        where
            $($t: IterVariants,)*
            $(<$t as IterVariants>::IterVariantsInput: IterVariants + Copy,)*
        {
            type IterVariantsInput = ($(<$t as IterVariants>::IterVariantsInput,)*);

            fn iter_variants<F: FnMut(Self::IterVariantsInput)>(mut f: F) {
                impl_iter_variants_tuple!(@expr {
                    f(($($t,)*))
                } $($t)*);
            }

            fn iter_variants_count() -> usize {
                impl_iter_variants_tuple!(@count {
                    1usize
                } $($t)*)
            }
        }
    };
    (@expr $b:block) => {
        $b
    };
    (@expr $b:block $f:ident $($t:ident)*) => {
        impl_iter_variants_tuple!(@expr {
            #[allow(non_snake_case)]
            $f::iter_variants(|$f| {
                $b
            })
        } $($t)*);
    };
    (@count {$b:expr}) => {
        $b
    };
    (@count {$b:expr} $f:ident $($t:ident)*) => {
        impl_iter_variants_tuple!(@count {
            $b.saturating_mul($f::iter_variants_count())
        } $($t)*)
    };
}

/// Iterate each variant
///
/// # Examples
/// ```
/// use iter_variants::IterVariants;
///
/// Option::<(bool, bool)>::iter_variants(|value| {
///     println!("{:?}", value);
/// });
/// ```
pub trait IterVariants {
    type IterVariantsInput;

    /// Iterate each variant
    ///
    /// Calls the provided function on all variants of `Self` to any depth.
    ///
    /// # Examples
    /// ```
    /// # use iter_variants::IterVariants;
    /// let mut vec = vec![];
    /// Option::<(bool, bool)>::iter_variants(|value| {
    ///     vec.push(value);
    /// });
    /// assert_eq!(vec, [
    ///     None,
    ///     Some((false, false)),
    ///     Some((true, false)),
    ///     Some((false, true)),
    ///     Some((true, true))
    /// ]);
    /// ```
    fn iter_variants<F: FnMut(Self::IterVariantsInput)>(f: F);

    /// Collect each variant into a `Vec`
    ///
    /// # Examples
    /// ```
    /// # use iter_variants::IterVariants;
    /// assert_eq!(Option::<(bool, bool)>::collect_variants(), [
    ///     None,
    ///     Some((false, false)),
    ///     Some((true, false)),
    ///     Some((false, true)),
    ///     Some((true, true))
    /// ]);
    /// ```
    fn collect_variants() -> alloc::vec::Vec<Self::IterVariantsInput> {
        let mut vec = alloc::vec::Vec::with_capacity(Self::iter_variants_count());
        Self::iter_variants(|value| {
            vec.push(value);
        });
        vec
    }

    /// Rough variants count, used for optimization
    ///
    /// The result may overflow usize, use [`usize::MAX`]
    /// assert_eq!(u8::iter_variants_count(), 256);
    /// assert_eq!(Option::<u8>::iter_variants_count(), 257);
    /// ```
    fn iter_variants_count() -> usize {
        1
    }
}

impl<T: IterVariants> IterVariants for Wrapping<T> {
    type IterVariantsInput = Wrapping<T::IterVariantsInput>;
    fn iter_variants<F: FnMut(Self::IterVariantsInput)>(mut f: F) {
        T::iter_variants(|value| f(Wrapping(value)));
    }

    fn iter_variants_count() -> usize {
        T::iter_variants_count()
    }
}
impl<T: ?Sized> IterVariants for PhantomData<T> {
    type IterVariantsInput = Self;
    fn iter_variants<F: FnMut(Self::IterVariantsInput)>(mut f: F) {
        f(PhantomData)
    }
}
impl IterVariants for PhantomPinned {
    type IterVariantsInput = Self;
    fn iter_variants<F: FnMut(Self::IterVariantsInput)>(mut f: F) {
        f(PhantomPinned)
    }
}
impl IterVariants for bool {
    type IterVariantsInput = Self;
    fn iter_variants<F: FnMut(Self::IterVariantsInput)>(mut f: F) {
        f(false);
        f(true);
    }

    fn iter_variants_count() -> usize {
        2
    }
}

impl<T: IterVariants> IterVariants for Option<T>
where
    <T as IterVariants>::IterVariantsInput: IterVariants,
{
    type IterVariantsInput = Option<<T as IterVariants>::IterVariantsInput>;
    fn iter_variants<F: FnMut(Self::IterVariantsInput)>(mut f: F) {
        f(None);
        T::iter_variants(|v| f(Some(v)));
    }

    fn iter_variants_count() -> usize {
        T::iter_variants_count().saturating_add(1)
    }
}

macro_rules! impl_iter_variants_for_primitives {
    ( $t:ty ) => {
        impl IterVariants for $t {
            type IterVariantsInput = Self;
            fn iter_variants<F: FnMut(Self::IterVariantsInput)>(mut f: F) {
                for i in <$t>::MIN..=<$t>::MAX {
                    f(i);
                }
            }

            fn iter_variants_count() -> usize {
                (<$t>::MAX as usize)
                    .wrapping_sub(<$t>::MIN as usize)
                    .saturating_add(1)
            }
        }
    };
}
macro_rules! impl_iter_variants_for_nonzeros {
    ( $prim:ty, $t:ty ) => {
        impl IterVariants for $t {
            type IterVariantsInput = Self;
            fn iter_variants<F: FnMut(Self::IterVariantsInput)>(mut f: F) {
                for i in <$prim>::MIN..=<$prim>::MAX {
                    if let Some(i) = <$t>::new(i) {
                        f(i);
                    }
                }
            }

            fn iter_variants_count() -> usize {
                (<$prim>::MAX as usize)
                    .wrapping_sub(<$prim>::MIN as usize)
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

impl_iter_variants_for_nonzeros!(u8, NonZeroU8);
impl_iter_variants_for_nonzeros!(u16, NonZeroU16);
impl_iter_variants_for_nonzeros!(u32, NonZeroU32);
impl_iter_variants_for_nonzeros!(u64, NonZeroU64);
impl_iter_variants_for_nonzeros!(u128, NonZeroU128);
impl_iter_variants_for_nonzeros!(usize, NonZeroUsize);

impl_iter_variants_for_nonzeros!(i8, NonZeroI8);
impl_iter_variants_for_nonzeros!(i16, NonZeroI16);
impl_iter_variants_for_nonzeros!(i32, NonZeroI32);
impl_iter_variants_for_nonzeros!(i64, NonZeroI64);
impl_iter_variants_for_nonzeros!(i128, NonZeroI128);
impl_iter_variants_for_nonzeros!(isize, NonZeroIsize);

impl_iter_variants_for_primitives!(char);

impl_iter_variants_tuple!();

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use core::num::{NonZeroI128, NonZeroI8, NonZeroIsize, NonZeroU128, NonZeroU16, NonZeroU8, NonZeroUsize, Wrapping};

    use super::IterVariants;

    extern crate alloc;
    use alloc::vec;

    #[derive(IterVariants)]
    struct Unit;

    #[derive(IterVariants, Clone, Copy)]
    enum A {
        B,
        C,
    }

    #[derive(IterVariants, Clone, Copy)]
    enum B {
        F3(bool, bool, bool),
        F2(bool, bool),
        F1(bool),
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
        D { x: i32, y: Option<A> },
    }

    #[derive(IterVariants, Debug, PartialEq, Eq, Clone, Copy)]
    enum Baz<T: Sync>
    where
        T: Send,
    {
        A(bool),
        B(T),
    }

    #[derive(IterVariants)]
    enum Assoc<T: IntoIterator> {
        A(bool),
        B(T::Item),
    }

    #[derive(IterVariants)]
    enum Assoc1<T: IntoIterator>
    where
        T::Item: Copy,
    {
        A(bool),
        B(T, T::Item),
    }

    #[derive(IterVariants)]
    struct Assoc2<T: IntoIterator>
    where
        T::Item: Copy,
    {
        a: bool,
        b: T::Item,
    }

    #[derive(IterVariants)]
    struct Assoc3<T: Copy> {
        a: bool,
        b: (T,),
    }

    #[test]
    fn test_generic_param() {
        let mut output = [None; 6];
        Baz::<(bool, bool)>::iter_variants(|v| {
            let slot = output.iter_mut().find(|x| x.is_none()).unwrap();
            *slot = Some(v);
        });
        assert_eq!(
            output,
            [
                Some(Baz::A(false)),
                Some(Baz::A(true)),
                Some(Baz::B((false, false))),
                Some(Baz::B((true, false))),
                Some(Baz::B((false, true))),
                Some(Baz::B((true, true))),
            ]
        );
    }

    #[test]
    fn iter_variants_example() {
        let mut vec = vec![];
        Option::<(bool, bool)>::iter_variants(|value| {
            vec.push(value);
        });
        assert_eq!(
            vec,
            [
                None,
                Some((false, false)),
                Some((true, false)),
                Some((false, true)),
                Some((true, true))
            ]
        );
    }

    #[test]
    fn collect_variants_example() {
        assert_eq!(
            Option::<(bool, bool)>::collect_variants(),
            [
                None,
                Some((false, false)),
                Some((true, false)),
                Some((false, true)),
                Some((true, true))
            ]
        );
    }

    fn test_variants_count() {
        assert_eq!(char::iter_variants_count(), 0x11_0000);
        assert_eq!(i8::iter_variants_count(), 0x100);
        assert_eq!(i16::iter_variants_count(), 0x10000);
        assert_eq!(u8::iter_variants_count(), 0x100);
        assert_eq!(u16::iter_variants_count(), 0x10000);
        assert_eq!(isize::iter_variants_count(), usize::MAX);
        assert_eq!(usize::iter_variants_count(), usize::MAX);
        assert_eq!(NonZeroI8::iter_variants_count(), 0xff);
        assert_eq!(NonZeroU8::iter_variants_count(), 0xff);
        assert_eq!(NonZeroU16::iter_variants_count(), 0xffff);
        assert_eq!(NonZeroIsize::iter_variants_count(), usize::MAX);
        assert_eq!(NonZeroUsize::iter_variants_count(), usize::MAX);
        #[cfg(any(target_pointer_width = "64", target_pointer_width = "32"))]
        {
            assert_eq!(NonZeroI128::iter_variants_count(), usize::MAX);
            assert_eq!(NonZeroU128::iter_variants_count(), usize::MAX);
            assert_eq!(i128::iter_variants_count(), usize::MAX);
            assert_eq!(u128::iter_variants_count(), usize::MAX);
        }
        assert_eq!(<()>::iter_variants_count(), 1);
        assert_eq!(<(bool,)>::iter_variants_count(), 2);
        assert_eq!(<(bool, bool)>::iter_variants_count(), 4);
        assert_eq!(<(bool, bool, bool)>::iter_variants_count(), 8);

        assert_eq!(A::iter_variants_count(), 2);
        assert_eq!(Baz::<bool>::iter_variants_count(), 4);
        assert_eq!(B::iter_variants_count(), 14);
    }
}
