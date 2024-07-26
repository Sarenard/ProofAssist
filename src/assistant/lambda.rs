use std::collections::HashMap;

use lazy_static::lazy_static;
use std::sync::Mutex;

use crate::assistant::types::Type as Type;

lazy_static! {
    static ref HASHMAP: Mutex<HashMap<String, usize>> = Mutex::new(HashMap::new());
}

#[derive(Debug, Clone, PartialEq)]
pub enum LambdaTerm {
    Var((String, usize)),
    #[allow(dead_code)]
    Couple(Box<LambdaTerm>, Box<LambdaTerm>),
    App(Box<LambdaTerm>, Box<LambdaTerm>),
    Abs(String, Type, Box<LambdaTerm>),
    #[allow(dead_code)]
    Fst(String), // for Couple
    #[allow(dead_code)]
    Snd(String), // for Couple
    Goal(Type),
    ExFalso(Type, Box<LambdaTerm>)
}

#[allow(dead_code)]
impl LambdaTerm {
    pub fn containsgoal(self) -> bool {
        let mut found = false;
        match self {
            LambdaTerm::Goal(..) => {
                found = true
            }
            LambdaTerm::Var(..) | LambdaTerm::Fst(..) | LambdaTerm::Snd(..) => {},
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
            LambdaTerm::ExFalso(_t, box proof) => {
                found |= proof.containsgoal();
            }
        }
        found
    }
    pub fn check(self, goal: Type) -> bool {
        let computed = compute_type(self, HashMap::new());
        println!("Computed : {:?}", computed);
        println!("Goal : {:?}", goal);
        goal == computed
    }
    pub fn intros(self) -> (Vec<String>, LambdaTerm) {
        let mut myvec: Vec<String> = vec![];
        let mut old = LambdaTerm::Var(("placeholder".to_string(), 0));
        let mut new = self;
        while old.clone() != new.clone() {
            old = new.clone();
            let (str, rnew) = new.intro();
            new = rnew.clone();
            myvec.push(str);
        }
        (myvec, new)
    }
    pub fn intro(self) -> (String, LambdaTerm) {
        // we find a non used name
        let nb = update_counter("hyp");
        let concatenated_name = format!("hyp{}", nb);
        (concatenated_name.clone(), aux_intro(self, concatenated_name))
    }
    pub fn introv(self, name: String) -> LambdaTerm {
        update_counter(&name.clone());
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
    pub fn split(self) -> LambdaTerm {
        aux_split(self, HashMap::new())
    }
    pub fn elim(self, name: String) -> LambdaTerm {
        aux_elim(self, name, HashMap::new())
    }
    pub fn absurd(self, statement: Type) -> LambdaTerm {
        aux_absurd(self, statement, HashMap::new())
    }
}

fn update_counter(key: &str) -> usize {
    let mut map = HASHMAP.lock().unwrap();
    let counter = map.entry(key.to_string()).or_insert(0);
    *counter += 1;
    let cpt = counter.clone();
    drop(map);
    cpt
}

fn aux_absurd(root: LambdaTerm, statement: Type, context: HashMap<String, Type>) -> LambdaTerm {
    match root {
        LambdaTerm::Goal(_) => {
            LambdaTerm::ExFalso(
                statement,
                Box::new(LambdaTerm::Goal(Type::Bottom))
            )
        }
        // we propagate
        LambdaTerm::ExFalso(t, box proof) => {
            LambdaTerm::ExFalso(t, Box::new(aux_absurd(proof, statement, context)))
        }
        LambdaTerm::Var(..) | LambdaTerm::Fst(..) | LambdaTerm::Snd(..) => {
            root
        },
        LambdaTerm::Couple(box term1, box term2) => {
            LambdaTerm::Couple(
                Box::new(aux_absurd(term1, statement.clone(), context.clone())), 
                Box::new(aux_absurd(term2, statement, context)), 
            )
        }
        LambdaTerm::App(box first, box second) => {
            LambdaTerm::App(
                Box::new(aux_absurd(first, statement.clone(), context.clone())), 
                Box::new(aux_absurd(second, statement, context)), 
            )
        }
        LambdaTerm::Abs(str, typ, box lambdaterm) => {
            let mut new_context = context.clone();
            new_context.insert(str.clone(), typ.clone());
            LambdaTerm::Abs(str, typ, Box::new(aux_absurd(lambdaterm, statement, new_context)))
        }
    }
}

fn aux_elim(root: LambdaTerm, name: String, context: HashMap<String, Type>) -> LambdaTerm {
    let type_h = context.get(&name).unwrap_or(&Type::Error).clone();
    let mut type_a = Type::Error;
    let mut type_b = Type::Error;
    if type_h != Type::Error {
        match type_h.clone() {
            Type::And(box typea, box typeb) => {
                type_a = typea.clone();
                type_b = typeb.clone();
            }
            _ => {
                panic!("Impossible...")
            }
        }
    }
    match root {
        LambdaTerm::Goal(type_c)
        if type_h != Type::Error => {
            // App(App(Goal(a->b->a),Fst(h1)),Snd(h1))
            LambdaTerm::App(
                Box::new(
                    LambdaTerm::App(
                        Box::new(LambdaTerm::Goal(
                            Type::Impl(
                                Box::new(type_a.clone()),
                                Box::new(
                                    Type::Impl(
                                        Box::new(type_b),
                                        Box::new(type_c),
                                    )
                                )
                            )
                        )),
                        Box::new(
                            LambdaTerm::Fst(name.clone())
                        ),
                    )
                ),
                Box::new(
                    LambdaTerm::Snd(name)
                )
            )
        }
        // we propagate
        LambdaTerm::ExFalso(t, box proof) => {
            LambdaTerm::ExFalso(t, Box::new(aux_elim(proof, name, context)))
        }
        LambdaTerm::Var(..) | LambdaTerm::Fst(..) | LambdaTerm::Snd(..) | LambdaTerm::Goal(..) => {
            root
        },
        LambdaTerm::Couple(box term1, box term2) => {
            LambdaTerm::Couple(
                Box::new(aux_elim(term1, name.clone(), context.clone())), 
                Box::new(aux_elim(term2, name, context)), 
            )
        }
        LambdaTerm::App(box first, box second) => {
            LambdaTerm::App(
                Box::new(aux_elim(first, name.clone(), context.clone())), 
                Box::new(aux_elim(second, name, context)), 
            )
        }
        LambdaTerm::Abs(str, typ, box lambdaterm) => {
            let mut new_context = context.clone();
            new_context.insert(str.clone(), typ.clone());
            LambdaTerm::Abs(str, typ, Box::new(aux_elim(lambdaterm, name, new_context)))
        }
    }
}

fn aux_split(root: LambdaTerm, context: HashMap<String, Type>) -> LambdaTerm {
    match root {
        LambdaTerm::Goal(Type::And(box a, box b))=> {
            LambdaTerm::Couple(
                Box::new(LambdaTerm::Goal(a)), 
                Box::new(LambdaTerm::Goal(b))
            )
        }
        // we propagate
        LambdaTerm::ExFalso(t, box proof) => {
            LambdaTerm::ExFalso(t, Box::new(aux_split(proof, context)))
        }
        LambdaTerm::Var(..) | LambdaTerm::Fst(..) | LambdaTerm::Snd(..) | LambdaTerm::Goal(..) => {
            root
        },
        LambdaTerm::Couple(box term1, box term2) => {
            LambdaTerm::Couple(
                Box::new(aux_split(term1, context.clone())), 
                Box::new(aux_split(term2, context)), 
            )
        }
        LambdaTerm::App(box first, box second) => {
            LambdaTerm::App(
                Box::new(aux_split(first, context.clone())), 
                Box::new(aux_split(second, context)), 
            )
        }
        LambdaTerm::Abs(str, typ, box lambdaterm) => {
            let mut new_context = context.clone();
            new_context.insert(str.clone(), typ.clone());
            LambdaTerm::Abs(str, typ, Box::new(aux_split(lambdaterm, new_context)))
        }
    }
}

fn compute_type(lambdaterm: LambdaTerm, mytypes: HashMap<String, Type>) -> Type {
    match lambdaterm {
        LambdaTerm::App(box LambdaTerm::Abs(name, wanted_type, box mybody), box second) => {
            let mut newtypes = mytypes.clone();
            newtypes.insert(name, wanted_type.clone());
            let input_type = compute_type(second, newtypes.clone()).clone();
            if input_type != wanted_type {
                panic!("Impossible ! {:?} {:?}", input_type, wanted_type);
            }
            compute_type(mybody, newtypes)
        }
        LambdaTerm::ExFalso(t, box proof) => {
            let ret = compute_type(proof, mytypes);
            match ret {
                Type::Bottom => {return t;}
                _ => panic!("Unreachable")
            }
        }
        LambdaTerm::App(box func, box body) => {
            let functype = compute_type(func, mytypes.clone());
            let bodytype = compute_type(body, mytypes);
            match functype {
                Type::Impl(box type1, box type2) if type1 == bodytype => {
                    return type2
                }
                other => panic!("Error, unknown : {:?}", other)
            }
        }
        LambdaTerm::Var((name, _nb)) => {
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
        LambdaTerm::Couple(box first, box second) => {
            Type::And(
                Box::new(compute_type(first, mytypes.clone())), 
                Box::new(compute_type(second, mytypes)),
            )
        }
        LambdaTerm::Fst(name) => {
            let mytype = mytypes.get(&name).unwrap().clone();
            match mytype {
                Type::And(box a, box _) => {
                    return a;
                }
                other => panic!("Error, unknown : {:?}", other)
            }
        }
        LambdaTerm::Snd(name) => {
            let mytype = mytypes.get(&name).unwrap().clone();
            match mytype {
                Type::And(box _, box b) => {
                    return b;
                }
                other => panic!("Error, unknown : {:?}", other)
            }
        }
    }
}

fn aux_intro(root: LambdaTerm, name: String) -> LambdaTerm {
    match root {
        LambdaTerm::Goal(Type::Impl(box a, box b)) => {
            LambdaTerm::Abs(name, a, Box::new(LambdaTerm::Goal(b)))
        }
        // we propagate
        LambdaTerm::ExFalso(t, box proof) => {
            LambdaTerm::ExFalso(t, Box::new(aux_intro(proof, name)))
        }
        LambdaTerm::Var(..) | LambdaTerm::Fst(..) | LambdaTerm::Snd(..) | LambdaTerm::Goal(..) => {
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
            let nb = update_counter(&name.clone());
            LambdaTerm::Var((name, nb))
        }
        // we propagate
        LambdaTerm::ExFalso(t, box proof) => {
            LambdaTerm::ExFalso(t, Box::new(aux_exact(proof, name, context)))
        }
        LambdaTerm::Var(..) | LambdaTerm::Fst(..) | LambdaTerm::Snd(..) | LambdaTerm::Goal(..) => {
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
        LambdaTerm::ExFalso(t, box proof) => {
            LambdaTerm::ExFalso(t, Box::new(aux_cut(proof, type_a)))
        }
        LambdaTerm::Var(..) | LambdaTerm::Fst(..) | LambdaTerm::Snd(..) => {
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
            let nb = update_counter(&name.clone());
            LambdaTerm::App(Box::new(LambdaTerm::Var((name, nb))), Box::new(LambdaTerm::Goal(type_a)))
        }
        // we propagate
        LambdaTerm::ExFalso(t, box proof) => {
            LambdaTerm::ExFalso(t, Box::new(aux_apply(proof, name, context)))
        }
        LambdaTerm::Var(..) | LambdaTerm::Fst(..) | LambdaTerm::Snd(..) | LambdaTerm::Goal(..) => {
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
