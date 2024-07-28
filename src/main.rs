#![feature(box_patterns)]

use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{self, BufReader, Write};

#[macro_use]
extern crate pest_derive;

use assistant::operations::OP;
use pest::Parser;

#[derive(Parser)]
#[grammar = "patterns.pest"] // relative to src
#[allow(dead_code)]
struct SimpleParser;

mod assistant;

#[cfg(test)]
mod tests;

use assistant::lambda::{rebuild_tree, LambdaTerm as LambdaTerm};

fn main() {
    let goal = get_goal();

    let (lambdaterme, operations) = emulate(goal.clone(), true);

    println!();
    println!("Proof ended, no goal left !");
    println!("Final typecheck running :");
    let ok = lambdaterme.clone().check(goal.clone());
    if ok {
        println!("Checked the proof, yields the good type !");
    } else {
        panic!("Ehh there is an error somewhere : {:?}", lambdaterme);
    }
    print!("Do you want to save? (Y/N) : ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line for some reason");
    input = input.trim().to_string();
    if ["Y".to_string(), "y".to_string()].contains(&input) {
        save(goal, operations);
    }
}

fn emulate(goal: LambdaTerm, real: bool) -> (LambdaTerm, Vec<OP>) {
    let mut operations: Vec<OP> = vec![];

    let mut lambdaterme = LambdaTerm::Goal(Box::new(goal.clone()), 0);

    let mut hypothesis : HashMap<String, (LambdaTerm, Vec<OP>)> = HashMap::new();

    while lambdaterme.clone().containsgoal() {
        // to fix problems
        lambdaterme = rebuild_tree(lambdaterme.clone(), &mut 1);
        if real {
            println!(); // to be beautiful
            print_hyp(lambdaterme.clone(), hypothesis.clone());
            get_command(&mut operations);
        }
        let last_op = operations.last().unwrap().clone();
        println!("{:?}", lambdaterme);
        (lambdaterme, hypothesis, operations) = run_command(last_op, lambdaterme, hypothesis, operations, true);
        println!("{:?}", lambdaterme);
    }

    (lambdaterme, operations)
}

fn print_hyp(lambdaterme: LambdaTerm, theorems: HashMap<String, (LambdaTerm, Vec<OP>)>) {
    let goals = bfs_find_goals(lambdaterme.clone());
    // we get the good one
    let paths: Vec<(LambdaTerm, Vec<LambdaTerm>)> = goals.iter().cloned()
        .filter(|x| match x.1.last().unwrap().clone() {LambdaTerm::Goal(_, i) => i == 1, _ => false}).collect();
    let result = paths.first().unwrap().clone();
    let mut hypotheses: HashMap<String, LambdaTerm> = HashMap::new();
    for elt in result.1 {
        match elt {
            _ => {}
        }
    }
    if theorems.len() > 0 {
        println!("Theorems :");
    }
    for (name, (typ, _op)) in theorems.iter() {
        println!("{} : {:?}", name, typ);
    }
    if !hypotheses.is_empty() {
        println!("Hypotheses :");
    }
    for (txt, typ) in hypotheses.iter() {
        println!("{} : {:?}", txt, typ);
    }
    println!("{}", format!("=============== {}/{}", 1, get_goal_count(lambdaterme.clone())));
    println!("{:?}", result.0);
}

fn run_command(op: OP, lambdaterme: LambdaTerm, hypothesis: HashMap<String, (LambdaTerm, Vec<OP>)>, operations: Vec<OP>, real: bool) 
-> (LambdaTerm, HashMap<String, (LambdaTerm, Vec<OP>)>, Vec<OP>) {
    match op {
        OP::Nothing => {
            (lambdaterme, hypothesis, operations)
        }
        OP::Load(name) => {
            use std::io::BufRead;
            let file = File::open(format!("./theorems/{}.th", name)).unwrap();
            let reader = BufReader::new(file);
            let mut lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();
            if real {
                println!("{}", format!("Theorem {} loaded.", name));
            }
            let first = lines.remove(0);
            let parse_result = SimpleParser::parse(
                Rule::typ, 
                first.as_str()
            );
            let mut val = parse_result.unwrap();
            let local_goal = parse_lambdaterm(val.next().unwrap());
            let mut proof_ops: Vec<OP> = vec![];
            for line in lines {
                let mut parse_result = SimpleParser::parse(
                    Rule::OP, 
                    line.as_str()
                ).unwrap();
                let op = parse_op(parse_result.next().unwrap());
                proof_ops.push(op)
            }
            let mut new_hypotheses = hypothesis.clone();
            new_hypotheses.insert(name, (local_goal.clone(), proof_ops));

            (lambdaterme, new_hypotheses, operations)
        }
        OP::Use(name) => {
            let (output_type, ops) = hypothesis.get(&name).unwrap().clone();

            let goals = bfs_find_goals(lambdaterme.clone());

            let (goal_type, _path) = goals[0].clone();

            if goal_type != output_type {
                println!("Error, theorem unusable.");
                return (lambdaterme, hypothesis, operations);
            }

            let mut new_operations = vec![];

            for op in ops {
                match op {
                    OP::Nothing => {

                    }
                    OP::Use(name) => {
                        new_operations.push(OP::Use(name));
                    }
                    OP::Load(name) => {
                        new_operations.push(OP::Load(name));
                    }
                }
            }

            let mut c_lambdaterme = lambdaterme.clone();
            let mut c_operations = operations.clone();
            let mut c_hypothesis = hypothesis.clone();

            for op in new_operations {
                (c_lambdaterme, c_hypothesis, c_operations) = run_command(op, c_lambdaterme, c_hypothesis, c_operations, false);
            }
            
            (c_lambdaterme, c_hypothesis, c_operations)
        }
    }
}

fn get_command(operations: &mut Vec<OP>) {
    let mut input = String::new();
    print!("> ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed to read line for some reason");
    input = input.trim().to_string();
    let mut splitted = input.split_whitespace().collect::<Vec<&str>>().into_iter();
    let command = splitted.next().unwrap();
    match command {
        "use" => {
            let theorem_name = splitted.next().unwrap();
            operations.push(OP::Use(theorem_name.to_string()))
        }
        "load" => {
            let theorem_name = splitted.next().unwrap();
            operations.push(OP::Load(theorem_name.to_string()))
        }
        _ => {
            println!("Unknown command.");
            operations.push(OP::Nothing)
        }
    }
}

fn save(goal: LambdaTerm, operations: Vec<OP>) {
    println!("Saving...");
    print!("Name of the theorem : ");
    io::stdout().flush().unwrap();
    let mut theorem_name = String::new();
    io::stdin().read_line(&mut theorem_name).expect("Failed to read line for some reason");
    theorem_name = theorem_name.trim().to_string();
    let theorem_name = format!("./theorems/{}.th", theorem_name);
    let mut theorem_file = File::create(theorem_name).expect("Error !");
    let line = format!("{}", goal);
    writeln!(theorem_file, "{}", line).unwrap();
    for op in operations {
        match op {
            OP::Nothing => {

            }
            OP::Use(name) => {
                writeln!(theorem_file, "Use(\"{}\")", name).unwrap();
            }
            OP::Load(name) => {
                writeln!(theorem_file, "Load(\"{}\")", name).unwrap();
            }
        }
    }
}

fn bfs_find_goals(root: LambdaTerm) -> Vec<(LambdaTerm, Vec<LambdaTerm>)> {
    let mut queue: VecDeque<(LambdaTerm, Vec<LambdaTerm>)> = VecDeque::new();
    let mut goals: Vec<(LambdaTerm, Vec<LambdaTerm>)> = Vec::new();

    queue.push_back((root.clone(), vec![root]));

    while let Some((current, path)) = queue.pop_front() {
        match current {
            LambdaTerm::Goal(box ref ty, _nb) => {
                goals.push((ty.clone(), path.clone()));
            },
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
    }
    total
}

#[allow(dead_code)]
fn get_goal() -> LambdaTerm {
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
                    Rule::typ, 
                    &rest
                );
                let mut val = match parse_result {
                    Ok(parsed) => parsed,
                    Err(_) => {
                        println!("Invalid goal, please retry");
                        continue;
                    },
                };
                let typ = parse_lambdaterm(val.next().unwrap());
                println!("Goal choosen.");
                return typ;
            }
            _ => println!("Unknown command.")
        }
    }
}

fn parse_lambdaterm(pair: pest::iterators::Pair<Rule>) -> LambdaTerm {
    todo!()
}

fn parse_op(pair: pest::iterators::Pair<Rule>) -> OP {
    match pair.as_rule() {
        Rule::Load => {
            let mut inner = pair.into_inner();
            let text = inner.next().unwrap().as_str().to_string();
            let text = text.chars().skip(1).take(text.chars().count() - 2).collect();
            OP::Load(text)
        }
        Rule::Use => {
            let mut inner = pair.into_inner();
            let text = inner.next().unwrap().as_str().to_string();
            let text = text.chars().skip(1).take(text.chars().count() - 2).collect();
            OP::Use(text)
        }
        other => panic!("Other : {:?}", other),
    }
}