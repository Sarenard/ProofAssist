use std::fmt;
use crate::terms::{term_trait::TermTrait, Term};

#[derive(Debug, Clone)]
pub struct Universe(pub usize);

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "U({})", self.0)
    }
}

impl TermTrait for Universe {
    fn replace(self, _to_replace: Term, _with: Term) -> Term {
        todo!()
    }
}