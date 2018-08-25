use terms::Term;
use terms::types::Type;
use terms::valbool::ValBool;
use terms::vali32::ValI32;
use terms::show::Show;
use terms::typable::Typable;


#[derive(Debug, Clone, PartialEq)]
pub enum BuildIns {
    Inc(Type),
    Dec(Type),
    Zerop(Type),
    Eq2I(Type),
    Eq1I(Type, i32),
    Eq2B(Type),
    Eq1B(Type, bool),
    Add2(Type),
    Add1(Type, i32),
    Mult2(Type),
    Mult1(Type, i32),
    ITE3(Type),
    ITE2(Type, bool),
    ITE1(Type, bool, Box<Term>),
}

impl BuildIns {
    pub fn to_fun(self) -> Box<Fn(Term) -> Term> {
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
                        Term::ValI32(v) => {
                            let t = Type::Arrow(box Type::I32, box Type::I32);
                            let b = BuildIns::Add1(t, v.val);
                            Term::BuildIn(b)
                        },
                        t   => t,
                    }
                })
            },
            BuildIns::Add1(_, i) => {
                box (move |t| {
                    match t {
                        Term::ValI32(v) => {
                            let b = ValI32::new(i + v.val);
                            Term::ValI32(b)
                        },
                        t   => t,
                    }
                })
            },
            BuildIns::Mult2(_) => {
                box (|t| {
                    match t {
                        Term::ValI32(v) => {
                            let t = Type::Arrow(box Type::I32, box Type::I32);
                            let b = BuildIns::Mult1(t, v.val);
                            Term::BuildIn(b)
                        },
                        t   => t,
                    }
                })
            },
            BuildIns::Mult1(_, i) => {
                box (move |t| {
                    match t {
                        Term::ValI32(v) => {
                            let b = ValI32::new(i * v.val);
                            Term::ValI32(b)
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
            BuildIns::Add2(t)  => t,
            BuildIns::Add1(t,_)  => t,
            BuildIns::Mult2(t)  => t,
            BuildIns::Mult1(t,_)  => t,
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
            BuildIns::Eq2I(_) => "=".to_string(),
            BuildIns::Eq1I(_,t) => format!("{}=", t),
            BuildIns::Eq2B(_) => "eq".to_string(),
            BuildIns::Eq1B(_,t) => format!("{} eq", t),
            BuildIns::Add2(_) => "+".to_string(),
            BuildIns::Add1(_,t) => format!("+{}", t),
            BuildIns::Mult2(_) => "*".to_string(),
            BuildIns::Mult1(_,t) => format!("*{}", t),
            BuildIns::ITE3(_) => format!("if"),
            BuildIns::ITE2(_, b) => format!("if {}", b),
            BuildIns::ITE1(_, b, t) => format!("if {} then {}", b, t.show()),

        }
    }
}
