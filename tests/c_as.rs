use ::punc::*;
use std::process::Command;

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
         (.len offset @len @msg)
      )
   );
   program.compile("a.out");

   let mut run_program = Command::new("a.out");
   assert_eq!(
      String::from_utf8_lossy(&run_program.output().expect("failed to execute process").stdout),
      "hello world\n"
   );
}
