use terms::types::Type;

pub trait Typable {
    fn get_type(&self) -> &Type;
}
