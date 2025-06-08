use crate::sqrt::{is_probably_square, sqrt_ceil, sqrt_exact};
use malachite::{
    base::{
        num::{
            arithmetic::traits::Square,
            basic::traits::{One, Two},
            conversion::{string::options::ToSciOptions, traits::ToSci},
        },
        rounding_modes::RoundingMode::Floor,
    },
    Integer,
};
use std::{io, io::Write};

pub fn scinot(n: &Integer, prec: u64) {
    let mut options = ToSciOptions::default();
    options.set_precision(prec);
    options.set_rounding_mode(Floor);
    options.set_force_exponent_plus_sign(true);
    print!("{}", n.to_sci_with_options(options));
}

pub fn verbose(iteration: &Integer, p: &Integer, q: &Integer, prec: u64) {
    print!("Iteration: ");
    scinot(iteration, prec);
    print!(" p = ");
    scinot(p, prec);
    print!(" q = ");
    scinot(q, prec);
}

pub fn factor(a: &Integer, x: &Integer, p: Integer, q: Integer) -> (Integer, Integer) {
    return ((a - x) / p, (a + x) / q);
}

/// Attempts to factor the given number `n` using Fermat’s Difference of Squares method.
///
/// This function iteratively searches for integers `x` and `y` such that `n = x² - y² = (x - y)(x + y)`.
/// It returns a non-trivial factor pair `(p, q)` if found.
///
/// # Arguments
/// * `n` - The number to factor.
/// * `iteration` - A mutable counter tracking the number of iterations attempted.
/// * `prec` - Precision used when printing intermediate values in scientific notation (if not in quiet mode).
/// * `quiet` - If `true`, suppresses all prompts and intermediate output.
///
/// # Returns
/// `Some((p, q))` if a non-trivial factor pair is found, otherwise `None`.
pub fn difference_of_squares(
    n: &Integer,
    iteration: &mut Integer,
    prec: u64,
    quiet: bool,
) -> Option<(Integer, Integer)> {
    let mut a: Integer = sqrt_ceil(n);
    let print_interval: Integer = Integer::const_from_unsigned(1_000_000);

    if *iteration > Integer::ONE {
        a += &*iteration - Integer::ONE;
    } else if *iteration < Integer::ONE {
        *iteration = Integer::ONE;
    }

    let mut x2: Integer = a.clone().square() - n;
    let mut _2a: Integer = Integer::TWO * &a;

    while &a < n {
        let should_print = &*iteration % &print_interval == Integer::ONE;
        let is_perf_sqr = if should_print {
            true
        } else {
            is_probably_square(&x2)
        };

        let (is_exact_sqrt, x) = if is_perf_sqr {
            sqrt_exact(&x2)
        } else {
            (false, x2.clone())
        };

        if is_exact_sqrt {
            let (p, q) = factor(&a, &x, Integer::ONE, Integer::ONE);

            if p == Integer::ONE || q == Integer::ONE || &p == n || &q == n {
                return None;
            }

            if !quiet {
                println!();
                verbose(iteration, &p, &q, prec);
                println!();
            }

            return Some((p, q));
        }

        if should_print && !quiet {
            let (p, q) = factor(&a, &x, Integer::ONE, Integer::ONE);
            verbose(iteration, &p, &q, prec);
            print!("\r");
            io::stdout().flush().unwrap();
        }

        a += Integer::ONE;
        _2a += Integer::ONE;
        x2 += &_2a;
        _2a += Integer::ONE;
        *iteration += Integer::ONE;
    }

    None
}
