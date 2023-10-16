use ::punc::{punc,Term,Type};

#[test]
fn var() {
   assert_eq!(
      punc!( a ),
      Term::var("a")
   )
}

/*
#[test]
fn abs1() {
   assert_eq!(
      punc!( λ ),
      Term::abs(vec![])
   )
}

#[test]
fn abs2() {
   assert_eq!(
      punc!( λ<x. y> ),
      Term::abs(vec![(
         Term::var("x"),
         Term::var("y")
      )])
   )
}

#[test]
fn abs3() {
   assert_eq!(
      punc!( λ<a.b><x. y> ),
      Term::abs(vec![(
         Term::var("a"),
         Term::var("b")
      ),(
         Term::var("x"),
         Term::var("y")
      )])
   )
}

#[test]
fn app1() {
   assert_eq!(
      punc!( x y ),
      Term::app(
         Term::var("x"),
         Term::var("y")
      )
   )
}

#[test]
fn app2() {
   assert_eq!(
      punc!( x y z ),
      Term::app(
         Term::app(
            Term::var("x"),
            Term::var("y"),
         ),
         Term::var("z")
      )
   )
}

#[test]
fn app3() {
   assert_eq!(
      punc!( x (y z) ),
      Term::app(
         Term::var("x"),
         Term::app(
            Term::var("y"),
            Term::var("z")
         ),
      )
   )
}

#[test]
fn asc() {
   assert_eq!(
      punc!( x: X ),
      Term::asc(
         Term::var("x"),
         Type::named("X")
      )
   )
}
*/
