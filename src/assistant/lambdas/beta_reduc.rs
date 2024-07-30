use crate::assistant::lambda as lambda;

use lambda::LambdaTerm;


fn betareduc_step(lambda: LambdaTerm, used_names: Vec<String>) -> Option<LambdaTerm> {
    match lambda {
        LambdaTerm::Error
        | LambdaTerm::Bot
        | LambdaTerm::Top
        | LambdaTerm::Types
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
                Some(reduced) => Some(LambdaTerm::pi(name, reduced, body)),
                None => {
                    match betareduc_step(body, used_names) {
                        Some(reduced) => Some(LambdaTerm::pi(name, typ, reduced)),
                        None => None
                    }
                }
            }
        }
        LambdaTerm::Sigma(name, box typ, box body) => {
            match betareduc_step(typ.clone(), used_names.clone()) {
                Some(reduced) => Some(LambdaTerm::sigma(name, reduced, body)),
                None => {
                    match betareduc_step(body, used_names) {
                        Some(reduced) => Some(LambdaTerm::sigma(name, typ, reduced)),
                        None => None
                    }
                }
            }
        }
        LambdaTerm::App(box first, box second) => {
            match betareduc_step(first.clone(), used_names.clone()) {
                Some(reduced) => Some(LambdaTerm::app(reduced, second)),
                None => match betareduc_step(second, used_names.clone()) {
                    Some(reduced) => Some(LambdaTerm::app(first, reduced)),
                    None => None
                }
            }
        }
        LambdaTerm::Or(box first, box second) => {
            match betareduc_step(first.clone(), used_names.clone()) {
                Some(reduced) => Some(LambdaTerm::or(reduced, second)),
                None => match betareduc_step(second, used_names.clone()) {
                    Some(reduced) => Some(LambdaTerm::or(first, reduced)),
                    None => None
                }   
            }
        }
        LambdaTerm::ExFalso(box first, box second) => {
            match betareduc_step(first.clone(), used_names.clone()) {
                Some(reduced) => Some(LambdaTerm::exfalso(reduced, second)),
                None => match betareduc_step(second, used_names.clone()) {
                    Some(reduced) => Some(LambdaTerm::exfalso(first, reduced)),
                    None => None
                }
            }
        }
        LambdaTerm::Couple(box first, box second, box third) => {
            match betareduc_step(first.clone(), used_names.clone()) {
                Some(reduced) => Some(LambdaTerm::couple(reduced, second, third)),
                None => match betareduc_step(second.clone(), used_names.clone()) {
                    Some(reduced) => Some(LambdaTerm::couple(first, reduced, third)),
                    None => match betareduc_step(third.clone(), used_names.clone()) {
                        Some(reduced) => Some(LambdaTerm::couple(first, second, reduced)),
                        None => None
                    }
                }
            }
        }
        LambdaTerm::Proj(box LambdaTerm::Couple(box first, box second, box third), box fourth) => {
            match betareduc_step(fourth.clone(), used_names) {
                Some(reduced) => Some(LambdaTerm::proj(LambdaTerm::couple(first, second, third), reduced)),
                None => Some(LambdaTerm::app(LambdaTerm::app(fourth, first), second))
            }
        }
        LambdaTerm::Proj(box first, box second) => {
            match betareduc_step(first.clone(), used_names.clone()) {
                Some(reduced) => Some(LambdaTerm::proj(reduced, second)),
                None => match betareduc_step(second, used_names.clone()) {
                    Some(reduced) => Some(LambdaTerm::proj(first, reduced)),
                    None => None
                }
            }
        }
        LambdaTerm::Left(box first, box second) => {
            let reduced = betareduc_step(first, used_names);
            match reduced {
                Some(reduced) => Some(LambdaTerm::left(reduced, second)),
                None => None
            }
        }
        LambdaTerm::Right(box first, box second) => {
            let reduced = betareduc_step(first, used_names);
            match reduced {
                Some(reduced) => Some(LambdaTerm::right(reduced, second)),
                None => None
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
