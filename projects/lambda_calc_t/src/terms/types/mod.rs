pub mod type_error;


use terms::show::Show;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Arrow(Box<Type>, Box<Type>),
    I32,
    Bool,
    Var(u32),
}

impl Type {

    pub fn match_t(t: Type, s: Type) -> Option<Type> {
        use Type::*;
        match t.clone() {
            I32 => {
                match s.clone() {
                    Var(_) => Some(I32),
                    _      => None,
                }
            },
            Bool => {
                match s.clone() {
                    Var(_) => Some(Bool),
                    _      => None,
                }
            },
            Var(_) => Some(s),
            Arrow(t_1, t_2) => {
                match s.clone() {
                    Var(_) => Some(Arrow(t_1, t_2)),
                    Arrow(s_1, s_2) => {
                        let r_1 = Type::match_t(*t_1, *s_1)?;
                        let r_2 = Type::match_t(*t_2, *s_2)?;
                        Some(Arrow(box r_1, box r_2))
                    },
                    _      => None,
                }
            },
        }
    }
    pub fn replace_var(self, i: u32, t: Type) -> Type {
        use Type::*;
        match self {
            I32 => I32,
            Bool => Bool,
            Arrow(t_, s_) => Arrow(box (*t_).replace_var(i, t.clone()),
                                   box (*s_).replace_var(i, t.clone())),
            Var(u) if u == i => t,
            m  => m,
        }
                                                  
    }
}



impl Show for Type {
    fn show(&self) -> String {
        match self {
            Type::I32 => "I32".to_string(),
            Type::Bool => "Bool".to_string(),
            Type::Arrow(t, s) => {
                let mut string = String::new();
                string.push_str("(");
                string.push_str(&t.show());
                string.push_str(" -> ");
                string.push_str(&s.show());
                string.push_str(")");
                string
            },
            Type::Var(i) => {
                let mut string = String::new();
                string.push_str("t_");
                string.push_str(&format!("{}", i));
                string
            },
        }
    }

}

pub fn free_type_var(context: &mut HashMap<String, Type>) -> Type{
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
    return t;
    
}
