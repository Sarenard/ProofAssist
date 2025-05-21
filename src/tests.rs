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

}