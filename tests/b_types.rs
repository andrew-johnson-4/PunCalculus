use ::punc::*;

#[test]
fn structural_equality() {

   assert_eq!(
      punc!( a: A ).typ(),
      Type::named("A")
   );
   assert_eq!(
      punc!( b: B ).typ(),
      Type::named("B")
   );
   assert_eq!(
      punc!( c: C ).typ(),
      Type::named("C")
   );

   assert_eq!(
      punc!( a: A+B+C ).typ(),
      Type::plural(vec![
         Type::named("A"),
         Type::named("B"),
         Type::named("C")
      ])
   );

   assert_eq!(
      punc!( a: (A->B) ).typ(),
      Type::arrow(
         Type::named("A"),
         Type::named("B")
      )
   );

}
