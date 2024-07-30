use std::collections::HashMap;

use crate::assistant::lambdas as lambdas;
use crate::assistant::lambda as lambda;

use lambda::LambdaTerm;

use lambdas::{
    update_nbs::update_goals_nb,
    alpha_equiv::replace_free_variable
};

use crate::DEBUG;

fn aux_exists(root: LambdaTerm, name: String, context: HashMap<String, LambdaTerm>) -> LambdaTerm {
    let goal_h = context.get(&name).unwrap_or(&LambdaTerm::Error).clone();
    match root.clone() {
        LambdaTerm::Goal(box LambdaTerm::Sigma(sigma_name, box first, box second), nb2) 
        if nb2 == 1 => {
            if DEBUG {println!("Exists {:?}", root)}
            LambdaTerm::couple(
                goal_h.clone(),
                LambdaTerm::goal(
                    replace_free_variable(sigma_name, goal_h, second.clone())
                ), 
                LambdaTerm::sigma(name, first, second)
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
        LambdaTerm::Pi(func_name, box first, box second) => {
            LambdaTerm::pi(
                func_name.clone(),
                aux_exists(first, name.clone(), context.clone()),
                aux_exists(second, name, context)
            )
        }
        LambdaTerm::Sigma(func_name, box first, box second) => {
            LambdaTerm::sigma(
                func_name.clone(),
                aux_exists(first, name.clone(), context.clone()),
                aux_exists(second, name, context)
            )
        }
        LambdaTerm::Func(func_name, box first, box second) => {
            let mut new_context = context.clone();
            new_context.insert(func_name.clone(), first.clone());
            LambdaTerm::func(
                func_name.clone(),
                aux_exists(first, name.clone(), new_context.clone()),
                aux_exists(second, name, new_context)
            )
        }
        LambdaTerm::App(box first, box second) => {
            LambdaTerm::app(
                aux_exists(first, name.clone(), context.clone()),
                aux_exists(second, name, context)
            )
        }
        LambdaTerm::ExFalso(box first, box second) => {
            LambdaTerm::exfalso(
                aux_exists(first, name.clone(), context.clone()),
                aux_exists(second, name, context)
            )
        }
        LambdaTerm::Proj(box first, box second) => {
            LambdaTerm::proj(
                aux_exists(first, name.clone(), context.clone()),
                aux_exists(second, name, context)
            )
        }
        LambdaTerm::Couple(box first, box second, box third) => {
            LambdaTerm::couple(
                aux_exists(first, name.clone(), context.clone()),
                aux_exists(second, name.clone(), context.clone()),
                aux_exists(third, name, context)
            )
        }
    }
}

impl LambdaTerm {
    pub fn exists(mut self, name: String) -> LambdaTerm {
        self = update_goals_nb(self.clone(), &mut 1);
        aux_exists(self.clone(), name, HashMap::new())
    }
}
