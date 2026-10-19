# L0 — wiring/diagnostics
Hash do Código: 52a3084e

**Layer:** L4
**Exclusive consumer:** `04_wiring/src/diagnostics.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/wiring/cli-observables.toml sha256:e9d6623a18ffc35f36051b280d96a75a0e4c1bd1768361a3a50c98587c568aef
```

## Current evidence

- `04_wiring/src/diagnostics.rs:15` — `drain_to_stderr`.
- `04_wiring/src/diagnostics.rs:24` — `drain_to_stderr_with_primary`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 04_wiring/src/diagnostics.rs`.

## Owned contract

- Own drain_to_stderr and drain_to_stderr_with_primary. Resolve diagnostic and trace FileId sources once, deduplicate, and use the existing main fallback for absent identity.
- A supplied transient primary overrides World for its own id. Display relative-to-cwd paths where appropriate with existing input fallback, then invoke the L2 DiagnosticSource formatter and emit to stderr.
- Preserve spans, traces, order and formatting selection without new coloring, filtering, reordering or a World dependency in L2. Do not centralize stdin in this helper or merge distinct eval/query provenance.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.

## Relative diagnostic paths

### Evidence

`lab/results/construction/diagnostics/diagnostic-paths-2026-10-06/before.json`
retains HEAD, working tree, UTC, sources, commands and binaries against historical
pin `9f2b6e871`. With cwd work/a and sibling project/src/main.typ, Tekt displayed
an absolute path while vanilla displayed ../../project/src/main.typ.
`diagnostics.rs:62–66` used strip_prefix, which covers only descendants. Pinned
`typst-cli/src/world.rs:157–168` uses pathdiff::diff_paths; its lock fixes 0.2.3,
checksum `df94ce210e5bc13cb6651479fa48d14f601d9858cfe0467f43ae157023b938d3`.
These historical measurements do not certify the current parity pin.

### Contract and acceptance

Compose pure `pathdiff =0.2.3` to calculate names relative to the available cwd,
including ancestors/siblings; do not implement another path algorithm in wiring.
Retain existing path/input fallback when cwd or the difference is unavailable,
transient sources, deduplication, spans, traces, order, color and stderr. No extra
filesystem access or canonicalization for presentation, new API/flag/phase or
file-resolution/sandbox change is authorized. Do not infer package paths or lost
constant-Content origins/spans here. Validate main/imported sources, query,
contextual eval, absolute/relative inputs and descendants. This internal parity
correction follows continuous ADR-0127 flow.
