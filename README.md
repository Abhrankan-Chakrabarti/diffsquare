## From the Author

This project is authored by [Abhrankan Chakrabarti](https://github.com/Abhrankan-Chakrabarti) and demonstrates an efficient implementation of Fermat’s Difference of Squares method for factoring large integers. The implementation is written in Rust and utilizes the [`malachite`](https://docs.rs/malachite/) crate for arbitrary-precision arithmetic.

Key features include:

- Support for both decimal and hexadecimal inputs.
- Command-line interface with support for optional precision and iteration control.
- Verbose scientific notation output for better readability of large numbers.

GitHub Repository: [Abhrankan-Chakrabarti/diffsquare](https://github.com/Abhrankan-Chakrabarti/diffsquare)

---

**Latest Commit:**

Add difference of squares implementation with CLI and malachite support

---

**Tags:**

`#RustLang` `#NumberTheory` `#OpenSource` `#BigInteger` `#Cryptography` `#Fermat` `#AbhrankanChakrabarti` `#Malachite`

---

## Build Instructions

Make sure you have Rust installed. Then clone the repo and build the project:

```bash
git clone https://github.com/Abhrankan-Chakrabarti/diffsquare.git
cd diffsquare
cargo build --release
```

---

## Usage Examples

Run the binary using default values or provide custom arguments:

```bash
# Using the default modulus
cargo run --release

# Using a specific modulus (in decimal)
cargo run --release -- -n 179769313486231590772930519078902473361797697894230657273430081157732675805505620686985379449212982959585501387537164015710139858647833778606925583497541085196591615128057575940752635007475935288710823649949940771895617054361149474865046711015101563940680527540071584560878577663743040086340742855278549092581

# Using a specific modulus (in hexadecimal)
cargo run --release -- -n 0xDEADBEEF1234567890

# Starting from a specific iteration
cargo run --release -- -i 1000000

# With custom precision for verbose scientific notation output
cargo run --release -- -p 30

# Combine all options
cargo run --release -- -n 0xC0FFEE123456789 -i 1000 -p 25
```

You can mix and match the flags:

| Short Flag | Long Flag   | Description                                             |
|------------|-------------|---------------------------------------------------------|
| `-n`       | `--mod`     | The modulus `n` to factor.                              |
| `-i`       | `--iter`    | Starting iteration point for the factorization.         |
| `-p`       | `--prec`    | Precision for verbose scientific notation output.       |
| `-h`       | `--help`    | Show the help message and exit.                         |
| `-v`       | `--version` | Show the version number and exit.                       |
