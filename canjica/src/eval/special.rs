use pipoquinha::parser::atom::Atom;

use crate::{eval, VarTable};

pub fn do_function(arguments: Vec<Atom>, variables: &VarTable) -> Atom {
    let mut atom = Atom::Nil;
    for item in arguments {
        atom = eval(item, variables);
    }
    atom
}
