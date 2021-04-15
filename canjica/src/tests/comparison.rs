use crate::tests::eval_with_empty_tables;

use pipoquinha::types::Atom;

#[test]
fn number_equality() {
  let literal_equal = "(.__eq__ 5 5)";
  let literal_diff = "(.__eq__ 3 5)";
  let sum_equal = "(.__eq__ (.__add__ 8 9) (.__add__ 4 13))";
  let mul_diff = "(.__eq__ (.__add__ 3 4) (.__mul__ 2 3))";

  assert_eq!(
    eval_with_empty_tables(literal_equal),
    Atom::make_boolean(true)
  );

  assert_eq!(
    eval_with_empty_tables(literal_diff),
    Atom::make_boolean(false)
  );

  assert_eq!(eval_with_empty_tables(sum_equal), Atom::make_boolean(true));

  assert_eq!(eval_with_empty_tables(mul_diff), Atom::make_boolean(false))
}

#[test]
fn string_equality() {
  assert_eq!(
    eval_with_empty_tables("(.__eq__ \"memes\" \"memes\")"),
    Atom::make_boolean(true)
  );

  assert_eq!(
    eval_with_empty_tables("(.__eq__ \"memes\" \"bozzano\")"),
    Atom::make_boolean(false)
  )
}

#[test]
fn list_equality() {
  assert_eq!(
    eval_with_empty_tables("(.__eq__ '(1 2 3 4 5 6 7 8 9 10) '(1 2 3 4 5 6 7 8 9 10))"),
    Atom::make_boolean(true)
  );

  assert_eq!(
    eval_with_empty_tables("(.__eq__ '(1 2 3 4 5) '(5 4 3 2 1))"),
    Atom::make_boolean(false)
  )
}

#[test]
fn vector_equality() {
  assert_eq!(
    eval_with_empty_tables("(.__eq__ [1 2 3 4 5 6 7 8 9 10] [1 2 3 4 5 6 7 8 9 10])"),
    Atom::make_boolean(true)
  );

  assert_eq!(
    eval_with_empty_tables("(.__eq__ [1 2 3 4 5] [5 4 3 2 1])"),
    Atom::make_boolean(false)
  )
}
