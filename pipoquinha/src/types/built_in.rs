#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum BuiltIn {
  Add,
  Mul,
  Negate,
  Invert,
  Eql,
  Def,
  Defn,
  Defmacro,
  Fun,
  Let,
  If,
  Read,
  Eval,
  Print,
  Loop,
  Do,
  Not,
  Cons,
  MakeList,
  Car,
  Cdr,
  Quote,
}
