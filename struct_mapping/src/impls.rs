use crate::{Error, ToStructMappingField};

macro_rules! primitive_impl {
    ($ty:ident) => {
        impl ToStructMappingField for $ty {
            #[inline]
            fn from_string(value: impl AsRef<str>) -> Result<Self, Error> {
                value
                    .as_ref()
                    .parse::<$ty>()
                    .map_err(|_| Error::InvalidInput)
            }
        }
    };
}

// TODO: Float
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

impl ToStructMappingField for bool {
    #[inline]
    fn from_string(value: impl AsRef<str>) -> Result<Self, Error> {
        if let Ok(number) = value.as_ref().parse::<i32>() {
            Ok(number > 0)
        } else {
            Ok(value.as_ref() == "true" || value.as_ref() == "True")
        }
    }
}

impl ToStructMappingField for char {
    #[inline]
    fn from_string(value: impl AsRef<str>) -> Result<Self, Error> {
        match value.as_ref().len() {
            0 => Err(Error::InvalidInput),
            1 => Ok(value.as_ref().as_bytes()[0] as char),
            _ => Err(Error::InvalidInput),
        }
    }
}
