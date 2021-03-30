use pom::parser::*;

use crate::{atom::expression, parser::atom::Expression};
use crate::parser::list::list;

pub struct UserFunction {
  arg_number: i64,
  function: Expression,
}

// pub fn defn<'a>() -> Parser<'a, u8, UserFunction> {
//     let x = sym(b'(') * seq(b"defn") + list() + expression() - sym(b')');
// 
// }
