#[derive(Debug, Clone, PartialEq)]
pub enum OP {
    // for using theorems
    Use(String),
    Load(String),
    
    // general
    Intro,
    Split,
    Elim(String),
    Apply(String),
    Exists(String),
    Rewrite(String),
    Left,
    Inversion(String),
    Right,
    Refl,
    Rec,
    
    // QOL macros for other things
    Assumption,
    Intros,

    Nothing
}