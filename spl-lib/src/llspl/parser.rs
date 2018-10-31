//! Concrete syntax parser for LLSPL.
//!
//! expr ::= seq
//!
//! seq ::= atom (";"  atom)*
//!
//! atom ::= pure
//!        | project
//!        | download
//!        | split
//!        | if
//!        | parens
//!
//! parens ::= "(" seq ")"
//!
//! pure ::= "pure" id
//!
//! project ::= "project" transformer // Defined in json_transformers::parse
//!
//! download ::= "download" url
//! 
//! split ::= "split" parens
//! 
//! if ::= "if" parens "{" seq "}" "else" "{" seq "}"

/* TODO(arjun):

    Fetch(String),
*/

extern crate nom;
extern crate serde_json;
use nom::types::CompleteStr;
use json_transformers;
use super::syntax::*;

named!(string_atom<CompleteStr,String>,
do_parse!(
        s : delimited!(char!('"'), is_not!("\""), char!('"')) >>
        (s.to_string())
));

named!(pure_e<CompleteStr,Expr>,
    do_parse!(
        _reserved : tag!("pure") >>
        name : ws!(string_atom) >>
        (Expr::Pure(name.to_string()))));

named!(download_e<CompleteStr, Expr>,
    do_parse!(
        _reserved : tag!("download") >>
        url : ws!(json_transformers::parse) >>
        (Expr::Download(url))));

named!(project_e<CompleteStr,Expr>,
    map!(preceded!(tag!("project"), ws!(json_transformers::parse)),
        |exp: json_transformers::Expr| Expr::Project(exp)));

named!(split_e<CompleteStr,Expr>,
    do_parse!(
        _reserved : tag!("split") >>
        e: ws!(parens_e) >>
        (Expr::Split(Box::new(e)))));

named!(if_e<CompleteStr,Expr>,
    do_parse!(
        _if : tag!("if") >>
        cond : ws!(parens_e) >> 
        t_branch : ws!(delimited!(char!('{'), seq_e, char!('}'))) >>
        _else : tag!("else") >>
        f_branch : ws!(delimited!(char!('{'), seq_e, char!('}'))) >>
        (Expr::If(Box::new(cond), Box::new(t_branch), Box::new(f_branch)))));

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
        split_e |
        if_e |
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
        assert!(parse_string("pure \"foo-bar\"") ==
            Expr::Pure("foo-bar".to_string()));
    }

    #[test]
    fn test_download() {
        assert!(parse_string("download \"http://foo.bar\"") ==
                Expr::Download(json_transformers::parse_string("\"http://foo.bar\"")));
    }

    #[test]
    fn test_project() {
        match parse_string("project $in.x") {
            Expr::Project(_transformer) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn test_split() {
        assert!(parse_string("split (pure \"foo\")")  ==
            Expr::Split(Box::new(Expr::Pure("foo".to_string()))));
    }

    #[test]
    fn test_if() {
        assert!(parse_string("if (pure \"foo\") { pure \"bar\" } else { pure \"baz\" }")  ==
            Expr::If(Box::new(Expr::Pure("foo".to_string())),
                Box::new(Expr::Pure("bar".to_string())),
                Box::new(Expr::Pure("baz".to_string()))));
    }

    #[test]
    fn test_project_seq_pure() {
        match parse_string("project $in.x; pure \"f\"") {
            Expr::Seq(_e1, _e2) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn test_seq() {
        assert!(parse_string("pure \"f\"; pure \"g\"") ==
            Expr::Seq(Box::new(Expr::Pure("f".to_string())),
                    Box::new(Expr::Pure("g".to_string()))));
    }

    #[test]
    fn test_seq_download() {
        assert!(parse_string("download \"f\"; download \"g\"") ==
            Expr::Seq(Box::new(Expr::Download(json_transformers::parse_string("\"f\""))),
                    Box::new(Expr::Download(json_transformers::parse_string("\"g\"")))));
    }


    #[test]
    fn test_parens() {
        assert!(parse_string("(pure \"f\"; pure \"g\"); pure \"h\"") ==
        Expr::Seq(
            Box::new(
                Expr::Seq(Box::new(Expr::Pure("f".to_string())),
                            Box::new(Expr::Pure("g".to_string())))),
            Box::new(Expr::Pure("h".to_string()))));
    }

    #[test]
    fn test_pure_in_parens() {
        assert!(parse_string("(pure \"foo\")") ==
            Expr::Pure("foo".to_string()));
    }

    #[test]
    fn test_download_in_parens() {
        assert!(parse_string("(download \"http://foo.bar\")") ==
        Expr::Download(json_transformers::parse_string("\"http://foo.bar\"")));
    }

    #[test]
    fn test_download_json() {
        match parse_string("download $in.url") {
            Expr::Download(_json) => assert!(true),
            _ => assert!(false)
        }
    }
}
