pub mod term_trait;

#[macro_use]
mod r#macro;

pub use term_trait::TermTrait as TermTrait;

use crate::exts::nat::{IndN, NSucc, NZero, Nat};
use crate::exts::universe::Universe;
use crate::exts::var::Var;
use crate::exts::pi::{Apply, Lambda, Pi};
use crate::exts::zero::{Ind0, Zero};

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
#[allow(unused)]
pub enum Term {
    Universe(Universe),
    Var(Var),
    Pi(Pi),
    Lambda(Lambda),
    Apply(Apply),
    Zero(Zero),
    Ind0(Ind0),
    NZero(NZero),
    NSucc(NSucc),
    IndN(IndN),
    Nat(Nat),
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Term::Universe(u) => write!(f, "{}", u),
            Term::Var(v) => write!(f, "{}", v),
            Term::Pi(v) => write!(f, "{}", v),
            Term::Lambda(v) => write!(f, "{}", v),
            Term::Apply(v) => write!(f, "{}", v),
            Term::Zero(v) => write!(f, "{}", v),
            Term::Ind0(v) => write!(f, "{}", v),
            Term::NZero(v) => write!(f, "{}", v),
            Term::NSucc(v) => write!(f, "{}", v),
            Term::IndN(v) => write!(f, "{}", v),
            Term::Nat(v) => write!(f, "{}", v),
        }
    }
}

impl TermTrait for Term {
    fn replace(self, to_replace: Term, with: Term) -> Term {
        match self {
            Term::Universe(t) => t.replace(to_replace, with),
            Term::Var(t) => t.replace(to_replace, with),
            Term::Pi(t) => t.replace(to_replace, with),
            Term::Lambda(t) => t.replace(to_replace, with),
            Term::Apply(t) => t.replace(to_replace, with),
            Term::Zero(t) => t.replace(to_replace, with),
            Term::Ind0(t) => t.replace(to_replace, with),
            Term::NZero(t) => t.replace(to_replace, with),
            Term::NSucc(t) => t.replace(to_replace, with),
            Term::IndN(t) => t.replace(to_replace, with),
            Term::Nat(t) => t.replace(to_replace, with),
        }
    }
}