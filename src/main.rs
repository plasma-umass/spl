extern crate hyper;
extern crate futures;
mod ast;
use ast::*;
use hyper::rt::{Future};

type EvalResult<'a> = Box<Future<Item = ast::Payload, Error = ()> + 'a>;

// An invoker invokes a serverless function with the provided name. We
// will different invoker implementations for different platforms. Moreoever,
// we can test the language by fudging an invoker that doesn't call any
// external functions, e.g, the TestInvoker below.
pub trait Invoker {
    fn invoke(&self, name: &str, input: &ast::Payload) -> EvalResult;
}

pub fn eval<'a>(invoker: &'a Invoker, input: ast::Payload, expr: &'a ast::Expr) -> EvalResult<'a> {
    match expr {
        Expr::Pure(n) => invoker.invoke(n, &input),
        Expr::Seq(e1, e2) => Box::new(eval(invoker, input, e1)
          .and_then(move |result| { eval(invoker, result, e2) }))
    }
}

fn main() {
    println!("Hello, world");
}

#[cfg(test)]
mod tests {
    use futures::future::{self};
    use super::*;

    struct TestInvoker { }

    impl Invoker for TestInvoker {
        fn invoke(&self, name: &str, _input: &ast::Payload) -> EvalResult {
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
        // HELP(arjun): We get a warning because the result of .map is unused.
        // Also, if r.wait() produces an error, we don't fail as we should.
        // However, the obvious approach, which is r.wait().map(|x| { ast::inspect(&x) })
        // makes the borrow checker sad.
        r.wait().map(|x| {
            assert_eq!(ast::inspect(&x), "fromF");
        });
        
        
        
    }
}