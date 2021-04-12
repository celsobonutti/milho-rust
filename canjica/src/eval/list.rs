use pipoquinha::parser::atom::Atom::{self, *};
use pipoquinha::parser::list::List;

use super::{arithmetic, boolean, comparison, definition, function, io, special, vector};
use crate::{eval, NamespaceTable, VarTable};

pub fn execute(
  mut list: List,
  namespace_variables: NamespaceTable,
  local_variables: &VarTable,
) -> Atom {
  let fun_name = list.head;
  match fun_name {
    Some(BuiltIn(x)) => match x.as_str() {
      ".__add__" => arithmetic::add(list.tail, namespace_variables, local_variables),
      ".__mul__" => arithmetic::multiply(list.tail, namespace_variables, local_variables),
      ".__div__" => arithmetic::divide(list.tail, namespace_variables, local_variables),
      ".__eq__" => comparison::eq(list.tail, namespace_variables, local_variables),
      ".__negate__" => arithmetic::negate(list.tail, namespace_variables, local_variables),
      ".__def__" => definition::variable(list.tail, namespace_variables, local_variables),
      ".__defn__" => definition::function(list.tail, namespace_variables, local_variables),
      ".__defmacro__" => definition::macro_d(list.tail, namespace_variables, local_variables),
      ".__fn__" => definition::anonymous_function(list.tail, namespace_variables, local_variables),
      ".__let__" => definition::local_variables(list.tail, namespace_variables, local_variables),
      ".__if__" => special::if_fun(list.tail, namespace_variables, local_variables),
      ".__read__" => io::read(list.tail),
      ".__eval__" => {
        if list.tail.len() == 1 {
          eval(
            eval(
              list.tail.remove(0),
              namespace_variables.clone(),
              local_variables,
            ),
            namespace_variables,
            local_variables,
          )
        } else {
          Error(format!(
            "Wrong number of arguments for 'eval': was expecting 1, found {}",
            list.tail.len()
          ))
        }
      }
      ".__print__" => io::print(list.tail, namespace_variables, local_variables),
      ".__loop__" => special::loop_function(list.tail, namespace_variables, local_variables),
      ".__do__" => special::do_function(list.tail, namespace_variables, local_variables),
      ".__not__" => boolean::not(list.tail, namespace_variables, local_variables),
      ".__head__" => vector::head(list.tail, namespace_variables, local_variables),
      ".__tail__" => vector::tail(list.tail, namespace_variables, local_variables),
      ".__concat__" => vector::concatenate(list.tail, namespace_variables, local_variables),
      ".__cons__" => cons(list.tail, namespace_variables, local_variables),
      ".__make-list__" => make_list(list.tail, namespace_variables, local_variables),
      ".__car__" => car(list.tail, namespace_variables, local_variables),
      ".__cdr__" => cdr(list.tail, namespace_variables, local_variables),
      ".__quote__" => {
        if list.tail.len() == 1 {
          list.tail.remove(0)
        } else {
          Error(format!(
            "Wrong number of arguments for 'quote': was expecting 1, found {}",
            list.tail.len()
          ))
        }
      }
      n => Error(format!("Undefined built-in: {}", n)),
    },
    Some(Identifier(name)) => {
      let item = eval(
        Identifier(name.to_string()),
        namespace_variables.clone(),
        local_variables,
      );

      match item {
        b @ BuiltIn(_) => execute(
          List {
            head: Some(b),
            tail: list.tail,
          },
          namespace_variables,
          local_variables,
        ),
        Function(function) => function::execute(
          *function.clone(),
          list.tail,
          namespace_variables,
          local_variables,
        ),
        Macro(m) => eval(
          function::execute_macro(
            *m.clone(),
            list.tail,
            namespace_variables.clone(),
            local_variables,
          ),
          namespace_variables,
          local_variables,
        ),
        MultiArityFn(functions) => function::multi_arity_function(
          *functions.clone(),
          list.tail,
          namespace_variables,
          local_variables,
        ),
        e @ Error(_) => e,
        _ => Error(format!("Cannot invoke {}, as it's not a function", name)),
      }
    }
    Some(Function(f)) => function::execute(*f, list.tail, namespace_variables, local_variables),
    Some(List(l)) => {
      let h = execute(*l, namespace_variables.clone(), local_variables);
      let new_list = List {
        head: Some(h),
        tail: list.tail,
      };

      execute(new_list, namespace_variables, local_variables)
    }
    Some(e @ Error(_)) => e,
    Some(value) => Error(format!("Cannot invoke {}, as it's not a function", value)),
    None => Nil,
  }
}

fn cons(
  mut arguments: Vec<Atom>,
  namespace_variables: NamespaceTable,
  local_variables: &VarTable,
) -> Atom {
  if arguments.len() == 2 {
    let new_head = eval(
      arguments.remove(0),
      namespace_variables.clone(),
      local_variables,
    );
    let target = arguments.remove(0);

    let x = eval(target, namespace_variables, local_variables);

    if let List(t) = x {
      List(Box::new(t.prepend(new_head)))
    } else {
      Error("Cannot cons into non-list value".to_string())
    }
  } else {
    Error(
      format!(
        "Wrong number of arguments for cons: was expecting 2, found {}",
        arguments.len()
      )
      .to_string(),
    )
  }
}

fn make_list(
  mut arguments: Vec<Atom>,
  namespace_variables: NamespaceTable,
  local_variables: &VarTable,
) -> Atom {
  if arguments.len() == 1 || arguments.len() == 2 {
    match (
      eval(arguments.remove(0), namespace_variables, local_variables),
      arguments.get(0).unwrap_or(&Nil),
    ) {
      (Number(x), value) => List(Box::new(List::from_vec(vec![value.clone(); x as usize]))),
      _ => Error("Memes".to_string()),
    }
  } else {
    Error(format!(
      "Wrong number of arguments for 'make_list': was expecing 1 or 2, found {}",
      arguments.len()
    ))
  }
}

fn car(
  mut arguments: Vec<Atom>,
  namespace_variables: NamespaceTable,
  local_variables: &VarTable,
) -> Atom {
  if arguments.len() == 1 {
    match eval(arguments.remove(0), namespace_variables, local_variables) {
      List(l) => l.head.unwrap_or(Nil),
      _ => {
        Error("Wrong type of arguments for 'car': it can only be applied into lists".to_string())
      }
    }
  } else {
    Error(format!(
      "Wrong number of arguments for 'car': was expecting 1, found {}",
      arguments.len()
    ))
  }
}

fn cdr(
  mut arguments: Vec<Atom>,
  namespace_variables: NamespaceTable,
  local_variables: &VarTable,
) -> Atom {
  if arguments.len() == 1 {
    match eval(arguments.remove(0), namespace_variables, local_variables) {
      List(l) => List(Box::new(List::from_vec(l.tail))),
      _ => {
        Error("Wrong type of arguments for 'car': it can only be applied into lists".to_string())
      }
    }
  } else {
    Error(format!(
      "Wrong number of arguments for 'cdr': was expecting 1, found {}",
      arguments.len()
    ))
  }
}
