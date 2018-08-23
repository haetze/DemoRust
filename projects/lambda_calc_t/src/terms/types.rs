#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Arrow(Box<Type>, Box<Type>),
    I32,
    Bool,
    Var(u32),
}
