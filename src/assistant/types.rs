#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Var(String),
    Impl(Box<Type>, Box<Type>),
    And(Box<Type>, Box<Type>),
}