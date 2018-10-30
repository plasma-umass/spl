extern crate hyper;
extern crate futures;
extern crate serde_json;

use std;
use hyper::{Body, Response};
use json_transformers;
use super::error::Error;

// An invoker invokes a serverless function with the provided name. We will
// have different invoker implementations for different platforms. Moreover,
// we can test the language by fudging an invoker that doesn't call any
// external functions, e.g, the TestInvoker below.
// pub trait Invoker {
//   fn invoke<'a,'b>(&'b self, name: &'b str, input: Payload) -> EvalResult<'a>;
// }

// Inputs and outputs of SPL programs

#[derive(Debug)]
pub enum Payload {
    Chunk(hyper::Chunk),
    Json(serde_json::Value)
}

impl Clone for Payload {
    fn clone(&self) -> Self {
        match self {
            Payload::Chunk(c) => Payload::Chunk(hyper::Chunk::from(c.to_vec())),
            Payload::Json(v) => Payload::Json(v.clone())
        }
    }
}

impl PartialEq for Payload {
  fn eq(&self, other: &Payload) -> bool {
      match (self, other) {
          (Payload::Json(x), Payload::Json(y)) => x == y,
          (Payload::Chunk(_x), Payload::Chunk(_y)) => false,
          _ => false
      }
  }
}

impl Payload {

    pub fn to_json(self) -> Result<serde_json::Value, Error> {
        match self {
            Payload::Json(json) => Ok(json),
            Payload::Chunk(chunk) =>
                serde_json::from_slice(chunk.as_ref())
                    .map_err(|e| Error::Json(e))
        }
    }
    pub fn to_body(self) -> Result<Body, Error> {
        match self {
           Payload::Chunk(chunk) => Ok(Body::from(chunk)),
           Payload::Json(json) =>
            serde_json::to_vec(&json)
                .map_err(|e| Error::Json(e))
                .map(|vec| Body::from(vec))
        }
    }

    pub fn to_response(self) -> Result<Response<Body>, Error> {
        self.to_body().and_then(|body|
          Response::builder()
          .body(body)
          .map_err(|err| Error::Http(err)))
    }

    pub fn from(a_str: &'static str) -> Payload {
        Payload::Chunk(hyper::Chunk::from(a_str))
    }

    pub fn from_vec(vec: Vec<u8>) -> Payload {
        Payload::Chunk(hyper::Chunk::from(vec))
    }

}

// SPL expressions
#[derive(Debug, PartialEq)]
pub enum Expr {
    Pure(String),
    Download(json_transformers::Expr),
    Seq(Box<Expr>, Box<Expr>),
    Project(json_transformers::Expr),
    Fetch(String),
    Split(Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>)
}

// Convenience function for testing
pub fn inspect(payload: &Payload) -> &str {
    match payload {
        Payload::Chunk(chunk) => std::str::from_utf8(chunk.as_ref())
          .expect("failed to parse chunk as UTF-8"),
        Payload::Json(json) => json.as_str().expect("could not show JSON")
    }
}
