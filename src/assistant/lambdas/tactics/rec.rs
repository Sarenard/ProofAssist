use crate::assistant::lambda::update_counter;
use crate::assistant::lambdas as lambdas;
use crate::assistant::lambda as lambda;

use lambda::LambdaTerm;

use lambdas::update_nbs::update_goals_nb;

use crate::assistant::lambdas::replace::replace;

use crate::DEBUG;

fn aux_rec(root: LambdaTerm) -> LambdaTerm {
    match root {
        // we match for a goal of the good type
        LambdaTerm::Goal(box LambdaTerm::Pi(name, box LambdaTerm::Naturals, box prop), nb) 
        if nb == 1 => {
            if DEBUG {println!("REC {} {}", name, prop);}
            // we need to be able to talk about prop(n) prop(s(n)) and prop(0)

            // we find a non used name
            let nb = update_counter("hyp");
            let concatenated_name = format!("hyp{}", nb);

            LambdaTerm::rec(
                LambdaTerm::goal(
                    replace(prop.clone(), LambdaTerm::Var(name.clone()), LambdaTerm::Zero)
                ),
                LambdaTerm::goal(LambdaTerm::imp(
                    replace(
                        prop.clone(), 
                        LambdaTerm::Var(name.clone()), 
                        LambdaTerm::Var(concatenated_name.clone())
                    ),
                    replace(
                        prop.clone(), 
                        LambdaTerm::Var(name.clone()), 
                        LambdaTerm::succ(LambdaTerm::Var(concatenated_name))
                    )
                )),
                LambdaTerm::pi(name, LambdaTerm::Naturals, prop)
            )
        }
        // we propagate
        LambdaTerm::Var(..) 
        | LambdaTerm::Goal(..)
        | LambdaTerm::Types
        | LambdaTerm::Bool
        | LambdaTerm::TBool
        | LambdaTerm::Naturals
        | LambdaTerm::Zero
        | LambdaTerm::FBool
        | LambdaTerm::Bot
        | LambdaTerm::Top
        | LambdaTerm::Error => {
            root
        },
        LambdaTerm::Pi(name, box first, box second) => {
            LambdaTerm::pi(
                name,
                aux_rec(first),
                aux_rec(second)
            )
        }
        LambdaTerm::Sigma(name, box first, box second) => {
            LambdaTerm::sigma(
                name,
                aux_rec(first),
                aux_rec(second)
            )
        }
        LambdaTerm::Func(name, box first, box second) => {
            LambdaTerm::func(
                name,
                aux_rec(first),
                aux_rec(second)
            )
        }
        LambdaTerm::App(box first, box second) => {
            LambdaTerm::app(
                aux_rec(first),
                aux_rec(second)
            )
        }
        LambdaTerm::Or(box first, box second) => {
            LambdaTerm::or(
                aux_rec(first),
                aux_rec(second)
            )
        }
        LambdaTerm::Left(box first, box second) => {
            LambdaTerm::left(
                aux_rec(first),
                aux_rec(second)
            )
        }
        LambdaTerm::Right(box first, box second) => {
            LambdaTerm::right(
                aux_rec(first),
                aux_rec(second)
            )
        }
        LambdaTerm::ExFalso(box first, box second) => {
            LambdaTerm::exfalso(
                aux_rec(first),
                aux_rec(second)
            )
        }
        LambdaTerm::Proj(box first, box second) => {
            LambdaTerm::proj(
                aux_rec(first),
                aux_rec(second)
            )
        }
        LambdaTerm::Eq(box first, box second) => {
            LambdaTerm::eq(
                aux_rec(first),
                aux_rec(second)
            )
        }
        LambdaTerm::Refl(box first) => {
            LambdaTerm::refl(
                aux_rec(first)
            )
        }
        LambdaTerm::Couple(box first, box second, box third) => {
            LambdaTerm::couple(
                aux_rec(first),
                aux_rec(second),
                aux_rec(third)
            )
        }
        LambdaTerm::Match(box first, box second, box third) => {
            LambdaTerm::match_new(
                aux_rec(first),
                aux_rec(second),
                aux_rec(third)
            )
        }
        LambdaTerm::Rewrite(box first, box second, box third) => {
            LambdaTerm::rewrite(
                aux_rec(first),
                aux_rec(second),
                aux_rec(third)
            )
        }
        LambdaTerm::Bif(box first, box second, box third) => {
            LambdaTerm::bif(
                aux_rec(first),
                aux_rec(second),
                aux_rec(third)
            )
        }
        LambdaTerm::Succ(box first) => {
            LambdaTerm::succ(
                aux_rec(first)
            )
        }
        LambdaTerm::Inversion(box first, box second) => {
            LambdaTerm::inversion(
                aux_rec(first),
                aux_rec(second)
            )
        }
        LambdaTerm::Rec(box first, box second, box third) => {
            LambdaTerm::rec(
                aux_rec(first),
                aux_rec(second),
                aux_rec(third),
            )
        }
    }
}

impl LambdaTerm {
    pub fn run_rec(mut self) -> LambdaTerm {
        self = update_goals_nb(self.clone(), &mut 1);
        aux_rec(self.clone())
    }
}