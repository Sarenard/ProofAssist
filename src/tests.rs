use crate::assistant::lambda::LambdaTerm;

fn check(goal: LambdaTerm, lambdaterme: LambdaTerm) {
    if lambdaterme.clone().containsgoal() {
        panic!("Pas fini ! {:?}", lambdaterme);
    }

    let ok = lambdaterme.clone().check(goal);
    if !ok {
        panic!("Ehh i'm wrong somewhere {:?}", lambdaterme);
    }
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
    println!("lambdaterme : {:?}", lambdaterme);
    let (_, lambdaterme) = lambdaterme.intros();
    println!("lambdaterme : {:?}", lambdaterme);
    let lambdaterme = lambdaterme.split();
    println!("lambdaterme : {:?}", lambdaterme);
    let lambdaterme = lambdaterme.assumption();
    println!("lambdaterme : {:?}", lambdaterme);
    let lambdaterme = lambdaterme.assumption();
    println!("lambdaterme : {:?}", lambdaterme);

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
    println!("lambdaterme : {:?}", lambdaterme);
    let (names, lambdaterme) = lambdaterme.intros();
    println!("lambdaterme : {:?}", lambdaterme);
    let lambdaterme = lambdaterme.apply(names[2].clone());
    println!("lambdaterme : {:?}", lambdaterme);
    let lambdaterme = lambdaterme.assumption();
    println!("lambdaterme : {:?}", lambdaterme);

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
    println!("lambdaterme : {:?}", lambdaterme);
    let (_, lambdaterme) = lambdaterme.intros();
    println!("lambdaterme : {:?}", lambdaterme);
    let lambdaterme = lambdaterme.assumption();
    println!("lambdaterme : {:?}", lambdaterme);

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
    println!("lambdaterme : {:?}", lambdaterme);
    let (_, lambdaterme) = lambdaterme.intro();
    println!("lambdaterme : {:?}", lambdaterme);
    let (_, lambdaterme) = lambdaterme.intro();
    println!("lambdaterme : {:?}", lambdaterme);
    let lambdaterme = lambdaterme.assumption();
    println!("lambdaterme : {:?}", lambdaterme);

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
    println!("lambdaterme : {:?}", lambdaterme);
    let (name1, lambdaterme) = lambdaterme.intro();
    println!("lambdaterme : {:?} {}", lambdaterme, name1);
    let (_, lambdaterme) = lambdaterme.intro();
    println!("lambdaterme : {:?}", lambdaterme);
    let (_, lambdaterme) = lambdaterme.intro();
    println!("lambdaterme : {:?}", lambdaterme);
    let lambdaterme = lambdaterme.apply(name1);
    println!("lambdaterme : {:?}", lambdaterme);
    let lambdaterme = lambdaterme.assumption();
    println!("lambdaterme : {:?}", lambdaterme);
    let lambdaterme = lambdaterme.assumption();
    println!("lambdaterme : {:?}", lambdaterme);

    check(goal, lambdaterme);
}


#[test]
// Pi(x:a, a)
fn better_apply() {
    let goal = LambdaTerm::pi(
        "x".to_string(),
        LambdaTerm::var("a"), 
        LambdaTerm::var("a"), 
    );

    let lambdaterme = LambdaTerm::goal(goal.clone());
    println!("lambdaterme : {:?}", lambdaterme);
    let (_, lambdaterme) = lambdaterme.intro();
    println!("lambdaterme : {:?}", lambdaterme);
    let lambdaterme = lambdaterme.assumption();
    println!("lambdaterme : {:?}", lambdaterme);

    check(goal, lambdaterme);
}
