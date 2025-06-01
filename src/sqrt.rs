use malachite::Integer;
use malachite::base::num::conversion::traits::WrappingFrom;
use malachite::base::num::arithmetic::traits::{FloorSqrt, CeilingSqrt, Square};

/// Check if `n` is a perfect square and return a tuple `(is_square, sqrt)`.
///
/// If `is_square` is `true`, `sqrt` is the exact root.
/// Otherwise, `sqrt` is the floor of the root.
pub fn sqrt_exact(n: &Integer) -> (bool, Integer) {
    let root = n.floor_sqrt();
    let is_square = root.clone().square() == *n;
    (is_square, root)
}

/// Return the smallest integer ≥ sqrt(n).
pub fn sqrt_ceil(n: &Integer) -> Integer {
    n.ceiling_sqrt()
}

/// Fast heuristic check if `n` could be a square using Legendre tables.
/// Helps skip obvious non-squares quickly in factorization algorithms.
pub fn is_probably_square(n: &Integer) -> bool {
    const LEGENDRE_TABLES: &[(u64, &[i8])] = &[
        (3, &[0, 1, -1]),
        (5, &[0, 1, -1, -1, 1]),
        (7, &[0, 1, 1, -1, 1, -1, -1]),
        (11, &[0, 1, -1, 1, 1, 1, -1, -1, -1, 1, -1]),
        (13, &[0, 1, -1, 1, 1, -1, -1, -1, -1, 1, 1, -1, 1]),
        (17, &[0, 1, 1, -1, 1, -1, -1, -1, 1, 1, -1, -1, -1, 1, -1, 1, 1]),
        (19, &[0, 1, -1, -1, 1, 1, 1, 1, -1, 1, -1, 1, -1, -1, -1, -1, 1, 1, -1]),
        (23, &[0, 1, 1, 1, 1, -1, 1, -1, 1, 1, -1, -1, 1, 1, -1, -1, 1, -1, 1, -1, -1, -1, -1]),
        (29, &[
            0, 1, -1, -1, 1, 1, 1, 1, -1, 1, -1, -1, -1, 1, -1, -1,
            1, -1, -1, -1, 1, -1, 1, 1, 1, 1, -1, -1, 1
        ]),
        (31, &[
            0, 1, 1, -1, 1, 1, -1, 1, 1, 1, 1, -1, -1, -1, 1, -1,
            1, -1, 1, 1, 1, -1, -1, -1, -1, 1, -1, -1, 1, -1, -1
        ]),
    ];

    for &(p, table) in LEGENDRE_TABLES {
        let r = u64::wrapping_from(&(n % Integer::from(p))) as usize;
        if table[r] == -1 {
            return false;
        }
    }
    true
}
