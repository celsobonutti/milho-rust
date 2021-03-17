use canjica::{eval};
use pipoquinha::{expression, Atom};
use std::io;

fn main() {
  loop {
    print!("λ> ");

    io::Write::flush(&mut io::stdout()).expect("flush failed!");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    match expression().parse(input.as_bytes()) {
      Ok(expr) => match eval(&Atom::Expr(expr)) {
        Ok(result) => {
          println!("=> {:?}", result)
        }
        Err(reason) => {
          println!("Evaluation error: {:?}", reason)
        }
      },
      Err(reason) => {
        println!("Parsing error: {:?}", reason);
      }
    };
  }
}
