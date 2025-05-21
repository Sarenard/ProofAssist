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
    tree.apply_tactic(tactic!(CTX_EXT));
    println!("{}", tree);
    println!("Is proven : {}", tree.is_proven());

    println!("\n=========================================================");
    let mut tree = Judgment::Typing(
        Context {content: vec![]},
        term!(Var("A")), 
        term!(U(1)),
    ).to_tree();
    tree.apply_tactic(tactic!(U_CUMUL));
    println!("{}", tree);
    println!("Is proven : {}", tree.is_proven());

    println!("\n=========================================================");
    let mut tree = Judgment::JudgEq(
        Context {content: vec![]},
        term!(Var("a")), 
        term!(Var("a")), 
        term!(Var("A")),
    ).to_tree();
    tree.apply_tactic(tactic!(JUGEQEQUIV_REFL));
    println!("{}", tree);
    println!("Is proven : {}", tree.is_proven());
}