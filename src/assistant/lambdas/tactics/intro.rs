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
            let term = LambdaTerm::Var(var_name.clone());
            LambdaTerm::func(
                var_name.clone(),
                term1,
                LambdaTerm::goal(substitute(term2, name, term)),
            )
        }
        // we propagate
        LambdaTerm::Var(..) 
        | LambdaTerm::Goal(..)
        | LambdaTerm::Types
        | LambdaTerm::Bot
        | LambdaTerm::Top
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
        LambdaTerm::Sigma(name, box first, box second) => {
            LambdaTerm::sigma(
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
        LambdaTerm::Or(box first, box second) => {
            LambdaTerm::or(
                aux_intro(first, var_name.clone()),
                aux_intro(second, var_name)
            )
        }
        LambdaTerm::Left(box first, box second) => {
            LambdaTerm::left(
                aux_intro(first, var_name.clone()),
                aux_intro(second, var_name)
            )
        }
        LambdaTerm::Right(box first, box second) => {
            LambdaTerm::right(
                aux_intro(first, var_name.clone()),
                aux_intro(second, var_name)
            )
        }
        LambdaTerm::ExFalso(box first, box second) => {
            LambdaTerm::exfalso(
                aux_intro(first, var_name.clone()),
                aux_intro(second, var_name)
            )
        }
        LambdaTerm::Proj(box first, box second) => {
            LambdaTerm::proj(
                aux_intro(first, var_name.clone()),
                aux_intro(second, var_name)
            )
        }
        LambdaTerm::Eq(box first, box second) => {
            LambdaTerm::eq(
                aux_intro(first, var_name.clone()),
                aux_intro(second, var_name)
            )
        }
        LambdaTerm::Refl(box first) => {
            LambdaTerm::refl(
                aux_intro(first, var_name)
            )
        }
        LambdaTerm::Couple(box first, box second, box third) => {
            LambdaTerm::couple(
                aux_intro(first, var_name.clone()),
                aux_intro(second, var_name.clone()),
                aux_intro(third, var_name)
            )
        }
        LambdaTerm::Match(box first, box second, box third) => {
            LambdaTerm::match_new(
                aux_intro(first, var_name.clone()),
                aux_intro(second, var_name.clone()),
                aux_intro(third, var_name)
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
    pub fn intros(mut self) -> (Vec<String>, LambdaTerm) {
        self = update_goals_nb(self.clone(), &mut 1);
        // we find a non used name
        let mut names: Vec<String> = vec![];
        let mut old_lambdaterm = self.clone();
        let (name, mut new_lambdaterm) = self.intro();
        names.push(name);
        while old_lambdaterm != new_lambdaterm.clone() {
            let name: String; 
            old_lambdaterm = new_lambdaterm.clone();
            (name, new_lambdaterm) = new_lambdaterm.intro();
            names.push(name);
        }
        (names.clone(), new_lambdaterm)
    }
}