# L0 — compiler/eval/repr/value
Hash do Código: 16d8f976

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/introspection/content-snapshot.toml sha256:d9b2ab50748dec1246d8a55c0b702fa2ee5e23dd737bedf3689e6c9f0297ac29

## Ownership and evidence

Layer L1. Exclusive productive consumer: `01_core/src/compiler/eval/repr/value.rs`. At HEAD `6b147bd213e07e1dc896a8e798b84bf4d8dbbdcb` with a clean tree, `01_core/src/compiler/eval/repr.rs:28-201 e 1516-1573` contains the separated behavior. SHA-256, time and state are recorded in `lab/results/construction/diagnostics/atomizacao-repr-2026-09-30-before.json.gz`. The purpose is to make algorithms discoverable by behavior, with no presumed performance gains.

## Contract

Format each Value variant as deterministic public morphology. Keep arrays limited to 40 items only in repr, empty dictionary (:), causal Args, nonfinite floats, DateTime, closures and LocatedContent snapshots without reconstructing fields from styles.

The detailed contract and historical parity evidence remain in `00_nucleo/prompts/core/compiler/eval/repr.md`; extraction preserves output, public signatures and phase. No I/O or mutable global state is permitted.

## Acceptance

Preserve language morphology tests, build, and pass `crystalline-lint .` with a valid owner and hash.

## Long dictionary formatting

transformacoes-2026-10-14/after-second/results.json.gz records
rotate-fields-explicit and skew-fields with equal values but Tekt dictionary
repr on one line, while the pin uses vertical formatting above 50 columns.
repr/value.rs:52 joins directly although the format owner already provides the
measured pretty_array_like rule. Reuse that formatter for nonempty Dict,
preserving order and (:). repr morphology is observable; helper/allocation are
mechanics. Do not change key quoting. Validate long/nested regressions and short
controls.

## Decimal Display use

### Evidence before decision

first-validation/validation-clippy.log under
`lab/results/construction/diagnostics/clippy-enums-2026-10-16` records
to_string_in_format_args in repr/value.rs's Value::Decimal arm after the approved
Display implementation. The measured text is
`format!("decimal(\"{}\")", d.to_string())`; the new contract's default Display
delegates to the same InnerDecimal. first-validation/validation-details.json
retains hashes, HEAD, working tree, UTC and diffstat. This is internal string
construction, not new parity measurement.

### Contract and acceptance

Pass d directly to format!, preserving decimal("...") and decimal text/scale/
zeros/sign. Change no other variant, quoting rule, signature or phase. This is
ADR-0127 mechanical continuous flow without suppression or a new contract.
Preserve all repr tests and run suite/Clippy/build/rustdoc/lineage under 12 GiB.
The previous diagnostic is lint RED; GREEN requires no new warning and preserved
existing output.
