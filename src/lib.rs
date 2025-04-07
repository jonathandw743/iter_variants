// pub use iter_variants_derive::IterVariants;

mod ex2;

pub trait IterVariants {
    type T;
    fn iter_variants<F: Fn(Self::T)>(&self, f: F);
}

struct Foo;

impl IterVariants for Foo {
    type T = Self;
    fn iter_variants<F: Fn(Self::T)>(&self, f: F) {
        f(Self);
    }
}

#[derive(Clone, Copy)]
enum Bar {
    A,
    B,
}

impl IterVariants for Bar {
    type T = Self;
    fn iter_variants<F: Fn(Self::T)>(&self, f: F) {
        f(Self::A);
        f(Self::B);
    }
}

struct Baz {
    a: Bar,
    b: Bar,
}

impl IterVariants for Baz {
    type T = Self;
    fn iter_variants<F: Fn(Self)>(&self, f: F) {
        self.a
            .iter_variants(|a| self.b.iter_variants(|b| f(Self { a, b })));
    }
}

struct A(Bar, Bar);

impl IterVariants for A {
    type T = Self;
    fn iter_variants<F: Fn(Self::T)>(&self, f: F) {
        self.0
            .iter_variants(|v0| self.1.iter_variants(|v1| f(Self(v0, v1))));
    }
}

enum B {
    A(Bar, Bar),
    B,
    C { a: Bar, b: Bar },
}

impl IterVariants for B {
    type T = Self;
    fn iter_variants<F: Fn(Self::T)>(&self, f: F) {
        Bar::A.iter_variants(|v0| Bar::B.iter_variants(|v1| f(Self::A(v0, v1))));
        f(Self::B);
        Bar::A.iter_variants(|a| Bar::B.iter_variants(|b| f(Self::C { a, b })));
    }
}