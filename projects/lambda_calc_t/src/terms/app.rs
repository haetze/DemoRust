use terms::Term;
use terms::types::Type;
use terms::build_ins::BuildIns;
use terms::types::type_error::TypeError;
use terms::show::Show;
use terms::typable::Typable;
use terms::eval::Evaluate;
use std::collections::HashMap;


#[derive(Debug, Clone, PartialEq)]
pub struct App{
    pub fun : Box<Term>,
    pub term: Box<Term>,
    pub t: Type,
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
            App{fun: box Term::BuildIn(BuildIns::ITE3(typ)),
                term: box t,
                t: _typ
            } => {
                let mut m = t;
                loop {
                    m = m.eval(context);
                    if let Term::ValBool(v) = m {
                        m = Term::ValBool(v);
                        break;
                    }
                }
                let t = BuildIns::ITE3(typ.clone()).to_fun()(m);
                t.eval(context)
            },
            App{fun: box Term::BuildIn(BuildIns::ITE2(ref typ, ref b)),
                term: ref t,
                t: ref _typ
            } => {
                let t = BuildIns::ITE2(typ.clone(), b.clone()).to_fun()(*t.clone());
                t.eval(context)
            },
            App{fun: box Term::BuildIn(BuildIns::ITE1(ref typ, ref b, ref t_1)),
                term: ref t,
                t: ref _typ
            } => {
                let t = BuildIns::ITE1(typ.clone(), b.clone(), t_1.clone()).to_fun()(*t.clone());
                let t = t.eval(context);
                t
                    
            },
            App{fun: box Term::BuildIn(b),
                term: box Term::Var(v),
                t: typ
            } => {
                
                Term::App(App {
                    fun: box Term::BuildIn(b),
                    term: box Term::Var(v).eval(context),
                    t: typ,
                }).eval(context)
            },
            App{fun: box Term::BuildIn(b),
                term: box t,
                t: _typ
            } => {
                let t = t.eval(context);
                let t = b.to_fun()(t);
                t.eval(context)
            },
            App{fun: box Term::Lambda(lambda),
                term: box t,
                t: _typ
            } => {
                let var = lambda.var;
                let term_fun = lambda.term;
                let m = t.eval(context);
                let tmp_t = context.remove(&var.var.clone());
                context.insert(var.var.clone(), m.clone());
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
                let result_s = *s.clone();
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


impl Typable for App {
    fn get_type(&self) -> &Type {
        &self.t
    }
}

impl Show for App {
    fn show(&self) -> String {
        format!("({} {})",
                self.fun.show(),
                self.term.show())
    }
}
