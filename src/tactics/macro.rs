#[macro_export]
macro_rules! tactic {
    (CTX_EMP) => { $crate::tactics::Tactics::Ctx($crate::tactics::CtxTactic::CTX_EMP) };
    (CTX_EXT) => { $crate::tactics::Tactics::Ctx($crate::tactics::CtxTactic::CTX_EXT) };
    (U_CUMUL) => { $crate::tactics::Tactics::U($crate::tactics::UTactic::U_CUMUL) };
    (U_INTRO) => { $crate::tactics::Tactics::U($crate::tactics::UTactic::U_INTRO) };
}