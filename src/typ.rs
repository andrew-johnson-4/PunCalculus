
#[derive(Debug)]
pub enum Type {
   Placeholder,
   Top,
   Bottom,
   Named(String),
   ArrowType(Box<Type>,Box<Type>),
   Plural(Vec<Type>),
}
