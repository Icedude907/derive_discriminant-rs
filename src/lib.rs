pub use derive_discriminant_proc::*;

pub trait HasDiscriminant {
    fn discriminant(&self) -> u8;
}