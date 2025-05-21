#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn ctx_emp_1() {
        let mut tree = Judgment::Ctx(Context {content: vec![]}).to_tree();
        tree.apply_tactic(tactic!(CTX_EMP));
        assert!(tree.is_proven());
    }

    #[test]
    fn u_intro_1() {
        let mut tree = Judgment::Typing(
            Context {content: vec![]}, 
            term!(U(0)), 
            term!(U(1)),
        ).to_tree();
        tree.apply_tactic(tactic!(U_INTRO));
        tree.hypo[0].apply_tactic(tactic!(CTX_EMP));
        assert!(tree.is_proven());
    }

}