use std::collections::HashMap;

use crate::assistant::lambdas as lambdas;
use crate::assistant::lambda as lambda;

use lambda::{
    LambdaTerm,
    update_counter,
};

use lambdas::update_nbs::update_goals_nb;

fn aux_exact(root: LambdaTerm, name: String, context: HashMap<String, LambdaTerm>) -> LambdaTerm {
    let goal_h = context.get(&name).unwrap_or(&LambdaTerm::Error).clone();
    match root {
        LambdaTerm::Goal(box goal, nb2) 
        if goal_h == goal && nb2 == 1 => {
            LambdaTerm::var(&name)
        }
        // we propagate
        LambdaTerm::Var(..) 
        | LambdaTerm::Goal(..)
        | LambdaTerm::Error => {
            root
        },
        LambdaTerm::Pi(name, box first, box second) => {
            LambdaTerm::pi(
                name.clone(),
                aux_exact(first, name.clone(), context.clone()),
                aux_exact(second, name, context)
            )
        }
        LambdaTerm::Func(name, box first, box second) => {
            let mut new_context = context.clone();
            new_context.insert(name.clone(), first.clone());
            LambdaTerm::func(
                name.clone(),
                aux_exact(first, name.clone(), new_context.clone()),
                aux_exact(second, name, new_context)
            )
        }
    }
}

impl LambdaTerm {
    pub fn exact(mut self, name: String) -> LambdaTerm {
        self = update_goals_nb(self.clone(), &mut 1);
        aux_exact(self.clone(), name, HashMap::new())
    }
}
