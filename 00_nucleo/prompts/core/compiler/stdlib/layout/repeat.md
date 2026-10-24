# L0 — stdlib/layout/repeat — repeat
Hash do Código: e7791f95

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/layout/repeat.rs`

## Evidence before decision

The uncommitted working tree captured in `lab/results/construction/diagnostics/atomizacao-stdlib-layout-2026-09-30-before.json.gz` contains this family's logic in `01_core/src/compiler/stdlib/layout.rs`; the complete L1 suite passed with 6401 tests and 2 ignored before extraction. This is structural measurement, not a benchmark or new bilateral vanilla parity evidence. Historical decisions, measurements and pin qualifications remain in `00_nucleo/prompts/core/compiler/stdlib/layout.md`.

## Language contract

repeat preserves count, gap, justify, limit and body arity, including the same diagnostics.

Extraction preserves signatures, function identity, syntax, morphology, errors and validation precedence. The hub and registry continue to forward calls statically without duplicating algorithms. Preserve public contracts, default behavior and pipeline phase.

## Acceptance

The L1 suite and `crystalline-lint .` must remain regression-free. Parity is at the language level under ADR-0107/0108; mechanics may diverge. Preserve exclusive ownership under ADR-0129/0130.

## Measured negative-gap correction — 2026-10-22

`lab/results/nested-fractions-repeat-2026-10-21/repeat-before/measurement.json.gz`
(SHA-256 `e6d3c085c28062ad22fdb3b1575108d3a778842a50bf0e5c7fa8381b445e88e0`)
records source/font/binary/code/L0 hashes, dirty HEAD
`cf21f47a3b63f0e9fc5308925e1d0b67925e7227`, UTC
`2026-10-22T01:15:47.337768+00:00`–`2026-10-22T01:16:29.201630+00:00`,
unchanged inputs and full commands/stderr. Product `94208ba4526d7bda97d4519aa8eb12212bd13dc1f8932552f235940e73238485`
rejects the immutable negative-gap source with the historical P156J diagnostic;
ratified bd5ebec285505889e8ff1d4f68804ab147a699f3 / oracle SHA-256
`9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`
accepts it and renders twelve X copies. Pinned library layout/repeat.rs declares
a Length gap without a nonnegative caster; layout/repeat.rs:32–65 resolves that
signed gap in the actual frame arithmetic. The separate count-zero witness
retains accepted -100pt gap with no X paint.

After a meaningful native-constructor RED test, remove only the explicit
negative-absolute/em gap rejection. Preserve the exact signed Length payload,
body Content/Str acceptance, missing/wrong body diagnostics, gap type caster,
justify Bool/default true, named-argument policy and validation order elsewhere.
Do not introduce a count argument or change any public signature/entity/default.
Render/count/finite-width behavior belongs to
`00_nucleo/prompts/core/compiler/layout/repeat.md`; this parser must not duplicate
it. This bounded parity correction supersedes the historical preserve-all-
diagnostics instruction only for the independently refuted negative-gap error.
ADR-0127 continuous flow applies: L0 first, observed RED, exact acceptance and
full hash/lineage/test gates, with no new phase or suppression.
