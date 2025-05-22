use std::fmt;
use crate::exts::universe::Universe;
use crate::terms::{Term, TermTrait};
use crate::{inftree::InfTree, judgments::Judgment};

use crate::{tactic, term};

use super::{Apply, Lambda, Pi};

use crate::tactics::tactic_trait::Tactic;

// TODO : add PI_COMP
// TODO : add PI_UNIQ
// and the == counterpart for the constructor
// TODO : add PI_INTRO_EQ
#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum PiTactic {
    PI_FORM,
    PI_INTRO,
    PI_ELIM,
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
            PiTactic::PI_INTRO => "PI_INTRO",
            PiTactic::PI_ELIM => "PI_ELIM",
        }
    }

    fn apply(&self, tree: &mut InfTree, args: Vec<Term>) {
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
            PiTactic::PI_INTRO => {
                match tree.conclusion.clone() {
                    Judgment::Typing(
                        ctx, 
                        Term::Lambda(Lambda(box x1, box a1, box b1)),
                        Term::Pi(Pi(box x2, box a2, box b2)), 
                    ) if x1 == x2 && a1 == a2=> {
                        tree.hypo = vec![
                            Judgment::Typing(ctx.add_term((x1, a1)), b1, b2).to_tree(),
                        ];
                        tree.tactic = Some(tactic!(PI_INTRO));
                        tree.prouved = true;
                    }
                    _ => panic!("PI_INTRO: Cant do that here !"),
                }
            }
            PiTactic::PI_ELIM => {
                match tree.conclusion.clone() {
                    Judgment::Typing(
                        ctx, 
                        Term::Apply(Apply(box f, box a)), 
                        B
                    ) => {
                        assert_eq!(args.len(), 2);
                        let x = args[0].clone();
                        let A = args[1].clone();
                        tree.hypo = vec![
                            Judgment::Typing(
                                ctx.clone(), 
                                a.clone(), 
                                A.clone()
                            ).to_tree(),
                            Judgment::Typing(
                                ctx.clone(), 
                                f, 
                                term!(Pi(
                                    x.clone(),
                                    A,
                                    B.replace(a, x) // TODO : check if works
                                ))
                            ).to_tree(),
                        ];
                        tree.tactic = Some(tactic!(PI_ELIM));
                        tree.prouved = true;
                    }
                    _ => panic!("PI_ELIM: Cant do that here !"),
                }
            }
        }
    }
}