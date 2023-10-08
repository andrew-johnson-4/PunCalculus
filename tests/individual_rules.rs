
use punc::term::Term;
use punc::typ::Type;
use punc::reference_solver::infer;

//single term | terminal?
//x           | False
//x y         | False
//\           | False
//\<x.x>      | False
//\<x.x><y.y> | False
//x: ?        | False
//x: \bot     | True
//x: \top     | True
//x: A        | True
//x: 0 -> A   | True
//x: 1 -> A   | True
//x: A -> B   | True
//x: A + B    | True
#[test]
fn single_term() {
   assert!( !infer( Term::var("x") ).is_concrete() );
   assert!( !infer( Term::app(Term::var("x"),Term::var("y")) ).is_concrete() );
   assert!( !infer( Term::abs(vec![]) ).is_concrete() );
   assert!( !infer( Term::abs(vec![ (Term::var("x"),Term::var("x")) ]) ).is_concrete() );
   assert!( !infer( Term::abs(vec![ (Term::var("x"),Term::var("x")), (Term::var("y"),Term::var("y")) ]) ).is_concrete() );
   assert!( infer( Term::ascript(Term::var("x"),Type::Bottom) ).is_concrete() );
   assert!( infer( Term::ascript(Term::var("x"),Type::Top) ).is_concrete() );
   assert!( infer( Term::ascript(Term::var("x"),Type::named("A")) ).is_concrete() );
   assert!( infer( Term::ascript(Term::var("x"),Type::arrow(Type::Bottom,Type::named("A"))) ).is_concrete() );
   assert!( infer( Term::ascript(Term::var("x"),Type::arrow(Type::Top,Type::named("A"))) ).is_concrete() );
   assert!( infer( Term::ascript(Term::var("x"),Type::arrow(Type::named("A"),Type::named("B"))) ).is_concrete() );
   assert!( infer( Term::ascript(Term::var("x"),Type::plural(vec![Type::named("A"),Type::named("B")])) ).is_concrete() );
}
