use std::{collections::HashMap, env, thread};

use canjica::{eval, VarTable};
use pipoquinha::parser::{atom::Atom, file::file};
mod file_interpreter;
mod repl;

const STACK_SIZE: usize = 4 * 1024 * 1024;

macro_rules! include_bytes_array {
  ($( $files:literal ),+) => {
    [$(include_bytes!($files).to_vec()),+].to_vec()
  }
}

fn load_builtins() -> VarTable {
  let mut table = HashMap::new();

  let folder = include_bytes_array!(
    "built_ins/math.milho",
    "built_ins/io.milho",
    "built_ins/vector.milho",
    "built_ins/list.milho"
  );

  for code_file in folder.iter() {
    let instructions = file().parse(code_file.as_slice()).unwrap();

    for instruction in instructions {
      match eval(instruction, &table) {
        Atom::VariableInsertion(name, value) => {
          table.insert(name, *value);
        }
        _ => (),
      }
    }
  }

  table
}

fn main() {
  let built_ins = load_builtins();

  let child = thread::Builder::new()
    .stack_size(STACK_SIZE)
    .spawn(|| {
      let args: Vec<String> = env::args().collect();

      match args.get(1).map(|arg| arg.as_str()) {
        Some("repl") => repl::start(built_ins),
        Some(path) => file_interpreter::start(path),
        _ => (),
      }
    })
    .unwrap();

  child.join().unwrap();
}
