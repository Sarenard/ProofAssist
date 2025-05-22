use std::fmt;
use crate::exts::pi::Lambda;
use crate::exts::universe::Universe;
use crate::terms::{Term, TermTrait};
use crate::{inftree::InfTree, judgments::Judgment};

use crate::{tactic, term};

use crate::tactics::tactic_trait::Tactic;

use super::{IndN, NSucc, NZero, Nat};

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(unused)]
pub enum NatTactic {
    NFORM,
    NINTRO1,
    NINTRO2,
    NELIM,
    NCOMP1,
    NCOMP2,
    NINTRO2_EQ,
}

impl fmt::Display for NatTactic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl Tactic for NatTactic {
    fn name(&self) -> &'static str {
        match self {
            NatTactic::NFORM => "NFORM",
            NatTactic::NINTRO1 => "NINTRO1",
            NatTactic::NINTRO2 => "NINTRO2",
            NatTactic::NELIM => "NELIM",
            NatTactic::NCOMP1 => "NCOMP1",
            NatTactic::NCOMP2 => "NCOMP2",
            NatTactic::NINTRO2_EQ => "NINTRO2_EQ",
        }
    }

    fn apply(&self, tree: &mut InfTree, args: Vec<Term>) {
        match self {
            NatTactic::NFORM => {
                match tree.conclusion.clone() {
                    Judgment::Typing(ctx, Term::Nat(Nat), Term::Universe(Universe(_)))=> {
                        tree.hypo = vec![
                            Judgment::Ctx(ctx).to_tree()
                        ];
                        tree.tactic = Some(tactic!(NFORM));
                        tree.prouved = true;
                    }
                    _ => panic!("NFORM: Cant do that here !"),
                }
            }
            NatTactic::NINTRO1 => {
                match tree.conclusion.clone() {
                    Judgment::Typing(ctx, Term::NZero(NZero), Term::Nat(Nat))=> {
                        tree.hypo = vec![
                            Judgment::Ctx(ctx).to_tree()
                        ];
                        tree.tactic = Some(tactic!(NINTRO1));
                        tree.prouved = true;
                    }
                    _ => panic!("NINTRO1: Cant do that here !"),
                }
            }
            NatTactic::NINTRO2 => {
                match tree.conclusion.clone() {
                    Judgment::Typing(ctx, Term::NSucc(NSucc(box n)), Term::Nat(Nat))=> {
                        tree.hypo = vec![
                            Judgment::Typing(ctx, n, term!(Nat)).to_tree()
                        ];
                        tree.tactic = Some(tactic!(NINTRO2));
                        tree.prouved = true;
                    }
                    _ => panic!("NINTRO1: Cant do that here !"),
                }
            }
            NatTactic::NELIM => {
                assert_eq!(args.len(), 2);
                let x = args[0].clone();
                let y = args[1].clone();
                match tree.conclusion.clone() { // TODO : FIX BINDINGS
                    Judgment::Typing(
                        ctx, 
                        Term::IndN(IndN(
                            box C1,
                            box c0,
                            box cs,
                            box n
                        )), 
                        C2
                    ) if C1.clone().replace(x.clone(), n.clone()) == C2 => {
                        tree.hypo = vec![
                            Judgment::Typing(
                                ctx.clone().add_term((x.clone(), term!(Nat))), 
                                C1.clone().replace(x.clone(), n.clone()), 
                                term!(U(0)) // todo : handle
                            ).to_tree(),
                            Judgment::Typing(
                                ctx.clone(), 
                                c0.clone(), 
                                C1.clone().replace(x.clone(), term!(NZero))
                            ).to_tree(),
                            Judgment::Typing(
                                ctx.clone().add_term(
                                    (x.clone(), term!(Nat))
                                ).add_term(
                                    (y, C1.clone())
                                ),
                                cs,
                                C1.clone().replace(x.clone(), term!(NSucc(x.clone())))
                            ).to_tree(),
                            Judgment::Typing(
                                ctx, 
                                n, 
                                term!(Nat)
                            ).to_tree(),
                        ];
                        tree.tactic = Some(tactic!(NELIM));
                        tree.prouved = true;
                    }
                    _ => panic!("NELIM: Cant do that here !"),
                }
            }
            NatTactic::NCOMP1 => {
                assert_eq!(args.len(), 2);
                let x = args[0].clone();
                let y = args[1].clone();
                match tree.conclusion.clone() {
                    Judgment::JudgEq(
                        ctx, 
                        Term::IndN(IndN(
                            box Term::Lambda(Lambda(
                                box x1,
                                box _,
                                box C_1,
                            )),
                            box c0_2,
                            box Term::Lambda(Lambda(
                                box x2,
                                box _,
                                box Term::Lambda(Lambda(
                                    box y1,
                                    box _,
                                    box cs,
                                )),
                            )),
                            box Term::NZero(NZero)
                        )), 
                        c0_1, 
                        C_2
                    ) if C_1.clone().replace(x.clone(), term!(NZero)) == C_2.clone() && c0_1 == c0_2 
                         && x1 == x.clone() && x2 == x.clone() && y1 == y.clone() => {
                        tree.hypo = vec![
                            Judgment::Typing(
                                ctx.clone().add_term((x.clone(), term!(Nat))), 
                                C_1.clone(), 
                                term!(U(0)) // todo : handle
                            ).to_tree(),
                            Judgment::Typing(
                                ctx.clone(), 
                                c0_1.clone(), 
                                C_2.clone()
                            ).to_tree(),
                            Judgment::Typing(
                                ctx.clone().add_term(
                                    (x.clone(), term!(Nat))
                                ).add_term(
                                    (y, C_2.clone())
                                ),
                                cs,
                                C_2.clone().replace(x.clone(), term!(NSucc(x.clone())))
                            ).to_tree(),
                        ];
                        tree.tactic = Some(tactic!(NCOMP1));
                        tree.prouved = true;
                    }
                    _ => panic!("NCOMP1: Cant do that here !"),
                }
            }
            NatTactic::NCOMP2 => {
                assert_eq!(args.len(), 2);
                let x = args[0].clone();
                let y = args[1].clone();
                match tree.conclusion.clone() {
                    Judgment::JudgEq(
                        ctx, 
                        Term::IndN(IndN(
                            box Term::Lambda(Lambda(
                                box x1,
                                box xtype1,
                                box C_1,
                            )),
                            box c_0,
                            box Term::Lambda(Lambda(
                                box x2,
                                box xtype2,
                                box Term::Lambda(Lambda(
                                    box y1,
                                    box ytype,
                                    box cs,
                                )),
                            )),
                            box Term::NSucc(NSucc(box n))
                        )),
                        cs_edited,
                        C_2,
                    ) if x1 == x && x2 == x && y1 == y && C_1.clone() == C_2.clone().replace(term!(NSucc(n.clone())), x.clone())
                    && xtype1 == xtype2 && cs_edited == cs.clone().replace(x.clone(), n.clone()).replace(
                        y.clone(),
                        term!(IndN(
                            term!(Lambda(
                                x.clone(),
                                xtype1.clone(),
                                C_1.clone()
                            )), 
                            c_0.clone(), 
                            term!(Lambda(
                                x.clone(),
                                xtype1.clone(),
                                term!(Lambda(
                                    y.clone(),
                                    ytype.clone(),
                                    cs.clone()
                                ))
                            )),
                            n.clone()
                        )),
                    ) => {
                        tree.hypo = vec![
                            Judgment::Typing(
                                ctx.clone().add_term((x.clone(), term!(Nat))), 
                                C_1.clone(),
                                term!(U(0))
                            ).to_tree(),
                            Judgment::Typing(
                                ctx.clone(), 
                                c_0,
                                C_1.clone().replace(x.clone(), term!(NZero))
                            ).to_tree(),
                            Judgment::Typing(
                                ctx.clone().add_term((x.clone(), term!(Nat))).add_term((y.clone(), C_1.clone())),
                                cs.clone(),
                                C_1.clone().replace(x.clone(), term!(NSucc(x))),
                            ).to_tree(),
                            Judgment::Typing(
                                ctx.clone(), 
                                n.clone(), 
                                term!(Nat)
                            ).to_tree(),
                        ];
                        tree.tactic = Some(tactic!(NCOMP2));
                        tree.prouved = true;
                    }
                    _ => panic!("NCOMP2: Cant do that here !"),
                }
            }
            NatTactic::NINTRO2_EQ => {
                match tree.conclusion.clone() {
                    Judgment::JudgEq(
                        ctx, 
                        Term::NSucc(NSucc(box n)),
                        Term::NSucc(NSucc(box m)),
                        Term::Nat(Nat)
                    ) => {
                        tree.hypo = vec![
                            Judgment::JudgEq(
                                ctx, 
                                n,
                                m,
                                term!(Nat)
                            ).to_tree(),
                        ];
                        tree.tactic = Some(tactic!(NINTRO2_EQ));
                        tree.prouved = true;
                    }
                    _ => panic!("NINTRO2_EQ: Cant do that here !"),
                }
            }
        }
    }
}