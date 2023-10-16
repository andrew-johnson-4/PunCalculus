
#[macro_export]
macro_rules! punc {
   ($($i:ident)* $j:ident) => { };
}

pub mod term;
pub mod typ;
pub mod reference_solver;

pub use term::Term;
pub use typ::Type;
pub use reference_solver::infer;
