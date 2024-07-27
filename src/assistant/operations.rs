use crate::assistant::types::Type;

#[derive(Debug, Clone, PartialEq)]
pub enum OP {
    // for using theorems
    Use(String),
    Load(String),
    
    // general
    Assumption, 
    Intro,
    Introv(String),
    Intros,
    Split,
    Exact(String),
    #[allow(dead_code)]
    Cut(Type),
    #[allow(dead_code)]
    Absurd(Type),
    Apply(String),
    Elim(String),
    Left,
    Right,

    Nothing
}