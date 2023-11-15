
use punc::term::Term;
use punc::typ::Type;
use punc::reference_solver::infer;

//single term | terminal?
//x           | False
//x y         | False
//\           | False
//\<x.x>      | False
//\<x.x><y.y> | False
//x: \bot     | True
//x: \top     | True
//x: A        | True
//x: 0 -> A   | True
//x: 1 -> A   | True
//x: A -> B   | True
//x: A + B    | True
#[test]
fn single_term() {
   assert!( !Term::var("x").is_concrete() );
   assert!( !Term::app(Term::var("x"),Term::var("y")).is_concrete() );
   assert!( !Term::abs(vec![]).is_concrete() );
   assert!( !Term::abs(vec![ (Term::var("x"),Term::var("x")) ]).is_concrete() );
   assert!( !Term::abs(vec![ (Term::var("x"),Term::var("x")), (Term::var("y"),Term::var("y")) ]).is_concrete() );
   assert!( !Term::asc(Term::var("x"),Type::Bottom).is_concrete() );
   assert!( Term::asc(Term::var("x"),Type::Top).is_concrete() );
   assert!( Term::asc(Term::var("x"),Type::named("A")).is_concrete() );
   assert!( Term::asc(Term::var("x"),Type::arrow(Type::Bottom,Type::named("A"))).is_concrete() );
   assert!( Term::asc(Term::var("x"),Type::arrow(Type::Top,Type::named("A"))).is_concrete() );
   assert!( Term::asc(Term::var("x"),Type::arrow(Type::named("A"),Type::named("B"))).is_concrete() );
   assert!( Term::asc(Term::var("x"),Type::plural(vec![Type::named("A"),Type::named("B")])).is_concrete() );
}

//\                    | \bot
//\<x:X.y:Y>           | X->Y
//\<a:A.b:B><x:X.y:Y>  | (A->B)+(X->Y)
#[test]
fn infer_arrow() {
   assert_eq!(
      infer( Term::abs(vec![]) ).typ(),
      Type::Bottom
   );
   assert_eq!(
      infer( Term::abs(vec![(
         Term::asc(Term::var("x"),Type::named("X")),
         Term::asc(Term::var("y"),Type::named("Y")),
      )]) ).typ(),
      Type::arrow(Type::named("X"),Type::named("Y"))
   );
   assert_eq!(
      infer( Term::abs(vec![(
         Term::asc(Term::var("a"),Type::named("A")),
         Term::asc(Term::var("b"),Type::named("B")),
      ),(
         Term::asc(Term::var("x"),Type::named("X")),
         Term::asc(Term::var("y"),Type::named("Y")),
      )]) ).typ(),
      Type::plural(vec![
         Type::arrow(Type::named("A"),Type::named("B")),
         Type::arrow(Type::named("X"),Type::named("Y"))
      ])
   );
}

//\<x:X.x>             | \<x:X.x:X>
#[test]
fn infer_var() {
   assert_eq!(
      infer( Term::abs(vec![(
         Term::asc(Term::var("x"),Type::named("X")),
         Term::var("x"),
      )]) ),
      Term::asc( Term::abs(vec![(
         Term::asc(Term::var("x"),Type::named("X")),
         Term::asc(Term::var("x"),Type::named("X")),
      )]), Type::arrow(Type::named("X"),Type::named("X")))
   );
}
