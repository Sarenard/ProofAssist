use std::collections::HashMap;

use crate::assistant::lambda as lambda;

use lambda::LambdaTerm;

use crate::DEBUG;

use super::{alpha_equiv::replace_free_variable, free_var::free_var, replace::replace};

pub fn compute_type(lambdaterm: LambdaTerm, context: HashMap<String, LambdaTerm>) -> LambdaTerm {
    if DEBUG {
        println!("compute_type: {:?} {:?}", lambdaterm, context);
    }
    match lambdaterm.clone() {
        LambdaTerm::Var(name) => {
            if DEBUG {println!("var : {}, context : {:?}", name, context)}
            let res = context.get(&name).unwrap().clone();
            res
        }
        LambdaTerm::FBool | LambdaTerm::TBool => {
            LambdaTerm::Bool
        }
        LambdaTerm::Bool => {
            LambdaTerm::Types
        }
        LambdaTerm::Naturals => {
            LambdaTerm::Types
        }
        LambdaTerm::Zero => {
            LambdaTerm::Naturals
        }
        LambdaTerm::Goal(box typ, _nb) => {
            return typ;
        }
        LambdaTerm::Error => {
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
        LambdaTerm::Couple(box first, box second, box LambdaTerm::Sigma(a, box b, box c)) => {
            let first_type = compute_type(first.clone(), context.clone());
            let second_type = compute_type(second, context);
            if first_type == b.clone() && second_type == replace_free_variable(a.clone(), first, c.clone()) {
                LambdaTerm::sigma(a, b, c)
            } else {
                panic!()
            }
        }
        LambdaTerm::Match(box first, box second, box third) => {
            let type_first = compute_type(first, context.clone());
            let type_second = compute_type(second, context.clone());
            let type_third = compute_type(third, context.clone());
            match (type_first, type_second, type_third) {
                (
                    LambdaTerm::Or(box first_a, box first_b), 
                    LambdaTerm::Pi(_name1, box impl1_premice, box impl1_body),
                    LambdaTerm::Pi(_name2, box impl2_premice, box impl2_body),
                ) if first_a == impl1_premice && first_b == impl2_premice && impl1_body == impl2_body => {
                    impl1_body
                },
                other => panic!("other: {:?}", other)
            }
        }
        LambdaTerm::Or(box first, box second) => {
            let _first_type = compute_type(first.clone(), context.clone());
            let _second_type = compute_type(second, context);
            LambdaTerm::Types
        }
        LambdaTerm::Left(box first, box second) => {
            LambdaTerm::or(
                compute_type(first, context),
                second
            )
        }
        LambdaTerm::Right(box first, box second) => {
            LambdaTerm::or(
                first,
                compute_type(second, context),
            )
        }
        LambdaTerm::Bot
        | LambdaTerm::Top
        | LambdaTerm::Types => {
            LambdaTerm::Types // maybe a paradox?
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
            let bodytype = compute_type(second.clone(), context.clone());
            match functype {
                LambdaTerm::Pi(name, box type1, box type2) 
                if type1 == bodytype && (free_var(type1.clone()).contains(&name) || free_var(type2.clone()).contains(&name)) => {
                    return replace_free_variable(name, second, type2)
                }
                LambdaTerm::Pi(name, box type1, box type2) 
                if type1 == bodytype && !free_var(type1.clone()).contains(&name) && !free_var(type2.clone()).contains(&name) => {
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
        LambdaTerm::ExFalso(box first, box second) => {
            let second_inferred = compute_type(second, context);
            match second_inferred {
                LambdaTerm::Bot => {
                    first
                },
                other => panic!("Other : {:?}", other)
            }
        }
        LambdaTerm::Eq(box a, box b) => {
            // we check the types
            let type_a = compute_type(a.clone(), context.clone());
            let type_b = compute_type(b.clone(), context.clone());
            if type_a == type_b {
                LambdaTerm::Types
            } else {
                panic!("Error, type mismatch")
            }
        }
        LambdaTerm::Refl(box a) => {
            LambdaTerm::eq(a.clone(), a)
        }
        LambdaTerm::Rewrite(box lambda, box obj, box witness) => {
            let lambda_type = compute_type(lambda, context.clone());
            match lambda_type {
                LambdaTerm::Eq(box a, box b) => {
                    let replaced = replace(witness.clone(), a.clone(), b.clone());
                    if replaced == compute_type(obj.clone(), context) {
                        witness
                    } else {
                        panic!("Error, not the same {:?} {:?}", replaced, obj)
                    }
                }
                other => panic!("impossible {:?}", other)
            }
        }
        LambdaTerm::Bif(box first, box second, box third) => {
            // maybe ?
            match first {
                LambdaTerm::TBool => {
                    compute_type(second, context)
                }
                LambdaTerm::FBool => {
                    compute_type(third, context)
                }
                _ => panic!("Error, not a boolean")
            }
        }
        LambdaTerm::Succ(box typ) => {
            let type_int = compute_type(typ, context);
            match type_int {
                LambdaTerm::Naturals => {
                    LambdaTerm::Naturals
                }
                other => panic!("Error, not a natural {:?}", other)
            }
        }
        LambdaTerm::Inversion(box first, box second) => {
            todo!()
        }
    }
}

