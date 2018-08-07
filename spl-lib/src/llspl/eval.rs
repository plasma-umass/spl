use futures;
use futures::Future;
use super::syntax::{Payload,Expr};
use super::error::Error;
use json_transformers;
pub type EvalResult<'a> = Box<Future<Item = Payload, Error = Error> + Send + 'a>;

fn eval_json(jt_expr: &json_transformers::Expr,
  input: Payload) -> Result<Payload, Error> {
  let in_json = input.to_json()?;
  let out_json = json_transformers::eval(jt_expr, &in_json)
    .ok_or(Error::JsonEval)?;
  Ok(Payload::Json(out_json))
}

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
      Expr::Project(jt_expr) =>
        Box::new(futures::future::result(eval_json(&jt_expr, input)))
    }
  }

}