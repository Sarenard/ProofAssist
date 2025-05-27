use std::{fmt, sync::atomic::Ordering};
use crate::{term, terms::{term_trait::TermTrait, Term}, VAR_COUNTER};

#[derive(Debug, Clone)]
// \forall x:A B
pub struct Pi(pub Box<Term>, pub Box<Term>, pub Box<Term>);

impl fmt::Display for Pi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // si free (= _ for now), on a A -> B et pas un type pi
        if *self.0 == term!(Var("_")) {
            write!(f, "({} -> {})", *self.1, *self.2)
        } else {
            write!(f, "\\forall ({}: {}). ({})", *self.0, *self.1, *self.2)
        }
    }
}

impl PartialEq for Pi {
    fn eq(&self, other: &Self) -> bool {
        let id = VAR_COUNTER.fetch_add(1, Ordering::Relaxed);
        let fresh = term!(Var(format!("__{}", id))); // TODO : make real new vars

        // Rename bound vars to fresh in both bodies
        let self_body = self.2.clone().replace(*self.0.clone(), fresh.clone());
        let other_body = other.2.clone().replace(*other.0.clone(), fresh.clone());

        self.1 == other.1 && self_body == other_body
    }
}

impl TermTrait for Pi {
    fn replace(self, to_replace: Term, with: Term) -> Term {
        if to_replace == *self.0 {
            term!(Pi(
                with.clone(),
                self.1.replace(to_replace.clone(), with.clone()),
                self.2.replace(to_replace.clone(), with.clone())
            ))
        } else {
            term!(Pi(
                *self.0,
                self.1.replace(to_replace.clone(), with.clone()),
                self.2.replace(to_replace.clone(), with.clone())
            ))
        }
    }
}

#[derive(Debug, Clone)]
// \lambda (x:A) b
pub struct Lambda(pub Box<Term>, pub Box<Term>, pub Box<Term>);

impl fmt::Display for Lambda {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\\lambda ({}: {}). ({})", *self.0, *self.1, *self.2)
    }
}


impl PartialEq for Lambda {
    fn eq(&self, other: &Self) -> bool {
        let id = VAR_COUNTER.fetch_add(1, Ordering::Relaxed);
        let fresh = term!(Var(format!("__{}", id))); // TODO : make real new vars

        // Rename bound vars to fresh in both bodies
        let self_body = self.2.clone().replace(*self.0.clone(), fresh.clone());
        let other_body = other.2.clone().replace(*other.0.clone(), fresh.clone());

        self.1 == other.1 && self_body == other_body
    }
}

impl TermTrait for Lambda {
    fn replace(self, to_replace: Term, with: Term) -> Term {
        // TODO : DO THAT BUT GOOD
        if to_replace == *self.0 {
            term!(Lambda(
                *self.0,
                *self.1,
                *self.2
            ))
        } else {
            term!(Lambda(
                *self.0,
                self.1.replace(to_replace.clone(), with.clone()),
                self.2.replace(to_replace.clone(), with.clone())
            ))
        }
    }
}

// func apply term
#[derive(Debug, Clone, PartialEq)]
// f(a)
pub struct Apply(pub Box<Term>, pub Box<Term>);

impl fmt::Display for Apply {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "App({})({})", *self.0, *self.1)
    }
}

impl TermTrait for Apply {
    fn replace(self, to_replace: Term, with: Term) -> Term {
        // TODO : CHECK THIS IS RIGHT
        term!(Apply(
            self.0.replace(to_replace.clone(), with.clone()),
            self.1.replace(to_replace.clone(), with.clone())
        ))
    }
}