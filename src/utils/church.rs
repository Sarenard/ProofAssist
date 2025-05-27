use std::sync::atomic::Ordering;

use crate::{term, terms::Term, VAR_COUNTER};

pub fn church(n: usize) -> Term {
    match n {
        0 => term!(NZero),
        k => term!(NSucc(church(k-1)))
    }
}

#[allow(dead_code)]
pub fn double(term: Term) -> Term {
    term!(IndN(
        term!(Nat),
        term!(NZero),
        term!(NSucc(term!(NSucc(term!(Var("DOUBLE")))))),
        term
    ))
}

pub fn add(term: Term) -> Term {
    let id1 = VAR_COUNTER.fetch_add(1, Ordering::Relaxed);
    let id2 = VAR_COUNTER.fetch_add(1, Ordering::Relaxed);
    term!(IndN(
        term!(Pi(
            term!(Var("_")),
            term!(Nat),
            term!(Nat)
        )),
        term!(Lambda(
            term!(Var(format!("__{}", id1))),
            term!(Nat),
            term!(Var(format!("__{}", id1)))
        )),
        term!(Lambda(
            term!(Var(format!("__{}", id2))),
            term!(Nat),
            term!(NSucc(term!(Apply(
                term!(Var("ADD")),
                term!(Var(format!("__{}", id2)))
            ))))
        )),
        
        term
    ))
}