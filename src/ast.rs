
pub enum Term {
   Variable(String),
   Literal(String),
   App(Vec<Term>),
   Abs(Vec<Term>,Vec<Term>)
}
