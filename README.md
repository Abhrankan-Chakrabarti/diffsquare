# diffsquare

[![Crates.io](https://img.shields.io/crates/v/diffsquare.svg)](https://crates.io/crates/diffsquare)
[![Crates.io Downloads](https://img.shields.io/crates/d/diffsquare.svg)](https://crates.io/crates/diffsquare)
[![Docs.rs](https://docs.rs/diffsquare/badge.svg)](https://docs.rs/diffsquare)
[![License](https://img.shields.io/crates/l/diffsquare)](./LICENSE)

---

## From the Author

Created by [Abhrankan Chakrabarti](https://github.com/Abhrankan-Chakrabarti), this project implements an efficient version of Fermat’s Difference of Squares method for factoring large integers. Written in Rust, it leverages the [`malachite`](https://docs.rs/malachite/) crate for high-performance arbitrary-precision arithmetic.

---

## Recent Update – v0.7.0

🚀 **New in v0.7.0:**

* ✅ **`--input <FILE>` flag**

  * Allows reading newline-separated numbers from a specified file.
  * Supports batch factorization of large input sets via file input.
  * Fully compatible with `--json`, `--csv`, `--quiet`, `--time-only`, and `--output` modes.
  * Can be combined with `--threads` to enable parallel factorization.

* ✅ **Thread pool configuration for batch input**

  * `--threads N` lets you control the number of Rayon threads for file or stdin input.
  * Improves performance tuning for large workloads.

* 🚀 **Progress bar improvements**

  * Displays progress for file and stdin input when not in `--quiet`, `--json`, `--csv`, or `--time-only` mode.
  * Cleaner and more informative progress display using `indicatif`.

---

## Key Features

`diffsquare` is a fast and lightweight CLI utility for factoring large integers using Fermat’s Difference of Squares method.

* Efficient Fermat's Difference of Squares factorization.
* Parallelized using [`rayon`](https://docs.rs/rayon) for faster factorization on multi-core systems (since v0.5.0).
* Optional file input for batch factorization (`--input`) (new in v0.7.0).
* Optional file output with `--output` to save results (since v0.6.0).
* Control thread count for batch processing with `--threads` (new in v0.7.0).
* Progress bar for file and stdin batch input (new in v0.7.0).
* Optional `--timeout N` to limit maximum time per factorization in milliseconds (since v0.6.1).
* CSV output support via `--csv` flag (since v0.6.1).
* JSON output mode (`--json`) for scripting and automation (since v0.3.0).
* Quiet mode (`-q`) disables prompts and hides intermediate output — ideal for scripting.
* `--time-only` flag for showing only execution time — useful for benchmarking (since v0.3.1).
* Batch factorization via `--stdin` (since v0.4.0).
* Support for decimal, hexadecimal, and scientific notation input.
* Command-line interface with interactive fallback.
* Optional control over iteration starting point (`--iter`) and precision (`--prec`).
* Scientific notation used in verbose mode for large integer readability.
* Execution time displayed after successful factorization.

> GitHub Repository: [`diffsquare`](https://github.com/Abhrankan-Chakrabarti/diffsquare)

---

## Installation

Install via Cargo:

```bash
cargo install diffsquare
```

Ensure `$HOME/.cargo/bin` is in your `PATH`:

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

---

## Build Instructions

To build from source manually:

```bash
git clone https://github.com/Abhrankan-Chakrabarti/diffsquare.git
cd diffsquare
cargo build --release
```

---

## 🔧 Usage Examples

Run `diffsquare` interactively or use flags for automation:

```bash
# 🔹 Fully interactive: prompts for modulus
diffsquare

# 🔹 Provide a decimal modulus
diffsquare -n 17976931348623159077293051907890247336179769789423065727343008115

# 🔹 Provide a hexadecimal modulus
diffsquare -n 0xDEADBEEFCAFEBABE1234567890

# 🔹 Specify starting iteration
diffsquare -n 0xC0FFEE123456789 -i 1000000

# 🔹 Use custom precision for verbose scientific output
diffsquare -n 0xABCD1234 -p 30

# 🔹 Quiet mode, suppress intermediate output
diffsquare -n 0xCAFED00D1234 -q

# 🔹 JSON output for scripting
diffsquare -n 0xC0FFEE123456789 --json

# 🔹 Show only execution time (for benchmarking)
diffsquare -n 0xCAFED00D1234 --time-only

# 🔹 Batch factorization from stdin
echo -e "2761929023323646159\n3189046231347719467" | diffsquare --stdin

# 🔹 Batch factorization from file input
diffsquare --input numbers.txt

# 🔹 Batch with threads and JSON output
diffsquare --input numbers.txt --threads 4 --json

# 🔹 Batch with CSV output and save to file
diffsquare --input numbers.txt --csv --output results.csv
```

---

### Command-Line Flags

| Short | Long          | Description                                                     |
| ----- | ------------- | --------------------------------------------------------------- |
| `-n`  | `--mod`       | Number to factor (supports `0x` for hex or scientific notation) |
| `-i`  | `--iter`      | Starting iteration value                                        |
| `-p`  | `--prec`      | Precision for verbose scientific output                         |
| `-q`  | `--quiet`     | Suppress prompts and intermediate output                        |
|       | `--json`      | Output result in JSON format                                    |
|       | `--csv`       | Output result as CSV                                            |
|       | `--time-only` | Show only execution time                                        |
|       | `--stdin`     | Read newline-separated input from stdin                         |
|       | `--input`     | Read newline-separated input from file                          |
|       | `--threads`   | Number of threads to use (default: 1)                           |
|       | `--output`    | Output results to file                                          |
|       | `--timeout`   | Timeout in milliseconds for each factorization                  |
| `-h`  | `--help`      | Show usage help                                                 |
| `-v`  | `--version`   | Show version                                                    |

---

## Tags

**Tags**
`#RustLang` `#NumberTheory` `#OpenSource` `#BigInteger` `#Cryptography` `#Fermat` `#AbhrankanChakrabarti` `#Malachite`

