use std::env;

mod repl;
mod file_interpreter;

fn main() {
  let args: Vec<String> = env::args().collect();

  match args.get(1).map(|arg| arg.as_str()) {
    Some("repl") => repl::start(),
    Some(path) => file_interpreter::start(path),
    _ => (),
  }
}
