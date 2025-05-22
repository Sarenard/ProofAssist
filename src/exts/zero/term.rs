use std::fmt;
use crate::terms::{term_trait::TermTrait, Term};

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Zero;

impl fmt::Display for Zero {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0")
    }
}

impl TermTrait for Zero {
    fn replace(self, _to_replace: Term, _with: Term) -> Term {
        todo!()
    }
}