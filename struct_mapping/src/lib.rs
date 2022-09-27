//! # StructMapping
//!
//! StructMapping is a library for create string-based accessors/mutators Rust data structure..
//!

#[derive(Debug)]
pub enum Error {
    InvalidInput,
    InvalidKey(&'static str),
}

pub trait ToStructMappingField
where
    Self: Sized,
{
    fn from_string(value: impl AsRef<str>) -> Result<Self, Error>;
}

mod impls;

// Re-export #[derive(StructMapping)].
#[cfg(feature = "struct_mapping_derive")]
#[doc(hidden)]
pub use struct_mapping_derive::*;
