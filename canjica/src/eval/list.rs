use pipoquinha::types::{Atom, BuiltIn::*, List, Number};

use super::{arithmetic, boolean, comparison, definition, function, io, special};
use crate::{eval, NamespaceTable};

pub fn execute(list: List, namespace_variables: NamespaceTable) -> Atom {
  if let Some((head, tail)) = list.split() {
    match head {
      Atom::BuiltIn(x) => match x {
        Add => arithmetic::add(tail.as_vec(), namespace_variables),
        Mul => arithmetic::multiply(tail.as_vec(), namespace_variables),
        Eql => comparison::eq(tail.as_vec(), namespace_variables),
        Negate => arithmetic::negate(tail.as_vec(), namespace_variables),
        Invert => arithmetic::invert(tail.as_vec(), namespace_variables),
        Def => definition::variable(tail.as_vec(), namespace_variables),
        Defn => definition::function(tail.as_vec(), namespace_variables),
        Defmacro => definition::macro_d(tail.as_vec(), namespace_variables),
        Fun => definition::anonymous_function(tail.as_vec(), namespace_variables),
        Let => definition::local_variables(tail.as_vec(), namespace_variables),
        If => special::if_fun(tail.as_vec(), namespace_variables),
        Read => io::read(tail.as_vec()),
        Eval => {
          if tail.len() == 1 {
            let value = eval(tail.pop().unwrap(), namespace_variables.clone());
            eval(value, namespace_variables)
          } else {
            Atom::Error(format!(
              "Wrong number of arguments for 'eval': was expecting 1, found {}",
              tail.len()
            ))
          }
        }
        Print => io::print(tail.as_vec(), namespace_variables),
        Loop => special::loop_function(tail.as_vec(), namespace_variables),
        Do => special::do_function(tail.as_vec(), namespace_variables),
        Not => boolean::not(tail.as_vec(), namespace_variables),
        Cons => cons(tail.as_vec(), namespace_variables),
        MakeList => make_list(tail.as_vec(), namespace_variables),
        Car => car(tail.as_vec(), namespace_variables),
        Cdr => cdr(tail.as_vec(), namespace_variables),
        Quote => {
          if tail.len() == 1 {
            tail.pop().unwrap()
          } else {
            Atom::Error(format!(
              "Wrong number of arguments for 'quote': was expecting 1, found {}",
              tail.len()
            ))
          }
        }
        Gt => comparison::gt(tail.as_vec(), namespace_variables),
      },
      Atom::Identifier(name) => {
        let item = eval(
          Atom::Identifier(name.to_string()),
          namespace_variables.clone(),
        );

        match item {
          b @ Atom::BuiltIn(_) => execute(tail.prepend(b), namespace_variables),
          Atom::Function(function) => {
            function::execute(*function.clone(), tail.as_vec(), namespace_variables)
          }
          Atom::Macro(m) => eval(
            function::execute_macro(*m.clone(), tail.as_vec(), namespace_variables.clone()),
            namespace_variables,
          ),
          Atom::MultiArityFn(functions) => {
            function::multi_arity_function(*functions.clone(), tail.as_vec(), namespace_variables)
          }
          e @ Atom::Error(_) => e,
          _ => Atom::Error(format!("Cannot invoke {}, as it's not a function", name)),
        }
      }
      Atom::Function(f) => function::execute(*f, tail.as_vec(), namespace_variables),
      Atom::List(l) => {
        let h = execute(l, namespace_variables.clone());
        let new_list = tail.prepend(h);

        execute(new_list, namespace_variables)
      }
      e @ Atom::Error(_) => e,
      value => Atom::Error(format!("Cannot invoke {}, as it's not a function", value))
    }
  } else {
    Atom::Nil
  }
}

fn cons(mut arguments: Vec<Atom>, namespace_variables: NamespaceTable) -> Atom {
  if arguments.len() == 2 {
    let new_head = eval(arguments.remove(0), namespace_variables.clone());
    let target = arguments.remove(0);

    let x = eval(target, namespace_variables);

    if let Atom::List(t) = x {
      Atom::List(t.prepend(new_head))
    } else {
      Atom::Error("Cannot cons into non-list value".to_string())
    }
  } else {
    Atom::Error(
      format!(
        "Wrong number of arguments for cons: was expecting 2, found {}",
        arguments.len()
      )
      .to_string(),
    )
  }
}

fn make_list(mut arguments: Vec<Atom>, namespace_variables: NamespaceTable) -> Atom {
  if arguments.len() == 1 || arguments.len() == 2 {
    match (
      eval(arguments.remove(0), namespace_variables),
      arguments.get(0).unwrap_or(&Atom::Nil),
    ) {
      (Atom::Number(x), value) => {
        if let Number(size, 1) = x {
          Atom::List(List::from_vec(vec![value.clone(); size as usize]))
        } else {
          Atom::Error(format!("The size for a list must be a integer."))
        }
      }
      _ => Atom::Error("Memes".to_string()),
    }
  } else {
    Atom::Error(format!(
      "Wrong number of arguments for 'make_list': was expecing 1 or 2, found {}",
      arguments.len()
    ))
  }
}

fn car(mut arguments: Vec<Atom>, namespace_variables: NamespaceTable) -> Atom {
  if arguments.len() == 1 {
    match eval(arguments.remove(0), namespace_variables) {
      Atom::List(l) => match l.split() {
        Some((head, _)) => head,
        _ => Atom::Nil,
      },
      _ => Atom::Error(
        "Wrong type of arguments for 'car': it can only be applied into lists".to_string(),
      ),
    }
  } else {
    Atom::Error(format!(
      "Wrong number of arguments for 'car': was expecting 1, found {}",
      arguments.len()
    ))
  }
}

fn cdr(mut arguments: Vec<Atom>, namespace_variables: NamespaceTable) -> Atom {
  if arguments.len() == 1 {
    match eval(arguments.remove(0), namespace_variables) {
      Atom::List(l) => match l.split() {
        Some((_, tail)) => Atom::List(tail),
        _ => Atom::Nil,
      },
      _ => Atom::Error(
        "Wrong type of arguments for 'car': it can only be applied into lists".to_string(),
      ),
    }
  } else {
    Atom::Error(format!(
      "Wrong number of arguments for 'cdr': was expecting 1, found {}",
      arguments.len()
    ))
  }
}
