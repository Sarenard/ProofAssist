pub mod term_trait;

#[macro_use]
mod r#macro;

pub use term_trait::TermTrait as TermTrait;

use crate::exts::universe::Universe;
use crate::exts::var::Var;
use crate::exts::pi::{Apply, Lambda, Pi};

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Term {
    Universe(Universe),
    Var(Var),
    Pi(Pi),
    Lambda(Lambda),
    Apply(Apply),
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Term::Universe(u) => write!(f, "{}", u),
            Term::Var(v) => write!(f, "{}", v),
            Term::Pi(v) => write!(f, "{}", v),
            Term::Lambda(v) => write!(f, "{}", v),
            Term::Apply(v) => write!(f, "{}", v),
        }
    }
}

impl TermTrait for Term {
    fn replace(&self, to_replace: Term, with: Term) {
        match self {
            Term::Universe(t) => t.replace(to_replace, with),
            Term::Var(t) => t.replace(to_replace, with),
            Term::Pi(t) => t.replace(to_replace, with),
            Term::Lambda(t) => t.replace(to_replace, with),
            Term::Apply(t) => t.replace(to_replace, with),
        }
    }
}