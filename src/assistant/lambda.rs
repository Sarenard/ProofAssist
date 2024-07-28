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
    Goal(Type, usize),
    ExFalso(Type, Box<LambdaTerm>),
    Left(Box<LambdaTerm>, Type),
    Right(Box<LambdaTerm>, Type),
    Match(Type, Box<LambdaTerm>, Box<LambdaTerm>), // Match(lambda, case1, case2)
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
            LambdaTerm::Left(box lambda, _typ) => {
                found |= lambda.containsgoal();
            }
            LambdaTerm::Right(box lambda, _typ) => {
                found |= lambda.containsgoal();
            }
            LambdaTerm::Match(_typ, box case1, box case2) => {
                found |= case1.containsgoal();
                found |= case2.containsgoal();
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
    pub fn assumption(mut self) -> LambdaTerm {
        self = rebuild_tree(self.clone(), &mut 1);
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
                Type::Or(box a, box b) => {
                   LambdaTerm::Right(Box::new(LambdaTerm::Goal(b, 0)), a)
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
        LambdaTerm::App(box first, box second) => {
            LambdaTerm::App(
                Box::new(aux_right(first)), 
                Box::new(aux_right(second)), 
            )
        }
        LambdaTerm::Abs(str, typ, box lambdaterm) => {
            LambdaTerm::Abs(str, typ, Box::new(aux_right(lambdaterm)))
        }
        LambdaTerm::Left(box lambda, typ) => {
            LambdaTerm::Left(Box::new(aux_right(lambda)), typ)
        }
        LambdaTerm::Right(box lambda, typ) => {
            LambdaTerm::Right(Box::new(aux_right(lambda)), typ)
        }
        LambdaTerm::Match(typ, box case1, box case2) => {
            LambdaTerm::Match(typ, Box::new(aux_right(case1)), Box::new(aux_right(case2)))
        }
    }
}

fn aux_left(root: LambdaTerm) -> LambdaTerm {
    match root {
        LambdaTerm::Goal(typ, nb2) if nb2 == 1 => {
            match typ {
                Type::Or(box a, box b) => {
                   LambdaTerm::Left(Box::new(LambdaTerm::Goal(a, 0)), b)
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
        LambdaTerm::App(box first, box second) => {
            LambdaTerm::App(
                Box::new(aux_left(first)), 
                Box::new(aux_left(second)), 
            )
        }
        LambdaTerm::Abs(str, typ, box lambdaterm) => {
            LambdaTerm::Abs(str, typ, Box::new(aux_left(lambdaterm)))
        }
        LambdaTerm::Left(box lambda, typ) => {
            LambdaTerm::Left(Box::new(aux_left(lambda)), typ)
        }
        LambdaTerm::Right(box lambda, typ) => {
            LambdaTerm::Right(Box::new(aux_left(lambda)), typ)
        }
        LambdaTerm::Match(typ, box case1, box case2) => {
            LambdaTerm::Match(typ, Box::new(aux_left(case1)), Box::new(aux_left(case2)))
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
    for (_i, (name, typ)) in local_hyp.iter().enumerate() {
        if typ.clone() == goal_type {
            return term.exact(name.clone());
        }
    }

    term
}

pub fn rebuild_tree(term: LambdaTerm, goal_index: &mut usize) -> LambdaTerm {
    match term {
        LambdaTerm::Var(name) => {
            return LambdaTerm::Var(name);
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
        LambdaTerm::Left(box lambda, typ) => {
            LambdaTerm::Left(Box::new(rebuild_tree(lambda, goal_index)), typ)
        }
        LambdaTerm::Right(box lambda, typ) => {
            LambdaTerm::Right(Box::new(rebuild_tree(lambda, goal_index)), typ)
        }
        LambdaTerm::Match(typ, box case1, box case2) => {
            LambdaTerm::Match(typ, Box::new(rebuild_tree(case1, goal_index)), Box::new(rebuild_tree(case2, goal_index)))
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
        LambdaTerm::Left(box lambda, typ) => {
            LambdaTerm::Left(Box::new(aux_absurd(lambda, statement, context)), typ)
        }
        LambdaTerm::Right(box lambda, typ) => {
            LambdaTerm::Right(Box::new(aux_absurd(lambda, statement, context)), typ)
        }
        LambdaTerm::Match(typ, box case1, box case2) => {
            LambdaTerm::Match(
                typ,
                Box::new(aux_absurd(case1, statement.clone(), context.clone())), 
                Box::new(aux_absurd(case2, statement, context))
            )
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
                    return LambdaTerm::Match(
                        type_c.clone(), 
                        Box::new(LambdaTerm::Goal(Type::imp(type_a, type_c.clone()), 0)),
                        Box::new(LambdaTerm::Goal(Type::imp(type_b, type_c), 0)),
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
        LambdaTerm::Left(box lambda, typ) => {
            LambdaTerm::Left(Box::new(aux_elim(lambda, name, context)), typ)
        }
        LambdaTerm::Right(box lambda, typ) => {
            LambdaTerm::Right(Box::new(aux_elim(lambda, name, context)), typ)
        }
        LambdaTerm::Match(typ, box case1, box case2) => {
            LambdaTerm::Match(
                typ,
                Box::new(aux_elim(case1, name.clone(), context.clone())), 
                Box::new(aux_elim(case2, name.clone(), context.clone()))
            )
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
        LambdaTerm::Left(box lambda, typ) => {
            LambdaTerm::Left(Box::new(aux_split(lambda, context)), typ)
        }
        LambdaTerm::Right(box lambda, typ) => {
            LambdaTerm::Right(Box::new(aux_split(lambda, context)), typ)
        }
        LambdaTerm::Match(typ, box case1, box case2) => {
            LambdaTerm::Match(
                typ,
                Box::new(aux_split(case1, context.clone())), 
                Box::new(aux_split(case2, context))
            )
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
                Type::Or(box a, _) => {
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
        LambdaTerm::Left(box lambda, typ) => {
            return Type::or(compute_type(lambda, mytypes), typ)
        }
        LambdaTerm::Right(box lambda, typ) => {
            return Type::or(typ, compute_type(lambda, mytypes))
        }
        LambdaTerm::Match(typ, box case1, box case2) => {
            let type1 = compute_type(case1, mytypes.clone());
            let type2 = compute_type(case2, mytypes);
            match (type1, type2, typ) {
                (
                    Type::Imp(box a, box b),
                    Type::Imp(box c, box d),
                    Type::Or(box e, box f)
                ) 
                if (b == d && a == f && c == e)
                => {
                    return b;
                }
                other => {
                    panic!("Erreur {:?}", other)
                }
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
        LambdaTerm::App(box first, box second) => {
            LambdaTerm::App(Box::new(aux_intro(first, name.clone())),Box::new(aux_intro(second, name)))
        }
        LambdaTerm::Abs(str, typ, box lambdaterm) => {
            LambdaTerm::Abs(str, typ, Box::new(aux_intro(lambdaterm, name)))
        }
        LambdaTerm::Left(box lambda, typ) => {
            LambdaTerm::Left(Box::new(aux_intro(lambda, name)), typ)
        }
        LambdaTerm::Right(box lambda, typ) => {
            LambdaTerm::Right(Box::new(aux_intro(lambda, name)), typ)
        }
        LambdaTerm::Match(typ, box case1, box case2) => {
            LambdaTerm::Match(typ, Box::new(aux_intro(case1, name.clone())), Box::new(aux_intro(case2, name)))
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
        LambdaTerm::Left(box lambda, typ) => {
            LambdaTerm::Left(Box::new(aux_exact(lambda, name, context)), typ)
        }
        LambdaTerm::Right(box lambda, typ) => {
            LambdaTerm::Right(Box::new(aux_exact(lambda, name, context)), typ)
        }
        LambdaTerm::Match(typ, box case1, box case2) => {
            LambdaTerm::Match(
                typ,
                Box::new(aux_exact(case1, name.clone(), context.clone())), 
                Box::new(aux_exact(case2, name, context))
            )
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
        LambdaTerm::App(box first, box second) => {
            LambdaTerm::App(Box::new(first.cut(type_a.clone())), Box::new(second.cut(type_a)))
        }
        LambdaTerm::Abs(str, typ, box lambdaterm) => {
            LambdaTerm::Abs(str, typ, Box::new(lambdaterm.cut(type_a)))
        }
        LambdaTerm::Left(box lambda, typ) => {
            LambdaTerm::Left(Box::new(lambda.cut(type_a)), typ)
        }
        LambdaTerm::Right(box lambda, typ) => {
            LambdaTerm::Right(Box::new(lambda.cut(type_a)), typ)
        }
        LambdaTerm::Match(typ, box case1, box case2) => {
            LambdaTerm::Match(typ, Box::new(case1.cut(type_a.clone())), Box::new(case2.cut(type_a)))
        }
    }
}

fn aux_apply(root: LambdaTerm, name: String, context: HashMap<String, Type>) -> LambdaTerm {
    fn get_types(typ: Type, vector: &mut Vec<Type>) -> Type {
        match typ {
            Type::Imp(box typea, box Type::Imp(box typeb, box typec)) => {
                vector.push(typea);
                return get_types(Type::Imp(Box::new(typeb), Box::new(typec)), vector)
            }
            Type::Imp(box typea, box typeb) => {
                vector.push(typea);
                return typeb;
            }
            other => {
                panic!("Impossible... : {:?}", other)
            }
        }
    }
    fn construct(types: &mut Vec<Type>, name: String) -> LambdaTerm {
        let new = types.pop();
        match new.clone() {
            Some(typ) => {
                return LambdaTerm::App(Box::new(construct(types, name)), Box::new(LambdaTerm::Goal(typ, 0)));
            }
            None => {
                let nb = update_counter(&name.clone());
                return LambdaTerm::Var((name, nb));
            }
        }
    }
    match root {
        LambdaTerm::Goal(typeb, nb) if nb == 1 => {
            let type_objective = context.get(&name).unwrap().clone();
            let mut myvec: Vec<Type> = vec![];
            let types = get_types(type_objective.clone(), &mut myvec);
            // println!("types : {:?}, vec : {:?}, type_objective : {:?}", types, myvec, type_objective);
            let constructed = construct(&mut myvec, name);
            // println!("new_thing {:?}", constructed);

            constructed
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
        LambdaTerm::Left(box lambda, typ) => {
            LambdaTerm::Left(Box::new(aux_apply(lambda, name, context)), typ)
        }
        LambdaTerm::Right(box lambda, typ) => {
            LambdaTerm::Right(Box::new(aux_apply(lambda, name, context)), typ)
        }
        LambdaTerm::Match(typ, box case1, box case2) => {
            LambdaTerm::Match(
                typ, 
                Box::new(aux_apply(case1, name.clone(), context.clone())), 
                Box::new(aux_apply(case2, name, context))
            )
        }
    }
}
