
use crate::term::Term;
use crate::typ::Type;
use im_lists::list::List;

//complex plural contexts are no longer needed
//just use a linked list

pub fn infer(mut t: Term) -> Term {
   let mut next_t = infer_one_pass(List::new(), &t);
   while t != next_t {
      t = next_t;
      next_t = infer_one_pass(List::new(), &t);
   }
   t
}

pub fn infer_one_pass(mut ctx: List<(String,Type)>, t: &Term) -> Term {
   //rule: infer abstraction
   if let Term::Abs(ps,tt) = t {
      let mut next_ps = Vec::new();
      let mut next_tt = Vec::new();
      for (lhs,rhs) in ps {
         let mut at = Type::Bottom;
         if let Term::Var(lv,lt) = lhs {
            at = lt.clone();
            ctx = List::cons( (lv.clone(),lt.clone()), ctx );
         }
         let next_r = infer_one_pass(ctx.clone(), rhs);
         if at != Type::Bottom && next_r.typ() != Type::Bottom {
            next_tt.push(Type::Arrow(
               Box::new(at),
               Box::new(next_r.typ()),
            ));
         }
         next_ps.push((
            lhs.clone(),
            next_r.clone()
         ));
      }
      if next_ps.len() == next_tt.len() {
         if next_tt.len()==1 {
            Term::Abs(next_ps, next_tt[0].clone())
         } else {
            Term::Abs(next_ps, Type::Plural(next_tt))
         }
      } else {
         Term::Abs(next_ps,tt.clone())
      }
   } else {
      t.clone()
   }
}
