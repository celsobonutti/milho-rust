use pipoquinha::types::{Atom, List, Number};

use super::{arithmetic, boolean, comparison, definition, function, io, special, vector};
use crate::{eval, NamespaceTable};

pub fn execute(mut list: List, namespace_variables: NamespaceTable) -> Atom {
  let fun_name = list.head;
  match fun_name {
    Some(Atom::BuiltIn(x)) => match x.as_str() {
      ".__add__" => arithmetic::add(list.tail, namespace_variables),
      ".__mul__" => arithmetic::multiply(list.tail, namespace_variables),
      ".__eq__" => comparison::eq(list.tail, namespace_variables),
      ".__negate__" => arithmetic::negate(list.tail, namespace_variables),
      ".__invert__" => arithmetic::invert(list.tail, namespace_variables),
      ".__def__" => definition::variable(list.tail, namespace_variables),
      ".__defn__" => definition::function(list.tail, namespace_variables),
      ".__defmacro__" => definition::macro_d(list.tail, namespace_variables),
      ".__fn__" => definition::anonymous_function(list.tail, namespace_variables),
      ".__let__" => definition::local_variables(list.tail, namespace_variables),
      ".__if__" => special::if_fun(list.tail, namespace_variables),
      ".__read__" => io::read(list.tail),
      ".__eval__" => {
        if list.tail.len() == 1 {
          eval(
            eval(list.tail.remove(0), namespace_variables.clone()),
            namespace_variables,
          )
        } else {
          Atom::Error(format!(
            "Wrong number of arguments for 'eval': was expecting 1, found {}",
            list.tail.len()
          ))
        }
      }
      ".__print__" => io::print(list.tail, namespace_variables),
      ".__loop__" => special::loop_function(list.tail, namespace_variables),
      ".__do__" => special::do_function(list.tail, namespace_variables),
      ".__not__" => boolean::not(list.tail, namespace_variables),
      ".__head__" => vector::head(list.tail, namespace_variables),
      ".__tail__" => vector::tail(list.tail, namespace_variables),
      ".__concat__" => vector::concatenate(list.tail, namespace_variables),
      ".__cons__" => cons(list.tail, namespace_variables),
      ".__make-list__" => make_list(list.tail, namespace_variables),
      ".__car__" => car(list.tail, namespace_variables),
      ".__cdr__" => cdr(list.tail, namespace_variables),
      ".__quote__" => {
        if list.tail.len() == 1 {
          list.tail.remove(0)
        } else {
          Atom::Error(format!(
            "Wrong number of arguments for 'quote': was expecting 1, found {}",
            list.tail.len()
          ))
        }
      }
      n => Atom::Error(format!("Undefined built-in: {}", n)),
    },
    Some(Atom::Identifier(name)) => {
      let item = eval(
        Atom::Identifier(name.to_string()),
        namespace_variables.clone(),
      );

      match item {
        b @ Atom::BuiltIn(_) => execute(
          List {
            head: Some(b),
            tail: list.tail,
          },
          namespace_variables,
        ),
        Atom::Function(function) => {
          function::execute(*function.clone(), list.tail, namespace_variables)
        }
        Atom::Macro(m) => eval(
          function::execute_macro(*m.clone(), list.tail, namespace_variables.clone()),
          namespace_variables,
        ),
        Atom::MultiArityFn(functions) => {
          function::multi_arity_function(*functions.clone(), list.tail, namespace_variables)
        }
        e @ Atom::Error(_) => e,
        _ => Atom::Error(format!("Cannot invoke {}, as it's not a function", name)),
      }
    }
    Some(Atom::Function(f)) => function::execute(*f, list.tail, namespace_variables),
    Some(Atom::List(l)) => {
      let h = execute(*l, namespace_variables.clone());
      let new_list = List {
        head: Some(h),
        tail: list.tail,
      };

      execute(new_list, namespace_variables)
    }
    Some(e @ Atom::Error(_)) => e,
    Some(value) => Atom::Error(format!("Cannot invoke {}, as it's not a function", value)),
    None => Atom::Nil,
  }
}

fn cons(mut arguments: Vec<Atom>, namespace_variables: NamespaceTable) -> Atom {
  if arguments.len() == 2 {
    let new_head = eval(arguments.remove(0), namespace_variables.clone());
    let target = arguments.remove(0);

    let x = eval(target, namespace_variables);

    if let Atom::List(t) = x {
      Atom::List(Box::new(t.prepend(new_head)))
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
          Atom::List(Box::new(List::from_vec(vec![value.clone(); size as usize])))
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
      Atom::List(l) => l.head.unwrap_or(Atom::Nil),
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
      Atom::List(l) => Atom::List(Box::new(List::from_vec(l.tail))),
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
