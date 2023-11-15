
use crate::typ::Type;

#[derive(Debug,PartialEq,Eq,Clone)]
pub enum Term {
   Var(String),
   Abs(Vec<(Term,Term)>), //lambdas are potentially plural, \ <a.x> <b.y> <c.z>
   App(Box<Term>,Box<Term>),
   Asc(Box<Term>,Type) //any term can be ascripted
}

/*

f(x:X) = x
f(y:Y) = y
...

can be written as

(\ <x:X.x> <y:Y.y>) ...

*/

impl Term {
   pub fn typ(&self) -> Type {
      if let Term::Asc(_,ref tt) = self {
         tt.clone()
      } else {
         Type::Bottom
      }
   }
   pub fn is_concrete(&self) -> bool {
      if let Term::Asc(ref t,_) = self {
         match **t {
            Term::Var(_) => true,
            Term::Abs(ref arrows) => arrows.iter().all(|(l,r)| l.is_concrete() && r.is_concrete()),
            Term::App(ref f,ref x) => f.is_concrete() && x.is_concrete(),
            Term::Asc(_,_) => panic!("Term is double ascripted: {:?}", self),
         }
      } else {
         false
      }
   }
   pub fn infer(&self) -> Term {
      self.clone()
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
   pub fn asc(t: Term, tt: Type) -> Term {
      Term::Asc(Box::new(t), tt)
   }
}
