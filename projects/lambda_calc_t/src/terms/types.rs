#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Arrow(Box<Type>, Box<Type>),
    I32,
    Bool,
    Var(u32),
}

impl Type {
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

#[derive(Debug, Clone, PartialEq)]
pub enum TypeError {
    TypeNotApplicable(Type),
    TypeMismatch(Type, Type),
    Unkown,
}

pub trait Show {
    fn show(&self) -> String;
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
