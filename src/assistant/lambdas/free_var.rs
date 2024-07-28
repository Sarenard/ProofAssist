use std::collections::HashMap;

use crate::assistant::lambdas as lambdas;
use crate::assistant::lambda as lambda;

use lambda::LambdaTerm;

pub fn free_var(lambda: LambdaTerm) -> Vec<String> {
    match lambda {
        LambdaTerm::Var(name) => {
            vec![name.clone()]
        }
        LambdaTerm::Goal(box typ, nb) => {
            vec![]
        }
        LambdaTerm::Func(name, box typ, box body)
        | LambdaTerm::Pi(name, box typ, box body) => {
            let mut vec_tot: Vec<String> = vec![];
            let variables_typ = free_var(typ);
            let variables_body: Vec<String> = free_var(body).iter().cloned()
            .filter(|x| *x != name).collect();

            vec_tot.extend(variables_typ);
            vec_tot.extend(variables_body);

            vec_tot
        }
        LambdaTerm::Error => unreachable!()
    }
}