# ADR-0115 — Scanner benchmark infrastructure

**Status:** ACCEPTED
**Status history:** `EM VIGOR` (Step 441 — benchmark infrastructure created; `cargo bench` works and is reproducible).
**Recorded owner decision:** use `criterion` 0.5 in `lab/benches/` to measure scanner/lexer tokenization time; create a corpus of 5 representative inputs; primary metric is ns/byte; reproducibility criterion is variation < 3% across 3 sequential runs.
**Related ADRs:** ADR-0032 (L1 `unsafe` policy), ADR-0030 (performance belongs to L1), ADR-0014 (scanner inherited from `unscanny`).

---

## Context

DEBT-42 (Step 84.8a) records 7 occurrences of `unsafe { self.string.get_unchecked(start..end) }` in `01_core/src/engine/lexer/scanner.rs`. ADR-0032 establishes that `unsafe` in L1 may remain as a permanent exception only if a reproducible benchmark demonstrates an unacceptable regression from removing it, recorded in a specific ADR with a concrete number.

Before this benchmark could run, the project needed infrastructure: a framework, a location, stress inputs, and reproducibility criteria.

## Decision

1. **Framework:** `criterion` 0.5 — a de facto Rust standard, already available in the local cache, providing robust statistics (outlier detection, confidence intervals) and HTML/CSV reports.
2. **Location:** the `typst-benches` crate lives in `lab/benches/`. This keeps benchmark code out of `01_core/src/` and leaves L1 free of benchmarking dependencies. The repository layout amendment dated 2026-10-18 changes only this location from the original root-level `benches/`; it does not change benchmark behavior, methods, inputs, metrics, or acceptance criteria.
3. **Measured API:** `typst_core::engine::lexer::Lexer::new(src, SyntaxMode::Markup).next()` — the minimal public API that consumes tokens. The `Lexer` struct became `pub` (visibility only; no functional change).
4. **Inputs:** 5 `.typ` files in `lab/benches/corpus/`, covering distinct scenarios:
   - `b1_hello.typ` — micro input (~11 bytes).
   - `b2_text.typ` — running text (~5 KB).
   - `b3_math.typ` — dense math (~3 KB).
   - `b4_code.typ` — dense code (~4 KB).
   - `b5_utf8.typ` — dense multibyte UTF-8 (~2 KB).
5. **Metric:** lexing time per byte (ns/byte), normalized by input size.
6. **Reproducibility:** 3 sequential runs; variation between median and mean below 3%.

## Consequences

- **Positive.** DEBT-42 is unblocked: P442 can measure the actual impact of removing `get_unchecked`, and P443 can decide using measured values.
- **Costs.** +1 workspace crate (`typst-benches`); +1 dev-dependency (`criterion`); additional compilation time for `cargo bench`.
- **Risks.** Local benchmarks are sensitive to thermal throttling and system load; the criterion of 3 runs with variation below 3% mitigates but does not eliminate noise.

## Alternatives considered

- **`iai-callgrind`**: rejected — depends on Valgrind/Callgrind, less portable and heavier for the goal of measuring percentage regression.
- **Benchmarks in `01_core/benches/`**: rejected — would add benchmark dependencies to the L1 crate and require exposing more internal API.
- **Manual benchmark with `std::time::Instant`**: rejected — `criterion` provides better statistics and reports without significant cost.
