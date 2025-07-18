# Derive Discriminant
This crate offers a convenient way to associate unique numbers to each variant of an `enum`.

Unlike rust's built in syntax `enum Foo { A = 2 }`, this macro doesn't change the underlying representation.

It can therefore be applied to `enum`s with associated data (aka discriminated unions, variants) without forcing rust to `#[repr(u8)]` or similar.

This was created to help binary serialisation, which is what I use it for.

### Example
```rust
#[derive(Discriminant)]
enum Foo{
    A(bool),
    B(u32),
    C{field: String},
    D
}
```

```rust
// Will generate the following
impl HasDiscriminant for Foo{
    pub fn discriminant(&self)->u8{match self{
        Self::A(..) => 0,
        Self::B(..) => 1,
        Self::C{..} => 2,
        Self::D => 3
    }}
}
```

#### Overriding Discriminants
```rust
#[derive(Discriminant)]
enum Foo{
    A(bool),
    #[discriminant(5)] B(u32),
    C{field: String},
    D
}
```
Maps `A=0, B=5, C=6, D=7`