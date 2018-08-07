use futures;
use futures::Future;
use super::syntax::{Payload,Expr, Error};
use json_transformers;
use serde_json;
pub type EvalResult<'a> = Box<Future<Item = Payload, Error = Error> + Send + 'a>;

pub trait Eval : Sync {

  fn invoke<'a,'b>(&'b self, name: &'b str, input: Payload) -> EvalResult<'a>;

  fn fetch<'a,'b>(&'b self, path: &'b str) -> EvalResult<'a>;

  fn eval<'a,'b>(&'b self, input: Payload, expr: &'a Expr) -> EvalResult<'b> 
  where 'a : 'b {
    match expr {
      Expr::Pure(n) => self.invoke(n, input),
      Expr::Seq(e1, e2) => Box::new(self.eval(input, e1)
          .and_then(move |result| self.eval(result, e2))),
      Expr::Fetch(path) => self.fetch(path),
      Expr::Project(jtExpr) => match input {
        Payload::Chunk(c) => 
        Box::new(serde_json::from_slice(c.bytes())
                   .map(|x| () ))
      }
    }
  }

}