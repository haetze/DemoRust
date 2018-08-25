use terms::Term;
use terms::types::Type;
use terms::show::Show;
use terms::typable::Typable;
use terms::eval::Evaluate;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Var {
    pub var: String,
    pub t: Type
}

impl Var {
    pub fn get_var(&self) -> String {
        self.var.clone()
    }
    
    pub fn new(var: String, context: &mut HashMap<String, Type>, free: bool) -> Var {

        let mut t;
        let mut smallest_available = 0;
        loop {
            let mut available = true;
            for (_, v) in context.iter() {
                if let Type::Var(s) = v {
                    if *s == smallest_available {
                        available = false;
                        smallest_available = s + 1;
                        break;
                    }
                }
            }
            if available {
                t = Type::Var(smallest_available);
                break;
            }
        }
        if !free {
            if let Some(t_) = context.get(&var)  {
                t = t_.clone();
            }
        }

        if context.contains_key(&var) {
            let val = context.remove(&var).unwrap();
            let key = format!("{}{}", var, val.show());
            context.insert(key, val);
        }
        context.insert(var.clone(), t.clone());
                
        Var {
            var: var,
            t: t,
        }
    }
}

impl Evaluate for Var {
    fn eval(self, context: &mut HashMap<String, Term>) -> Term {

        match context.get(&self.var) {
            Some(t) => {
                //println!("Replacing {} with {}", self.show(), t.show());
                t.clone()
            },
            None    => Term::Var(self),
        }   
            
    }
}

impl Typable for Var {
    fn get_type(&self) -> &Type {
        &self.t
    }
}

impl Show for Var {
    fn show(&self) -> String {
        format!("{}", self.var)
    }
}
