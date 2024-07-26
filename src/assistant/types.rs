use std::fmt;

#[derive(Clone, PartialEq)]
pub enum Type {
    Var(String),
    Impl(Box<Type>, Box<Type>),
    And(Box<Type>, Box<Type>),
    Not(Box<Type>),
    #[allow(dead_code)]
    Bottom,
    #[allow(dead_code)]
    Top,

    Error,
}


impl fmt::Debug for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Var(s) => write!(f, "Var({:?})", s),
            Type::Impl(t1, t2) => {
                if **t2 == Type::Bottom {
                    write!(f, "Not({:?})", t1)
                } else {
                    write!(f, "Impl({:?}, {:?})", t1, t2)
                }
            }
            Type::And(t1, t2) => write!(f, "And({:?}, {:?})", t1, t2),
            Type::Not(_) => {panic!("WTF");}
            Type::Bottom => write!(f, "Bottom"),
            Type::Top => write!(f, "Top"),
            Type::Error => write!(f, "Error"),
        }
    }
}

impl Type {
    pub fn removenot(self) -> Type {
        match self {
            Type::Not(box rest) => {
                Type::Impl(Box::new(rest.removenot()), Box::new(Type::Bottom))
            }
            Type::Bottom | Type::Top | Type::Var(..) => {
                self
            }
            Type::And(box a, box b) => {
                Type::And(Box::new(a.removenot()), Box::new(b.removenot()))
            }
            Type::Impl(box a, box b) => {
                Type::Impl(Box::new(a.removenot()), Box::new(b.removenot()))
            }
            Type::Error => {
                panic!("Unreachable !")
            }
        }
    }
}