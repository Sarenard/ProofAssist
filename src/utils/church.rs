use crate::{term, terms::Term};

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
        term!(NSucc(term!(NSucc(term!(Var("y")))))),
        term
    ))
}

pub fn add(term: Term) -> Term {
    term!(IndN(
        term!(Pi(
            term!(Var("_")),
            term!(Nat),
            term!(Nat)
        )),
        term!(Lambda(
            term!(Var("VAR1")),
            term!(Nat),
            term!(Var("VAR1"))
        )),
        term!(Lambda(
            term!(Var("VAR2")),
            term!(Nat),
            term!(NSucc(term!(Apply(
                term!(Var("FUNC1")),
                term!(Var("VAR2"))
            ))))
        )),
        
        term
    ))
}