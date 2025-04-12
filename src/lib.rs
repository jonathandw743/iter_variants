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
        let mut vec = alloc::vec::Vec::new();
        Self::iter_variants(|value| {
            vec.push(value);
        });
        vec
    }
}

impl<T: IterVariants> IterVariants for Wrapping<T> {
    type IterVariantsInput = Wrapping<T::IterVariantsInput>;
    fn iter_variants<F: FnMut(Self::IterVariantsInput)>(mut f: F) {
        T::iter_variants(|value| f(Wrapping(value)));
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
    use core::num::{NonZeroU8, Wrapping};

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
}
