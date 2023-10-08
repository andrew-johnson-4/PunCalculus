
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

pub fn infer(t: Term) -> Term {
   if let Term::Abs(ref arrows) = t {
      let mut computed_type = Vec::new();
      for (l,r) in arrows {
         if let (Term::Ascript(l,lt),Term::Ascript(r,rt)) = (l,r) {
            computed_type.push(Type::Arrow(Box::new(lt.clone()),Box::new(rt.clone())));
         } else {
            return t;
         }
      }
      if computed_type.len()==0 {
         Term::ascript(t,Type::Bottom)
      } else if computed_type.len()==1 {
         Term::ascript(t,computed_type[0].clone())
      } else {
         Term::ascript(t,Type::plural(computed_type))
      }
   } else {
      t
   }
}
