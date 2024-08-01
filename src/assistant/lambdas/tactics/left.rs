use crate::assistant::lambdas as lambdas;
use crate::assistant::lambda as lambda;

use lambda::LambdaTerm;

use lambdas::update_nbs::update_goals_nb;

fn aux_left(root: LambdaTerm) -> LambdaTerm {
    match root {
        // we match for a goal of the good type
        LambdaTerm::Goal(box LambdaTerm::Or(box first, box second), nb) 
        if nb == 1 => {
            LambdaTerm::left(
                LambdaTerm::goal(first),
                second
            )
        }
        // we propagate
        LambdaTerm::Var(..) 
        | LambdaTerm::Goal(..)
        | LambdaTerm::Types
        | LambdaTerm::Bool
        | LambdaTerm::TBool
        | LambdaTerm::FBool
        | LambdaTerm::Bot
        | LambdaTerm::Top
        | LambdaTerm::Error => {
            root
        },
        LambdaTerm::Pi(name, box first, box second) => {
            LambdaTerm::pi(
                name,
                aux_left(first),
                aux_left(second)
            )
        }
        LambdaTerm::Sigma(name, box first, box second) => {
            LambdaTerm::sigma(
                name,
                aux_left(first),
                aux_left(second)
            )
        }
        LambdaTerm::Func(name, box first, box second) => {
            LambdaTerm::func(
                name,
                aux_left(first),
                aux_left(second)
            )
        }
        LambdaTerm::App(box first, box second) => {
            LambdaTerm::app(
                aux_left(first),
                aux_left(second)
            )
        }
        LambdaTerm::Or(box first, box second) => {
            LambdaTerm::or(
                aux_left(first),
                aux_left(second)
            )
        }
        LambdaTerm::Left(box first, box second) => {
            LambdaTerm::left(
                aux_left(first),
                aux_left(second)
            )
        }
        LambdaTerm::Right(box first, box second) => {
            LambdaTerm::right(
                aux_left(first),
                aux_left(second)
            )
        }
        LambdaTerm::ExFalso(box first, box second) => {
            LambdaTerm::exfalso(
                aux_left(first),
                aux_left(second)
            )
        }
        LambdaTerm::Proj(box first, box second) => {
            LambdaTerm::proj(
                aux_left(first),
                aux_left(second)
            )
        }
        LambdaTerm::Eq(box first, box second) => {
            LambdaTerm::eq(
                aux_left(first),
                aux_left(second)
            )
        }
        LambdaTerm::Refl(box first) => {
            LambdaTerm::refl(
                aux_left(first)
            )
        }
        LambdaTerm::Couple(box first, box second, box third) => {
            LambdaTerm::couple(
                aux_left(first),
                aux_left(second),
                aux_left(third)
            )
        }
        LambdaTerm::Match(box first, box second, box third) => {
            LambdaTerm::match_new(
                aux_left(first),
                aux_left(second),
                aux_left(third)
            )
        }
        LambdaTerm::Rewrite(box first, box second, box third) => {
            LambdaTerm::rewrite(
                aux_left(first),
                aux_left(second),
                aux_left(third)
            )
        }
        LambdaTerm::Bif(box first, box second, box third) => {
            LambdaTerm::bif(
                aux_left(first),
                aux_left(second),
                aux_left(third)
            )
        }
    }
}

impl LambdaTerm {
    pub fn run_left(mut self) -> LambdaTerm {
        self = update_goals_nb(self.clone(), &mut 1);
        aux_left(self.clone())
    }
}