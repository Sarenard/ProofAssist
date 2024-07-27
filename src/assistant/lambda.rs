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
    Union(Box<LambdaTerm>, Box<LambdaTerm>),
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
            LambdaTerm::Union(box term1, box term2) => {
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
    pub fn intros(mut self) -> (Vec<String>, LambdaTerm) {
        self = rebuild_tree(self.clone(), &mut 1);
        let mut myvec: Vec<String> = vec![];
        let mut old = LambdaTerm::Var(("placeholder".to_string(), 0));
        let mut new = self.clone();
        while old.clone() != new.clone() {
            old = new.clone();
            let (str, rnew) = new.intro();
            new = rnew.clone();
            myvec.push(str);
        }
        (myvec, new)
    }
    pub fn intro(mut self) -> (String, LambdaTerm) {
        self = rebuild_tree(self.clone(), &mut 1);
        // we find a non used name
        let nb = update_counter("hyp");
        let concatenated_name = format!("hyp{}", nb);
        (concatenated_name.clone(), aux_intro(self.clone(), concatenated_name))
    }
    pub fn introv(mut self, name: String) -> LambdaTerm {
        self = rebuild_tree(self.clone(), &mut 1);
        update_counter(&name.clone());
        aux_intro(self.clone(), name)
    }
    pub fn exact(mut self, name: String) -> LambdaTerm {
        self = rebuild_tree(self.clone(), &mut 1);
        aux_exact(self.clone(), name, HashMap::new())
    }
    pub fn cut(mut self, type_a: Type) -> LambdaTerm {
        self = rebuild_tree(self.clone(), &mut 1);
        aux_cut(self.clone(), type_a)
    }
    pub fn apply(mut self, name: String) -> LambdaTerm {
        self = rebuild_tree(self.clone(), &mut 1);
        aux_apply(self.clone(), name, HashMap::new())
    }
    pub fn split(mut self) -> LambdaTerm {
        self = rebuild_tree(self.clone(), &mut 1);
        aux_split(self.clone(), HashMap::new())
    }
    pub fn elim(mut self, name: String) -> LambdaTerm {
        self = rebuild_tree(self.clone(), &mut 1);
        aux_elim(self.clone(), name, HashMap::new())
    }
    pub fn absurd(mut self, statement: Type) -> LambdaTerm {
        self = rebuild_tree(self.clone(), &mut 1);
        aux_absurd(self.clone(), statement, HashMap::new())
    }
    pub fn assumption(self) -> LambdaTerm {
        aux_assumtion(self.clone())
    }
    pub fn left(mut self) -> LambdaTerm {
        self = rebuild_tree(self.clone(), &mut 1);
        aux_left(self.clone())
    }
    pub fn right(mut self) -> LambdaTerm {
        self = rebuild_tree(self.clone(), &mut 1);
        aux_right(self.clone())
    }
}

fn aux_right(root: LambdaTerm) -> LambdaTerm {
    match root {
        LambdaTerm::Goal(typ, nb2) if nb2 == 1 => {
            match typ {
                Type::Or(box _a, box b) => {
                    LambdaTerm::Goal(b, 0)
                }
                _ => {
                    panic!("Cant use that here, sry !");
                }
            }
        }
        // we propagate
        LambdaTerm::ExFalso(t, box proof) => {
            LambdaTerm::ExFalso(t, Box::new(aux_right(proof)))
        }
        LambdaTerm::Var(..) | LambdaTerm::Fst(..) | LambdaTerm::Snd(..) | LambdaTerm::Goal(..) => {
            root
        },
        LambdaTerm::Couple(box term1, box term2) => {
            LambdaTerm::Couple(
                Box::new(aux_right(term1)), 
                Box::new(aux_right(term2)), 
            )
        }
        LambdaTerm::Union(box term1, box term2) => {
            LambdaTerm::Union(
                Box::new(aux_right(term1)), 
                Box::new(aux_right(term2)), 
            )
        }
        LambdaTerm::App(box first, box second) => {
            LambdaTerm::App(
                Box::new(aux_right(first)), 
                Box::new(aux_right(second)), 
            )
        }
        LambdaTerm::Abs(str, typ, box lambdaterm) => {
            LambdaTerm::Abs(str, typ, Box::new(aux_right(lambdaterm)))
        }
    }
}

fn aux_left(root: LambdaTerm) -> LambdaTerm {
    match root {
        LambdaTerm::Goal(typ, nb2) if nb2 == 1 => {
            match typ {
                Type::Or(box a, box b) => {
                    LambdaTerm::Goal(a, 0)
                }
                _ => {
                    panic!("Cant use that here, sry !");
                }
            }
        }
        // we propagate
        LambdaTerm::ExFalso(t, box proof) => {
            LambdaTerm::ExFalso(t, Box::new(aux_left(proof)))
        }
        LambdaTerm::Var(..) | LambdaTerm::Fst(..) | LambdaTerm::Snd(..) | LambdaTerm::Goal(..) => {
            root
        },
        LambdaTerm::Couple(box term1, box term2) => {
            LambdaTerm::Couple(
                Box::new(aux_left(term1)), 
                Box::new(aux_left(term2)), 
            )
        }
        LambdaTerm::Union(box term1, box term2) => {
            LambdaTerm::Union(
                Box::new(aux_left(term1)), 
                Box::new(aux_left(term2)), 
            )
        }
        LambdaTerm::App(box first, box second) => {
            LambdaTerm::App(
                Box::new(aux_left(first)), 
                Box::new(aux_left(second)), 
            )
        }
        LambdaTerm::Abs(str, typ, box lambdaterm) => {
            LambdaTerm::Abs(str, typ, Box::new(aux_left(lambdaterm)))
        }
    }
}

fn aux_assumtion(term: LambdaTerm) -> LambdaTerm {
    use crate::bfs_find_goals;

    // TODO : this into a function, used 2 times (other is in main.rs)
    let goals = bfs_find_goals(term.clone());
    let paths: Vec<(Type, Vec<LambdaTerm>)> = goals.iter().cloned()
        .filter(|x| match x.1.last().unwrap().clone() {LambdaTerm::Goal(_, i) => i == 1, _ => false}).collect();
    let (goal_type, path) = paths.first().unwrap().clone();
    let mut local_hyp: Vec<(String, Type)> = Vec::new();
    for elt in path {
        match elt {
            LambdaTerm::Abs(name, typ, _) => {
                local_hyp.push((name, typ));
            }
            _ => {}
        }
    }
    println!("Hyps : {:?}", local_hyp);
    for (i, (name, typ)) in local_hyp.iter().enumerate() {
        if typ.clone() == goal_type {
            return term.exact(name.clone());
        }
    }

    term
}

pub fn rebuild_tree(term: LambdaTerm, goal_index: &mut usize) -> LambdaTerm {
    match term {
        LambdaTerm::Var((name)) => {
            return LambdaTerm::Var((name));
        }
        LambdaTerm::Couple(box left, box right) => {
            let leftside = rebuild_tree(left, goal_index);
            let rightside = rebuild_tree(right, goal_index);
            return LambdaTerm::Couple(
                Box::new(leftside), 
                Box::new(rightside)
            );
        }
        LambdaTerm::Union(box left, box right) => {
            let leftside = rebuild_tree(left, goal_index);
            let rightside = rebuild_tree(right, goal_index);
            return LambdaTerm::Union(
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

fn aux_absurd(root: LambdaTerm, statement: Type, context: HashMap<String, Type>) -> LambdaTerm {
    match root {
        LambdaTerm::Goal(_typ, nb2) 
        if nb2 == 1 => {
            LambdaTerm::ExFalso(
                statement,
                Box::new(LambdaTerm::Goal(Type::Bottom, 0))
            )
        }        // we propagate
        LambdaTerm::ExFalso(t, box proof) => {
            LambdaTerm::ExFalso(t, Box::new(aux_absurd(proof, statement, context)))
        }
        LambdaTerm::Var(..) | LambdaTerm::Fst(..) | LambdaTerm::Snd(..) | LambdaTerm::Goal(..) => {
            root
        },
        LambdaTerm::Couple(box term1, box term2) => {
            LambdaTerm::Couple(
                Box::new(aux_absurd(term1, statement.clone(), context.clone())), 
                Box::new(aux_absurd(term2, statement, context)), 
            )
        }
        LambdaTerm::Union(box term1, box term2) => {
            LambdaTerm::Union(
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
    let mut state = -1; // todo : fix
    if type_h != Type::Error {
        match type_h.clone() {
            Type::And(box typea, box typeb) => {
                state = 0;
                type_a = typea.clone();
                type_b = typeb.clone();
            }
            Type::Or(box typea, box typeb) => {
                state = 1;
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
        if type_h != Type::Error && 1 == nb2 => {
            // App(App(Goal(a->b->a),Fst(h1)),Snd(h1))
            match state {
                0 => {
                    return LambdaTerm::App(
                        Box::new(
                            LambdaTerm::App(
                                Box::new(LambdaTerm::Goal(
                                    Type::Imp(
                                        Box::new(type_a.clone()),
                                        Box::new(Type::Imp(Box::new(type_b),Box::new(type_c),))),
                                    0
                                )),
                                Box::new(LambdaTerm::Fst(name.clone())),
                            )
                        ),
                        Box::new(LambdaTerm::Snd(name))
                    );
                },
                1 => {
                    return LambdaTerm::Union(
                        Box::new(
                            LambdaTerm::App(
                                Box::new(LambdaTerm::Goal(
                                    Type::Imp(Box::new(type_a.clone()),Box::new(type_c.clone()),),0
                                )),
                                Box::new(LambdaTerm::Fst(name.clone()))
                            )
                        ),
                        Box::new(
                            LambdaTerm::App(
                                Box::new(LambdaTerm::Goal(
                                    Type::Imp(Box::new(type_b.clone()),Box::new(type_c.clone()),),0
                                )),
                                Box::new(LambdaTerm::Snd(name.clone()))
                            )
                        )
                    )
                }
                _ => unreachable!()
            }
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
        LambdaTerm::Union(box term1, box term2) => {
            LambdaTerm::Union(
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
        LambdaTerm::Goal(Type::And(box a, box b), nb2) 
        if nb2 == 1 => {
            LambdaTerm::Couple(
                Box::new(LambdaTerm::Goal(a, 0)), 
                Box::new(LambdaTerm::Goal(b, 0))
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
        LambdaTerm::Union(box term1, box term2) => {
            LambdaTerm::Union(
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
        LambdaTerm::Union(box first, box second) => {
            Type::Or(
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
                Type::Or(box a, box_b) => {
                    return a
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
                Type::Or(box _, box b) => {
                    return b
                }
                other => panic!("Error, unknown : {:?}", other)
            }
        }
    }
}

fn aux_intro(root: LambdaTerm, name: String) -> LambdaTerm {
    match root {
        LambdaTerm::Goal(Type::Imp(box a, box b), nb2) 
        if 1 == nb2
        => {
            LambdaTerm::Abs(name, a, Box::new(LambdaTerm::Goal(b, 0)))
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
        LambdaTerm::Union(box term1, box term2) => {
            LambdaTerm::Union(Box::new(aux_intro(term1, name.clone())), Box::new(aux_intro(term2, name)))
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
        LambdaTerm::Goal(typ, nb2) 
        if typ == type_h && nb2 == 1 => {
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
        LambdaTerm::Union(box term1, box term2) => {
            LambdaTerm::Union(
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
        LambdaTerm::Goal(type_b, nb2) 
        if 1 == nb2 => {
            LambdaTerm::App(
                Box::new(LambdaTerm::Goal(Type::Imp(Box::new(type_a.clone()), Box::new(type_b)), 0)),
                Box::new(LambdaTerm::Goal(type_a, 0)),
            )
        }
        // we propagate
        LambdaTerm::ExFalso(t, box proof) => {
            LambdaTerm::ExFalso(t, Box::new(aux_cut(proof, type_a)))
        }
        LambdaTerm::Var(..) | LambdaTerm::Fst(..) | LambdaTerm::Snd(..) | LambdaTerm::Goal(..) => {
            root
        },
        LambdaTerm::Couple(box term1, box term2) => {
            LambdaTerm::Couple(Box::new(term1.cut(type_a.clone())), Box::new(term2.cut(type_a)))
        }
        LambdaTerm::Union(box term1, box term2) => {
            LambdaTerm::Union(Box::new(term1.cut(type_a.clone())), Box::new(term2.cut(type_a)))
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
        LambdaTerm::Union(box term1, box term2) => {
            LambdaTerm::Union(
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
