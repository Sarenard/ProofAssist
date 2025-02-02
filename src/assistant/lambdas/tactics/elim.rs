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
                LambdaTerm::Or(box type1, box type2) => {
                    LambdaTerm::match_new(
                        LambdaTerm::Var(name),
                        LambdaTerm::goal(LambdaTerm::imp(type1, typ.clone())),
                        LambdaTerm::goal(LambdaTerm::imp(type2, typ))
                    )
                }
                LambdaTerm::Bot => {
                    LambdaTerm::exfalso(
                        LambdaTerm::Var(name),
                        typ
                    )
                }
                LambdaTerm::Eq(box LambdaTerm::FBool, box LambdaTerm::TBool)
                | LambdaTerm::Eq(box LambdaTerm::TBool, box LambdaTerm::FBool) => {
                    LambdaTerm::exfalso(
                        LambdaTerm::Var(name),
                        typ
                    )
                }
                LambdaTerm::Eq(box LambdaTerm::Zero, box LambdaTerm::Succ(_))
                | LambdaTerm::Eq(box LambdaTerm::Succ(_), box LambdaTerm::Zero) => {
                    LambdaTerm::exfalso(
                        LambdaTerm::Var(name),
                        typ
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
        | LambdaTerm::Bool
        | LambdaTerm::TBool
        | LambdaTerm::FBool
        | LambdaTerm::Naturals
        | LambdaTerm::Zero
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
        LambdaTerm::Inversion(box first, box second) => {
            LambdaTerm::inversion(
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
        LambdaTerm::Eq(box first, box second) => {
            LambdaTerm::eq(
                aux_elim(first, name.clone(), context.clone()),
                aux_elim(second, name, context)
            )
        }
        LambdaTerm::Refl(box first) => {
            LambdaTerm::refl(
                aux_elim(first, name, context)
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
        LambdaTerm::Rewrite(box first, box second, box third) => {
            LambdaTerm::rewrite(
                aux_elim(first, name.clone(), context.clone()),
                aux_elim(second, name.clone(), context.clone()),
                aux_elim(third, name, context)
            )
        }
        LambdaTerm::Bif(box first, box second, box third) => {
            LambdaTerm::bif(
                aux_elim(first, name.clone(), context.clone()),
                aux_elim(second, name.clone(), context.clone()),
                aux_elim(third, name, context)
            )
        }
        LambdaTerm::Succ(box first) => {
            LambdaTerm::succ(
                aux_elim(first, name, context)
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
