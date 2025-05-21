use std::fmt;

use crate::inftree::InfTree;

pub trait Tactic: fmt::Debug {
    fn name(&self) -> &'static str;
    fn apply(&self, tree: &mut InfTree);
}
