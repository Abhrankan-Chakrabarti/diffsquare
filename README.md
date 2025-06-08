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

v0.2.3: Refine CLI experience, prompt behavior, and help messages

---

## Tags

`#RustLang` `#NumberTheory` `#OpenSource` `#BigInteger` `#Cryptography` `#Fermat` `#AbhrankanChakrabarti` `#Malachite`

