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
    pub fn sigma(name: String, term1: LambdaTerm, term2: LambdaTerm) -> LambdaTerm {
        LambdaTerm::Sigma(
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
    pub fn app(term1: LambdaTerm, term2: LambdaTerm) -> LambdaTerm {
        LambdaTerm::App(
            Box::new(term1),
            Box::new(term2),
        )
    }
    pub fn proj(term1: LambdaTerm, term2: LambdaTerm) -> LambdaTerm {
        LambdaTerm::Proj(
            Box::new(term1),
            Box::new(term2),
        )
    }
    pub fn types() -> LambdaTerm {
        LambdaTerm::Types
    }
    pub fn couple(term1: LambdaTerm, term2: LambdaTerm, term3: LambdaTerm) -> LambdaTerm {
        LambdaTerm::Couple(
            Box::new(term1),
            Box::new(term2),
            Box::new(term3),
        )
    }
}