use crate::tests::eval_with_empty_tables;

use pipoquinha::parser::atom::Atom;

#[test]
fn addition() {
  let empty = "(.__add__)";
  let one_item = "(.__add__ 5)";
  let two_items = "(.__add__ 2 5)";
  let negative_item = "(.__add__ 1 -8)";
  let multiple_items = "(.__add__ 1 8 -20 7 5)";

  assert_eq!(eval_with_empty_tables(empty), Atom::unsafe_number(0, 1));
  assert_eq!(eval_with_empty_tables(one_item), Atom::unsafe_number(5, 1));
  assert_eq!(eval_with_empty_tables(two_items), Atom::unsafe_number(7, 1));
  assert_eq!(
    eval_with_empty_tables(negative_item),
    Atom::unsafe_number(-7, 1)
  );
  assert_eq!(
    eval_with_empty_tables(multiple_items),
    Atom::unsafe_number(1, 1)
  );
}

#[test]
fn multiplication() {
  let empty = "(.__mul__)";
  let one_item = "(.__mul__ 5)";
  let two_items = "(.__mul__ 2 5)";
  let negative_item = "(.__mul__ 2 -8)";
  let multiple_items = "(.__mul__ 1 8 -20 7 5)";

  assert_eq!(eval_with_empty_tables(empty), Atom::unsafe_number(1, 1));
  assert_eq!(eval_with_empty_tables(one_item), Atom::unsafe_number(5, 1));
  assert_eq!(
    eval_with_empty_tables(two_items),
    Atom::unsafe_number(10, 1)
  );
  assert_eq!(
    eval_with_empty_tables(negative_item),
    Atom::unsafe_number(-16, 1)
  );
  assert_eq!(
    eval_with_empty_tables(multiple_items),
    Atom::unsafe_number(-5600, 1)
  );
}

#[test]
fn negate() {
  let negate_positive = "(.__negate__ 5)";
  let negate_negative = "(.__negate__ -5)";
  let negate_fraction = "(.__negate__ -2/3)";
  let negate_zero = "(.__negate__ 0)";

  assert_eq!(
    eval_with_empty_tables(negate_positive),
    Atom::unsafe_number(-5, 1)
  );

  assert_eq!(
    eval_with_empty_tables(negate_negative),
    Atom::unsafe_number(5, 1)
  );

  assert_eq!(
    eval_with_empty_tables(negate_fraction),
    Atom::unsafe_number(2, 3)
  );

  assert_eq!(
    eval_with_empty_tables(negate_zero),
    Atom::unsafe_number(0, 1)
  )
}

#[test]
fn invert() {
  let invert_number = "(.__invert__ 8/3)";
  let invert_zero = "(.__invert__ 0/3)";

  assert_eq!(
    eval_with_empty_tables(invert_number),
    Atom::unsafe_number(3, 8)
  );

  assert!(eval_with_empty_tables(invert_zero).is_error())
}
