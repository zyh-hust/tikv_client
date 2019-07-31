#[derive(Debug)]
pub enum Error {
    TiKVError(String),
    PdError(String),
    OperationError(String),
    Other,
}

pub struct Res{
    resp: bool,
    put_resp: std::option::Option<RawPutResponse>,
    split_resp: std::option::Option<SplitRegionResponse>,
}

impl Res {
    pub fn new() -> Self {
        Res{
            resp: true,
            put_resp: None,
            split_resp: None,
        }
    }

    pub fn put_err(resp: RawPutResponse) -> Self{
        Res{
            resp: false,
            put_resp: Some(resp),
            split_resp: None,
        }
    }

    pub fn split_err(resp: SplitRegionResponse) -> Self{
        Res{
            resp: false,
            put_resp: None,
            split_resp: Some(resp),
        }
    }
}

pub type Result<T> = result::Result<T, Error>;