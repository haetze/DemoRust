#![allow(dead_code)]
#![feature(box_patterns)]


use std::io;
use std::io::Write;
use std::io::BufRead;
use std::collections::HashMap;
use std::cmp::Ordering;
use std::env;
use std::fs::File;
use std::io::BufReader;



type Var = String;

enum Kind {
    Quit,
    Reload,
    Term(String),
    Set(String),
}

#[derive(Clone, Debug, PartialEq)]
enum Term {
    Lambda(Var, Box<Term>),
    App(Box<Term>, Box<Term>),
    Var(Var),
    Val(i32),
    Inc,
    Dec,
    Eq,
    GT,
}

impl PartialOrd for Term {
    fn partial_cmp(&self, other: &Term) -> Option<Ordering> {
        use Term::*;
        
        match *self {
            Val(i) => match *other {
                Val(j) => i.partial_cmp(&j),
                _      => None,
            },
            _      => None,
        }
    }
}


impl Term {

    fn show(&self) -> String {
        use Term::*;
        
        let mut string = String::new();
        match self {
            Lambda(v, t) => {
                let var = format!("{}", v);
                let term = t.show();
                string.push_str("(λ");
                string.push_str(&var);
                string.push_str(".");
                string.push_str(&term);
                string.push_str(")");
            },
            App(t, s) => {
                let term_1 = t.show();
                let term_2 = s.show();
                string.push_str("(");
                string.push_str(&term_1);
                string.push_str(" ");
                string.push_str(&term_2);
                string.push_str(")");
            },
            Var(v) => {
                let var = format!("{}", v);
                string.push_str(&var);
            },
            Val(val) => {
                let val = format!("{}", val);
                string.push_str(&val);
            },
            Inc => string.push_str(&"inc"),
            Dec => string.push_str(&"inc"),
            Eq  => string.push_str(&"="),
            GT  => string.push_str(&">"),

        }
        string
    }
    
    fn free(&mut self, v: Var) -> bool {
        match self {
            Term::Lambda(u, t) => v != *u && t.free(v),
            Term::App(t, s)    => t.free(v.clone()) && s.free(v),
            Term::Var(u)       => v != *u,
            Term::Val(_)       => true,
            Term::Inc          => true,
            Term::Dec          => true,
            Term::Eq           => true,
            Term::GT           => true,
        }
    }

    fn replace_vars(self, map: &HashMap<Var, Term>) -> Term {
        let mut term = self;
        for (key, value) in map {
            term = term.replace_var(key.clone(), value);
        }
        return term;
    }

    fn replace_var(self, v: Var, t: &Term) -> Term {
        match self {
            Term::Lambda(u, s)       => {
                if v != u {
                    Term::Lambda(u, Box::new(s.replace_var(v, &t.clone())))
                } else {
                    Term::Lambda(u, s)
                }
            },
            Term::App(s, u)              => {
                Term::App(Box::new(s.replace_var(v.clone(), &t.clone())),
                          Box::new(u.replace_var(v.clone(), &t.clone())))
            },
            Term::Var(u)                 => {
                if u == v {
                    return t.clone();
                } else {
                    return Term::Var(u);
                }
            },
            s                            => s,

        }
    }

    fn one_step_eval(self, vars: &HashMap<Var, Term>) -> Term {
        match self {
            Term::App(box Term::App(box Term::Eq, box Term::Val(i)), box Term::Val(j)) => {
                if i == j {
                    return Term::true_func();
                } else {
                    return Term::false_func();
                }
            },
            Term::App(box Term::App(box Term::GT, box Term::Val(i)), box Term::Val(j)) => {
                if i < j {
                    return Term::true_func();
                } else {
                    return Term::false_func();
                }
            },
            Term::App(t, s) => {
                match *t {
                    Term::Lambda(ref v, ref t) => t.clone().replace_var(v.clone(), &s),
                    Term::Inc          => {
                        match *s {
                            Term::Val(v) => Term::Val(v+1),
                            s            => Term::App(t,
                                                      Box::new(s.one_step_eval(vars))),
                        }
                    },
                    Term::Dec          => {
                        match *s {
                            Term::Val(v) => Term::Val(v-1),
                            s            => Term::App(t,
                                                      Box::new(s.one_step_eval(vars))),
                        }
                    },
                    t                    => Term::App(Box::new(t.one_step_eval(vars)),                                                 
                                                    Box::new(s.one_step_eval(vars))),
                }
            },
            Term::Lambda(v, t)         => Term::Lambda(v, Box::new(t.one_step_eval(vars))),
            s                          => s,
        }
    }

    fn eval(self, vars: &HashMap<Var, Term>) -> Term {
        let mut t_1 = self;
        let mut t_2     = t_1.clone();
        loop {
            t_1 = t_1.one_step_eval(vars).replace_vars(vars);
            if t_1 == t_2 {
                break;
            }
            t_2 = t_1.clone();
        }
        t_1
    }

    fn zero() -> Term {
        use Term::*;
        Lambda("f".to_string(),
               Box::new(Lambda("i".to_string(),
                               Box::new(Var("i".to_string())))))
    }

    fn succ() -> Term {
        use Term::*;
        Lambda("n".to_string(),
               Box::new(Lambda("f".to_string(),
                               Box::new(Lambda("i".to_string(),
                                               Box::new(App(Box::new(Var("f".to_string())),
                                                            Box::new(App(Box::new(App(Box::new(Var("n".to_string())),
                                                                                      Box::new(Var("f".to_string())))),
                                                                         Box::new(Var("i".to_string())))))))))))
    }

    fn inc(self, vars: &HashMap<Var, Term>) -> Term {
        let succ = Term::succ();
        Term::App(Box::new(succ), Box::new(self)).eval(vars)
    }

    fn true_func() -> Term {
        use Term::*;
        Lambda("x".to_string(),
               Box::new(Lambda("y".to_string(),
                               Box::new(Var("x".to_string())))))
    }

    fn false_func() -> Term {
        use Term::*;
        Lambda("x".to_string(),
               Box::new(Lambda("y".to_string(),
                               Box::new(Var("y".to_string())))))
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
            match Term::read_char(s, i) {
                Ok(_) => read.push(i),
                Err(_) => {
                    s.insert_str(0, &read);
                    return Err(());
                },
            }               
        }
        return Ok(());
    }

    fn read_inc(s: &mut String) -> Result<Term, ()> {
        Term::read_str(s, "inc")?;
        Ok(Term::Inc)
    }

    fn read_dec(s: &mut String) -> Result<Term, ()> {
        Term::read_str(s, "dec")?;
        Ok(Term::Dec)
    }

    fn read_eq(s: &mut String) -> Result<Term, ()> {
        Term::read_str(s, "=")?;
        Ok(Term::Eq)
    }

    fn read_gt(s: &mut String) -> Result<Term, ()> {
        Term::read_str(s, ">")?;
        Ok(Term::GT)
    }



    fn read_var(s: &mut String) -> Result<Term, ()>  {
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
            return Ok(Term::Var(st));
        }
    }

    fn read_val(s: &mut String) -> Result<Term, ()>  {
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
        return Ok(Term::Val(st.parse().unwrap()));
    }

    fn read_lambda(s: &mut String) -> Result<Term, ()> {
        Term::read_char(s, '(')?;
        match Term::read_char(s, 'λ') {
            Err(_) => {
                s.insert(0, '(');
                return Err(());
            },
            _ => (),
        };
        if let Term::Var(var) = Term::read_var(s)? {
            Term::read_char(s, '.')?;
            let term = Term::read_term(s)?;
            Term::read_char(s, ')')?;
            return Ok(Term::Lambda(var, Box::new(term)));
        }
        return Err(());
    }

    fn read_app(s: &mut String) -> Result<Term, ()> {
        Term::read_char(s, '(')?;
        let t_1 = Term::read_term(s)?;
        if s.len() == 0 {
            return Err(());
        }
        Term::read_char(s, ' ')?;
        let t_2 = Term::read_term(s)?;
        Term::read_char(s, ')')?;
        Ok(Term::App(Box::new(t_1), Box::new(t_2)))
    }

    fn read_term(s: &mut String) -> Result<Term, ()> {
        if let Ok(Term::Inc) = Term::read_inc(s) {
            return Ok(Term::Inc);
        }
        if let Ok(Term::Dec) = Term::read_dec(s) {
            return Ok(Term::Dec);
        }
        if let Ok(Term::Eq) = Term::read_eq(s) {
            return Ok(Term::Eq);
        }
        if let Ok(Term::GT) = Term::read_gt(s) {
            return Ok(Term::GT);
        }
        if let Ok(Term::Val(v)) = Term::read_val(s) {
            return Ok(Term::Val(v));
        }
        if let Ok(Term::Var(v)) = Term::read_var(s) {
            return Ok(Term::Var(v));
        }
        if let Ok(Term::Lambda(v, t)) = Term::read_lambda(s) {
            return Ok(Term::Lambda(v, t));
        }
        Term::read_app(s)
    }

    
        
}

fn read_kind(mut st: String) -> Kind {
    if st.as_str() == ":q" {
        return Kind::Quit;
    }
    if st.as_str() == ":r" {
        return Kind::Reload;
    }
    if st.as_str().starts_with(":set") {
        st.remove(0);
        st.remove(0);
        st.remove(0);
        st.remove(0);
        st.remove(0);
        return Kind::Set(st);
    }
    return Kind::Term(st);

}


fn handle_line(line: Result<String, std::io::Error>,
               vars: &mut HashMap<Var, Term>,
               paths: &Vec<String>) -> bool {
    
    match line {
        Ok(exp_string) => {
            match read_kind(exp_string) {
                Kind::Quit => {
                    println!("Quitting..");
                    return true;
                },
                Kind::Reload => {
                    println!("Reloading..");
                    let mut map = HashMap::new();
                    for path in paths.iter() {
                        read_in_file(path, &mut map);
                    }
                    std::mem::swap(vars, &mut map);
                    return false;
                },
                Kind::Term(mut exp_string) => {
                    match Term::read_term(&mut exp_string) {
                        Err(()) => println!(">!>!>!>! Error"),
                        Ok(exp) => {
                            let exp = exp.eval(vars);
                            println!(">=>=>=>= {}", exp.show())
                        },
                    };
                },
                Kind::Set(mut st) => {
                    if let Ok(Term::Var(var)) = Term::read_var(&mut st) {
                        st.remove(0);
                        match Term::read_term(&mut st) {
                            Err(()) => println!(">!>!>!>! Error in Term"), 
                            Ok(exp) => {
                                println!(">=>=>=>= :set {} <- {}", var, exp.show());
                                vars.insert(var, exp);
                            },
                        };
                    } else {
                        println!(">!>!>!>! Error in Var");
                    }
                },
            }
        },
        Err(_) => println!("Read Error"),
    }
    
    return false;
}

fn read_in_file(path: &String, vars: &mut HashMap<Var, Term>) {
    let file = File::open(path).expect("file not found");
    let file = BufReader::new(&file);
    for line in file.lines() {
        handle_line(line, vars, &Vec::new());
    }
    
}


fn main() -> Result<(), ()>{
    let mut vars: HashMap<Var, Term> = HashMap::new();
    let paths: Vec<String>           = env::args().skip(1).collect();
    
    for path in &paths {
        read_in_file(path, &mut vars);
    }
    
    let stdin = io::stdin();

    print!("<=<=<=<= ");
    io::stdout().flush().ok();
    
    for line in stdin.lock().lines() {
        
        if handle_line(line, &mut vars, &paths) {
            break;
        }
        print!("<=<=<=<= ");
        io::stdout().flush().ok();
 
    }

    Ok(())

}
