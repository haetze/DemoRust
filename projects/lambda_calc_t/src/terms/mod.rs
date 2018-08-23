pub mod types;

use terms::types::Type;
use terms::types::TypeError;
use terms::types::Show;
use std::collections::HashSet;
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
    fn eval(self, _context: &mut HashMap<String, Term>) -> Term {
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
    fn eval(self, _context: &mut HashMap<String, Term>) -> Term {
        Term::ValBool(self)
    }
}



#[derive(Debug, Clone, PartialEq)]
pub struct Var {
    var: String,
    t: Type
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
    fn eval(self, context: &mut HashMap<String, Term>) -> Term {
        Term::Lambda(Lambda {
            var: self.var,
            term: box self.term.eval(context),
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
    fn eval(self, context: &mut HashMap<String, Term>) -> Term {
        match self {
            App{fun: box Term::Lambda(lambda),
                term: box t,
                t: _typ
            } => {
                let var = lambda.var;
                let term_fun = lambda.term;
                let tmp_t = context.remove(&var.var.clone());
                context.insert(var.var.clone(), t.clone());
                let result = term_fun.eval(context);
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
                
                let result_t = t.eval(context);
                let result_s = s.eval(context);
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
    fn eval(self, context: &mut HashMap<String, Term>) -> Term {
            match self {
                Term::ValI32(v) => v.eval(context),
                Term::ValBool(v) => v.eval(context),
                Term::Var(v) => v.eval(context),
                Term::Lambda(v) => v.eval(context),
                Term::App(v) => v.eval(context),
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
    fn eval (self, context: &mut HashMap<String, Term>) -> Term;
}


fn read_char(s: &mut String, c: char) -> Result<(), ()> {
    if s.len() == 0 {
        return Err(());
    }
    let head = s.remove(0);
    if head == c {
        return Ok(());
    }
    s.insert(0, head);
    return Err(());   
}

fn read_str(s: &mut String, st: &str) -> Result<(), ()> {
    let mut read = String::new();
    for i in st.chars() {
        match read_char(s, i) {
            Ok(_) => read.push(i),
            Err(_) => {
                s.insert_str(0, &read);
                return Err(());
            },
        }               
    }
    return Ok(());
}

pub fn read_true(s: &mut String) -> Result<Term, ()> {
        read_str(s, "true")?;
        Ok(Term::ValBool(ValBool::new(true)))
}

pub fn read_false(s: &mut String) -> Result<Term, ()> {
        read_str(s, "false")?;
        Ok(Term::ValBool(ValBool::new(false)))
}

pub fn read_var(s: &mut String, context: &mut HashMap<String, Type>, free: bool) -> Result<Term, ()>  {
    let mut st = String::new();
    loop {
        if s.len() == 0 {
            break;
        }
        let head = s.remove(0);
        if  !head.is_numeric() &&
            !head.is_whitespace() &&
            head != '(' &&
            head != ')' &&
            head != 'λ' &&
            head != '=' &&
            head != '>' &&
            head != '+' &&
            head != '*' &&
            head != '.' {
                st.push(head);
            } else {
                s.insert(0, head);
                break;
            }
    }
    if st.len() == 0 {
        return Err(());
    } else {
        let v = Var::new(st, context, free);
        return Ok(Term::Var(v));
    }
}

pub fn read_val_i32(s: &mut String) -> Result<Term, ()>  {
    let mut st = String::new();
    loop {
        if s.len() == 0 {
            break;
        }
        let head = s.remove(0);
        if head.is_numeric() {
            st.push(head);
        } else {
            s.insert(0, head);
            break;
        }
    }
    if st.len() == 0 {
        return Err(());
    }
    let v = ValI32::new(st.parse().unwrap());
    return Ok(Term::ValI32(v));
}

pub fn read_lambda(s: &mut String,
                   context: &mut HashMap<String, Type>,
                   locals: &mut HashSet<String>) -> Result<Term, ()> {
    read_char(s, '(')?;
    match read_char(s, 'λ') {
        Err(_) => {
            s.insert(0, '(');
            return Err(());
        },
        _ => (),
    };
    let mut context_new = context.clone();
    if let Term::Var(var) = read_var(s, &mut context_new, true)? {
        read_char(s, '.')?;
        locals.insert(var.var.clone());
        let term = read_term(s, &mut context_new, locals)?;
        for local in locals.iter() {
            context.insert(local.clone(),
                           context_new.get(local).unwrap().clone());
        }
        read_char(s, ')')?;
        let var = Var {
            t: context_new.get(&var.var).unwrap().clone(),
            var: var.var,
        };
        let lambda = Lambda::new(var, term);
        return Ok(Term::Lambda(lambda));
    }
    return Err(());
}

pub fn read_app(s: &mut String,
                context: &mut HashMap<String, Type>,
                locals: &mut HashSet<String>) -> Result<Term, ()> {
    read_char(s, '(')?;
    let t_1 = read_term(s, context, locals)?;
    if s.len() == 0 {
        return Err(());
    }
    read_char(s, ' ')?;
    let t_2 = read_term(s, context, locals)?;
    read_char(s, ')')?;
    let app = App::new(t_1, t_2, context);
    match app {
        Ok(app) => Ok(Term::App(app)),
        Err(_)  => Err(()),
    }
}

pub fn read_term(s: &mut String,
                 context: &mut HashMap<String, Type>,
                 locals: &mut HashSet<String>) -> Result<Term, ()> {
    if let Ok(Term::ValI32(v)) = read_val_i32(s) {
        return Ok(Term::ValI32(v));
    }
    if let Ok(Term::ValBool(v)) = read_true(s) {
        return Ok(Term::ValBool(v));
    }
    if let Ok(Term::ValBool(v)) = read_false(s) {
        return Ok(Term::ValBool(v));
    }
    if let Ok(Term::Var(v)) = read_var(s, context, false) {
        return Ok(Term::Var(v));
    }
    if let Ok(Term::Lambda(l)) = read_lambda(s, context, locals) {
        return Ok(Term::Lambda(l));
    }
    read_app(s, context, locals)
}
