use terms::Term;
use terms::var::Var;
use terms::types::Type;
use terms::show::Show;
use terms::typable::Typable;
use terms::eval::Evaluate;
use std::collections::HashMap;



#[derive(Debug, Clone, PartialEq)]
pub struct Lambda {
    pub var : Var,
    pub term: Box<Term>,
    pub t: Type,
}

impl Lambda {
    pub fn new(var: Var, term: Term) -> Lambda {
        let var_t = Box::new(var.get_type().clone());
        let term_t = Box::new(term.get_type().clone());
        Lambda {
            var: var,
            term: Box::new(term),
            t: Type::Arrow(var_t,
                           term_t),
        }   
    }
}

impl Evaluate for Lambda {
    fn eval(self, context: &mut HashMap<String, Term>) -> Term {
        let tmp = context.remove(&self.var.var);
        let t = Term::Lambda(Lambda {
            var: self.var.clone(),
            term: box self.term.clone().eval(context),
            t: self.t.clone(),
        });
        tmp.and_then(|t| context.insert(self.var.var.clone(), t));
        t
    }
}

impl Typable for Lambda {
    fn get_type(&self) -> &Type {
        &self.t
    }
}

impl Show for Lambda {
    fn show(&self) -> String {
        format!("(λ{}.{})",
                self.var.show(),
                self.term.show())
    }
}
