#![feature(box_patterns)]

use std::io;
use std::process::exit;

#[macro_use]
extern crate pest_derive;

use pest::iterators::Pairs;
use pest::Parser;

#[derive(Parser)]
#[grammar = "simple.pest"] // relative to src
struct SimpleParser;

mod assistant;

use assistant::lambda::LambdaTerm as LambdaTerm;
use assistant::types::Type as Type;

static SHELL: bool = true;

fn main() {
    // on veut prouver a => (a => b) => b
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
                ["cut", var] => {
                    let parse_result = SimpleParser::parse(
                        Rule::main, 
                        var
                    );
                    let mut val = match parse_result {
                        Ok(parsed) => parsed,
                        Err(_) => {
                            println!("Invalid command, please retry.");
                            continue;
                        },
                    };

                    let my_type = parse_type(val.next().unwrap());
                    lambdaterme = lambdaterme.cut(my_type);
                    println!("{:?}", lambdaterme);
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

    let ok = lambdaterme.check();
    if ok {
        println!("OK !");
    } else {
        panic!("Ehh i'm wrong");
    }

}

fn parse_type(pair: pest::iterators::Pair<Rule>) -> Type {
    match pair.as_rule() {
        Rule::main => {
            let inner_pair = pair.into_inner().next().unwrap();
            parse_type(inner_pair)
        }
        Rule::var => {
            let inner = pair.into_inner().next().unwrap();
            let text = inner.as_str();
            let length = text.len();
            let text = &text[1..length-1];
            Type::Var(text.to_string())
        }
        Rule::impl_type => {
            let mut inner = pair.into_inner();
            let first = parse_type(inner.next().unwrap());
            let second = parse_type(inner.next().unwrap());
            Type::Impl(Box::new(first), Box::new(second))
        }
        Rule::and_type => {
            let mut inner = pair.into_inner();
            let first = parse_type(inner.next().unwrap());
            let second = parse_type(inner.next().unwrap());
            Type::And(Box::new(first), Box::new(second))
        }
        _ => unreachable!(),
    }
}
