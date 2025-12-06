# 🎄 Advent of Code in Rust

![Language](https://img.shields.io/badge/language-Rust-orange.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)

My journey through the [Advent of Code](https://adventofcode.com/) challenges, implemented in **Rust**. 

The goal of this repository is not just to solve the puzzles, but to solve them **idiomatically** and **efficiently**. I focus on:
- **Zero-cost abstractions:** Using iterators and generics effectively.
- **Memory Safety:** Avoiding unnecessary allocations (`Clone` vs `Copy`).

## Project Structure

This repository is organized as a Cargo workspace. Each year is a separate crate containing binaries for that year's days.

```text
.
├── Cargo.toml          # Workspace definition
├── src/
│   └── lib.rs          # Shared utilities across all years
└── 2025/
    ├── Cargo.toml
    ├── inputs/
    │   ├── day01.txt
    │   └── day02.txt
    └── src/
        ├── bin/
        │   ├── day01.rs
        │   └── day02.rs
        └── lib.rs      # Year-specific shared logic
```

## Usage
Ensure you have the latest stable Rust toolchain installed.

## Running a Solution
To run the solution for a specific day (e.g., Day 1 of 2025):

```bash
cargo run --bin day01 -p aoc2025
```

```bash
# Run with release optimizations for accurate timing
cargo run --release --bin day01 -p aoc2025
```

## Testing
Each day includes unit tests for the example inputs provided in the puzzle description.

```bash
cargo test --bin day01 -p aoc2025
```

## Progress Tracker

| Year | Stars | Completion | Remark |
|---|---|---|---|
| 2025 | 2/12 | 16% | In Progress |

## Tooling & Clippy
```bash
cargo clippy -- -D warnings
cargo fmt --check
```

## License
MIT
