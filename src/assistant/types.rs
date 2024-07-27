use std::fmt;

#[derive(Clone, PartialEq, Hash, Eq)]
pub enum Type {
    Var(String),
    Imp(Box<Type>, Box<Type>),
    And(Box<Type>, Box<Type>),
    Not(Box<Type>),
    #[allow(dead_code)]
    Bottom,
    #[allow(dead_code)]
    Top,

    Error,
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Var(s) => {
                let new = s.to_string();
                write!(f, "Var(\"{}\")", new)
            },
            Type::Imp(t1, t2) => {
                if **t2 == Type::Bottom {
                    write!(f, "Not({})", t1)
                } else {
                    write!(f, "Impl({}, {})", t1, t2)
                }
            }
            Type::And(t1, t2) => write!(f, "And({}, {})", t1, t2),
            Type::Bottom => write!(f, "Bottom"),
            Type::Top => write!(f, "Top"),
            Type::Error => write!(f, "Error"),
            Type::Not(_) => {panic!("Unreachable");}
        }
    }
}

impl fmt::Debug for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Var(s) => {
                let new = s.trim_matches(|c| c == '\"').to_string();
                write!(f, "{}", new)
            },
            Type::Imp(t1, t2) => {
                if **t2 == Type::Bottom {
                    write!(f, "~({:?})", t1)
                } else {
                    write!(f, "{:?} -> {:?}", t1, t2)
                }
            }
            Type::And(t1, t2) => write!(f, "{:?} /\\ {:?}", t1, t2),
            Type::Bottom => write!(f, "Bottom"),
            Type::Top => write!(f, "Top"),
            Type::Error => write!(f, "Error"),
            Type::Not(_) => {panic!("Unreachable");}
        }
    }
}

#[allow(dead_code)]
impl Type {
    pub fn var(str: &str) -> Type {
        Type::Var(str.to_string())
    }
    pub fn imp(type1: Type, type2: Type) -> Type {
        Type::Imp(Box::new(type1), Box::new(type2))
    }
    pub fn and(type1: Type, type2: Type) -> Type {
        Type::And(Box::new(type1), Box::new(type2))
    }
    pub fn not(type1: Type) -> Type {
        Type::Not(Box::new(type1))
    }
}

impl Type {
    pub fn removenot(self) -> Type {
        match self {
            Type::Not(box rest) => {
                Type::Imp(Box::new(rest.removenot()), Box::new(Type::Bottom))
            }
            Type::Bottom | Type::Top | Type::Var(..) => {
                self
            }
            Type::And(box a, box b) => {
                Type::And(Box::new(a.removenot()), Box::new(b.removenot()))
            }
            Type::Imp(box a, box b) => {
                Type::Imp(Box::new(a.removenot()), Box::new(b.removenot()))
            }
            Type::Error => {
                panic!("Unreachable !")
            }
        }
    }
}