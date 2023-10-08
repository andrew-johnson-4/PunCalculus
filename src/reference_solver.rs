
use crate::typ::Type;

pub struct Context {
   pub parent: Option<Box<Context>>,
   pub bindings: Vec<(String,Type)>,
}
