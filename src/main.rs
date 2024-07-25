#![feature(box_patterns)]

mod assistant;

use std::io;

use assistant::lambda::LambdaTerm as LambdaTerm;
use assistant::types::Type as Type;

fn main() {
    // on veut prouver a ^ b => b ^ a
    let goal = Type::Impl(
        Box::new(
            Type::Var("a".to_string())
        ),
        Box::new(
            Type::Var("a".to_string()),
        )
    );

    let mut lambdaterme = LambdaTerm::Goal(goal.clone());
    println!("{:?}", lambdaterme);

    while lambdaterme.clone().containsgoal() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        input = input.trim().to_string();
        let splitted: Vec<&str> = input.split_whitespace().take(2).collect();
        let array = [splitted[0], splitted[1]];
        match array {
            ["intro", var] => {
                lambdaterme = lambdaterme.intro(var.to_string());
                println!("{:?}", lambdaterme);
            }
            ["exact", var] => {
                lambdaterme = lambdaterme.exact(var.to_string());
                println!("{:?}", lambdaterme);
            }
            _ => {
                println!("Command not recognised !");
            }
        }
    }

    println!("Theorem proved : {:?}", goal);

}

