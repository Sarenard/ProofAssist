#![feature(box_patterns)]
#![allow(non_snake_case)]

mod judgments;
mod context;
mod inftree;
mod tactics;
mod terms;

mod utils;
use utils::{add, church, double};

mod exts;

mod tests;

use judgments::Judgment;
use context::Context;
use inftree::InfTree;
use terms::Term;

fn main() {
    /*
    println!("\n=========================================================");
    let mut tree = Judgment::Ctx(Context {content: vec![
        (term!(Var("x1")), term!(Var("A1")))
    ]}).to_tree();
    apply_tactic!(tree, CTX_EXT);
    println!("{}", tree);
    println!("Is proven : {}", tree.is_proven());

    println!("\n=========================================================");
    let mut tree = Judgment::Typing(
        Context {content: vec![]},
        term!(Var("A")), 
        term!(U(1)),
    ).to_tree();
    apply_tactic!(tree, U_CUMUL);
    println!("{}", tree);
    println!("Is proven : {}", tree.is_proven());

    println!("\n=========================================================");
    let mut tree = Judgment::JudgEq(
        Context {content: vec![]},
        term!(Var("a")), 
        term!(Var("a")), 
        term!(Var("A")),
    ).to_tree();
    apply_tactic!(tree, JUGEQEQUIV_REFL);
    println!("{}", tree);
    println!("Is proven : {}", tree.is_proven());

    println!("\n=========================================================");
    let mut tree = Judgment::Typing(
        Context {content: vec![]},
        term!(Pi(term!(Var("x")), term!(Var("A")), term!(Var("B")))),
        term!(U(1)),
    ).to_tree();
    apply_tactic!(tree, PI_FORM);
    println!("{}", tree);
    println!("Is proven : {}", tree.is_proven());

    println!("\n=========================================================");
    let mut tree = Judgment::Typing(
        Context {content: vec![]},
        term!(NZero),
        term!(Nat),
    ).to_tree();
    apply_tactic!(tree, NINTRO1);
    apply_tactic!(tree.hypo[0], CTX_EMP);
    println!("{}", tree);
    println!("Is proven : {}", tree.is_proven());

    println!("\n=========================================================");
    let mut tree = Judgment::Typing(
        Context {content: vec![]},
        term!(Nat),
        term!(U(0)),
    ).to_tree();
    apply_tactic!(tree, NFORM);
    apply_tactic!(tree.hypo[0], CTX_EMP);
    println!("{}", tree);
    println!("Is proven : {}", tree.is_proven());

    println!("\n=========================================================");
    let mut tree = Judgment::Typing(
        Context {content: vec![]},
        term!(NSucc(term!(NZero))),
        term!(Nat),
    ).to_tree();
    apply_tactic!(tree, NINTRO2);
    apply_tactic!(tree.hypo[0], NINTRO1);
    apply_tactic!(tree.hypo[0].hypo[0], CTX_EMP);
    println!("{}", tree);
    println!("Is proven : {}", tree.is_proven());
    */

    /*
    println!("\n=========================================================");
    let mut tree = Judgment::JudgEq(
        Context {content: vec![]},
        add(church(0)),
        term!(Lambda(
            term!(Var("VAR1")),
            term!(Nat),
            term!(Var("VAR1"))
        )),
        term!(Pi(
            term!(Var("_")), 
            term!(Nat), 
            term!(Nat)
        ))
    ).to_tree();
    println!("{}", tree);
    apply_tactic!(tree, NCOMP1, vec![term!(Var("VAR2")), term!(Var("FUNC1"))]);
    apply_tactic!(tree.hypo[0], PI_FORM);
    apply_tactic!(tree.hypo[0].hypo[0], NFORM);
    apply_tactic!(tree.hypo[0].hypo[0].hypo[0], CTX_EXT);
    apply_tactic!(tree.hypo[0].hypo[0].hypo[0].hypo[0], NFORM);
    apply_tactic!(tree.hypo[0].hypo[0].hypo[0].hypo[0].hypo[0], CTX_EMP);
    apply_tactic!(tree.hypo[0].hypo[1], NFORM);
    apply_tactic!(tree.hypo[0].hypo[1].hypo[0], CTX_EXT);
    apply_tactic!(tree.hypo[0].hypo[1].hypo[0].hypo[0], NFORM);
    apply_tactic!(tree.hypo[0].hypo[1].hypo[0].hypo[0].hypo[0], CTX_EXT);
    apply_tactic!(tree.hypo[0].hypo[1].hypo[0].hypo[0].hypo[0].hypo[0], NFORM);
    apply_tactic!(tree.hypo[0].hypo[1].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0], CTX_EMP);
    apply_tactic!(tree.hypo[1], PI_INTRO);
    apply_tactic!(tree.hypo[1].hypo[0], VBLE);
    apply_tactic!(tree.hypo[1].hypo[0].hypo[0], CTX_EXT);
    apply_tactic!(tree.hypo[1].hypo[0].hypo[0].hypo[0], NFORM);
    apply_tactic!(tree.hypo[1].hypo[0].hypo[0].hypo[0].hypo[0], CTX_EMP);
    apply_tactic!(tree.hypo[2], PI_INTRO);
    apply_tactic!(tree.hypo[2].hypo[0], NINTRO2);
    apply_tactic!(tree.hypo[2].hypo[0].hypo[0], PI_ELIM, vec![term!(Var("N1")), term!(Nat)]);
    apply_tactic!(tree.hypo[2].hypo[0].hypo[0].hypo[0], VBLE);
    apply_tactic!(tree.hypo[2].hypo[0].hypo[0].hypo[0].hypo[0], CTX_EXT);
    apply_tactic!(tree.hypo[2].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0], NFORM);
    apply_tactic!(tree.hypo[2].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0], CTX_EXT);
    apply_tactic!(tree.hypo[2].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0], PI_FORM);
    apply_tactic!(tree.hypo[2].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0], NFORM);
    apply_tactic!(tree.hypo[2].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0], CTX_EXT);
    apply_tactic!(tree.hypo[2].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0], NFORM);
    apply_tactic!(tree.hypo[2].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0], CTX_EMP);
    apply_tactic!(tree.hypo[2].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0].hypo[1], NFORM);
    apply_tactic!(tree.hypo[2].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0].hypo[1].hypo[0], CTX_EXT);
    apply_tactic!(tree.hypo[2].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0].hypo[1].hypo[0].hypo[0], NFORM);
    apply_tactic!(tree.hypo[2].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0].hypo[1].hypo[0].hypo[0].hypo[0], CTX_EXT);
    apply_tactic!(tree.hypo[2].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0].hypo[1].hypo[0].hypo[0].hypo[0].hypo[0], NFORM);
    apply_tactic!(tree.hypo[2].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0].hypo[1].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0], CTX_EMP);
    apply_tactic!(tree.hypo[2].hypo[0].hypo[0].hypo[1], VBLE);
    apply_tactic!(tree.hypo[2].hypo[0].hypo[0].hypo[1].hypo[0], CTX_EXT);
    apply_tactic!(tree.hypo[2].hypo[0].hypo[0].hypo[1].hypo[0].hypo[0], NFORM);
    apply_tactic!(tree.hypo[2].hypo[0].hypo[0].hypo[1].hypo[0].hypo[0].hypo[0], CTX_EXT);
    apply_tactic!(tree.hypo[2].hypo[0].hypo[0].hypo[1].hypo[0].hypo[0].hypo[0].hypo[0], PI_FORM);
    apply_tactic!(tree.hypo[2].hypo[0].hypo[0].hypo[1].hypo[0].hypo[0].hypo[0].hypo[0].hypo[1], NFORM);
    apply_tactic!(tree.hypo[2].hypo[0].hypo[0].hypo[1].hypo[0].hypo[0].hypo[0].hypo[0].hypo[1].hypo[0], CTX_EXT);
    apply_tactic!(tree.hypo[2].hypo[0].hypo[0].hypo[1].hypo[0].hypo[0].hypo[0].hypo[0].hypo[1].hypo[0].hypo[0], NFORM);
    apply_tactic!(tree.hypo[2].hypo[0].hypo[0].hypo[1].hypo[0].hypo[0].hypo[0].hypo[0].hypo[1].hypo[0].hypo[0].hypo[0], CTX_EXT);
    apply_tactic!(tree.hypo[2].hypo[0].hypo[0].hypo[1].hypo[0].hypo[0].hypo[0].hypo[0].hypo[1].hypo[0].hypo[0].hypo[0].hypo[0], NFORM);
    apply_tactic!(tree.hypo[2].hypo[0].hypo[0].hypo[1].hypo[0].hypo[0].hypo[0].hypo[0].hypo[1].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0], CTX_EMP);
    apply_tactic!(tree.hypo[2].hypo[0].hypo[0].hypo[1].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0], NFORM);
    apply_tactic!(tree.hypo[2].hypo[0].hypo[0].hypo[1].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0], CTX_EXT);
    apply_tactic!(tree.hypo[2].hypo[0].hypo[0].hypo[1].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0], NFORM);
    apply_tactic!(tree.hypo[2].hypo[0].hypo[0].hypo[1].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0], CTX_EMP);
    */

    println!("\n=========================================================");
    let mut tree = Judgment::JudgEq(
        Context {content: vec![]},
        add(church(1)),
        term!(Lambda(
            term!(Var("VAR3")),
            term!(Nat),
            term!(NSucc(term!(Var("VAR3"))))
        )),
        term!(Pi(
            term!(Var("_")), 
            term!(Nat), 
            term!(Nat)
        ))
    ).to_tree();
    tree.apply_tactic(tactic!(NCOMP2), vec![term!(Var("VAR2")), term!(Var("FUNC1"))]);
    println!("{}", tree);

    println!("{}", tree);
    println!("Is proven : {}", tree.is_proven());
}