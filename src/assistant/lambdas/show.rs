use crate::assistant::lambda as lambda;

use lambda::LambdaTerm;

use std::fmt;

impl std::fmt::Display for LambdaTerm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LambdaTerm::Var(name) => {
                write!(f, "{}", name)
            }
            LambdaTerm::Goal(box typ, _nb) => {
                write!(f, "Goal({})", typ)
            }
            LambdaTerm::Pi(_name, box first, box second) => {
                // todo : forall
                write!(f, "({} -> {})", first, second)
            }
            LambdaTerm::App(box first, box second) => {
                write!(f, "{}({})", first, second)
            }
            other => unimplemented!("{}", other)
        }
    }
}