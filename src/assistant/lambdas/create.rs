use crate::assistant::lambda as lambda;

use lambda::{
    LambdaTerm,
    update_counter,
};

impl LambdaTerm {
    pub fn var(name: &str) -> LambdaTerm {
        let nb = update_counter(name);
        LambdaTerm::Var(
            name.to_string(),
            nb
        )
    }
    pub fn varnb(name: &str, nb: usize) -> LambdaTerm {
        LambdaTerm::Var(
            name.to_string(),
            nb
        )
    }
    pub fn goal(lambda: LambdaTerm, nb: usize) -> LambdaTerm {
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