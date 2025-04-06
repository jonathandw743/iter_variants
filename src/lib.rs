pub use iter_variants_derive::IterVariants;

pub trait IterVariants {
    fn iter_variants(&self, f: fn(Self));
}

#[derive(IterVariants)]
pub struct Foo;