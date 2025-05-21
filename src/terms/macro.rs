#[macro_export]
macro_rules! term {
    (U($val:expr)) => { $crate::terms::term::Term::Universe($crate::terms::universe::Universe($val)) };
    (Var($name:expr)) => { $crate::terms::term::Term::Var($crate::terms::var::Var($name.to_string())) };
}