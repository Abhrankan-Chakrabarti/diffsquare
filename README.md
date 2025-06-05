# diffsquare

[![Crates.io](https://img.shields.io/crates/v/diffsquare.svg)](https://crates.io/crates/diffsquare)
[![Crates.io Downloads](https://img.shields.io/crates/d/diffsquare.svg)](https://crates.io/crates/diffsquare)
[![Docs.rs](https://docs.rs/diffsquare/badge.svg)](https://docs.rs/diffsquare)
[![License](https://img.shields.io/crates/l/diffsquare)](./LICENSE)

---

## From the Author

This project is authored by [Abhrankan Chakrabarti](https://github.com/Abhrankan-Chakrabarti) and showcases an efficient implementation of Fermat’s Difference of Squares method for factoring large integers. Written in Rust, it leverages the [`malachite`](https://docs.rs/malachite/) crate for high-performance arbitrary-precision arithmetic.

### Key Features

* Support for both decimal and hexadecimal input formats.
* Command-line interface with interactive fallback.
* Optional control over iteration starting point and scientific precision.
* Verbose output in scientific notation for readability of large numbers.
* Quiet mode (`-q`) to suppress prompts and reduce output.
* Execution time displayed after successful factorization.

> GitHub Repository: [Abhrankan-Chakrabarti/diffsquare](https://github.com/Abhrankan-Chakrabarti/diffsquare)

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

To build from source:

```bash
git clone https://github.com/Abhrankan-Chakrabarti/diffsquare.git
cd diffsquare
cargo build --release
```

---

## Usage Examples

You can run `diffsquare` interactively or provide flags for automation:

```bash
# Fully interactive: enter modulus, iteration, and precision when prompted
diffsquare

# With a specific modulus (decimal)
diffsquare -n 17976931348623159...

# With a hexadecimal modulus
diffsquare -n 0xDEADBEEF1234567890

# Specify starting iteration
diffsquare -i 1000000

# Use custom precision for scientific notation
diffsquare -p 30

# All options combined
diffsquare -n 0xC0FFEE123456789 -i 1000 -p 25 -q
```

### Command-Line Flags

| Short | Long        | Description                                                      |
| ----- | ----------- | ---------------------------------------------------------------- |
| `-n`  | `--mod`     | Number to factor (supports `0x` for hex, or scientific notation) |
| `-i`  | `--iter`    | Starting iteration value                                         |
| `-p`  | `--prec`    | Precision for verbose scientific output                          |
| `-q`  | `--quiet`   | Suppress prompts and intermediate output                         |
| `-h`  | `--help`    | Show help message                                                |
| `-v`  | `--version` | Show version                                                     |

---

## Recent Update

Release v0.2.2

---

## Tags

`#RustLang` `#NumberTheory` `#OpenSource` `#BigInteger` `#Cryptography` `#Fermat` `#AbhrankanChakrabarti` `#Malachite`

