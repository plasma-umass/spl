extern crate hyper;
extern crate std;
extern crate futures;

use futures::Future;
use hyper::Body;

pub type EvalResult<'a> = Box<Future<Item = Payload, Error = ()> + Send + 'a>;

// An invoker invokes a serverless function with the provided name. We will
// have different invoker implementations for different platforms. Moreover,
// we can test the language by fudging an invoker that doesn't call any
// external functions, e.g, the TestInvoker below.
pub trait Invoker {
  fn invoke<'a,'b>(&'b self, name: &'b str, input: Payload) -> EvalResult<'a>;
}

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

}

// SPL expressions
pub enum Expr {
    Pure(String),
    Seq(Box<Expr>, Box<Expr>)
}

// Convenience function for testing
pub fn inspect(payload: &Payload) -> &str {
    match payload {
        Payload::Chunk(chunk) => std::str::from_utf8(chunk.as_ref())
          .expect("failed to parse chunk as UTF-8")
    }
}
