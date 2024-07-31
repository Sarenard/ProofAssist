use crate::assistant::lambdas::alpha_equiv::rename_free_variable;
use crate::assistant::lambdas::gen_name::gen_name;
use crate::assistant::lambdas as lambdas;
use crate::assistant::lambda as lambda;

use lambda::LambdaTerm;

use lambdas::free_var::free_var;

pub fn substitute(lambda: LambdaTerm, var_name: String, what: LambdaTerm) -> LambdaTerm {
    match lambda.clone() {
        LambdaTerm::Bot => {
            LambdaTerm::Bot
        }
        LambdaTerm::Top => {
            LambdaTerm::Top
        }
        LambdaTerm::Types => {
            LambdaTerm::Types
        }
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
                let mut var_used: Vec<String> = vec![];
                var_used.extend(free_var(what.clone()));
                var_used.extend(free_var(lambda.clone()));
                var_used.push(name.clone());
                let new_name = gen_name(var_used);
                let new_body = rename_free_variable(name, new_name.clone(), body);
                substitute(LambdaTerm::pi(new_name, typ, new_body), var_name, what)
            } else {
                // we just insert
                LambdaTerm::pi(
                    name, 
                    substitute(typ, var_name.clone(), what.clone()),
                    substitute(body, var_name, what),
                )
            }
        }
        LambdaTerm::Sigma(name, box typ, box body) => {
            let free_vars = free_var(what.clone());
            if var_name == name || free_vars.contains(&name) {
                // we need to rename the var before substituting
                let mut var_used: Vec<String> = vec![];
                var_used.extend(free_var(what.clone()));
                var_used.extend(free_var(lambda.clone()));
                var_used.push(name.clone());
                let new_name = gen_name(var_used);
                let new_body = rename_free_variable(name, new_name.clone(), body);
                substitute(LambdaTerm::sigma(new_name, typ, new_body), var_name, what)
            } else {
                // we just insert
                LambdaTerm::sigma(
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
        LambdaTerm::Or(box first, box second) => {
            LambdaTerm::or(
                substitute(first, var_name.clone(), what.clone()),
                substitute(second, var_name, what),
            )
        }
        LambdaTerm::Left(box first, box second) => {
            LambdaTerm::left(
                substitute(first, var_name.clone(), what.clone()),
                substitute(second, var_name, what),
            )
        }
        LambdaTerm::Right(box first, box second) => {
            LambdaTerm::right(
                substitute(first, var_name.clone(), what.clone()),
                substitute(second, var_name, what),
            )
        }
        LambdaTerm::ExFalso(box first, box second) => {
            LambdaTerm::exfalso(
                substitute(first, var_name.clone(), what.clone()),
                substitute(second, var_name, what),
            )
        }
        LambdaTerm::Proj(box first, box second) => {
            LambdaTerm::proj(
                substitute(first, var_name.clone(), what.clone()),
                substitute(second, var_name, what),
            )
        }
        LambdaTerm::Couple(box first, box second, box third) => {
            LambdaTerm::couple(
                substitute(first, var_name.clone(), what.clone()),
                substitute(second, var_name.clone(), what.clone()),
                substitute(third, var_name, what),
            )
        }
        LambdaTerm::Match(box first, box second, box third) => {
            LambdaTerm::match_new(
                substitute(first, var_name.clone(), what.clone()),
                substitute(second, var_name.clone(), what.clone()),
                substitute(third, var_name, what),
            )
        }
        LambdaTerm::Error => unreachable!()
    }
}
