# L0 — stdlib/calc/round
Hash do Código: d8cbd364

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/decimal-rounding.toml sha256:dcf8dc8e474613bab33bbf5dff39124b8711f2facb85b731e5d63363a347e6d3

## Evidence before specification

L1 exclusive consumer: `01_core/src/compiler/stdlib/calc/round.rs`; former calc.rs:360-516,1002-1051. The clean HEAD `d9e851a850c66905bdb0c3ea4b155a6f36881106` and `lab/results/construction/diagnostics/atomizacao-calc-2026-09-30-before.json.gz` preserve hashes/time/state before extracting algorithms by behavior. Static module composition and shared numeric boundary criteria remain in `00_nucleo/prompts/core/compiler/stdlib/calc.md`; this owner exclusively implements its declared consumer, without transferring composition ownership. Extraction changes no public signature/module function/phase/result/error policy and introduces no I/O or mutable globals. Preserve L1/calc tests, build and crystalline-lint with correct 1:1 lineage; no performance gain is inferred.

Historical probes against `9f2b6e8715237cb086899a42873660fe744622e8` retain their original snapshot limits and do not certify current `bd5ebec285505889e8ff1d4f68804ab147a699f3` without remeasurement. Language semantics, syntax, morphology and observable diagnostics govern acceptance; enum/match/algorithm structure is mechanics. Source inspection is not inferred upstream intent.

`lab/results/construction/diagnostics/arredondamento-ci-2026-10-13/` snapshots preserve HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, dirty tree/UTC/stat/binaries before round.rs:67-69 modular i64→i32 precision truncation and :117-119 negative-precision multiplication correction. Pinned calc.rs:923-949/typst-utils round.rs:26-60 saturate precision and avoid intermediate overflow. probe-before-refined.json in evidence.json.gz measures type/value:24 oracle pass,15 pre-repair Tekt failures. First min-i64 literal trial was replaced by an integer expression valid in both parsers; large-float textual formatting is not equality evidence. `lab/results/construction/diagnostics/representacao-medidas-ci-2026-10-13/before.json.gz` measures already-protected finite rules at 104-124 before shared-nucleus extraction; repr and calc share finite invariants, not local propagation/error policy.

## Exclusive contract

Implement floor/ceil/round/trunc/fract with specified Int/Float/Decimal kinds. Integer negative precision rounds away from zero with checked overflow. digits accepts i64, saturates to i16 for Int/Float and i32 for Decimal, never modular truncates. Finite Float rounding follows pinned decimal-rounding claims. Keep floor/ceil/trunc/fract prior policy.

Preserve categorical nonfinite policy: precision 0 uses round, precision>=15 returns operand, precision<-308 uses value*0; remaining range uses guard_float including true overflow. Large-value preservation requires finite operand; do not broaden IEEE propagation or claim complete diagnostic parity. Shared nucleus introduces no changed guard_float/type conversion/Rust body; calc errors and repr propagation remain separately owned. Continuous internal finite-parity correction changes no signature/default/phase.

## Acceptance

Native calc.round argument-path RED→GREEN for large values/extreme digits in all three kinds, ties/signed zero/nonfinite policy,24 type/value probes passing both binaries and unchanged Tekt error diagnostics. Do not test a copied implementation formula against itself. Preserve L1/calc suite, build/lint/lineage/pins without performance claim.
