#[cfg(test)]
mod tests {
    use crate::{utils::church, *};

    #[test]
    fn ctx_emp_1() {
        let mut tree = Judgment::Ctx(Context {content: vec![]}).to_tree();
        apply_tactic!(tree, CTX_EMP);
        assert!(tree.is_proven());
    }

    #[test]
    fn u_intro_1() {
        let mut tree = Judgment::Typing(
            Context {content: vec![]}, 
            term!(U(0)), 
            term!(U(1)),
        ).to_tree();
        apply_tactic!(tree, U_INTRO);
        apply_tactic!(tree.hypo[0], CTX_EMP);
        assert!(tree.is_proven());
    }

    #[test]
    fn pi_form_1() {
        let context = Context {content: vec![]};
        let x = term!(Var("x"));
        let a = term!(Var("A"));
        let b = term!(Var("B"));
        let mut tree = Judgment::Typing(
            context.clone(),
            term!(Pi(x.clone(), a.clone(), b.clone())),
            term!(U(1)),
        ).to_tree();
        apply_tactic!(tree, PI_FORM);
        assert_eq!(
            tree,
            InfTree {
                hypo: vec![
                    Judgment::Typing(context.clone(), a.clone(), term!(U(1))).to_tree(),
                    Judgment::Typing(context.add_term((x.clone(), a.clone())), b.clone(), term!(U(1))).to_tree(),

                ],
                conclusion: Judgment::Typing(
                    Context {content: vec![]},
                    term!(Pi(x, a, b)),
                    term!(U(1)),
                ),
                tactic: Some(tactic!(PI_FORM)),
                prouved: true
            }
        )
    }

    #[test]
    fn zero_form_1() {
        let mut tree = Judgment::Typing(
            Context {content: vec![]}, 
            term!(Zero), 
            term!(U(4)),
        ).to_tree();
        apply_tactic!(tree, ZERO_FORM);
        apply_tactic!(tree.hypo[0], CTX_EMP);
        assert!(tree.is_proven());
    }

    #[test]
    fn nat_intro_1_1() {
        let mut tree = Judgment::Typing(
            Context {content: vec![]},
            term!(NZero),
            term!(Nat),
        ).to_tree();
        apply_tactic!(tree, NINTRO1);
        apply_tactic!(tree.hypo[0], CTX_EMP);
        assert!(tree.is_proven());
    }

    #[test]
    fn nat_form_1() {
        let mut tree = Judgment::Typing(
            Context {content: vec![]},
            term!(Nat),
            term!(U(0)),
        ).to_tree();
        apply_tactic!(tree, NFORM);
        apply_tactic!(tree.hypo[0], CTX_EMP);
        assert!(tree.is_proven());
    }

    #[test]
    // 1 in N
    fn nat_1() {
        let mut tree = Judgment::Typing(
            Context {content: vec![]},
            term!(NSucc(term!(NZero))),
            term!(Nat),
        ).to_tree();
        apply_tactic!(tree, NINTRO2);
        apply_tactic!(tree.hypo[0], NINTRO1);
        apply_tactic!(tree.hypo[0].hypo[0], CTX_EMP);
        assert!(tree.is_proven());
    }

    #[test]
    // double(0) == 0
    fn double_zero() {
        fn double(term: Term) -> Term {
            term!(IndN(
                term!(Lambda(
                    term!(Var("n")),
                    term!(Nat),
                    term!(Nat)
                )),
                term!(NZero),
                term!(Lambda(
                    term!(Var("n")),
                    term!(Nat),
                    term!(Lambda(
                        term!(Var("y")),
                        term!(Nat),
                        term!(NSucc(term!(NSucc(term!(Var("y"))))))
                    ))
                )),
                term
            ))
        }
        let mut tree = Judgment::JudgEq(
            Context {content: vec![]},
            double(church(0)),
            church(0),
            term!(Nat),
        ).to_tree();
        apply_tactic!(tree, NCOMP1, vec![term!(Var("n")), term!(Var("y"))]);
        apply_tactic!(tree.hypo[0], NFORM);
        apply_tactic!(tree.hypo[0].hypo[0], CTX_EXT);
        apply_tactic!(tree.hypo[0].hypo[0].hypo[0], NFORM);
        apply_tactic!(tree.hypo[0].hypo[0].hypo[0].hypo[0], CTX_EMP);
        apply_tactic!(tree.hypo[1], NINTRO1);
        apply_tactic!(tree.hypo[1].hypo[0], CTX_EMP);
        apply_tactic!(tree.hypo[2], NINTRO2);
        apply_tactic!(tree.hypo[2].hypo[0], NINTRO2);
        apply_tactic!(tree.hypo[2].hypo[0].hypo[0], VBLE);
        apply_tactic!(tree.hypo[2].hypo[0].hypo[0].hypo[0], CTX_EXT);
        apply_tactic!(tree.hypo[2].hypo[0].hypo[0].hypo[0].hypo[0], NFORM);
        apply_tactic!(tree.hypo[2].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0], CTX_EXT);
        apply_tactic!(tree.hypo[2].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0], NFORM);
        apply_tactic!(tree.hypo[2].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0], CTX_EMP);
        assert!(tree.is_proven());
    }

    #[test]
    // double(1) == 2
    fn double_1() {
        fn double(term: Term) -> Term {
            term!(IndN(
                term!(Lambda(
                    term!(Var("x")),
                    term!(Nat),
                    term!(Nat)
                )),
                term!(NZero),
                term!(Lambda(
                    term!(Var("x")),
                    term!(Nat),
                    term!(Lambda(
                        term!(Var("y")),
                        term!(Nat),
                        term!(NSucc(term!(NSucc(term!(Var("y"))))))
                    ))
                )),
                term
            ))
        }
        let mut tree = Judgment::JudgEq(
            Context {content: vec![]},
            double(church(1)),
            church(2),
            term!(Nat),
        ).to_tree();
        apply_tactic!(tree, JUGEQEQUIV_TRANS, vec![term!(NSucc(term!(NSucc(double(term!(NZero))))))]);
        apply_tactic!(tree.hypo[0], NCOMP2, vec![term!(Var("x")), term!(Var("y"))]);
        apply_tactic!(tree.hypo[0].hypo[0], NFORM);
        apply_tactic!(tree.hypo[0].hypo[0].hypo[0], CTX_EXT);
        apply_tactic!(tree.hypo[0].hypo[0].hypo[0].hypo[0], NFORM);
        apply_tactic!(tree.hypo[0].hypo[0].hypo[0].hypo[0].hypo[0], CTX_EMP);
        apply_tactic!(tree.hypo[0].hypo[1], NINTRO1);
        apply_tactic!(tree.hypo[0].hypo[1].hypo[0], CTX_EMP);
        apply_tactic!(tree.hypo[0].hypo[3], NINTRO1);
        apply_tactic!(tree.hypo[0].hypo[3].hypo[0], CTX_EMP);
        apply_tactic!(tree.hypo[0].hypo[2], NINTRO2);
        apply_tactic!(tree.hypo[0].hypo[2].hypo[0], NINTRO2);
        apply_tactic!(tree.hypo[0].hypo[2].hypo[0].hypo[0], VBLE);
        apply_tactic!(tree.hypo[0].hypo[2].hypo[0].hypo[0].hypo[0], CTX_EXT);
        apply_tactic!(tree.hypo[0].hypo[2].hypo[0].hypo[0].hypo[0].hypo[0], NFORM);
        apply_tactic!(tree.hypo[0].hypo[2].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0], CTX_EXT);
        apply_tactic!(tree.hypo[0].hypo[2].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0], NFORM);
        apply_tactic!(tree.hypo[0].hypo[2].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0], CTX_EMP);
        apply_tactic!(tree.hypo[1], NINTRO2_EQ);
        apply_tactic!(tree.hypo[1].hypo[0], NINTRO2_EQ);
        apply_tactic!(tree.hypo[1].hypo[0].hypo[0], NCOMP1, vec![term!(Var("x")), term!(Var("y"))]);
        apply_tactic!(tree.hypo[1].hypo[0].hypo[0].hypo[0], NFORM);
        apply_tactic!(tree.hypo[1].hypo[0].hypo[0].hypo[0].hypo[0], CTX_EXT);
        apply_tactic!(tree.hypo[1].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0], NFORM);
        apply_tactic!(tree.hypo[1].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0], CTX_EMP);
        apply_tactic!(tree.hypo[1].hypo[0].hypo[0].hypo[1], NINTRO1);
        apply_tactic!(tree.hypo[1].hypo[0].hypo[0].hypo[1].hypo[0], CTX_EMP);
        apply_tactic!(tree.hypo[1].hypo[0].hypo[0].hypo[2], NINTRO2);
        apply_tactic!(tree.hypo[1].hypo[0].hypo[0].hypo[2].hypo[0], NINTRO2);
        apply_tactic!(tree.hypo[1].hypo[0].hypo[0].hypo[2].hypo[0].hypo[0], VBLE);
        apply_tactic!(tree.hypo[1].hypo[0].hypo[0].hypo[2].hypo[0].hypo[0].hypo[0], CTX_EXT);
        apply_tactic!(tree.hypo[1].hypo[0].hypo[0].hypo[2].hypo[0].hypo[0].hypo[0].hypo[0], NFORM);
        apply_tactic!(tree.hypo[1].hypo[0].hypo[0].hypo[2].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0], CTX_EXT);
        apply_tactic!(tree.hypo[1].hypo[0].hypo[0].hypo[2].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0], NFORM);
        apply_tactic!(tree.hypo[1].hypo[0].hypo[0].hypo[2].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0].hypo[0], CTX_EMP);
        assert!(tree.is_proven());
    }

 
}