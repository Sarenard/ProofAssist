mod judgments;
mod context;
mod inftree;
mod tactics;
mod terms;

use judgments::Judgment;
use context::Context;
use inftree::InfTree;

fn main() {
    println!("=========================================================");
    let mut tree = Judgment::Ctx(Context {content: vec![]}).to_tree();
    tree.apply_tactic(tactic!(CTX_EMP));
    println!("{}", tree);
    println!("Is proven : {}", tree.is_proven());

    println!("=========================================================");
    let mut tree = Judgment::Ctx(Context {content: vec![
        (term!(Var("x1")), term!(Var("A1")))
    ]}).to_tree();
    tree.apply_tactic(tactic!(CTX_EXT));
    println!("{}", tree);
    println!("Is proven : {}", tree.is_proven());

    println!("=========================================================");
    let mut tree = Judgment::Typing(
        Context {content: vec![]}, 
        term!(U(0)), 
        term!(U(1)),
    ).to_tree();
    tree.apply_tactic(tactic!(U_INTRO));
    tree.hypo[0].apply_tactic(tactic!(CTX_EMP));
    println!("{}", tree);
    println!("Is proven : {}", tree.is_proven());

    println!("=========================================================");
    let mut tree = Judgment::Typing(
        Context {content: vec![]},
        term!(Var("A")), 
        term!(U(1)),
    ).to_tree();
    tree.apply_tactic(tactic!(U_CUMUL));
    println!("{}", tree);
    println!("Is proven : {}", tree.is_proven());
}