use std::collections::HashMap;

use crate::assistant::lambdas::compute_type::compute_type;
use crate::assistant::lambdas::replace::replace;
use crate::assistant::lambdas as lambdas;
use crate::assistant::lambda as lambda;

use lambda::LambdaTerm;

use lambdas::{
    update_nbs::update_goals_nb,
};

fn aux_rewrite(root: LambdaTerm, name: String, context: HashMap<String, LambdaTerm>) -> LambdaTerm {
    let goal_h = context.get(&name).unwrap_or(&LambdaTerm::Error).clone();
    match root.clone() {
        LambdaTerm::Goal(box type_goal, nb2) 
        if nb2 == 1 => {
            match goal_h.clone() {
                LambdaTerm::Eq(box first, box second) => {
                    LambdaTerm::rewrite(
                        LambdaTerm::Var(name),
                        LambdaTerm::goal(replace(type_goal.clone(), first, second)),
                        type_goal
                    )
                }
                other => panic!("other : {}", other)
            }
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
                aux_rewrite(first, name.clone(), context.clone()),
                aux_rewrite(second, name, context)
            )
        }
        LambdaTerm::Sigma(func_name, box first, box second) => {
            LambdaTerm::sigma(
                func_name.clone(),
                aux_rewrite(first, name.clone(), context.clone()),
                aux_rewrite(second, name, context)
            )
        }
        LambdaTerm::Func(func_name, box first, box second) => {
            let mut new_context = context.clone();
            new_context.insert(func_name.clone(), first.clone());
            LambdaTerm::func(
                func_name.clone(),
                aux_rewrite(first, name.clone(), new_context.clone()),
                aux_rewrite(second, name, new_context)
            )
        }
        LambdaTerm::App(box first, box second) => {
            LambdaTerm::app(
                aux_rewrite(first, name.clone(), context.clone()),
                aux_rewrite(second, name, context)
            )
        }
        LambdaTerm::Or(box first, box second) => {
            LambdaTerm::or(
                aux_rewrite(first, name.clone(), context.clone()),
                aux_rewrite(second, name, context)
            )
        }
        LambdaTerm::Left(box first, box second) => {
            LambdaTerm::left(
                aux_rewrite(first, name.clone(), context.clone()),
                aux_rewrite(second, name, context)
            )
        }
        LambdaTerm::Right(box first, box second) => {
            LambdaTerm::right(
                aux_rewrite(first, name.clone(), context.clone()),
                aux_rewrite(second, name, context)
            )
        }
        LambdaTerm::ExFalso(box first, box second) => {
            LambdaTerm::exfalso(
                aux_rewrite(first, name.clone(), context.clone()),
                aux_rewrite(second, name, context)
            )
        }
        LambdaTerm::Proj(box first, box second) => {
            LambdaTerm::proj(
                aux_rewrite(first, name.clone(), context.clone()),
                aux_rewrite(second, name, context)
            )
        }
        LambdaTerm::Eq(box first, box second) => {
            LambdaTerm::eq(
                aux_rewrite(first, name.clone(), context.clone()),
                aux_rewrite(second, name, context)
            )
        }
        LambdaTerm::Refl(box first) => {
            LambdaTerm::refl(
                aux_rewrite(first, name, context)
            )
        }
        LambdaTerm::Couple(box first, box second, box third) => {
            LambdaTerm::couple(
                aux_rewrite(first, name.clone(), context.clone()),
                aux_rewrite(second, name.clone(), context.clone()),
                aux_rewrite(third, name, context)
            )
        }
        LambdaTerm::Match(box first, box second, box third) => {
            LambdaTerm::match_new(
                aux_rewrite(first, name.clone(), context.clone()),
                aux_rewrite(second, name.clone(), context.clone()),
                aux_rewrite(third, name, context)
            )
        }
        LambdaTerm::Rewrite(box first, box second, box third) => {
            LambdaTerm::rewrite(
                aux_rewrite(first, name.clone(), context.clone()),
                aux_rewrite(second, name.clone(), context.clone()),
                aux_rewrite(third, name, context)
            )
        }
    }
}

impl LambdaTerm {
    pub fn rewrite_run(mut self, name: String) -> LambdaTerm {
        self = update_goals_nb(self.clone(), &mut 1);
        aux_rewrite(self.clone(), name, HashMap::new())
    }
}
