use std::{collections::HashMap, fmt};

use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref HASHMAP: Mutex<HashMap<String, usize>> = Mutex::new(HashMap::new());
}

#[derive(Debug, Clone, PartialEq)]
pub enum LambdaTerm {
    Goal(Box<LambdaTerm>, usize),
}

impl std::fmt::Display for LambdaTerm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            _ => unimplemented!()
        }
    }
}

#[allow(dead_code)]
impl LambdaTerm {
    pub fn containsgoal(self) -> bool {
        let mut found = false;
        match self {
            LambdaTerm::Goal(..) => {
                found = true
            }
        }
        found
    }
    pub fn check(self, goal: LambdaTerm) -> bool {
        todo!()
    }
    pub fn model(mut self, thing: LambdaTerm) -> LambdaTerm {
        self = rebuild_tree(self.clone(), &mut 1);
        todo!()
    }
}

pub fn rebuild_tree(term: LambdaTerm, goal_index: &mut usize) -> LambdaTerm {
    match term {
        LambdaTerm::Goal(typ, _index) => {
            *goal_index += 1;
            return LambdaTerm::Goal(
                typ,
                *goal_index - 1
            )
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
