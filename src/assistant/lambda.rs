use std::collections::HashMap;

use crate::assistant::lambdas as lambdas;

use lambdas::compute_type::compute_type;

#[derive(Debug, Clone)]
pub enum LambdaTerm {
    Var(String),

    Goal(Box<LambdaTerm>, usize),
    Types, // maybe some problems?

    Pi(String, Box<LambdaTerm>, Box<LambdaTerm>),
    Func(String, Box<LambdaTerm>, Box<LambdaTerm>),
    App(Box<LambdaTerm>, Box<LambdaTerm>),

    Sigma(String, Box<LambdaTerm>, Box<LambdaTerm>),
    Couple(Box<LambdaTerm>, Box<LambdaTerm>, Box<LambdaTerm>),
    Proj(Box<LambdaTerm>, Box<LambdaTerm>),

    Error,
}

#[allow(dead_code)]
impl LambdaTerm {
    pub fn containsgoal(self) -> bool {
        use crate::get_goal_count;
        get_goal_count(self) != 0
    }

    pub fn check(self, goal: LambdaTerm) -> bool {
        let computed = compute_type(self, HashMap::new());
        if goal != computed {
            println!("Computed : {:?}", computed);
            println!("Goal : {:?}", goal);
        }
        goal == computed
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
