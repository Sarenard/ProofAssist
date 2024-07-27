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
// (A ^ (not A)) => B
fn absurd() {
    let goal = Type::Imp(
        Box::new(Type::And(
            Box::new(Type::Var("a".to_string())),
            Box::new(Type::Not(Box::new(Type::Var("a".to_string()))))
        )),
        Box::new(Type::Var("b".to_string()))
    ).removenot();

    let lambdaterme = LambdaTerm::Goal(goal.clone(), 0);
    let lambdaterme = lambdaterme.introv("h1".to_string());
    let lambdaterme = lambdaterme.absurd(Type::Var("b".to_string()));
    let lambdaterme = lambdaterme.elim("h1".to_string());
    let lambdaterme = lambdaterme.introv("h2".to_string());
    let lambdaterme = lambdaterme.introv("h3".to_string());
    let lambdaterme = lambdaterme.apply("h3".to_string());
    let lambdaterme = lambdaterme.exact("h2".to_string());

    check(goal, lambdaterme);
}

#[test]
// not (a ^ (not a))
fn test_not_3() {
    let goal = Type::Not(
        Box::new(Type::And(
            Box::new(Type::Var("a".to_string())),
            Box::new(Type::Not(Box::new(Type::Var("a".to_string()))))
        ))
    ).removenot();

    let lambdaterme = LambdaTerm::Goal(goal.clone(), 0);
    let lambdaterme = lambdaterme.introv("h1".to_string());
    let lambdaterme = lambdaterme.elim("h1".to_string());
    let lambdaterme = lambdaterme.introv("h2".to_string());
    let lambdaterme = lambdaterme.introv("h3".to_string());
    let lambdaterme = lambdaterme.apply("h3".to_string());
    let lambdaterme = lambdaterme.exact("h2".to_string());

    check(goal, lambdaterme);
}

#[test]
// (not (a ^ b)) => a => (not b)
fn test_not_2() {
    let goal = Type::Imp(
        Box::new(Type::Not(
            Box::new(Type::And(
                Box::new(Type::Var("a".to_string())),
                Box::new(Type::Var("b".to_string())),
            )))
        ),
        Box::new(Type::Imp(
            Box::new(Type::Var("a".to_string())),
            Box::new(Type::Not(Box::new(Type::Var("b".to_string()))))
        ))

    ).removenot();

    let lambdaterme = LambdaTerm::Goal(goal.clone(), 0);
    let lambdaterme = lambdaterme.introv("h1".to_string());
    let lambdaterme = lambdaterme.introv("h2".to_string());
    let lambdaterme = lambdaterme.introv("h3".to_string());
    let lambdaterme = lambdaterme.apply("h1".to_string());
    let lambdaterme = lambdaterme.split();
    let lambdaterme = lambdaterme.exact("h2".to_string());
    let lambdaterme = lambdaterme.exact("h3".to_string());

    check(goal, lambdaterme);
}

#[test]
// (not a) => a => bottom
fn test_not() {
    let goal = Type::Imp(
        Box::new(
            Type::Not(
                Box::new(Type::Var("a".to_string())),
            )),
            Box::new(Type::Imp(
                Box::new(Type::Var("a".to_string())),
                Box::new(Type::Bottom)
            )
        )
    ).removenot();

    let lambdaterme = LambdaTerm::Goal(goal.clone(), 0);
    let lambdaterme = lambdaterme.introv("h1".to_string());
    let lambdaterme = lambdaterme.exact("h1".to_string());

    check(goal, lambdaterme);
}

#[test]
// (a ^ b) => b
fn and_destruct_right() {
    let goal = Type::Imp(
        Box::new(Type::And(
            Box::new(Type::Var("a".to_string())),
            Box::new(Type::Var("b".to_string())),
        )),
        Box::new(
            Type::Var("b".to_string())
        )
    );

    let lambdaterme = LambdaTerm::Goal(goal.clone(), 0);
    let lambdaterme = lambdaterme.introv("h1".to_string());
    let lambdaterme = lambdaterme.elim("h1".to_string());
    let lambdaterme = lambdaterme.introv("h2".to_string());
    let lambdaterme = lambdaterme.introv("h3".to_string());
    let lambdaterme = lambdaterme.exact("h3".to_string());

    check(goal, lambdaterme);
}

#[test]
// (a ^ b) => a
fn and_destruct_left() {
    let goal = Type::Imp(
        Box::new(Type::And(
            Box::new(Type::Var("a".to_string())),
            Box::new(Type::Var("b".to_string())),
        )),
        Box::new(
            Type::Var("a".to_string())
        )
    );

    let lambdaterme = LambdaTerm::Goal(goal.clone(), 0);
    let lambdaterme = lambdaterme.introv("h1".to_string());
    let lambdaterme = lambdaterme.elim("h1".to_string());
    let lambdaterme = lambdaterme.introv("h2".to_string());
    let lambdaterme = lambdaterme.introv("h3".to_string());
    let lambdaterme = lambdaterme.exact("h2".to_string());

    check(goal, lambdaterme);
}

#[test]
// a => a
fn basic_impl() {
    let goal = Type::Imp(
        Box::new(Type::Var("A".to_string())),
        Box::new(Type::Var("A".to_string())),
    );

    let lambdaterme = LambdaTerm::Goal(goal.clone(), 0);
    let lambdaterme = lambdaterme.introv("h1".to_string());
    let lambdaterme = lambdaterme.exact("h1".to_string());

    check(goal, lambdaterme);
}

#[test]
// a => b => a ^ b
fn and_construct() {
    let goal = Type::Imp(
        Box::new(Type::Var("a".to_string())),
        Box::new(Type::Imp(
            Box::new(Type::Var("b".to_string())),
            Box::new(Type::And(
                Box::new(Type::Var("a".to_string())),
                Box::new(Type::Var("b".to_string())),
            )),
        )),
    );

    let lambdaterme = LambdaTerm::Goal(goal.clone(), 0);
    let lambdaterme = lambdaterme.introv("h1".to_string());
    let lambdaterme = lambdaterme.introv("h2".to_string());
    let lambdaterme = lambdaterme.split();
    let lambdaterme = lambdaterme.exact("h1".to_string());
    let lambdaterme = lambdaterme.exact("h2".to_string());

    check(goal, lambdaterme);
}

#[test]
// a => (a => b) => b
fn k_combinator() {
    let goal = Type::Imp(
        Box::new(Type::Var("A".to_string())),
        Box::new(Type::Imp(
            Box::new(Type::Imp(
                Box::new(Type::Var("A".to_string())),
                Box::new(Type::Var("B".to_string()))
            )),
            Box::new(Type::Var("B".to_string()))
        )),
    );

    let lambdaterme = LambdaTerm::Goal(goal.clone(), 0);
    let lambdaterme = lambdaterme.introv("h1".to_string());
    let lambdaterme = lambdaterme.introv("h2".to_string());
    let lambdaterme = lambdaterme.apply("h2".to_string());
    let lambdaterme = lambdaterme.exact("h1".to_string());

    check(goal, lambdaterme);
}

#[test]
// a => b => a
fn basic_impl_v2() {
    let goal = Type::Imp(
        Box::new(Type::Var("A".to_string())),
        Box::new(Type::Imp(
            Box::new(Type::Var("B".to_string())),
            Box::new(Type::Var("A".to_string()))
        )),
    );

    let lambdaterme = LambdaTerm::Goal(goal.clone(), 0);
    let lambdaterme = lambdaterme.introv("h1".to_string());
    let lambdaterme = lambdaterme.introv("h2".to_string());
    let lambdaterme = lambdaterme.exact("h1".to_string());

    check(goal, lambdaterme);
}
