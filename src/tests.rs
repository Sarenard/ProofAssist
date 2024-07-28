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
