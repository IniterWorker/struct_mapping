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
