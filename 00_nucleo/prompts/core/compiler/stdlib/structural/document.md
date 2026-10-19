# L0 — stdlib/structural/document
Hash do Código: bb17eb4a

**Historical ratified vanilla:** `586e1bd43`, not the current parity target. document is independent; asset belongs to typst-bundle/src/format.rs.

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/structural/document.rs`

## Evidence and classification

`01_core/src/compiler/stdlib/structural/document.rs:25`, `01_core/src/compiler/stdlib/structural/document.rs:100`, `01_core/src/compiler/stdlib/structural/document.rs:145` identifies the ownership boundary. Inspection is not new bilateral parity evidence or upstream intent. Historical evidence retains its original provenance in `lab/results/construction/diagnostics/sanitization-parser-stdlib-history.md`.

## Contract and ownership

Own pure direct native_document/native_asset constructors, bundle-path normalization and string-list extraction, not set rules or target compilation. Preserve the existing direct bundle signatures, not the retired metadata-only document()/asset(path,kind:) placeholders.

asset requires two positionals path and data and no kind. Path is a nonempty Str normalized lexically to the virtual bundle root: repeated slashes and "." disappear, ".." removes one component without escaping root. Reject backslash (`backslashes are not allowed in bundle paths`, hint `use forward slashes instead`), escape (`path {path} would escape the bundle root`) and no components (`path must have at least one component`). data accepts UTF-8 encoded Str or byte-exact Bytes. Emit Content::Asset with normalized path/data; no reading/writing or extension-based categorization. Preserve missing path/data, third positional, named path/data/kind and cast diagnostics measured in `lab/results/construction/diagnostics/typst-p1485-bundle-sondas.md`, without claiming a new probe.

document requires positional path:Str and body:Content. Optional format is Func; metadata title is Content, author/keywords Str|Array[Str] normalized as lists, date Datetime. Preserve metadata and format explicitness in the Content carrier. Missing direct path is not silently metadata; set document remains an eval-rule operation for individual targets. Outside Bundle target retain `constructing a document is only supported in the bundle target`, whole-call span and target/set-rule hints. Bundle target inference/explicit format compilation is L3 ownership, not a native I/O operation. Registry feature gating is separately owned and must not publish the retired placeholder. Historical public-contract proposal/gates are archived, not newly ratified.

## Acceptance

Preserve syntax, semantics, morphology, diagnostics/hints/spans, bindings, defaults and identities. No productive code or compatibility change is authorized by this sanitation. Preserve pinned shared claims and exclusive ownership under ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130.


## Nominal diagnostic type projection

### Evidence before decision

`01_core/src/compiler/stdlib/structural/document.rs:149`
(diagnostic_type_name, emitted-v16) is the selected projection. The current
Value enum at entities/value.rs:59–195 has 39 materialized variants; current
payload/catalog data, not the historical future-variant table, govern the audit.
`lab/results/construction/diagnostics/tekt-v16-value-projections-2026-10-16/before.json.gz`
retains HEAD, uncommitted working tree, UTC, status/diff HEAD --stat, hashes and
original owners/consumers. contexts.json comes from the current AST checked
against pinned SARIF; classification.json distinguishes nine emitted V16 from
eight latent projections. Rust form/exhaustiveness is mechanics; names,
conversions, borrows and language observables retain the existing contract.
This is not new parity or performance evidence.

### Exclusive responsibility

diagnostic_type_name changes only Int→integer and Bool→boolean; Str remains str
through value.type_name(), not string. Name the other 37 variants in the same
delegating body. Preserve native_document/native_asset, bindings/targets, path
normalization, data cast, messages/hints/spans and bundle signatures/defaults.
C7 covers integer/boolean, Str→str and none/auto/float/array/content/bytes names;
explicitly distinguish this owner's policy from format_options.

Alternatives sharing a result share one body. A growing Value enum requires a
new nominal decision; do not duplicate bodies to remove V19. V19 is informational:
it counts written alternatives, not executed tests, reachable combinations or
an N× coverage deficit. Choose positive/negative/boundary controls by behavior;
executing one arm does not prove every alternative. No allow/expect, severity
relaxation or exception may hide V16.

### Acceptance and limits

ADR-0127 continuous flow adds no public signature/trait/field, productive variant,
product default, compatibility break or phase change. L0 first, restricted
bilateral resealing and valid V1/V5/V15/V26. Write local controls and verify before/
after behavior. Structural RED is the absence of E0004 at the 17 selected points
in a core copy with a sentinel added only to copied Value. GREEN requires E0004
at all 17 without stubs and exactly the nine selected V16 removed, preserving
other diagnostics. Validate build/workspace suite, strict Clippy, fmt, Rustdoc
and lineage under 12 GiB, zero swap and one Cargo job. The global gate stays open
while external V16 remain. The parser false positive for explicit None is a
separate investigation and must not be hidden here.
