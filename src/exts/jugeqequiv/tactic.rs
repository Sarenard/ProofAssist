use std::fmt;
use crate::terms::Term;
use crate::{inftree::InfTree, judgments::Judgment};

use crate::{tactic, term};

use crate::tactics::tactic_trait::Tactic;

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(unused)]
pub enum JUGEQEQUIVTactic {
    JUGEQEQUIV_REFL,
    JUGEQEQUIV_SYM,
    JUGEQEQUIV_TRANS,
    JUGEQEQUIV_CONV_TERM,
    JUGEQEQUIV_CONV_EQ,
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
            JUGEQEQUIVTactic::JUGEQEQUIV_SYM => "JUGEQEQUIV_SYM",
            JUGEQEQUIVTactic::JUGEQEQUIV_TRANS => "JUGEQEQUIV_TRANS",
            JUGEQEQUIVTactic::JUGEQEQUIV_CONV_TERM => "JUGEQEQUIV_CONV_TERM",
            JUGEQEQUIVTactic::JUGEQEQUIV_CONV_EQ => "JUGEQEQUIV_CONV_EQ",
        }
    }

    fn apply(&self, tree: &mut InfTree, args: Vec<Term>) {
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
            JUGEQEQUIVTactic::JUGEQEQUIV_SYM => {
                match tree.conclusion.clone() {
                    Judgment::JudgEq(ctx, a, b, ty) => {
                        tree.hypo = vec![Judgment::JudgEq(ctx, b, a, ty).to_tree()];
                        tree.tactic = Some(tactic!(JUGEQEQUIV_SYM));
                        tree.prouved = true;
                    }
                    _ => panic!("JUGEQEQUIV_SYM: Cannot apply"),
                }
            }

            JUGEQEQUIVTactic::JUGEQEQUIV_TRANS => {
                match tree.conclusion.clone() {
                    Judgment::JudgEq(ctx, a, c, ty) => {
                        assert_eq!(args.len(), 1);
                        let var = args[0].clone();
                        tree.hypo = vec![
                            Judgment::JudgEq(ctx.clone(), a.clone(), var.clone(), ty.clone()).to_tree(),
                            Judgment::JudgEq(ctx, var, c, ty).to_tree(),
                        ];
                        tree.tactic = Some(tactic!(JUGEQEQUIV_TRANS));
                        tree.prouved = true;
                    }
                    _ => panic!("JUGEQEQUIV_TRANS: Cannot apply"),
                }
            }

            JUGEQEQUIVTactic::JUGEQEQUIV_CONV_TERM => {
                match tree.conclusion.clone() {
                    Judgment::Typing(ctx, a, b) => {
                        assert_eq!(args.len(), 1);
                        let var = args[0].clone();
                        tree.hypo = vec![
                            Judgment::Typing(ctx.clone(), a.clone(), var.clone()).to_tree(),
                            Judgment::JudgEq(ctx, var, b, term!(U(0))).to_tree(), // TODO : handle universes
                        ];
                        tree.tactic = Some(tactic!(JUGEQEQUIV_CONV_TERM));
                        tree.prouved = true;
                    }
                    _ => panic!("JUGEQEQUIV_CONV_TERM: Cannot apply"),
                }
            }

            JUGEQEQUIVTactic::JUGEQEQUIV_CONV_EQ => {
                match tree.conclusion.clone() {
                    Judgment::JudgEq(ctx, a, b, ty) => {
                        assert_ne!(args.len(), 0);
                        let var = args.first().unwrap().clone();
                        tree.hypo = vec![
                            Judgment::JudgEq(ctx.clone(), a.clone(), b.clone(), var.clone()).to_tree(),
                            Judgment::JudgEq(ctx, var, ty, term!(U(0))).to_tree(), // TODO : handle universes
                        ];
                        tree.tactic = Some(tactic!(JUGEQEQUIV_CONV_EQ));
                        tree.prouved = true;
                    }
                    _ => panic!("JUGEQEQUIV_CONV_EQ: Cannot apply"),
                }
            }
        }
    }
}