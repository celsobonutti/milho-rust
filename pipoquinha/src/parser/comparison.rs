extern crate pom;
use pom::parser::*;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Comparison {
  Eql,
  Dif,
  Lt,
  Gt,
  LtE,
  GtE
}

pub fn comparison_functions<'a>() -> Parser<'a, u8, Comparison> {
  let equal = sym(b'=').map(|_| Comparison::Eql);
  let different = seq(b"/=").map(|_| Comparison::Dif);
  let lt = sym(b'<').map(|_| Comparison::Lt);
  let gt = sym(b'>').map(|_| Comparison::Gt);
  let lte = seq(b"<=").map(|_| Comparison::LtE);
  let gte = seq(b">=").map(|_| Comparison::GtE);

  (different | gte | lte | equal | gt | lt).name("Comparison")
}
