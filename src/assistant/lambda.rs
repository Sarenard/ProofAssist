use std::collections::HashMap;
use std::sync::Mutex;

use crate::assistant::types::Type as Type;

use lazy_static::lazy_static;

lazy_static! {
    static ref HYP: Mutex<HashMap<String, Type>> = Mutex::new(HashMap::new());
}

#[derive(Debug, Clone)]
pub enum LambdaTerm {
    #[allow(dead_code)]
    Var(String),
    #[allow(dead_code)]
    Couple(Box<LambdaTerm>, Box<LambdaTerm>),
    App(Box<LambdaTerm>, Box<LambdaTerm>),
    #[allow(dead_code)]
    Abs(String, Type, Box<LambdaTerm>),
    #[allow(dead_code)]
    Fst, // for Couple
    #[allow(dead_code)]
    Snd, // for Couple
    Goal(Type)
}

impl LambdaTerm {
    pub fn containsgoal(self) -> bool {
        let mut found = false;
        match self {
            LambdaTerm::Goal(..) => {
                found = true
            }
            LambdaTerm::Var(..) | LambdaTerm::Fst | LambdaTerm::Snd => {},
            LambdaTerm::Couple(box term1, box term2) => {
                found |= term1.containsgoal();
                found |= term2.containsgoal();
            }
            LambdaTerm::App(box first, box second) => {
                found |= first.containsgoal();
                found |= second.containsgoal();
            }
            LambdaTerm::Abs(_str, _typ, box lambdaterm) => {
                found |= lambdaterm.containsgoal();
            }
        }
        found
    }
    pub fn check(self) -> bool {
        false
    }
    pub fn intro(self, name: String) -> LambdaTerm {
        aux_intro(self, name)
    }
    pub fn exact(self, name: String) -> LambdaTerm {
        aux_exact(self, name)
    }
    pub fn cut(self, type_a: Type) -> LambdaTerm {
        aux_cut(self, type_a)
    }
    pub fn apply(self, name: String) -> LambdaTerm {
        aux_apply(self, name)
    }
}

fn aux_intro(root: LambdaTerm, name: String) -> LambdaTerm {
    match root {
        LambdaTerm::Goal(Type::Impl(box a, box b)) => {
            let mut hashmap = HYP.lock().unwrap();
            hashmap.insert(name.clone(), a.clone());
            LambdaTerm::Abs(name, a, Box::new(LambdaTerm::Goal(b)))
        }
        // we propagate
        LambdaTerm::Var(..) | LambdaTerm::Fst | LambdaTerm::Snd | LambdaTerm::Goal(..) => {
            root
        },
        LambdaTerm::Couple(box term1, box term2) => {
            LambdaTerm::Couple(Box::new(aux_intro(term1, name.clone())), Box::new(aux_intro(term2, name)))
        }
        LambdaTerm::App(box first, box second) => {
            LambdaTerm::App(Box::new(aux_intro(first, name.clone())),Box::new(aux_intro(second, name)))
        }
        LambdaTerm::Abs(str, typ, box lambdaterm) => {
            LambdaTerm::Abs(str, typ, Box::new(aux_intro(lambdaterm, name)))
        }
    }
}

fn aux_exact(root: LambdaTerm, name: String) -> LambdaTerm {
    let hashmap = HYP.lock().unwrap();
    let type_h = hashmap.get(&name).unwrap().clone();
    drop(hashmap);
    match root {
        LambdaTerm::Goal(typ) if typ == type_h => {
            LambdaTerm::Var(name)
        }
        // we propagate
        LambdaTerm::Var(..) | LambdaTerm::Fst | LambdaTerm::Snd | LambdaTerm::Goal(..) => {
            root
        },
        LambdaTerm::Couple(box term1, box term2) => {
            LambdaTerm::Couple(Box::new(aux_exact(term1, name.clone())), Box::new(aux_exact(term2, name)))
        }
        LambdaTerm::App(box first, box second) => {
            LambdaTerm::App(Box::new(aux_exact(first, name.clone())), Box::new(aux_exact(second, name)))
        }
        LambdaTerm::Abs(str, typ, box lambdaterm) => {
            LambdaTerm::Abs(str, typ, Box::new(aux_exact(lambdaterm, name)))
        }
    }
}

fn aux_cut(root: LambdaTerm, type_a: Type) -> LambdaTerm{
    match root {
        LambdaTerm::Goal(type_b) => {
            LambdaTerm::App(
                Box::new(LambdaTerm::Goal(Type::Impl(Box::new(type_a.clone()), Box::new(type_b)))),
                Box::new(LambdaTerm::Goal(type_a)),
            )
        }
        // we propagate
        LambdaTerm::Var(..) | LambdaTerm::Fst | LambdaTerm::Snd => {
            root
        },
        LambdaTerm::Couple(box term1, box term2) => {
            LambdaTerm::Couple(Box::new(term1.cut(type_a.clone())), Box::new(term2.cut(type_a)))
        }
        LambdaTerm::App(box first, box second) => {
            LambdaTerm::App(Box::new(first.cut(type_a.clone())), Box::new(second.cut(type_a)))
        }
        LambdaTerm::Abs(str, typ, box lambdaterm) => {
            LambdaTerm::Abs(str, typ, Box::new(lambdaterm.cut(type_a)))
        }
    }
}

fn aux_apply(root: LambdaTerm, name: String) -> LambdaTerm {
    let hashmap = HYP.lock().unwrap();
    let type_h = hashmap.get(&name).unwrap();
    let type_a: Type;
    let type_b: Type;
    match type_h {
        Type::Impl(box typea, box typeb) => {
            type_a = typea.clone();
            type_b = typeb.clone();
        }
        _ => {
            panic!("Impossible...")
        }
    }
    drop(hashmap);
    match root {
        LambdaTerm::Goal(typeb)
        if typeb == type_b => {
            LambdaTerm::App(Box::new(LambdaTerm::Var(name)), Box::new(LambdaTerm::Goal(type_a)))
        }
        // we propagate
        LambdaTerm::Var(..) | LambdaTerm::Fst | LambdaTerm::Snd | LambdaTerm::Goal(..) => {
            root
        },
        LambdaTerm::Couple(box term1, box term2) => {
            LambdaTerm::Couple(Box::new(term1.apply(name.clone())), Box::new(term2.apply(name)))
        }
        LambdaTerm::App(box first, box second) => {
            LambdaTerm::App(Box::new(first.apply(name.clone())), Box::new(second.apply(name)))
        }
        LambdaTerm::Abs(str, typ, box lambdaterm) => {
            LambdaTerm::Abs(str, typ, Box::new(lambdaterm.apply(name)))
        }
    }
}
