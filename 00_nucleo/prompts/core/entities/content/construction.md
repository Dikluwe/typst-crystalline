# L0 — entities/content/construction
Hash do Código: 68508250

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/raw-field-presence.toml sha256:93054c60b918734522155fc8d1140982c656478ffccbc1ce55b14911309e1a5c

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/content/construction.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Contract

Own the residual public `Content` constructors other than sequence normalization. The math, layout, table and reference families are owned respectively by `00_nucleo/prompts/core/entities/content/construction/math.md`, `00_nucleo/prompts/core/entities/content/construction/layout.md`, `00_nucleo/prompts/core/entities/content/construction/tables.md` and `00_nucleo/prompts/core/entities/content/construction/references.md`. They must preserve every signature, default, omission
state, and payload. Construction remains pure L1 and uses only closed native
variants plus the existing explicit `Dynamic` extension boundary. No registry,
property map, I/O, or rendering is allowed.

## Acceptance

All existing constructor, representation, morphology, and parity tests pass;
callers compile unchanged; the module includes local tests.

## Ratified Raw lexical presence

### Evidence before specification

Historical content-serialization-2026-10-14/{before,raw-controls}/results.json.gz
records HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree, UTC,
sources/binaries and pin `9f2b6e8715237cb086899a42873660fe744622e8`.
RawElem raw.rs:21–25 lost omission; native_raw markup.rs:78–136 collapsed
absence/false/none. Fields, equality, and encoders distinguished them at that
pin. These are semantics/morphology, not Rust mechanics or a fresh current-pin
measurement.

### Contract and acceptance

Content::raw retains its signature, always marks block present, and marks lang
only for Some. Test getters and maps without changing rendering.
Approved manifesto SHA-256
`b799576490d06bb6fde27c32c78572b63e1279b599d4684629fe4484bc4e0a45`;
receipt:
`lab/history/construction/records/00_nucleo/materialization/raw-presenca-2026-10-14/approval-receipt.json`.
This completes only Raw's previously incomplete presence contract and
supersedes clauses requiring no presence mask. Preserve all other contracts;
no new phase/default/render behavior. Validate RED→GREEN tests, suite, build,
and lineage.


## Approved Figure/Grid occurrence amendment (2026-10-19)

### Evidence before specification

Baseline `731f0ffdf4c7c152f590551e72ff63f7bd85ef85` preserves the measured proposal in `lab/results/v16-corrections-2026-10-19/figure-kind/review.md`, its `agents/contracts/design.md`, semantic partition, independent audit and counter receipts. The fixed oracle is commit `bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree `cac836c07e4dec2a89219370428292393a3216b5`. Constructor string kind loses function identity; constructor inference precedes occurrence styles; separated Grid fields lose source order. Typed identity, lexical presence, field traversal and counter results are language semantics/morphology. The chosen Rust carriers are implementation mechanics. The owner explicitly approved the linked public fields, contracts, phases and compatibility changes on 2026-10-19 under ADR-0127; this amendment materializes that bounded decision.

### Exclusive contract

Keep Content::figure(body,caption,kind:Option<String>,numbering) unchanged as a public adapter. Some(name) becomes kind Some(Some(FigureKind::Name(name.into()))), None means lexical omission. Initialize resolved_kind, supplement, resolved_supplement to None. Preserve existing numbering Styled transport exactly; no numbering field is restored. Legacy Some("table"/"image") now means a custom name, intentionally requiring typed migration where callers intend actual element identity. Native figure builds the typed entity directly.

### Acceptance and limits

Preserve all unrelated prior obligations and nucleus pins. This amendment supersedes only conflicting Figure/Grid clauses stated above. Require independent semantic RED→GREEN, exhaustive decision extension builds, existing workspace/build/tests/all-targets/Clippy/Rustdoc/fmt, canonical paths, strict forward/reverse lineage and valid V15/V26 before scoped resealing. No suppression, severity downgrade, weakened assertion or global hash repair. The bounded package does not certify full Figure/Grid parity: standalone HTML rendering, complete caption synthesis, supplement-dependent selector preview, external Dynamic enumeration, Grid footer phase and complex Table supplemental target order remain visible debt.
