/*

Copyright 2023 - Andrew Johnson

This code and all related intellectual property is available under the terms of
the attached permissive MIT license. This license is intended only to protect
the future development of the project while otherwise allowing people to use
the code and IP as they would like. Please, just be nice.

*/

#![feature(thread_id_value)]

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
   (; $i:tt $($j:tt)+ ) => { Term::app( Term::var(";"), Term::app( punc!($i), punc!($($j)*) ) ) };
   ({ $t:expr }) => { $t };
   ({ $t:expr } $($s:tt)+ ) => { Term::app( $t, punc!($($s)*) ) };
   ([ $($t:tt)+ ]) => { Term::app( Term::var("[]"), punc!($($t)*) ) };
   ([ $($t:tt)+ ] $($s:tt)+ ) => { Term::app( Term::app( Term::var("[]"), punc!($($t)*) ), punc!($($s)*) ) };
   ( $i:tt : $($t:tt)* ) => { Term::asc( punc!($i), punc_type!($($t)*) ) };
   ( $i:tt $( ( $($j:tt)* ) )+ ) => { Term::app( punc!($i), punc!($( ( $($j)* ) )*) ) };
   ( $i:ident : $($t:tt)* ) => { Term::asc( punc!($i), punc_type!($($t)*) ) };
   ( $i:ident $($j:tt)+ ) => { Term::app( Term::var(stringify!($i)), punc!($($j)*) ) };
   ( @ $i:tt $($j:tt)+ ) => { Term::app( Term::var(&("@".to_string() + stringify!($i))), punc!($($j)*) ) };
   ( % $i:tt $($j:tt)+ ) => { Term::app( Term::var(&("%".to_string() + stringify!($i))), punc!($($j)*) ) };
   ( . $i:tt $($j:tt)+ ) => { Term::app( Term::var(&(".".to_string() + stringify!($i))), punc!($($j)*) ) };
   ( = $($i:tt)+ ) => { Term::app( Term::var("="), punc!($($i)*) ) };
   ( $i:literal ) => { Term::var(stringify!($i)) };
   ( $i:ident ) => { Term::var(stringify!($i)) };
   ( @ $i:tt ) => { Term::var(&("@".to_string() + stringify!($i))) };
   ( % $i:tt ) => { Term::var(&("%".to_string() + stringify!($i))) };
   ( . $i:tt ) => { Term::var(&(".".to_string() + stringify!($i))) };
   ( $i:ident $($j:tt)+ ) => { Term::app( Term::var(stringify!($i)), punc!($($j)*) ) };
   (( $($i:tt)+ )) => { punc!($($i)*) };
}

pub mod term;
pub mod typ;
pub mod reference_solver;

pub use term::Term;
pub use typ::Type;
pub use reference_solver::infer;
