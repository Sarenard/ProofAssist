use std::collections::HashMap;

use crate::assistant::lambdas as lambdas;
use crate::assistant::lambda as lambda;

use lambda::LambdaTerm;

use lambdas::update_nbs::update_goals_nb;


fn aux_absurd(root: LambdaTerm, typ: LambdaTerm, context: HashMap<String, LambdaTerm>) -> LambdaTerm {
    match root {
        LambdaTerm::Goal(_goal, nb2) 
        if nb2 == 1 => {
            LambdaTerm::exfalso(
                typ,
                LambdaTerm::goal(LambdaTerm::Bot)
            )
        }
        // we propagate
        LambdaTerm::Var(..) 
        | LambdaTerm::Goal(..)
        | LambdaTerm::Types
        | LambdaTerm::Bot
        | LambdaTerm::Naturals
        | LambdaTerm::Zero
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
                aux_absurd(first, typ.clone(), context.clone()),
                aux_absurd(second, typ, context)
            )
        }
        LambdaTerm::Sigma(func_name, box first, box second) => {
            LambdaTerm::sigma(
                func_name.clone(),
                aux_absurd(first, typ.clone(), context.clone()),
                aux_absurd(second, typ, context)
            )
        }
        LambdaTerm::Inversion(box first, box second) => {
            LambdaTerm::inversion(
                aux_absurd(first, typ.clone(), context.clone()),
                aux_absurd(second, typ, context)
            )
        }
        LambdaTerm::Func(func_name, box first, box second) => {
            let mut new_context = context.clone();
            new_context.insert(func_name.clone(), first.clone());
            LambdaTerm::func(
                func_name.clone(),
                aux_absurd(first, typ.clone(), new_context.clone()),
                aux_absurd(second, typ, new_context)
            )
        }
        LambdaTerm::App(box first, box second) => {
            LambdaTerm::app(
                aux_absurd(first, typ.clone(), context.clone()),
                aux_absurd(second, typ, context)
            )
        }
        LambdaTerm::Or(box first, box second) => {
            LambdaTerm::or(
                aux_absurd(first, typ.clone(), context.clone()),
                aux_absurd(second, typ, context)
            )
        }
        LambdaTerm::Left(box first, box second) => {
            LambdaTerm::left(
                aux_absurd(first, typ.clone(), context.clone()),
                aux_absurd(second, typ, context)
            )
        }
        LambdaTerm::Right(box first, box second) => {
            LambdaTerm::right(
                aux_absurd(first, typ.clone(), context.clone()),
                aux_absurd(second, typ, context)
            )
        }
        LambdaTerm::ExFalso(box first, box second) => {
            LambdaTerm::exfalso(
                aux_absurd(first, typ.clone(), context.clone()),
                aux_absurd(second, typ, context)
            )
        }
        LambdaTerm::Proj(box first, box second) => {
            LambdaTerm::proj(
                aux_absurd(first, typ.clone(), context.clone()),
                aux_absurd(second, typ, context)
            )
        }
        LambdaTerm::Eq(box first, box second) => {
            LambdaTerm::eq(
                aux_absurd(first, typ.clone(), context.clone()),
                aux_absurd(second, typ, context)
            )
        }
        LambdaTerm::Refl(box first) => {
            LambdaTerm::refl(
                aux_absurd(first, typ, context)
            )
        }
        LambdaTerm::Couple(box first, box second, box third) => {
            LambdaTerm::couple(
                aux_absurd(first, typ.clone(), context.clone()),
                aux_absurd(second, typ.clone(), context.clone()),
                aux_absurd(third, typ, context)
            )
        }
        LambdaTerm::Match(box first, box second, box third) => {
            LambdaTerm::match_new(
                aux_absurd(first, typ.clone(), context.clone()),
                aux_absurd(second, typ.clone(), context.clone()),
                aux_absurd(third, typ, context)
            )
        }
        LambdaTerm::Rewrite(box first, box second, box third) => {
            LambdaTerm::rewrite(
                aux_absurd(first, typ.clone(), context.clone()),
                aux_absurd(second, typ.clone(), context.clone()),
                aux_absurd(third, typ, context)
            )
        }
        LambdaTerm::Bif(box first, box second, box third) => {
            LambdaTerm::bif(
                aux_absurd(first, typ.clone(), context.clone()),
                aux_absurd(second, typ.clone(), context.clone()),
                aux_absurd(third, typ, context)
            )
        }
        LambdaTerm::Succ(box first) => {
            LambdaTerm::succ(
                aux_absurd(first, typ, context)
            )
        }
        LambdaTerm::Rec(box first, box second, box third) => {
            LambdaTerm::rec(
                aux_absurd(first, typ.clone(), context.clone()),
                aux_absurd(second, typ.clone(), context.clone()),
                aux_absurd(third, typ.clone(), context.clone()),
            )
        }
    }
}

impl LambdaTerm {
    pub fn absurd(mut self, typ: LambdaTerm) -> LambdaTerm {
        self = update_goals_nb(self.clone(), &mut 1);
        aux_absurd(self.clone(), typ, HashMap::new())
    }
}
