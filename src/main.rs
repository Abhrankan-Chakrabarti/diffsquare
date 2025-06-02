use anyhow::anyhow;
use malachite::{
    Integer,
    base::num::conversion::traits::{FromSciString, FromStringBase},
};
use std::{env, io, io::Write};
use diffsquare::factor::difference_of_squares;

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

fn help(bin_path: String) {
    println!("Usage: {bin_path} [options]");
    println!("  -n, --mod <modulus>      Number to factor (default: 1024-bit number).");
    println!("  -i, --iter <iteration>   Starting iteration (prompted if not specified).");
    println!("  -p, --prec <precision>   Precision for scientific notation output (prompted if not specified).");
    println!("  -h, --help               Show help.");
    println!("  -v, --version            Show version.");
}

fn version(bin_path: String) {
    println!("diffsquare v0.1.2 ({bin_path})");
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
        println!("\n✅ Factors of n:");
        println!("p =\n{}\n", p);
        println!("q =\n{}\n", q);
    }

    Ok(())
}
