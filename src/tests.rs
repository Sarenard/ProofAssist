#[cfg(test)]
mod tests {
    use crate::*;

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

}