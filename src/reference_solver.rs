
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

pub fn infer_one_pass(ctx: List<(String,Term)>, t: &Term) -> Term {
   t.clone()
}
