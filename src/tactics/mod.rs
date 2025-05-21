// tactics/mod.rs
pub mod tactic_trait;
pub mod ctx;
pub mod universe;

#[macro_use]
mod r#macro;

pub use ctx::CtxTactic;
use tactic_trait::Tactic;
pub use universe::UTactic;

#[derive(Debug)]
pub enum Tactics {
    Ctx(CtxTactic),
    U(UTactic),
}

impl Tactic for Tactics {
    fn name(&self) -> &'static str {
        match self {
            Tactics::Ctx(t) => t.name(),
            Tactics::U(t) => t.name(),
        }
    }

    fn apply(&self, tree: &mut crate::InfTree) {
        match self {
            Tactics::Ctx(t) => t.apply(tree),
            Tactics::U(t) => t.apply(tree),
        }
    }
}
