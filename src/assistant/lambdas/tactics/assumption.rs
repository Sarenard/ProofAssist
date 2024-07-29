use crate::assistant::lambdas as lambdas;
use crate::assistant::lambda as lambda;

use lambda::LambdaTerm;

use lambdas::update_nbs::update_goals_nb;

use crate::DEBUG as DEBUG;

fn aux_assumtion(term: LambdaTerm) -> LambdaTerm {
    use crate::bfs_find_goals;

    // TODO : this into a function, used 2 times (other is in main.rs)
    let goals = bfs_find_goals(term.clone());
    let paths: Vec<(LambdaTerm, Vec<LambdaTerm>)> = goals.iter().cloned()
        .filter(|x| match x.1.last().unwrap().clone() {LambdaTerm::Goal(_, i) => i == 1, _ => false}).collect();
    let (goal_type, path) = paths.first().unwrap().clone();
    let mut local_hyp: Vec<(String, LambdaTerm)> = Vec::new();
    for elt in path {
        match elt {
            LambdaTerm::Func(name, box typ, _) => {
                local_hyp.push((name, typ));
            }
            _ => {}
        }
    }
    if DEBUG {println!("local_hyp : {:?}", local_hyp);}
    for (_i, (name, typ)) in local_hyp.iter().enumerate() {
        if typ.clone() == goal_type {
            if DEBUG {println!("applying : {:?}", name);}
            return term.exact(name.clone());
        }
    }

    term
}

impl LambdaTerm {
    pub fn assumption(mut self) -> LambdaTerm {
        self = update_goals_nb(self.clone(), &mut 1);
        aux_assumtion(self.clone())
    }
}
