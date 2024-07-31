use std::collections::HashMap;

use crate::assistant::lambdas::compute_type::compute_type;
use crate::assistant::lambdas as lambdas;
use crate::assistant::lambda as lambda;

use lambda::LambdaTerm;

use lambdas::{
    update_nbs::update_goals_nb,
    alpha_equiv::replace_free_variable
};

use crate::DEBUG;

fn aux_exists(root: LambdaTerm, obj: LambdaTerm, context: HashMap<String, LambdaTerm>) -> LambdaTerm {
    match root.clone() {
        LambdaTerm::Goal(box LambdaTerm::Sigma(sigma_name, box first, box second), nb2) 
        if nb2 == 1 && compute_type(obj.clone(), context.clone()) == first => {
            if DEBUG {println!("Exists {:?} {:?}", root, obj.clone())}
            LambdaTerm::couple(
                obj.clone(),
                LambdaTerm::goal(replace_free_variable(sigma_name.clone(), obj.clone(), second.clone())),
                LambdaTerm::sigma(sigma_name, first, second)
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
                aux_exists(first, obj.clone(), context.clone()),
                aux_exists(second, obj, context)
            )
        }
        LambdaTerm::Sigma(func_name, box first, box second) => {
            LambdaTerm::sigma(
                func_name.clone(),
                aux_exists(first, obj.clone(), context.clone()),
                aux_exists(second, obj, context)
            )
        }
        LambdaTerm::Func(func_name, box first, box second) => {
            let mut new_context = context.clone();
            new_context.insert(func_name.clone(), first.clone());
            LambdaTerm::func(
                func_name.clone(),
                aux_exists(first, obj.clone(), new_context.clone()),
                aux_exists(second, obj, new_context)
            )
        }
        LambdaTerm::App(box first, box second) => {
            LambdaTerm::app(
                aux_exists(first, obj.clone(), context.clone()),
                aux_exists(second, obj, context)
            )
        }
        LambdaTerm::Or(box first, box second) => {
            LambdaTerm::or(
                aux_exists(first, obj.clone(), context.clone()),
                aux_exists(second, obj, context)
            )
        }
        LambdaTerm::Left(box first, box second) => {
            LambdaTerm::left(
                aux_exists(first, obj.clone(), context.clone()),
                aux_exists(second, obj, context)
            )
        }
        LambdaTerm::Right(box first, box second) => {
            LambdaTerm::right(
                aux_exists(first, obj.clone(), context.clone()),
                aux_exists(second, obj, context)
            )
        }
        LambdaTerm::ExFalso(box first, box second) => {
            LambdaTerm::exfalso(
                aux_exists(first, obj.clone(), context.clone()),
                aux_exists(second, obj, context)
            )
        }
        LambdaTerm::Proj(box first, box second) => {
            LambdaTerm::proj(
                aux_exists(first, obj.clone(), context.clone()),
                aux_exists(second, obj, context)
            )
        }
        LambdaTerm::Couple(box first, box second, box third) => {
            LambdaTerm::couple(
                aux_exists(first, obj.clone(), context.clone()),
                aux_exists(second, obj.clone(), context.clone()),
                aux_exists(third, obj, context)
            )
        }
        LambdaTerm::Match(box first, box second, box third) => {
            LambdaTerm::match_new(
                aux_exists(first, obj.clone(), context.clone()),
                aux_exists(second, obj.clone(), context.clone()),
                aux_exists(third, obj, context)
            )
        }
    }
}

impl LambdaTerm {
    pub fn exists(mut self, obj: LambdaTerm) -> LambdaTerm {
        self = update_goals_nb(self.clone(), &mut 1);
        aux_exists(self.clone(), obj, HashMap::new())
    }
}
