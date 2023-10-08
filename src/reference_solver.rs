
use crate::term::Term;
use crate::typ::Type;

pub struct Context {
   pub parent: Option<Box<Context>>,
   pub bindings: Vec<(String,Term)>,
}

impl Context {
   pub fn rule_application(mut self, fx: &Term) {
      if let Term::App(ref f,ref _x) = fx {
         if let Term::Var(ref v) = **f {
            todo!("TODO: typecheck Var {:?}", v)
         } else if let Term::App(ref g,ref y) = **f {
            todo!("TODO: typecheck App ({:?} {:?})", g, y)
         } else if let Term::Abs(ref _arrows) = **f {
            todo!("TODO: typecheck Abs \\")
         } else {
            unreachable!("Context::rule_application.1")
         }
      } else {
         panic!("Cannot apply rule Application to {:?}", fx)
      }
   }
}
