use crate::assistant::lambda as lambda;

use lambda::LambdaTerm;

use super::{alpha_equiv::{alpha_convert, alpha_equiv}, free_var::free_var};

use crate::DEBUG;

fn replace_intern(lambdaterm: LambdaTerm, to_replace: LambdaTerm, replacement: LambdaTerm) -> LambdaTerm {
    // println!("{}, {}, equiv: {}, {}", lambdaterm, to_replace, alpha_equiv(lambdaterm.clone(), to_replace.clone()), lambdaterm == to_replace);
    if alpha_equiv(lambdaterm.clone(), to_replace.clone()) {
        return replacement;
    }
    match lambdaterm.clone() {
        // we propagate
        LambdaTerm::Var(..) 
        | LambdaTerm::Goal(..)
        | LambdaTerm::Types
        | LambdaTerm::Bool
        | LambdaTerm::FBool
        | LambdaTerm::TBool
        | LambdaTerm::Naturals
        | LambdaTerm::Zero
        | LambdaTerm::Bot
        | LambdaTerm::Top
        | LambdaTerm::Error => {
            lambdaterm
        },
        LambdaTerm::Pi(name, box first, box second) => {
            if free_var(to_replace.clone()).contains(&name) {
                lambdaterm
            } else {
                LambdaTerm::pi(
                    name,
                    replace_intern(first, to_replace.clone(), replacement.clone()),
                    replace_intern(second, to_replace.clone(), replacement.clone()),
                )
            }
        }
        LambdaTerm::Sigma(name, box first, box second) => {
            if free_var(to_replace.clone()).contains(&name) {
                lambdaterm
            } else {
                LambdaTerm::sigma(
                    name,
                    replace_intern(first, to_replace.clone(), replacement.clone()),
                    replace_intern(second, to_replace.clone(), replacement.clone()),
                )
            }
        }
        LambdaTerm::Func(name, box first, box second) => {
            if free_var(to_replace.clone()).contains(&name) {
                lambdaterm
            } else {
                LambdaTerm::func(
                    name,
                    replace_intern(first, to_replace.clone(), replacement.clone()),
                    replace_intern(second, to_replace.clone(), replacement.clone()),
                )
            }
        }
        LambdaTerm::App(box first, box second) => {
            LambdaTerm::app(
                replace_intern(first, to_replace.clone(), replacement.clone()),
                replace_intern(second, to_replace.clone(), replacement.clone()),
            )
        }
        LambdaTerm::Or(box first, box second) => {
            LambdaTerm::or(
                replace_intern(first, to_replace.clone(), replacement.clone()),
                replace_intern(second, to_replace.clone(), replacement.clone()),
            )
        }
        LambdaTerm::Left(box first, box second) => {
            LambdaTerm::left(
                replace_intern(first, to_replace.clone(), replacement.clone()),
                replace_intern(second, to_replace.clone(), replacement.clone()),
            )
        }
        LambdaTerm::Right(box first, box second) => {
            LambdaTerm::right(
                replace_intern(first, to_replace.clone(), replacement.clone()),
                replace_intern(second, to_replace.clone(), replacement.clone()),
            )
        }
        LambdaTerm::Rec(box first, box second, box third) => {
            LambdaTerm::rec(
                replace_intern(first, to_replace.clone(), replacement.clone()),
                replace_intern(second, to_replace.clone(), replacement.clone()),
                replace_intern(third, to_replace.clone(), replacement.clone()),
            )
        }
        LambdaTerm::ExFalso(box first, box second) => {
            LambdaTerm::exfalso(
                replace_intern(first, to_replace.clone(), replacement.clone()),
                replace_intern(second, to_replace.clone(), replacement.clone()),
            )
        }
        LambdaTerm::Proj(box first, box second) => {
            LambdaTerm::proj(
                replace_intern(first, to_replace.clone(), replacement.clone()),
                replace_intern(second, to_replace.clone(), replacement.clone()),
            )
        }
        LambdaTerm::Eq(box first, box second) => {
            LambdaTerm::eq(
                replace_intern(first, to_replace.clone(), replacement.clone()),
                replace_intern(second, to_replace.clone(), replacement.clone()),
            )
        }
        LambdaTerm::Refl(box first) => {
            LambdaTerm::refl(
                replace_intern(first, to_replace.clone(), replacement.clone()),
            )
        }
        LambdaTerm::Couple(box first, box second, box third) => {
            LambdaTerm::couple(
                replace_intern(first, to_replace.clone(), replacement.clone()),
                replace_intern(second, to_replace.clone(), replacement.clone()),
                replace_intern(third, to_replace.clone(), replacement.clone()),
            )
        }
        LambdaTerm::Bif(box first, box second, box third) => {
            LambdaTerm::bif(
                replace_intern(first, to_replace.clone(), replacement.clone()),
                replace_intern(second, to_replace.clone(), replacement.clone()),
                replace_intern(third, to_replace.clone(), replacement.clone()),
            )
        }
        LambdaTerm::Match(box first, box second, box third) => {
            LambdaTerm::match_new(
                replace_intern(first, to_replace.clone(), replacement.clone()),
                replace_intern(second, to_replace.clone(), replacement.clone()),
                replace_intern(third, to_replace.clone(), replacement.clone()),
            )
        }
        LambdaTerm::Rewrite(box first, box second, box third) => {
            LambdaTerm::rewrite(
                replace_intern(first, to_replace.clone(), replacement.clone()),
                replace_intern(second, to_replace.clone(), replacement.clone()),
                replace_intern(third, to_replace.clone(), replacement.clone()),
            )
        }
        LambdaTerm::Succ(box first) => {
            LambdaTerm::succ(
                replace_intern(first, to_replace.clone(), replacement.clone()),
            )
        }
        LambdaTerm::Inversion(box first, box second) => {
            LambdaTerm::inversion(
                replace_intern(first, to_replace.clone(), replacement.clone()),
                replace_intern(second, to_replace.clone(), replacement.clone()),
            )
        }
    }
}

// thx coda for thinking bout alpha conversion
pub fn replace(lambdaterm: LambdaTerm, to_replace: LambdaTerm, replacement: LambdaTerm) -> LambdaTerm {
    let mut usedvariables: Vec<String> = vec![];

    usedvariables.extend(free_var(to_replace.clone()));
    usedvariables.extend(free_var(replacement.clone()));

    let converted = alpha_convert(usedvariables, lambdaterm.clone());

    if DEBUG {println!("{:?}, replace({:?}, {:?}, {:?})", lambdaterm, converted, to_replace, replacement);}

    replace_intern(
        converted,
        to_replace,
        replacement
    )
}