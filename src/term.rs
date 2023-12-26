
use crate::typ::Type;
use crate::reference_solver::infer;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

#[derive(Debug,PartialEq,Eq,Clone)]
pub enum Term {
   Var(String,Type),
   Abs(Vec<(Term,Term)>,Type), //lambdas are potentially plural, \ <a.x> <b.y> <c.z>
   App(Box<Term>,Box<Term>,Type),
}

const BINOP: [&'static str; 3] = ["mov", "movl", "xor"];
const UNOP: [&'static str; 1] = ["int"];
const ZOP: [&'static str; 1] = ["syscall"];

impl Term {
   pub fn to_string(&self) -> String {
      match self {
         Term::Var(v,_vt) => { v.clone() },
         Term::Abs(_ps,_pt) => { "λ".to_string() },
         Term::App(l,r,_t) => { format!("({} {})", l.to_string(), r.to_string()) },
      }
   }
   pub fn typ(&self) -> Type {
      match self {
         Term::Var(_,tt) => tt.clone(),
         Term::Abs(_,tt) => tt.clone(),
         Term::App(_,_,tt) => tt.clone(),
      }
   }
   pub fn is_concrete(&self) -> bool {
      self.typ() != Type::Bottom
   }
   pub fn infer(&self) -> Term {
      infer(self.clone())
   }
   pub fn var(s: &str) -> Term {
      Term::Var(s.to_string(),Type::Bottom)
   }
   pub fn abs(ts: Vec<(Term,Term)>) -> Term {
      Term::Abs(ts,Type::Bottom)
   }
   pub fn app(f: Term, x: Term) -> Term {
      Term::App(Box::new(f),Box::new(x),Type::Bottom)
   }
   pub fn asc(t: Term, tt: Type) -> Term {
      match t {
         Term::Var(v,_) => Term::Var(v,tt),
         Term::Abs(a,_) => Term::Abs(a,tt),
         Term::App(f,x,_) => Term::App(f,x,tt),
      }
   }
   pub fn as_assembly(&self) -> String {
      if let Term::App(dir,r,_) = self {

         //sections
         if let Term::Var(ref dir,_) = **dir {
         if dir==".ascii" {
            return format!("\t.ascii {}\n", r.as_assembly());
         }}
         if let Term::Var(ref dir,_) = **dir {
         if dir.starts_with(".") {
            return format!("\t{}\n{}", dir.to_string(), r.as_assembly());
         }}
         if let Term::App(ref ldir,ref label,_) = **dir {
         if let Term::Var(ref ldir,_) = **ldir {
         if ldir.starts_with(".") {
            return format!("\t{} {}\n{}", ldir.to_string(), label.to_string(), r.as_assembly());
         }}}

         //labels
         if let Term::Var(ref dir,_) = **dir {
         if dir=="label" {
         if let Term::Var(ref ln,_) = **r {
            return format!("{}:\n", ln.to_string());
         }}}
         if let Term::Var(ref dir,_) = **dir {
         if dir=="label" {
         if let Term::App(ref ln,ref inner,_) = **r {
         if let Term::Var(ref ln,_) = **ln {
            return format!("{}:\n{}", ln.to_string(), inner.as_assembly());
         }}}}
         if let Term::App(ref ldir,ref label,_) = **dir {
         if let Term::Var(ref ldir,_) = **ldir {
         if ldir=="label" {
         if let Term::App(ref ln,ref inner,_) = **label {
         if let Term::Var(ref ln,_) = **ln {
            return format!("{}:\n{}\n{}", ln.to_string(), inner.as_assembly(), r.as_assembly());
         }}}}}

         //instructions
         if let Term::Var(ref dir,_) = **dir {
         if ZOP.contains(&dir.as_str()) {
            return format!("\t{}\n{}", dir, r.as_assembly());
         }}
         if let Term::Var(ref dir,_) = **dir {
         if UNOP.contains(&dir.as_str()) {
            return format!("\t{} {}\n", dir, r.as_assembly());
         }}
         if let Term::Var(ref dir,_) = **dir {
         if BINOP.contains(&dir.as_str()) {
         if let Term::App(ref a1,ref a2,_) = **r {
            return format!("\t{} {}, {}\n", dir, a1.as_assembly(), a2.as_assembly());
         }}}
         if let Term::App(ref ldir,ref inner,_) = **dir {
         if let Term::Var(ref ldir,_) = **ldir {
         if UNOP.contains(&ldir.as_str()) {
            return format!("\t{} {}\n{}", ldir, inner.as_assembly(), r.as_assembly());
         }}}
         if let Term::App(ref ldir,ref inner,_) = **dir {
         if let Term::Var(ref ldir,_) = **ldir {
         if BINOP.contains(&ldir.as_str()) {
         if let Term::App(ref a1,ref a2,_) = **inner {
            return format!("\t{} {}, {}\n{}", ldir, a1.as_assembly(), a2.as_assembly(), r.as_assembly());
         }}}}
      }
      if let Term::Var(v, _) = self {
         if ZOP.contains(&v.as_str()) {
            return format!("\t{}\n", v);
         }
         if v.starts_with("@") {
            return format!("${}", &v[1..]);
         }
         return v.clone();
      }
      panic!("unknown directive: {}", self.to_string());
   }
   pub fn compile(&self, cfg: &str) {
      let assembly = self.as_assembly();
      if cfg.ends_with(".s") {
         let mut file = File::create(cfg).expect("Could not create file in Term::compile");
         file.write_all(assembly.as_bytes()).expect("Could not write to file in Term::compile");
      } else {
         let mut file = File::create("tmp.s").expect("Could not create file in Term::compile");
         file.write_all(assembly.as_bytes()).expect("Could not write to file in Term::compile");

         Command::new("as")
                 .arg("tmp.s")
                 .arg("-o")
                 .arg("tmp.o")
                 .spawn()
                 .expect("Could not run assembler in Term::compile")
                 .wait()
                 .expect("Could not wait for assembler in Term::compile");

         Command::new("ld")
                 .arg("-s")
                 .arg("-o")
                 .arg(cfg)
                 .arg("tmp.o")
                 .spawn()
                 .expect("Could not run linker in Term::compile")
                 .wait()
                 .expect("Could not wait for linker in Term::compile");
      }
   }
}
