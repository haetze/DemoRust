use terms::types::Type;
use terms::vali32::ValI32;
use terms::valbool::ValBool;
use terms::var::Var;
use terms::lambda::Lambda;
use terms::app::App;
use terms::build_ins::BuildIns;
use terms::Term;
use terms::matching::Match;
use terms::matching::Matches;
use std::collections::HashSet;
use std::collections::HashMap;

use terms::types::free_type_var;

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

pub fn read_match(s: &mut String,
                  context:& mut HashMap<String, Type>,
                  locals: &mut HashSet<String>,
                  vars: &mut HashMap<String, Term>) -> Result<Term, ()> {
    read_str(s, "(μ(")?;
    
    let mut matches: Option<Matches> = None;
    loop {

        read_str(s, "(")?;
        let t = read_term(s, context, locals, vars)?;
        read_str(s, " -> ")?;
        let r = read_term(s, context, locals, vars)?;
        read_str(s, ")")?;
        let m = Match::new(Term::ValBool(ValBool::new(false)),
                           t,
                           r);
        if let None = matches.clone() {
            matches = Some(Matches::new(m));
        } else {
            let mut ma = matches.clone().unwrap();
            match ma.add(m) {
                None => return Err(()),
                _    => {
                    matches = Some(ma);
                },
            };
        }

        if let Err(()) = read_str(s, "))") {
            continue;
        } else {
            break;
        }
    }

    Ok(Term::Match(matches.unwrap()))
    
    
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

pub fn read_eq(s: &mut String, context:& mut HashMap<String, Type>) -> Result<Term, ()> {
    read_str(s, "=")?;
    let typ = free_type_var(context);
    let t = Type::Arrow(box typ.clone(),
                        box Type::Arrow(box typ.clone(),
                                        box Type::Bool));
    Ok(Term::BuildIn(BuildIns::Eq2(t)))
}

pub fn read_ite(s: &mut String, context: &mut HashMap<String, Type>) -> Result<Term, ()> {
    read_str(s, "if")?;
    let typ = free_type_var(context);
    let t = Type::Arrow(box Type::Bool,
                        box Type::Arrow(box typ.clone(),
                                        box Type::Arrow(box typ.clone(),
                                                        box typ.clone())));
    Ok(Term::BuildIn(BuildIns::ITE3(t)))
}


pub fn read_add(s: &mut String) -> Result<Term, ()> {
    read_str(s, "+")?;
    let t = Type::Arrow(box Type::I32,
                        box Type::Arrow(box Type::I32,
                                        box Type::I32));
    Ok(Term::BuildIn(BuildIns::Add2(t)))
}

pub fn read_mult(s: &mut String) -> Result<Term, ()> {
    read_str(s, "*")?;
    let t = Type::Arrow(box Type::I32,
                        box Type::Arrow(box Type::I32,
                                        box Type::I32));
    Ok(Term::BuildIn(BuildIns::Mult2(t)))
}

pub fn read_build_in(s: &mut String,
                     context: &mut HashMap<String, Type>) -> Result<Term, ()> {
    if let Ok(t) = read_inc(s) {
        return Ok(t);
    }
    if let Ok(t) = read_dec(s) {
        return Ok(t);
    }
    if let Ok(t) = read_zerop(s) {
        return Ok(t);
    }
    if let Ok(t) = read_eq(s, context) {
        return Ok(t);
    }
    if let Ok(t) = read_add(s) {
        return Ok(t);
    }
    if let Ok(t) = read_mult(s) {
        return Ok(t);
    }
    if let Ok(t) = read_ite(s, context) {
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
    let mut sign = 1;
    if let Ok(()) = read_char(s, '-') {
        sign = -1;
    }
    if s.as_str().starts_with(" ") {
        s.insert(0, '-');
        return Err(());
    }
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
    let val: i32 = st.parse().unwrap();
    let v = ValI32::new(sign * val);
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
    let t_1 = correct(t_1, context);
    let t_2 = correct(t_2, context);
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
    let t;
    if let Ok(Term::ValI32(v)) = read_val_i32(s) {
        t = Ok(Term::ValI32(v));
    } else if let Ok(Term::ValBool(v)) = read_true(s) {
        t = Ok(Term::ValBool(v));
    } else if let Ok(Term::ValBool(v)) = read_false(s) {
        t = Ok(Term::ValBool(v));
    } else if let Ok(Term::BuildIn(b)) = read_build_in(s, context) {
        t = Ok(Term::BuildIn(b));
    } else if let Ok(Term::Var(v)) = read_var(s, context, false) {
        match vars.get(&v.var) {
            Some(s) => {
                t = Ok(s.clone());
            },
            None => {
                let v = correct(Term::Var(v), context);
                t = Ok(v);
            },
        }
    } else if let Ok(Term::Lambda(l)) = read_lambda(s, context, locals, vars) {
        t = Ok(correct(Term::Lambda(l), context));
    } else {
        t = read_app(s, context, locals, vars).map(|x| correct(x, context));
    }
    return t;
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
