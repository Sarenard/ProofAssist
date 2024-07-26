use std::collections::HashMap;

use crate::assistant::types::Type as Type;

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
    pub fn check(self, goal: Type) -> bool {
        fn compute_type(lambdaterm: LambdaTerm, mytypes: HashMap<String, Type>) -> Type {
            match lambdaterm {
                LambdaTerm::App(box first, box second) => {
                    let var_name = match first {LambdaTerm::Var(name) => name, _ => panic!("Error !")};
                    let res = mytypes.get(&var_name).unwrap().clone();
                    let typea = compute_type(second, mytypes);
                    let globala: Type;
                    let globalb: Type;
                    match res {
                        Type::Impl(box a, box b) => {
                            globala = a;
                            globalb = b;
                        }
                        _ => panic!("Error !")
                    }
                    if typea == globala {
                        globalb
                    } else {
                        panic!("Error !")
                    }
                }
                LambdaTerm::Var(name) => {
                    let res = mytypes.get(&name).unwrap().clone();
                    res
                }
                LambdaTerm::Abs(name, typ, box other) => {
                    let mut newtypes = mytypes.clone();
                    newtypes.insert(name, typ.clone());
                    Type::Impl(
                        Box::new(typ), 
                        Box::new(compute_type(other, newtypes))
                    )
                }
                LambdaTerm::Goal(..) => {
                    panic!("Not supposed to happend !")
                }
                LambdaTerm::Fst | LambdaTerm::Snd | LambdaTerm::Couple(..) => {
                    todo!()
                }
            }
        }
        let computed = compute_type(self, HashMap::new());
        goal == computed
    }
    pub fn intro(self, name: String) -> LambdaTerm {
        aux_intro(self, name)
    }
    pub fn exact(self, name: String) -> LambdaTerm {
        aux_exact(self, name, HashMap::new())
    }
    pub fn cut(self, type_a: Type) -> LambdaTerm {
        aux_cut(self, type_a)
    }
    pub fn apply(self, name: String) -> LambdaTerm {
        aux_apply(self, name, HashMap::new())
    }
}

fn aux_intro(root: LambdaTerm, name: String) -> LambdaTerm {
    match root {
        LambdaTerm::Goal(Type::Impl(box a, box b)) => {
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

fn aux_exact(root: LambdaTerm, name: String, context: HashMap<String, Type>) -> LambdaTerm {
    let type_h = context.get(&name).unwrap_or(&Type::Error).clone();
    match root {
        LambdaTerm::Goal(typ) if typ == type_h => {
            LambdaTerm::Var(name)
        }
        // we propagate
        LambdaTerm::Var(..) | LambdaTerm::Fst | LambdaTerm::Snd | LambdaTerm::Goal(..) => {
            root
        },
        LambdaTerm::Couple(box term1, box term2) => {
            LambdaTerm::Couple(
                Box::new(aux_exact(term1, name.clone(), context.clone())), 
                Box::new(aux_exact(term2, name, context))
            )
        }
        LambdaTerm::App(box first, box second) => {
            LambdaTerm::App(
                Box::new(aux_exact(first, name.clone(), context.clone())), 
                Box::new(aux_exact(second, name, context))
            )
        }
        LambdaTerm::Abs(str, typ, box lambdaterm) => {
            let mut new_context = context.clone();
            new_context.insert(str.clone(), typ.clone());
            LambdaTerm::Abs(str, typ, Box::new(aux_exact(lambdaterm, name, new_context)))
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

fn aux_apply(root: LambdaTerm, name: String, context: HashMap<String, Type>) -> LambdaTerm {
    let type_h = context.get(&name).unwrap_or(&Type::Error).clone();
    let mut type_a = Type::Error;
    let mut type_b = Type::Error;
    if type_h != Type::Error {
        match type_h {
            Type::Impl(box typea, box typeb) => {
                type_a = typea.clone();
                type_b = typeb.clone();
            }
            _ => {
                panic!("Impossible...")
            }
        }
    }
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
            LambdaTerm::Couple(
                Box::new(aux_apply(term1, name.clone(), context.clone())), 
                Box::new(aux_apply(term2, name, context)), 
            )
        }
        LambdaTerm::App(box first, box second) => {
            LambdaTerm::App(
                Box::new(aux_apply(first, name.clone(), context.clone())), 
                Box::new(aux_apply(second, name, context)), 
            )
        }
        LambdaTerm::Abs(str, typ, box lambdaterm) => {
            let mut new_context = context.clone();
            new_context.insert(str.clone(), typ.clone());
            LambdaTerm::Abs(str, typ, Box::new(aux_apply(lambdaterm, name, new_context)))
        }
    }
}
