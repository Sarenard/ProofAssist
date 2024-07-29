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
                    write!(f, "({} -> {})", first, second)
                } else {
                    write!(f, "∀ {}:{}, {}", name, first, second)
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
                    write!(f, "∃ {}:{}, {}", name, first, second)
                }
            }
            LambdaTerm::App(box first, box second) => {
                write!(f, "App{}({})", first, second)
            }
            LambdaTerm::Types => {
                write!(f, "Prop")
            }
            LambdaTerm::Func(name, box first, box second) => {
                write!(f, "(func {} : {}=>{})", name, first, second)
            }
            LambdaTerm::Couple(box first, box second, _typecheck) => {
                write!(f, "({}, {})", first, second)
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