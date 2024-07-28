use std::{collections::HashMap, fmt};

use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref HASHMAP: Mutex<HashMap<String, usize>> = Mutex::new(HashMap::new());
}

use crate::assistant::lambdas as lambdas;

// in dependant type theory, lambdaexpr and types are the same exact thing
#[derive(Debug, Clone, PartialEq)]
pub enum LambdaTerm {
    Var(String, usize),
    Goal(Box<LambdaTerm>, usize),
    Pi(String, Box<LambdaTerm>, Box<LambdaTerm>),
}

#[allow(dead_code)]
impl LambdaTerm {
    pub fn containsgoal(self) -> bool {
        use crate::get_goal_count;
        get_goal_count(self) != 0
    }

    pub fn check(self, goal: LambdaTerm) -> bool {
        todo!()
    }
    
    pub fn model(mut self, thing: LambdaTerm) -> LambdaTerm {
        self = update_goals_nb(self.clone(), &mut 1);
        todo!()
    }
}

pub fn update_goals_nb(term: LambdaTerm, goal_index: &mut usize) -> LambdaTerm {
    match term {
        LambdaTerm::Var(..) => {
            term
        }
        LambdaTerm::Goal(box typ, _index) => {
            *goal_index += 1;
            LambdaTerm::goal(
                typ,
                *goal_index - 1
            )
        }
        LambdaTerm::Pi(name, box lb1, box lb2) => {
            let part1 = update_goals_nb(lb1, goal_index);
            let part2 = update_goals_nb(lb2, goal_index);
            LambdaTerm::pi(name, part1, part2)
        }
    }
}

pub fn update_counter(key: &str) -> usize {
    let mut map = HASHMAP.lock().unwrap();
    let counter = map.entry(key.to_string()).or_insert(0);
    *counter += 1;
    let cpt = counter.clone();
    drop(map);
    cpt
}
