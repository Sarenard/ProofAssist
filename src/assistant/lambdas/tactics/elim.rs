use std::collections::HashMap;

use crate::assistant::lambdas as lambdas;
use crate::assistant::lambda as lambda;

use lambda::LambdaTerm;

use lambdas::update_nbs::update_goals_nb;

use crate::DEBUG;

fn aux_elim(root: LambdaTerm, name: String, context: HashMap<String, LambdaTerm>) -> LambdaTerm {
    let type_to_elim = context.get(&name).unwrap_or(&LambdaTerm::Error).clone();
    match root.clone() {
        LambdaTerm::Goal(box typ, nb2) 
        if nb2 == 1 => {
            assert!(type_to_elim != LambdaTerm::Error);
            if DEBUG {println!("try elim {:?}", typ);}
            match type_to_elim {
                LambdaTerm::Sigma(sigmaname, box type1, box body) => {
                    LambdaTerm::proj(
                        LambdaTerm::Var(name),
                        LambdaTerm::goal(
                            LambdaTerm::pi(
                                sigmaname,
                                type1,
                                LambdaTerm::imp(
                                    body,
                                    typ
                                )
                            )
                        )
                    )
                }
                other => panic!("Cant elim for {:?}", other)
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
                aux_elim(first, name.clone(), context.clone()),
                aux_elim(second, name, context)
            )
        }
        LambdaTerm::Sigma(func_name, box first, box second) => {
            LambdaTerm::sigma(
                func_name.clone(),
                aux_elim(first, name.clone(), context.clone()),
                aux_elim(second, name, context)
            )
        }
        LambdaTerm::Func(func_name, box first, box second) => {
            let mut new_context = context.clone();
            new_context.insert(func_name.clone(), first.clone());
            LambdaTerm::func(
                func_name.clone(),
                aux_elim(first, name.clone(), new_context.clone()),
                aux_elim(second, name, new_context)
            )
        }
        LambdaTerm::App(box first, box second) => {
            LambdaTerm::app(
                aux_elim(first, name.clone(), context.clone()),
                aux_elim(second, name, context)
            )
        }
        LambdaTerm::Left(box first, box second) => {
            LambdaTerm::left(
                aux_elim(first, name.clone(), context.clone()),
                aux_elim(second, name, context)
            )
        }
        LambdaTerm::Right(box first, box second) => {
            LambdaTerm::right(
                aux_elim(first, name.clone(), context.clone()),
                aux_elim(second, name, context)
            )
        }
        LambdaTerm::Or(box first, box second) => {
            LambdaTerm::or(
                aux_elim(first, name.clone(), context.clone()),
                aux_elim(second, name, context)
            )
        }
        LambdaTerm::ExFalso(box first, box second) => {
            LambdaTerm::exfalso(
                aux_elim(first, name.clone(), context.clone()),
                aux_elim(second, name, context)
            )
        }
        LambdaTerm::Proj(box first, box second) => {
            LambdaTerm::proj(
                aux_elim(first, name.clone(), context.clone()),
                aux_elim(second, name, context)
            )
        }
        LambdaTerm::Couple(box first, box second, box third) => {
            LambdaTerm::couple(
                aux_elim(first, name.clone(), context.clone()),
                aux_elim(second, name.clone(), context.clone()),
                aux_elim(third, name, context)
            )
        }
        LambdaTerm::Match(box first, box second, box third) => {
            LambdaTerm::match_new(
                aux_elim(first, name.clone(), context.clone()),
                aux_elim(second, name.clone(), context.clone()),
                aux_elim(third, name, context)
            )
        }
    }
}

impl LambdaTerm {
    pub fn elim(mut self, name: String) -> LambdaTerm {
        self = update_goals_nb(self.clone(), &mut 1);
        aux_elim(self.clone(), name, HashMap::new())
    }
}
