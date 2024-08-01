use std::collections::HashMap;

use crate::assistant::lambdas as lambdas;
use crate::assistant::lambda as lambda;

use lambda::LambdaTerm;

use lambdas::update_nbs::update_goals_nb;

use crate::DEBUG;

fn aux_exact(root: LambdaTerm, name: String, context: HashMap<String, LambdaTerm>) -> LambdaTerm {
    let goal_h = context.get(&name).unwrap_or(&LambdaTerm::Error).clone();
    match root {
        LambdaTerm::Goal(box goal, nb2) 
        if goal_h == goal && nb2 == 1 => {
            if DEBUG {println!("Substitution {} {:?} {:?}", name, goal_h, goal)}
            LambdaTerm::var(&name)
        }
        // we propagate
        LambdaTerm::Var(..) 
        | LambdaTerm::Goal(..)
        | LambdaTerm::Types
        | LambdaTerm::Bot
        | LambdaTerm::Bool
        | LambdaTerm::TBool
        | LambdaTerm::FBool
        | LambdaTerm::Top
        | LambdaTerm::Error => {
            root
        },
        LambdaTerm::Pi(func_name, box first, box second) => {
            LambdaTerm::pi(
                func_name.clone(),
                aux_exact(first, name.clone(), context.clone()),
                aux_exact(second, name, context)
            )
        }
        LambdaTerm::Sigma(func_name, box first, box second) => {
            LambdaTerm::sigma(
                func_name.clone(),
                aux_exact(first, name.clone(), context.clone()),
                aux_exact(second, name, context)
            )
        }
        LambdaTerm::Func(func_name, box first, box second) => {
            let mut new_context = context.clone();
            new_context.insert(func_name.clone(), first.clone());
            LambdaTerm::func(
                func_name.clone(),
                aux_exact(first, name.clone(), new_context.clone()),
                aux_exact(second, name, new_context)
            )
        }
        LambdaTerm::App(box first, box second) => {
            LambdaTerm::app(
                aux_exact(first, name.clone(), context.clone()),
                aux_exact(second, name, context)
            )
        }
        LambdaTerm::Or(box first, box second) => {
            LambdaTerm::or(
                aux_exact(first, name.clone(), context.clone()),
                aux_exact(second, name, context)
            )
        }
        LambdaTerm::Left(box first, box second) => {
            LambdaTerm::left(
                aux_exact(first, name.clone(), context.clone()),
                aux_exact(second, name, context)
            )
        }
        LambdaTerm::Right(box first, box second) => {
            LambdaTerm::right(
                aux_exact(first, name.clone(), context.clone()),
                aux_exact(second, name, context)
            )
        }
        LambdaTerm::ExFalso(box first, box second) => {
            LambdaTerm::exfalso(
                aux_exact(first, name.clone(), context.clone()),
                aux_exact(second, name, context)
            )
        }
        LambdaTerm::Proj(box first, box second) => {
            LambdaTerm::proj(
                aux_exact(first, name.clone(), context.clone()),
                aux_exact(second, name, context)
            )
        }
        LambdaTerm::Eq(box first, box second) => {
            LambdaTerm::eq(
                aux_exact(first, name.clone(), context.clone()),
                aux_exact(second, name, context)
            )
        }
        LambdaTerm::Couple(box first, box second, box third) => {
            LambdaTerm::couple(
                aux_exact(first, name.clone(), context.clone()),
                aux_exact(second, name.clone(), context.clone()),
                aux_exact(third, name, context)
            )
        }
        LambdaTerm::Refl(box first) => {
            LambdaTerm::refl(
                aux_exact(first, name, context)
            )
        }
        LambdaTerm::Match(box first, box second, box third) => {
            LambdaTerm::match_new(
                aux_exact(first, name.clone(), context.clone()),
                aux_exact(second, name.clone(), context.clone()),
                aux_exact(third, name, context)
            )
        }
        LambdaTerm::Rewrite(box first, box second, box third) => {
            LambdaTerm::rewrite(
                aux_exact(first, name.clone(), context.clone()),
                aux_exact(second, name.clone(), context.clone()),
                aux_exact(third, name, context)
            )
        }
        LambdaTerm::Bif(box first, box second, box third) => {
            LambdaTerm::bif(
                aux_exact(first, name.clone(), context.clone()),
                aux_exact(second, name.clone(), context.clone()),
                aux_exact(third, name, context)
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
