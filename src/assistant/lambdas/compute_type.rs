use std::collections::HashMap;

use crate::assistant::lambda as lambda;

use lambda::{
    LambdaTerm,
};

use crate::DEBUG;

use super::{alpha_equiv::{alpha_equiv, replace_free_variable}, beta_reduc::beta_reduce};

pub fn compute_type(lambdaterm: LambdaTerm, context: HashMap<String, LambdaTerm>) -> LambdaTerm {
    match lambdaterm.clone() {
        LambdaTerm::Var(name) => {
            if DEBUG {println!("var : {}, context : {:?}", name, context)}
            let res = context.get(&name).unwrap().clone();
            res
        }
        LambdaTerm::Goal(..) 
        | LambdaTerm::Error => {
            panic!("Not supposed to happend !")
        }
        // for those two : we verify that first and second are correcly typed and just return Types
        LambdaTerm::Pi(_name, box first, box second) => {
            let _first_type = compute_type(first.clone(), context.clone());
            let _second_type = compute_type(second, context);
            LambdaTerm::Types
        }
        LambdaTerm::Sigma(_name, box first, box second) => {
            let _first_type = compute_type(first.clone(), context.clone());
            let _second_type = compute_type(second, context);
            LambdaTerm::Types
        }
        LambdaTerm::Types => {
            LambdaTerm::Types
        }
        LambdaTerm::Couple(box first, box second, box LambdaTerm::Sigma(a, box b, box c)) => {
            let first_type = compute_type(first.clone(), context.clone());
            let second_type = compute_type(second, context);
            if first_type == b.clone() && second_type == replace_free_variable(a.clone(), first, c.clone()) {
                LambdaTerm::sigma(a, b, c)
            } else {
                panic!()
            }
        }
        LambdaTerm::Couple(..) => {
            // same as above
            panic!()
        }
        LambdaTerm::Func(var, box typ, box body) => {
            let mut new_context = context.clone();
            new_context.insert(var.clone(), typ.clone());

            let body_type = compute_type(body, new_context);
            LambdaTerm::pi(var, typ, body_type)
        }
        LambdaTerm::App(box first, box second) => {
            let functype = compute_type(first, context.clone());
            let bodytype = compute_type(second, context);
            match functype {
                LambdaTerm::Func(_name, box type1, box type2) if type1 == bodytype => {
                    return type2
                }
                LambdaTerm::Pi(_name, box type1, box type2) if type1 == bodytype => {
                    return type2
                }
                other => panic!("Error, unknown : {:?}", other)
            }
        }
        LambdaTerm::Proj(box first, box second) => {
            let firsttype = compute_type(first.clone(), context.clone());
            let secondtype = compute_type(second.clone(), context);
            // firsttype : exists a, P(a)
            // secondtype : forall k, P(k) -> lambdaterm
            // then whole is lambdaterm
            match (firsttype, secondtype) {
                (
                    LambdaTerm::Sigma(name1, box first1, box second1),
                    LambdaTerm::Pi(name2, box first2, box LambdaTerm::Pi(_name3, box second2, box result))
                ) if first1 == first2 
                    && (replace_free_variable(name1.clone(), first1.clone(), second1.clone()) 
                        == replace_free_variable(name2.clone(), first2.clone(), second2.clone()))
                => {
                    result
                }
                _ => panic!("Unreachable")
            }
        }
    }
}