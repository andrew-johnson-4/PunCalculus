
use crate::term::Term;
use crate::typ::Type;

//complex plural contexts are no longer needed
//just use a linked list

pub fn infer(t: Term) -> Term {
   if let Term::Abs(ref arrows) = t {
      let mut computed_type = Vec::new();
      for (l,r) in arrows {
         if let (Term::Asc(l,lt),Term::Asc(r,rt)) = (l,r) {
            computed_type.push(Type::Arrow(Box::new(lt.clone()),Box::new(rt.clone())));
         } else {
            return t;
         }
      }
      if computed_type.len()==0 {
         Term::asc(t,Type::Bottom)
      } else if computed_type.len()==1 {
         Term::asc(t,computed_type[0].clone())
      } else {
         Term::asc(t,Type::plural(computed_type))
      }
   } else {
      t
   }
}
