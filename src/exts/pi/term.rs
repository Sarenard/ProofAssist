use std::fmt;
use crate::terms::{term_trait::TermTrait, Term};

#[derive(Debug, Clone, PartialEq)]
// \forall x:A B
pub struct Pi(pub Box<Term>, pub Box<Term>, pub Box<Term>);

impl fmt::Display for Pi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\\forall ({}: {}). ({})", *self.0, *self.1, *self.2)
    }
}

impl TermTrait for Pi {
    
}

#[derive(Debug, Clone, PartialEq)]
// \lambda (x:A) b
pub struct Lambda(pub Box<Term>, pub Box<Term>, pub Box<Term>);

impl fmt::Display for Lambda {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\\lambda ({}: {}). ({})", *self.0, *self.1, *self.2)
    }
}

impl TermTrait for Lambda {
    
}

// func apply term
#[derive(Debug, Clone, PartialEq)]
// f(a)
pub struct Apply(pub Box<Term>, pub Box<Term>);

impl fmt::Display for Apply {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})({})", *self.0, *self.1)
    }
}

impl TermTrait for Apply {
    
}