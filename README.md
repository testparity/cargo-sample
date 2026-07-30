<p align="center">
  <img src="https://testparity.com/brand/testparity-readme-banner.jpg" alt="TestParity" width="100%">
</p>

# Cargo Sample

[![Parity Sample](https://github.com/testparity/cargo-sample/actions/workflows/parity.yml/badge.svg)](https://github.com/testparity/cargo-sample/actions/workflows/parity.yml)

This repository is an end-to-end Parity demo for Cargo. CI runs the real suite, then `parity test` runs each belonging test target in isolation and builds fresh per-test coverage evidence.

## What this sample proves

No coverage result is committed. `cargo-llvm-cov` emits one Cobertura report per isolated test target, and Parity normalizes those reports into `.parity/per-test` before checking them.

| Source file | Matching test | All-test file coverage | Matching-test coverage | Other covering tests | Why it matters |
| --- | --- | ---: | ---: | ---: | --- |
| `src/lib.rs` | `tests/lib_test.rs` | 80% | 40% | 1 | Global coverage hides that half of the covered lines are exercised by another test target. |
| `src/discount.rs` | `tests/discount_test.rs` | 100% | 100% | 0 | The matching test owns this file's coverage. |
| Project total | - | 84.21% | - | - | Global coverage is healthy while one file still needs better direct tests. |

Parity enforces these thresholds in `parity.yaml`:

- `min_coverage_global: 80` keeps the aggregate project gate.
- `minimum-coverage: 70` keeps every source file above the ordinary coverage floor.
- `matched-coverage: 40` requires the belonging test to cover a meaningful portion itself.
- `coverage-attribution` reports `2|1` for `lib.rs`: two covering tests, one of them non-matching.

## Run locally

Install the coverage tool and the public Parity package:

```bash
cargo install cargo-llvm-cov --locked
composer global require "testparity/parity:^1.2"
```

Run the native suite, then generate and check fresh per-test evidence:

```bash
cargo test --locked
parity test --config=parity.yaml --format=json --show-tests
```

Expected highlights from the JSON output:

- `lib.rs`: `minimum-coverage = 80%`, `matched-coverage = 40%`, `coverage-attribution = 2|1`.
- `discount.rs`: `minimum-coverage = 100%`, `matched-coverage = 100%`, `coverage-attribution = 1|0`.
- `global_coverage = 84.21` and `passed = true`.

## CI

GitHub Actions runs the full Cargo suite, rejects the old fixture path, clears generated evidence, and then runs:

```bash
parity test --config=parity.yaml --format=json --show-tests
```

The workflow uploads `.parity/per-test` and `parity-result.json` so every green run has inspectable proof. No private token is required.
