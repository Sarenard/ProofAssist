use std::collections::HashMap;

use crate::assistant::lambdas as lambdas;

use lambdas::update_nbs::update_goals_nb;

// in dependant type theory, lambdaexpr and types are the same exact thing
#[derive(Debug, Clone, PartialEq)]
pub enum LambdaTerm {
    Var(String),
    Goal(Box<LambdaTerm>, usize),
    Pi(String, Box<LambdaTerm>, Box<LambdaTerm>),
    Func(String, Box<LambdaTerm>, Box<LambdaTerm>),
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
}

use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref HASHMAP: Mutex<HashMap<String, usize>> = Mutex::new(HashMap::new());
}

pub fn update_counter(key: &str) -> usize {
    let mut map = HASHMAP.lock().unwrap();
    let counter = map.entry(key.to_string()).or_insert(0);
    *counter += 1;
    let cpt = counter.clone();
    drop(map);
    cpt
}
