use crate::assistant::lambda as lambda;

use lambda::LambdaTerm;

pub fn update_goals_nb(term: LambdaTerm, goal_index: &mut usize) -> LambdaTerm {
    match term {
        LambdaTerm::Var(..)
        | LambdaTerm::Types
        | LambdaTerm::Error => {
            term
        }
        LambdaTerm::Goal(box typ, _index) => {
            *goal_index += 1;
            LambdaTerm::goalnb(
                typ,
                *goal_index - 1
            )
        }
        LambdaTerm::Pi(name, box lb1, box lb2) => {
            let part1 = update_goals_nb(lb1, goal_index);
            let part2 = update_goals_nb(lb2, goal_index);
            LambdaTerm::pi(name, part1, part2)
        }
        LambdaTerm::Sigma(name, box lb1, box lb2) => {
            let part1 = update_goals_nb(lb1, goal_index);
            let part2 = update_goals_nb(lb2, goal_index);
            LambdaTerm::sigma(name, part1, part2)
        }
        LambdaTerm::Func(name, box lb1, box lb2) => {
            let part1 = update_goals_nb(lb1, goal_index);
            let part2 = update_goals_nb(lb2, goal_index);
            LambdaTerm::func(name, part1, part2)
        }
        LambdaTerm::App(box lb1, box lb2) => {
            let part1 = update_goals_nb(lb1, goal_index);
            let part2 = update_goals_nb(lb2, goal_index);
            LambdaTerm::app(part1, part2)
        }
        LambdaTerm::Proj(box lb1, box lb2) => {
            let part1 = update_goals_nb(lb1, goal_index);
            let part2 = update_goals_nb(lb2, goal_index);
            LambdaTerm::proj(part1, part2)
        }
        LambdaTerm::Couple(box lb1, box lb2, box lb3) => {
            let part1 = update_goals_nb(lb1, goal_index);
            let part2 = update_goals_nb(lb2, goal_index);
            let part3 = update_goals_nb(lb3, goal_index);
            LambdaTerm::couple(part1, part2, part3)
        }
    }
}