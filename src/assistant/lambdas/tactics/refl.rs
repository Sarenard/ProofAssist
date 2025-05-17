use crate::assistant::lambdas as lambdas;
use crate::assistant::lambda as lambda;

use lambda::LambdaTerm;

use lambdas::update_nbs::update_goals_nb;

fn aux_refl(root: LambdaTerm) -> LambdaTerm {
    match root {
        // we match for a goal of the good type
        LambdaTerm::Goal(box LambdaTerm::Eq(box first, box second), nb) 
        if nb == 1 && first == second => {
            LambdaTerm::refl(first)
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
                aux_refl(first),
                aux_refl(second)
            )
        }
        LambdaTerm::Sigma(name, box first, box second) => {
            LambdaTerm::sigma(
                name,
                aux_refl(first),
                aux_refl(second)
            )
        }
        LambdaTerm::Func(name, box first, box second) => {
            LambdaTerm::func(
                name,
                aux_refl(first),
                aux_refl(second)
            )
        }
        LambdaTerm::App(box first, box second) => {
            LambdaTerm::app(
                aux_refl(first),
                aux_refl(second)
            )
        }
        LambdaTerm::Or(box first, box second) => {
            LambdaTerm::or(
                aux_refl(first),
                aux_refl(second)
            )
        }
        LambdaTerm::Left(box first, box second) => {
            LambdaTerm::left(
                aux_refl(first),
                aux_refl(second)
            )
        }
        LambdaTerm::Right(box first, box second) => {
            LambdaTerm::right(
                aux_refl(first),
                aux_refl(second)
            )
        }
        LambdaTerm::ExFalso(box first, box second) => {
            LambdaTerm::exfalso(
                aux_refl(first),
                aux_refl(second)
            )
        }
        LambdaTerm::Proj(box first, box second) => {
            LambdaTerm::proj(
                aux_refl(first),
                aux_refl(second)
            )
        }
        LambdaTerm::Eq(box first, box second) => {
            LambdaTerm::eq(
                aux_refl(first),
                aux_refl(second)
            )
        }
        LambdaTerm::Refl(box first) => {
            LambdaTerm::refl(
                aux_refl(first)
            )
        }
        LambdaTerm::Couple(box first, box second, box third) => {
            LambdaTerm::couple(
                aux_refl(first),
                aux_refl(second),
                aux_refl(third)
            )
        }
        LambdaTerm::Match(box first, box second, box third) => {
            LambdaTerm::match_new(
                aux_refl(first),
                aux_refl(second),
                aux_refl(third)
            )
        }
        LambdaTerm::Rewrite(box first, box second, box third) => {
            LambdaTerm::rewrite(
                aux_refl(first),
                aux_refl(second),
                aux_refl(third)
            )
        }
        LambdaTerm::Bif(box first, box second, box third) => {
            LambdaTerm::bif(
                aux_refl(first),
                aux_refl(second),
                aux_refl(third)
            )
        }
        LambdaTerm::Succ(box first) => {
            LambdaTerm::succ(
                aux_refl(first)
            )
        }
        LambdaTerm::Inversion(box first, box second) => {
            LambdaTerm::inversion(
                aux_refl(first),
                aux_refl(second)
            )
        }
        LambdaTerm::Rec(box first, box second, box third) => {
            LambdaTerm::rec(
                aux_refl(first),
                aux_refl(second),
                aux_refl(third),
            )
        }
    }
}

impl LambdaTerm {
    pub fn refl_run(mut self) -> LambdaTerm {
        self = update_goals_nb(self.clone(), &mut 1);
        aux_refl(self.clone())
    }
}