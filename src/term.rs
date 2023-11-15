
use crate::typ::Type;
use crate::reference_solver::infer;

#[derive(Debug,PartialEq,Eq,Clone)]
pub enum Term {
   Var(String,Type),
   Abs(Vec<(Term,Term)>,Type), //lambdas are potentially plural, \ <a.x> <b.y> <c.z>
   App(Box<Term>,Box<Term>,Type),
}

impl Term {
   pub fn typ(&self) -> Type {
      match self {
         Term::Var(_,tt) => tt.clone(),
         Term::Abs(_,tt) => tt.clone(),
         Term::App(_,_,tt) => tt.clone(),
      }
   }
   pub fn is_concrete(&self) -> bool {
      self.typ() != Type::Bottom
   }
   pub fn infer(&self) -> Term {
      infer(self.clone())
   }
   pub fn var(s: &str) -> Term {
      Term::Var(s.to_string(),Type::Bottom)
   }
   pub fn abs(ts: Vec<(Term,Term)>) -> Term {
      Term::Abs(ts,Type::Bottom)
   }
   pub fn app(f: Term, x: Term) -> Term {
      Term::App(Box::new(f),Box::new(x),Type::Bottom)
   }
   pub fn asc(t: Term, tt: Type) -> Term {
      match t {
         Term::Var(v,_) => Term::Var(v,tt),
         Term::Abs(a,_) => Term::Abs(a,tt),
         Term::App(f,x,_) => Term::App(f,x,tt),
      }
   }
}
