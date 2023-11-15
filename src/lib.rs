
#[macro_export]
macro_rules! punc_type {
   ( $t:ident ) => { Type::named(stringify!($t)) };
   ( $t:ident + $($s:tt)+ ) => { Type::cons(
      Type::named(stringify!($t)),
      punc_type!($($s)*)
   ) };
   ( ( $($t:tt)+ ) + $($s:tt)+ ) => { Type::cons(
      punc_type!($($t)*),
      punc_type!($($s)*)
   ) };
   (( $($t:tt)+ )) => { punc_type!($($t)*) };
   ( $l:tt -> $($r:tt)+ ) => { Type::arrow( punc_type!($l), punc_type!($($r)*) ) };
   ( ( $($l:tt)+ ) -> $($r:tt)+ ) => { Type::arrow( punc_type!($($l)*), punc_type!($($r)*) ) };
}

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
            ( Term::asc( Term::var(stringify!($x)), punc_type!($t) ), punc!($($y)*) )
         ),*
      ])
   };
   ( $i:tt : $($t:tt)* ) => { Term::asc( punc!($i), punc_type!($($t)*) ) };
   ( $i:tt ( $($j:tt)* ) ) => { Term::app( punc!($i), punc!($($j)*) ) };
   ( $i:ident : $($t:tt)* ) => { Term::asc( punc!($i), punc_type!($($t)*) ) };
   ( $i:ident ) => { Term::var(stringify!($i)) };
   ( $i:ident $($j:tt)+ ) => { Term::app( Term::var(stringify!($i)), punc!($($j)*) ) };
   (( $($i:tt)+ )) => { punc!($($i)*) };
}

pub mod term;
pub mod typ;
pub mod reference_solver;

pub use term::Term;
pub use typ::Type;
pub use reference_solver::infer;
