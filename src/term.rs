
use crate::typ::Type;

#[derive(Debug)]
pub enum Term {
   Var(String),
   Abs(Vec<(Term,Term)>), //lambdas are potentially plural, \ <a.x> <b.y> <c.z>
   App(Box<Term>,Box<Term>),
   Ascript(Box<Term>,Type) //any term can be ascripted
}

/*

f(x:X) = x
f(y:Y) = y
...

can be written as

(\ <x:X.x> <y:Y.y>) ...

*/
