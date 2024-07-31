use crate::assistant::{lambda as lambda, lambdas::free_var::free_var};

use lambda::LambdaTerm;

use std::fmt;

impl std::fmt::Display for LambdaTerm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LambdaTerm::Var(name) => {
                write!(f, "{}", name)
            }
            LambdaTerm::Goal(box typ, _nb) => {
                write!(f, "Goal({})", typ)
            }
            LambdaTerm::Pi(name, box first, box second) => {
                // we look if name is free in first and second
                let first_vars = free_var(first.clone());
                let second_vars = free_var(second.clone());
                let mut total_vars: Vec<String> = vec![];
                total_vars.extend(first_vars);
                total_vars.extend(second_vars);
                if !total_vars.contains(name) {
                    if *second == LambdaTerm::Bot {
                        write!(f, "~({})", first)
                    } else {
                        write!(f, "({} -> {})", first, second)
                    }
                } else {
                    write!(f, "(∀ {}:{}, {})", name, first, second)
                }
            }
            LambdaTerm::Sigma(name, box first, box second) => {
                // we look if name is free in first and second
                let first_vars = free_var(first.clone());
                let second_vars = free_var(second.clone());
                let mut total_vars: Vec<String> = vec![];
                total_vars.extend(first_vars);
                total_vars.extend(second_vars);
                if !total_vars.contains(name) {
                    write!(f, "({} /\\ {})", first, second)
                } else {
                    write!(f, "(∃ {}:{}, {})", name, first, second)
                }
            }
            LambdaTerm::App(box first, box second) => {
                write!(f, "App{}({})", first, second)
            }
            LambdaTerm::Or(box first, box second) => {
                write!(f, "({} \\/ {})", first, second)
            }
            LambdaTerm::Left(box first, box _second) => {
                write!(f, "Left({})", first)
            }
            LambdaTerm::Right(box first, box _second) => {
                write!(f, "Right({})", first)
            }
            LambdaTerm::ExFalso(box first, box second) => {
                write!(f, "ExFalso({})({})", first, second)
            }
            LambdaTerm::Types => {
                write!(f, "Prop")
            }
            LambdaTerm::Bot => {
                write!(f, "Bot")
            }
            LambdaTerm::Top => {
                write!(f, "Top")
            }
            LambdaTerm::Func(name, box first, box second) => {
                write!(f, "(func {} : {}=>{})", name, first, second)
            }
            LambdaTerm::Couple(box first, box second, _typecheck) => {
                write!(f, "({}, {})", first, second)
            }
            LambdaTerm::Match(box first, box second, box third) => {
                write!(f, "Match({}, {}, {})", first, second, third)
            }
            LambdaTerm::Error => {
                write!(f, "ERROR")
            }
            LambdaTerm::Proj(box first, box second) => {
                write!(f, "Proj({}, {})", first, second)

            }
        }
    }
}