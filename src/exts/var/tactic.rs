use std::fmt;
use crate::terms::Term;
use crate::{inftree::InfTree, judgments::Judgment};

use crate::tactic;

use crate::tactics::tactic_trait::Tactic;

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(unused)]
pub enum VarTactic {
    VBLE,
}

impl fmt::Display for VarTactic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl Tactic for VarTactic {
    fn name(&self) -> &'static str {
        match self {
            VarTactic::VBLE => "VBLE",
        }
    }

    fn apply(&self, tree: &mut InfTree, _args: Vec<Term>) {
        match self {
            VarTactic::VBLE => {
                match tree.conclusion.clone() {
                    Judgment::Typing(ctx, xi, Ai) => {
                        let mut found = false;
                        for (name, typ) in &ctx.content {
                            // TODO : remove this debug
                            // println!("{} == {} : {}", typ, &Ai, typ == &Ai);
                            if name == &xi && typ == &Ai {
                                found = true;
                                break;
                            }
                        }

                        if !found {
                            panic!("VBLE: ({} : {}) not found in context", xi, Ai);
                        }

                        tree.hypo = vec![
                            Judgment::Ctx(ctx).to_tree()
                        ];
                        tree.tactic = Some(tactic!(VBLE));
                        tree.prouved = true;
                    }
                    _ => panic!("U_INTRO: Cant do that here !"),
                }
            }
        }
    }
}