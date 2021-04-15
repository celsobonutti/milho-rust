use std::collections::VecDeque;

use super::Atom;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct List {
  list: VecDeque<Atom>,
}

impl List {
  pub fn new() -> List {
    List {
      list: VecDeque::new(),
    }
  }

  pub fn prepend(mut self, atom: Atom) -> Self {
    self.list.push_front(atom);
    self
  }

  pub fn append(mut self, atom: Atom) -> Self {
    self.list.push_back(atom);
    self
  }

  pub fn split(mut self) -> Option<(Atom, Self)> {
    self.list.pop_front().map(|head| (head, self))
  }

  pub fn from_vec(vec: Vec<Atom>) -> Self {
    Self {
      list: VecDeque::from(vec),
    }
  }

  pub fn len(&self) -> usize {
    self.list.len()
  }

  pub fn is_empty(&self) -> bool {
    self.list.is_empty()
  }

  pub fn as_vec(self) -> Vec<Atom> {
    Vec::from(self.list)
  }

  pub fn pop(mut self) -> Option<Atom> {
    self.list.pop_front()
  }
}

impl IntoIterator for List {
  type Item = Atom;
  type IntoIter = std::collections::vec_deque::IntoIter<Atom>;

  fn into_iter(self) -> std::collections::vec_deque::IntoIter<Atom> {
    self.list.into_iter()
  }
}

impl std::ops::Index<usize> for List {
  type Output = Atom;

  fn index(&self, index: usize) -> &Self::Output {
    &self.list[index]
  }
}

#[macro_export]
macro_rules! list {
  ($( $tail:expr ),*) => {
    List::from_vec(vec![$($tail),*])
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::id;
  use crate::types::Number;

  #[test]
  fn macro_test() {
    let empty_list = list![];
    let one_item_list = list![id!("+")];
    let multiple_items_list = list![
      id!("-"),
      Atom::Number(Number::new(5, 1).unwrap()),
      Atom::Number(Number::new(4, 1).unwrap())
    ];

    assert_eq!(empty_list.split(), None);

    let (head, tail) = one_item_list.split().unwrap();
    assert_eq!(head, id!("+"));
    assert_eq!(tail, list![]);

    let (head, tail) = multiple_items_list.split().unwrap();
    assert_eq!(head, id!("-"));
    assert_eq!(
      tail,
      list![
        Atom::Number(Number::new(5, 1).unwrap()),
        Atom::Number(Number::new(4, 1).unwrap())
      ]
    );
  }

  #[test]
  fn prepend() {
    let l1 = list![Atom::Nil, Atom::Number(Number::new(5, 1).unwrap())];
    let l2 = l1.prepend(id!("+"));

    let (head, tail) = l2.split().unwrap();

    assert_eq!(head, id!("+"));
    assert_eq!(
      tail,
      list![Atom::Nil, Atom::Number(Number::new(5, 1).unwrap())]
    );
  }

  #[test]
  fn append() {
    let l1 = list![Atom::Nil];
    let l2 = l1.append(Atom::Number(Number::new(7, 1).unwrap()));

    let (head, tail) = l2.split().unwrap();

    assert_eq!(head, Atom::Nil);
    assert_eq!(tail, list![Atom::Number(Number::new(7, 1).unwrap())]);
  }
}
