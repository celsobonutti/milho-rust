use crate::tests::eval_with_empty_tables;

use pipoquinha::list;
use pipoquinha::types::{Atom, BuiltIn::*, List};

#[test]
fn cons() {
  assert_eq!(
    eval_with_empty_tables("(.__cons__ .__add__ '(1 2))"),
    Atom::List(list![
      Atom::BuiltIn(Add),
      Atom::unsafe_number(1, 1),
      Atom::unsafe_number(2, 1)
    ])
  )
}
