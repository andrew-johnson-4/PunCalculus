
#[derive(Debug)]
pub enum Type {
   Top,
   Bottom,
   Named(String),
   Arrow(Box<Type>,Box<Type>),
   Plural(Vec<Type>),
}

impl Type {
   pub fn named(v: &str) -> Type {
      Type::Named(v.to_string())
   }
   pub fn arrow(l: Type, r: Type) -> Type {
      Type::Arrow(Box::new(l), Box::new(r))
   }
   pub fn plural(ts: Vec<Type>) -> Type {
      Type::Plural(ts)
   }
}
