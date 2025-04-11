# iter_variants

`cargo add iter_variants`

Provides a trait `IterVariants` with one method `iter_variants` that takes one argument, an `FnMut` that is called on all variants of the given struct/enum. For exampe, `bool::iter_variants(f)` would call `f(false)` and `f(true)`.

There is a corresponding derive macro that implements `IterVariants` for enums and structs where all fields are either unit or only contain types that also implement `IterVariants`.

Thanks to @A4-Tacks for work on generics and implementations for many core types.