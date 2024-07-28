use crate::assistant::lambdas as lambdas;
use crate::assistant::lambda as lambda;

use lambda::LambdaTerm;

use lambdas::{
    free_var::free_var,
    gen_name::gen_name,
    substitute::substitute,
};

use crate::DEBUG;

pub fn alpha_equiv(first: LambdaTerm, second: LambdaTerm) -> bool {
    match (first, second) {
        (LambdaTerm::Var(name1), LambdaTerm::Var(name2)) => {
            name1 == name2
        }
        (LambdaTerm::Goal(..), LambdaTerm::Goal(..)) => {
            true
        }
        (LambdaTerm::Func(name1, box first1, box second1), LambdaTerm::Func(name2, box first2, box second2))
        | (LambdaTerm::Pi(name1, box first1, box second1), LambdaTerm::Func(name2, box first2, box second2)) => {
            let mut free_names: Vec<String> = vec![];

            free_names.extend(free_var(second1.clone()));
            free_names.extend(free_var(second2.clone()));

            let new_name = gen_name(free_names);

            let replaced_second1 = rename_free_variable(name1, new_name.clone(), second1);
            let replaced_second2 = rename_free_variable(name2, new_name, second2);

            alpha_equiv(replaced_second1, replaced_second2) && alpha_equiv(first1, first2)
        }
        other => {
            if DEBUG {
                println!("ALPHA EQUIV FALSE : {:?}", other);
            }
            false
        }
    }
}

fn replace_free_variable_r(var_name: String, new_thing: LambdaTerm, lambda: LambdaTerm) -> LambdaTerm {
    match lambda.clone() {
        LambdaTerm::Var(x) => {
            if x == var_name {
                return new_thing;
            } else {
                return lambda;
            }
        }
        LambdaTerm::Goal(box typ, nb) => {
            LambdaTerm::goalnb(replace_free_variable_r(var_name, new_thing, typ), nb)
        }
        LambdaTerm::Func(name, box first, box second) => {
            if name == var_name {
                LambdaTerm::func(name, replace_free_variable_r(var_name, new_thing, first), second)
            } else {
                LambdaTerm::func(
                    name, 
                    replace_free_variable_r(var_name.clone(), new_thing.clone(), first), 
                    replace_free_variable_r(var_name, new_thing, second), 
                )
            }
        }
        LambdaTerm::Pi(name, box first, box second) => {
            if name == var_name {
                LambdaTerm::pi(name, replace_free_variable_r(var_name, new_thing, first), second)
            } else {
                LambdaTerm::pi(
                    name, 
                    replace_free_variable_r(var_name.clone(), new_thing.clone(), first), 
                    replace_free_variable_r(var_name, new_thing, second), 
                )
            }
        }
        LambdaTerm::Error => panic!()
    }
}

fn alpha_convert(used_names: Vec<String>, lambda: LambdaTerm) -> LambdaTerm {
    match lambda.clone() {
        LambdaTerm::Var(name) => {
            lambda
        }
        LambdaTerm::Goal(box typ, nb) => {
            LambdaTerm::goalnb(alpha_convert(used_names, typ), nb)
        }
        LambdaTerm::Func(name, box first, box second) => {
            let new_name = gen_name(used_names.clone());
            let mut used_names = used_names.clone();
            used_names.push(new_name.clone());
            let converted = alpha_convert(used_names.clone(), second);
            let renamed = replace_free_variable_r(name, LambdaTerm::var(new_name.as_str()), converted.clone());
            
            LambdaTerm::func(new_name, alpha_convert(used_names, first), renamed)
        }
        LambdaTerm::Pi(name, box first, box second) => {
            let new_name = gen_name(used_names.clone());
            let mut used_names = used_names.clone();
            used_names.push(new_name.clone());
            let converted = alpha_convert(used_names.clone(), second);
            let renamed = replace_free_variable_r(name, LambdaTerm::var(new_name.as_str()), converted);
            
            LambdaTerm::pi(new_name, alpha_convert(used_names, first), renamed)
        }
        LambdaTerm::Error => panic!()
    }
}

fn replace_free_variable(name: String, new_term: LambdaTerm, lambda: LambdaTerm) -> LambdaTerm {
    let mut myfree = free_var(new_term.clone());
    myfree.push(name.clone());
    replace_free_variable_r(name, new_term, alpha_convert(myfree, lambda))
}

fn rename_free_variable(name: String, new_name: String, lambda: LambdaTerm) -> LambdaTerm { 
    replace_free_variable(name, LambdaTerm::Var(new_name), lambda)
}
