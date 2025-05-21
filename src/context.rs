use std::fmt;

use crate::terms::Term;

#[derive(Debug, Clone, PartialEq)]
pub struct Context {
    pub content: Vec<(Term, Term)>
}

impl Context {
    pub fn add_term(self, term: (Term, Term)) -> Context {
        let mut new = self.clone();
        new.content.push(term);
        new
    }
}

impl fmt::Display for Context {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Iterate over each (Term, Term) pair and display them
        for (_, (t1, t2)) in self.content.iter().enumerate() {
            write!(f, "({} : {})", t1, t2)?;
        }
        Ok(())
    }
}
