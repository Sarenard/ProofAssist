use crate::assistant::lambda as lambda;

use lambda::LambdaTerm;

impl PartialEq for LambdaTerm {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Self::Var(l0), 
                Self::Var(r0)
            ) => 
            {
                l0 == r0
            }
            (
                Self::Goal(l0, l1), 
                Self::Goal(r0, r1)
            ) => {
                l0 == r0 && l1 == r1
            }
            (
                Self::Pi(l0, l1, l2), 
                Self::Pi(r0, r1, r2)
            ) => {
                l0 == r0 && l1 == r1 && l2 == r2
            }
            (
                Self::Func(l0, l1, l2), 
                Self::Func(r0, r1, r2)
            ) => {
                l0 == r0 && l1 == r1 && l2 == r2
            }
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}