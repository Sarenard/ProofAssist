
use crate::assistant::lambda as lambda;

use lambda::LambdaTerm;

pub fn free_var(lambda: LambdaTerm) -> Vec<String> {
    match lambda {
        LambdaTerm::Var(name) => {
            vec![name.clone()]
        }
        LambdaTerm::Types
        | LambdaTerm::Top
        | LambdaTerm::Bot => {
            vec![]
        }
        LambdaTerm::Goal(box _typ, _nb) => {
            vec![]
        }
        LambdaTerm::Refl(box typ) => {
            free_var(typ)
        }
        LambdaTerm::Func(name, box typ, box body)
        | LambdaTerm::Sigma(name, box typ, box body)
        | LambdaTerm::Pi(name, box typ, box body) => {
            let mut vec_tot: Vec<String> = vec![];
            let variables_typ = free_var(typ);
            let variables_body: Vec<String> = free_var(body).iter().cloned()
            .filter(|x| *x != name).collect();

            vec_tot.extend(variables_typ);
            vec_tot.extend(variables_body);

            vec_tot
        }
        LambdaTerm::ExFalso(box first, box second)
        | LambdaTerm::Or(box first, box second)
        | LambdaTerm::Eq(box first, box second)
        | LambdaTerm::Left(box first, box second)
        | LambdaTerm::Right(box first, box second)
        | LambdaTerm::App(box first, box second) => {
            let mut vec_tot: Vec<String> = vec![];
            let variables_typ = free_var(first);
            let variables_body: Vec<String> = free_var(second);

            vec_tot.extend(variables_typ);
            vec_tot.extend(variables_body);

            vec_tot
        }
        LambdaTerm::Proj(box first, box second) => {
            let mut vec_tot: Vec<String> = vec![];
            let variables_typ = free_var(first);
            let variables_body: Vec<String> = free_var(second);

            vec_tot.extend(variables_typ);
            vec_tot.extend(variables_body);

            vec_tot
        }
        LambdaTerm::Match(box first, box second, box third)
        | LambdaTerm::Rewrite(box first, box second, box third)
        | LambdaTerm::Couple(box first, box second, box third) => {
            let mut vec_tot: Vec<String> = vec![];
            let variables_first = free_var(first);
            let variables_second: Vec<String> = free_var(second);
            let variables_third: Vec<String> = free_var(third);

            vec_tot.extend(variables_first);
            vec_tot.extend(variables_second);
            vec_tot.extend(variables_third);

            vec_tot
        }
        LambdaTerm::Error => unreachable!()
    }
}