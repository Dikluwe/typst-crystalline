# L0 — compiler/stdlib/structural/math/casts
Hash do Código: 2b97c2a4

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/stdlib/structural/math/casts.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/symbol-identity.toml sha256:5060f0989415e92d54eab2a82afd84ba11472aef635d15830f9fb6ee3469aea9

## Measurement before decision

The uncommitted working tree captured in `lab/results/construction/diagnostics/atomizacao-stdlib-math-restante-2026-09-30-before.json.gz` places this behavior in `01_core/src/compiler/stdlib/structural/math.rs`. The full core baseline passed 6,401 tests with 2 ignored before movement. This is a structural regression baseline, not a new bilateral vanilla measurement or benchmark. Historical language measurements and pin qualifications remain in the parent L0.

## Language contract

Own shared argument casts used by more than one mathematical constructor: long vanilla type names (`str`→`string`, `int`→`integer`, `bool`→`boolean`); `math_lr_content_value` accepts Content, LocatedContent, String, Symbol and None with the current distinct conversions; `math_lr_size_from_args` validates only `size`, detects duplicate occurrences, and accepts Relative, Ratio or Length while retaining explicit presence; `p1293_math_content` accepts Content, LocatedContent, String and Symbol and preserves MathText morphology. Preserve exact diagnostics, validation order and spans. Other feature owners call these free functions; they do not duplicate the cast table. No layout or I/O.

The parent `math.rs` retains only static registration and shared catalog orchestration. Existing `compiler::stdlib` bridges retain their names and visibility via direct reexports. No public signature, default, pipeline phase or compatibility change is authorized.

## Acceptance

Existing core regressions remain green. The full core suite, `cargo build` and `crystalline-lint .` pass; header ownership is 1:1. No speedup is claimed without separate measurement.

## Approved batch E integration

Measurement: pre-E inputs and bilateral reflection in
`lab/results/symbol-identity-2026-10-23/{baseline.json,before/receipt.json,additional/receipt.json}`.
The owner approved the native Symbol declaration on 2026-10-23.
Preserve the selected raw Symbol as Content::Symbol when casting a content argument; preserve quoted/numeric/manual legacy carriers and argument diagnostics. Matrix comma syntax recognizes the native Symbol comma without altering the payload.
Preserve the existing unit responsibility and all other variants/effects; no public contract beyond the approved Content addition or phase move.


## R4 — measured public mathematical projection / qualified conversion

### Measurement before decision

`lab/results/math-fields-r4-2026-10-23/baseline.json` and `before/receipt.json`
retain UTC, HEAD, complete uncommitted diff stat, source map, archived sources
and binary SHA-256. The oracle is pinned bd5ebec285505889e8ff1d4f68804ab147a699f3
(tree cac836c07e4dec2a89219370428292393a3216b5). The before probe records
fields/has/at and accepted/rejected constructor values rather than inferring
intent from implementation. Upstream math/matrix.rs:45–67,238–272 and
math/mod.rs:227–234 declare the already existing language fields. Local
content_methods.rs:151–158,378–380 omits these projections; frac.rs:43–53
and root.rs:27–38 reject supported operands. These are language semantics
and morphology, distinct from byte or Rust algorithm parity.

### Exclusive correction and gate

Keep math_lr_content_value as the sole general qualified constructor content cast. Separately, p1293_math_content accepts None as Empty for attach base, while its existing String MathText and Symbol morphology stay unchanged. This does not alter optional attach-slot conversion.

This R4 clause supersedes only conflicting historical operand/field clauses
above. Internal parity correction follows continuous ADR-0127: L0 first,
reseal, actual functional RED before implementation, GREEN and full validation.
No new public Rust field/signature/trait method, default mode or phase change.
Preserve other semantics and all received changes. Record source-bound builds,
full workspace tests/build/fmt/Clippy/Rustdoc, lineage and full lint zero; replay
A–H and R1–R3 without reclassifying their existing limitations as parity.
