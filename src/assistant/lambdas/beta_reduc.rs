use crate::assistant::lambdas as lambdas;
use crate::assistant::lambda as lambda;

use lambda::LambdaTerm;

use lambdas::free_var::free_var;

fn substitute(lambda: LambdaTerm, var_name: String, what: LambdaTerm) -> LambdaTerm {
    match lambda.clone() {
        LambdaTerm::Var(name) => {
            if name == var_name {
                what
            } else {
                lambda
            }
        }
        LambdaTerm::Goal(box typ, nb) => {
            LambdaTerm::goalnb(substitute(typ, var_name, what), nb)
        }
        LambdaTerm::Pi(name, box typ, box body) => {
            let free_vars = free_var(what.clone());
            if var_name == name || free_vars.contains(&name) {
                // we need to rename the var before substituting
                todo!()
            } else {
                // we just insert
                LambdaTerm::pi(
                    name, 
                    substitute(typ, var_name.clone(), what.clone()),
                    substitute(body, var_name, what),
                )
            }
        }
        LambdaTerm::Func(name, box typ, box body) => {
            let free_vars = free_var(what.clone());
            if var_name == name || free_vars.contains(&name) {
                // we need to rename the var before substituting
                todo!()
            } else {
                // we just insert
                LambdaTerm::func(
                    name, 
                    substitute(typ, var_name.clone(), what.clone()),
                    substitute(body, var_name, what),
                )
            }
        }
        LambdaTerm::Error => unreachable!()
    }
}

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

fn beta_reduce(lambda: LambdaTerm) -> LambdaTerm {
    match betareduc_step(lambda.clone(), vec![]) {
        Some(reduced) => beta_reduce(reduced),
        None => lambda
    }
}
