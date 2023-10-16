
#[macro_export]
macro_rules! punc {
   (( $($i:tt)+ )) => { punc!($($i)*) };
   ( $i:ident ) => { Term::var(stringify!($i)) };
   ( $i:ident $($j:tt)+ ) => { Term::app( Term::var(stringify!($i)), punc!($($j)*) ) };
}

pub mod term;
pub mod typ;
pub mod reference_solver;

pub use term::Term;
pub use typ::Type;
pub use reference_solver::infer;
