use std::collections::HashMap;
use std::fs::read;

use canjica::{eval, VarTable};
use pipoquinha::parser::atom::Atom;
use pipoquinha::parser::file::file;

pub fn start(path: &str) {
  let code = read(path).unwrap();
  let instructions = file().parse(code.as_slice());

  match instructions {
    Ok(instructions) => {
      let mut table: VarTable = HashMap::new();

      for instruction in instructions {
        match eval(instruction, &table) {
          Atom::VariableInsertion(name, value) => {
            table.insert(name, *value);
          }
          result => {
            println!("{}", result);
          }
        }
      }
    }
    Err(reason) => println!("Parsing error: {}", reason),
  }
}
