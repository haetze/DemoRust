use terms::types::Type;

#[derive(Debug, Clone, PartialEq)]
pub enum TypeError {
    TypeNotApplicable(Type),
    TypeMismatch(Type, Type),
    Unkown,
}
