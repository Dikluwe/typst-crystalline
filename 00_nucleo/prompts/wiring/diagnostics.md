# L0 — wiring/diagnostics
Hash do Código: 390788e9

**Layer:** L4
**Exclusive consumer:** `04_wiring/src/diagnostics.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/wiring/cli-observables.toml sha256:e9d6623a18ffc35f36051b280d96a75a0e4c1bd1768361a3a50c98587c568aef
```

## Current evidence

- `04_wiring/src/diagnostics.rs:19` — `prepare`.
- `04_wiring/src/diagnostics.rs:28` — `prepare_with_primary`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 04_wiring/src/diagnostics.rs`.

## Owned contract

- Own private prepare and prepare_with_primary (superseding drain_to_stderr and drain_to_stderr_with_primary under the approved preflight contract). Resolve diagnostic and trace FileId sources once, deduplicate, and use the existing main fallback for absent identity.
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


## Approved diagnostic resource propagation — approved 2026-10-22

Consume path_of's FileResult and distinguish Ok(None) from Err. Only true absent
identity uses the existing input fallback. Prepare sources/paths through private
fallible helpers and return existing structured errors to compile/eval/query;
add no public entity or formatter contract. Preserve every original diagnostic,
hint, trace and collection order. If metadata is unavailable, retain and emit
the original diagnostic without fabricating its source/path, followed by an
explicit operational error; never turn poison into normal fallback/success or
drop the original error. Let callers preflight before success publication while
keeping actual status/diagnostic emission order. No second compilation, custom
source parser, source identity rewrite or public L2 formatter change.


Approval of this exact metadata correction is retained in `lab/results/world-metadata-errors-2026-10-22/approval.json`, review SHA-256 `31d632a54d40ca7df4139081c5297af774970a93b08cd605986b53e75e0ccc62`. The approved clauses supersede prior infallible signature/return preservation only for the two public APIs and the described private composition interfaces.

Private preparation may return (formatted text, Vec<FileError>) and accept a private source-resolution closure for fault controls. Missing/failed metadata must not delete original diagnostics/hints/traces or fabricate source names. Preserve source lookup memoization behavior and canonical path formatting.

## Trace acquisition failure acceptance

The earlier `trace-preservation-gate` failure is retained as causal RED evidence.
The additive L2 function was approved on 2026-10-22 and the unchanged
original_call preservation assertion now passes in `trace-format-wiring`.
Full candidate validation remains required; no waiver applies. Registered-source
and metadata acquisition/publication guarantees remain required.

A registered path whose Source fails to load remains an actual FileError, including NotFound; only a truly absent identity may follow the existing missing-source omission. Preserve the acquisition error and do not disguise it as unknown metadata.


## Approved use of unresolved-trace formatting on metadata failure — approved 2026-10-22

The separately approved metadata preparation already preserves original typed
diagnostics and returns actual FileError values. If source/path resolution fails
for a diagnostic, format that original diagnostic through the additive L2
format_diagnostic_with_unresolved_traces function. Healthy preparation, true
unknown identity, absent sources without acquisition error and normal CLI use
keep the existing formatter. Preserve exact FileError variants/messages and all
original messages/hints/traces; no L4 Tracepoint rendering, synthetic Source/path,
second evaluation, discarded diagnostic or hidden operational error is allowed.
This narrowly supersedes the earlier no-public-L2-formatter-extension restriction
only for this exact additive call. CLI/watch return and publication gates remain
as already approved. Keep the currently failing preservation assertion; it must
turn GREEN without weakening the old formatter's missing-source omission tests.

Approval receipt: `lab/results/world-metadata-errors-2026-10-22/trace-format-approval.json`; review SHA-256 `c7507b43112d46931bdafb95c26bd83f4fdc605e543911a163ca112c0c95cfb8`.


## Checked stderr composition — authorized 2026-10-22

### Measurement before classification

`lab/results/stderr-watch-errors-2026-10-22/agents/measurement/baseline.json`
binds the unchanged candidate-4 source archive, HEAD/uncommitted state and UTC.
`agents/measurement/before/receipt.json` under that result root (SHA-256
`e49aa4458f4cb413995af35f8297ecf4fcafd20141cbdab41067fc7ed5b3895c`) reproduces ordinary errors/warnings and watch
status panics with a failing stderr. `late-before/receipt.json` (SHA-256
`07ddcb85110604877f56d369dec0d2ac3c7d96ba9ac95f27d9e4e49469f3435c`) retains the closed-pipe later-cycle control.
Actual stderr failure and exception termination are observed process behavior;
healthy diagnostic text/order and document language behavior remain acceptance.
This does not infer upstream intent from a reproduced oracle panic.

### Owner contract and acceptance

Own private write_stderr(text: &str) -> std::io::Result<()> and its writer-injected helper. Empty text does not acquire/write/flush a sink. Nonempty text uses write_all then flush, preserving exact bytes, partial-write/interrupted handling and the original write/flush io::Error. Never replace failed output with success, retry a broken stderr with an error message, or move rendering/World resource acquisition here. Callers retain their exact diagnostic text/order and propagate the actual error to their private process boundary. Add writer controls for write, short/interrupted, flush and empty input; retain all source/trace tests. No L2 formatter or public signature changes.

The explicit user instruction and ADR-0127 classification are retained in
`lab/results/stderr-watch-errors-2026-10-22/authorization-and-classification.json`.
Only the described private exceptional-I/O assumptions are superseded.
