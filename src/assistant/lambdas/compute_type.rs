use std::collections::HashMap;

use crate::assistant::lambda as lambda;

use lambda::{
    LambdaTerm,
};

pub fn compute_type(lambdaterm: LambdaTerm, context: HashMap<String, LambdaTerm>) -> LambdaTerm {
    match lambdaterm.clone() {
        LambdaTerm::Var(name, _nb) => {
            let res = context.get(&name).unwrap().clone();
            res
        }
        LambdaTerm::Goal(..) 
        | LambdaTerm::Error => {
            panic!("Not supposed to happend !")
        }
        LambdaTerm::Pi(name, box first, box second) => {
            LambdaTerm::pi(
                name, 
                compute_type(first, context.clone()), 
                compute_type(second, context)
            )
        }
        LambdaTerm::Func(var, box typ, box body) => {
            let mut new_context = context.clone();
            new_context.insert(var.clone(), typ.clone());

            let body_type = compute_type(body, new_context);
            LambdaTerm::pi(var, typ, body_type)
        }

    }
}