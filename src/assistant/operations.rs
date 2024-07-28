#[derive(Debug, Clone, PartialEq)]
pub enum OP {
    // for using theorems
    Use(String),
    Load(String),
    
    // general
    Intro,

    Nothing
}