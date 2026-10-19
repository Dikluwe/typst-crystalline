# L0 — builder/metadata
Hash do Código: 6a0a6dde

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:395a968c2fe49429b03b369cd5047df513132bfef6511ec21af466670ec2e147
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:88391e330334a2221170109c82b5dbf1fb60c4d2f6e2660e732df24b67554d9f

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/builder/metadata.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/builder/metadata.rs:22` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Metadata and identification

Own Info, XMP, timestamps and document/instance ID generation, archival output
intents, tagged structure trees and PDF/UA viewer preferences. Do not serialize
xref/trailer or attachments; publish the existing stored references for those owners.

Emit Info except in A4/A4f/A4e. Encode Title/Author/Keywords as UTF-16BE hex with BOM
and retain the Typst Crystalline Creator branding. Select the present
CRYSTALLINE_PDF_FIXED_EPOCH value, otherwise SOURCE_DATE_EPOCH, before parsing.
A missing or invalid selected value falls back to current UTC; an invalid primary
value does not retry the secondary variable. Info PDF dates and XMP ISO-8601 dates use that policy.
Always emit a Type Metadata/Subtype XML stream and catalog reference. Escape XML
metacharacters, preserve user metadata presence, application/pdf format, proof
rendition class and selected page count. Generate separate standard-base64 instance
and document identifiers from 16 bytes; tests/fixed-epoch use the existing distinct
constants. An external document ID overrides only the document identifier. Persist
the pair for serialization; do not claim production bytes are globally deterministic.

Use archival identification exactly for the selected mode: A1/A2/A3 have part and
their A/B/U conformance as applicable; A4 has part 4 and rev 2020 without conformance;
A4f/A4e additionally use F/E. Emit the archival namespace only when applicable.
All archival modes receive the output intent and version-selected profile supplied
by the images ICC owner; preserve the GTS_PDFA1 subtype, profile reference and
existing descriptive fields. Do not infer full archival compliance from these fields.

For UA1, emit pdfuaid part 1 before the NPages field and DisplayDocTitle. With archival plus UA1 include
the minimal PDFUAIdentification extension schema, pdfuaid namespace/prefix and
internal Integer part property. Preserve document language in catalog and XMP;
use en only when the document language carrier is absent. Local styled language
does not retroactively replace the document language. Do not add viewer Direction.

## Tagged structure

Build structure only when PdfTags is Enabled. Bind to the page-local MCID table
owned by the stream façade rather than assigning a second numbering algorithm.
Emit per-page StructParents, ordered ParentTree associations, StructTreeRoot and
MarkInfo with Marked true/Suspects false. Disabled tags omit this structure.
Artifacts and descendants never become StructElem/MCID content. Formula alt is
nonvisual; preserve absent versus present-empty alt. Labels use Lbl.

Tables preserve Table/TR/TD, uniform automatic headers THead/TBody/TR/TH/TD,
and direct TR for explicitly mixed rows. Explicit Data in a header remains TD.
Summary belongs only to Table and preserves absent versus empty. TH Scope supports
Row/Column/Both; IDs and header associations follow the logical header carriers,
not a numeric PDF level attribute. Emit span attributes only above one. Repeated
headers across pages reuse the logical TH and do not duplicate IDs or relationships.

UA1 represents a physical heading as one H1-H6 unit and a paragraph as one P unit,
including multi-run content. Preserve Span children, Strong/Em role mapping,
Link/Figure alternative descriptions and language. Do not infer headings from
boldness or font size. Paragraph/line boundaries do not create empty nodes.
Link annotation OBJR and ParentTree relationships bind the references prepared by
navigation; no orphan keys. Keep meaningful content real, not disguised as Artifact.
Structural positives do not establish assistive-technology, reflow or full UA claims.

## Verification

Exercise metadata presence, fixed/external IDs, selected page counts, all archival
variants, UA-only/archival+UA, language, formulas, tables, repeated headers,
paragraph boundaries, links and tag-disabled omission.

## Causal text carriers in structure

Source→PDF RED/handler inspection in fatia-funcional-tekt-2026-09-28 precedes
layout_types public-carrier approval received2026-09-28 and recorded in provenance.
TextSeparator belongs to the same textual leaf/Span language/style under UA1;
never add separate MCID algorithm/heading/paragraph solely for the carrier.
Approved identidade-hifen-discricionario-2026-09-28 initial/before eight U+002D/
SHY equal-bbox controls/audit retain exact state and layout_types owner approval.
Structural walker descends TextReplacement without its own structure/MCID;
visual leaves own style/language/MCID. Keep text/bbox/causal/recursive/literal-
hyphen controls, not complete PDF bytes/new break selection or shared ownership.

## Explicit structural guards

### Evidence

`lab/results/construction/diagnostics/tekt-guards-2026-10-16/before.json.gz`
retains checked sources/L0/SHA-256, HEAD
06848cdcb5450f2c555626bf116bc7acd82bd82b, dirty tree/UTC/status/diffstat.
metadata.rs:808/:835 are archived T310/T312. Hash-reconfirmed inventory 28 guards/
21 owners, pinned CI requires split/body conditions. Rust structure proves no
functional defect/AI readability/upstream intent.

### Contract and gates

Merge Link arms in collect_top, decide ua1 && !inside_artifact in body. Eligible
retains Link tree/children/empty removal; fallback recurses directly into same out.
Remove Link from Group/TextReplacement arm. Catchall decides eligibility in body;
keep semantic_node/MCID counters/Artifact/PDF-UA boundaries/order. No public/default/
compatibility/phase, conjunction-hiding helper/suppression/linter change. L0 first,
existing functional controls unchanged; real V17 RED28→GREEN0, success/fallback/
error/short-circuit/effects checked. Scoped seals/build/full workspace suite/strict
all-target Clippy warnings+unfulfilled expectations/fmt/Rustdoc warnings/strict
V1/V5/V15/V26/reverse/SARIF no new warnings under12GiB/zero swap/one job. No
unmeasured memory/performance/global-parity claim.

## Explicit existing FrameItem complements

tekt-frame-items-2026-10-16/before.json.gz retains same HEAD/dirty tree/UTC/sources/
L0/configuration/hashes/status/diffstat, pre-code checked AST/spans/patterns/guards/
bodies/complements. classification.json has 70 cases/26 owners under pinned CI.
Rust exhaustivity is mechanics; semantics/morphology/geometry/effects/diagnostics
stay. No new architecture/intent.

- ua1_leaf_node :744 (historical F053): replace only `_` with
  FrameItem::TextReplacement/Semantic/Line/Shape/Group/Link.
- collect_top :854 (F054): replace only `_` with
  FrameItem::TextSeparator/Text/TextShaped/Line/Glyph/Image/Shape.

Preserve full fallback body/prior arms/guards/transforms/calls/returns/order,
callbacks/effects/metadata/clipping/fonts/position/ink/width. No traversal expansion,
wrapper→leaf or text/ActualText geometry substitution. Tests keep fixtures/expected/
tolerances/counts/Unknown failures. V19 group notes visible, not proof every
alternative executes.

Continuous ADR-0127 adds no enum/field/trait method/public signature/default/
dependency/compatibility/phase and approves no historical pending proposal.
L0 first, scoped bidirectional resealing; no suppression/higher threshold or
match→if-let evasion. Before code run new FrameVisitor control/workspace suite
with unchanged expectations and confirm AST RED for all 70 family warnings.
Afterward zero scoped-family V16/no new warnings; outside V16 remains pending.
For productive core matches reached by cargo check -p typst-core, use a disposable
FrameItem sentinel variant to observe absent E0004 before/present after. That
probe covers only compiled sites, never unbuilt cfg(test) or blocked L3 sites.
For the full batch replay patterns/guards/bodies, verify enum/signatures unchanged,
real build/tests, no stub bodies.

Revalidate full workspace suite/cargo build/strict workspace-all-target Clippy
(-D warnings, -D unfulfilled_lint_expectations)/fmt/Rustdoc -D warnings/four HTML
indexes/strict V1/V5/V15/V26/reverse lineage/SARIF/diff --check. Freeze sources/L0/
configuration during runs; heavy commands sequential under MemoryMax=12 GiB,
MemorySwapMax=0, CARGO_BUILD_JOBS=1. Structural cleanup is not new global parity,
memory gain or measured AI benefit.

## Exhaustive semantic-boundary classification

### Source evidence before decision

At HEAD `841c9855f8ad6695f4e140d1ee4a33e4f4a46689`,
`03_infra/src/export/builder/metadata.rs:767–872` contains `collect_top`;
its `match kind` at `:778–790` is the V16-086 site. The existing
`ua1 && !inside_artifact` guard intercepts ParbreakBoundary by appending a
Boundary marker and returning, and ExplicitLinebreakBoundary by returning.
`01_core/src/entities/layout_types.rs:312–352` defines the eleven current
SemanticKind alternatives. The other nine currently continue at `metadata.rs:792`
into artifact classification, `semantic_node`, and recursive child collection.
This is static control-flow evidence, not a new PDF/UA or language measurement.
The source/L0 inventory is retained in
`lab/results/v16-cause-analysis-2026-10-19/entities-boundaries-inventory.json`.

### Decision and preserved complement

Keep both boundary arms and their guards, returns, ordering and marker fields.
Replace only the inner unchecked continuation with the explicit current kinds
Formula, Label, Artifact, Table, TableHead, TableBody, TableRow,
TableHeaderCell and TableDataCell, retaining their empty continuation body.
That body means continue through existing semantic processing, not discard the
kind, its children or its MCIDs. Preserve inside-artifact propagation, no empty
UA1 paragraphs, tag-disabled behavior, Link associations, table-header reuse,
Formula alt presence, semantic-node selection and page-local MCID order.
No inference from typography or new kind-to-PDF-role mapping is introduced.

### Acceptance and exact-site evolution proof

Propose a focused export-structure control covering both intercepted boundaries
between meaningful children, all nine continuation kinds, UA1/non-UA1 and nested
Artifact. Assert existing paragraph/line grouping, absence of artifact StructElem/
MCID entries and unchanged meaningful-child order/MCID relationships. Observe
structure and diagnostics rather than complete PDF bytes. New tests outside this
consumer require their own test L0 update; this owner does not absorb them.
Existing integration controls include
`export::tests::p1140_6_tagged_pdf::{default_emite_formula_mcid_parent_tree_e_mark_info,disabled_remove_toda_a_estrutura_sem_remover_desenho,mcids_sao_deterministicos_e_alt_vazio_e_preservado}`
in `03_infra/src/export/tests/mod.rs:10021–10112`, and the three
`p1481_*` controls in `03_infra/src/export/tests/pdfua_structure_tests.rs`.
Their assertions and fixtures remain unchanged.

In a disposable workspace copy, add a sentinel only to copied
SemanticKind. The original consumer must compile without the sentinel; its
wildcard version lacks E0004 at this inner match, and the repaired version must
produce E0004 at this exact inner match, not merely another SemanticKind match.
Compile the actual typst-infra library target so this nested function is checked;
no test-only extract, stub or productive enum mutation counts as evidence.
A continuation that skips semantic_node/children or changes MCID/order/Artifact
behavior refutes the repair even if V16 disappears.

This is an internal classification correction under ADR-0127 continuous flow:
no public field, trait, signature, product default, compatibility rule, dependency
version or pipeline phase changes. Preserve the owning layer and static dispatch.
No suppression, severity change, `if let`/`matches!` replacement, or helper that
relocates the unchecked complement is an acceptable repair. Validate V15/V26
before restricted forward/reverse resealing. Freeze source/dependency identity during structural and functional validation;
record exact targets, compiler diagnostics, resource settings and outcomes before
claiming acceptance. A structural proof is not a functional test result.

### Attributable actual-domain omission proof

For V16-086, an equally strict structural alternative avoids a new enum variant
preventing core or external-dependency metadata emission before the real infra
consumer is type-checked. Use two disposable copies with the same unchanged real
dependencies: one with the original wildcard decision and one with its repaired
exhaustive classification. At the collect_top inner SemanticKind match at metadata.rs:778–790, remove only
the complete unguarded ParbreakBoundary arm in each copy. This arm is unique and unguarded; no other arm
in that match handles ParbreakBoundary. With the omission, the wildcard baseline
still covers ParbreakBoundary and must lack E0004 at this site; the repaired copy must
emit E0004 at this exact match. Retain function/target identity, mutation diff,
source/dependency hashes, compiler JSON and no-omission controls. An unrelated
error or dependency failure remains inconclusive. No replacement stub, new public
API, linter or target-scope change is allowed.

This paired omission experiment proves that the actual-domain classification no
longer has an implicit complement. The normal repaired match must remain fully
nominal and exhaustive, so future enum growth still forces an explicit owner
decision. This alternative changes only how the structural obligation is measured;
it weakens neither the future-variant obligation nor the independent functional
controls, and no probe result is claimed by the specification.
