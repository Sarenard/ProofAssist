use crate::assistant::lambdas as lambdas;
use crate::assistant::lambda as lambda;

use lambda::LambdaTerm;

use lambdas::{
    free_var::free_var,
    gen_name::gen_name,
};

use crate::DEBUG;

pub fn alpha_equiv(first: LambdaTerm, second: LambdaTerm) -> bool {
    if DEBUG {
        println!("trying to alpha equiv {:?} {:?}", first, second);
    }
    match (first, second) {
        (LambdaTerm::Var(name1), LambdaTerm::Var(name2)) => {
            name1 == name2
        }
        (LambdaTerm::Goal(..), LambdaTerm::Goal(..)) => {
            true
        }
        (LambdaTerm::Refl(box first), LambdaTerm::Refl(box second)) => {
            alpha_equiv(first, second)
        }
        (LambdaTerm::Eq(box first1, box second1), LambdaTerm::Eq(box first2, box second2)) => {
            let one = alpha_equiv(first1, first2);
            let two = alpha_equiv(second1, second2);

            one && two
        }
        (LambdaTerm::Top, LambdaTerm::Top) => {
            true
        }
        (LambdaTerm::Bot, LambdaTerm::Bot) => {
            true
        }
        (LambdaTerm::Func(name1, box first1, box second1), LambdaTerm::Func(name2, box first2, box second2))
        | (LambdaTerm::Sigma(name1, box first1, box second1), LambdaTerm::Sigma(name2, box first2, box second2))
        | (LambdaTerm::Pi(name1, box first1, box second1), LambdaTerm::Pi(name2, box first2, box second2)) => {
            let mut free_names: Vec<String> = vec![];

            free_names.extend(free_var(second1.clone()));
            free_names.extend(free_var(second2.clone()));

            let new_name = gen_name(free_names);

            let replaced_second1 = rename_free_variable(name1.clone(), new_name.clone(), second1);
            let replaced_second2 = rename_free_variable(name2.clone(), new_name, second2);

            let first = alpha_equiv(replaced_second1, replaced_second2);
            let second = alpha_equiv(first1, first2);

            if DEBUG {
                println!("{}{}First : {}, Second : {}", name1, name2, first, second);
            }

            first && second
        }
        (LambdaTerm::App(box a1, box b1), LambdaTerm::App(box a2, box b2))
        | (LambdaTerm::ExFalso(box a1, box b1), LambdaTerm::ExFalso(box a2, box b2))
        | (LambdaTerm::Or(box a1, box b1), LambdaTerm::Or(box a2, box b2))
        | (LambdaTerm::Left(box a1, box b1), LambdaTerm::Left(box a2, box b2))
        | (LambdaTerm::Right(box a1, box b1), LambdaTerm::Right(box a2, box b2)) => {
            let first = alpha_equiv(a1, a2);
            let second = alpha_equiv(b1, b2);

            first && second
        }
        (
            LambdaTerm::Rewrite(box first1, box second1, box third1),
            LambdaTerm::Rewrite(box first2, box second2, box third2)
        )
        | (
            LambdaTerm::Couple(box first1, box second1, box third1),
            LambdaTerm::Couple(box first2, box second2, box third2)
        )
        | (
            LambdaTerm::Match(box first1, box second1, box third1),
            LambdaTerm::Match(box first2, box second2, box third2)
        ) => {
            let one = alpha_equiv(first1, first2);
            let two = alpha_equiv(second1, second2);
            let three = alpha_equiv(third1, third2);

            if DEBUG {
                println!("one : {}, two : {}, three : {}", one, two, three);
            }

            one && two && three
        }
        (
            LambdaTerm::Proj(box first1, box second1),
            LambdaTerm::Proj(box first2, box second2)
        ) => {
            let one = alpha_equiv(first1, first2);
            let two = alpha_equiv(second1, second2);

            if DEBUG {
                println!("one : {}, two : {}", one, two);
            }

            one && two
        }
        (LambdaTerm::Types, LambdaTerm::Types) => {
            true
        },
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
        LambdaTerm::Types => LambdaTerm::Types,
        LambdaTerm::Var(x) => {
            if x == var_name {
                return new_thing;
            } else {
                return lambda;
            }
        }
        LambdaTerm::Bot => {
            LambdaTerm::Bot
        }
        LambdaTerm::Top => {
            LambdaTerm::Top
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
        LambdaTerm::Sigma(name, box first, box second) => {
            if name == var_name {
                LambdaTerm::sigma(name, replace_free_variable_r(var_name, new_thing, first), second)
            } else {
                LambdaTerm::sigma(
                    name, 
                    replace_free_variable_r(var_name.clone(), new_thing.clone(), first), 
                    replace_free_variable_r(var_name, new_thing, second), 
                )
            }
        }
        LambdaTerm::App(box first, box second) => {
            LambdaTerm::app(
                replace_free_variable_r(var_name.clone(), new_thing.clone(), first), 
                replace_free_variable_r(var_name, new_thing, second), 
            )
        }
        LambdaTerm::ExFalso(box first, box second) => {
            LambdaTerm::exfalso(
                first,
                replace_free_variable_r(var_name, new_thing, second), 
            )
        }
        LambdaTerm::Proj(box first, box second) => {
            LambdaTerm::proj(
                replace_free_variable_r(var_name.clone(), new_thing.clone(), first), 
                replace_free_variable_r(var_name, new_thing, second), 
            )
        }
        LambdaTerm::Or(box first, box second) => {
            LambdaTerm::or(
                replace_free_variable_r(var_name.clone(), new_thing.clone(), first), 
                replace_free_variable_r(var_name, new_thing, second), 
            )
        }
        LambdaTerm::Left(box first, box second) => {
            LambdaTerm::left(
                replace_free_variable_r(var_name.clone(), new_thing.clone(), first), 
                replace_free_variable_r(var_name, new_thing, second), 
            )
        }
        LambdaTerm::Right(box first, box second) => {
            LambdaTerm::right(
                replace_free_variable_r(var_name.clone(), new_thing.clone(), first), 
                replace_free_variable_r(var_name, new_thing, second), 
            )
        }
        LambdaTerm::Couple(box first, box second, box third) => {
            LambdaTerm::couple(
                replace_free_variable_r(var_name.clone(), new_thing.clone(), first), 
                replace_free_variable_r(var_name.clone(), new_thing.clone(), second), 
                replace_free_variable_r(var_name, new_thing, third), 
            )
        }
        LambdaTerm::Rewrite(box first, box second, box third) => {
            LambdaTerm::rewrite(
                replace_free_variable_r(var_name.clone(), new_thing.clone(), first), 
                replace_free_variable_r(var_name.clone(), new_thing.clone(), second), 
                replace_free_variable_r(var_name, new_thing, third), 
            )
        }
        LambdaTerm::Match(box first, box second, box third) => {
            LambdaTerm::match_new(
                replace_free_variable_r(var_name.clone(), new_thing.clone(), first), 
                replace_free_variable_r(var_name.clone(), new_thing.clone(), second), 
                replace_free_variable_r(var_name, new_thing, third), 
            )
        }
        LambdaTerm::Error => panic!(),
        LambdaTerm::Eq(box first, box second) => {
            LambdaTerm::eq(
                replace_free_variable_r(var_name.clone(), new_thing.clone(), first),
                replace_free_variable_r(var_name, new_thing, second),
            )
        }
        LambdaTerm::Refl(box thing) => {
            LambdaTerm::refl(replace_free_variable_r(var_name, new_thing, thing))
        }
    }
}

pub fn alpha_convert(used_names: Vec<String>, lambda: LambdaTerm) -> LambdaTerm {
    match lambda.clone() {
        LambdaTerm::Types => LambdaTerm::Types,
        LambdaTerm::Top => LambdaTerm::Top,
        LambdaTerm::Bot => LambdaTerm::Bot,
        LambdaTerm::Var(_name) => {
            lambda
        }
        LambdaTerm::Goal(box typ, nb) => {
            LambdaTerm::goalnb(alpha_convert(used_names, typ), nb)
        }
        LambdaTerm::App(box first, box second) => {
            LambdaTerm::app(
                alpha_convert(used_names.clone(), first),
                alpha_convert(used_names, second),
            )
        }
        LambdaTerm::Or(box first, box second) => {
            LambdaTerm::or(
                alpha_convert(used_names.clone(), first),
                alpha_convert(used_names, second),
            )
        }
        LambdaTerm::Left(box first, box second) => {
            LambdaTerm::left(
                alpha_convert(used_names.clone(), first),
                alpha_convert(used_names, second),
            )
        }
        LambdaTerm::Right(box first, box second) => {
            LambdaTerm::right(
                alpha_convert(used_names.clone(), first),
                alpha_convert(used_names, second),
            )
        }
        LambdaTerm::Proj(box first, box second) => {
            LambdaTerm::proj(
                alpha_convert(used_names.clone(), first),
                alpha_convert(used_names, second),
            )
        }
        LambdaTerm::Couple(box first, box second, box third) => {
            LambdaTerm::couple(
                alpha_convert(used_names.clone(), first),
                alpha_convert(used_names.clone(), second),
                alpha_convert(used_names, third),
            )
        }
        LambdaTerm::Match(box first, box second, box third) => {
            LambdaTerm::match_new(
                alpha_convert(used_names.clone(), first),
                alpha_convert(used_names.clone(), second),
                alpha_convert(used_names, third),
            )
        }
        LambdaTerm::Rewrite(box first, box second, box third) => {
            LambdaTerm::rewrite(
                alpha_convert(used_names.clone(), first),
                alpha_convert(used_names.clone(), second),
                alpha_convert(used_names, third),
            )
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
        LambdaTerm::Sigma(name, box first, box second) => {
            let new_name = gen_name(used_names.clone());
            let mut used_names = used_names.clone();
            used_names.push(new_name.clone());
            let converted = alpha_convert(used_names.clone(), second);
            let renamed = replace_free_variable_r(name, LambdaTerm::var(new_name.as_str()), converted);
            
            LambdaTerm::sigma(new_name, alpha_convert(used_names, first), renamed)
        }
        LambdaTerm::Error => panic!(),
        LambdaTerm::ExFalso(box first, box second) => {
            LambdaTerm::exfalso(first, alpha_convert(used_names, second))
        }
        LambdaTerm::Eq(box a, box b) => {
            LambdaTerm::eq(
                alpha_convert(used_names.clone(), a),
                alpha_convert(used_names, b),
            )
        }
        LambdaTerm::Refl(box a) => {
            LambdaTerm::refl(alpha_convert(used_names, a))
        }
    }
}

pub fn replace_free_variable(name: String, new_term: LambdaTerm, lambda: LambdaTerm) -> LambdaTerm {
    let mut myfree = free_var(new_term.clone());
    myfree.push(name.clone());
    replace_free_variable_r(name, new_term, alpha_convert(myfree, lambda))
}

pub fn rename_free_variable(name: String, new_name: String, lambda: LambdaTerm) -> LambdaTerm { 
    replace_free_variable(name, LambdaTerm::Var(new_name), lambda)
}
