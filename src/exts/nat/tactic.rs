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
                                ctx.clone().add_term(
                                    (x.clone(), term!(Nat))
                                ), 
                                C1.clone(), 
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
                            box C1,
                            box c0_0,
                            box cs,
                            box Term::NZero(NZero)
                        )), 
                        c0_1,
                        C2
                    ) if C1.clone().replace(x.clone(), term!(NZero)) == C2.clone() 
                        && c0_0 == c0_1 => {
                        tree.hypo = vec![
                            Judgment::Typing(
                                ctx.clone().add_term((x.clone(), term!(Nat))), 
                                C1.clone(), 
                                term!(U(0)) // todo : handle
                            ).to_tree(),
                            Judgment::Typing(
                                ctx.clone(), 
                                c0_0.clone(), 
                                C2.clone() // ou alors C1[0/x], c'est pareil :D
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
                            box C0,
                            box c0,
                            box cs_0,
                            box Term::NSucc(NSucc(box n))
                        )),
                        cs_1,
                        C1,
                    ) if C1 == C0.clone().replace(x.clone(), term!(NSucc(n.clone())))
                    && cs_1 == cs_0.clone().replace(
                        x.clone(),
                        n.clone()
                    ).replace(
                        y.clone(), 
                    term!(IndN(
                        C0.clone(),
                        c0.clone(),
                        cs_0.clone(),
                        n.clone()
                    ))) => {
                        tree.hypo = vec![
                            Judgment::Typing(
                                ctx.clone().add_term((x.clone(), term!(Nat))), 
                                C0.clone(),
                                term!(U(0)) // TODO : HANDLE THIS
                            ).to_tree(),
                            Judgment::Typing(
                                ctx.clone(), 
                                c0,
                                C0.clone().replace(x.clone(), term!(NZero))
                            ).to_tree(),
                            Judgment::Typing(
                                ctx.clone().add_term(
                                    (x.clone(), term!(Nat))
                                ).add_term(
                                    (y.clone(), C0.clone())
                                ),
                                cs_0.clone(),
                                C0.clone().replace(x.clone(), term!(NSucc(x))),
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
                    Judgment::JudgEq(
                        ctx, 
                        Term::IndN(IndN(
                            box C0,
                            box c0,
                            box cs_0,
                            box Term::NSucc(NSucc(box n))
                        )),
                        cs_1,
                        C1,
                    ) => {
                        println!("{}", C1 == C0.clone().replace(x.clone(), term!(NSucc(n.clone()))));
                        println!("{}", cs_1 == cs_0.clone().replace(
                            x.clone(),
                            n.clone()
                        ).replace(
                            y.clone(), 
                        term!(IndN(
                            C0.clone(),
                            c0.clone(),
                            cs_0.clone(),
                            n.clone()
                        ))));
                        println!("====");
                        println!("{}", cs_0);
                        println!("{}", cs_0.clone().replace(
                            x.clone(),
                            n.clone()
                        ).replace(
                            y.clone(), 
                            term!(IndN(
                                C0.clone(),
                                c0.clone(),
                                cs_0.clone(),
                                n.clone()
                        ))));
                        println!("{}", cs_1);
                        panic!("NCOMP2: KDLJFLKJDSF")
                    }
                    _ => panic!("NCOMP2: Cant do that here !")
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