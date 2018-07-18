extern crate hyper;
extern crate std;

// Inputs and outputs of SPL programs
pub enum Payload {
    Chunk(hyper::Chunk)
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