//! A DSL to transform JSON values.
mod parser;
mod syntax;
mod eval;

pub use self::syntax::{Expr, Pat};
pub use self::eval::eval;
pub use self::parser::parse;
