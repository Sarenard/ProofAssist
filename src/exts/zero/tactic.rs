use std::fmt;
use crate::exts::universe::Universe;
use crate::terms::Term;
use crate::{inftree::InfTree, judgments::Judgment};

use crate::tactic;

use crate::tactics::tactic_trait::Tactic;

use super::Zero;

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(unused)]
// TODO : add ZERO_ELIM
pub enum ZeroTactic {
    ZERO_FORM,
}

impl fmt::Display for ZeroTactic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl Tactic for ZeroTactic {
    fn name(&self) -> &'static str {
        match self {
            ZeroTactic::ZERO_FORM => "ZERO_FORM",
        }
    }

    fn apply(&self, tree: &mut InfTree, _args: Vec<Term>) {
        match self {
            ZeroTactic::ZERO_FORM => {
                match tree.conclusion.clone() {
                    Judgment::Typing(
                        ctx, 
                        Term::Zero(Zero),
                        Term::Universe(Universe(_)),
                    ) => {
                        tree.hypo = vec![
                            Judgment::Ctx(ctx).to_tree(),
                        ];
                        tree.tactic = Some(tactic!(ZERO_FORM));
                        tree.prouved = true;
                    }
                    _ => panic!("PI_FORM: Cant do that here !"),
                }
            }
        }
    }
}