use pipoquinha::parser::atom::Atom::{self, *};
use pipoquinha::parser::list::List;

use super::{arithmetic, boolean, comparison, definition, function, io, special, vector};
use crate::{eval, VarTable};

pub fn execute(list: List, variables: &VarTable) -> Atom {
  let fun_name = &list.head;
  match fun_name {
    Some(Identifier(x)) => match x.as_str() {
      "+" => arithmetic::add(list.tail, variables),
      "-" => arithmetic::subtract(list.tail, variables),
      "*" => arithmetic::multiply(list.tail, variables),
      "/" => arithmetic::divide(list.tail, variables),
      "=" => comparison::eq(list.tail, variables),
      "def" => definition::variable(list.tail, variables),
      "defn" => definition::function(list.tail, variables),
      "fn" => definition::anonymous_function(list.tail, variables),
      "let" => definition::local_variables(list.tail, variables),
      "if" => special::if_fun(list.tail, variables),
      "read" => io::read(list.tail),
      "eval" => eval_list(list.tail, variables),
      "print" => io::print(list.tail, variables),
      "loop" => special::loop_function(list.tail, variables),
      "do" => special::do_function(list.tail, variables),
      "not" => boolean::not(list.tail, variables),
      "head" => vector::head(list.tail, variables),
      "tail" => vector::tail(list.tail, variables),
      "concat" => vector::concatenate(list.tail, variables),
      "cons" => cons(list.tail, variables),
      "make-list" => make_list(list.tail, variables),
      "car" => car(list.tail, variables),
      "cdr" => cdr(list.tail, variables),
      _ => function::execute(list, variables),
    },
    Some(e @ Error(_)) => e.clone(),
    Some(h) => Error(format!("{} cannot be executed, as it's not a function.", h)),
    None => Nil,
  }
}

fn cons(mut arguments: Vec<Atom>, variables: &VarTable) -> Atom {
  if arguments.len() == 2 {
    let new_head = eval(arguments.remove(0), variables);

    let target = arguments.remove(0);

    if let List(t) = eval(target, variables) {
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

fn make_list(mut arguments: Vec<Atom>, variables: &VarTable) -> Atom {
  if arguments.len() == 1 || arguments.len() == 2 {
    match (
      eval(arguments.remove(0), variables),
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

fn eval_list(mut arguments: Vec<Atom>, variables: &VarTable) -> Atom {
  if arguments.len() == 1 {
    match arguments.remove(0) {
      id @ Identifier(_) => {
        let mut atom = eval(id, variables);

        while let l @ List(_) = atom {
          atom = eval(l, variables);
        }

        atom
      }
      l @ List(_) => {
        let mut atom = l;

        while let l @ List(_) = atom {
          atom = eval(l, variables);
        }

        atom
      }
      x => x,
    }
  } else {
    Error(format!(
      "Wrong number of arguments for 'eval': was expecting 1, found {}",
      arguments.len()
    ))
  }
}

fn car(mut arguments: Vec<Atom>, variables: &VarTable) -> Atom {
  if arguments.len() == 1 {
    match eval(arguments.remove(0), variables) {
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

fn cdr(mut arguments: Vec<Atom>, variables: &VarTable) -> Atom {
  if arguments.len() == 1 {
    match eval(arguments.remove(0), variables) {
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
