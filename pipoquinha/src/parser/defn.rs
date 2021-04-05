use pom::parser::*;

use crate::{atom::{Atom, atom, space}, identifier::identifier};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct UserFunction {
  pub name: String,
  pub parameters: Vec<String>,
  pub atom: Atom,
}

pub fn id_list<'a>() -> Parser<'a, u8, Vec<String>> {
    sym(b'[') * space().opt() * list(identifier(), space()) - space().opt() - sym(b']')
}

pub fn defn<'a>() -> Parser<'a, u8, UserFunction> {
    let rules = sym(b'(') * space().opt() * seq(b"defn") * space() * identifier() - space() + id_list() - space() + atom() - space().opt() - sym(b')');

    rules
    .map(|((name, parameters), atom)| UserFunction { name, parameters, atom })
    .name("Function definition")
}

#[cfg(test)]
mod tests {
use super::*;

#[test]
fn simple_function() {
    let input = b"(defn add [ x y ] (+ x y))";
    let name = String::from("add");
    let parameters = vec![String::from("x"), String::from("y")];
    let atom = atom().parse(b"(+ x y)").unwrap();
    let expected = UserFunction {
        name,
        parameters,
        atom
    };
    assert_eq!(Ok(expected), defn().parse(input));
    }
}
