use crate::assistant::lambdas as lambdas;
use crate::assistant::lambda as lambda;

use lambda::LambdaTerm;

use lambdas::substitute::substitute;

fn betareduc_step(lambda: LambdaTerm, used_names: Vec<String>) -> Option<LambdaTerm> {
    match lambda {
        LambdaTerm::Error
        | LambdaTerm::Var(..) => None,
        LambdaTerm::Goal(box typ, nb) => {
            match betareduc_step(typ, used_names) {
                Some(reduced) => Some(LambdaTerm::goalnb(reduced, nb)),
                None => None
            }
        }
        LambdaTerm::Func(name, box typ, box body) => {
            match betareduc_step(typ.clone(), used_names.clone()) {
                Some(reduced) => Some(LambdaTerm::func(name, reduced, body)),
                None => {
                    match betareduc_step(body, used_names) {
                        Some(reduced) => Some(LambdaTerm::func(name, typ, reduced)),
                        None => None
                    }
                }
            }
        }
        LambdaTerm::Pi(name, box typ, box body) => {
            match betareduc_step(typ.clone(), used_names.clone()) {
                Some(reduced) => Some(LambdaTerm::func(name, reduced, body)),
                None => {
                    match betareduc_step(body, used_names) {
                        Some(reduced) => Some(LambdaTerm::pi(name, typ, reduced)),
                        None => None
                    }
                }
            }
        }

    }
}

pub fn beta_reduce(lambda: LambdaTerm) -> LambdaTerm {
    match betareduc_step(lambda.clone(), vec![]) {
        Some(reduced) => beta_reduce(reduced),
        None => lambda
    }
}
