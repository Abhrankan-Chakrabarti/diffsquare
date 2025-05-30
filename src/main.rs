use anyhow::anyhow;
use malachite::{
    Integer,
    base::{
        rounding_modes::RoundingMode::Floor,
        num::{
            arithmetic::traits::{Square, FloorSqrt},
            basic::traits::{Zero, One, Two},
            conversion::{
                string::options::ToSciOptions,
                traits::{FromSciString, FromStringBase, ToSci},
            },
        },
    },
};
use std::{env, io, io::Write};

fn input(prompt: &str) -> anyhow::Result<String> {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut n = String::new();
    io::stdin().read_line(&mut n)?;
    Ok(n.trim().to_string())
}

fn parse(args: Vec<String>, options: Vec<&str>) -> Option<Option<String>> {
    let mut found = false;
    for arg in args {
        if found {
            return Some(Some(arg.clone()));
        }
        if options.contains(&arg.as_str()) {
            found = true;
        }
    }
    if found {
        return Some(None);
    }
    return None;
}

fn starts_with(string: &str, prefixes: &[&str]) -> bool {
    prefixes.iter().any(|&prefix| string.starts_with(prefix))
}

fn bigint(n: String) -> anyhow::Result<Integer> {
    let parsed = if starts_with(&n, &["0x", "0X"]) {
        Integer::from_string_base(16, &n[2..])
    } else {
        Integer::from_sci_string(&n)
    };

    parsed.ok_or_else(|| anyhow!("Invalid integer format"))
}

fn int(n: String) -> anyhow::Result<u64> {
    n.parse().map_err(|_| anyhow!("Unable to parse integer"))
}

fn scinot(n: &Integer, prec: u64) {
    let mut options = ToSciOptions::default();
    options.set_precision(prec);
    options.set_rounding_mode(Floor);
    options.set_force_exponent_plus_sign(true);
    print!("{}", n.to_sci_with_options(options));
}

fn verbose(iteration: &Integer, p: &Integer, q: &Integer, prec: u64) {
    print!("Iteration: ");
    scinot(iteration, prec);
    print!(" p = ");
    scinot(p, prec);
    print!(" q = ");
    scinot(q, prec);
}

static PRIMES: [u64; 10] = [3, 5, 7, 11, 13, 17, 19, 23, 29, 31];

/// Function to check if a number is a perfect square
fn sqr_perf(n: &Integer) -> bool {
    for i in 0..10 {
        let p = PRIMES[i];
        let q = n % Integer::from(p);
        if q == Integer::ZERO {
            continue;
        }
        let mut nq = true;
        for j in 1..p {
            if Integer::from(j * j % p) == q {
                nq = false;
                break;
            }
        }
        if nq {
            return false;
        }
    }
    return true;
}

/// Function to check if a number is a perfect square and return its square root
fn sqrt_exact(n: &Integer) -> (bool, Integer) {
    let approx_sqrt = n.floor_sqrt();
    if approx_sqrt.clone().square() == *n {
        return (true, approx_sqrt);
    }
    return (false, approx_sqrt);
}

/// Function to compute ceil(sqrt(n))
fn sqrt_ceil(n: &Integer) -> Integer {
    let (is_exact_sqrt, k) = sqrt_exact(n);
    if is_exact_sqrt {
        return k;
    }
    return k + Integer::ONE;
}

fn factor(a: &Integer, x: &Integer, p: Integer, q: Integer) -> (Integer, Integer) {
    return ((a - x) / p, (a + x) / q);
}

fn help(bin_path: String) {
    println!("Usage: {bin_path} [options...]");
    println!(" -n/--mod <modulus>: The modulus n to factor.");
    println!(" -i/--iter <iteration>: Starting iteration point for the factorization.");
    println!(" -p/--prec <precision>: Precision for verbose scientific notation output.");
    println!(" -h/--help: Show this help message and exit.");
    println!(" -v/--version: Show version number and exit.");
}

fn version(bin_path: String) {
    println!("diffsquare v0.1.0 ({bin_path})");
}

/// Perform the difference of squares method to find factors of n.
/// Returns the factors if found, otherwise None.
fn difference_of_squares(n: &Integer, iteration: &mut Integer, prec: u64) -> Option<(Integer, Integer)> {
    // Start a at ceil(sqrt(n))
    let mut a: Integer = sqrt_ceil(n);
    let _100k: Integer = Integer::const_from_unsigned(100000);

    if *iteration > Integer::ONE {
        a += &*iteration - Integer::ONE; // Start from a specific iteration
    } else if *iteration < Integer::ONE {
        *iteration = Integer::ONE;
    }

    let mut x2: Integer = a.clone().square() - n;
    let mut _2a: Integer = Integer::TWO * &a;

    // Loop to find x^2 as a perfect square
    while &a < n {
        let (is_exact_sqrt, x, is_perf_sqr);
        let should_print = &*iteration % &_100k == Integer::ONE;
        if should_print {
            is_perf_sqr = true;
        } else {
            is_perf_sqr = sqr_perf(&x2);
        }
        if is_perf_sqr {
            (is_exact_sqrt, x) = sqrt_exact(&x2);
        } else {
            (is_exact_sqrt, x) = (false, x2.clone());
        }
        if is_exact_sqrt {
            // Factor n into p and q
            let (p, q) = factor(&a, &x, Integer::ONE, Integer::ONE);
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

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if let Some(_arg) = parse(args.clone(), vec!["-h", "--help"]) {
        help(args[0].clone());
        return Ok(());
    }
    if let Some(_arg) = parse(args.clone(), vec!["-v", "--version"]) {
        version(args[0].clone());
        return Ok(());
    }

    // Provide n as a command-line argument, or let the program use the default value.
    let n: Integer = bigint(
        {
            if let Some(Some(n_str)) = parse(args.clone(), vec!["-n", "--mod"]) {
                n_str
            } else {
                "179769313486231590772930519078902473361797697894230657273430081157732675805505620686985379449212982959585501387537164015710139858647833778606925583497541085196591615128057575940752635007475935288710823649949940771895617054361149474865046711015101563940680527540071584560878577663743040086340742855278549092581".to_string()
            }
        },
    )?;

    // Starting iteration point for the factorization.
    let mut iteration: Integer = bigint(
        {
            if let Some(Some(i_str)) = parse(args.clone(), vec!["-i", "--iter"]) {
                i_str
            } else {
                input("Enter the starting iteration: ")?
            }
        },
    )?;

    // Precision for verbose scientific notation output.
    let prec: u64 = int(
    {
        if let Some(Some(p_str)) = parse(args.clone(), vec!["-p", "--prec"]) {
            p_str
        } else {
            input("Enter the verbose precision: ")?
        }
    },
    )? + 1;

    if let Some((p, q)) = difference_of_squares(&n, &mut iteration, prec) {
        println!("Factors found:");
        println!("p = {}", p);
        println!("q = {}", q);
    }

    Ok(())
}
