#![feature(box_patterns)]

use std::cmp::{max, min};
use std::collections::{HashMap, VecDeque};
use std::io::{self, Write};

#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "simple.pest"] // relative to src
#[allow(dead_code)]
struct SimpleParser;

mod assistant;

use assistant::lambda::LambdaTerm as LambdaTerm;
use assistant::types::Type as Type;

fn main() {
    let goal = get_goal();
    /*
    let goal = Type::imp(
        Type::var("A"),
        Type::imp(
            Type::var("B"),
            Type::and(Type::var("A"), Type::var("B"))
        )
    ).removenot();
    */
        
    let mut goals_index = 1;

    let mut lambdaterme = LambdaTerm::Goal(goal.clone());
    while lambdaterme.clone().containsgoal() {
        goals_index = min(goals_index, get_goal_count(lambdaterme.clone()));
        println!(); // to be beautiful

        let goals = bfs_find_goals(lambdaterme.clone());
        let path = goals[goals_index-1].1.clone();
        let mut hypotheses: HashMap<String, Type> = HashMap::new();
        for elt in path {
            match elt {
                LambdaTerm::Abs(name, typ, _) => {
                    hypotheses.insert(name, typ);
                }
                _ => {}
            }
        }
        for (txt, typ) in hypotheses.iter() {
            println!("{} : {:?}", txt, typ);
        }
        println!("{}", format!("=============== {}/{}", goals_index, get_goal_count(lambdaterme.clone())));
        println!("{:?}", goals[goals_index-1].0);
        let mut input = String::new();
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("Failed to read line for some reason");
        input = input.trim().to_string();
        let mut splitted = input.split_whitespace().collect::<Vec<&str>>().into_iter();
        let command = splitted.next().unwrap();
        match command {
            "intro" => {
                let name_var = splitted.next();
                match name_var {
                    None => {
                        let (_name, new_lambdaterme) = lambdaterme.intro();
                        lambdaterme = new_lambdaterme;
                    }
                    Some(name) => {
                        lambdaterme = lambdaterme.introv(name.to_string());
                    }
                }
            }
            "intros" => {
                let (_names, new_lambdaterme) = lambdaterme.intros();
                lambdaterme = new_lambdaterme;
            }
            "split" => {
                lambdaterme = lambdaterme.split();
            }
            "exact" => {
                let name_var = splitted.next().unwrap();
                lambdaterme = lambdaterme.exact(name_var.to_string());
            }
            "cut" => {
                todo!()
            }
            "absurd" => {
                todo!()
            }
            "apply" => {
                let name = splitted.next().unwrap();
                lambdaterme = lambdaterme.apply(name.to_string());
            }
            "elim" => {
                let name = splitted.next().unwrap();
                lambdaterme = lambdaterme.elim(name.to_string());
            }
            "+" => {
                goals_index = min(get_goal_count(lambdaterme.clone()), goals_index+1);
            }
            "-" => {
                goals_index = max(1, goals_index-1);
            }
            _ => println!("Unknown command.")
        }
    }
    println!();
    println!("Proof ended, no goal left !");
    println!("Final typecheck running :");
    let ok = lambdaterme.check(goal);
    if ok {
        println!("Checked the proof, yields the good type !");
    } else {
        panic!("Ehh there is an error somewhere");
    }
}

fn bfs_find_goals(root: LambdaTerm) -> Vec<(Type, Vec<LambdaTerm>)> {
    let mut queue: VecDeque<(LambdaTerm, Vec<LambdaTerm>)> = VecDeque::new();
    let mut goals: Vec<(Type, Vec<LambdaTerm>)> = Vec::new();

    queue.push_back((root.clone(), vec![root]));

    while let Some((current, path)) = queue.pop_front() {
        match current {
            LambdaTerm::Var(_) => {},
            LambdaTerm::Couple(ref left, ref right)
            | LambdaTerm::App(ref left, ref right) => {
                let mut left_path = path.clone();
                left_path.push(*left.clone());
                queue.push_back((*left.clone(), left_path));

                let mut right_path = path.clone();
                right_path.push(*right.clone());
                queue.push_back((*right.clone(), right_path));
            },
            LambdaTerm::Abs(_, _, ref body)
            | LambdaTerm::ExFalso(_, ref body) => {
                let mut new_path = path.clone();
                new_path.push(*body.clone());
                queue.push_back((*body.clone(), new_path));
            },
            LambdaTerm::Goal(ref ty) => {
                goals.push((ty.clone(), path.clone()));
            },
            LambdaTerm::Fst(_) | LambdaTerm::Snd(_) => {},
        }
    }

    goals
}

fn get_goal_count(lambda: LambdaTerm) -> usize {
    let mut total = 0;
    match lambda {
        LambdaTerm::Goal(..) => {
            total += 1
        }
        LambdaTerm::Var(..) | LambdaTerm::Fst(..) | LambdaTerm::Snd(..) => {},
        LambdaTerm::Couple(box term1, box term2) => {
            total += get_goal_count(term1);
            total += get_goal_count(term2);
        }
        LambdaTerm::App(box first, box second) => {
            total += get_goal_count(first);
            total += get_goal_count(second);
        }
        LambdaTerm::Abs(_str, _typ, box lambdaterm) => {
            total += get_goal_count(lambdaterm);
        }
        LambdaTerm::ExFalso(_t, box proof) => {
            total += get_goal_count(proof);
        }
    }
    total
}

#[allow(dead_code)]
fn get_goal() -> Type {
    loop {
        let mut input = String::new();
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("Failed to read line for some reason");
        input = input.trim().to_string();
        let mut splitted = input.split_whitespace().collect::<Vec<&str>>().into_iter();
        if splitted.len() == 0 {
            continue;
        }
        let first = splitted.next().unwrap();
        match first {
            "Goal" => {
                let rest = splitted.collect::<Vec<&str>>().concat();
                let parse_result = SimpleParser::parse(
                    Rule::main, 
                    &rest
                );
                let mut val = match parse_result {
                    Ok(parsed) => parsed,
                    Err(_) => {
                        println!("Invalid goal, please retry");
                        continue;
                    },
                };
                let typ = parse_type(val.next().unwrap());
                println!("Goal choosen.");
                return typ;
            }
            _ => println!("Unknown command.")
        }
    }
}

#[cfg(test)]
mod tests;

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
            Type::Imp(Box::new(first), Box::new(second))
        }
        Rule::and_type => {
            let mut inner = pair.into_inner();
            let first = parse_type(inner.next().unwrap());
            let second = parse_type(inner.next().unwrap());
            Type::And(Box::new(first), Box::new(second))
        }
        Rule::not_type => {
            let inner = pair.into_inner().next().unwrap();
            let ty = parse_type(inner);
            Type::Not(Box::new(ty))
        }
        Rule::bottom => Type::Bottom,
        Rule::top => Type::Top,
        _ => unreachable!(),
    }
}