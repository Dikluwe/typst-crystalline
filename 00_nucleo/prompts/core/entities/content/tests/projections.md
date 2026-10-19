# L0 — entities/content/tests/projections
Hash do Código: 1e8966d8

**Layer:** L1 test-only
**Exclusive consumer:** `01_core/src/entities/content/tests/projections.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Contract

Own regression coverage for semantic emptiness, plain-text projection, and named-field observations across content variants. Tests observe public or crate-visible
behavior and must not prescribe private implementation structure, dynamic
dispatch, or reverse layer dependencies.

## Acceptance

Cases retain independent assertions and fixtures, execute as
part of the `Content` test suite, and introduce no productive dependency.


## Ratified complete text size fixtures

### Evidence before specification

`lab/results/construction/diagnostics/text-argumentos-2026-10-17/before.json.gz`,
before-measurement.json.gz, morph-before-measurement.json.gz and audit.json
retain HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree, UTC,
status/diffstat, sources/L0/binaries/argv and historical pin
`9f2b6e8715237cb086899a42873660fe744622e8`. named-em measured 0pt versus 24pt,
affine 3pt versus 27pt, nested over 18pt 0pt versus 36pt. Six Size/Color/body
permutations were accepted by the pin and rejected by Tekt; positional set
18pt was ignored (12 versus 18pt). Pinned text/mod.rs:290,303,924–930,1136–1142
and args.rs:178–247 establish typed search, named preference, remaining body
and composition. style_chain.rs:513–517 and constructor.rs:123 dropped em;
collapse at style_chain.rs:394–395,419–422 used top-wins where size requires fold.
These are language size/scope observables, not required Rust representation.
Do not infer historical intent. Restoration, Raw, heading, math, capture/show,
duplicates/spans, zero/negative sizes or divergent metrics can refute a fix.
Equality probes refuted declared-value comparison: the pin equates
text(size:2em)[A] and text(size:24pt)[A]; StyledElem::eq at content/mod.rs:779–782
compares child. That equality defect requires its own correction; a complete
size type does not close that axis.

### Approval and exclusive responsibility

The human ratified complete text size and its two public contracts on
2026-10-17, manifesto SHA-256
`771f5fd3d36545eb4b4fb7f0c0f2c91061470d1f010fc9a887b001a636d8feba`.
Implementation and S1–S16 tests are approved under 12 GiB/no swap, without
release/merge/push; reseal only after implementation and a valid graph.
The separate RG1 obligation remains independent.

This owner changes only test fixtures/adapters: Style::Size(Pt) becomes
Size(absolute Length), and StyleDelta.size expectations become Option<Length>.
Preserve language observables and existing functional contracts; do not replace
correct geometry with snapshots of the new representation.
