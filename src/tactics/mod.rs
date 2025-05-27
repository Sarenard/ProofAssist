pub mod tactic_trait;

#[macro_use]
mod r#macro;

use tactic_trait::Tactic;

use crate::exts::nat::NatTactic;
use crate::exts::pi::PiTactic;
use crate::exts::universe::UTactic;
use crate::exts::ctx::CtxTactic;
use crate::exts::jugeqequiv::JUGEQEQUIVTactic;
use crate::exts::var::VarTactic;
use crate::exts::zero::ZeroTactic;
use crate::terms::Term;

#[derive(Debug, PartialEq, Clone)]
pub enum Tactics {
    Ctx(CtxTactic),
    U(UTactic),
    JUGEQEQUIV(JUGEQEQUIVTactic),
    PI(PiTactic),
    ZERO(ZeroTactic),
    NAT(NatTactic),
    VAR(VarTactic),
}

impl Tactic for Tactics {
    fn name(&self) -> &'static str {
        match self {
            Tactics::Ctx(t) => t.name(),
            Tactics::U(t) => t.name(),
            Tactics::JUGEQEQUIV(t) => t.name(),
            Tactics::PI(t) => t.name(),
            Tactics::ZERO(t) => t.name(),
            Tactics::NAT(t) => t.name(),
            Tactics::VAR(t) => t.name(),
        }
    }

    fn apply(&self, tree: &mut crate::InfTree, args: Vec<Term>) {
        match self {
            Tactics::Ctx(t) => t.apply(tree, args),
            Tactics::U(t) => t.apply(tree, args),
            Tactics::JUGEQEQUIV(t) => t.apply(tree, args),
            Tactics::PI(t) => t.apply(tree, args),
            Tactics::ZERO(t) => t.apply(tree, args),
            Tactics::NAT(t) => t.apply(tree, args),
            Tactics::VAR(t) => t.apply(tree, args),
        }
    }
}
