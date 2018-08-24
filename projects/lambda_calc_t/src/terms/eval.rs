use std::collections::HashMap;
use terms::Term;

pub trait Evaluate {
    fn eval (self, context: &mut HashMap<String, Term>) -> Term;
}
