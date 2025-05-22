use std::fmt;
use crate::exts::universe::Universe;
use crate::terms::{Term, TermTrait};
use crate::{inftree::InfTree, judgments::Judgment};

use crate::{tactic, term};

use crate::tactics::tactic_trait::Tactic;

use super::{Ind0, Zero};

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(unused)]
// TODO : add ZERO_ELIM
pub enum ZeroTactic {
    ZERO_FORM,
    ZERO_ELIM,
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
            ZeroTactic::ZERO_ELIM => "ZERO_ELIM",
        }
    }

    fn apply(&self, tree: &mut InfTree, args: Vec<Term>) {
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
            ZeroTactic::ZERO_ELIM => {
                match tree.conclusion.clone() {
                    Judgment::Typing(
                        ctx,
                        Term::Ind0(Ind0(box C1, box a)),
                        C2,
                    ) if C1 == C2 => {
                        assert_eq!(args.len(), 1);
                        let x = args[0].clone();
                        tree.hypo = vec![
                            Judgment::Typing(
                                ctx.clone().add_term((x.clone(), term!(Zero))), 
                                C1.replace(a.clone(), x),
                                term!(U(0)) // TODO : handle universes
                            ).to_tree(),
                            Judgment::Typing(ctx, a, term!(Zero)).to_tree(),
                        ];
                        tree.tactic = Some(tactic!(ZERO_ELIM));
                        tree.prouved = true;
                    }
                    _ => panic!("ZERO_ELIM: Cant do that here !"),
                }
            }
        }
    }
}