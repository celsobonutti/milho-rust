use std::fmt::{Display, Formatter, Result};

use crate::atom::Atom::{self, *};
use crate::boolean::Boolean;

impl Display for Atom {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Number(x) => write!(f, "{}", x),
            Bool(Boolean::False) => write!(f, "False"),
            Bool(Boolean::True) => write!(f, "True"),
            List(list) => {
                let mut text = String::from("[ ");
                for item in list {
                    text.push_str(&format!("{} ", *item));
                }
                text.push(']');
                write!(f, "{}", text)
            },
            Error(reason) => {
                write!(f, "Error: {}", reason)
            },
            Macro(m) => write!(f, "Macro: {:?}", m),
            Expr(expr) => write!(f, "Expression: {:?}", expr), 
        }
    }
}
