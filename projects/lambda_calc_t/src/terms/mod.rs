pub mod types;

use terms::types::Type;
use terms::types::TypeError;
use terms::types::Show;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct ValI32 {
    val: i32,
    t: Type,
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
    fn one_step(self, _context: &mut HashMap<String, Term>) -> Term {
        Term::ValI32(self)
    }
}



#[derive(Debug, Clone, PartialEq)]
pub struct ValBool {
    val: bool,
    t: Type,
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
    fn one_step(self, _context: &mut HashMap<String, Term>) -> Term {
        Term::ValBool(self)
    }
}



#[derive(Debug, Clone, PartialEq)]
pub struct Var {
    var: String,
    t: Type
}

impl Var {
    pub fn new(var: String, context: &mut HashMap<String, Type>) -> Var {

        let t;
        if let Some(t_) = context.get(&var) {
            t = t_.clone();
        } else {
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

        }

        context.insert(var.clone(), t.clone());
                
        Var {
            var: var,
            t: t,
        }
    }
}

impl Evaluate for Var {
    fn one_step(self, context: &mut HashMap<String, Term>) -> Term {
        match context.get(&self.var) {
            Some(t) => t.clone(),
            None    => Term::Var(self),
        }   
            
    }
}



#[derive(Debug, Clone, PartialEq)]
pub struct Lambda {
    var : Var,
    term: Box<Term>,
    t: Type,
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
    fn one_step(self, context: &mut HashMap<String, Term>) -> Term {
        Term::Lambda(Lambda {
            var: self.var,
            term: box self.term.one_step(context),
            t: self.t,
        })
    }
}



#[derive(Debug, Clone, PartialEq)]
pub struct App{
    fun : Box<Term>,
    term: Box<Term>,
    t: Type,
}

impl App {
    pub fn new(fun: Term, term: Term, context: &mut HashMap<String, Type>) -> Result<App, TypeError> {
        let fun_t = fun.get_type().clone();
        let term_t = term.get_type().clone();
        match &fun_t {
            Type::I32 => Err(TypeError::TypeNotApplicable(fun_t.clone())),
            Type::Bool => Err(TypeError::TypeNotApplicable(fun_t.clone())),
            Type::Var(_) => {
                match fun {
                    Term::Var(mut v) => {
                        let t;
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
                        let t_ = Type::Arrow(Box::new(term_t),
                                             Box::new(t.clone()));
                        context.insert(v.var.clone(), t_.clone());
                        v.t = t_;
                        Ok(App {
                            fun: Box::new(Term::Var(v)),
                            term: Box::new(term),
                            t: t,
                        })
                    },
                    _ => Err(TypeError::Unkown),
                }
                        
            },
            Type::Arrow(t_1, t_2) => {
                if term_t == **t_1 {
                    Ok(App {
                        fun: Box::new(fun),
                        term: Box::new(term),
                        t: (**t_2).clone(),
                    })
                    
                } else {
                    Err(TypeError::TypeMismatch(term_t, (**t_1).clone()))
                }
            },
        }
                             
                
    }
}

impl Evaluate for App {
    fn one_step(self, context: &mut HashMap<String, Term>) -> Term {
        match self {
            App{fun: box Term::Lambda(lambda),
                term: box t,
                t: _typ
            } => {
                let var = lambda.var;
                let term_fun = lambda.term;
                let tmp_t = context.remove(&var.var.clone());
                context.insert(var.var.clone(), t.clone());
                let result = term_fun.one_step(context);
                context.remove(&var.var.clone());
                match tmp_t {
                    None => None,
                    Some(term) => context.insert(var.var.clone(), term.clone()),
                };
                result
            },
            App{fun: t,
                term: s,
                t: typ} => {
                
                let result_t = t.one_step(context);
                let result_s = s.one_step(context);
                Term::App(App{
                    fun: box result_t,
                    term: box result_s,
                    t:typ,
                })
            },
        }
    }
}


#[derive(Debug, Clone, PartialEq)]
pub enum Term{
    ValI32(ValI32),
    ValBool(ValBool),
    Var(Var),
    Lambda(Lambda),
    App(App),
}

impl Evaluate for Term {
    fn one_step(self, context: &mut HashMap<String, Term>) -> Term {
        match self {
            Term::ValI32(v) => v.one_step(context),
            Term::ValBool(v) => v.one_step(context),
            Term::Var(v) => v.one_step(context),
            Term::Lambda(v) => v.one_step(context),
            Term::App(v) => v.one_step(context),
        }
    }
}


pub trait Typable {
    fn get_type(&self) -> &Type;
}

impl Typable for ValI32 {
    fn get_type(&self) -> &Type {
        &self.t
    }
}

impl Typable for ValBool {
    fn get_type(&self) -> &Type {
        &self.t
    }
}

impl Typable for Var {
    fn get_type(&self) -> &Type {
        &self.t
    }
}

impl Typable for Lambda {
    fn get_type(&self) -> &Type {
        &self.t
    }
}

impl Typable for App {
    fn get_type(&self) -> &Type {
        &self.t
    }
}

impl Typable for Term {
    fn get_type(&self) -> &Type {
        match self {
            Term::ValI32(v) => v.get_type(),
            Term::ValBool(v) => v.get_type(),
            Term::Var(v) => v.get_type(),
            Term::Lambda(v) => v.get_type(),
            Term::App(v) => v.get_type(),
        }
    }
}

impl Show for ValI32 {
    fn show(&self) -> String {
        format!("{}", self.val)
    }
}

impl Show for ValBool {
    fn show(&self) -> String {
        format!("{}", self.val)
    }
}

impl Show for Var {
    fn show(&self) -> String {
        format!("{}", self.var)
    }
}

impl Show for Lambda {
    fn show(&self) -> String {
        format!("(λ{}.{})",
                self.var.show(),
                self.term.show())
    }
}

impl Show for App {
    fn show(&self) -> String {
        format!("({} {})",
                self.fun.show(),
                self.term.show())
    }
}

impl Show for Term {
    fn show(&self) -> String {
        match self {
            Term::ValI32(v) => v.show(),
            Term::ValBool(v) => v.show(),
            Term::Var(v) => v.show(),
            Term::Lambda(v) => v.show(),
            Term::App(v) => v.show(),
        }
    }
}


pub trait Evaluate {
    fn one_step (self, context: &mut HashMap<String, Term>) -> Term;
}

