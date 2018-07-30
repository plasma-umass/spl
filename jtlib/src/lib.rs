extern crate serde_json;
#[macro_use]
extern crate nom;

pub mod syntax;
pub mod parser;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
