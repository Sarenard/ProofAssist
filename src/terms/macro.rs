#[macro_export]
macro_rules! term {
    (U($val:expr)) => { $crate::terms::term::Term::Universe($crate::exts::universe::Universe($val)) };
    (Var($name:expr)) => { $crate::terms::term::Term::Var($crate::exts::var::Var($name.to_string())) };
}