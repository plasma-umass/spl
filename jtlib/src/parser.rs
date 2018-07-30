extern crate nom;
extern crate serde_json;
use nom::{is_alphanumeric, is_alphabetic, recognize_float};

use std::str;
use syntax::*;

named!(id<String>,
  map_res!(
    map!(
      pair!(
        take_while!(is_alphabetic),
        take_while!(is_alphanumeric)),
      |tuple| {
        let mut vec = Vec::from(tuple.0);
        vec.extend_from_slice(tuple.1);
        vec
      }),
      String::from_utf8));

named!(empty_pat<Pat>,
  map!(tag!("$in"), |_x| Pat::Empty));

named!(dot_pat<Pat>,
  map!(
    separated_pair!(pat, char!('.'), id),
    |tuple| Pat::Dot(tuple.1, Box::new(tuple.0))));

named!(pat<Pat>,
  ws!(alt!(empty_pat | dot_pat)));


named!(
  strings<String>,
  delimited!(
    tag!("\""),
    map!(
      map_res!(
        escaped!(take_while1!(is_alphanumeric), '\\', one_of!("\"n\\")),
        str::from_utf8),
        String::from),
    tag!("\"")
  )
);

named!(number_e<Expr>,
    flat_map!(recognize_float,
      map_opt!(
        parse_to!(f64),
        |x| serde_json::Number::from_f64(x).map(|n| Expr::Number(n)))));


named!(null_e<Expr>,
  map!(tag!("null"), |_x| Expr::Null));

named!(bool_e<Expr>,
  alt!(
    map!(tag!("true"), |_x| Expr::Bool(true)) |
    map!(tag!("false"), |_x| Expr::Bool(false))));

named!(
  string_e<Expr>,
  map!(strings,
    |x| Expr::String(x.to_string())));

named!(
  array_e<Expr>,
  ws!(delimited!(
    tag!("["),
      map!(separated_list!(tag!(","), expr_e), |vec| Expr::Array(vec)),
    tag!("]"))));

named!(
  object_e<Expr>,
  ws!(map!(
    delimited!(tag!("{"), separated_list!(tag!(","), key_val), tag!("}")),
    |tuple_vec| Expr::Object(tuple_vec))));

named!(pat_e<Expr>,
  map!(pat, |p| Expr::Pat(p)));

named!(
  expr_e<Expr>,
  ws!(alt!(
    pat_e |
    string_e |
    array_e |
    object_e |
    bool_e |
    null_e |
    number_e)));

named!(
  key_val<(String, Expr)>,
  ws!(separated_pair!(strings, tag!(":"), expr_e)));

pub fn parse<I>(input: &[u8])  -> Result<Expr, nom::Err<&[u8]>> {
  expr_e(input)
    .map(|tuple| tuple.1)
}

#[test]
fn test_parse_object() {
  println!("hih\n");
  let s = &b"{ \"x\": 20 }\0";
  println!("HIHI: {:?}", expr_e(&s[..]).unwrap());
  assert!(false);
}