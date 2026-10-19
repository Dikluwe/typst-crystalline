# L0 — stdlib/structural/markup
Hash do Código: f8d25fb5

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/raw-field-presence.toml sha256:93054c60b918734522155fc8d1140982c656478ffccbc1ce55b14911309e1a5c

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/structural/markup.rs`

## Evidence and classification

`01_core/src/compiler/stdlib/structural/markup.rs:28`, `01_core/src/compiler/stdlib/structural/markup.rs:54`, `01_core/src/compiler/stdlib/structural/markup.rs:78` identifies the current ownership boundary. This inspection is not a new bilateral parity measurement or proof of upstream intent. Historical decision evidence retains its original snapshot qualifications in `lab/results/construction/diagnostics/sanitization-parser-stdlib-history.md`.

## Contract and ownership

Own strong/emph/raw/link construction and validation, not show matching or layout. strong/emph wrap Content|Str with Bold(true)/Italic(true). raw accepts only Str, optional lang and block, and emits Content::Raw; never coerce Content to raw text. link requires a nonempty Str URL and optional Content body, defaulting the body to the URL text, with no named arguments. Preserve the existing exact diagnostics, lang/block casts/defaults and pure execution without Engine/I/O.

The original requirement that all these natives support no-argument show-selector sentinels remains a contract obligation. Code discrepancy: strong/emph/raw produce empty content on no arguments, but native_link rejects the missing URL at `01_core/src/compiler/stdlib/structural/markup.rs:164`. Do not silently weaken that requirement or change the code during sanitation; function-valued show selection is not proof that no-argument native calls satisfy it.

## Acceptance

Preserve the specified syntax, semantics, morphology, errors/hints/spans, defaults and public identities, with no productive-code change. Preserve pinned shared claims and the exclusive owner boundary under ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130. Historical evidence does not establish parity against a newer snapshot.

## Ratified Raw lexical presence

### Evidence before decision

content-serialization-2026-10-14/{before,raw-controls}/results.json.gz records
HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree, UTC, sources/binaries
and historical pin `9f2b6e8715237cb086899a42873660fe744622e8`.
RawElem raw.rs:21–25 loses omission; native_raw markup.rs:78–136 collapses absent/
false/none. Fields, equality and encoders distinguish those states at the pin.
This is semantics/morphology, not Rust mechanics.

### Exclusive responsibility and acceptance

native_raw marks block when lexical Bool and lang whenever the argument is
present, including none. Legacy block:none remains omitted; preserve legacy
casts/diagnostics/arity. Approved manifest:
`b799576490d06bb6fde27c32c78572b63e1279b599d4684629fe4484bc4e0a45`,
receipt `lab/history/construction/records/00_nucleo/materialization/raw-presenca-2026-10-14/approval-receipt.json`.
This completes only Raw's formerly incomplete presence contract and supersedes
no-mask clauses. Preserve other contracts; add no phase/default/render behavior.
Validate RED→GREEN tests, suite, build and lineage.
