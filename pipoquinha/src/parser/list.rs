use pom::parser::*;

use super::atom::{atom, Atom};
use super::space::space;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct List {
  pub head: Option<Atom>,
  pub tail: Vec<Atom>,
}

pub fn list_parser<'a>() -> Parser<'a, u8, List> {
  let parser = sym(b'(') * space().opt() * (atom() - space()).opt() + list(atom(), space())
    - space().opt()
    - sym(b')');

  parser.map(|(head, tail)| List { head, tail }).name("List")
}

#[cfg(test)]
mod tests {
  use super::{list_parser, List};
  use crate::atom::Atom::*;

  #[test]
  fn parse_sum_list_parser() {
    let input = b"(+ 3 3 4)";
    let output = list_parser().parse(input);

    assert_eq!(
      output,
      Ok(List {
        head: Some(Identifier(String::from("+"))),
        tail: vec![Number(3), Number(3), Number(4)]
      })
    );
  }

  #[test]
  fn parse_sum_within_sum() {
    let input = b"(+ 3 (+ 5 3))";
    let output = list_parser().parse(input);
    let internal_sum = Box::new(list_parser().parse(b"(+ 5 3)").unwrap());

    assert_eq!(
      Ok(List {
        head: Some(Identifier(String::from("+"))),
        tail: vec![Number(3), List(internal_sum)]
      }),
      output
    )
  }

  #[test]
  fn parse_function_definition() {
    let input = b"(defn add [ x y ] (+ x y))";

    let variables = vec![Identifier(String::from("x")), Identifier(String::from("y"))];
    let head = Some(Identifier(String::from("defn")));
    let function_name = Identifier(String::from("add"));
    let parameters = Vector(variables.clone());
    let expression = Box::new(List {
      head: Some(Identifier(String::from("+"))),
      tail: variables,
    });

    assert_eq!(
      Ok(List {
        head,
        tail: vec![function_name, parameters, List(expression)]
      }),
      list_parser().parse(input)
    )
  }

  #[test]
  fn parse_variable_definition() {
    let input = b"(def my_variable 250)";

    let head = Some(Identifier(String::from("def")));
    let var_name = Identifier(String::from("my_variable"));
    let value = Number(250);

    assert_eq!(
      Ok(List {
        head,
        tail: vec![var_name, value]
      }),
      list_parser().parse(input)
    );
  }

  #[test]
  fn parse_anonymous_function() {
    let input = b"(fn [ var-one var-two ] (/ var-one var-two))";

    let variables = vec![
      Identifier(String::from("var-one")),
      Identifier(String::from("var-two")),
    ];
    let head = Some(Identifier(String::from("fn")));
    let parameters = Vector(variables.clone());
    let expression = List(Box::new(List {
      head: Some(Identifier(String::from("/"))),
      tail: variables,
    }));

    assert_eq!(
      Ok(List {
        head,
        tail: vec![parameters, expression]
      }),
      list_parser().parse(input)
    );
  }

  #[test]
  fn parse_local_variables() {
    let input = b"(let [ x 2 y 9 ] (* x y 10))";

    let head = Some(Identifier(String::from("let")));
    let local_variables = Vector(vec![
      Identifier(String::from("x")),
      Number(2),
      Identifier(String::from("y")),
      Number(9),
    ]);
    let expression = Box::new(List {
      head: Some(Identifier(String::from("*"))),
      tail: vec![
        Identifier(String::from("x")),
        Identifier(String::from("y")),
        Number(10),
      ],
    });

    assert_eq!(
      Ok(List {
        head,
        tail: vec![local_variables, List(expression)]
      }),
      list_parser().parse(input)
    );
  }

  #[test]
  fn empty_list() {
    let input = b"()";

    assert_eq!(
      Ok(List {
        head: None,
        tail: vec![]
      }),
      list_parser().parse(input)
    );
  }
}
