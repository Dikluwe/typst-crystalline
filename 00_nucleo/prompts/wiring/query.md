# L0 — wiring/query
Hash do Código: 4a2e0651

**Layer:** L4
**Exclusive consumer:** `04_wiring/src/query.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/network/custom-ca-cert.toml sha256:4b9f69ff41790a55a5b2d6a7b9982da858092e4bb2f0c503d6332a14a510e3ec
- 00_nucleo/prompts/_nuclei/wiring/cli-observables.toml sha256:e9d6623a18ffc35f36051b280d96a75a0e4c1bd1768361a3a50c98587c568aef
```

## Current evidence

- `04_wiring/src/query.rs:16` — `run_query`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 04_wiring/src/query.rs`.

## Owned contract

- Own run_query composition. Emit the specific deprecation warning after successful serialization or after compilation diagnostics; use the registered stdin Source versus physical root/main/font/CA paths.
- Call L3 query_compiled_elements; drain compilation errors then warnings; retrieval emits its standalone error, and delegate field/one/pretty/format output to L2 serialize_query_with_format. Selectors/cardinality are not reimplemented in L4.
- Success exits 0; semantic/serialization errors exit 1; I/O/argument errors exit 2, except measured missing physical input exits 1. Preserve diagnostic origins through the dedicated drain owner.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.

## Query ordering, acquisition and compiled retrieval

### Evidence and approvals

`lab/results/construction/diagnostics/plano-debitos-2026-10-06/before.json.gz`
measured error → document warnings → deprecation and the specific replacement hint
against the historical pin. Upstream query.rs:43,51–64 and compile.rs:779 order
these explicitly. Human `confirmo` on 2026-10-06 approved replacing ADR-0045 §8's
prior warnings→errors order through its amendment; candidates/hashes are retained
in `lab/history/construction/records/00_nucleo/materialization/warnings-cli-aprovacao-2026-10-06.json`.

`query-residuos-2026-10-06/before.json.gz` retains HEAD/tree/UTC, pin/binary hashes,
sources/commands/stdout/stderr/status. Missing-file controls (relative, ./relative,
absolute) show the pinned oracle exits 1, empty stdout and one line:
`error: input file not found (searched at INPUT)`. Tekt exited 2 using MainNotFound
Display. `infra/world.rs:150–155` already distinguishes typed MainNotFound from Io.

The same diagnostic's query-stdin-callback succeeded where vanilla reported error
and trace; query-context-success lost generated metadata; nested-error disappeared;
query-selector-quotes failed at the old selector limitation before a document
error; where-level was rejected rather than returning [2]. Document-error-invalid-
selector establishes compilation precedence; valid-document-invalid/text-selector
establishes retrieval failure without deprecation. Contextual eval reference and
where controls already match through the current paged compilation. Upstream
query.rs:34–66 compiles before retrieval; :86–105 evaluates selector in default
Scope then converts to LocatableSelector. Locally query_helpers.rs:488–522 consulted
only Module, while pipeline.rs:182–214 already compiled with the final introspector.

Human `confirmo` on 2026-10-06 approved the new compiled-query API/path, receipt
`query-compilada-aprovacao-2026-10-06.json`. Historical candidates/manifests remain
receipts; this owning contract is current. These measurements do not certify the
new upstream pin or require copying upstream execution mechanics.

### Current owner contract

Use `query_deprecation_hint` owned by `00_nucleo/prompts/shell/cli.md`.
Acquisition failure before compilation emits no deprecation. Match typed
MainNotFound in query composition, without string inference or another stat:
use the exact missing-file envelope above, exact input.display(), exit 1 and no
deprecation. Other errors, World and other commands retain their contracts. This
internal parity correction uses RED→GREEN under continuous ADR-0127.

For input `-`, compose for_stdin with acquired text and absolute cwd root, then
obtain `world.source(main())` rather than fabricating a Source with an existing
id. Stdin remains the causal origin and import/read base, outside physical
dependencies. Existing APIs suffice; never execute the document again for this.

Compose `query_compiled_elements` instead of query_elements: compile the complete
paged document before evaluating its selector, without export. Program execution
remains once under existing realization/context owners. Reuse the full paged
pipeline/final introspector; no isolated expansion, pre-context index, parallel
stabilization cycle or PDF extraction. Distinguish compilation/retrieval with an
explicit enum, never messages, detached spans or substring search. No selector/
realization algorithm, new flag or target belongs in wiring.

Compilation failure drains errors, then warnings, then deprecation and exits 1.
Retrieval failure emits one line `error: MENSAGEM`, exits 1, and emits no warnings
or deprecation (the measured upstream early return). Success delegates unchanged
field/one/pretty/format serialization and publication to L2, then drains warnings
and deprecation. Cardinality/field/serialization failure returns its existing error
without deprecation. Preserve order inside each diagnostic collection, public
SourceDiagnostic and the existing stdout/status contract. Validate the documented
stdin, metadata, callback, selector/error-precedence and missing-file controls
without loosening expectations.
