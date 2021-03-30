use canjica::eval;
use pipoquinha::atom::Atom::Error;
use pipoquinha::atom::atom;
use std::io;

fn main() {
  loop {
    print!("🌽> ");

    io::Write::flush(&mut io::stdout()).expect("flush failed!");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    match atom().parse(input.as_bytes()) {
      Ok(a) => match eval(a) {
        result => println!("🍿> {}", result)
      },
      Err(reason) => {
        println!("Parsing error: {}", reason);
      }
    };
  }
}
