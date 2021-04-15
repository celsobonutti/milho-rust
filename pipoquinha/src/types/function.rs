use super::atom::Atom;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Function {
  pub parameters: Vec<String>,
  pub variadic: bool,
  pub atom: Atom,
}

impl Function {
  pub fn param_len(&self) -> usize {
    self.parameters.len()
  }
}
