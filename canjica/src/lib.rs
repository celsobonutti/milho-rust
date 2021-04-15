mod eval;

#[cfg(test)]
mod tests;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub use eval::eval;
use pipoquinha::types::Atom;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Table {
  global_variables: HashMap<String, Atom>,
  local_variables: HashMap<String, Vec<Atom>>,
}

impl Table {
  pub fn insert_global_var(&mut self, name: &str, atom: Atom) {
    self.global_variables.insert(name.to_string(), atom);
  }

  pub fn insert_local_var(&mut self, name: &str, atom: Atom) {
    if let Some(values) = self.local_variables.get_mut(name) {
      values.push(atom);
    } else {
      let mut values = Vec::new();
      values.push(atom);
      self.local_variables.insert(name.to_string(), values);
    }
  }

  pub fn drop_local_var(&mut self, name: &str) {
    if let Some(values) = self.local_variables.get_mut(name) {
      values.pop();
      if values.is_empty() {
        self.local_variables.remove(name);
      }
    }
  }

  pub fn get(&self, name: &str) -> Option<Atom> {
    if let Some(values) = self.local_variables.get(name) {
      values.last().map(|v| v.clone())
    } else {
      self.global_variables.get(name).map(|v| v.clone())
    }
  }

  pub fn initialize(std_functions: Vec<Atom>) -> Rc<RefCell<Self>> {
    let table = Table {
      global_variables: HashMap::new(),
      local_variables: HashMap::new(),
    };

    let table = Rc::new(RefCell::new(table));

    for atom in std_functions {
      eval(atom, table.clone());
    }

    table
  }
}

pub type NamespaceTable = Rc<RefCell<Table>>;
