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
    pub fn containsgoal(self) -> bool {
        let mut found = false;
        match self {
            LambdaTerm::Goal(..) => {
                found = true
            }
            LambdaTerm::Var(..) | LambdaTerm::Fst | LambdaTerm::Snd => {},
            LambdaTerm::Couple(box term1, box term2) => {
                found |= term1.containsgoal();
                found |= term2.containsgoal();
            }
            LambdaTerm::App(box first, box second) => {
                found |= first.containsgoal();
                found |= second.containsgoal();
            }
            LambdaTerm::Abs(str, typ, box lambdaterm) => {
                found |= lambdaterm.containsgoal();
            }
        }
        found
    }
    pub fn intro(self, name: String) -> LambdaTerm {
        match self {
            LambdaTerm::Goal(Type::Impl(box a, box b)) => {
                LambdaTerm::Abs(name, a, Box::new(LambdaTerm::Goal(b)))
            }
            LambdaTerm::Var(..) | LambdaTerm::Fst | LambdaTerm::Snd | LambdaTerm::Goal(..) => {
                self
            },
            LambdaTerm::Couple(box term1, box term2) => {
                LambdaTerm::Couple(Box::new(term1.intro(name.clone())), Box::new(term2.intro(name)))
            }
            LambdaTerm::App(box first, box second) => {
                LambdaTerm::App(Box::new(first.intro(name.clone())),Box::new(second.intro(name)))
            }
            LambdaTerm::Abs(str, typ, box lambdaterm) => {
                LambdaTerm::Abs(str, typ, Box::new(lambdaterm.intro(name)))
            }
        }
    }
    pub fn exact(self, name: String) -> LambdaTerm {
        match self {
            LambdaTerm::Abs(str, typ1, box LambdaTerm::Goal(typ2)) if typ1 == typ2 => {
                LambdaTerm::Abs(str.clone(), typ1, Box::new(LambdaTerm::Var(str)))
            }
            LambdaTerm::Var(..) | LambdaTerm::Fst | LambdaTerm::Snd | LambdaTerm::Goal(..) => {
                self
            },
            LambdaTerm::Couple(box term1, box term2) => {
                LambdaTerm::Couple(Box::new(term1.exact(name.clone())), Box::new(term2.exact(name)))
            }
            LambdaTerm::App(box first, box second) => {
                LambdaTerm::App(Box::new(first.exact(name.clone())), Box::new(second.exact(name)))
            }
            LambdaTerm::Abs(str, typ, box lambdaterm) => {
                LambdaTerm::Abs(str, typ, Box::new(lambdaterm.exact(name)))
            }
        }
    }
}
