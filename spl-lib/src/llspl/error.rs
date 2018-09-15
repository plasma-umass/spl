extern crate hyper;
extern crate http;
extern crate s3;
extern crate serde_json;

use std;

#[derive(Debug)]
pub enum Error {
    Storage(s3::error::S3Error),
    Http(http::Error),
    Hyper(hyper::Error),
    Json(serde_json::Error),
    JsonEval,
    InvokeError(String)
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Storage(e) => e.fmt(f),
            Error::Http(e) => e.fmt(f),
            Error::Hyper(e) => e.fmt(f),
            Error::Json(e) => e.fmt(f),
            Error::JsonEval => f.write_str("JsonEval"),
            Error::InvokeError(message) =>
                f.write_str(message)
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match self {
            Error::Storage(err) => err.description(),
            Error::Http(err) => err.description(),
            Error::Hyper(err) => err.description(),
            Error::Json(e) => e.description(),
            Error::JsonEval => "JsonEval",
            Error::InvokeError(message) => message
        }
    }

    // fn cause(&self) -> Option<&std::error::Error> {
    //     match self {
    //         Error::Storage(err) => Some(err),
    //         Error::Http(err) => Some(err),
    //         Error::Hyper(err) => Some(err)

    //     }
    // }
}
impl From<s3::error::S3Error> for Error {
    fn from(err: s3::error::S3Error) -> Error {
        Error::Storage(err)
    }
}
