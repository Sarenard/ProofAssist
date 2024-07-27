#![feature(box_patterns)]

use std::cmp::{max, min};
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

use assistant::lambda::LambdaTerm as LambdaTerm;
use assistant::types::Type as Type;

fn main() {
    // let goal = get_goal();
    let goal = Type::Imp(
        Box::new(Type::Var("A".to_string())),
        Box::new(Type::Imp(
            Box::new(Type::Var("B".to_string())),
            Box::new(Type::Var("A".to_string()))
        )),
    ).removenot();

    emulate(goal, true);
}

fn emulate(goal: Type, real: bool) {
    let mut goals_index = 1;

    let mut operations: Vec<OP> = vec![];

    let mut lambdaterme = LambdaTerm::Goal(goal.clone(), 0);

    let mut hypothesis : HashMap<Type, Vec<OP>> = HashMap::new();

    while lambdaterme.clone().containsgoal() {
        // to fix problems
        goals_index = min(goals_index, get_goal_count(lambdaterme.clone()));
        if real {
            println!(); // to be beautiful
            print_hyp(lambdaterme.clone(), goals_index, hypothesis.clone());
            lambdaterme = get_command(lambdaterme, &mut operations, goals_index);

        }
        let last_op = operations.last().unwrap().clone();
        (goals_index, lambdaterme, hypothesis) = run_command(last_op, goals_index, lambdaterme, hypothesis);
    }
    if real {
        println!();
        println!("Proof ended, no goal left !");
        println!("{:?}", operations);
        println!("Final typecheck running :");
    }
    let ok = lambdaterme.check(goal.clone());
    if ok {
        if real {
            println!("Checked the proof, yields the good type !");
        }
    } else {
        panic!("Ehh there is an error somewhere");
    }
    if real {
        print!("Do you want to save? (Y/N) : ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line for some reason");
        input = input.trim().to_string();
        if ["Y".to_string(), "y".to_string()].contains(&input) {
            save(goal, operations);
        }
    }
}

fn print_hyp(lambdaterme: LambdaTerm, goals_index: usize, theorems: HashMap<Type, Vec<OP>>) {
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
    for (typ, _operations) in theorems.iter() {
        println!("Theorem : {:?}", typ);
    }
    for (txt, typ) in hypotheses.iter() {
        println!("{} : {:?}", txt, typ);
    }
    println!("{}", format!("=============== {}/{}", goals_index, get_goal_count(lambdaterme.clone())));
    println!("{:?}", goals[goals_index-1].0);
}

fn run_command(op: OP, goals_index: usize, lambdaterme: LambdaTerm, hypothesis: HashMap<Type, Vec<OP>>) -> (usize, LambdaTerm, HashMap<Type, Vec<OP>>) {
    match op {
        OP::Load(name) => {
            use std::io::BufRead;
            let file = File::open(format!("./theorems/{}.th", name)).unwrap();
            let reader = BufReader::new(file);
            let mut lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();
            println!("{}", format!("Theorem {} loaded.", name));
            let first = lines.remove(0);
            let parse_result = SimpleParser::parse(
                Rule::typ, 
                first.as_str()
            );
            let mut val = parse_result.unwrap();
            let local_goal = parse_type(val.next().unwrap());
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
            new_hypotheses.insert(local_goal, proof_ops);
            (goals_index, lambdaterme, new_hypotheses)
        }
        OP::Intro(index) => {
            let (_name, new_lambdaterme) = lambdaterme.intro(index);
            (goals_index, new_lambdaterme, hypothesis)
        }
        OP::Introv(name, index) => {
            let lt = lambdaterme.introv(name.to_string(), index);
            (goals_index, lt, hypothesis)
        }
        OP::Intros(index) => {
            let (_names, new_lambdaterme) = lambdaterme.intros(index);
            (goals_index, new_lambdaterme, hypothesis)
        }
        OP::Split(index) => {
            (goals_index, lambdaterme.split(index), hypothesis)
        }
        OP::Exact(name, index) => {
            (goals_index, lambdaterme.exact(name.to_string(), index), hypothesis)
        }
        OP::Cut(_typ) => {
            todo!()
        }
        OP::Absurd(_typ) => {
            todo!()
        }
        OP::Apply(name, index) => {
            (goals_index, lambdaterme.apply(name.to_string(), index), hypothesis)
        }
        OP::Elim(name, index) => {
            (goals_index, lambdaterme.elim(name.to_string(), index), hypothesis)
        }
        OP::Add => {
            (min(get_goal_count(lambdaterme.clone()), goals_index+1), lambdaterme, hypothesis)
        }
        OP::Sub => {
            (max(1, goals_index-1), lambdaterme, hypothesis)
        }
    }
}

fn get_command(lambdaterme: LambdaTerm, operations: &mut Vec<OP>, goals_index: usize) -> LambdaTerm {
    let mut input = String::new();
    print!("> ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed to read line for some reason");
    input = input.trim().to_string();
    let mut splitted = input.split_whitespace().collect::<Vec<&str>>().into_iter();
    let command = splitted.next().unwrap();
    match command {
        "load" => {
            let theorem_name = splitted.next().unwrap();
            operations.push(OP::Load(theorem_name.to_string()))
        }
        "intro" => {
            let name_var = splitted.next();
            match name_var {
                None => {
                    operations.push(OP::Intro(goals_index));
                }
                Some(name) => {
                    operations.push(OP::Introv(name.to_string(), goals_index));
                }
            }
        }
        "intros" => {
            operations.push(OP::Intros(goals_index));
            
        }
        "split" => {
            operations.push(OP::Split(goals_index));
        }
        "exact" => {
            let name_var = splitted.next().unwrap();
            operations.push(OP::Exact(name_var.to_string(), goals_index));
        }
        "cut" => {
            todo!()
        }
        "absurd" => {
            todo!()
        }
        "apply" => {
            let name = splitted.next().unwrap();
            operations.push(OP::Apply(name.to_string(), goals_index));
        }
        "elim" => {
            let name = splitted.next().unwrap();
            operations.push(OP::Elim(name.to_string(), goals_index));
        }
        "+" => {
            operations.push(OP::Add);
        }
        "-" => {
            operations.push(OP::Sub);
        }
        _ => println!("Unknown command.")
    }
    lambdaterme
}

fn save(goal: Type, operations: Vec<OP>) {
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
            OP::Intro(index) => {
                writeln!(theorem_file, "Intro({})", index).unwrap();
            }
            OP::Introv(name, index) => {
                writeln!(theorem_file, "Introv(\"{}\", {})", name, index).unwrap();
            }
            OP::Intros(index) => {
                writeln!(theorem_file, "Intros({})", index).unwrap();
            }
            OP::Split(index) => {
                writeln!(theorem_file, "Split({})", index).unwrap();
            }
            OP::Exact(name, index) => {
                writeln!(theorem_file, "Exact(\"{}\", {})", name, index).unwrap();
            }
            OP::Cut(_typ) => {
                todo!()
            }
            OP::Absurd(_typ) => {
                todo!()
            }
            OP::Apply(name, index) => {
                writeln!(theorem_file, "Apply(\"{}\", {})", name, index).unwrap();
            }
            OP::Elim(name, index) => {
                writeln!(theorem_file, "Elim(\"{}\", {})", name, index).unwrap();
            }
            OP::Add => {
                writeln!(theorem_file, "Add()").unwrap();
            }
            OP::Sub => {
                writeln!(theorem_file, "Sub()").unwrap();
            }
            OP::Load(name) => {
                writeln!(theorem_file, "Load(\"{}\")", name).unwrap();
            }
        }
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
            LambdaTerm::Goal(ref ty, _nb) => {
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
                let typ = parse_type(val.next().unwrap());
                println!("Goal choosen.");
                return typ;
            }
            _ => println!("Unknown command.")
        }
    }
}

fn parse_type(pair: pest::iterators::Pair<Rule>) -> Type {
    match pair.as_rule() {
        Rule::typ => {
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

fn parse_op(pair: pest::iterators::Pair<Rule>) -> OP {
    match pair.as_rule() {
        Rule::Intro => {
            let mut inner = pair.into_inner();
            let nb = inner.next().unwrap().as_str().parse::<usize>().unwrap();
            OP::Intro(nb)
        }
        Rule::Introv => {
            let mut inner = pair.into_inner();
            let text = inner.next().unwrap().as_str().to_string();
            let nb = inner.next().unwrap().as_str().parse::<usize>().unwrap();
            OP::Introv(text, nb)
        }
        Rule::Intros => {
            let mut inner = pair.into_inner();
            let nb = inner.next().unwrap().as_str().parse::<usize>().unwrap();
            OP::Intros(nb)
        }
        Rule::Split => {
            let mut inner = pair.into_inner();
            let nb = inner.next().unwrap().as_str().parse::<usize>().unwrap();
            OP::Split(nb)
        }
        Rule::Exact => {
            let mut inner = pair.into_inner();
            let text = inner.next().unwrap().as_str().to_string();
            let nb = inner.next().unwrap().as_str().parse::<usize>().unwrap();
            OP::Exact(text, nb)
        }
        Rule::Cut => {
            todo!()
        }
        Rule::Absurd => {
            todo!()
        }
        Rule::Apply => {
            let mut inner = pair.into_inner();
            let text = inner.next().unwrap().as_str().to_string();
            let nb = inner.next().unwrap().as_str().parse::<usize>().unwrap();
            OP::Apply(text, nb)
        }
        Rule::Elim => {
            let mut inner = pair.into_inner();
            let text = inner.next().unwrap().as_str().to_string();
            let nb = inner.next().unwrap().as_str().parse::<usize>().unwrap();
            OP::Elim(text, nb)
        }
        Rule::Load => {
            let mut inner = pair.into_inner();
            let text = inner.next().unwrap().as_str().to_string();
            OP::Load(text)
        }
        Rule::Add => OP::Add,
        Rule::Sub => OP::Sub,
        _ => unreachable!(),
    }
}