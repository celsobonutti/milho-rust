use pipoquinha::{
  Atom::{self, *},
  Arithmetic,
};

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum EvalError {
  NotImplemented,
  NotEnoughArguments,
  TypeError(&'static str),
  DividedByZero,
}

fn do_op<'a, F>(
  input: &'a Result<Atom, EvalError>,
  func: F,
) -> Box<dyn Fn(Atom) -> Result<Atom, EvalError> + 'a>
where
  F: Fn(i64, i64) -> i64 + 'static,
{
  Box::new(move |atom: Atom| {
    input.clone().and_then(|result| match (result, atom) {
      (Int(x), Int(y)) => Ok(Int(func(y, x))),
      _ => Err(EXPECING_NUMBER),
    })
  })
}

const EXPECING_NUMBER: EvalError = EvalError::TypeError("Expecting number");

pub fn eval(atom: &Atom) -> Result<Atom, EvalError> {
  match atom {
    Int(value) => Ok(Int(*value)),
    Expr(expr) => match expr.function {
      Arithmetic::Add => expr.values.iter().fold(Ok(Int(0)), |acc, val| {
        acc.and_then(do_op(&eval(val), |x, y| x + y))
      }),
      Arithmetic::Mul => expr.values.iter().fold(Ok(Int(1)), |acc, val| {
        acc.and_then(do_op(&eval(val), |x, y| x * y))
      }),
      Arithmetic::Sub => expr
        .values
        .iter()
        .fold(Err(EvalError::NotEnoughArguments), |acc, val| match acc {
          Ok(acc_value) => do_op(&eval(val), |x, y| x - y)(acc_value),
          Err(EvalError::NotEnoughArguments) => eval(val),
          err @ Err(_) => err,
        }),
      Arithmetic::Div => expr
        .values
        .iter()
        .fold(Err(EvalError::NotEnoughArguments), |acc, val| match acc {
          Ok(acc_value) => match eval(val) {
            Ok(Int(0)) => Err(EvalError::DividedByZero),
            result => do_op(&result, |x, y| x / y)(acc_value),
          },
          Err(EvalError::NotEnoughArguments) => eval(val),
          err @ Err(_) => err,
        }),
      _ => Err(EvalError::NotImplemented),
    },
  }
}

#[cfg(test)]
mod tests {
  use super::{eval, EvalError};
  use pipoquinha::{
    expression,
    Atom::{self, *},
  };

  fn eval_text(x: &[u8]) -> Result<Atom, EvalError> {
    let atom = Atom::Expr(expression().parse(x).unwrap());

    eval(&atom)
  }

  #[test]
  fn addition() {
    assert_eq!(Ok(Int(10)), eval_text(b"(+ 5 5)"));
    assert_eq!(Ok(Int(13)), eval_text(b"(+ 3 (+ 5 (+ 2 3)))"))
  }

  #[test]
  fn multiplication() {
    assert_eq!(Ok(Int(20)), eval_text(b"(* 2 10)"));
    assert_eq!(Ok(Int(0)), eval_text(b"(* 200 0)"));
    assert_eq!(Ok(Int(40)), eval_text(b"(* 10 (* 2 2))"))
  }

  #[test]
  fn subtraction() {
    assert_eq!(Ok(Int(5)), eval_text(b"(- 10 5)"));
    assert_eq!(Ok(Int(-10)), eval_text(b"(- 20 (+ 20 10))"));
    assert_eq!(Ok(Int(2)), eval_text(b"(+ (- 10 14) 6)"))
  }

  #[test]
  fn division() {
    assert_eq!(Ok(Int(2)), eval_text(b"(/ 10 5)"));
    assert_eq!(Ok(Int(3)), eval_text(b"(/ (/ 27 3) 3)"));
    assert_eq!(Err(EvalError::DividedByZero), eval_text(b"(/ 5 (- 10 10))"));
    assert_eq!(Err(EvalError::DividedByZero), eval_text(b"(/ 5 0)"));
    assert_eq!(Ok(Int(0)), eval_text(b"(/ 0 100)"))
  }
}
