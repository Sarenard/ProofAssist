use crate::assistant::lambda as lambda;

use lambda::{
    LambdaTerm,
};

use super::{free_var::free_var, gen_name::gen_name};

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
    pub fn imp(term1: LambdaTerm, term2: LambdaTerm) -> LambdaTerm {
        let mut free_vars: Vec<String> = vec![];
        free_vars.extend(free_var(term1.clone()));
        free_vars.extend(free_var(term2.clone()));
        let name = gen_name(free_vars);
        LambdaTerm::Pi(
            name,
            Box::new(term1),
            Box::new(term2),
        )
    }
    pub fn and(term1: LambdaTerm, term2: LambdaTerm) -> LambdaTerm {
        let mut free_vars: Vec<String> = vec![];
        free_vars.extend(free_var(term1.clone()));
        free_vars.extend(free_var(term2.clone()));
        let name = gen_name(free_vars);
        LambdaTerm::Sigma(
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
    pub fn or(term1: LambdaTerm, term2: LambdaTerm) -> LambdaTerm {
        LambdaTerm::Or(
            Box::new(term1),
            Box::new(term2),
        )
    }
    pub fn left(term1: LambdaTerm, term2: LambdaTerm) -> LambdaTerm {
        LambdaTerm::Left(
            Box::new(term1),
            Box::new(term2),
        )
    }
    pub fn right(term1: LambdaTerm, term2: LambdaTerm) -> LambdaTerm {
        LambdaTerm::Right(
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
    pub fn match_new(term1: LambdaTerm, term2: LambdaTerm, term3: LambdaTerm) -> LambdaTerm {
        LambdaTerm::Match(
            Box::new(term1),
            Box::new(term2),
            Box::new(term3),
        )
    }
    pub fn exfalso(term1: LambdaTerm, term2: LambdaTerm) -> LambdaTerm {
        LambdaTerm::ExFalso(
            Box::new(term1),
            Box::new(term2),
        )
    }
    pub fn not(term1: LambdaTerm) -> LambdaTerm {
        LambdaTerm::imp(
            term1, 
            LambdaTerm::Bot
        )
    }
    pub fn eq(term1: LambdaTerm, term2: LambdaTerm) -> LambdaTerm {
        LambdaTerm::Eq(
            Box::new(term1),
            Box::new(term2),
        )
    }
    pub fn refl(term1: LambdaTerm) -> LambdaTerm {
        LambdaTerm::Refl(
            Box::new(term1),
        )
    }
    pub fn rewrite(var: LambdaTerm, goal: LambdaTerm, witness: LambdaTerm) -> LambdaTerm {
        LambdaTerm::Rewrite(
            Box::new(var),
            Box::new(goal),
            Box::new(witness),
        )
    }
}