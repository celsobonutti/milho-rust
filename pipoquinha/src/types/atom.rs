use super::{Boolean, BuiltIn, Function, List, Number};

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Atom {
  List(List),
  Number(Number),
  Bool(Boolean),
  Error(String),
  Identifier(String),
  Function(Box<Function>),
  Macro(Box<Function>),
  MultiArityFn(Box<Vec<Function>>),
  Str(String),
  BuiltIn(BuiltIn),
  Nil,
}

impl Atom {
  pub fn new_function(parameters: Vec<Atom>, atom: Atom, is_macro: bool) -> Self {
    let variadic_identifier = Self::Identifier("+rest".to_string());
    let is_variadic = parameters
      .iter()
      .position(|item| item == &variadic_identifier);

    if parameters.iter().all(|item| item.is_identifier()) {
      let params = parameters
        .into_iter()
        .filter_map(|value| {
          if let Self::Identifier(id) = value {
            if id == "+rest" {
              Some("rest".to_string())
            } else {
              Some(id)
            }
          } else {
            None
          }
        })
        .collect::<Vec<String>>();

      if let Some(index) = is_variadic {
        if index == params.len() - 1 {
          if is_macro {
            return Self::Macro(Box::new(Function {
              parameters: params,
              atom,
              variadic: true,
            }));
          } else {
            return Self::Function(Box::new(Function {
              parameters: params,
              atom,
              variadic: true,
            }));
          }
        } else {
          return Self::Error("+rest is a reserved identifier for variadics, and should only be placed at the end of your parameters".to_string());
        }
      }

      if is_macro {
        Self::Macro(Box::new(Function {
          parameters: params,
          atom,
          variadic: false,
        }))
      } else {
        Self::Function(Box::new(Function {
          parameters: params,
          atom,
          variadic: false,
        }))
      }
    } else {
      Self::Error("Every argument in a function must be a identifier".to_string())
    }
  }

  pub fn is_list(&self) -> bool {
    if let Atom::List(_) = self {
      true
    } else {
      false
    }
  }

  pub fn is_identifier(&self) -> bool {
    if let Atom::Identifier(_) = self {
      true
    } else {
      false
    }
  }

  pub fn unwrap_id(self) -> String {
    if let Atom::Identifier(id) = self {
      id
    } else {
      panic!("Trying to unwrap an id from a non-identifier atom");
    }
  }

  pub fn unwrap_list(self) -> List {
    if let Atom::List(l) = self {
      l
    } else {
      panic!("Trying to unwrap a list from a non-list atom");
    }
  }

  pub fn is_string(&self) -> bool {
    if let Atom::Str(_) = self {
      true
    } else {
      false
    }
  }

  pub fn is_error(&self) -> bool {
    if let Atom::Error(_) = self {
      true
    } else {
      false
    }
  }

  pub fn unsafe_number(x: i64, y: i64) -> Self {
    Self::Number(Number(x, y))
  }

  pub fn make_boolean(x: bool) -> Self {
    if x {
      Self::Bool(Boolean::True)
    } else {
      Self::Bool(Boolean::False)
    }
  }

  pub fn make_string(x: &str) -> Self {
    Self::Str(x.to_string())
  }

  pub fn is_number(&self) -> bool {
    if let Atom::Number(_) = self {
      true
    } else {
      false
    }
  }

  pub fn type_of(&self) -> &str {
    match self {
      Atom::Bool(_) => "boolean",
      Atom::Number(_) => "number",
      Atom::Identifier(_) => "identifier",
      Atom::List(_) => "list",
      Atom::Function(_) | Atom::MultiArityFn(_) | Atom::BuiltIn(_) => "function",
      Atom::Error(_) => "error",
      Atom::Macro(_) => "macro",
      Atom::Str(_) => "string",
      Atom::Nil => "nil",
    }
  }
}
