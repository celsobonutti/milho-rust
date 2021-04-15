use super::Atom;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct List {
  pub head: Option<Atom>,
  pub tail: Vec<Atom>,
}

impl List {
  pub fn new() -> List {
    List {
      head: None,
      tail: vec![],
    }
  }

  pub fn prepend(self, atom: Atom) -> List {
    let List { head, mut tail } = self;

    if let Some(h) = head {
      let mut new_tail = Vec::new();

      new_tail.push(h);
      new_tail.append(&mut tail);

      List {
        head: Some(atom),
        tail: new_tail,
      }
    } else {
      List {
        head: Some(atom),
        tail,
      }
    }
  }

  pub fn append(self, atom: Atom) -> List {
    let List { head, mut tail } = self;

    tail.push(atom);

    List { head, tail }
  }

  pub fn split(self) -> Option<(Atom, List)> {
    let List { head, tail } = self;

    head.map(|h| (h, List::from_vec(tail)))
  }

  pub fn from_vec(vec: Vec<Atom>) -> List {
    if vec.len() == 0 {
      List {
        head: None,
        tail: vec,
      }
    } else {
      let mut vec = vec;
      let head = vec.remove(0);

      List {
        head: Some(head),
        tail: vec,
      }
    }
  }
}

impl IntoIterator for List {
  type Item = Atom;
  type IntoIter = std::vec::IntoIter<Atom>;

  fn into_iter(self) -> std::vec::IntoIter<Atom> {
    let List { head, mut tail } = self;

    if let Some(h) = head {
      tail.insert(0, h);
    }

    tail.into_iter()
  }
}

#[macro_export]
macro_rules! list {
  () => {
    List::new()
  };
  ($head:expr) => {
    List {
      head: Some($head),
      tail: vec![]
    }
  };
  ($head:expr, $( $tail:expr ),*) => {
    List {
      head: Some($head),
      tail: vec![$($tail),*]
    }
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

    assert_eq!(empty_list.head, None);
    assert_eq!(one_item_list.head, Some(id!("+")));
    assert_eq!(one_item_list.tail, vec![]);
    assert_eq!(multiple_items_list.head, Some(id!("-")));
    assert_eq!(
      multiple_items_list.tail,
      vec![
        Atom::Number(Number::new(5, 1).unwrap()),
        Atom::Number(Number::new(4, 1).unwrap())
      ]
    );
  }

  #[test]
  fn prepend() {
    let l1 = list![Atom::Nil, Atom::Number(Number::new(5, 1).unwrap())];
    let l2 = l1.prepend(id!("+"));

    assert_eq!(l2.head, Some(id!("+")));
    assert_eq!(
      l2.tail,
      vec![Atom::Nil, Atom::Number(Number::new(5, 1).unwrap())]
    );
  }

  #[test]
  fn append() {
    let l1 = list![Atom::Nil];
    let l2 = l1.append(Atom::Number(Number::new(7, 1).unwrap()));

    assert_eq!(l2.head, Some(Atom::Nil));
    assert_eq!(l2.tail, vec![Atom::Number(Number::new(7, 1).unwrap())]);
  }
}
