mod eval;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub use eval::eval;
use pipoquinha::parser::atom::Atom;

type Variables = HashMap<String, Atom>;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Table {
  module: Variables,
}

impl Table {
  pub fn insert_module(&mut self, name: String, atom: Atom) {
    self.module.insert(name, atom);
  }

  pub fn get(&self, name: &str) -> Option<Atom> {
    if let Some(atom) = self.module.get(name) {
      Some(atom.clone())
    } else {
      None
    }
  }

  pub fn initialize(std_functions: Vec<Atom>) -> Rc<RefCell<Self>> { 
    let table = Table {
      module: HashMap::new()
    };

    let table = Rc::new(RefCell::new(table));

    for atom in std_functions {
      eval(atom, table.clone(), &HashMap::new());
    }

    table
  }
}

pub type NamespaceTable = Rc<RefCell<Table>>;

pub type VarTable = Variables;
