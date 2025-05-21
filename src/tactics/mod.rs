pub mod tactic_trait;

#[macro_use]
mod r#macro;

use tactic_trait::Tactic;

use crate::exts::universe::UTactic;
use crate::exts::ctx::CtxTactic;
use crate::exts::jugeqequiv::JUGEQEQUIVTactic;
use crate::terms::Term;

#[derive(Debug)]
pub enum Tactics {
    Ctx(CtxTactic),
    U(UTactic),
    JUGEQEQUIV(JUGEQEQUIVTactic)
}

impl Tactic for Tactics {
    fn name(&self) -> &'static str {
        match self {
            Tactics::Ctx(t) => t.name(),
            Tactics::U(t) => t.name(),
            Tactics::JUGEQEQUIV(t) => t.name(),
        }
    }

    fn apply(&self, tree: &mut crate::InfTree, args: Vec<Term>) {
        match self {
            Tactics::Ctx(t) => t.apply(tree, args),
            Tactics::U(t) => t.apply(tree, args),
            Tactics::JUGEQEQUIV(t) => t.apply(tree, args),
        }
    }
}
