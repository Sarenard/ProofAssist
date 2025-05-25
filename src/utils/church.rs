use crate::{term, terms::Term};

pub fn church(n: usize) -> Term {
    match n {
        0 => term!(NZero),
        k => term!(NSucc(church(k-1)))
    }
}

pub fn double(term: Term) -> Term {
    term!(IndN(
        term!(Nat),
        term!(NZero),
        term!(NSucc(term!(NSucc(term!(Var("y")))))),
        term
    ))
}