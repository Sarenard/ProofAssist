use std::fmt;

use crate::terms::Term;

#[derive(Debug, Clone)]
pub struct Context {
    pub content: Vec<(Term, Term)>
}

impl fmt::Display for Context {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Iterate over each (Term, Term) pair and display them
        for (_, (t1, t2)) in self.content.iter().enumerate() {
            writeln!(f, "({} : {})", t1, t2)?;
        }
        Ok(())
    }
}
