# L0 — stdlib/loading/yaml_encode
Hash do Código: ce556da4

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/introspection/content-snapshot.toml sha256:d9b2ab50748dec1246d8a55c0b702fa2ee5e23dd737bedf3689e6c9f0297ac29

## Ownership and evidence

Layer L1. Exclusive productive consumer: `01_core/src/compiler/stdlib/loading/yaml_encode.rs`. At HEAD `4ac3e2082e6e708dd815ab01b47ac8ed281a14fa` with a clean tree, encoders were in `01_core/src/compiler/stdlib/loading.rs:40-646`; complete state in `lab/results/construction/diagnostics/atomizacao-loading-2026-09-30-before.json.gz`. Separation is internal and is not a performance measurement.

## Contract

Emit YAML from Value, including sequences, maps, projected Content, special scalars, quoting and indentation; preserve newline and error spans. Reuse common text projection without reads or callbacks.

The detailed specification and historical evidence are in `00_nucleo/prompts/core/compiler/stdlib/loading.md`. A present snapshot is authoritative under the pinned nucleus; do not reconstruct from the current style.

## Acceptance

Preserve output and existing diagnostics, build, and pass the L1 suite and `crystalline-lint .`.


## Closed decision coverage

### Measurement before decision

At HEAD `91ea6ee91105b97b1aedf172e34780dddde14c94`, the source/owner hashes
and exact inspection state are in
`lab/results/v16-corrections-2026-10-19/batch-f/agents/encoding/read-only-audit.json`.
The baseline binary build and UTC-stamped bilateral commands/output are retained
in `lab/results/v16-corrections-2026-10-19/batch-f/agents/encoding/baseline-enabled-probes.json`;
the separate HTML textual projection receipt is `baseline-html-projection.json`
in that directory. The initial feature-disabled HTML run is setup evidence only.
The oracle is pinned to commit `bd5ebec285505889e8ff1d4f68804ab147a699f3`,
tree `cac836c07e4dec2a89219370428292393a3216b5`, binary SHA-256
`9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`.
These observations do not infer upstream intent or certify unrelated parity.

`yaml_encode.rs:37–91` has guarded nonempty Array/Dict arms followed by a
nested scalar match; empty containers must reach []/{}. The inner repr fallback
also obscures the actual opaque Value complement. Pinned probes distinguish empty
and nested containers, signed zero, numeric tokens, strings, bytes and projected
content. Multiline sequence indentation differs textually but must be assessed
as decoded YAML, not byte parity; existing output remains preserved here.

### Decision and acceptance

Use one exhaustive Value decision at write_yaml_value, making empty and nonempty
Array/Dict handling explicit within their own arms. Give None/Bool/Int/Float their
existing scalar tokens and newline, including negative zero, NaN and both
infinities. Str/Symbol retain string writing; Content/LocatedContent retain shared
projection, snapshots and errors. Name every opaque variant for canonical repr
string emission. Remove the causally adjacent open inner repr fallback rather
than moving the same hole. Preserve indentation/quoting/Unicode/trailing newline
and errors, with no callback/I/O. Fixed-output and decoded source probes cover
container boundaries, scalar classes, strings/bytes and snapshots. A future Value
variant must fail this exact writer match. No general YAML formatting parity or
new content schema is claimed; correction is ADR-0127 internal continuous flow.
