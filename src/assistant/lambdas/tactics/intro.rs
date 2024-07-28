use crate::assistant::lambdas as lambdas;
use crate::assistant::lambda as lambda;

use lambda::{
    LambdaTerm,
    update_counter,
};

use lambdas::{
    update_nbs::update_goals_nb,
    substitute::substitute,
};

fn aux_intro(root: LambdaTerm, var_name: String) -> LambdaTerm {
    match root {
        // we match for a goal of the good type
        LambdaTerm::Goal(box LambdaTerm::Pi(name, box term1, box term2), nb) 
        if nb == 1 => {
            let total_x = update_counter("x");
            let new_name = format!("x{}", total_x);
            let term = LambdaTerm::Var(new_name);
            LambdaTerm::func(
                name.clone(),
                term1,
                LambdaTerm::goal(substitute(term2, name, term)),
            )
        }
        // we propagate
        LambdaTerm::Var(..) 
        | LambdaTerm::Goal(..)
        | LambdaTerm::Error => {
            root
        },
        LambdaTerm::Pi(name, box first, box second) => {
            LambdaTerm::pi(
                name,
                aux_intro(first, var_name.clone()),
                aux_intro(second, var_name)
            )
        }
        LambdaTerm::Func(name, box first, box second) => {
            LambdaTerm::func(
                name,
                aux_intro(first, var_name.clone()),
                aux_intro(second, var_name)
            )
        }
        LambdaTerm::App(box first, box second) => {
            LambdaTerm::app(
                aux_intro(first, var_name.clone()),
                aux_intro(second, var_name)
            )
        }
    }
}

impl LambdaTerm {
    pub fn intro(mut self) -> (String, LambdaTerm) {
        self = update_goals_nb(self.clone(), &mut 1);
        // we find a non used name
        let nb = update_counter("hyp");
        let concatenated_name = format!("hyp{}", nb);
        (concatenated_name.clone(), aux_intro(self.clone(), concatenated_name))
    }
}