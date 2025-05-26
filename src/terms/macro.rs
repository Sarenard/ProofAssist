#[macro_export]
macro_rules! term {
    (U($val:expr)) => { $crate::terms::Term::Universe($crate::exts::universe::Universe($val)) };
    (Var($name:expr)) => { $crate::terms::Term::Var($crate::exts::var::Var($name.to_string())) };
    (Pi($x:expr, $a:expr, $b:expr)) => { $crate::terms::Term::Pi($crate::exts::pi::Pi(Box::new($x), Box::new($a), Box::new($b))) };
    (Lambda($x:expr, $a:expr, $b:expr)) => { $crate::terms::Term::Lambda($crate::exts::pi::Lambda(Box::new($x), Box::new($a), Box::new($b))) };
    (Apply($f:expr, $a:expr)) => { $crate::terms::Term::Apply($crate::exts::pi::Apply(Box::new($f), Box::new($a))) };
    (Zero) => { $crate::terms::Term::Zero($crate::exts::zero::Zero) };
    (Ind0($C:expr, $a:expr)) => { $crate::terms::Term::Ind0($crate::exts::zero::Ind0($C, $a)) };
    (NZero) => { $crate::terms::Term::NZero($crate::exts::nat::NZero) };
    (NSucc($term:expr)) => { $crate::terms::Term::NSucc($crate::exts::nat::NSucc(Box::new($term))) };
    (IndN($C:expr, $c0:expr, $cs:expr, $n:expr)) => { 
        $crate::terms::Term::IndN($crate::exts::nat::IndN(Box::new($C), Box::new($c0), Box::new($cs), Box::new($n))) 
    };
    (Nat) => { $crate::terms::Term::Nat($crate::exts::nat::Nat) };
}