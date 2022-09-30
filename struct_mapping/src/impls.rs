use crate::{MutatorError, ToStructMappingField};

macro_rules! primitive_impl {
    ($ty:ident) => {
        impl ToStructMappingField for $ty {
            #[inline]
            fn sm_mutator(key: &str, value: &str) -> Result<Self, MutatorError> {
                value
                    .parse::<$ty>()
                    .map_err(|err| MutatorError::InvalidValue {
                        key: key.to_string(),
                        error: err.to_string(),
                    })
            }
        }
    };
}

primitive_impl!(isize);
primitive_impl!(i8);
primitive_impl!(i16);
primitive_impl!(i32);
primitive_impl!(i64);
primitive_impl!(usize);
primitive_impl!(u8);
primitive_impl!(u16);
primitive_impl!(u32);
primitive_impl!(u64);
primitive_impl!(f32);
primitive_impl!(f64);

impl ToStructMappingField for bool {
    #[inline]
    fn sm_mutator(key: &str, value: &str) -> Result<Self, MutatorError> {
        value
            .parse::<bool>()
            .map_err(|err| MutatorError::InvalidValue {
                key: key.to_string(),
                error: err.to_string(),
            })
    }
}

impl ToStructMappingField for char {
    #[inline]
    fn sm_mutator(key: &str, value: &str) -> Result<Self, MutatorError> {
        value
            .parse::<char>()
            .map_err(|err| MutatorError::InvalidValue {
                key: key.to_string(),
                error: err.to_string(),
            })
    }
}
