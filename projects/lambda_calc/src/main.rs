type Var = u32;

#[derive(Clone, Debug)]
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

    fn onestep_eval(self) -> Term {
        match self {
            Term::App(t, s) => {
                match *t {
                    Term::Lambda(v, t) => t.replace_var(v, *s),
                    t                  => panic!("Can't eval {:?}", t),
                }
            },
            t               => panic!("Can't eval {:?}", t),   
        }
    }
    
}

fn main() {
    use Term::*;
    let var_0 = Box::new(Var(0));
    let var_1 = Box::new(Var(1));
    let lambda = Box::new(Lambda(0, var_0));
    let exp = App(lambda, var_1);
    println!("{:?}", exp);
    let exp = exp.onestep_eval();
    println!("{:?}", exp);
}
