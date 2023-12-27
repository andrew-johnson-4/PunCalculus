use ::punc::*;

#[test]
fn var() {
   assert_eq!(
      punc!( a ),
      Term::var("a")
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
         Term::var("x"),
         Term::app(
            Term::var("y"),
            Term::var("z")
         ),
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
fn abs1() {
   assert_eq!(
      punc!( λ ),
      Term::abs(vec![])
   )
}

#[test]
fn abs2() {
   assert_eq!(
      punc!( λ[x . y] ),
      Term::abs(vec![(
         Term::var("x"),
         Term::var("y")
      )])
   )
}

#[test]
fn abs3() {
   assert_eq!(
      punc!( λ[a.b][x. y] ),
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
fn asc1() {
   assert_eq!(
      punc!( x: X ),
      Term::asc(
         Term::var("x"),
         Type::named("X")
      )
   )
}

#[test]
fn asc2() {
   assert_eq!(
      punc!( λ[x:X.y:Y] ),
      Term::abs(vec![(
         Term::asc(
            Term::var("x"),
            Type::named("X")
         ),
         Term::asc(
            Term::var("y"),
            Type::named("Y")
         )
      )])
   )
}

#[test]
fn rust1() {
   assert_eq!(
      punc!( x {Term::var("y")} ),
      Term::app(
         Term::var("x"),
         Term::var("y"),
      )
   )
}

#[test]
fn rust2() {
   assert_eq!(
      punc!( x {Term::var("y")} z ),
      Term::app(
         Term::var("x"),
         Term::app(
            Term::var("y"),
            Term::var("z"),
         )
      )
   )
}
