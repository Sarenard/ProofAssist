use std::fmt;

use crate::inftree::InfTree;
use crate::Context;

use crate::terms::Term;

#[derive(Debug, Clone)]
pub enum Judgment {
    // Gamma Ctx.
    Ctx(Context),
    // Gamma |- a : A
    Typing(Context, Term, Term),
    // Gamma |- a = a' : A
    #[allow(unused)]
    JudgEq(Context, Term, Term, Term),
}

impl fmt::Display for Judgment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Judgment::Ctx(ctx) => write!(f, "Ctx = ({})", ctx),
            Judgment::Typing(ctx, a, b) => write!(f, "{} |- {} : {}", ctx, a, b),
            Judgment::JudgEq(ctx, a, a_prime, b) => write!(
                f,
                "{} |- {} = {} : {}",
                ctx, a, a_prime, b
            ),
        }
    }
}

impl Judgment {
    pub fn to_tree(self) -> InfTree {
        InfTree {
            hypo: vec![],
            conclusion: self,
            tactic: None,
            prouved: false,
        }
    }
}