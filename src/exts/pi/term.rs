use std::fmt;
use crate::terms::{term_trait::TermTrait, Term};

// TODO : add lambda functions

#[derive(Debug, Clone, PartialEq)]
// \forall x:A B
pub struct Pi(pub Box<Term>, pub Box<Term>, pub Box<Term>);

impl fmt::Display for Pi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\\forall {}: {}. {}", *self.0, *self.1, *self.2)
    }
}

impl TermTrait for Pi {
    
}