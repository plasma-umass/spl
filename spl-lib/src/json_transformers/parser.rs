extern crate nom;
extern crate serde_json;
use nom::{recognize_float};
use nom::alphanumeric0;
use nom::alpha;
use nom::digit1;
use nom::types::CompleteStr;
use json_transformers::syntax::*;
use nom::Needed; // https://github.com/Geal/nom/issues/780

named!(id<CompleteStr,String>,
  do_parse!(
    x : alpha >>
    y : alphanumeric0 >>
    ({
      let mut s = String::from(x.0);
      s.push_str(y.0);
      s
    })));

named!(index<CompleteStr,usize>,
  do_parse!(
    x : digit1 >>
    ({
      x.parse().unwrap() // Should be safe to unwrap this, as otherwise it would not have parsed digit1
    })));


named!(string<CompleteStr,String>, delimited!(
  char!('\"'),
  escaped_transform!(
    none_of!("\\\"\n"),
    '\\',
    alt!(
      // TODO(arjun): Fill in other escape characters
      char!('n') => { |_| &"\n"[..] } |
      char!('\\') => { |_| &"\\"[..] })),
  char!('\"')));

/*

  Original grammar:

    pat ::= $in            Pat::Empty
          | pat.x          Pat::Pat(Select(x), pat)
          | pat[str]       Pat::Pat(Select(str), pat)
          | pat[num]       Pat::Pat(Index(n), pat)
          | pat1.map(pat2) Pat::Pat(Map(pat2), pat1)

  Eliminating left-recursion:

    pat      ::= $in pat_rest

    pat_rest ::= ε
               | .map (pat) pat_rest
               | .x pat_rest
               | [str] pat_rest
               | [num] pat_rest

*/
named!(pat<CompleteStr,Pat>, do_parse!(
  init: preceded!(tag!("$in"), value!(Pat::Empty)) >>
  res: fold_many0!(pat_rest, init,
    |acc: Pat, next: PatAtom| Pat::Pat(next, Box::new(acc))) >>
  (res)));

named!(pat_rest<CompleteStr, PatAtom>, alt!(
  delimited!(tag!("["), bracketed_rest, tag!("]")) |
    preceded!(tag!("."), dot_rest) ));

named!(dot_rest<CompleteStr, PatAtom>, alt!(
  map_p |
  select_p));

named!(map_p<CompleteStr, PatAtom>, do_parse!(
  _map : tag!("map") >>
  _lparen : tag!("(") >>
  e : expr_e >>
  _rparen : tag!(")") >>
  (PatAtom::Map(Box::new(e)))));

named!(bracketed_rest<CompleteStr, PatAtom>, alt!(
  bracketed_select_p | bracketed_index_p));

named!(bracketed_select_p<CompleteStr, PatAtom>, do_parse!(
  x : string >>
  (PatAtom::Select(x))));

named!(bracketed_index_p<CompleteStr, PatAtom>, do_parse!(
  x : index >>
  (PatAtom::Index(x))));

named!(select_p<CompleteStr, PatAtom>, do_parse!(
  x : id >>
  (PatAtom::Select(x))));

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
  map!(string,
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
  ws!(separated_pair!(string, tag!(":"), expr_e)));

named!(pub parse<CompleteStr, Expr>,
  complete!(expr_e));

#[cfg(test)]
mod tests {
  use super::*;

  fn parse_string(input: &str) -> Expr {
    return parse(CompleteStr(input)).unwrap().1;
  }

  #[test]
  fn test_parse_num() {
    let e = parse_string("1234");
    assert!(e == Expr::Number(serde_json::Number::from_f64(1234.0).unwrap()));
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
    assert!(e == Expr::Pat(
      Pat::Pat(PatAtom::Select(String::from("x")),
        Box::new(Pat::Empty))));
  }

  #[test]
  fn test_parse_pattern2() {
    let e = parse_string("$in.x.y");
    println!("e = {:?}", e);
    assert!(e == Expr::Pat(
      Pat::Pat(PatAtom::Select(String::from("y")),
        Box::new(
          Pat::Pat(PatAtom::Select(String::from("x")),
            Box::new(Pat::Empty))))));
  }

  #[test]
  fn test_parse_map() {
    let e = parse_string("$in.map($in.x)");
    assert!(e == Expr::Pat(
      Pat::Pat(
        PatAtom::Map(
          Box::new(
            Expr::Pat(
              Pat::Pat(PatAtom::Select(String::from("x")),
                Box::new(Pat::Empty))))),
        Box::new(Pat::Empty))))
  }

  #[test]
  fn test_parse_bracket() {
    let e = parse_string("$in[\"x y\"]");
    println!("e = {:?}", e);
    assert!(e == Expr::Pat(
      Pat::Pat(PatAtom::Select(String::from("x y")),
        Box::new(Pat::Empty))));
  }

  #[test]
  fn test_parse_bracket_escaped() {
    let e = parse_string("$in[\"x \\ny\"]");
    println!("e = {:?}", e);
    assert!(e == Expr::Pat(
      Pat::Pat(PatAtom::Select(String::from("x \ny")),
        Box::new(Pat::Empty))));
  }

}
