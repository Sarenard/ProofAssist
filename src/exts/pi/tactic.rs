use std::fmt;
use crate::exts::universe::Universe;
use crate::terms::Term;
use crate::{inftree::InfTree, judgments::Judgment};

use crate::{tactic, term};

use super::Pi;

use crate::tactics::tactic_trait::Tactic;

// TODO : add :
// PI_INTRO
// PI_ELIM
// PI_COMP
// PI_UNIQ
// and the == counterpart for the constructor
// PI_INTRO_EQ
#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum PiTactic {
    PI_FORM,
}

impl fmt::Display for PiTactic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl Tactic for PiTactic {
    fn name(&self) -> &'static str {
        match self {
            PiTactic::PI_FORM => "PI_FORM",
        }
    }

    fn apply(&self, tree: &mut InfTree, _args: Vec<Term>) {
        match self {
            PiTactic::PI_FORM => {
                match tree.conclusion.clone() {
                    Judgment::Typing(
                        ctx, 
                        Term::Pi(Pi(box x, box a, box b)), 
                        Term::Universe(Universe(i)),
                    ) => {
                        tree.hypo = vec![
                            Judgment::Typing(ctx.clone(), a.clone(), term!(U(i))).to_tree(),
                            Judgment::Typing(ctx.add_term((x, a)), b, term!(U(i))).to_tree(),
                        ];
                        tree.tactic = Some(tactic!(PI_FORM));
                        tree.prouved = true;
                    }
                    _ => panic!("PI_FORM: Cant do that here !"),
                }
            }
        }
    }
}