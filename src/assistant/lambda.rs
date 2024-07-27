use std::collections::HashMap;

use lazy_static::lazy_static;
use std::sync::Mutex;

use crate::assistant::types::Type as Type;

use super::operations::OP;

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
    Goal(Type, usize),
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
        if goal != computed {
            println!("Computed : {:?}", computed);
            println!("Goal : {:?}", goal);
        }
        goal == computed
    }
    pub fn intros(mut self, nb: usize) -> (Vec<String>, LambdaTerm) {
        self = rebuild_tree(self.clone(), &mut 1);
        let mut myvec: Vec<String> = vec![];
        let mut old = LambdaTerm::Var(("placeholder".to_string(), 0));
        let mut new = self;
        while old.clone() != new.clone() {
            old = new.clone();
            let (str, rnew) = new.intro(nb);
            new = rnew.clone();
            myvec.push(str);
        }
        (myvec, new)
    }
    pub fn intro(mut self, mynb: usize) -> (String, LambdaTerm) {
        self = rebuild_tree(self.clone(), &mut 1);
        // we find a non used name
        let nb = update_counter("hyp");
        let concatenated_name = format!("hyp{}", nb);
        (concatenated_name.clone(), aux_intro(self, concatenated_name, mynb))
    }
    pub fn introv(mut self, name: String, nb: usize) -> LambdaTerm {
        self = rebuild_tree(self.clone(), &mut 1);
        update_counter(&name.clone());
        aux_intro(self, name, nb)
    }
    pub fn exact(mut self, name: String, nb: usize) -> LambdaTerm {
        self = rebuild_tree(self.clone(), &mut 1);
        aux_exact(self, name, HashMap::new(), nb)
    }
    pub fn cut(mut self, type_a: Type, nb: usize) -> LambdaTerm {
        self = rebuild_tree(self.clone(), &mut 1);
        aux_cut(self, type_a, nb)
    }
    pub fn apply(mut self, name: String, nb: usize) -> LambdaTerm {
        self = rebuild_tree(self.clone(), &mut 1);
        aux_apply(self, name, HashMap::new(), nb)
    }
    pub fn split(mut self, nb: usize) -> LambdaTerm {
        self = rebuild_tree(self.clone(), &mut 1);
        aux_split(self, HashMap::new(), nb)
    }
    pub fn elim(mut self, name: String, nb: usize) -> LambdaTerm {
        self = rebuild_tree(self.clone(), &mut 1);
        aux_elim(self, name, HashMap::new(), nb)
    }
    pub fn absurd(mut self, statement: Type, nb: usize) -> LambdaTerm {
        self = rebuild_tree(self.clone(), &mut 1);
        aux_absurd(self, statement, HashMap::new(), nb)
    }
}

fn rebuild_tree(term: LambdaTerm, goal_index: &mut usize) -> LambdaTerm {
    match term {
        LambdaTerm::Var((name, nb)) => {
            return LambdaTerm::Var((name, nb));
        }
        LambdaTerm::Couple(box left, box right) => {
            let leftside = rebuild_tree(left, goal_index);
            let rightside = rebuild_tree(right, goal_index);
            return LambdaTerm::Couple(
                Box::new(leftside), 
                Box::new(rightside)
            );
        }
        LambdaTerm::App(box left, box right) => {
            let leftside = rebuild_tree(left, goal_index);
            let rightside: LambdaTerm = rebuild_tree(right, goal_index);
            return LambdaTerm::App(
                Box::new(leftside), 
                Box::new(rightside)
            );
        }
        LambdaTerm::Abs(name, typ, box body) => {
            return LambdaTerm::Abs(
                name,
                typ,
                Box::new(rebuild_tree(body, goal_index))
            );
        }
        LambdaTerm::ExFalso(typ, box body) => {
            return LambdaTerm::ExFalso(
                typ, 
                Box::new(rebuild_tree(body, goal_index))
            );
        }
        LambdaTerm::Goal(typ, _index) => {
            *goal_index += 1;
            return LambdaTerm::Goal(
                typ,
                *goal_index - 1
            )
        }
        LambdaTerm::Fst(name) => {
            return LambdaTerm::Fst(name);
        }
        LambdaTerm::Snd(name) => {
            return LambdaTerm::Snd(name);

        }
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

fn aux_absurd(root: LambdaTerm, statement: Type, context: HashMap<String, Type>, nb: usize) -> LambdaTerm {
    match root {
        LambdaTerm::Goal(_typ, nb2) 
        if nb2 ==  nb => {
            LambdaTerm::ExFalso(
                statement,
                Box::new(LambdaTerm::Goal(Type::Bottom, 0))
            )
        }        // we propagate
        LambdaTerm::ExFalso(t, box proof) => {
            LambdaTerm::ExFalso(t, Box::new(aux_absurd(proof, statement, context, nb)))
        }
        LambdaTerm::Var(..) | LambdaTerm::Fst(..) | LambdaTerm::Snd(..) | LambdaTerm::Goal(..) => {
            root
        },
        LambdaTerm::Couple(box term1, box term2) => {
            LambdaTerm::Couple(
                Box::new(aux_absurd(term1, statement.clone(), context.clone(), nb)), 
                Box::new(aux_absurd(term2, statement, context, nb)), 
            )
        }
        LambdaTerm::App(box first, box second) => {
            LambdaTerm::App(
                Box::new(aux_absurd(first, statement.clone(), context.clone(), nb)), 
                Box::new(aux_absurd(second, statement, context, nb)), 
            )
        }
        LambdaTerm::Abs(str, typ, box lambdaterm) => {
            let mut new_context = context.clone();
            new_context.insert(str.clone(), typ.clone());
            LambdaTerm::Abs(str, typ, Box::new(aux_absurd(lambdaterm, statement, new_context, nb)))
        }
    }
}

fn aux_elim(root: LambdaTerm, name: String, context: HashMap<String, Type>, nb: usize) -> LambdaTerm {
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
        LambdaTerm::Goal(type_c, nb2)
        if type_h != Type::Error && nb == nb2 => {
            // App(App(Goal(a->b->a),Fst(h1)),Snd(h1))
            LambdaTerm::App(
                Box::new(
                    LambdaTerm::App(
                        Box::new(LambdaTerm::Goal(
                            Type::Imp(
                                Box::new(type_a.clone()),
                                Box::new(
                                    Type::Imp(
                                        Box::new(type_b),
                                        Box::new(type_c),
                                    )
                                )
                            ),
                            0
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
            LambdaTerm::ExFalso(t, Box::new(aux_elim(proof, name, context, nb)))
        }
        LambdaTerm::Var(..) | LambdaTerm::Fst(..) | LambdaTerm::Snd(..) | LambdaTerm::Goal(..) => {
            root
        },
        LambdaTerm::Couple(box term1, box term2) => {
            LambdaTerm::Couple(
                Box::new(aux_elim(term1, name.clone(), context.clone(), nb)), 
                Box::new(aux_elim(term2, name, context, nb)), 
            )
        }
        LambdaTerm::App(box first, box second) => {
            LambdaTerm::App(
                Box::new(aux_elim(first, name.clone(), context.clone(), nb)), 
                Box::new(aux_elim(second, name, context, nb)), 
            )
        }
        LambdaTerm::Abs(str, typ, box lambdaterm) => {
            let mut new_context = context.clone();
            new_context.insert(str.clone(), typ.clone());
            LambdaTerm::Abs(str, typ, Box::new(aux_elim(lambdaterm, name, new_context, nb)))
        }
    }
}

fn aux_split(root: LambdaTerm, context: HashMap<String, Type>, nb: usize) -> LambdaTerm {
    match root {
        LambdaTerm::Goal(Type::And(box a, box b), nb2) 
        if nb2 == nb => {
            LambdaTerm::Couple(
                Box::new(LambdaTerm::Goal(a, 0)), 
                Box::new(LambdaTerm::Goal(b, 0))
            )
        }
        // we propagate
        LambdaTerm::ExFalso(t, box proof) => {
            LambdaTerm::ExFalso(t, Box::new(aux_split(proof, context, nb)))
        }
        LambdaTerm::Var(..) | LambdaTerm::Fst(..) | LambdaTerm::Snd(..) | LambdaTerm::Goal(..) => {
            root
        },
        LambdaTerm::Couple(box term1, box term2) => {
            LambdaTerm::Couple(
                Box::new(aux_split(term1, context.clone(), nb)), 
                Box::new(aux_split(term2, context, nb)), 
            )
        }
        LambdaTerm::App(box first, box second) => {
            LambdaTerm::App(
                Box::new(aux_split(first, context.clone(), nb)), 
                Box::new(aux_split(second, context, nb)), 
            )
        }
        LambdaTerm::Abs(str, typ, box lambdaterm) => {
            let mut new_context = context.clone();
            new_context.insert(str.clone(), typ.clone());
            LambdaTerm::Abs(str, typ, Box::new(aux_split(lambdaterm, new_context, nb)))
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
                Type::Imp(box type1, box type2) if type1 == bodytype => {
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
            Type::Imp(
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

fn aux_intro(root: LambdaTerm, name: String, nb: usize) -> LambdaTerm {
    match root {
        LambdaTerm::Goal(Type::Imp(box a, box b), nb2) 
        if nb == nb2
        => {
            LambdaTerm::Abs(name, a, Box::new(LambdaTerm::Goal(b, 0)))
        }
        // we propagate
        LambdaTerm::ExFalso(t, box proof) => {
            LambdaTerm::ExFalso(t, Box::new(aux_intro(proof, name, nb)))
        }
        LambdaTerm::Var(..) | LambdaTerm::Fst(..) | LambdaTerm::Snd(..) | LambdaTerm::Goal(..) => {
            root
        },
        LambdaTerm::Couple(box term1, box term2) => {
            LambdaTerm::Couple(Box::new(aux_intro(term1, name.clone(), nb)), Box::new(aux_intro(term2, name, nb)))
        }
        LambdaTerm::App(box first, box second) => {
            LambdaTerm::App(Box::new(aux_intro(first, name.clone(), nb)),Box::new(aux_intro(second, name, nb)))
        }
        LambdaTerm::Abs(str, typ, box lambdaterm) => {
            LambdaTerm::Abs(str, typ, Box::new(aux_intro(lambdaterm, name, nb)))
        }
    }
}

fn aux_exact(root: LambdaTerm, name: String, context: HashMap<String, Type>, nb: usize) -> LambdaTerm {
    let type_h = context.get(&name).unwrap_or(&Type::Error).clone();
    match root {
        LambdaTerm::Goal(typ, nb2) 
        if typ == type_h && nb2 == nb => {
            let nb = update_counter(&name.clone());
            LambdaTerm::Var((name, nb))
        }
        // we propagate
        LambdaTerm::ExFalso(t, box proof) => {
            LambdaTerm::ExFalso(t, Box::new(aux_exact(proof, name, context, nb)))
        }
        LambdaTerm::Var(..) | LambdaTerm::Fst(..) | LambdaTerm::Snd(..) | LambdaTerm::Goal(..) => {
            root
        },
        LambdaTerm::Couple(box term1, box term2) => {
            LambdaTerm::Couple(
                Box::new(aux_exact(term1, name.clone(), context.clone(), nb)), 
                Box::new(aux_exact(term2, name, context, nb))
            )
        }
        LambdaTerm::App(box first, box second) => {
            LambdaTerm::App(
                Box::new(aux_exact(first, name.clone(), context.clone(), nb)), 
                Box::new(aux_exact(second, name, context, nb))
            )
        }
        LambdaTerm::Abs(str, typ, box lambdaterm) => {
            let mut new_context = context.clone();
            new_context.insert(str.clone(), typ.clone());
            LambdaTerm::Abs(str, typ, Box::new(aux_exact(lambdaterm, name, new_context, nb)))
        }
    }
}

fn aux_cut(root: LambdaTerm, type_a: Type, nb: usize) -> LambdaTerm{
    match root {
        LambdaTerm::Goal(type_b, nb2) 
        if nb == nb2 => {
            LambdaTerm::App(
                Box::new(LambdaTerm::Goal(Type::Imp(Box::new(type_a.clone()), Box::new(type_b)), 0)),
                Box::new(LambdaTerm::Goal(type_a, 0)),
            )
        }
        // we propagate
        LambdaTerm::ExFalso(t, box proof) => {
            LambdaTerm::ExFalso(t, Box::new(aux_cut(proof, type_a, nb)))
        }
        LambdaTerm::Var(..) | LambdaTerm::Fst(..) | LambdaTerm::Snd(..) | LambdaTerm::Goal(..) => {
            root
        },
        LambdaTerm::Couple(box term1, box term2) => {
            LambdaTerm::Couple(Box::new(term1.cut(type_a.clone(), nb)), Box::new(term2.cut(type_a, nb)))
        }
        LambdaTerm::App(box first, box second) => {
            LambdaTerm::App(Box::new(first.cut(type_a.clone(), nb)), Box::new(second.cut(type_a, nb)))
        }
        LambdaTerm::Abs(str, typ, box lambdaterm) => {
            LambdaTerm::Abs(str, typ, Box::new(lambdaterm.cut(type_a, nb)))
        }
    }
}

fn aux_apply(root: LambdaTerm, name: String, context: HashMap<String, Type>, nb: usize) -> LambdaTerm {
    let type_h = context.get(&name).unwrap_or(&Type::Error).clone();
    let mut type_a = Type::Error;
    let mut type_b = Type::Error;
    if type_h != Type::Error {
        match type_h {
            Type::Imp(box typea, box typeb) => {
                type_a = typea.clone();
                type_b = typeb.clone();
            }
            _ => {
                panic!("Impossible...")
            }
        }
    }
    match root {
        LambdaTerm::Goal(typeb, _nb)
        if typeb == type_b => {
            let nb = update_counter(&name.clone());
            LambdaTerm::App(Box::new(LambdaTerm::Var((name, nb))), Box::new(LambdaTerm::Goal(type_a, 0)))
        }
        // we propagate
        LambdaTerm::ExFalso(t, box proof) => {
            LambdaTerm::ExFalso(t, Box::new(aux_apply(proof, name, context, nb)))
        }
        LambdaTerm::Var(..) | LambdaTerm::Fst(..) | LambdaTerm::Snd(..) | LambdaTerm::Goal(..) => {
            root
        },
        LambdaTerm::Couple(box term1, box term2) => {
            LambdaTerm::Couple(
                Box::new(aux_apply(term1, name.clone(), context.clone(), nb)), 
                Box::new(aux_apply(term2, name, context, nb)), 
            )
        }
        LambdaTerm::App(box first, box second) => {
            LambdaTerm::App(
                Box::new(aux_apply(first, name.clone(), context.clone(), nb)), 
                Box::new(aux_apply(second, name, context, nb)), 
            )
        }
        LambdaTerm::Abs(str, typ, box lambdaterm) => {
            let mut new_context = context.clone();
            new_context.insert(str.clone(), typ.clone());
            LambdaTerm::Abs(str, typ, Box::new(aux_apply(lambdaterm, name, new_context, nb)))
        }
    }
}
