
#[macro_export]
macro_rules! punc {
   ( λ $( [ $x:ident . $($y:tt)* ] )* ) => {
      Term::abs(vec![
         $(
            ( Term::var(stringify!($x)), punc!($($y)*) )
         ),*
      ])
   };
   ( λ $( [ $x:ident : $t:ident . $($y:tt)* ] )* ) => {
      Term::abs(vec![
         $(
            ( Term::asc( Term::var(stringify!($x)), Type::named(stringify!($t)) ), punc!($($y)*) )
         ),*
      ])
   };
   (( $($i:tt)+ ) : $t:ty ) => { Term::asc( punc!($($i)*), Type::named(stringify!($t)) ) };
   (( $($i:tt)+ )) => { punc!($($i)*) };
   ( $i:tt : $t:ty ) => { Term::asc( punc!($i), Type::named(stringify!($t)) ) };
   ( $i:ident ) => { Term::var(stringify!($i)) };
   ( $i:ident $($j:tt)+ ) => { Term::app( Term::var(stringify!($i)), punc!($($j)*) ) };
}

pub mod term;
pub mod typ;
pub mod reference_solver;

pub use term::Term;
pub use typ::Type;
pub use reference_solver::infer;
