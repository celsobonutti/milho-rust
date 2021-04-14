#[cfg(test)]
mod tests {
  use std::collections::HashMap;

  use crate::{NamespaceTable, Table, VarTable};

  use crate::eval::*;
  use pipoquinha::types::number::Number;
  use pipoquinha::parser::{atom::Atom, unsafe_parse_atom};

  fn make_empty_tables() -> (NamespaceTable, VarTable) {
    (Table::initialize(vec![]), HashMap::new())
  }

  fn eval_with_empty_tables(atom: Atom) -> Atom {
    let (namespace, local) = make_empty_tables();
    eval(atom, namespace, &local)
  }

  #[test]
  fn addition() {
    let empty = unsafe_parse_atom("(.__add__)");
    let one_item = unsafe_parse_atom("(.__add__ 5)");
    let two_items = unsafe_parse_atom("(.__add__ 2 5)");
    let negative_item = unsafe_parse_atom("(.__add__ 1 -8)");
    let multiple_items = unsafe_parse_atom("(.__add__ 1 8 -20 7 5)");

    assert_eq!(eval_with_empty_tables(empty), Atom::Number(Number::new(0, 1).unwrap()));
    assert_eq!(eval_with_empty_tables(one_item), Atom::Number(Number::new(5, 1).unwrap()));
    assert_eq!(eval_with_empty_tables(two_items), Atom::Number(Number::new(7, 1).unwrap()));
    assert_eq!(eval_with_empty_tables(negative_item), Atom::Number(Number::new(-7, 1).unwrap()));
    assert_eq!(eval_with_empty_tables(multiple_items), Atom::Number(Number::new(1, 1).unwrap()));
  }

  #[test]
  fn multiplication() {
    let empty = unsafe_parse_atom("(.__mul__)");
    let one_item = unsafe_parse_atom("(.__mul__ 5)");
    let two_items = unsafe_parse_atom("(.__mul__ 2 5)");
    let negative_item = unsafe_parse_atom("(.__mul__ 2 -8)");
    let multiple_items = unsafe_parse_atom("(.__mul__ 1 8 -20 7 5)");

    assert_eq!(eval_with_empty_tables(empty), Atom::Number(Number::new(1, 1).unwrap()));
    assert_eq!(eval_with_empty_tables(one_item), Atom::Number(Number::new(5, 1).unwrap()));
    assert_eq!(eval_with_empty_tables(two_items), Atom::Number(Number::new(10, 1).unwrap()));
    assert_eq!(eval_with_empty_tables(negative_item), Atom::Number(Number::new(-16, 1).unwrap()));
    assert_eq!(eval_with_empty_tables(multiple_items), Atom::Number(Number::new(-5600, 1).unwrap()));
  }
}
