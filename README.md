# diffsquare

[![Crates.io](https://img.shields.io/crates/v/diffsquare.svg)](https://crates.io/crates/diffsquare)
[![Crates.io Downloads](https://img.shields.io/crates/d/diffsquare.svg)](https://crates.io/crates/diffsquare)
[![Docs.rs](https://docs.rs/diffsquare/badge.svg)](https://docs.rs/diffsquare)
[![License](https://img.shields.io/crates/l/diffsquare)](./LICENSE)

---

## From the Author

Created by [Abhrankan Chakrabarti](https://github.com/Abhrankan-Chakrabarti), this project implements an efficient version of Fermat’s Difference of Squares method for factoring large integers. Written in Rust, it leverages the [`malachite`](https://docs.rs/malachite/) crate for high-performance arbitrary-precision arithmetic.

## Recent Update – v0.3.2  

✅ **New in v0.3.2:** Added test suite for factoring large semiprimes using the `difference_of_squares` method.

---

### Key Features

`diffsquare` is a fast and lightweight CLI utility for factoring large integers using Fermat’s Difference of Squares method.

* Efficient Fermat's Difference of Squares factorization.
* Support for decimal, hexadecimal, and scientific notation input.
* Command-line interface with interactive fallback.
* Quiet mode (`-q`) disables interactive prompts and hides intermediate output — useful for piping or scripting.
* JSON output mode (`--json`) for scripting and automation.
* `--time-only` flag for displaying only the execution time (useful for benchmarking).
* Optional control over iteration starting point and precision.
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
# 🔹 Fully interactive: prompts for modulus, iteration, and precision
diffsquare

# 🔹 Provide a decimal modulus
diffsquare -n 17976931348623159077293051907890247336179769789423065727343008115

# 🔹 Provide a hexadecimal modulus
diffsquare -n 0xDEADBEEFCAFEBABE1234567890

# 🔹 Specify starting iteration (modulus still required)
diffsquare -n 0xC0FFEE123456789 -i 1000000

# 🔹 Use custom precision for verbose scientific output
diffsquare -n 0xABCD1234 -p 30

# 🔹 Combine all options and suppress output (precision not required in quiet mode)
diffsquare -n 0xCAFED00D1234 -i 50000 -q

# 🔹 JSON output for scripting and automation
diffsquare -n 0xC0FFEE123456789 --json

# 🔹 Show only execution time (no output of factors)
diffsquare -n 0xCAFED00D1234 --time-only

# 🔹 Check installed version
diffsquare --version
```

### Command-Line Flags

| Short | Long          | Description                                                      |
| ----- | ------------- | ---------------------------------------------------------------- |
| `-n`  | `--mod`       | Number to factor (supports `0x` for hex or scientific notation)  |
| `-i`  | `--iter`      | Starting iteration value                                         |
| `-p`  | `--prec`      | Precision for verbose scientific output                          |
| `-q`  | `--quiet`     | Suppress prompts and intermediate output                         |
|       | `--json`      | Print result as JSON (suppresses all other output)               |
|       | `--time-only` | Display only the execution time (useful for benchmarking)        |
| `-h`  | `--help`      | Show usage help                                                  |
| `-v`  | `--version`   | Show version                                                     |

---

## Tags
**Tags**  
`#RustLang` `#NumberTheory` `#OpenSource` `#BigInteger` `#Cryptography` `#Fermat` `#AbhrankanChakrabarti` `#Malachite`
