use canjica::eval;
use pipoquinha::atom::Atom::Error;
use pipoquinha::atom::atom;
use std::io;

fn main() {
  loop {
    print!("λ> ");

    io::Write::flush(&mut io::stdout()).expect("flush failed!");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    match atom().parse(input.as_bytes()) {
      Ok(a) => match eval(a) {
        Error(reason) => println!("Eval error: {:?}", reason),
        result => println!("=> {:?}", result)
      },
      Err(reason) => {
        println!("Parsing error: {:?}", reason);
      }
    };
  }
}
