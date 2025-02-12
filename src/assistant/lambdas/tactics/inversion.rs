use std::collections::HashMap;

use crate::assistant::lambdas as lambdas;
use crate::assistant::lambda as lambda;

use lambda::LambdaTerm;

use lambdas::update_nbs::update_goals_nb;

fn aux_inversion(root: LambdaTerm, var_name: String, context: HashMap<String, LambdaTerm>) -> LambdaTerm {
    let goal_h = context.get(&var_name).unwrap_or(&LambdaTerm::Error).clone();
    match root {
        // we match for a goal of the good type
        LambdaTerm::Goal(box mygoal, nb) 
        if nb == 1 => {
            match goal_h {
                LambdaTerm::Eq(box LambdaTerm::Succ(box a), box LambdaTerm::Succ(box b)) => {
                    LambdaTerm::inversion(
                        LambdaTerm::eq(
                            LambdaTerm::succ(a.clone()),
                            LambdaTerm::succ(b.clone())
                        ),
                        LambdaTerm::goal(
                            LambdaTerm::imp(
                                LambdaTerm::eq(a, b),
                                mygoal
                            )
                        )
                    )
                }
                other => panic!("other : {}", other)
            }
        }
        // we propagate
        LambdaTerm::Var(..) 
        | LambdaTerm::Goal(..)
        | LambdaTerm::Types
        | LambdaTerm::Naturals
        | LambdaTerm::Zero
        | LambdaTerm::Bot
        | LambdaTerm::Top
        | LambdaTerm::Bool
        | LambdaTerm::TBool
        | LambdaTerm::FBool
        | LambdaTerm::Error => {
            root
        },
        LambdaTerm::Pi(name, box first, box second) => {
            LambdaTerm::pi(
                name,
                aux_inversion(first, var_name.clone(), context.clone()),
                aux_inversion(second, var_name, context.clone())
            )
        }
        LambdaTerm::Sigma(name, box first, box second) => {
            LambdaTerm::sigma(
                name,
                aux_inversion(first, var_name.clone(), context.clone()),
                aux_inversion(second, var_name, context.clone())
            )
        }
        LambdaTerm::Func(name, box first, box second) => {
            let mut new_context = context.clone();
            new_context.insert(name.clone(), first.clone());
            LambdaTerm::func(
                name,
                aux_inversion(first, var_name.clone(), new_context.clone()),
                aux_inversion(second, var_name, new_context)
            )
        }
        LambdaTerm::App(box first, box second) => {
            LambdaTerm::app(
                aux_inversion(first, var_name.clone(), context.clone()),
                aux_inversion(second, var_name, context)
            )
        }
        LambdaTerm::Or(box first, box second) => {
            LambdaTerm::or(
                aux_inversion(first, var_name.clone(), context.clone()),
                aux_inversion(second, var_name, context)
            )
        }
        LambdaTerm::Left(box first, box second) => {
            LambdaTerm::left(
                aux_inversion(first, var_name.clone(), context.clone()),
                aux_inversion(second, var_name, context)
            )
        }
        LambdaTerm::Right(box first, box second) => {
            LambdaTerm::right(
                aux_inversion(first, var_name.clone(), context.clone()),
                aux_inversion(second, var_name, context)
            )
        }
        LambdaTerm::ExFalso(box first, box second) => {
            LambdaTerm::exfalso(
                aux_inversion(first, var_name.clone(), context.clone()),
                aux_inversion(second, var_name, context)
            )
        }
        LambdaTerm::Proj(box first, box second) => {
            LambdaTerm::proj(
                aux_inversion(first, var_name.clone(), context.clone()),
                aux_inversion(second, var_name, context)
            )
        }
        LambdaTerm::Eq(box first, box second) => {
            LambdaTerm::eq(
                aux_inversion(first, var_name.clone(), context.clone()),
                aux_inversion(second, var_name, context)
            )
        }
        LambdaTerm::Refl(box first) => {
            LambdaTerm::refl(
                aux_inversion(first, var_name, context)
            )
        }
        LambdaTerm::Couple(box first, box second, box third) => {
            LambdaTerm::couple(
                aux_inversion(first, var_name.clone(), context.clone()),
                aux_inversion(second, var_name.clone(), context.clone()),
                aux_inversion(third, var_name, context.clone())
            )
        }
        LambdaTerm::Match(box first, box second, box third) => {
            LambdaTerm::match_new(
                aux_inversion(first, var_name.clone(), context.clone()),
                aux_inversion(second, var_name.clone(), context.clone()),
                aux_inversion(third, var_name, context.clone())
            )
        }
        LambdaTerm::Rewrite(box first, box second, box third) => {
            LambdaTerm::rewrite(
                aux_inversion(first, var_name.clone(), context.clone()),
                aux_inversion(second, var_name.clone(), context.clone()),
                aux_inversion(third, var_name, context.clone())
            )
        }
        LambdaTerm::Bif(box first, box second, box third) => {
            LambdaTerm::bif(
                aux_inversion(first, var_name.clone(), context.clone()),
                aux_inversion(second, var_name.clone(), context.clone()),
                aux_inversion(third, var_name, context.clone())
            )
        }
        LambdaTerm::Succ(box first) => {
            LambdaTerm::succ(
                aux_inversion(first, var_name, context)
            )
        }
        LambdaTerm::Inversion(box first, box second) => {
            LambdaTerm::inversion(
                aux_inversion(first, var_name.clone(), context.clone()),
                aux_inversion(second, var_name, context)
            )
        }
    }
}

impl LambdaTerm {
    pub fn inversion_run(mut self, name: String) -> LambdaTerm {
        self = update_goals_nb(self.clone(), &mut 1);
        aux_inversion(self.clone(), name, HashMap::new())
    }
}