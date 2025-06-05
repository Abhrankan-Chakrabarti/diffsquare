use clap::{ArgAction, Parser};
use anyhow::{Result, anyhow};
use malachite::{
    Integer,
    base::num::conversion::traits::{FromSciString, FromStringBase},
};
use std::{io::{self, Write}, time::Instant};
use diffsquare::factor::difference_of_squares;

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
    /// Number to factor (hex prefix 0x or scientific notation supported)
    #[arg(short = 'n', long = "mod", display_order = 1)]
    modulus: Option<String>,

    /// Starting iteration value
    #[arg(short, long, display_order = 2)]
    iter: Option<String>,

    /// Precision for verbose scientific output
    #[arg(short, long, display_order = 3)]
    prec: Option<u64>,

    /// Suppress prompts and intermediate output
    #[arg(short, long, help = "Suppress prompts and verbose output", display_order = 4)]
    quiet: bool,

    /// Show help
    #[arg(short = 'h', long = "help", action = ArgAction::Help, display_order = 100)]
    help: Option<bool>,

    /// Show version
    #[arg(short = 'v', long = "version", action = ArgAction::Version, display_order = 101)]
    version: Option<bool>,
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

    let n = match args.modulus {
        Some(ref val) => parse_bigint(val)?,
        None => {
            if args.quiet {
                return Err(anyhow!("Missing required input -n in quiet mode"));
            } else {
                parse_bigint(&input("Enter the modulus: ")?)?
            }
        }
    };

    let mut iter = if args.quiet {
        match args.iter {
            Some(ref val) => parse_bigint(val)?,
            None => Integer::from(1),
        }
    } else {
        match args.iter {
            Some(ref val) => parse_bigint(val)?,
            None => parse_bigint(&input("Enter the starting iteration: ")?)?,
        }
    };

    let prec = if args.quiet {
        args.prec.unwrap_or(0)
    } else {
        match args.prec {
            Some(val) => val + 1,
            None => input("Enter the verbose precision: ")?.parse::<u64>()? + 1,
        }
    };

    let start_time = Instant::now();

    if let Some((p, q)) = difference_of_squares(&n, &mut iter, prec, args.quiet) {
        println!("\n✅ Factors of n:\n\np =\n{}\n\nq =\n{}\n", p, q);
    } else {
        println!("❌ Failed to factor the given number.");
    }

    let duration = start_time.elapsed();
    if !args.quiet {
        println!("⏱️  Execution time: {:?}", duration);
    }

    Ok(())
}
