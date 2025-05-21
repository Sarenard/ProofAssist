use std::fmt;
use crate::terms::Term;
use crate::{inftree::InfTree, judgments::Judgment};

use crate::{tactic, term};

use crate::tactics::tactic_trait::Tactic;

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub enum JUGEQEQUIVTactic {
    JUGEQEQUIV_REFL,
}

impl fmt::Display for JUGEQEQUIVTactic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl Tactic for JUGEQEQUIVTactic {
    fn name(&self) -> &'static str {
        match self {
            JUGEQEQUIVTactic::JUGEQEQUIV_REFL => "JUGEQEQUIV_REFL",
        }
    }

    fn apply(&self, tree: &mut InfTree) {
        match self {
            JUGEQEQUIVTactic::JUGEQEQUIV_REFL => {
                match tree.conclusion.clone() {
                    Judgment::JudgEq(ctx, a, b, c) if a == b => {
                        tree.hypo = vec![
                            Judgment::Typing(ctx, a, c).to_tree(),
                        ];
                        tree.tactic = Some(tactic!(JUGEQEQUIV_REFL));
                        tree.prouved = true;
                    }
                    _ => panic!("JUGEQEQUIVTactic: Cant do that here !")
                }
            }
        }
    }
}