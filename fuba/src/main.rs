use std::{env, thread};

use canjica::{Table, NamespaceTable};
use pipoquinha::parser::file;
mod file_interpreter;
mod repl;

const STACK_SIZE: usize = 4 * 1024 * 1024;

macro_rules! include_bytes_array {
  ($( $files:literal ),+) => {
    [$(include_bytes!($files).to_vec()),+]
  }
}

fn initialize_var_table() -> NamespaceTable {
  let folder = include_bytes_array!(
    "std/basic.milho",
    "std/math.milho",
    "std/io.milho",
    "std/vector.milho",
    "std/list.milho",
    "std/comparison.milho"
  );

  let stdlib = folder
    .iter()
    .flat_map(|code_file| file::parser().parse(code_file.as_slice()).unwrap())
    .collect();

  Table::initialize(stdlib)
}

fn main() {
  let child = thread::Builder::new()
    .stack_size(STACK_SIZE)
    .spawn(|| {
      let var_table = initialize_var_table();

      let args: Vec<String> = env::args().collect();

      match args.get(1).map(|arg| arg.as_str()) {
        Some("repl") => repl::start(var_table),
        Some(path) => file_interpreter::start(path, var_table),
        _ => (),
      }
    })
    .unwrap();

  child.join().unwrap();
}
