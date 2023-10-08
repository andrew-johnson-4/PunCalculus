
pub enum Term {
   Variable(String),
   Abs(String,Box<Term>),
   App(Box<Term>,Box<Term>),
}
