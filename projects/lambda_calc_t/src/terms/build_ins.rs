use terms::Term;
use terms::types::Type;
use terms::valbool::ValBool;
use terms::vali32::ValI32;
use terms::show::Show;
use terms::eval::Evaluate;
use terms::typable::Typable;

use std::collections::HashMap;


#[derive(Debug, Clone, PartialEq)]
pub enum BuildIns {
    Inc(Type),
    Dec(Type),
    Zerop(Type),
    Eq2(Type),
    Eq1(Type, Box<Term>),
    Eq0(Type, Box<Term>, Box<Term>),
    Add2(Type),
    Add1(Type, Box<Term>),
    Add0(Type, Box<Term>, Box<Term>),
    Mult2(Type),
    Mult1(Type, Box<Term>),
    Mult0(Type, Box<Term>, Box<Term>),
    ITE3(Type),
    ITE2(Type, bool),
    ITE1(Type, bool, Box<Term>),
}

impl BuildIns {
    pub fn to_fun(self) -> Box<Fn(Term) -> Term> {
        match self {
            BuildIns::Eq2(_) => {
                box (|t| {
                    match t {
                        t => {
                            let t_t = t.get_type().clone();
                            let typ = Type::Arrow(box t_t.clone(), box Type::Bool);
                            let b = BuildIns::Eq1(typ, box t);
                            Term::BuildIn(b)
                        },
                    }
                })
            },
            BuildIns::Eq1(_, i) => {
                box (move |t| {
                    match t {
                        t   => {
                            let b = BuildIns::Eq0(Type::Bool,
                                                   i.clone(),
                                                   box t);
                            Term::BuildIn(b)
                        },
                    }
                })
            },
            BuildIns::ITE3(typ) => {
                box (move |t| {
                    if let Type::Arrow(box Type::Bool, box Type::Arrow(b, c)) = typ.clone() {
                        match t {
                            Term::ValBool(v) => {
                                let t = Type::Arrow(b, c);
                                let b = BuildIns::ITE2(t, v.val);

                                Term::BuildIn(b)
                                
                            },
                            t   => t,
                        }
                    } else {
                        panic!("Big Problem");
                    }
                })
            },
            BuildIns::ITE2(typ, bl) => {
                box (move |t| {
                    if let Type::Arrow(box _a, box Type::Arrow(b, c)) = typ.clone() {   
                        let typ = Type::Arrow(b, c);
                        let b = BuildIns::ITE1(typ, bl, box t);
                        Term::BuildIn(b)
                    } else {
                        panic!("Big Problem");
                    }
                })
            },
            BuildIns::ITE1(_, b, t_1) => {
                box (move |t| {
                    if b {
                        *t_1.clone()
                    } else {
                        t
                    }
                })
            },
            BuildIns::Add2(_) => {
                box (|t| {
                    match t {
                        t => {
                            let typ = Type::Arrow(box Type::I32, box Type::I32);
                            let b = BuildIns::Add1(typ, box t);
                            Term::BuildIn(b)
                        },
                    }
                })
            },
            BuildIns::Add1(_, i) => {
                box (move |t| {
                    match t {
                        t   => {
                            let b = BuildIns::Add0(Type::I32,
                                                   i.clone(),
                                                   box t);
                            Term::BuildIn(b)
                        },
                    }
                })
            },
            BuildIns::Mult2(_) => {
                box (|t| {
                    match t {
                        t => {
                            let typ = Type::Arrow(box Type::I32, box Type::I32);
                            let b = BuildIns::Mult1(typ, box t);
                            Term::BuildIn(b)
                        },
                    }
                })
            },
            BuildIns::Mult1(_, i) => {
                box (move |t| {
                    match t {
                        t => {
                            let b = BuildIns::Mult0(Type::I32,
                                                   i.clone(),
                                                   box t);
                            Term::BuildIn(b)
                        },
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

            t => {
                box (move |_| Term::BuildIn(t.clone()))
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
            BuildIns::Eq2(t)  => t,
            BuildIns::Eq1(t,_)  => t,
            BuildIns::Eq0(t,_,_)  => t,
            BuildIns::Add2(t)  => t,
            BuildIns::Add1(t,_)  => t,
            BuildIns::Add0(t,_,_)  => t,
            BuildIns::Mult2(t)  => t,
            BuildIns::Mult1(t,_)  => t,
            BuildIns::Mult0(t,_,_)  => t,
            BuildIns::ITE3(t)  => t,
            BuildIns::ITE2(t,_)  => t,
            BuildIns::ITE1(t,_,_)  => t,

        }
    }
}

impl Show for BuildIns {
    fn show(&self) -> String {
        match self {
            BuildIns::Inc(_) => "inc".to_string(),
            BuildIns::Dec(_) => "dec".to_string(),
            BuildIns::Zerop(_) => "zerop".to_string(),
            BuildIns::Eq2(_) => "=".to_string(),
            BuildIns::Eq1(_,t) => format!("{}=", t.show()),
            BuildIns::Eq0(_,t,s) => format!("{}={}", t.show(), s.show()),
            BuildIns::Add2(_) => "+".to_string(),
            BuildIns::Add1(_,t) => format!("+{}", t.show()),
            BuildIns::Add0(_,t,s) => format!("{}+{}", t.show(), s.show()),
            BuildIns::Mult2(_) => "*".to_string(),
            BuildIns::Mult1(_,t) => format!("*{}", t.show()),
            BuildIns::Mult0(_,t,s) => format!("{}*{}", t.show(), s.show()),
            BuildIns::ITE3(_) => format!("if"),
            BuildIns::ITE2(_, b) => format!("if {}", b),
            BuildIns::ITE1(_, b, t) => format!("if {} then {}", b, t.show()),

        }
    }
}

impl Evaluate for BuildIns {
    fn eval(self, context: &mut HashMap<String, Term>) -> Term{
        match self {
            BuildIns::Add0(t,
                           mut a,
                           mut b) => {
                loop {
                    let a_ = a.clone();
                    a = box a.eval(context);
                    if a == a_ {
                        break;
                    }
                }

                loop {
                    let b_ = b.clone();
                    b = box b.eval(context);
                    if b == b_ {
                        break;
                    }
                }

                if let Term::ValI32(v) = *a.clone() {
                    if let Term::ValI32(w) = *b.clone() {
                        return Term::ValI32(ValI32::new(v.val + w.val));
                    }
                }       
                Term::BuildIn(BuildIns::Add0(t,
                                             a,
                                             b))
            },
            BuildIns::Mult0(t,
                           mut a,
                           mut b) => {
                loop {
                    let a_ = a.clone();
                    a = box a.eval(context);
                    if a == a_ {
                        break;
                    }
                }

                loop {
                    let b_ = b.clone();
                    b = box b.eval(context);
                    if b == b_ {
                        break;
                    }
                }

                if let Term::ValI32(v) = *a.clone() {
                    if let Term::ValI32(w) = *b.clone() {
                        return Term::ValI32(ValI32::new(v.val * w.val));
                    }
                }       

                Term::BuildIn(BuildIns::Mult0(t,
                                             a,
                                             b))
            },
            BuildIns::Eq0(_,
                           mut a,
                           mut b) => {
                loop {
                    let a_ = a.clone();
                    a = box a.eval(context);
                    if a == a_ {
                        break;
                    }
                }

                loop {
                    let b_ = b.clone();
                    b = box b.eval(context);
                    if b == b_ {
                        break;
                    }
                }

                return Term::ValBool(ValBool::new(a == b));
                
            },
            t => Term::BuildIn(t),
        }
         
    }
}
