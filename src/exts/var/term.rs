use std::fmt;
use crate::terms::{Term, TermTrait};

#[derive(Debug, Clone, PartialEq)]
pub struct Var(pub String);

impl fmt::Display for Var {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TermTrait for Var {
    fn replace(self, to_replace: Term, with: Term) -> Term {
        if Term::Var(self.clone()) == to_replace {
            with
        } else {
            Term::Var(self)
        }
    }
}
