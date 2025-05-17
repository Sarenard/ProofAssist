use crate::assistant::lambda as lambda;

use lambda::LambdaTerm;

pub fn update_goals_nb(term: LambdaTerm, goal_index: &mut usize) -> LambdaTerm {
    match term {
        LambdaTerm::Var(..)
        | LambdaTerm::Types
        | LambdaTerm::Top
        | LambdaTerm::Bot
        | LambdaTerm::Bool
        | LambdaTerm::TBool
        | LambdaTerm::FBool
        | LambdaTerm::Naturals
        | LambdaTerm::Zero
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
        LambdaTerm::ExFalso(box lb1, box lb2) => {
            let part1 = update_goals_nb(lb1, goal_index);
            let part2 = update_goals_nb(lb2, goal_index);
            LambdaTerm::exfalso(part1, part2)
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
        LambdaTerm::Or(box lb1, box lb2) => {
            let part1 = update_goals_nb(lb1, goal_index);
            let part2 = update_goals_nb(lb2, goal_index);
            LambdaTerm::or(part1, part2)
        }
        LambdaTerm::Left(box lb1, box lb2) => {
            let part1 = update_goals_nb(lb1, goal_index);
            let part2 = update_goals_nb(lb2, goal_index);
            LambdaTerm::left(part1, part2)
        }
        LambdaTerm::Right(box lb1, box lb2) => {
            let part1 = update_goals_nb(lb1, goal_index);
            let part2 = update_goals_nb(lb2, goal_index);
            LambdaTerm::right(part1, part2)
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
        LambdaTerm::Bif(box lb1, box lb2, box lb3) => {
            let part1 = update_goals_nb(lb1, goal_index);
            let part2 = update_goals_nb(lb2, goal_index);
            let part3 = update_goals_nb(lb3, goal_index);
            LambdaTerm::bif(part1, part2, part3)
        }
        LambdaTerm::Rewrite(box lb1, box lb2, box lb3) => {
            let part1 = update_goals_nb(lb1, goal_index);
            let part2 = update_goals_nb(lb2, goal_index);
            let part3 = update_goals_nb(lb3, goal_index);
            LambdaTerm::rewrite(part1, part2, part3)
        }
        LambdaTerm::Match(box lb1, box lb2, box lb3) => {
            let part1 = update_goals_nb(lb1, goal_index);
            let part2 = update_goals_nb(lb2, goal_index);
            let part3 = update_goals_nb(lb3, goal_index);
            LambdaTerm::match_new(part1, part2, part3)
        }
        LambdaTerm::Eq(box lb1, box lb2) => {
            let part1 = update_goals_nb(lb1, goal_index);
            let part2 = update_goals_nb(lb2, goal_index);
            LambdaTerm::eq(part1, part2)
        }
        LambdaTerm::Rec(box lb1, box lb2, box lb3) => {
            let part1 = update_goals_nb(lb1, goal_index);
            let part2 = update_goals_nb(lb2, goal_index);
            let part3 = update_goals_nb(lb3, goal_index);
            LambdaTerm::rec(part1, part2, part3)
        }
        LambdaTerm::Refl(box lb) => {
            let part = update_goals_nb(lb, goal_index);
            LambdaTerm::refl(part)
        }
        LambdaTerm::Succ(box first) => {
            let part = update_goals_nb(first, goal_index);
            LambdaTerm::succ(part)
        }
        LambdaTerm::Inversion(box first, box second) => {
            let part2 = update_goals_nb(second, goal_index);
            LambdaTerm::inversion(first, part2)
        }
    }
}