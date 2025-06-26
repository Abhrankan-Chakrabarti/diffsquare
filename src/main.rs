use anyhow::{anyhow, Result};
use atty;
use clap::{ArgAction, Parser};
use diffsquare::factor::difference_of_squares;
use indicatif::{ProgressBar, ProgressStyle};
use malachite::{
    base::num::conversion::traits::{FromSciString, FromStringBase},
    Integer,
};
use rayon::prelude::*;
use serde::Serialize;
use std::{
    fs::OpenOptions,
    io::{self, Read, Write},
    thread,
    time::{Duration, Instant},
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
    #[arg(short, long, display_order = 4)]
    quiet: bool,

    /// Output result in JSON format
    #[arg(long, display_order = 5)]
    json: bool,

    /// Output result as CSV
    #[arg(long, display_order = 6)]
    csv: bool,

    /// Show only execution time
    #[arg(long, display_order = 7)]
    time_only: bool,

    /// Read newline-separated input from stdin
    #[arg(long, display_order = 8)]
    stdin: bool,

    /// Read newline-separated input from file
    #[arg(long, display_order = 9)]
    input: Option<String>,

    /// Number of threads for batch factorization (`--stdin` / `--input`) (default: logical CPUs)
    #[arg(long, display_order = 10)]
    threads: Option<usize>,

    /// Output results to file
    #[arg(long, display_order = 11)]
    output: Option<String>,

    /// Timeout in milliseconds for each factorization
    #[arg(long, display_order = 12)]
    timeout: Option<u64>,

    /// Show usage help
    #[arg(short = 'h', long = "help", action = ArgAction::Help, display_order = 100)]
    help: Option<bool>,

    /// Show version
    #[arg(short = 'v', long = "version", action = ArgAction::Version, display_order = 101)]
    version: Option<bool>,
}

impl Args {
    fn is_quiet(&self) -> bool {
        self.quiet || self.json || self.csv || self.time_only
    }
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

fn write_output(file: &str, content: &str) -> Result<()> {
    let mut f = OpenOptions::new().create(true).append(true).open(file)?;
    writeln!(f, "{}", content)?;
    Ok(())
}

trait JoinTimeout<T> {
    fn join_timeout(self, dur: Duration) -> Option<T>;
}

impl<T: Send + 'static> JoinTimeout<T> for thread::JoinHandle<T> {
    fn join_timeout(self, dur: Duration) -> Option<T> {
        use std::sync::mpsc::channel;
        let (tx, rx) = channel();
        thread::spawn(move || {
            if let Ok(val) = self.join() {
                let _ = tx.send(val);
            }
        });
        rx.recv_timeout(dur).ok()
    }
}

fn factor_and_print(
    n: Integer,
    iter: Integer,
    prec: u64,
    args: &Args,
    write_if_needed: &dyn Fn(&str) -> Result<()>,
) -> Result<()> {
    let start_time = Instant::now();
    let quiet = args.is_quiet();
    let mut iter_clone = iter.clone();

    let result = if let Some(ms) = args.timeout {
        let n_clone = n.clone();
        let handle = thread::spawn(move || {
            difference_of_squares(&n_clone, &mut iter_clone, prec, quiet)
                .map(|(p, q)| (p, q, iter_clone))
        });
        handle.join_timeout(Duration::from_millis(ms)).flatten()
    } else {
        difference_of_squares(&n, &mut iter_clone, prec, quiet).map(|(p, q)| (p, q, iter_clone))
    };

    let duration = start_time.elapsed();

    if let Some((p, q, iterations)) = result {
        if args.csv {
            let out = format!("{},{},{},{},{}", n, p, q, iterations, duration.as_millis());
            println!("{}", &out);
            write_if_needed(&out)?;
        } else if args.json {
            let result = JsonResult {
                modulus: n.to_string(),
                factor_1: p.to_string(),
                factor_2: q.to_string(),
                iterations: iterations.to_string(),
                time_ms: duration.as_millis(),
            };
            let out = serde_json::to_string_pretty(&result)?;
            println!("{}", &out);
            write_if_needed(&out)?;
        } else if args.time_only {
            let out = duration.as_millis().to_string();
            println!("{}", &out);
            write_if_needed(&out)?;
        } else if args.quiet {
            let out = format!("{} {}", p, q);
            println!("{}", &out);
            write_if_needed(&out)?;
        } else {
            let out = format!(
                "\n✅ Factors of {}:\n\np = {}\nq = {}\n⏱️  Execution time: {:?}",
                n, p, q, duration
            );
            println!("{}", &out);
            write_if_needed(&out)?;
        }
    } else {
        let err = if args.csv {
            format!("{},ERROR,ERROR,ERROR,ERROR", n)
        } else if args.json {
            format!(
                "{{\n  \"modulus\": \"{}\",\n  \"error\": \"Factorization failed or timed out\"\n}}",
                n
            )
        } else {
            format!("❌ Failed to factor {} (timeout or failure).", n)
        };
        eprintln!("{}", &err);
        write_if_needed(&err)?;
    }

    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();

    let write_if_needed = |content: &str| -> Result<()> {
        if let Some(ref file) = args.output {
            write_output(file, content)?;
        }
        Ok(())
    };

    let prec = args.prec.unwrap_or(30);

    if args.stdin || args.input.is_some() {
        let inputs: Vec<String> = if args.stdin {
            io::stdin()
                .lines()
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .filter(|line| !line.trim().is_empty())
                .collect()
        } else {
            let file = args.input.as_ref().unwrap();
            std::fs::read_to_string(file)?
                .lines()
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect()
        };

        if let Some(t) = args.threads {
            rayon::ThreadPoolBuilder::new()
                .num_threads(t)
                .build_global()?;
        }

        let pb = if !args.is_quiet() {
            let pb = ProgressBar::new(inputs.len() as u64);
            pb.set_style(
                ProgressStyle::with_template("{bar:40.cyan/blue} {pos}/{len} [{elapsed_precise}]")
                    .unwrap()
                    .progress_chars("=> "),
            );
            Some(pb)
        } else {
            None
        };

        inputs.par_iter().for_each(|input| {
            let n = match parse_bigint(input) {
                Ok(val) => val,
                Err(e) => {
                    eprintln!("❌ Error parsing '{}': {e}", input);
                    return;
                }
            };
            let iter = Integer::from(1);
            let _ = factor_and_print(n, iter, prec, &args, &write_if_needed);
            if let Some(ref pb) = pb {
                pb.inc(1);
            }
        });

        if let Some(ref pb) = pb {
            pb.finish_with_message("Done");
        }
    } else {
        let n = if let Some(ref m) = args.modulus {
            parse_bigint(m)?
        } else if !atty::is(atty::Stream::Stdin) {
            // Read full piped input (supports multi-line with backslashes)
            let mut s = String::new();
            io::stdin().read_to_string(&mut s)?;
            let cleaned = s.replace("\\\n", "").replace('\n', "").trim().to_string();
            parse_bigint(&cleaned)?
        } else if args.is_quiet() {
            return Err(anyhow!(
                "Modulus must be provided in quiet/json/csv/time-only mode (prompts are disabled)"
            ));
        } else {
            let m = input("Modulus: ")?;
            parse_bigint(&m)?
        };

        let iter = if let Some(ref i) = args.iter {
            parse_bigint(i)?
        } else {
            Integer::from(1)
        };

        factor_and_print(n, iter, prec, &args, &write_if_needed)?;
    }

    Ok(())
}
