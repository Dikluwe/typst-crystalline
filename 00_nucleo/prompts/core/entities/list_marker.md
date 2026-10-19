# Prompt L0 — `entities/list_marker` — `ListMarker`
Hash do Código: 2a887cd0

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/list_marker.rs`

## Evidence before specification

The initial scope described Default/Custom markers. The later Default review
at `01_core/src/entities/list_marker.rs:23` used HEAD
`06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted working tree, and
`lab/results/construction/diagnostics/expressoes-defaults-ci-2026-10-13/before.json.gz`
for source/owners/spans/UTC/status/diffstat. The non-generic type can derive
Default without adding generic bounds or changing the existing value.

## Contract

Retain Default and Custom(EcoString), Debug/Clone/PartialEq/Hash.
Derive Default with #[default] on Default, replacing only the old manual
implementation. Default renders `•` (U+2022); Custom returns its supplied
string by &str, for example `→`, `-`, or `*`. Preserve render and existing
variants/signatures; the later preservation clause explicitly includes Array.

## Pending scope reconciliation

The initial spec deferred level-dependent markers `([•], [–], [·])` and an
arbitrary Content marker. The later amendment mentions preserving Array,
without defining it here. Keep that Array preservation obligation and record
the missing specification; do not delete the variant, invent its behavior,
or claim the earlier level-dependent scope-out is fully closed.

## Acceptance

Default.render() is `•`; Custom("→").render() is `→`; Default::default()
is Default. Default equals Default; Custom("x") differs from Custom("y").
Preserve tests and expectations, fields/variants, product defaults and phase.
Validate all workspace targets, core suite, test inventory, build, Rustdoc,
Clippy and lineage. Default derivation is an internal ADR-0127 change, not a
new parity or performance measurement.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/list_marker.md`. Historical observations are not a new parity certification.
