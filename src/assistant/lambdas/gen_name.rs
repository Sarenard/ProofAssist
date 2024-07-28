use std::collections::HashMap;

use crate::assistant::lambdas as lambdas;
use crate::assistant::lambda as lambda;

use lambda::LambdaTerm;

pub fn gen_name(already_seen: Vec<String>) -> String {
    for i in 0..already_seen.len() {
        let name = format!("VAR{}", i);
        if !already_seen.contains(&name) {
            return name;
        }
    }
    unreachable!()
}