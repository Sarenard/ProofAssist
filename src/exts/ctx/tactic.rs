use std::fmt;
use crate::terms::Term;
use crate::{context::Context, inftree::InfTree, judgments::Judgment};

use crate::term;

use crate::tactics::tactic_trait::Tactic;

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum CtxTactic {
    CTX_EMP,
    CTX_EXT,
}

impl fmt::Display for CtxTactic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl Tactic for CtxTactic {
    fn name(&self) -> &'static str {
        match self {
            CtxTactic::CTX_EMP => "CTX_EMP",
            CtxTactic::CTX_EXT => "CTX_EXT",
        }
    }
    
    fn apply(&self, tree: &mut InfTree, _args: Vec<Term>) {
        match self {
            CtxTactic::CTX_EMP => {
                match tree.conclusion.clone() {
                    Judgment::Ctx(a) if a.content.is_empty() => {
                        tree.hypo = vec![];
                        tree.tactic = Some(crate::tactics::Tactics::Ctx(self.clone()));
                        tree.prouved = true;
                    }
                    _ => panic!("CTX_EMP: Cant do that here !"),
                }
            }
            CtxTactic::CTX_EXT => {
                match tree.conclusion.clone() {
                    Judgment::Ctx(a) if !a.content.is_empty() => {
                        let last = a.content.last().unwrap();
                        tree.hypo = vec![
                            Judgment::Typing(
                                Context { content: vec![] },
                                last.1.clone(),
                                term!(U(0)),
                            )
                            .to_tree(),
                        ];
                        tree.tactic = Some(crate::tactics::Tactics::Ctx(self.clone()));
                        tree.prouved = true;
                    }
                    _ => panic!("CTX_EXT: Cant do that here !"),
                }
            }
        }
    }
}