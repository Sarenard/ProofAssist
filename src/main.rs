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
use assistant::types::Type as Type;

fn main() {
    // let goal = get_goal();
    let goal = Type::imp(
        Type::or(
            Type::var("a"),
            Type::var("b"),
        ),
        Type::or(
            Type::var("b"),
            Type::var("a"),
        ),
    ).removenot();
    /*
    let goal = Type::imp(
        Type::var("a"),
        Type::imp(
            Type::var("b"),
            Type::or(
                Type::var("b"),
                Type::var("a"),
            ),
        )
    ).removenot();
    */

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

fn emulate(goal: Type, real: bool) -> (LambdaTerm, Vec<OP>) {
    let mut operations: Vec<OP> = vec![];

    let mut lambdaterme = LambdaTerm::Goal(goal.clone(), 0);

    let mut hypothesis : HashMap<String, (Type, Vec<OP>)> = HashMap::new();

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

fn print_hyp(lambdaterme: LambdaTerm, theorems: HashMap<String, (Type, Vec<OP>)>) {
    let goals = bfs_find_goals(lambdaterme.clone());
    // we get the good one
    let paths: Vec<(Type, Vec<LambdaTerm>)> = goals.iter().cloned()
        .filter(|x| match x.1.last().unwrap().clone() {LambdaTerm::Goal(_, i) => i == 1, _ => false}).collect();
    let result = paths.first().unwrap().clone();
    let mut hypotheses: HashMap<String, Type> = HashMap::new();
    for elt in result.1 {
        match elt {
            LambdaTerm::Abs(name, typ, _) => {
                hypotheses.insert(name, typ);
            }
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

fn run_command(op: OP, lambdaterme: LambdaTerm, hypothesis: HashMap<String, (Type, Vec<OP>)>, operations: Vec<OP>, real: bool) 
-> (LambdaTerm, HashMap<String, (Type, Vec<OP>)>, Vec<OP>) {
    match op {
        OP::Nothing => {
            (lambdaterme, hypothesis, operations)
        }
        OP::Assumption => {
            (lambdaterme.assumption(), hypothesis, operations)
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
                    OP::Left => {
                        new_operations.push(OP::Left)
                    }
                    OP::Right => {
                        new_operations.push(OP::Right)
                    }
                    OP::Intro => {
                        new_operations.push(OP::Intro);
                    }
                    OP::Use(name) => {
                        new_operations.push(OP::Use(name));
                    }
                    OP::Introv(name) => {
                        new_operations.push(OP::Introv(name));
                    }
                    OP::Intros => {
                        new_operations.push(OP::Intros);
                    }
                    OP::Split => {
                        new_operations.push(OP::Split);
                    }
                    OP::Exact(_name) => {
                        panic!("Cant use Exact in theorem did you want to use assumtion!");
                    }
                    OP::Cut(_typ) => {
                        todo!()
                    }
                    OP::Absurd(_typ) => {
                        todo!()
                    }
                    OP::Apply(name) => {
                        new_operations.push(OP::Apply(name));
                    }
                    OP::Elim(name) => {
                        new_operations.push(OP::Elim(name));
                    }
                    OP::Assumption => {
                        new_operations.push(OP::Assumption);
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
        OP::Intro => {
            let (_name, new_lambdaterme) = lambdaterme.intro();
            (new_lambdaterme, hypothesis, operations)
        }
        OP::Introv(name) => {
            let lt = lambdaterme.introv(name.to_string());
            (lt, hypothesis, operations)
        }
        OP::Intros => {
            let (_names, new_lambdaterme) = lambdaterme.intros();
            (new_lambdaterme, hypothesis, operations)
        }
        OP::Split => {
            (lambdaterme.split(), hypothesis, operations)
        }
        OP::Left => {
            (lambdaterme.left(), hypothesis, operations)
        }
        OP::Right => {
            (lambdaterme.right(), hypothesis, operations)
        }
        OP::Exact(name) => {
            (lambdaterme.exact(name.to_string()), hypothesis, operations)
        }
        OP::Cut(_typ) => {
            todo!()
        }
        OP::Absurd(_typ) => {
            todo!()
        }
        OP::Apply(name) => {
            (lambdaterme.apply(name.to_string()), hypothesis, operations)
        }
        OP::Elim(name) => {
            (lambdaterme.elim(name.to_string()), hypothesis, operations)
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
        "left" => {
            operations.push(OP::Left)
        }
        "right" => {
            operations.push(OP::Right)
        }
        "assu" => {
            operations.push(OP::Assumption);
        }
        "use" => {
            let theorem_name = splitted.next().unwrap();
            operations.push(OP::Use(theorem_name.to_string()))
        }
        "load" => {
            let theorem_name = splitted.next().unwrap();
            operations.push(OP::Load(theorem_name.to_string()))
        }
        "intro" => {
            let name_var = splitted.next();
            match name_var {
                None => {
                    operations.push(OP::Intro);
                }
                Some(name) => {
                    operations.push(OP::Introv(name.to_string()));
                }
            }
        }
        "intros" => {
            operations.push(OP::Intros);
            
        }
        "split" => {
            operations.push(OP::Split);
        }
        "exact" => {
            let name_var = splitted.next().unwrap();
            operations.push(OP::Exact(name_var.to_string()));
        }
        "cut" => {
            todo!()
        }
        "absurd" => {
            todo!()
        }
        "apply" => {
            let name = splitted.next().unwrap();
            operations.push(OP::Apply(name.to_string()));
        }
        "elim" => {
            let name = splitted.next().unwrap();
            operations.push(OP::Elim(name.to_string()));
        }
        _ => {
            println!("Unknown command.");
            operations.push(OP::Nothing)
        }
    }
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
            OP::Nothing => {

            }
            OP::Left => {
                writeln!(theorem_file, "Left").unwrap();
            }
            OP::Right => {
                writeln!(theorem_file, "Right").unwrap();
            }
            OP::Assumption => {
                writeln!(theorem_file, "Assumption").unwrap();
            }
            OP::Intro => {
                writeln!(theorem_file, "Intro").unwrap();
            }
            OP::Use(name) => {
                writeln!(theorem_file, "Use(\"{}\")", name).unwrap();
            }
            OP::Introv(name) => {
                writeln!(theorem_file, "Introv(\"{}\")", name).unwrap();
            }
            OP::Intros => {
                writeln!(theorem_file, "Intros").unwrap();
            }
            OP::Split => {
                writeln!(theorem_file, "Split").unwrap();
            }
            OP::Exact(name) => {
                writeln!(theorem_file, "Exact(\"{}\")", name).unwrap();
            }
            OP::Cut(_typ) => {
                todo!()
            }
            OP::Absurd(_typ) => {
                todo!()
            }
            OP::Apply(name) => {
                writeln!(theorem_file, "Apply(\"{}\")", name).unwrap();
            }
            OP::Elim(name) => {
                writeln!(theorem_file, "Elim(\"{}\")", name).unwrap();
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
            | LambdaTerm::Match(_, ref left, ref right)
            | LambdaTerm::App(ref left, ref right) => {
                let mut left_path = path.clone();
                left_path.push(*left.clone());
                queue.push_back((*left.clone(), left_path));

                let mut right_path = path.clone();
                right_path.push(*right.clone());
                queue.push_back((*right.clone(), right_path));
            },
            LambdaTerm::Left(ref body, _)
            | LambdaTerm::Right(ref body, _)
            | LambdaTerm::Abs(_, _, ref body)
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
        LambdaTerm::Match(_, box term1, box term2) => {
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
        LambdaTerm::Left(box lambda, _) => {
            total += get_goal_count(lambda);
        }
        LambdaTerm::Right(box lambda, _) => {
            total += get_goal_count(lambda);
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
        Rule::or_type => {
            let mut inner = pair.into_inner();
            let first = parse_type(inner.next().unwrap());
            let second = parse_type(inner.next().unwrap());
            Type::Or(Box::new(first), Box::new(second))
        }
        Rule::bottom => Type::Bottom,
        Rule::top => Type::Top,
        _ => unreachable!(),
    }
}

fn parse_op(pair: pest::iterators::Pair<Rule>) -> OP {
    match pair.as_rule() {
        Rule::Intro => {
            OP::Intro
        }
        Rule::Introv => {
            let mut inner = pair.into_inner();
            let text = inner.next().unwrap().as_str().to_string();
            let text = text.chars().skip(1).take(text.chars().count() - 2).collect();
            OP::Introv(text)
        }
        Rule::Intros => {
            OP::Intros
        }
        Rule::Split => {
            OP::Split
        }
        Rule::Exact => {
            let mut inner = pair.into_inner();
            let text = inner.next().unwrap().as_str().to_string();
            let text = text.chars().skip(1).take(text.chars().count() - 2).collect();
            OP::Exact(text)
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
            let text = text.chars().skip(1).take(text.chars().count() - 2).collect();
            OP::Apply(text)
        }
        Rule::Elim => {
            let mut inner = pair.into_inner();
            let text = inner.next().unwrap().as_str();
            let text = text.chars().skip(1).take(text.chars().count() - 2).collect();
            OP::Elim(text)
        }
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
        Rule::Assumption => {
            OP::Assumption
        }
        Rule::Left => {
            OP::Left
        }
        Rule::Right => {
            OP::Right
        }
        other => panic!("Other : {:?}", other),
    }
}