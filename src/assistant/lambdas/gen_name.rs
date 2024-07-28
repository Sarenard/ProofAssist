use std::collections::HashMap;

use crate::assistant::lambdas as lambdas;
use crate::assistant::lambda as lambda;

use lambda::LambdaTerm;

pub fn gen_name(already_seen: Vec<String>) -> String {
    for i in 0..already_seen.len() {
        let old_name = already_seen[i].clone();
        let name = format!("{}{}", old_name, i);
        if !already_seen.contains(&name) {
            return name;
        }
    }
    unreachable!()
}