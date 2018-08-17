#![allow(dead_code)]

type Var = u32;

#[derive(Clone, Debug, PartialEq)]
enum Term {
    Lambda(Var, Box<Term>),
    App(Box<Term>, Box<Term>),
    Var(Var),
}

impl Term {
    
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
        Lambda(1, Box::new(Lambda(0, Box::new(Var(0)))))
    }

    fn succ() -> Term {
        use Term::*;
        Lambda(2,
               Box::new(Lambda(3,
                               Box::new(Lambda(4,
                                               Box::new(App(Box::new(Var(3)),
                                                            Box::new(App(Box::new(App(Box::new(Var(2)),
                                                                                      Box::new(Var(3)))),
                                                                         Box::new(Var(4)))))))))))
    }
        
}

fn main() {
    use Term::*;
    
    let zero = Term::zero();
    let succ = Term::succ();
    let one  = App(Box::new(succ), Box::new(zero));
    println!("{:?}", one);
    let one = one.eval();
    println!("{:?}", one);
    let succ = Term::succ();
    let two  = App(Box::new(succ), Box::new(one)).eval();
    println!("{:?}", two);
}
