pub mod term_trait;
pub mod term;

#[macro_use]
mod r#macro;

pub use term::Term;
pub use term_trait::TermTrait as TermTrait;