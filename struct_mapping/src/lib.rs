//! # StructMapping
//!
//! StructMapping is a library for create string-based accessors/mutators Rust data structure..
//!
//! ## Custom type implementation
//!
//! ```ignore
//! use struct_mapping::ToStructMappingField;
//!
//! #[inline]
//! fn sm_mutator(key: &str, value: &str) -> Result<Self, MutatorError> {
//!     value
//!         .parse::<bool>()
//!         .map_err(|err| MutatorError::InvalidValue {
//!             key: key.to_string(),
//!             error: err.to_string(),
//!         })
//! }
//! ```
//!

#[derive(Debug)]
pub enum MutatorError {
    // Mutator's error on invalid key
    InvalidKey,
    /// Mutator's error on value serialization
    InvalidValue {
        /// Key parameters from mutator
        key: String,
        /// Error string from mutator's serialization error
        error: String,
    },
}

#[allow(rustdoc::invalid_rust_codeblocks)]
/// The public trait `ToStructMappingField` allows to implement a new type serialization
///
/// # Example
///
/// ```ignore
/// impl ToStructMappingField for bool {
/// #[inline]
/// fn sm_mutator(key: &str, value: &str) -> Result<Self, MutatorError> {
///     value
///         .parse::<bool>()
///         .map_err(|err| MutatorError::InvalidValue {
///             key: key.to_string(),
///             error: err.to_string(),
///         })
/// }
/// ```
pub trait ToStructMappingField
where
    Self: Sized,
{
    /// Method to convert a `&str` into a `Self` value for serialization
    fn sm_mutator(key: &str, value: &str) -> Result<Self, MutatorError>;
}

// Basic types/implementations
mod impls;

// Re-export #[derive(StructMapping)].
#[cfg(feature = "struct_mapping_derive")]
#[doc(hidden)]
pub use struct_mapping_derive::*;
