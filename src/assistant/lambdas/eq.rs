use crate::assistant::lambdas as lambdas;
use crate::assistant::lambda as lambda;

use lambda::LambdaTerm;

use lambdas::beta_reduc::beta_reduce;
use lambdas::alpha_equiv::alpha_equiv;

impl PartialEq for LambdaTerm {
    fn eq(&self, other: &Self) -> bool {
        let beta_self = beta_reduce(self.clone());
        let beta_other = beta_reduce(other.clone());
        alpha_equiv(beta_other, beta_self)
    }
}