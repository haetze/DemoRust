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
                    
                } else if let box Type::Var(i) = t_1 {
                    Ok(App {
                        fun: Box::new(fun),
                        term: Box::new(term),
                        t: (**t_2).clone().replace_var(*i, term_t.clone()),
                    })
                } else if let Type::Var(_) = term_t {
                    match term {
                        Term::Var(mut var) => {
                            context.insert(var.var.clone(),
                                           (**t_1).clone());
                            var.t = (**t_1).clone();
                            Ok(App {
                                fun: Box::new(fun),
                                term: Box::new(Term::Var(var)),
                                t: (**t_2).clone(),
                            })
                        },
                        Term::App(mut app) => {
                            app.t = (**t_1).clone();
                            Ok(App {
                                fun: Box::new(fun),
                                term: Box::new(Term::App(app)),
                                t: (**t_2).clone(),
                            })
                        },
                        Term::Lambda(mut lam) => {
                            lam.t = (**t_1).clone();
                            Ok(App {
                                fun: Box::new(fun),
                                term: Box::new(Term::Lambda(lam)),
                                t: (**t_2).clone(),
                            })
                        },
                        _   => Err(TypeError::TypeMismatch(term_t, (**t_1).clone())),
                        
                    }
                    
                }else {
                    Err(TypeError::TypeMismatch(term_t, (**t_1).clone()))
                }
            },
        }
                             
                
    }
}

impl Evaluate for App {
    fn eval(self, context: &mut HashMap<String, Term>) -> Term {
        match self {
            App{fun: box Term::BuildIn(b),
                term: box t,
                t: _typ
            } => {
                (b.to_fun()(t.eval(context))).eval(context)
            },
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
                
                let result_t = t.clone().eval(context);
                let result_s = s.clone().eval(context);
                if result_t == *t && result_s == *s {
                    return Term::App(App{
                        fun: box result_t,
                        term: box result_s,
                        t:typ,
                    });
                }
                Term::App(App{
                    fun: box result_t,
                    term: box result_s,
                    t:typ,
                }).eval(context)
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
    BuildIn(BuildIns),
}

#[derive(Debug, Clone, PartialEq)]
pub enum BuildIns {
    Inc(Type),
    Dec(Type),
    Zerop(Type),
    Eq2I(Type),
    Eq1I(Type, i32),
    Eq2B(Type),
    Eq1B(Type, bool),
    
}

impl BuildIns {
    fn to_fun(self) -> Box<Fn(Term) -> Term> {
        match self {
            BuildIns::Eq2I(_) => {
                box (|t| {
                    match t {
                        Term::ValI32(v) => {
                            let t = Type::Arrow(box Type::I32, box Type::Bool);
                            let b = BuildIns::Eq1I(t, v.val);
                            Term::BuildIn(b)
                        },
                        t   => t,
                    }
                })
            },
            BuildIns::Eq1I(_, i) => {
                box (move |t| {
                    match t {
                        Term::ValI32(v) => {
                            let b = ValBool::new(i == v.val);
                            Term::ValBool(b)
                        },
                        t   => t,
                    }
                })
            },
            BuildIns::Eq2B(_) => {
                box (|t| {
                    match t {
                        Term::ValBool(v) => {
                            let t = Type::Arrow(box Type::Bool, box Type::Bool);
                            let b = BuildIns::Eq1B(t, v.val);
                            Term::BuildIn(b)
                        },
                        t   => t,
                    }
                })
            },
            BuildIns::Eq1B(_, i) => {
                box (move |t| {
                    match t {
                        Term::ValBool(v) => {
                            let b = ValBool::new(i == v.val);
                            Term::ValBool(b)
                        },
                        t   => t,
                    }
                })
            },
            BuildIns::Inc(_) => {
                box (|t| {
                    match t {
                        Term::ValI32(mut v) => {
                            v.val = v.val + 1;
                            Term::ValI32(v)
                        },
                        t   => t,
                    }
                })
            },
            BuildIns::Dec(_) => {
                box (|t| {
                    match t {
                        Term::ValI32(mut v) => {
                            v.val = v.val - 1;
                            Term::ValI32(v)
                        },
                        t   => t,
                    }
                })
            },
            BuildIns::Zerop(_) => {
                box (|t| {
                    match t {
                        Term::ValI32(v) => {
                            let b  = ValBool::new(v.val == 0);
                            Term::ValBool(b)
                        },
                        t   => t,
                    }
                })
            },
            
        }
    }
}
    
impl Typable for BuildIns {
    fn get_type(&self) -> &Type {
        match self {
            BuildIns::Inc(t) => t,
            BuildIns::Dec(t) => t,
            BuildIns::Zerop(t) => t,
            BuildIns::Eq2I(t)  => t,
            BuildIns::Eq1I(t,_)  => t,
            BuildIns::Eq2B(t)  => t,
            BuildIns::Eq1B(t,_)  => t,
        }
    }
}


impl Evaluate for Term {
    fn eval(self, context: &mut HashMap<String, Term>) -> Term {
            match self {
                Term::ValI32(v) => v.eval(context),
                Term::ValBool(v) => v.eval(context),
                Term::Var(v) => v.eval(context),
                Term::Lambda(v) => v.eval(context),
                Term::App(v) => v.eval(context),
                t            => t,
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
            Term::BuildIn(b) => b.get_type(),
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

impl Show for BuildIns {
    fn show(&self) -> String {
        match self {
            BuildIns::Inc(_) => "inc".to_string(),
            BuildIns::Dec(_) => "dec".to_string(),
            BuildIns::Zerop(_) => "zerop".to_string(),
            BuildIns::Eq2I(_) => "=".to_string(),
            BuildIns::Eq1I(_,t) => format!("{}=", t),
            BuildIns::Eq2B(_) => "eq".to_string(),
            BuildIns::Eq1B(_,t) => format!("{} eq", t),
        }
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
            Term::BuildIn(b) => b.show(),
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

pub fn read_inc(s: &mut String) -> Result<Term, ()> {
    read_str(s, "inc")?;
    let t = Type::Arrow(box Type::I32, box Type::I32);
    Ok(Term::BuildIn(BuildIns::Inc(t)))
}

pub fn read_dec(s: &mut String) -> Result<Term, ()> {
    read_str(s, "dec")?;
    let t = Type::Arrow(box Type::I32, box Type::I32);
    Ok(Term::BuildIn(BuildIns::Dec(t)))
}

pub fn read_zerop(s: &mut String) -> Result<Term, ()> {
    read_str(s, "zerop")?;
    let t = Type::Arrow(box Type::I32, box Type::Bool);
    Ok(Term::BuildIn(BuildIns::Zerop(t)))
}

pub fn read_eq(s: &mut String) -> Result<Term, ()> {
    read_str(s, "=")?;
    let t = Type::Arrow(box Type::I32,
                        box Type::Arrow(box Type::I32,
                                        box Type::Bool));
    Ok(Term::BuildIn(BuildIns::Eq2I(t)))
}

pub fn read_eq_b(s: &mut String) -> Result<Term, ()> {
    read_str(s, "eq")?;
    let t = Type::Arrow(box Type::Bool,
                        box Type::Arrow(box Type::Bool,
                                        box Type::Bool));
    Ok(Term::BuildIn(BuildIns::Eq2B(t)))
}

pub fn read_build_in(s: &mut String) -> Result<Term, ()> {
    if let Ok(t) = read_inc(s) {
        return Ok(t);
    }
    if let Ok(t) = read_dec(s) {
        return Ok(t);
    }
    if let Ok(t) = read_zerop(s) {
        return Ok(t);
    }
    if let Ok(t) = read_eq(s) {
        return Ok(t);
    }
    if let Ok(t) = read_eq_b(s) {
        return Ok(t);
    }
    Err(())
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
                   locals: &mut HashSet<String>,
                   vars: &mut HashMap<String, Term>) -> Result<Term, ()> {
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
        let term = read_term(s, &mut context_new, locals, vars)?;
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
                locals: &mut HashSet<String>,
                vars: &mut HashMap<String, Term>) -> Result<Term, ()> {
    read_char(s, '(')?;
    let mut t_1 = read_term(s, context, locals, vars)?;
    if s.len() == 0 {
        return Err(());
    }
    read_char(s, ' ')?;
    let mut t_2 = read_term(s, context, locals, vars)?;
    read_char(s, ')')?;
    if let Term::Var(var) = t_1 {
        locals.insert(var.var.clone());
        t_1 = Term::Var(var);
    }
    if let Term::Var(var) = t_2 {
        locals.insert(var.var.clone());
        t_2 = Term::Var(var);
    }
    let app = App::new(t_1, t_2, context);
    match app {
        Ok(app) => Ok(Term::App(app)),
        Err(_)  => Err(()),
    }
}

pub fn read_term(s: &mut String,
                 context: &mut HashMap<String, Type>,
                 locals: &mut HashSet<String>,
                 vars: &mut HashMap<String, Term>) -> Result<Term, ()> {
    
    if let Ok(Term::ValI32(v)) = read_val_i32(s) {
        return Ok(Term::ValI32(v));
    }
    if let Ok(Term::ValBool(v)) = read_true(s) {
        return Ok(Term::ValBool(v));
    }
    if let Ok(Term::ValBool(v)) = read_false(s) {
        return Ok(Term::ValBool(v));
    }
    if let Ok(Term::BuildIn(b)) = read_build_in(s) {
        return Ok(Term::BuildIn(b));
    }
    if let Ok(Term::Var(v)) = read_var(s, context, false) {
        match vars.get(&v.var) {
            Some(t) => {
                return Ok(t.clone());
            },
            None => {
                return Ok(correct(Term::Var(v), context));
            },
        }
    }
    if let Ok(Term::Lambda(l)) = read_lambda(s, context, locals, vars) {
        return Ok(correct(Term::Lambda(l), context));
    }
    read_app(s, context, locals, vars).map(|x| correct(x, context))
}

fn correct(term: Term, context: &mut HashMap<String, Type>) -> Term {
    match term {
        Term::Var(mut var) => {
            var.t = context.get(&var.var).unwrap().clone();
            Term::Var(var)
        },
        Term::Lambda(lambda) => {
            let mut var = lambda.var;
            var.t = context.get(&var.var).unwrap().clone();
            let term = correct(*lambda.term, context);
            let lambda = Lambda::new(var, term);
            Term::Lambda(lambda)
        },
        Term::App(app) => {
            let f   = correct(*app.fun , context);
            let t   = correct(*app.term, context);
            let app = App::new(f, t, context).unwrap();
            Term::App(app)
        },
        t => t,
    }
}
