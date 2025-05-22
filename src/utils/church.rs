use crate::{term, terms::Term};

pub fn church(n: usize) -> Term {
    match n {
        0 => term!(NZero),
        k => term!(NSucc(church(k-1)))
    }
}