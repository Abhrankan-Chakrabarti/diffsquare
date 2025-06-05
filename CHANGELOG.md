## \[v0.2.2] - 07-06-2025

### Changed

* CLI logic improved: `-p` (precision) is no longer required in `--quiet` mode.
* Default starting iteration (`1`) is used in `--quiet` mode if `-i` is not provided.
* The `input()` function no longer checks the `--quiet` flag internally; this is now handled only where necessary (e.g., for modulus input).

### Fixed

* Graceful handling of missing parameters in `--quiet` mode using sensible defaults where possible.

---

## \[v0.2.1] - 06-06-2025

* Added quiet mode (`-q`, `--quiet`)
* Show execution time after factorization

## \[v0.2.0] - 05-06-2025

* Removed default modulus value; now prompted interactively if not provided via `-n` / `--mod`
* Introduced `clap` for structured and user-friendly command-line argument parsing
* Updated README usage examples to reflect removal of default modulus and clarify argument use

