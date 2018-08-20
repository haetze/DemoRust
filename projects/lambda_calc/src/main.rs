#![allow(dead_code)]

type Var = char;

#[derive(Clone, Debug, PartialEq)]
enum Term {
    Lambda(Var, Box<Term>),
    App(Box<Term>, Box<Term>),
    Var(Var),
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
        }
        string
    }
    
    fn free(&mut self, v: Var) -> bool {
        match self {
            Term::Lambda(u, t) => v != *u && t.free(v),
            Term::App(t, s)    => t.free(v) && s.free(v),
            Term::Var(u)       => v != *u,
        }
    }

    fn replace_var(self, v: Var, t: Term) -> Term {
        match self {
            Term::Lambda(u, s)       => {
                if v != u {
                    Term::Lambda(u, Box::new(s.replace_var(v, t)))
                } else {
                    Term::Lambda(u, s)
                }
            },
            Term::App(s, u)              => {
                Term::App(Box::new(s.replace_var(v, t.clone())),
                          Box::new(u.replace_var(v, t)))
            },
            Term::Var(u)       if v == u => t,
            s                            => s,

        }
    }

    fn one_step_eval(self) -> Term {
        match self {
            Term::App(t, s) => {
                match *t {
                    Term::Lambda(v, t) => t.replace_var(v, *s),
                    t                  => Term::App(Box::new(t.one_step_eval()),
                                                    Box::new(s.one_step_eval())),
                }
            },
            Term::Lambda(v, t)         => Term::Lambda(v, Box::new(t.one_step_eval())),
            Term::Var(v)               => Term::Var(v),
        }
    }

    fn n_step_eval(self, u: u32) -> Term {
        let mut t = self;
        for _ in 0..u {
            t = t.one_step_eval();
        }
        t
    }

    fn eval(self) -> Term {
        let mut t_1 = self;
        let mut t_2     = t_1.clone();
        loop {
            t_1 = t_1.one_step_eval();
            if t_1 == t_2 {
                break;
            }
            t_2 = t_1.clone();
        }
        t_1
    }

    fn zero() -> Term {
        use Term::*;
        Lambda('f', Box::new(Lambda('i', Box::new(Var('i')))))
    }

    fn succ() -> Term {
        use Term::*;
        Lambda('n',
               Box::new(Lambda('f',
                               Box::new(Lambda('i',
                                               Box::new(App(Box::new(Var('f')),
                                                            Box::new(App(Box::new(App(Box::new(Var('n')),
                                                                                      Box::new(Var('f')))),
                                                                         Box::new(Var('i')))))))))))
    }

    fn inc(self) -> Term {
        let succ = Term::succ();
        Term::App(Box::new(succ), Box::new(self)).eval()
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

    fn read_var(s: &mut String) -> Result<Term, ()>  {
        if s.len() == 0 {
            return Err(());
        }
        let head = s.remove(0);
        if head.is_alphabetic() {
            return Ok(Term::Var(head));
        }
        s.insert(0, head);
        return Err(());
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
        if let Ok(Term::Var(v)) = Term::read_var(s) {
            return Ok(Term::Var(v));
        }
        if let Ok(Term::Lambda(v, t)) = Term::read_lambda(s) {
            return Ok(Term::Lambda(v, t));
        }
        Term::read_app(s)
    }

    
        
}

fn main() -> Result<(), ()>{
    use Term::*;
    
    let mut x = "(λf.(λi.(f i)))".to_string();
    let term = Term::read_term(&mut x)?;
    println!("{}", term.show());
    Ok(())
}
