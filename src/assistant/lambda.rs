use std::collections::VecDeque;

use crate::assistant::types::Type as Type;

#[derive(Debug, Clone)]
pub enum LambdaTerm {
   Var(String),
   Couple(Box<LambdaTerm>, Box<LambdaTerm>),
   App(Box<LambdaTerm>, Box<LambdaTerm>),
   Abs(String, Type, Box<LambdaTerm>),
   Fst, Snd, // for Couple
   Goal(Type)
}

impl LambdaTerm {
    pub fn intro(self, var: String) -> LambdaTerm {
        remplace_intro(self, var)
    }
    pub fn exact(self, var: String) -> LambdaTerm {
        remplace_exact(self, var)
    }
}

fn remplace_exact(root: LambdaTerm, name: String) -> LambdaTerm {
    match root {
        LambdaTerm::Abs(str, typ1, box LambdaTerm::Goal(typ2)) if typ1 == typ2 => {
            LambdaTerm::Abs(str.clone(), typ1, Box::new(LambdaTerm::Var(str)))
        }
        LambdaTerm::Var(..) | LambdaTerm::Fst | LambdaTerm::Snd | LambdaTerm::Goal(..) => {
            root
        },
        LambdaTerm::Couple(box term1, box term2) => {
            LambdaTerm::Couple(Box::new(remplace_intro(term1, name.clone())), Box::new(remplace_intro(term2, name)))
        }
        LambdaTerm::App(box first, box second) => {
            LambdaTerm::App(Box::new(remplace_intro(first, name.clone())), Box::new(remplace_intro(second, name)))
        }
        LambdaTerm::Abs(str, typ, box lambdaterm) => {
            LambdaTerm::Abs(str, typ, Box::new(remplace_intro(lambdaterm, name)))
        }
    }
}

fn remplace_intro(root: LambdaTerm, name: String) -> LambdaTerm {
    match root {
        LambdaTerm::Goal(Type::Impl(box a, box b)) => {
            LambdaTerm::Abs(name, a, Box::new(LambdaTerm::Goal(b)))
        }
        LambdaTerm::Var(..) | LambdaTerm::Fst | LambdaTerm::Snd | LambdaTerm::Goal(..) => {
            root
        },
        LambdaTerm::Couple(box term1, box term2) => {
            LambdaTerm::Couple(Box::new(remplace_intro(term1, name.clone())), Box::new(remplace_intro(term2, name)))
        }
        LambdaTerm::App(box first, box second) => {
            LambdaTerm::App(Box::new(remplace_intro(first, name.clone())), Box::new(remplace_intro(second, name)))
        }
        LambdaTerm::Abs(str, typ, box lambdaterm) => {
            LambdaTerm::Abs(str, typ, Box::new(remplace_intro(lambdaterm, name)))
        }
    }
}