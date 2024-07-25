#![feature(box_patterns)]

mod assistant;

use std::io;
use std::process::exit;

use assistant::lambda::LambdaTerm as LambdaTerm;
use assistant::types::Type as Type;

static SHELL: bool = false;

fn main() {
    // on veut prouver a ^ b => b ^ a
    let goal = Type::Impl(
        Box::new(Type::Var("A".to_string())),
        Box::new(Type::Impl(
            Box::new(Type::Impl(
                Box::new(Type::Var("A".to_string())),
                Box::new(Type::Var("B".to_string()))
            )),
            Box::new(Type::Var("B".to_string()))
        )),
    );

    let mut lambdaterme = LambdaTerm::Goal(goal.clone());
    println!("{:?}", lambdaterme);

    if SHELL {

        while lambdaterme.clone().containsgoal() {
            let mut bad = false;
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            input = input.trim().to_string();
            let splitted: Vec<&str> = input.split_whitespace().take(2).collect();
            if splitted.len() < 2 {
                bad = true;
            }
            let array = if !bad {[splitted[0], splitted[1]]} else {["_", "_"]};
            match array {
                ["intro", var] => {
                    lambdaterme = lambdaterme.intro(var.to_string());
                    println!("{:?}", lambdaterme);
                }
                ["exact", var] => {
                    lambdaterme = lambdaterme.exact(var.to_string());
                    println!("{:?}", lambdaterme);
                }
                ["cut", _] => {
                    println!("Idk how to implement it shellwise");
                }
                ["apply", var] => {
                    lambdaterme = lambdaterme.apply(var.to_string());
                    println!("{:?}", lambdaterme);
                }
                _ => {
                    bad = true;
                }
            }
            if bad {
                println!("Invalid command, please retry.");
            }
        }

        println!("Theorem proved : {:?}", goal);

        exit(0);
    }

    let lambdaterme = lambdaterme.intro("h1".to_string());
    println!("{:?}", lambdaterme);

    let lambdaterme = lambdaterme.intro("h2".to_string());
    println!("{:?}", lambdaterme);

    let lambdaterme = lambdaterme.apply("h2".to_string());
    println!("{:?}", lambdaterme);

    let lambdaterme = lambdaterme.exact("h1".to_string());
    println!("{:?}", lambdaterme);

}

