use std::collections::HashMap;

use crate::assistant::lambdas::free_var::free_var;
use crate::assistant::lambdas as lambdas;
use crate::assistant::lambda as lambda;

use lambda::LambdaTerm;

use lambdas::{
    update_nbs::update_goals_nb,
    compute_type::compute_type,
};

use crate::DEBUG;

fn aux_apply(root: LambdaTerm, name: String, context: HashMap<String, LambdaTerm>, instanciation: HashMap<String, LambdaTerm>) -> LambdaTerm {
    if DEBUG {println!("aux_apply : {:?}", root);}
    // with the help of Coda    
    fn construct(goal: LambdaTerm, instanciation: HashMap<String, LambdaTerm>, context: HashMap<String, LambdaTerm>, accu: LambdaTerm) -> LambdaTerm {
        let accu_inferred = compute_type(accu.clone(), context.clone());
        println!("construct accu:{:?}, accu_inf:{:?}, context:{:?}", accu, accu_inferred, context);
        if accu_inferred == goal {
            return accu;
        }
        match accu_inferred {
            // first -> second
            LambdaTerm::Pi(name, box first, box second)
            if !free_var(second.clone()).contains(&name) => {
                println!("impl : {:?}", accu);
                construct(goal, instanciation, context,
                    LambdaTerm::app(accu, LambdaTerm::goal(first))
                )
            }
            // forall name:typ, body
            LambdaTerm::Pi(pi_name, box _typ, box body)
            if free_var(body.clone()).contains(&pi_name) => {
                println!("forall : {:?} {}", accu, pi_name);
                let type_name = instanciation.get(&pi_name).unwrap().clone();
                construct(goal, instanciation, context, 
                    LambdaTerm::app(accu, type_name)
                )
            }
            other => panic!("Unexpected {:?}", other)
        }
    }
    match root {
        LambdaTerm::Goal(box typeb, nb) if nb == 1 => {
            construct(typeb, instanciation, context, LambdaTerm::Var(name))
        }
        // we propagate
        LambdaTerm::Var(..) 
        | LambdaTerm::Types
        | LambdaTerm::Bot
        | LambdaTerm::Top
        | LambdaTerm::Goal(..) => {
            root
        },
        LambdaTerm::Func(str, box typ, box lambdaterm) => {
            let mut new_context = context.clone();
            new_context.insert(str.clone(), typ.clone());
            LambdaTerm::func(
                str, 
                aux_apply(typ, name.clone(), context.clone(), instanciation.clone()), 
                aux_apply(lambdaterm, name, new_context, instanciation)
        )
        }
        LambdaTerm::Pi(str, box typ, box lambdaterm) => {
            LambdaTerm::pi(
                str, 
                aux_apply(typ, name.clone(), context.clone(), instanciation.clone()), 
                aux_apply(lambdaterm, name, context, instanciation)
            )
        }
        LambdaTerm::Sigma(str, box typ, box lambdaterm) => {
            LambdaTerm::sigma(
                str, 
                aux_apply(typ, name.clone(), context.clone(), instanciation.clone()), 
                aux_apply(lambdaterm, name, context, instanciation)
            )
        }
        LambdaTerm::App(box first, box second) => {
            LambdaTerm::app(
                aux_apply(first, name.clone(), context.clone(), instanciation.clone()),
                aux_apply(second, name, context, instanciation)
            )
        }
        LambdaTerm::ExFalso(box first, box second) => {
            LambdaTerm::exfalso(
                aux_apply(first, name.clone(), context.clone(), instanciation.clone()),
                aux_apply(second, name, context, instanciation)
            )
        }
        LambdaTerm::Proj(box first, box second) => {
            LambdaTerm::proj(
                aux_apply(first, name.clone(), context.clone(), instanciation.clone()),
                aux_apply(second, name, context, instanciation)
            )
        }
        LambdaTerm::Couple(box first, box second, box third) => {
            LambdaTerm::couple(
                aux_apply(first, name.clone(), context.clone(), instanciation.clone()),
                aux_apply(second, name.clone(), context.clone(), instanciation.clone()),
                aux_apply(third, name, context, instanciation)
            )
        }
        LambdaTerm::Error => panic!()
    }
}

impl LambdaTerm {
    // naive approch
    pub fn apply(mut self, name: String, instanciation: HashMap<String, LambdaTerm>) -> LambdaTerm {
        self = update_goals_nb(self.clone(), &mut 1);
        aux_apply(self.clone(), name, HashMap::new(), instanciation)
    }
}
