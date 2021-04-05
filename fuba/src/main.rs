use canjica::{eval, VarTable};
use pipoquinha::{
  action::{action, Action},
  atom::Atom,
  def::Variable,
};
use std::{collections::HashMap, io};

fn main() {
  let mut table: VarTable = HashMap::new();
  println!("Welcome to the 🌽 repl!\n");

  loop {
    print!("🌽> ");

    io::Write::flush(&mut io::stdout()).expect("flush failed!");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    match action().parse(input.as_bytes()) {
      Ok(action) => match action {
        Action::Atom(a) => println!("🍿> {}", eval(a, &table)),
        Action::VariableDefinition(Variable { name, value }) => {
          let value = eval(value, &table);
          println!("🍿> #{}", name);
          table.insert(name, value);
        }
        Action::FunctionDefinition(function) => {
          let name = function.name.clone();
          let function = Atom::UserFunction(Box::new(function));

          println!("🍿> {}", function);

          table.insert(name, function);
        }
      },
      Err(reason) => {
        println!("Parsing error: {}", reason);
      }
    };
  }
}
