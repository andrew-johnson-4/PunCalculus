
use crate::ast;
use regex::Regex;

pub fn prs(filename: &str, contents: &str) -> ast::Term {
   let whitespace = Regex::new(r"[ \t\n]").unwrap();
   let variable = Regex::new(r"[_a-zA-Z][_a-zA-Z0-9]*").unwrap();
   let literal = Regex::new(r"[^ ]+").unwrap();
   ast::Term::Literal("".to_string())
}
