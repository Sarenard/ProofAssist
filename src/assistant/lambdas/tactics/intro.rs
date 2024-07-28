use crate::assistant::lambdas as lambdas;
use crate::assistant::lambda as lambda;

use lambda::{
    LambdaTerm,
    update_counter,
};

use lambdas::update_nbs::update_goals_nb;

fn aux_intro(root: LambdaTerm, var_name: String) -> LambdaTerm {
    match root {
        // we match for a goal of the good type
        LambdaTerm::Goal(box LambdaTerm::Pi(name, box term1, box term2), nb) 
        if nb == 1 => {
            LambdaTerm::func(name, term1, LambdaTerm::goal(term2))
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