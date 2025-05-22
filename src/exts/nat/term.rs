use std::fmt;
use crate::{term, terms::{term_trait::TermTrait, Term}};

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct NZero;

impl fmt::Display for NZero {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0_N")
    }
}

impl TermTrait for NZero {
    fn replace(self, _to_replace: Term, _with: Term) -> Term {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Nat;

impl fmt::Display for Nat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Nat")
    }
}

impl TermTrait for Nat {
    fn replace(self, _to_replace: Term, _with: Term) -> Term {
        term!(Nat)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NSucc(pub Box<Term>);

impl fmt::Display for NSucc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NSucc({})", self.0)
    }
}

impl TermTrait for NSucc {
    fn replace(self, to_replace: Term, with: Term) -> Term {
        term!(NSucc(self.0.replace(to_replace, with)))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct IndN(pub Box<Term>, pub Box<Term>, pub Box<Term>, pub Box<Term>);

impl fmt::Display for IndN {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NRec({}, {}, {}, {})", *self.0, *self.1, *self.2, *self.3)
    }
}

impl TermTrait for IndN {
    fn replace(self, _to_replace: Term, _with: Term) -> Term {
        todo!()
    }
}