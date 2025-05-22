#![feature(box_patterns)]
#![allow(non_snake_case)]

mod judgments;
mod context;
mod inftree;
mod tactics;
mod terms;

mod exts;

mod tests;

use judgments::Judgment;
use context::Context;
use inftree::InfTree;

fn main() {
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
}