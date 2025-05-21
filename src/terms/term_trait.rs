use std::fmt;

use super::Term;

pub trait TermTrait: fmt::Debug + fmt::Display {
    fn replace(&self, to_replace: Term, with: Term);
}
