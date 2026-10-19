# L0 — stdlib/calc/power
Hash do Código: 755b6ff1

## Evidence before specification

L1 exclusive consumer: `01_core/src/compiler/stdlib/calc/power.rs`; former calc.rs:256-359,1443-1483. The clean HEAD `d9e851a850c66905bdb0c3ea4b155a6f36881106` and `lab/results/construction/diagnostics/atomizacao-calc-2026-09-30-before.json.gz` preserve hashes/time/state before extracting algorithms by behavior. Static module composition and shared numeric boundary criteria remain in `00_nucleo/prompts/core/compiler/stdlib/calc.md`; this owner exclusively implements its declared consumer, without transferring composition ownership. Extraction changes no public signature/module function/phase/result/error policy and introduces no I/O or mutable globals. Preserve L1/calc tests, build and crystalline-lint with correct 1:1 lineage; no performance gain is inferred.

Historical probes against `9f2b6e8715237cb086899a42873660fe744622e8` retain their original snapshot limits and do not certify current `bd5ebec285505889e8ff1d4f68804ab147a699f3` without remeasurement. Language semantics, syntax, morphology and observable diagnostics govern acceptance; enum/match/algorithm structure is mechanics. Source inspection is not inferred upstream intent.

`lab/results/construction/diagnostics/tekt-v16-power-2026-10-17/before.json.gz` preserves three open zero/pair fallbacks at power.rs:32-95, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, dirty state/UTC/stat/hashes. before-measurement.json.gz has 20 bilateral cases with identified binary SHAs: invalid base/large exponent currently reports exponent first; Decimal exponent lacks conversion hint. Pinned calc.rs:172-201 validates DecNum base then Num exponent before numerical guards. No upstream intent inferred.

## Exclusive contract

Implement pow/sqrt/root, preserving 0^0 rejection/overflow, Int/Float/Decimal kinds, finite guards, integer index, radicand-before-index and negative real root only for odd index. Validate base before exponent and both before power guards. Base accepts only Int/Float/Decimal, exponent Int/Float; Decimal exponent is rejected with exact hint `if loss of precision is acceptable, explicitly cast the decimal to a float with `float(value)``. Base error names Decimal too. Preserve other local text/spans/arity/named arguments/results/guard_float/sqrt/root; this does not claim complete message/span/infinity parity.

Project Value to private three-variant numeric base enum and two-variant exponent enum, explicitly grouping every nonnumeric Value in one error-policy arm; Decimal exponent rejects before guards. Zero predicates/numeric dispatch are exhaustive over projections, no catch-all/unreachable/V16 suppression. Intentional V19 error-policy group stays grouped. Future Value must force boundary classification/E0004; acceptance without it refutes maintenance inference. No public API/trait/phase/mode change, ADR-0127 continuous flow.

## Acceptance

Functional RED→GREEN for invalid base before large/non-normal exponent, Decimal exponent hint, kind preservation,0^0/overflow/Decimal×Float error and complete suite. Future-enum isolated-target probe rejects Value conversion. Differential removes only three covered V16 and permits V19 only prescribed group; restricted owner-consumer reseal, build/fmt/strict Clippy/Rustdoc/direct-reverse lineage/diff/all tests under 12GiB/swap 0/jobs 1.
