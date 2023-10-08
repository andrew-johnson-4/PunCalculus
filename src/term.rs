
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

impl Term {
   pub fn is_concrete(&self) -> bool {
      if let Term::Ascript(ref t,_) = self {
         match **t {
            Term::Var(_) => true,
            Term::Abs(ref arrows) => todo!("Term::is_concrete(Term::Abs)"),
            Term::App(ref f,ref x) => f.is_concrete() && x.is_concrete(),
            Term::Ascript(_,_) => panic!("Term is double ascripted: {:?}", self),
         }
      } else {
         false
      }
   }
   pub fn var(s: &str) -> Term {
      Term::Var(s.to_string())
   }
   pub fn abs(ts: Vec<(Term,Term)>) -> Term {
      Term::Abs(ts)
   }
   pub fn app(f: Term, x: Term) -> Term {
      Term::App(Box::new(f),Box::new(x))
   }
   pub fn ascript(t: Term, tt: Type) -> Term {
      Term::Ascript(Box::new(t), tt)
   }
}
