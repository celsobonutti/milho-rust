use std::fs::read;
use std::collections::HashMap;

use canjica::{eval, NamespaceTable};
use pipoquinha::parser::file;

pub fn start(path: &str, var_table: NamespaceTable) {
  let code = read(path).unwrap();
  let instructions = file::parser().parse(code.as_slice());

  match instructions {
    Ok(instructions) => {
      for instruction in instructions {
        match eval(instruction, var_table.clone(), &HashMap::new()) {
          result => {
            println!("{}", result);
          }
        }
      }
    }
    Err(reason) => println!("Parsing error: {}", reason),
  }
}
