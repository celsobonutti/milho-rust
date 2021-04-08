use pipoquinha::parser::atom::Atom;

use crate::{eval, VarTable};

pub fn print(arguments: Vec<Atom>, variables: &VarTable) -> Atom {
  for (index, item) in arguments.into_iter().enumerate() {
    match eval(item, variables) {
      Atom::Str(string) => {
        if index == 0 {
          print!("{}", string);
        } else {
          print!(" {}", string);
        }
      }
      res => {
        if index == 0 {
          print!("{}", res);
        } else {
          print!(" {}", res);
        }
      }
    }
  }
  Atom::Nil
}
