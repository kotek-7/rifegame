#[derive(thiserror::Error, Debug)]
#[error("index is out of bounds")]
pub struct IndexOutOfBoundsError;

#[derive(thiserror::Error, Debug)]
#[error("invalid format")]
pub struct InvalidFormatError;