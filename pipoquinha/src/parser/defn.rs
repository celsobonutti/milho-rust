use pom::parser::*;

use crate::{atom::{expression, Expression, space}, identifier::identifier};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct UserFunction {
  pub name: String,
  pub arguments: Vec<String>,
  pub function: Expression,
}

pub fn id_list<'a>() -> Parser<'a, u8, Vec<String>> {
    sym(b'[') * space().opt() * list(identifier(), space()) - space().opt() - sym(b']')
}

pub fn defn<'a>() -> Parser<'a, u8, UserFunction> {
    let rules = sym(b'(') * space().opt() * seq(b"defn") * space() * identifier() - space() + id_list() - space() + expression() - space().opt() - sym(b')');

    rules
    .map(|((name, arguments), function)| UserFunction { name, arguments, function })
    .name("Function definition")
}

#[cfg(test)]
mod tests {
use super::*;

#[test]
fn simple_function() {
    let input = b"(defn add [ x y ] (+ x y))";
    let name = String::from("add");
    let arguments = vec![String::from("x"), String::from("y")];
    let function = expression().parse(b"(+ x y)").unwrap();
    let expected = UserFunction {
        name,
        arguments,
        function
    };
    assert_eq!(Ok(expected), defn().parse(input));
    }
}
