use std::collections::HashMap;

use crate::assistant::lambdas as lambdas;

use lambdas::compute_type::compute_type;

#[derive(Debug, Clone)]
pub enum LambdaTerm {
    Var(String),

    Goal(Box<LambdaTerm>, usize),
    Types, // maybe some problems?
    Bot,
    Top,
    ExFalso(Box<LambdaTerm>, Box<LambdaTerm>),

    // pi type, functions and forall
    Pi(String, Box<LambdaTerm>, Box<LambdaTerm>),
    Func(String, Box<LambdaTerm>, Box<LambdaTerm>),
    App(Box<LambdaTerm>, Box<LambdaTerm>),

    // sigma type, exists and and
    Sigma(String, Box<LambdaTerm>, Box<LambdaTerm>),
    Couple(Box<LambdaTerm>, Box<LambdaTerm>, Box<LambdaTerm>),
    Proj(Box<LambdaTerm>, Box<LambdaTerm>),

    // or
    Or(Box<LambdaTerm>, Box<LambdaTerm>),
    Left(Box<LambdaTerm>, Box<LambdaTerm>), // right is for typecheck purposes
    Right(Box<LambdaTerm>, Box<LambdaTerm>), // right is for typecheck purposes
    Match(Box<LambdaTerm>, Box<LambdaTerm>, Box<LambdaTerm>), // right is for typecheck purposes

    // equality
    Eq(Box<LambdaTerm>, Box<LambdaTerm>), 
    Refl(Box<LambdaTerm>),
    // rewrite(lambda, new_type, old_type)
    // Goal(C) + lambda: A = B => (rewrite lambda) => rewrite(lambda, Goal(C[A <- B]), C)
    Rewrite(Box<LambdaTerm>, Box<LambdaTerm>, Box<LambdaTerm>),

    // booleans
    Bool,
    TBool,
    FBool,
    // if first then second else third end
    Bif(Box<LambdaTerm>, Box<LambdaTerm>, Box<LambdaTerm>),

    // naturals
    Naturals,
    Zero,
    Succ(Box<LambdaTerm>),
    // first : S(a) = S(b), second : a = b => goal
    Inversion(Box<LambdaTerm>, Box<LambdaTerm>),

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
        println!("calcul de OK");
        let ok = goal == computed;
        if !ok {
            println!("Computed : {:?}\n{}", computed, computed);
            println!("Goal : {:?}\n{}", goal, goal);
        }
        ok
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
