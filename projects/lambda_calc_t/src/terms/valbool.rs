use terms::show::Show;
use terms::eval::Evaluate;
use terms::typable::Typable;
use terms::types::Type;
use terms::Term;

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct ValBool {
    pub val: bool,
    pub t: Type,
}


impl ValBool {
    pub fn new(v: bool) -> ValBool {
        ValBool {
            val: v,
            t: Type::Bool,
        }
    }
}


impl Evaluate for ValBool {
    fn eval(self, _context: &mut HashMap<String, Term>) -> Term {
        Term::ValBool(self)
    }
}

impl Typable for ValBool {
    fn get_type(&self) -> &Type {
        &self.t
    }
}

impl Show for ValBool {
    fn show(&self) -> String {
        format!("{}", self.val)
    }
}

