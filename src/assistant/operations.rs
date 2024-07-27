use crate::assistant::types::Type;

#[derive(Debug, Clone, PartialEq)]
pub enum OP {
    Use(String), // for using theorems
    
    Assumption(usize), 
    Intro(usize),
    Introv(String, usize),
    Intros(usize),
    Split(usize),
    Exact(String, usize),
    #[allow(dead_code)]
    Cut(Type),
    #[allow(dead_code)]
    Absurd(Type),
    Apply(String, usize),
    Elim(String, usize),
    Load(String),
    Add,
    Sub,
}