use ::punc::*;
use std::process::Command;

#[test]
fn hello_world() {
   let program = punc!(
      (.global _start)
      (.text)
      (label _start
         (mov @1 %rax)
         (mov @1 %rdi)
         (mov @message %rsi)
         (mov @13 %rdx)
         (syscall)

         (mov @60 %rax)
         (xor %rdi %rdi)
         (syscall)
      )
      (label message
         (.ascii "Hello, world\n")
      )
   );
   program.compile("a.out");

   let mut run_program = Command::new("a.out");
   assert_eq!(
      String::from_utf8_lossy(&run_program.output().expect("failed to execute process").stdout),
      "hello world\n"
   );
}
