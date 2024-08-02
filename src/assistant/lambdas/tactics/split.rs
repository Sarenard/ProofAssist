
use crate::assistant::lambdas::free_var::free_var;
use crate::assistant::lambdas as lambdas;
use crate::assistant::lambda as lambda;

use lambda::LambdaTerm;

use lambdas::update_nbs::update_goals_nb;


fn aux_split(root: LambdaTerm) -> LambdaTerm {
    match root {
        LambdaTerm::Goal(box LambdaTerm::Sigma(name, box first, box second), nb) 
        if nb == 1 && !free_var(first.clone()).contains(&name) && !free_var(second.clone()).contains(&name) => {
            LambdaTerm::couple(
                LambdaTerm::goal(first.clone()),
                LambdaTerm::goal(second.clone()),
                LambdaTerm::sigma(name, first, second),
            )
        }
        // we propagate
        LambdaTerm::Var(..) 
        | LambdaTerm::Types
        | LambdaTerm::Bot
        | LambdaTerm::Top
        | LambdaTerm::Bool
        | LambdaTerm::Naturals
        | LambdaTerm::Zero
        | LambdaTerm::TBool
        | LambdaTerm::FBool
        | LambdaTerm::Goal(..) => {
            root
        },
        LambdaTerm::Refl(box first) => {
            LambdaTerm::refl(aux_split(first))
        }
        LambdaTerm::Func(str, box typ, box lambdaterm) => {
            LambdaTerm::func(
                str, 
                aux_split(typ), 
                aux_split(lambdaterm)
        )
        }
        LambdaTerm::Pi(str, box typ, box lambdaterm) => {
            LambdaTerm::pi(
                str, 
                aux_split(typ), 
                aux_split(lambdaterm)
            )
        }
        LambdaTerm::Sigma(str, box typ, box lambdaterm) => {
            LambdaTerm::sigma(
                str, 
                aux_split(typ), 
                aux_split(lambdaterm)
            )
        }
        LambdaTerm::App(box first, box second) => {
            LambdaTerm::app(
                aux_split(first),
                aux_split(second)
            )
        }
        LambdaTerm::Left(box first, box second) => {
            LambdaTerm::left(
                aux_split(first),
                aux_split(second)
            )
        }
        LambdaTerm::Right(box first, box second) => {
            LambdaTerm::right(
                aux_split(first),
                aux_split(second)
            )
        }
        LambdaTerm::Or(box first, box second) => {
            LambdaTerm::or(
                aux_split(first),
                aux_split(second)
            )
        }
        LambdaTerm::ExFalso(box first, box second) => {
            LambdaTerm::exfalso(
                aux_split(first),
                aux_split(second)
            )
        }
        LambdaTerm::Proj(box first, box second) => {
            LambdaTerm::proj(
                aux_split(first),
                aux_split(second)
            )
        }
        LambdaTerm::Eq(box first, box second) => {
            LambdaTerm::eq(
                aux_split(first),
                aux_split(second)
            )
        }
        LambdaTerm::Couple(box first, box second, box third) => {
            LambdaTerm::couple(
                aux_split(first),
                aux_split(second),
                aux_split(third)
            )
        }
        LambdaTerm::Match(box first, box second, box third) => {
            LambdaTerm::match_new(
                aux_split(first),
                aux_split(second),
                aux_split(third)
            )
        }
        LambdaTerm::Error => panic!(),
        LambdaTerm::Rewrite(box first, box second, box third) => {
            LambdaTerm::rewrite(
                aux_split(first),
                aux_split(second),
                aux_split(third)
            )
        }
        LambdaTerm::Bif(box first, box second, box third) => {
            LambdaTerm::bif(
                aux_split(first),
                aux_split(second),
                aux_split(third)
            )
        }
        LambdaTerm::Succ(box first) => {
            LambdaTerm::succ(
                aux_split(first)
            )
        }
        LambdaTerm::Inversion(box first, box second) => {
            LambdaTerm::inversion(
                aux_split(first),
                aux_split(second)
            )
        }
    }
}

impl LambdaTerm {
    // naive approch
    pub fn split(mut self) -> LambdaTerm {
        self = update_goals_nb(self.clone(), &mut 1);
        aux_split(self.clone())
    }
}
