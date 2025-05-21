use std::fmt;
use crate::terms::Term;
use crate::{inftree::InfTree, judgments::Judgment};

use crate::term;

use crate::tactics::tactic_trait::Tactic;

use super::Universe;

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum UTactic {
    U_INTRO,
    U_CUMUL,
}

impl fmt::Display for UTactic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl Tactic for UTactic {
    fn name(&self) -> &'static str {
        match self {
            UTactic::U_INTRO => "U_INTRO",
            UTactic::U_CUMUL => "U_CUMUL",
        }
    }

    fn apply(&self, tree: &mut InfTree, _args: Vec<Term>) {
        match self {
            UTactic::U_INTRO => {
                match tree.conclusion.clone() {
                    Judgment::Typing(ctx, a, b)
                        if matches!((a.clone(), b.clone()), (Term::Universe(Universe(i)), Term::Universe(Universe(j))) if j == i + 1) =>
                    {
                        tree.hypo = vec![
                            Judgment::Ctx(ctx).to_tree()
                        ];
                        tree.tactic = Some(crate::tactics::Tactics::U(self.clone()));
                        tree.prouved = true;
                    }
                    _ => panic!("U_INTRO: Cant do that here !"),
                }
            }
            UTactic::U_CUMUL => {
                match tree.conclusion.clone() {
                    Judgment::Typing(ctx, a, b) => {
                        if let Term::Universe(Universe(i)) = b {
                            if i > 0 {
                                tree.hypo = vec![
                                    Judgment::Typing(ctx, a, term!(U(i - 1))).to_tree()
                                ];
                                tree.tactic = Some(crate::tactics::Tactics::U(self.clone()));
                                tree.prouved = true;
                            } else {
                                panic!("U_CUMUL: i must be > 0");
                            }
                        } else {
                            panic!("U_CUMUL: b is not Term::U");
                        }
                    }
                    _ => panic!("U_CUMUL: Cant do that here !"),
                }
            }
        }
    }
}