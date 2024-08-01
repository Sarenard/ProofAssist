use std::collections::HashMap;

use crate::assistant::{lambda::{self, LambdaTerm}, lambdas::{alpha_equiv::alpha_equiv, compute_type::compute_type, replace::replace}};

fn check(goal: LambdaTerm, lambdaterme: LambdaTerm) {
    if lambdaterme.clone().containsgoal() {
        panic!("Pas fini ! {:?}", lambdaterme);
    }
    println!("Checking...\n\n");
    let ok = lambdaterme.clone().check(goal.clone());
    if !ok {
        panic!("Ehh i'm wrong somewhere\n{:?}\n{:?}", goal, compute_type(lambdaterme, HashMap::new()));
    }
}

#[test]
// ∀ A:Prop, A = A
fn rewrite_test() {
    let goal = LambdaTerm::pi(
        "A".to_string(),
        LambdaTerm::types(),
        LambdaTerm::pi(
            "B".to_string(),
            LambdaTerm::types(),
            LambdaTerm::pi(
                "C".to_string(),
                LambdaTerm::types(),
                LambdaTerm::pi(
                    "free_name".to_string(), // TODO : fix this
                    LambdaTerm::eq(
                        LambdaTerm::var("A"),
                        LambdaTerm::var("B")
                    ),
                    LambdaTerm::pi(
                    "free_name2".to_string(),
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

    let lambdaterme = LambdaTerm::goal(goal.clone());
    println!("\nlambdaterme : {:?}\n", lambdaterme);
    let (names, lambdaterme) = lambdaterme.intros();
    println!("\nlambdaterme : {:?} {:?}\n", lambdaterme, names);
    let lambdaterme = lambdaterme.rewrite_run(names[3].clone());
    println!("\nlambdaterme : {:?}\n", lambdaterme);
    let lambdaterme = lambdaterme.assumption();
    println!("\nlambdaterme : {:?}\n", lambdaterme);

    check(goal, lambdaterme);
}

#[test]
// ∀ A:Prop, A = A
fn refl_test() {
    let goal = LambdaTerm::pi(
        "A".to_string(),
        LambdaTerm::types(),
        LambdaTerm::eq(
            LambdaTerm::var("A"),
            LambdaTerm::var("A"),
        )
    );

    let lambdaterme = LambdaTerm::goal(goal.clone());
    println!("\nlambdaterme : {:?}\n", lambdaterme);
    let (_, lambdaterme) = lambdaterme.intros();
    println!("\nlambdaterme : {:?}\n", lambdaterme);
    let lambdaterme = lambdaterme.refl_run();
    println!("\nlambdaterme : {:?}\n", lambdaterme);

    check(goal, lambdaterme);
}

#[test]
// ∀ A:Prop, ∀ B:Prop, (A or B) -> (B or A)
fn or_swap() {
    let goal = LambdaTerm::pi(
        "A".to_string(),
        LambdaTerm::types(),
        LambdaTerm::pi(
            "B".to_string(),
            LambdaTerm::types(),
            LambdaTerm::imp(
                LambdaTerm::or(LambdaTerm::var("A"), LambdaTerm::var("B")),
                LambdaTerm::or(LambdaTerm::var("B"), LambdaTerm::var("A")),
            )
        )
    );

    let lambdaterme = LambdaTerm::goal(goal.clone());
    println!("\nlambdaterme : {:?}\n{}\n", lambdaterme, lambdaterme);
    let (names, lambdaterme) = lambdaterme.intros();
    println!("\nlambdaterme : {:?}\n{}\n", lambdaterme, lambdaterme);
    let lambdaterme = lambdaterme.elim(names[2].clone());
    println!("\nlambdaterme : {:?}\n{}\n", lambdaterme, lambdaterme);
    let (_, lambdaterme) = lambdaterme.intro();
    println!("\nlambdaterme : {:?}\n{}\n", lambdaterme, lambdaterme);
    let lambdaterme = lambdaterme.run_right();
    println!("\nlambdaterme : {:?}\n{}\n", lambdaterme, lambdaterme);
    let lambdaterme = lambdaterme.assumption();
    println!("\nlambdaterme : {:?}\n{}\n", lambdaterme, lambdaterme);
    let (_, lambdaterme) = lambdaterme.intro();
    println!("\nlambdaterme : {:?}\n{}\n", lambdaterme, lambdaterme);
    let lambdaterme = lambdaterme.run_left();
    println!("\nlambdaterme : {:?}\n{}\n", lambdaterme, lambdaterme);
    let lambdaterme = lambdaterme.assumption();
    println!("\nlambdaterme : {:?}\n{}\n", lambdaterme, lambdaterme);
    

    check(goal, lambdaterme);
}

#[test]
// ∀ A:Prop, ∃ B:Prop, ~(A /\ B)
fn harder() {
    let goal = LambdaTerm::pi(
        "A".to_string(),
        LambdaTerm::Types,
        LambdaTerm::sigma(
            "B".to_string(),
            LambdaTerm::Types,
            LambdaTerm::not(
                LambdaTerm::and(LambdaTerm::var("A"), LambdaTerm::var("B"))
            )
        )
    );

    let lambdaterme = LambdaTerm::goal(goal.clone());
    println!("\nlambdaterme : {:?}\n", lambdaterme);
    let (name1, lambdaterme) = lambdaterme.intro();
    println!("\nlambdaterme : {:?}\n", lambdaterme);
    let terme_magique = LambdaTerm::not(LambdaTerm::Var(name1.clone()));
    let lambdaterme = lambdaterme.exists(terme_magique);
    println!("\nlambdaterme : {:?}\n", lambdaterme);
    let (names, lambdaterme) = lambdaterme.intros();
    println!("\nlambdaterme : {} {:?}\n", lambdaterme, names);
    let lambdaterme = lambdaterme.elim(names[0].clone());
    println!("\nlambdaterme : {}\n", lambdaterme);
    let (names2, lambdaterme) = lambdaterme.intros();
    println!("\nlambdaterme : {:?} {:?}\n", lambdaterme, names2);
    let hashmap: HashMap<String, LambdaTerm> = HashMap::new();
    let lambdaterme = lambdaterme.apply(names2[1].to_string(), hashmap);
    println!("\nlambdaterme : {:?} {:?}\n", lambdaterme, names2);
    let lambdaterme = lambdaterme.assumption();
    println!("\nlambdaterme : {:?} {:?}\n", lambdaterme, names2);

    check(goal, lambdaterme);
}

#[test]
// ∀ A:Prop, ∀ B:Prop, B -> A \/ B
fn right() {
    let goal = LambdaTerm::pi(
        "A".to_string(),
        LambdaTerm::Types,
        LambdaTerm::pi(
            "B".to_string(),
            LambdaTerm::Types,
            LambdaTerm::imp(
                LambdaTerm::var("B"),
                LambdaTerm::or(LambdaTerm::var("A"), LambdaTerm::var("B")),
            )
        )
    );

    let lambdaterme = LambdaTerm::goal(goal.clone());
    println!("\nlambdaterme : {:?}\n", lambdaterme);
    let (_, lambdaterme) = lambdaterme.intros();
    println!("\nlambdaterme : {:?}\n", lambdaterme);
    let lambdaterme = lambdaterme.run_right();
    println!("\nlambdaterme : {:?}\n", lambdaterme);
    let lambdaterme = lambdaterme.assumption();
    println!("\nlambdaterme : {:?}\n", lambdaterme);

    check(goal, lambdaterme);
}

#[test]
// ∀ A:Prop, ∀ B:Prop, A -> A \/ B
fn left() {
    let goal = LambdaTerm::pi(
        "A".to_string(),
        LambdaTerm::Types,
        LambdaTerm::pi(
            "B".to_string(),
            LambdaTerm::Types,
            LambdaTerm::imp(
                LambdaTerm::var("A"),
                LambdaTerm::or(LambdaTerm::var("A"), LambdaTerm::var("B")),
            )
        )
    );

    let lambdaterme = LambdaTerm::goal(goal.clone());
    println!("\nlambdaterme : {:?}\n", lambdaterme);
    let (_, lambdaterme) = lambdaterme.intros();
    println!("\nlambdaterme : {:?}\n", lambdaterme);
    let lambdaterme = lambdaterme.run_left();
    println!("\nlambdaterme : {:?}\n", lambdaterme);
    let lambdaterme = lambdaterme.assumption();
    println!("\nlambdaterme : {:?}\n", lambdaterme);

    check(goal, lambdaterme);
}

#[test]
// ∀ A:Prop, Bot -> A
fn absurd() {
    let goal = LambdaTerm::pi(
        "A".to_string(),
        LambdaTerm::Types,
            LambdaTerm::imp(
            LambdaTerm::Bot,
            LambdaTerm::var("A"),
        )
    );

    let lambdaterme = LambdaTerm::goal(goal.clone());
    println!("\nlambdaterme : {:?}\n", lambdaterme);
    let (names, lambdaterme) = lambdaterme.intros();
    println!("\nlambdaterme : {:?}\n", lambdaterme);
    let lambdaterme = lambdaterme.absurd(LambdaTerm::var(names[0].as_str()));
    println!("\nlambdaterme : {:?}\n", lambdaterme);
    let lambdaterme = lambdaterme.assumption();
    println!("\nlambdaterme : {:?}\n", lambdaterme);

    check(goal, lambdaterme);
}

#[test]
// ∀ A:Prop, (∀ B:Prop, B ) -> A
fn apply_forall() {
    let goal = LambdaTerm::pi(
        "A".to_string(),
        LambdaTerm::types(),
        LambdaTerm::imp(
            LambdaTerm::pi(
                "B".to_string(),
                LambdaTerm::types(),
                LambdaTerm::var("B")
            ),
            LambdaTerm::var("A")
        )
    );

    

    let lambdaterme = LambdaTerm::goal(goal.clone());
    println!("\nlambdaterme : {:?}\n", lambdaterme);
    let (names, lambdaterme) = lambdaterme.intros();
    println!("\nlambdaterme : {:?}\n", lambdaterme);
    let mut hashmap: HashMap<String, LambdaTerm> = HashMap::new();
    hashmap.insert("B".to_string(), LambdaTerm::var(names[0].as_str()));
    let lambdaterme = lambdaterme.apply(names[1].clone(), hashmap);
    println!("\nlambdaterme : {:?}\n", lambdaterme);

    check(goal, lambdaterme);
}

#[test]
// ∀ A:Prop, ∀ B:Prop A /\ B -> B /\ A
fn swap_and() {
    let goal = LambdaTerm::pi(
        "A".to_string(),
        LambdaTerm::types(),
        LambdaTerm::pi(
            "B".to_string(),
            LambdaTerm::types(),
            LambdaTerm::imp(
                LambdaTerm::and(
                    LambdaTerm::var("A"),
                    LambdaTerm::var("B"),
                ),
                LambdaTerm::and(
                    LambdaTerm::var("B"),
                    LambdaTerm::var("A"),
                ),
            )
        )
    );


    let lambdaterme = LambdaTerm::goal(goal.clone());
    println!("\nlambdaterme : {:?}\n", lambdaterme);
    let (names, lambdaterme) = lambdaterme.intros();
    println!("\nlambdaterme : {:?}\n", lambdaterme);
    let lambdaterme = lambdaterme.elim(names[2].clone());
    println!("\nlambdaterme : {:?}\n", lambdaterme);
    let (_, lambdaterme) = lambdaterme.intros();
    println!("\nlambdaterme : {:?}\n", lambdaterme);
    let lambdaterme = lambdaterme.split();
    println!("\nlambdaterme : {:?}\n", lambdaterme);
    let lambdaterme = lambdaterme.assumption();
    println!("\nlambdaterme : {:?}\n", lambdaterme);
    let lambdaterme = lambdaterme.assumption();
    println!("\nlambdaterme : {:?}\n", lambdaterme);

    check(goal, lambdaterme);
}

#[test]
// ∀ A:Prop, ∀ B:Prop A /\ B -> B
fn elim_sigma_right() {
    let goal = LambdaTerm::pi(
        "A".to_string(),
        LambdaTerm::types(),
        LambdaTerm::pi(
            "B".to_string(),
            LambdaTerm::types(),
            LambdaTerm::imp(
                LambdaTerm::and(
                    LambdaTerm::var("A"),
                    LambdaTerm::var("B")
                ),
                LambdaTerm::var("B"),
            )
        )
    );


    let lambdaterme = LambdaTerm::goal(goal.clone());
    println!("\nlambdaterme : {:?}\n", lambdaterme);
    let (names, lambdaterme) = lambdaterme.intros();
    println!("\nlambdaterme : {:?}\n", lambdaterme);
    let lambdaterme = lambdaterme.elim(names[2].clone());
    println!("\nlambdaterme : {:?}\n", lambdaterme);
    let (_, lambdaterme) = lambdaterme.intros();
    println!("\nlambdaterme : {:?}\n", lambdaterme);
    let lambdaterme = lambdaterme.assumption();
    println!("\nlambdaterme : {:?}\n", lambdaterme);

    check(goal, lambdaterme);
}

#[test]
// ∀ A:Prop, ∀ B:Prop A /\ B -> A
fn elim_sigma_left() {
    let goal = LambdaTerm::pi(
        "A".to_string(),
        LambdaTerm::types(),
        LambdaTerm::pi(
            "B".to_string(),
            LambdaTerm::types(),
            LambdaTerm::imp(
                LambdaTerm::and(
                    LambdaTerm::var("A"),
                    LambdaTerm::var("B")
                ),
                LambdaTerm::var("A"),
            )
        )
    );


    let lambdaterme = LambdaTerm::goal(goal.clone());
    println!("\nlambdaterme : {:?}\n", lambdaterme);
    let (names, lambdaterme) = lambdaterme.intros();
    println!("\nlambdaterme : {:?}\n", lambdaterme);
    let lambdaterme = lambdaterme.elim(names[2].clone());
    println!("\nlambdaterme : {:?}\n", lambdaterme);
    let (_, lambdaterme) = lambdaterme.intros();
    println!("\nlambdaterme : {:?}\n", lambdaterme);
    let lambdaterme = lambdaterme.assumption();
    println!("\nlambdaterme : {:?}\n", lambdaterme);

    check(goal, lambdaterme);
}

#[test]
// ∀ A:Prop, ∀ B:Prop A -> B -> A /\ B
fn split() {
    let goal = LambdaTerm::pi(
        "A".to_string(),
        LambdaTerm::types(),
        LambdaTerm::pi(
            "B".to_string(),
            LambdaTerm::types(),
            LambdaTerm::imp(
                LambdaTerm::var("A"),
                LambdaTerm::imp(
                    LambdaTerm::var("B"),
                    LambdaTerm::and(
                        LambdaTerm::var("A"),
                        LambdaTerm::var("B")
                    )
                )
            )
        )
    );

    let lambdaterme = LambdaTerm::goal(goal.clone());
    println!("\nlambdaterme : {:?}\n", lambdaterme);
    let (_, lambdaterme) = lambdaterme.intros();
    println!("\nlambdaterme : {:?}\n", lambdaterme);
    let lambdaterme = lambdaterme.split();
    println!("\nlambdaterme : {:?}\n", lambdaterme);
    let lambdaterme = lambdaterme.assumption();
    println!("\nlambdaterme : {:?}\n", lambdaterme);
    let lambdaterme = lambdaterme.assumption();
    println!("\nlambdaterme : {:?}\n", lambdaterme);

    check(goal, lambdaterme);
}

#[test]
// ∀ A:Prop, ∀ B:Prop (A -> B) -> A -> B
fn apply_test() {
    let goal = LambdaTerm::pi(
        "A".to_string(),
        LambdaTerm::types(),
        LambdaTerm::pi(
            "B".to_string(),
            LambdaTerm::types(),
            LambdaTerm::imp(
                LambdaTerm::imp(
                    LambdaTerm::var("A"),
                    LambdaTerm::var("B"),
                ),
                LambdaTerm::imp(
                    LambdaTerm::var("A"),
                    LambdaTerm::var("B"),
                )
            )
        )
    );

    let lambdaterme = LambdaTerm::goal(goal.clone());
    println!("\nlambdaterme : {:?}\n", lambdaterme);
    let (names, lambdaterme) = lambdaterme.intros();
    println!("\nlambdaterme : {:?}\n", lambdaterme);
    let lambdaterme = lambdaterme.apply(names[2].clone(), HashMap::new());
    println!("\nlambdaterme : {:?}\n", lambdaterme);
    let lambdaterme = lambdaterme.assumption();
    println!("\nlambdaterme : {:?}\n", lambdaterme);

    check(goal, lambdaterme);
}

#[test]
// ∀ A:Prop, A -> A
fn imply_prop_intros() {
    let goal = LambdaTerm::pi(
        "A".to_string(),
        LambdaTerm::types(),
        LambdaTerm::pi(
            "impl".to_string(),
            LambdaTerm::var("A"),
            LambdaTerm::var("A")
        )
    );

    let lambdaterme = LambdaTerm::goal(goal.clone());
    println!("\nlambdaterme : {:?}\n", lambdaterme);
    let (_, lambdaterme) = lambdaterme.intros();
    println!("\nlambdaterme : {:?}\n", lambdaterme);
    let lambdaterme = lambdaterme.assumption();
    println!("\nlambdaterme : {:?}\n", lambdaterme);

    check(goal, lambdaterme);
}

#[test]
// ∀ A:Prop, A -> A
fn imply_prop() {
    let goal = LambdaTerm::pi(
        "A".to_string(),
        LambdaTerm::types(),
        LambdaTerm::pi(
            "impl".to_string(),
            LambdaTerm::var("A"),
            LambdaTerm::var("A")
        )
    );

    let lambdaterme = LambdaTerm::goal(goal.clone());
    println!("\nlambdaterme : {:?}\n", lambdaterme);
    let (_, lambdaterme) = lambdaterme.intro();
    println!("\nlambdaterme : {:?}\n", lambdaterme);
    let (_, lambdaterme) = lambdaterme.intro();
    println!("\nlambdaterme : {:?}\n", lambdaterme);
    let lambdaterme = lambdaterme.assumption();
    println!("\nlambdaterme : {:?}\n", lambdaterme);

    check(goal, lambdaterme);
}

#[test]
// Pi(x:a, a)
fn pi_hell() {
    let goal = LambdaTerm::pi(
        "n1".to_string(),
        LambdaTerm::pi(
            "n2".to_string(),
            LambdaTerm::var("a"),
            LambdaTerm::pi(
                "n3".to_string(),
                LambdaTerm::var("b"),
                LambdaTerm::var("c"),
            ),
        ),
        LambdaTerm::pi(
            "n4".to_string(),
            LambdaTerm::var("a"),
            LambdaTerm::pi(
                "n5".to_string(),
                LambdaTerm::var("b"),
                LambdaTerm::var("c"),
            )
        )
    );

    let lambdaterme = LambdaTerm::goal(goal.clone());
    println!("\nlambdaterme : {:?}\n", lambdaterme);
    let (name1, lambdaterme) = lambdaterme.intro();
    println!("\nlambdaterme : {:?}\n {}", lambdaterme, name1);
    let (_, lambdaterme) = lambdaterme.intro();
    println!("\nlambdaterme : {:?}\n", lambdaterme);
    let (_, lambdaterme) = lambdaterme.intro();
    println!("\nlambdaterme : {:?}\n", lambdaterme);
    let lambdaterme = lambdaterme.apply(name1, HashMap::new());
    println!("\nlambdaterme : {:?}\n", lambdaterme);
    let lambdaterme = lambdaterme.assumption();
    println!("\nlambdaterme : {:?}\n", lambdaterme);
    let lambdaterme = lambdaterme.assumption();
    println!("\nlambdaterme : {:?}\n", lambdaterme);

    check(goal, lambdaterme);
}


#[test]
// a -> a
fn better_apply() {
    let goal = LambdaTerm::imp(
        LambdaTerm::var("a"), 
        LambdaTerm::var("a"), 
    );

    let lambdaterme = LambdaTerm::goal(goal.clone());
    println!("\nlambdaterme : {:?}\n", lambdaterme);
    let (_, lambdaterme) = lambdaterme.intro();
    println!("\nlambdaterme : {:?}\n", lambdaterme);
    let lambdaterme = lambdaterme.assumption();
    println!("\nlambdaterme : {:?}\n", lambdaterme);

    check(goal, lambdaterme);
}
