use std::{collections::HashMap, io};

use canjica::{eval, VarTable};
use pipoquinha::parser::atom::{atom, Atom};

pub fn start() {
  let mut table: VarTable = HashMap::new();
  println!("Welcome to the 🌽 repl!\n");

  loop {
    print!("🌽> ");

    io::Write::flush(&mut io::stdout()).expect("flush failed!");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    match atom().parse(input.as_bytes()) {
      Ok(atom) => match eval(atom, &table) {
        Atom::VariableInsertion(name, value) => {
          println!("🍿> #{}", name);

          table.insert(name, *value);
        }
        v => println!("🍿> {}", v),
      },
      Err(reason) => {
        println!("Parsing error: {}", reason);
      }
    };
  }
}
