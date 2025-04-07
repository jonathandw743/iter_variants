pub use iter_variants_derive::IterVariants;

pub trait IterVariants {
    type T;
    fn iter_variants<F: Fn(Self::T)>(f: F);
}

#[derive(IterVariants)]
struct Foo;

// impl IterVariants for Foo {
//     type T = Self;
//     fn iter_variants<F: Fn(Self::T)>(f: F) {
//         f(Self);
//     }
// }

#[derive(Clone, Copy, IterVariants, Debug)]
enum Bar {
    A,
    B,
}

// impl IterVariants for Bar {
//     type T = Self;
//     fn iter_variants<F: Fn(Self::T)>(f: F) {
//         f(Self::A);
//         f(Self::B);
//     }
// }

#[derive(IterVariants)]
struct Baz {
    a: Bar,
    b: Bar,
}

// impl IterVariants for Baz {
//     type T = Self;
//     fn iter_variants<F: Fn(Self)>(f: F) {
//         Bar::iter_variants(|a| Bar::iter_variants(|b| f(Self { a, b })));
//     }
// }

#[derive(IterVariants)]
struct A(Bar, Bar);

// impl IterVariants for A {
//     type T = Self;
//     fn iter_variants<F: Fn(Self::T)>(f: F) {
//         Bar::iter_variants(|v0| Bar::iter_variants(|v1| f(Self(v0, v1))));
//     }
// }

#[derive(IterVariants, Debug)]
pub enum B {
    A(Bar, Bar),
    B,
    C { a: Bar, b: Bar },
}

// impl IterVariants for B {
//     type T = Self;
//     fn iter_variants<F: Fn(Self::T)>(f: F) {
//         Bar::iter_variants(|v0| Bar::iter_variants(|v1| f(Self::A(v0, v1))));
//         f(Self::B);
//         Bar::iter_variants(|a| Bar::iter_variants(|b| f(Self::C { a, b })));
//     }
// }
