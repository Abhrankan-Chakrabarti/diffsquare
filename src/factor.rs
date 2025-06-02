use malachite::{
    Integer,
    base::{
        rounding_modes::RoundingMode::Floor,
        num::{
            arithmetic::traits::Square,
            basic::traits::{One, Two},
            conversion::{
                string::options::ToSciOptions,
                traits::ToSci,
            },
        },
    },
};
use std::{io, io::Write};
use crate::sqrt::{sqrt_exact, sqrt_ceil, is_probably_square};


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

/// Attempts to factor the given number `n` using the difference of squares method.
/// 
/// # Arguments
/// * `n` - The number to factor.
/// * `iteration` - A mutable counter for the number of iterations performed.
/// * `prec` - Controls how many digits of precision are used when displaying
///   `iteration`, `p`, and `q` in scientific notation during verbose output.
pub fn difference_of_squares(n: &Integer, iteration: &mut Integer, prec: u64) -> Option<(Integer, Integer)> {
    // Start a at ceil(sqrt(n))
    let mut a: Integer = sqrt_ceil(n);
    let print_interval: Integer = Integer::const_from_unsigned(1_000_000);

    if *iteration > Integer::ONE {
        a += &*iteration - Integer::ONE; // Start from a specific iteration
    } else if *iteration < Integer::ONE {
        *iteration = Integer::ONE;
    }

    let mut x2: Integer = a.clone().square() - n;
    let mut _2a: Integer = Integer::TWO * &a;

    // Loop to find x^2 as a perfect square
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
            // Factor n into p and q
            let (p, q) = factor(&a, &x, Integer::ONE, Integer::ONE);

            // Avoid trivial factorizations like (1, n) or (n, 1)
            if p == Integer::ONE || q == Integer::ONE || &p == n || &q == n {
                return None;
            }

            println!();
            verbose(iteration, &p, &q, prec);
            println!();

            return Some((p, q));
        }

        // Print verbose info
        if should_print {
            let (p, q) = factor(&a, &x, Integer::ONE, Integer::ONE);
            verbose(iteration, &p, &q, prec);
            print!("\r");
            io::stdout().flush().unwrap();
        }

        // Increment and try again
        a += Integer::ONE;
        _2a += Integer::ONE;
        x2 += &_2a;
        _2a += Integer::ONE;
        *iteration += Integer::ONE;
    }

    None
}
