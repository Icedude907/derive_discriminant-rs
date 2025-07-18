use derive_discriminant::{Discriminant, HasDiscriminant};


// Discriminant should be the same as rust's default behaviour
#[derive(Discriminant)]
pub enum Test1 {
    A, B, C, D
}

#[derive(Discriminant)]
pub enum Test2 {
    A = 5, B = 10, C, D = 2
}

#[derive(Discriminant)]
pub enum Test3 {
    // A = 0, B = 1, C = 2, D = 3
    A(bool), B(String), C(u32), D
}

#[derive(Discriminant)]
pub enum Test4{
    #[discriminant(5)] A(bool),
    B(String),
    #[discriminant(0)] C(u32),
    D
}
// #[derive_discriminant(u8)] // change the descriminant type

#[test]
fn main() {
    println!("Hello, world!");
    println!("Test1");
    println!("{}", Test1::A.discriminant());
    println!("{}", Test1::B.discriminant());
    println!("{}", Test1::C.discriminant());
    println!("{}", Test1::D.discriminant());
    println!("Test2");
    println!("{}", Test2::A.discriminant());
    println!("{}", Test2::B.discriminant());
    println!("{}", Test2::C.discriminant());
    println!("{}", Test2::D.discriminant());
    println!("Test3");
    println!("{}", Test3::A(false).discriminant());
    println!("{}", Test3::B("Ok".into()).discriminant());
    println!("{}", Test3::C(20).discriminant());
    println!("{}", Test3::D.discriminant());
    println!("Test4");
    println!("{}", Test4::A(false).discriminant());
    println!("{}", Test4::B("Ok".into()).discriminant());
    println!("{}", Test4::C(20).discriminant());
    println!("{}", Test4::D.discriminant());
}
