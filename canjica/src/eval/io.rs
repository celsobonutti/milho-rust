use std::io;

use crate::{eval, NamespaceTable};
use pipoquinha::parser::atom;
use pipoquinha::types::Atom;

pub fn print(arguments: Vec<Atom>, namespace_variables: NamespaceTable) -> Atom {
  for (index, item) in arguments.into_iter().enumerate() {
    match eval(item, namespace_variables.clone()) {
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

pub fn read(arguments: Vec<Atom>) -> Atom {
  if arguments.is_empty() {
    io::Write::flush(&mut io::stdout()).expect("flush failed!");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let a = atom::parser().parse(input.as_bytes());

    match a {
      Err(_) => Atom::Str(input),
      Ok(a) => a,
    }
  } else {
    Atom::Error("Wrong number of arguments for 'read': it takes no arguments".to_string())
  }
}
