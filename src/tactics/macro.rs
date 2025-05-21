#[macro_export]
macro_rules! tactic {
    (CTX_EMP) => { $crate::tactics::Tactics::Ctx($crate::exts::ctx::CtxTactic::CTX_EMP) };
    (CTX_EXT) => { $crate::tactics::Tactics::Ctx($crate::exts::ctx::CtxTactic::CTX_EXT) };
    (U_CUMUL) => { $crate::tactics::Tactics::U($crate::exts::universe::UTactic::U_CUMUL) };
    (U_INTRO) => { $crate::tactics::Tactics::U($crate::exts::universe::UTactic::U_INTRO) };
    (JUGEQEQUIV_REFL) => { $crate::tactics::Tactics::JUGEQEQUIV($crate::exts::jugeqequiv::JUGEQEQUIVTactic::JUGEQEQUIV_REFL) };
    (JUGEQEQUIV_SYM) => { $crate::tactics::Tactics::JUGEQEQUIV($crate::exts::jugeqequiv::JUGEQEQUIVTactic::JUGEQEQUIV_SYM) };
    (JUGEQEQUIV_TRANS) => { $crate::tactics::Tactics::JUGEQEQUIV($crate::exts::jugeqequiv::JUGEQEQUIVTactic::JUGEQEQUIV_TRANS) };
    (JUGEQEQUIV_CONV_TERM) => { $crate::tactics::Tactics::JUGEQEQUIV($crate::exts::jugeqequiv::JUGEQEQUIVTactic::JUGEQEQUIV_CONV_TERM) };
    (JUGEQEQUIV_CONV_EQ) => { $crate::tactics::Tactics::JUGEQEQUIV($crate::exts::jugeqequiv::JUGEQEQUIVTactic::JUGEQEQUIV_CONV_EQ) };
}

#[macro_export]
macro_rules! apply_tactic {
    ($tree:expr, $name:ident) => {{
        let tactic = tactic!($name);
        $tree.apply_tactic(tactic, vec![]);
    }};

    ($tree:expr, $name:ident, $args:expr) => {{
        let tactic = tactic!($name);
        $tree.apply_tactic(tactic, $args);
    }};
}