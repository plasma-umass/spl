extern crate hyper;
extern crate futures;
extern crate http;

use std;
use hyper::Body;
use json_transformers;
use s3::error::S3Error;

#[derive(Debug)]
pub enum Error {
    Storage(S3Error),
    Http(http::Error),
    Hyper(hyper::Error)
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Storage(e) => e.fmt(f),
            Error::Http(e) => e.fmt(f),
            Error::Hyper(e) => e.fmt(f)
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match self {
            Error::Storage(err) => err.description(),
            Error::Http(err) => err.description(),
            Error::Hyper(err) => err.description()
        }
    }

    fn cause(&self) -> Option<&std::error::Error> {
        match self {
            Error::Storage(err) => Some(err),
            Error::Http(err) => Some(err),
            Error::Hyper(err) => Some(err)

        }
    }
}
impl From<S3Error> for Error {
    fn from(err: S3Error) -> Error {
        Error::Storage(err)
    }
}

// An invoker invokes a serverless function with the provided name. We will
// have different invoker implementations for different platforms. Moreover,
// we can test the language by fudging an invoker that doesn't call any
// external functions, e.g, the TestInvoker below.
// pub trait Invoker {
//   fn invoke<'a,'b>(&'b self, name: &'b str, input: Payload) -> EvalResult<'a>;
// }

// Inputs and outputs of SPL programs
pub enum Payload {
    Chunk(hyper::Chunk)
}

impl Payload {
    pub fn to_body(self) -> Body {
        match self {
        Payload::Chunk(chunk) => Body::from(chunk),
        }
    }

    pub fn from(a_str: &'static str) -> Payload {
        Payload::Chunk(hyper::Chunk::from(a_str))
    }

    pub fn from_vec(vec: Vec<u8>) -> Payload {
        Payload::Chunk(hyper::Chunk::from(vec))
    }

}

// SPL expressions
pub enum Expr {
    Pure(String),
    Seq(Box<Expr>, Box<Expr>),
    Project(json_transformers::Expr),
    Fetch(String)    
}

// Convenience function for testing
pub fn inspect(payload: &Payload) -> &str {
    match payload {
        Payload::Chunk(chunk) => std::str::from_utf8(chunk.as_ref())
          .expect("failed to parse chunk as UTF-8")
    }
}
