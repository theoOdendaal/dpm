/// Assess whether floating
/// point values approximate each other.
#[macro_export]
macro_rules! assert_approx_eq {
    ($a:expr, $b:expr) => {
        assert_approx_eq!($a, $b, 1e-6)
    };
    ($a:expr, $b:expr, $tol:expr) => {

        if ($a - $b).abs() > $tol {
            panic!(
            "assertion failed: `(left ≈ right)`\n  left: `{:?}`,\n right: `{:?}`,\n  tol: `{:?}`",
            $a, $b, $tol
            );
        }
    };
}
