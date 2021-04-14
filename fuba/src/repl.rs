use std::{collections::HashMap, io};

use canjica::{eval, NamespaceTable};
use pipoquinha::parser::atom;

pub fn start(var_table: NamespaceTable) {
  println!("Welcome to the 🌽 repl!\n");

  loop {
    print!("🌽> ");

    io::Write::flush(&mut io::stdout()).expect("flush failed!");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    match atom::parser().parse(input.as_bytes()) {
      Ok(atom) => match eval(atom, var_table.clone(), &HashMap::new()) {
        v => println!("🍿> {}", v),
      },
      Err(reason) => {
        println!("Parsing error: {}", reason);
      }
    };
  }
}
