# L0 — stdlib/structural/document
Hash do Código: 093b7ea0

**Vanilla ratificado**: `586e1bd43`; `document` é independente, `asset` pertence a `typst-bundle/src/format.rs`.

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/structural/document.rs`

## Evidence and classification

`01_core/src/compiler/stdlib/structural/document.rs:25`, `01_core/src/compiler/stdlib/structural/document.rs:100`, `01_core/src/compiler/stdlib/structural/document.rs:145` identifies the ownership boundary. Inspection is not new bilateral parity evidence or upstream intent. Historical evidence retains its original provenance in `00_nucleo/diagnosticos/sanitization-parser-stdlib-history.md`.

## Contract and ownership

Own pure direct native_document/native_asset constructors, bundle-path normalization and string-list extraction, not set rules or target compilation. Preserve the existing direct bundle signatures, not the retired metadata-only document()/asset(path,kind:) placeholders.

asset requires two positionals path and data and no kind. Path is a nonempty Str normalized lexically to the virtual bundle root: repeated slashes and "." disappear, ".." removes one component without escaping root. Reject backslash (`backslashes are not allowed in bundle paths`, hint `use forward slashes instead`), escape (`path {path} would escape the bundle root`) and no components (`path must have at least one component`). data accepts UTF-8 encoded Str or byte-exact Bytes. Emit Content::Asset with normalized path/data; no reading/writing or extension-based categorization. Preserve missing path/data, third positional, named path/data/kind and cast diagnostics measured in `00_nucleo/diagnosticos/typst-p1485-bundle-sondas.md`, without claiming a new probe.

document requires positional path:Str and body:Content. Optional format is Func; metadata title is Content, author/keywords Str|Array[Str] normalized as lists, date Datetime. Preserve metadata and format explicitness in the Content carrier. Missing direct path is not silently metadata; set document remains an eval-rule operation for individual targets. Outside Bundle target retain `constructing a document is only supported in the bundle target`, whole-call span and target/set-rule hints. Bundle target inference/explicit format compilation is L3 ownership, not a native I/O operation. Registry feature gating is separately owned and must not publish the retired placeholder. Historical public-contract proposal/gates are archived, not newly ratified.

## Acceptance

Preserve syntax, semantics, morphology, diagnostics/hints/spans, bindings, defaults and identities. No productive code or compatibility change is authorized by this sanitation. Preserve pinned shared claims and exclusive ownership under ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130.
