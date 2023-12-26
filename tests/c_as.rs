use ::punc::*;

#[test]
fn hello_world() {
   let program = punc!(
      (.global _start)
      (.text)
      (label _start
         (movl @4 %eax) 
         (movl @1 %ebx)
         (movl @msg %ecx)
         (movl @len %edx)
         (int @0x80)
         (movl @1 %eax)
         (movl @0 %ebx)
         (int @0x80)
      )
      (.data)
      (label msg
         (.ascii "Hello, world!\n")
         (len = offset @len @msg)
      )
   );
}
