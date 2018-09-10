extern crate nom;
extern crate serde_json;
use nom::{recognize_float};
use nom::alphanumeric0;
use nom::alpha;
use nom::types::CompleteStr;
use std::str;
use json_transformers::syntax::*;

named!(id<CompleteStr,String>,
  do_parse!(
    x : alpha >>
    y : alphanumeric0 >>
    ({
      let mut s = String::from(x.0);
      s.push_str(y.0);
      s
    })));

/*
  pat ::= $in       Empty
        | pat . x   Dot(pat, x)

  pat ::= $in dots(Empty)

  dots(inner) ::= epsilon   inner
                | .x dots   dots(inner.x)
*/
named!(pat<CompleteStr,Pat>, do_parse!(
  init: preceded!(tag!("$in"), value!(Pat::Empty)) >>
  res: fold_many0!(preceded!(tag!("."), id), init,
    |acc: Pat, next: String| Pat::Dot(next, Box::new(acc))) >>
  (res)));


named!(
  strings<CompleteStr,String>,
  delimited!(
    tag!("\""),
    map!(
      escaped!(take_while1!(|x: char| x.is_alphabetic()), '\\', one_of!("\"n\\")),
      |x: CompleteStr| String::from(x.0)),
    tag!("\"")
  )
);

named!(number_e<CompleteStr,Expr>,
  flat_map!(recognize_float,
    map_opt!(
      parse_to!(f64),
      |x| serde_json::Number::from_f64(x).map(|n| Expr::Number(n)))));


named!(null_e<CompleteStr,Expr>,
  map!(tag!("null"), |_x| Expr::Null));

named!(bool_e<CompleteStr,Expr>,
  alt!(
    map!(tag!("true"), |_x| Expr::Bool(true)) |
    map!(tag!("false"), |_x| Expr::Bool(false))));

named!(
  string_e<CompleteStr,Expr>,
  map!(strings,
    |x| Expr::String(x.to_string())));

named!(
  array_e<CompleteStr,Expr>,
  ws!(delimited!(
    tag!("["),
      map!(separated_list!(tag!(","), expr_e), |vec| Expr::Array(vec)),
    tag!("]"))));

named!(
  object_e<CompleteStr,Expr>,
  ws!(map!(
    delimited!(tag!("{"), separated_list!(tag!(","), key_val), tag!("}")),
    |tuple_vec| Expr::Object(tuple_vec))));

named!(pat_e<CompleteStr,Expr>,
  map!(pat, |p| Expr::Pat(p)));

named!(
  expr_e<CompleteStr,Expr>,
  ws!(alt!(
    number_e |
    string_e |
    array_e |
    object_e |
    bool_e |
    null_e |
    pat_e
    )));

named!(
  key_val<CompleteStr,(String, Expr)>,
  ws!(separated_pair!(strings, tag!(":"), expr_e)));

named!(
  parse_expr<CompleteStr,Expr>,
  complete!(expr_e));

pub fn parse(input: &str)  -> Result<Expr, nom::Err<CompleteStr>> {
  parse_expr(CompleteStr(input))
    .map(|tuple| tuple.1)
}

#[cfg(test)]
mod tests {
  use json_transformers::syntax::*;
  use json_transformers::parser::*;
  use std::str::FromStr;

  fn parse_string(input: &str) -> Expr {
    return parse_expr(CompleteStr(input)).unwrap().1;
  }

  #[test]
  fn test_parse_num() {
    let e = parse_string("1234");
    println!("e = {:?}", e);
    // assert!(e == Expr::Number(serde_json::Number::from(1234)));
  }

  #[test]
  fn test_parse_string() {
    let e = parse_string("\"Hello\"");
    assert!(e == Expr::String("Hello".to_string()));
  }

  #[test]
  fn test_parse_object() {
    let s = parse_string("{ \"x\": \"HI\" }");
    println!("HIHI: {:?}", s);
  }

  #[test]
  fn test_parse_pattern1() {
    let e = parse_string("$in.x");
    println!("e = {:?}", e);
    assert!(e == Expr::Pat(Pat::Dot(String::from("x"), Box::new(Pat::Empty))));
  }

  #[test]
  fn test_parse_pattern2() {
    let e = parse_string("$in.x.y");
    println!("e = {:?}", e);
    assert!(e == Expr::Pat(
      Pat::Dot(String::from("y"), 
        Box::new(Pat::Dot(String::from("x"), Box::new(Pat::Empty))))));
  }

}
