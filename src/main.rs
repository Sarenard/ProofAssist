#![feature(box_patterns)]

use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{self, BufReader, Write};

#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "patterns.pest"] // relative to src
#[allow(dead_code)]
struct SimpleParser;

#[cfg(test)]
mod tests;

mod assistant;

use assistant::{
    operations::OP,
    lambda::LambdaTerm as LambdaTerm,
    lambdas::update_nbs::update_goals_nb as update_goals_nb,
};

static DEBUG: bool = true;

fn main() {
    // let goal = get_goal();
    // Goal forall A B C:Set, A = B -> B = C -> A = C. intros A B C h1 h2. rewrite h1. exact h2.
    let goal = LambdaTerm::pi(
        "A".to_string(),
        LambdaTerm::types(),
        LambdaTerm::pi(
            "B".to_string(),
            LambdaTerm::types(),
            LambdaTerm::pi(
                "C".to_string(),
                LambdaTerm::types(),
                LambdaTerm::imp(
                    LambdaTerm::eq(
                        LambdaTerm::var("A"),
                        LambdaTerm::var("B")
                    ),
                    LambdaTerm::imp(
                        LambdaTerm::eq(
                            LambdaTerm::var("B"),
                            LambdaTerm::var("C")
                        ),
                        LambdaTerm::eq(
                            LambdaTerm::var("A"),
                            LambdaTerm::var("C")
                        )
                    )
                )
            )
        )
    );

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
        lambdaterme = update_goals_nb(lambdaterme.clone(), &mut 1);
        if real {
            println!(); // to be beautiful
            print_hyp(lambdaterme.clone(), hypothesis.clone());
            get_command(&mut operations);
        }
        let last_op = operations.last().unwrap().clone();
        if DEBUG {println!("OLD : {:?}", lambdaterme);}
        (lambdaterme, hypothesis, operations) = run_command(last_op, lambdaterme, hypothesis, operations, true);
        if DEBUG {println!("NEW : {:?}", lambdaterme);}
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
            LambdaTerm::Func(name, box typ, _index) => {
                hypotheses.insert(name, typ);
            }
            _ => {}
        }
    }
    if theorems.len() > 0 {
        println!("Theorems :");
    }
    for (name, (typ, _op)) in theorems.iter() {
        println!("{} : {}", name, typ);
    }
    if !hypotheses.is_empty() {
        println!("Hypotheses :");
    }
    for (txt, typ) in hypotheses.iter() {
        println!("{} : {}", txt, typ);
    }
    println!("{}", format!("=============== {}/{}", 1, get_goal_count(lambdaterme.clone())));
    println!("{}", result.0);
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
                Rule::lambdaexpr, 
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
                    _ => {
                        new_operations.push(op)
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
        },
        OP::Intro => {
            let (_name, new_lambdaterm) = lambdaterme.intro();
            (new_lambdaterm, hypothesis, operations)
        }
        OP::Intros => {
            let (_name, new_lambdaterm) = lambdaterme.intros();
            (new_lambdaterm, hypothesis, operations)
        }
        OP::Apply(name) => {
            let new_lambdaterm = lambdaterme.apply(name, HashMap::new());
            (new_lambdaterm, hypothesis, operations)
        }
        OP::Assumption => {
            let new_lambdaterm = lambdaterme.assumption();
            (new_lambdaterm, hypothesis, operations)
        }
        OP::Exists(name) => {
            todo!()
        }
        OP::Split => {
            let new_lambdaterm = lambdaterme.split();
            (new_lambdaterm, hypothesis, operations)
        }
        OP::Elim(name) => {
            let new_lambdaterm = lambdaterme.elim(name);
            (new_lambdaterm, hypothesis, operations)
        },
        OP::Left => {
            let new_lambdaterm = lambdaterme.run_left();
            (new_lambdaterm, hypothesis, operations)
        }
        OP::Right => {
            let new_lambdaterm = lambdaterme.run_right();
            (new_lambdaterm, hypothesis, operations)
        }
        OP::Refl => {
            let new_lambdaterm = lambdaterme.refl_run();
            (new_lambdaterm, hypothesis, operations)
        }
        OP::Rewrite(name) => {
            let new_lambdaterm = lambdaterme.rewrite_run(name);
            (new_lambdaterm, hypothesis, operations)
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
        "intro" => {
            operations.push(OP::Intro)
        }
        "intros" => {
            operations.push(OP::Intros)
        }
        "left" => {
            operations.push(OP::Left)
        }
        "right" => {
            operations.push(OP::Right)
        }
        "assu" => {
            operations.push(OP::Assumption)
        }
        "refl" => {
            operations.push(OP::Refl)
        }
        "apply" => {
            let name = splitted.next().unwrap();
            operations.push(OP::Apply(name.to_string()))
        }
        "rewrite" => {
            let name = splitted.next().unwrap();
            operations.push(OP::Rewrite(name.to_string()))
        }
        "exists" => {
            let name = splitted.next().unwrap();
            operations.push(OP::Exists(name.to_string()))
        }
        "split" => {
            operations.push(OP::Split)
        }
        "elim" => {
            let name = splitted.next().unwrap();
            operations.push(OP::Elim(name.to_string()))
        }
        _ => {
            println!("Command unknown.");
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
    let line = format!("{:?}", goal);
    writeln!(theorem_file, "{}", line).unwrap();
    for op in operations {
        match op {
            OP::Nothing => {

            }
            OP::Use(name) => {
                writeln!(theorem_file, "Use(\"{}\")", name).unwrap();
            }
            OP::Elim(name) => {
                writeln!(theorem_file, "Elim(\"{}\")", name).unwrap();
            }
            OP::Load(name) => {
                writeln!(theorem_file, "Load(\"{}\")", name).unwrap();
            }
            OP::Intro => {
                writeln!(theorem_file, "Intro").unwrap();
            }
            OP::Rewrite(name) => {
                writeln!(theorem_file, "Rewrite(\"{}\")", name).unwrap();
            }
            OP::Left => {
                writeln!(theorem_file, "Left").unwrap();
            }
            OP::Refl => {
                writeln!(theorem_file, "Refl").unwrap();
            }
            OP::Right => {
                writeln!(theorem_file, "Right").unwrap();
            }
            OP::Assumption => {
                writeln!(theorem_file, "Assumption").unwrap();
            }
            OP::Split => {
                writeln!(theorem_file, "Split").unwrap();
            }
            OP::Apply(name) => {
                writeln!(theorem_file, "Apply(\"{}\")", name).unwrap();
            }
            OP::Intros => {
                writeln!(theorem_file, "Intros").unwrap();
            }
            OP::Exists(name) => {
                writeln!(theorem_file, "Exists(\"{}\")", name).unwrap();
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
            LambdaTerm::Var(..)
            | LambdaTerm::Types
            | LambdaTerm::Top
            | LambdaTerm::Bot
            | LambdaTerm::Error => {}
            LambdaTerm::Goal(box ref ty, _nb) => {
                goals.push((ty.clone(), path.clone()));
            },
            LambdaTerm::Func(_name, ref left, ref right)
            | LambdaTerm::Sigma(_name, ref left, ref right)
                | LambdaTerm::Pi(_name, ref left, ref right) => {
                let mut left_path = path.clone();
                left_path.push(*left.clone());
                queue.push_back((*left.clone(), left_path));

                let mut right_path = path.clone();
                right_path.push(*right.clone());
                queue.push_back((*right.clone(), right_path));
            },
            LambdaTerm::Refl(ref main) => {
                let mut main_path = path.clone();
                main_path.push(*main.clone());
                queue.push_back((*main.clone(), main_path));
            }
            LambdaTerm::Proj(ref left, ref right)
            | LambdaTerm::ExFalso(ref left, ref right)
            | LambdaTerm::Eq(ref left, ref right)
            | LambdaTerm::Or(ref left, ref right)
            | LambdaTerm::Left(ref left, ref right)
            | LambdaTerm::Right(ref left, ref right)
            | LambdaTerm::App(ref left, ref right) => {
                let mut left_path = path.clone();
                left_path.push(*left.clone());
                queue.push_back((*left.clone(), left_path));

                let mut right_path = path.clone();
                right_path.push(*right.clone());
                queue.push_back((*right.clone(), right_path));
            },
            LambdaTerm::Match(ref left, ref center, ref right)
            | LambdaTerm::Rewrite(ref left, ref center, ref right)
            | LambdaTerm::Couple(ref left, ref center, ref right) => {
                let mut left_path = path.clone();
                left_path.push(*left.clone());
                queue.push_back((*left.clone(), left_path));

                let mut center_path = path.clone();
                center_path.push(*center.clone());
                queue.push_back((*center.clone(), center_path));

                let mut right_path = path.clone();
                right_path.push(*right.clone());
                queue.push_back((*right.clone(), right_path));
            }
        }
    }

    goals
}

pub fn get_goal_count(lambda: LambdaTerm) -> usize {
    bfs_find_goals(lambda).len()
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
                    Rule::lambdaexpr, 
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

fn parse_lambdaterm(_pair: pest::iterators::Pair<Rule>) -> LambdaTerm {
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
        Rule::Intro => {
            OP::Intro
        }
        Rule::Assumption => {
            OP::Assumption
        }
        other => panic!("Other : {:?}", other),
    }
}