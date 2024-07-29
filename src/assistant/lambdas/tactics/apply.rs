use std::collections::HashMap;

use crate::assistant::lambdas as lambdas;
use crate::assistant::lambda as lambda;

use lambda::{
    LambdaTerm,
    update_counter,
};

use lambdas::update_nbs::update_goals_nb;

use crate::DEBUG;

fn aux_apply(root: LambdaTerm, name: String, context: HashMap<String, LambdaTerm>) -> LambdaTerm {
    fn get_types(typ: LambdaTerm, vector: &mut Vec<LambdaTerm>) -> LambdaTerm {
        match typ {
            LambdaTerm::Pi(_name, box typea, box LambdaTerm::Pi(name2, box typeb, box typec)) => {
                vector.push(typea);
                return get_types(LambdaTerm::Pi(name2, Box::new(typeb), Box::new(typec)), vector)
            }
            LambdaTerm::Pi(_name, box typea, box typeb) => {
                vector.push(typea);
                return typeb;
            }
            other => {
                panic!("Impossible... : {:?}", other)
            }
        }
    }
    fn construct(types: &mut Vec<LambdaTerm>, name: String) -> LambdaTerm {
        let new = types.pop();
        match new.clone() {
            Some(typ) => {
                return LambdaTerm::app(construct(types, name), LambdaTerm::goalnb(typ, 0));
            }
            None => {
                return LambdaTerm::Var(name);
            }
        }
    }
    match root {
        LambdaTerm::Goal(_typeb, nb) if nb == 1 => {
            let type_objective = context.get(&name).unwrap().clone();
            let mut myvec: Vec<LambdaTerm> = vec![];
            let types = get_types(type_objective.clone(), &mut myvec);
            if DEBUG { println!("types : {:?}, vec : {:?}, type_objective : {:?}", types, myvec, type_objective); }
            let constructed = construct(&mut myvec, name);
            if DEBUG { println!("new_thing {:?}", constructed); }

            constructed
        }
        // we propagate
        LambdaTerm::Var(..) 
        | LambdaTerm::Types
        | LambdaTerm::Goal(..) => {
            root
        },
        LambdaTerm::Func(str, box typ, box lambdaterm) => {
            let mut new_context = context.clone();
            new_context.insert(str.clone(), typ.clone());
            LambdaTerm::func(
                str, 
                aux_apply(typ, name.clone(), context.clone()), 
                aux_apply(lambdaterm, name, new_context)
        )
        }
        LambdaTerm::Pi(str, box typ, box lambdaterm) => {
            LambdaTerm::pi(
                str, 
                aux_apply(typ, name.clone(), context.clone()), 
                aux_apply(lambdaterm, name, context)
            )
        }
        LambdaTerm::Sigma(str, box typ, box lambdaterm) => {
            LambdaTerm::sigma(
                str, 
                aux_apply(typ, name.clone(), context.clone()), 
                aux_apply(lambdaterm, name, context)
            )
        }
        LambdaTerm::App(box first, box second) => {
            LambdaTerm::app(
                aux_apply(first, name.clone(), context.clone()),
                aux_apply(second, name, context)
            )
        }
        LambdaTerm::Proj(box first, box second) => {
            LambdaTerm::proj(
                aux_apply(first, name.clone(), context.clone()),
                aux_apply(second, name, context)
            )
        }
        LambdaTerm::Couple(box first, box second, box third) => {
            LambdaTerm::couple(
                aux_apply(first, name.clone(), context.clone()),
                aux_apply(second, name.clone(), context.clone()),
                aux_apply(third, name, context)
            )
        }
        LambdaTerm::Error => panic!()
    }
}

impl LambdaTerm {
    // naive approch
    pub fn apply(mut self, name: String) -> LambdaTerm {
        self = update_goals_nb(self.clone(), &mut 1);
        aux_apply(self.clone(), name, HashMap::new())
    }
}
