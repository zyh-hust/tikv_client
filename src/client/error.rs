#[derive(Debug)]
pub enum Error {
    PdError(String),
}

pub type Result<T> = std::result::Result<T, Error>;
