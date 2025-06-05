# diffsquare

[![Crates.io](https://img.shields.io/crates/v/diffsquare.svg)](https://crates.io/crates/diffsquare)
[![Crates.io Downloads](https://img.shields.io/crates/d/diffsquare.svg)](https://crates.io/crates/diffsquare)
[![Docs.rs](https://docs.rs/diffsquare/badge.svg)](https://docs.rs/diffsquare)
[![License](https://img.shields.io/crates/l/diffsquare)](./LICENSE)

---

## From the Author

This project is authored by [Abhrankan Chakrabarti](https://github.com/Abhrankan-Chakrabarti) and demonstrates an efficient implementation of Fermat’s Difference of Squares method for factoring large integers. The implementation is written in Rust and utilizes the [`malachite`](https://docs.rs/malachite/) crate for arbitrary-precision arithmetic.

Key features include:

* Support for both decimal and hexadecimal inputs.
* Command-line interface with support for optional precision and iteration control.
* Verbose scientific notation output for better readability of large numbers.

GitHub Repository: [Abhrankan-Chakrabarti/diffsquare](https://github.com/Abhrankan-Chakrabarti/diffsquare)

---

## Installation

To install `diffsquare` directly using Cargo:

```bash
cargo install diffsquare
```

Make sure `$HOME/.cargo/bin` is in your system's `PATH` to run `diffsquare` from anywhere:

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

---

## Build Instructions

If you'd rather clone and build manually:

```bash
git clone https://github.com/Abhrankan-Chakrabarti/diffsquare.git
cd diffsquare
cargo build --release
```

---

## Usage Examples

Run the binary using default values or provide custom arguments:

```bash
# Provide modulus interactively
diffsquare

# Using a specific modulus (in decimal)
diffsquare -n 179769313486231590772930519078902473361797697894230657273430081157732675805505620686985379449212982959585501387537164015710139858647833778606925583497541085196591615128057575940752635007475935288710823649949940771895617054361149474865046711015101563940680527540071584560878577663743040086340742855278549092581

# Using a specific modulus (in hexadecimal)
diffsquare -n 0xDEADBEEF1234567890

# Starting from a specific iteration
diffsquare -i 1000000

# With custom precision for verbose scientific notation output
diffsquare -p 30

# Combine all options
diffsquare -n 0xC0FFEE123456789 -i 1000 -p 25
```

You can mix and match the flags:

| Short Flag | Long Flag   | Description                                                       |
| ---------- | ----------- | ----------------------------------------------------------------- |
| `-n`       | `--mod`     | Number to factor (hex prefix 0x or scientific notation supported) |
| `-i`       | `--iter`    | Starting iteration value                                          |
| `-p`       | `--prec`    | Precision for verbose scientific output                           |
| `-h`       | `--help`    | Show help                                                         |
| `-v`       | `--version` | Show version                                                      |

---

**Latest Commit:**

feat: remove default modulus and migrate to clap for CLI parsing

---

**Tags:**

`#RustLang` `#NumberTheory` `#OpenSource` `#BigInteger` `#Cryptography` `#Fermat` `#AbhrankanChakrabarti` `#Malachite`

