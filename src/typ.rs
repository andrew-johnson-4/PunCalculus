
pub enum Type {
   Top,
   Bottom,
   Named(String),
   ArrowType(Box<Type>,Box<Type>),
}
