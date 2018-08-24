use terms::eval::Evaluate;
use terms::typable::Typable;
use terms::show::Show;
use terms::types::Type;
use terms::Term;

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct ValI32 {
    pub val: i32,
    pub t: Type,
}

impl ValI32 {
    pub fn new(v: i32) -> ValI32 {
        ValI32 {
            val: v,
            t: Type::I32,
        }
    }
}

impl Evaluate for ValI32 {
    fn eval(self, _context: &mut HashMap<String, Term>) -> Term {
        Term::ValI32(self)
    }
}

impl Typable for ValI32 {
    fn get_type(&self) -> &Type {
        &self.t
    }
}

impl Show for ValI32 {
    fn show(&self) -> String {
        format!("{}", self.val)
    }
}
