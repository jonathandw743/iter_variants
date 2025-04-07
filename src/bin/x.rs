use iter_variants::{B, IterVariants};

fn main() {
    B::iter_variants(|x| {
        dbg!(x);
    });
}
