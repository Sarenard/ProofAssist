use crate::tactics::tactic_trait::Tactic;
use crate::tactics::Tactics;
use crate::terms::Term;
use crate::Judgment;

use std::fmt;

#[derive(Debug)]
pub struct InfTree {
    pub hypo: Vec<InfTree>,
    pub conclusion: Judgment,
    pub tactic: Option<Tactics>,
    pub prouved: bool,
}

impl InfTree {
    fn fmt_with_indent(&self, f: &mut fmt::Formatter<'_>, indent: usize) -> fmt::Result {
        let indent_str = "  ".repeat(indent);
        writeln!(f, "{}Conclusion:\n{}{}", indent_str, indent_str, self.conclusion)?;

        writeln!(f, "{}Tactic: {:?}", indent_str, self.tactic)?;
        writeln!(f, "{}Proved: {}", indent_str, self.prouved)?;

        for (i, hypo) in self.hypo.iter().enumerate() {
            writeln!(f, "{}Hypothesis [{}]:", indent_str, i)?;
            hypo.fmt_with_indent(f, indent + 1)?;
        }
        Ok(())
    }
}

impl fmt::Display for InfTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_with_indent(f, 0)
    }
}

impl InfTree {
    pub fn is_proven(self) -> bool {
        let mut ok = true;
        ok &= self.prouved;
        for tree in self.hypo {
            ok = ok & tree.is_proven()
        }
        ok
    }
    pub fn apply_tactic(&mut self, tactic: Tactics, args: Vec<Term>) {
        tactic.apply(self, args);
    }
}