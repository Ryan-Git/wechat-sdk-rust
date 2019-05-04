use serde::Deserialize;
use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    Busy,
    InvalidArgument { field: String, msg: String },
    RateLimited(i32, String),
    ServerError(i32, String),
    Reqwest(reqwest::Error),
    IO(std::io::Error),
    Serde(serde_json::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Busy => write!(f, "wechat server is busy, please try later."),
            Error::InvalidArgument { field, msg } => {
                write!(f, "argument {} is invalid, msg: {}", field, msg)
            }
            Error::RateLimited(i, u) => write!(
                f,
                "request rate limited by wechat server, allowed {} per {}",
                i, u
            ),
            Error::ServerError(code, msg) => {
                write!(f, "wechat server returns error. code={} msg={}", code, msg)
            }
            Error::Reqwest(ref e) => e.fmt(f),
            Error::IO(ref e) => e.fmt(f),
            Error::Serde(ref e) => e.fmt(f),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Busy => None,
            Error::InvalidArgument { .. } => None,
            Error::RateLimited(..) => None,
            Error::ServerError(..) => None,
            Error::Reqwest(ref e) => e.source(),
            Error::IO(ref e) => e.source(),
            Error::Serde(ref e) => e.source(),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IO(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

pub(crate) type Result<T> = std::result::Result<T, Error>;

#[derive(Deserialize, Default)]
#[serde(default)]
pub(crate) struct CommonResponse {
    #[serde(alias = "errcode")]
    pub(crate) err_code: i32,
    #[serde(alias = "errmsg")]
    pub(crate) err_msg: String,
}

impl CommonResponse {
    pub(crate) const SUCCESS: i32 = 0;
    pub(crate) const BUSY: i32 = -1;
}
