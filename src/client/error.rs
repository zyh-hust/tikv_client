#[derive(Debug)]
pub enum Error {
    PdError(String),
    KvError(String),
}

pub type Result<T> = std::result::Result<T, Error>;
