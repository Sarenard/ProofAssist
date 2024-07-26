use crate::assistant::lambda::LambdaTerm;
use crate::assistant::types::Type;

fn check(goal: Type, lambdaterme: LambdaTerm) {
    if lambdaterme.clone().containsgoal() {
        panic!("Pas fini ! {:?}", lambdaterme);
    }

    let ok = lambdaterme.clone().check(goal);
    if !ok {
        panic!("Ehh i'm wrong somewhere {:?}", lambdaterme);
    }
}

#[test]
// a => a
fn basic_impl() {
    let goal = Type::Impl(
        Box::new(Type::Var("A".to_string())),
        Box::new(Type::Var("A".to_string())),
    );

    let lambdaterme = LambdaTerm::Goal(goal.clone());
    let lambdaterme = lambdaterme.intro("h1".to_string());
    let lambdaterme = lambdaterme.exact("h1".to_string());

    check(goal, lambdaterme);
}


#[test]
// a => (a => b) => b
fn k_combinator() {
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

    let lambdaterme = LambdaTerm::Goal(goal.clone());
    let lambdaterme = lambdaterme.intro("h1".to_string());
    let lambdaterme = lambdaterme.intro("h2".to_string());
    let lambdaterme = lambdaterme.apply("h2".to_string());
    let lambdaterme = lambdaterme.exact("h1".to_string());

    check(goal, lambdaterme);
}

#[test]
// a => b => a
fn other_thing() {
    let goal = Type::Impl(
        Box::new(Type::Var("A".to_string())),
        Box::new(Type::Impl(
            Box::new(Type::Var("B".to_string())),
            Box::new(Type::Var("A".to_string()))
        )),
    );

    let lambdaterme = LambdaTerm::Goal(goal.clone());
    let lambdaterme = lambdaterme.intro("h1".to_string());
    let lambdaterme = lambdaterme.intro("h2".to_string());
    let lambdaterme = lambdaterme.exact("h1".to_string());

    check(goal, lambdaterme);
}
