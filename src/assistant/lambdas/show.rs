use crate::assistant::lambda as lambda;

use lambda::LambdaTerm;

use std::fmt;

impl std::fmt::Display for LambdaTerm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            _ => unimplemented!()
        }
    }
}