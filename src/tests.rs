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

