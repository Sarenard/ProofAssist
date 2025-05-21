use crate::exts::universe::Universe;
use crate::exts::var::Var;

use super::term_trait::TermTrait;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Term {
    Universe(Universe),
    Var(Var),
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Term::Universe(u) => write!(f, "{}", u),
            Term::Var(v) => write!(f, "{}", v),
        }
    }
}

// is this nessessary
impl TermTrait for Term {
    
}
