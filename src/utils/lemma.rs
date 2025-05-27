use crate::inftree::InfTree;

#[macro_export]
macro_rules! apply_lemma {
    ($to_prove:expr, $proof:expr) => {
        apply_lemma(&mut $to_prove, &$proof)
    };
}

pub fn apply_lemma(to_prove: &mut InfTree, proof: &InfTree) {
    if proof.clone().is_proven() && to_prove.conclusion == proof.conclusion {
        *to_prove = proof.clone();
    } else {
        panic!("Eh, can't use this lemma here");
    }
}