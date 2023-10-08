
use crate::typ::Type;

#[derive(Debug)]
pub enum Term {
   Var(String),
   Abs(Vec<(Term,Term)>), //lambdas are potentially plural, \ <a.x> <b.y> <c.z>
   App(Box<Term>,Box<Term>),
   Ascript(Box<Term>,Type) //any term can be ascripted
}
