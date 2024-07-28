use crate::assistant::lambdas as lambdas;
use crate::assistant::lambda as lambda;

use lambda::LambdaTerm;

use lambdas::free_var::free_var;

pub fn substitute(lambda: LambdaTerm, var_name: String, what: LambdaTerm) -> LambdaTerm {
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
        LambdaTerm::App(box first, box second) => {
            LambdaTerm::app(
                substitute(first, var_name.clone(), what.clone()),
                substitute(second, var_name, what),
            )
        }
        LambdaTerm::Error => unreachable!()
    }
}
