
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


#[derive(Debug, Clone, PartialEq)]
pub enum Term{
    ValI32(ValI32),
    ValBool(ValBool),
    Var(Var),
    Lambda(Lambda),
    App(App),
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



