extern crate hyper;
extern crate futures;

mod ast;
mod google_cloud_functions;

use ast::*;
use hyper::rt::{Future};

pub fn eval<'a,'b>(invoker: &'b (Invoker + Sync), input: ast::Payload, expr: &'a ast::Expr) -> EvalResult<'b> 
  where 'a : 'b {
    match expr {
        Expr::Pure(n) => invoker.invoke(n, input),
        Expr::Seq(e1, e2) => Box::new(eval(invoker, input, e1)
          .and_then(move |result| { eval(invoker, result, e2) }))
    }
}

fn main() {
    let invoker = google_cloud_functions::new("arjunguha-research-group", "us-central1");
    let f =   invoker.invoke("hello", Payload::from("helloz.txt"))
        .map(|x| ());
    hyper::rt::run(f);
}

#[cfg(test)]
mod tests {
    use futures::future::{self};
    use super::*;

    struct TestInvoker { }

    impl Invoker for TestInvoker {
        fn invoke(&self, name: &str, _input: ast::Payload) -> EvalResult {
            match name {
                "f" => Box::new(future::ok(ast::Payload::Chunk(hyper::Chunk::from("fromF")))),
                _ => Box::new(future::ok(ast::Payload::Chunk(hyper::Chunk::from("unknown"))))
            }
        }
    }

    #[test]
    fn trivial() {
        let invoker = TestInvoker { };
        let input = ast::Payload::Chunk(hyper::Chunk::from("lol"));
        let expr = ast::Expr::Pure(String::from("f"));
        let r = eval(&invoker, input, &expr);
        assert_eq!(
            r.wait().as_ref().map(|x| { ast::inspect(x) }), 
            Result::Ok("fromF"));
    }
}