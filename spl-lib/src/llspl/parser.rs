//! Concrete syntax parser for LLSPL.
//!
//! expr ::= seq
//!
//! seq ::= atom (";"  atom)*
//!
//! atom ::= pure
//!        | project
//!        | download
//!        | parens
//!
//! parens ::= "(" seq ")"
//!
//! pure ::= "pure" id
//!
//! project ::= "project" transformer // Defined in json_transformers::parse
//!
//! download ::= "download" url

/* TODO(arjun):

    Fetch(String),
    Split(Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>)
*/

extern crate nom;
extern crate serde_json;
use nom::alphanumeric1;
use nom::types::CompleteStr;
use json_transformers;
use super::syntax::*;

named!(pure_e<CompleteStr,Expr>,
    do_parse!(
        _reserved : tag!("pure") >>
        name : ws!(alphanumeric1) >>
        (Expr::Pure(name.to_string()))));

named!(download_e<CompleteStr, Expr>,
    do_parse!(
        _reserved : tag!("download") >>
        url : ws!(take_until!(";")) >>
        (Expr::Download(url.to_string()))));

named!(project_e<CompleteStr,Expr>,
    map!(preceded!(tag!("project"), ws!(json_transformers::parse)),
        |exp: json_transformers::Expr| Expr::Project(exp)));

named!(parens_e<CompleteStr,Expr>,
    delimited!(tag!("("), seq_e, tag!(")")));

named!(seq_e<CompleteStr, Expr>,
    do_parse!(
        init: atom_e >>
        res: fold_many0!(
            preceded!(tag!(";"), atom_e),
            init,
            |acc: Expr, next: Expr| Expr::Seq(Box::new(acc), Box::new(next))) >>
        (res)));

named!(atom_e<CompleteStr,Expr>,
    ws!(alt!(
        pure_e |
        download_e |
        project_e |
        parens_e)));

named!(parse_e<CompleteStr,Expr>, do_parse!(
    e : seq_e >>
    _eof : eof!() >>
    (e)));

pub fn parse(input: &str)  ->
    Result<(CompleteStr, Expr), nom::Err<CompleteStr>> {
  parse_e(CompleteStr(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_string(input: &str) -> Expr {
        let r = parse(input);
        println!("Result = {:?}", r);
        return r.unwrap().1;
    }

    #[test]
    fn test_pure() {
        assert!(parse_string("pure foo") ==
            Expr::Pure("foo".to_string()));
    }

    #[test]
    fn test_project() {
        match parse_string("project $in.x") {
            Expr::Project(_transformer) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn test_project_seq_pure() {
        match parse_string("project $in.x; pure f") {
            Expr::Seq(_e1, _e2) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn test_seq() {
        assert!(parse_string("pure f; pure g") ==
            Expr::Seq(Box::new(Expr::Pure("f".to_string())),
                    Box::new(Expr::Pure("g".to_string()))));
    }

    #[test]
    fn test_parens() {
        assert!(parse_string("(pure f; pure g); pure h") ==
        Expr::Seq(
            Box::new(
                Expr::Seq(Box::new(Expr::Pure("f".to_string())),
                            Box::new(Expr::Pure("g".to_string())))),
            Box::new(Expr::Pure("h".to_string()))));
    }
}
