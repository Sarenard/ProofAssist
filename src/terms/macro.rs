#[macro_export]
macro_rules! term {
    (U($val:expr)) => { $crate::terms::term::Term::Universe($crate::exts::universe::Universe($val)) };
    (Var($name:expr)) => { $crate::terms::term::Term::Var($crate::exts::var::Var($name.to_string())) };
    (Pi($x:expr, $a:expr, $b:expr)) => { $crate::terms::term::Term::Pi($crate::exts::pi::Pi(Box::new($x), Box::new($a), Box::new($b))) };
    (Lambda($x:expr, $a:expr, $b:expr)) => { $crate::terms::term::Term::Lambda($crate::exts::pi::Lambda(Box::new($x), Box::new($a), Box::new($b))) };
}