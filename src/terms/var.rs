// var.rs
use std::fmt;
use super::term_trait::TermTrait;

#[derive(Debug, Clone)]
pub struct Var(pub String);

impl fmt::Display for Var {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Var({})", self.0)
    }
}

impl TermTrait for Var {
    
}