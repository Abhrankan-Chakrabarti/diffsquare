use anyhow::{anyhow, Result};
use clap::{ArgAction, Parser};
use diffsquare::factor::difference_of_squares;
use malachite::{
    base::num::conversion::traits::{FromSciString, FromStringBase},
    Integer,
};
use rayon::prelude::*;
use serde::Serialize;
use std::{
    io::{self, Write},
    process::exit,
    time::Instant,
};

/// Fast and efficient Fermat factorization CLI
#[derive(Parser)]
#[command(
    version = env!("CARGO_PKG_VERSION"),
    disable_help_flag = true,
    disable_version_flag = true,
    about,
    long_about = None
)]
struct Args {
    /// Number to factor (supports `0x` for hex or scientific notation)
    #[arg(short = 'n', long = "mod", display_order = 1)]
    modulus: Option<String>,

    /// Starting iteration value
    #[arg(short, long, display_order = 2)]
    iter: Option<String>,

    /// Precision for verbose scientific output
    #[arg(short, long, display_order = 3)]
    prec: Option<u64>,

    /// Suppress prompts and intermediate output
    #[arg(
        short,
        long,
        help = "Suppress prompts and intermediate output",
        display_order = 4
    )]
    quiet: bool,

    /// Output result in JSON format
    #[arg(
        long,
        help = "Print result as JSON (suppresses all other output)",
        display_order = 5
    )]
    json: bool,

    /// Show only execution time
    #[arg(
        long,
        help = "Display only the execution time (useful for benchmarking)",
        display_order = 6
    )]
    time_only: bool,

    /// Read newline-separated input from stdin
    #[arg(
        long,
        help = "Read newline-separated numbers from standard input",
        display_order = 7
    )]
    stdin: bool,

    #[arg(
        long,
        help = "Number of threads for parallel factorization (default: 1)",
        display_order = 8
    )]
    threads: Option<usize>,

    /// Show usage help
    #[arg(short = 'h', long = "help", action = ArgAction::Help, display_order = 100)]
    help: Option<bool>,

    /// Show version
    #[arg(short = 'v', long = "version", action = ArgAction::Version, display_order = 101)]
    version: Option<bool>,
}

#[derive(Serialize)]
struct JsonResult {
    modulus: String,
    factor_1: String,
    factor_2: String,
    iterations: String,
    time_ms: u128,
}

fn input(prompt: &str) -> Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    Ok(s.trim().to_string())
}

fn parse_bigint(s: &str) -> Result<Integer> {
    if s.starts_with("0x") || s.starts_with("0X") {
        Integer::from_string_base(16, &s[2..]).ok_or_else(|| anyhow!("Invalid hex"))
    } else {
        Integer::from_sci_string(s).ok_or_else(|| anyhow!("Invalid integer"))
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.stdin {
        let prec = match args.prec {
            Some(p) if p > 0 => p,
            _ => 30, // Provide a sensible default if not given
        };

        let inputs: Vec<_> = io::stdin()
            .lines()
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .filter(|line| !line.trim().is_empty())
            .collect();

        if let Some(t) = args.threads {
            rayon::ThreadPoolBuilder::new()
                .num_threads(t)
                .build_global()?;
        }

        inputs.par_iter().for_each(|input| {
            let n = match parse_bigint(input) {
                Ok(val) => val,
                Err(e) => {
                    eprintln!("❌ Error parsing '{}': {e}", input);
                    return;
                }
            };

            let mut iter = Integer::from(1);
            let start_time = Instant::now();

            if let Some((p, q)) = difference_of_squares(
                &n,
                &mut iter,
                prec,
                args.quiet || args.json || args.time_only,
            ) {
                let duration = start_time.elapsed();

                if args.json {
                    let result = JsonResult {
                        modulus: n.to_string(),
                        factor_1: p.to_string(),
                        factor_2: q.to_string(),
                        iterations: iter.to_string(),
                        time_ms: duration.as_millis(),
                    };
                    println!("{}", serde_json::to_string_pretty(&result).unwrap());
                } else if args.time_only {
                    println!("{}", duration.as_millis());
                } else {
                    println!("\n✅ Factors of {}:\n\np = {}\nq = {}", n, p, q);
                    if !args.quiet {
                        println!("⏱️  Execution time: {:?}", duration);
                    }
                }
            } else {
                if args.json {
                    eprintln!(
                        "{{\n  \"modulus\": \"{}\",\n  \"error\": \"Factorization failed\"\n}}",
                        n
                    );
                } else {
                    eprintln!("❌ Failed to factor {}.", n);
                }
            }
        });

        return Ok(());
    }

    // Single-modulus mode
    let n = match args.modulus {
        Some(ref val) => parse_bigint(val)?,
        None => {
            if args.quiet || args.json || args.time_only {
                return Err(anyhow!(
                "❌ Missing required argument: -n / --mod must be provided in quiet, JSON, or time-only mode"
            ));
            } else {
                parse_bigint(&input("Enter the modulus: ")?)?
            }
        }
    };

    let mut iter = match args.iter {
        Some(ref val) => parse_bigint(val)?,
        None => {
            if args.quiet || args.json || args.time_only {
                Integer::from(1)
            } else {
                parse_bigint(&input("Enter the starting iteration: ")?)?
            }
        }
    };

    let prec = if args.quiet || args.json || args.time_only {
        args.prec.unwrap_or(0)
    } else {
        match args.prec {
            Some(val) => val + 1,
            None => input("Enter the verbose precision: ")?.parse::<u64>()? + 1,
        }
    };

    let start_time = Instant::now();

    if let Some((p, q)) = difference_of_squares(
        &n,
        &mut iter,
        prec,
        args.quiet || args.json || args.time_only,
    ) {
        let duration = start_time.elapsed();

        if args.json {
            let result = JsonResult {
                modulus: n.to_string(),
                factor_1: p.to_string(),
                factor_2: q.to_string(),
                iterations: iter.to_string(),
                time_ms: duration.as_millis(),
            };
            println!("{}", serde_json::to_string_pretty(&result)?);
        } else if args.time_only {
            println!("{}", duration.as_millis());
        } else {
            println!("\n✅ Factors of n:\n\np =\n{}\n\nq =\n{}\n", p, q);
            if !args.quiet {
                println!("⏱️  Execution time: {:?}", duration);
            }
        }
    } else {
        if args.json {
            eprintln!(
                "{{\n  \"modulus\": \"{}\",\n  \"error\": \"Factorization failed\"\n}}",
                n
            );
        } else {
            eprintln!("❌ Failed to factor the given number.");
        }
        exit(1);
    }

    Ok(())
}
