
//plural applications can be categorized into splits, merges, carries, and narrows
//splits turn singular types into plural types
//merges turn plural types into singular types
//carries apply pluralities of functions and values
//narrows discard some lines of computation entirely

use ::punc::*;

//narrow application argument
//(f: A -> B)(x: A + C): B
#[test]
fn narrow_application_argument() {
   assert_eq!(
      punc!( (f: A -> B)(x: A + C)  ).infer().typ(),
      Type::named("B")
   )
}

//narrow application function
//(f: A -> B + C -> D)(x: A): B
#[test]
fn narrow_application_function() {
   assert_eq!(
      punc!( (f: (A -> B) + (C -> D))(x: A)  ).infer().typ(),
      Type::named("B")
   )
}

//split application
//(f: A -> B + A -> C)(x: A): B + C
#[test]
fn split_application_function() {
   assert_eq!(
      punc!( (f: (A -> B) + (A -> C))(x: A)  ).infer().typ(),
      Type::plural(vec![
         Type::named("B"),
         Type::named("C"),
      ])
   )
}

//merge application
//(f: A -> C + B -> C)(x: A + B): C
#[test]
fn merge_application_function() {
   assert_eq!(
      punc!( (f: (A -> C) + (B -> C))(x: A + B)  ).infer().typ(),
      Type::named("C"),
   )
}

//carry application
//(f: A -> B + C -> D)(x: A + C): B + D
#[test]
fn carry_application_function() {
   assert_eq!(
      punc!( (f: (A -> B) + (C -> D))(x: A + C)  ).infer().typ(),
      Type::plural(vec![
         Type::named("B"),
         Type::named("D"),
      ])
   )
}
