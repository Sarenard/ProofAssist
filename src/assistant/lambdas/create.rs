use crate::assistant::lambda as lambda;

use lambda::{
    LambdaTerm,
    update_counter,
};

impl LambdaTerm {
    pub fn var(name: &str) -> LambdaTerm {
        LambdaTerm::Var(
            name.to_string(),
        )
    }
    pub fn goal(lambda: LambdaTerm) -> LambdaTerm {
        LambdaTerm::Goal(
            Box::new(lambda),
            0
        )
    }
    pub fn goalnb(lambda: LambdaTerm, nb: usize) -> LambdaTerm {
        LambdaTerm::Goal(
            Box::new(lambda),
            nb
        )
    }
    pub fn pi(name: String, term1: LambdaTerm, term2: LambdaTerm) -> LambdaTerm {
        LambdaTerm::Pi(
            name,
            Box::new(term1),
            Box::new(term2),
        )
    }
    pub fn func(name: String, term1: LambdaTerm, term2: LambdaTerm) -> LambdaTerm {
        LambdaTerm::Func(
            name,
            Box::new(term1),
            Box::new(term2),
        )
    }
}