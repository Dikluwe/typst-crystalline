# L0 — Sealed page metadata store
Hash do Código: 1b4fd2a2

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/bundle-numbering-input.toml sha256:2e87970ea88f2802436705618856cc6aff0563d0fdbe8724d3d0221e59d494d9



**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/page_store.rs`

## Evidence and resolved specification

Retained architectural evidence is
`lab/typst-original/crates/typst-layout/src/introspect.rs:38-58,22-145`.
Sealing uses a separate immutable sub-store. Existing later raw-Numbering and
realized-view clauses supersede the old EcoString-only schema; this comes from
the L0 amendments, not from adopting code as authority. Earlier staging and
conflicting derive notes are recorded in the sanitation diagnostic.

## Construction and queries

`PageStore` is pure L1 with private storage, Debug/Clone/Default, immutable after
construction/injection. `empty` is the default sentinel: total_pages is None,
`is_empty` is true, and every page query returns None.
`from_total_pages(NonZeroUsize)` supplies only total and empty metadata:
`is_empty` is false even when individual queries return None.
`from_runtime(total, Vec<Option<Numbering>>, Vec<Content>)` remains compatible;
it uses physical one-based indices as logical snapshots and supplies no realized
numbering views. Caller-provided numbering/supplement slots align with total.
`from_realized` accepts total, raw Numbering slots, logical numbers, spans,
visible-numbering slots, reference-numbering slots and supplements in physical
page order. All vectors align one-to-one with total pages.
Raw numbering, the realized two-argument marginal view and the realized unary
reference view remain distinct. Missing view slots differ from present empty
Content. No Engine or callback is executed by the store.

`total_pages` returns the optional total. Nonzero one-based accessors
`numbering_for_page`, `logical_number_for_page`, `numbering_span_for_page`,
`visible_numbering_for_page`, `reference_numbering_for_page` and
`supplement_for_page` expose corresponding snapshots. Raw numbering returns
`Option<&Numbering>`; views/supplements return `Option<&Content>`.
Outside available slots, return None without panic. An absent supplement slot
differs from a supplied empty supplement. `is_empty` tests only absence of total.

## Boundaries and unresolved restriction

Capture, injection and query routing belong to their pipeline/introspector
owners, including `00_nucleo/prompts/core/entities/introspector.md`. Tracking remains
at the Introspector boundary, not on a reference-returning store impl.
The earlier explicit no-PartialEq restriction conflicts with the current derive
and is unresolved; sanitation does not approve a public trait change.
No I/O or mutable global state is allowed. Verify sentinel/minimal/full stores,
aligned slots, distinct raw/realized views and nonpanicking range queries.

## Approved document-local numbering input (2026-10-28)

Measure before decision: readiness-fixes-2026-10-28/physical-candidate-execution.json,
its source archive and full preservation log bind dirty HEAD212aed0449, exact
diffstat, inputs and UTC. p1486 keeps /St20 and /St22 but the last PDF paints10
instead of20, because run_planned consults global PageStore by local ordinal.
The review review-bundle-numbering-input.md SHA-256
d248aa88694a06da945a2036ec33887fcbba0f67adf3e3bd7ccb30757354670d was approved
in direct human reply, retained in bundle-numbering-input-approval.json.
This ratifies only the named additive method and its local transport, superseding
earlier no-new-API clauses only for that method; all other boundaries remain.

Own only an internal crate-visible interval completeness certificate, inspecting
existing private vector lengths and checked bounds. This is pure data validation
for the approved facade, not an additional externally public API. Require every
raw/realized/logical/span/supplement slot even when its stored Option is None.
Leave all existing constructors, fields, query methods and defaults unchanged.
Never use Content/Func equality as completeness proof or execute any content.

Require functional RED→GREEN, original integrated controls, complete technical
gates and independent PDF/SVG/word/page/0.001pt validation with source provenance.
Finite tests do not certify general parity, geometry, performance or JPEG.

Functional RED before implementation: numbering-input-red.json/log and source
archive retain the approved API skeleton returning its unchanged clone. The
valid non-first interval has no local view, and an invalid4/1 interval is accepted;
both tests fail. outline-original3-green separately passes its corrected carrier
observer under the same frozen inputs. The p1486 original integrated RED remains
the independent transport acceptance gate; no expected values may be relaxed.


## Laboratory complete physical-result retention — 2026-11-13

Evidence before decision: layout-read-audit/summary.json, observations and source
freeze bind actual nominal-suffix compilations. Normal/narrower/numbered outlines
each perform six planned calls; three preserve the source plan, tracked-read
screen and weak PageStore screen with empty running/math/par.line channels.
Header controls do not pass the empty-channel screen. The audit's runtime test
proves comemo Content hashes do not preserve separator origin. Neither that hash
nor PageStore PartialEq is a reuse certificate. Source evidence:
layout/convergence.rs:211-250 reads PageStore and bibliography outside tracking;
metrics.rs:343-400 contains an overridable line geometry method; all overrides
must survive. This is mechanics, not a new language parity claim. The user
authorizes laboratory changes without an approval stop; integration is excluded.
This refinement supersedes no-new-public-API clauses only for its named lab API.

Add PageStore::same_scalar_snapshot(&self, other: &Self)->bool, a conservative
exact proof over every private field and vector length. Compare totals, logical
numbers and spans exactly; raw numbering accepts None or Pattern only, refusing
Func even when shared. Content snapshots accept Empty, Text, Space,
SpaceFromNewline, Parbreak and bounded ordered Sequence recursively, preserving
variant/origin; all other variants refuse rather than use Content equality.
Compare optional presence and missing vectors distinctly; never execute content.
This does not change PartialEq or any constructor/query. Add actual RED positives
plus negative origin/length/numbering tests before implementation. The retained
cloned PageStore keeps all scalar data owned; the certificate authorizes no
source/semantic identity outside this exact snapshot.

## Productive adoption scope — 2026-11-18

The user explicitly authorizes the cumulative packages with measured gain. This
owner adopts only its finalized implementation in the preserved capture candidate
29115606, qualified together with its required dependency owners; earlier
laboratory-only exclusions are superseded only for this specific implementation.
Existing current project ADR scope clarifications and conservative fallbacks
remain in force. No additional public method, default or phase is invented here;
already specified optional capabilities keep their explicit authority/None paths.
Diagnostic census, no-retain ablations and blocking watch markers are not product
features. Current adoption uses disk-backed task targets, one Cargo job,12GiB AS,
8GiB tree RSS and no swap; prior task-specific temporary-target paths are scoped
to their original correction. Require real capture RED→GREEN, complete suites,
lineage/ownership, source/effect/invalidation controls and measured before/after
and pinned vanilla comparison. No commit is authorized by this adoption.
