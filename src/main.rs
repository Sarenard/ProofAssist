#![feature(box_patterns)]

mod assistant;

use assistant::lambda::LambdaTerm as LambdaTerm;
use assistant::types::Type as Type;

fn main() {
    // on veut prouver a ^ b => b ^ a
    let goal = Type::Impl(
        Box::new(
            Type::Var("a".to_string())
        ),
        Box::new(
            Type::Var("a".to_string()),
        )
    );
    let lambdaterme = LambdaTerm::Goal(goal);
    println!("{:?}", lambdaterme);

    let lambdaterme = lambdaterme.intro("h".to_string());
    println!("{:?}", lambdaterme);

    let lambdaterme = lambdaterme.exact("h".to_string());
    println!("{:?}", lambdaterme);

    // we are done
}

