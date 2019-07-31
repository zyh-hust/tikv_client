#[derive(Debug)]
pub enum Error {
    TiKVError(String),
    PdError(String),
    OperationError(String),
    Other,
}

pub type Result<T> = std::result::Result<T, Error>;
