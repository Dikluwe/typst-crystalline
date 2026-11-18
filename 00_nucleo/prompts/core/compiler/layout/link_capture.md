# L0 — Private link capture scopes
Hash do Código: 386e7d9b

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/link-frame-scopes.toml sha256:91cb1c097fc694c138acf2d044a3fc46dc7c41ec608ac557f086e1dc41ec0f57
- 00_nucleo/prompts/_nuclei/layout/physical-page-occurrences.toml sha256:b33f2e3c6238da01aa48783aecee6a8b6153b4f36ddcdc23edfe78434bae5b42



**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/link_capture.rs`

## Evidence before specification

`lab/results/construction/diagnostics/links-paginacao-2026-10-14.md` and receipts under `posicoes-compostas-2026-10-14/automatic-boundary-probes` record source, HEAD/dirty tree/UTC, binaries and vanilla pin. On a 160×40pt page with 10pt margins, ordinary text crosses the break; Link URL, Ref and Ref Page abort with a stale index after buffers are emptied. `references.rs:270` appears in the stack; `link.rs:22-32` uses the same mechanism. `outline.rs:28-50` also saves indices before the body. Clamping an index proves neither prefix/order/fragment/destination preservation nor geometry. Earlier snapshot: `lab/results/construction/diagnostics/links-captura-2026-10-14/before.json.gz`. Content, destination, regions and bounds are observables; private boundaries are mechanics.

`links-captura-2026-10-14/architecture.sarif` records V2: the new module lacks its own tests, while flow tests live in the Link owner.

## Ownership and contract

Own a private destination/lexical-scope registry and neutral open/close boundaries encoded through existing FrameItem variants (Semantic Artifact Other Inline, own NUL prefix, one zero Rect Shape without fill/stroke). Recognize exactly this format. The registry allocates no Location and executes no closure. Scope start/end append independent boundaries to line and committed buffer; line closure segments both before content transfer and reopens continuations afterwards. Region closure segments the committed buffer. Subframes have local pairs, and outer buffers retain their own pairs.

Resolve the tree after fixups and before document publication: consume boundaries, preserve every frame's order/region and construct FrameItem::Link for each nonempty fragment. Do not confuse membership with geometric intervals or collect old buffers by length. Descend through groups/wrappers; bounds use the helper owned by `00_nucleo/prompts/core/compiler/layout/link.md` in each vector's coordinate system. Equal inherited membership inside/outside a group needs no redundant nested same-destination links; preserve distinct destinations. Metadata-only segments create no clickable area. Do not remove positions, text identity or semantic wrappers.

Continue capture across line/page/column changes, preserve membership in deferred frames, exclude unrelated marginals/note/float and respect rollback. The public facade and phase remain layout. Do not hide capture loss by discarding text or fabricating a rectangle on the last page. This is internal ADR-0127 work without a new API/default/phase.

## Acceptance and limits

Locally test preservation of unlinked neighbors, fragment bounds, no area for an empty scope and preservation of a visible artifact whose alt resembles the private marker. These controls do not replace full-flow pagination, subframe and rollback tests.


## Planned source body and mixed siblings

Current link_capture.rs dispatches a source-owned Styled outline destination or a prepared Ref supplement through layout_content, which deliberately clears source cursors. The approved protocol and actual semantic failures are retained in `lab/results/set-page-header-implementation-2026-10-21/`; source fields and typed edges belong to the respective feature owners.

Retain the existing begin/body/fraction-expansion/end link scope and legacy helper signature. Private planned helpers may dispatch either one explicitly supplied actual body cursor or the unchanged part slice with an exact parallel mixed generated/source route. No caller may infer a source route from target text, Content equality or links. Mixed parts use the Sequence owner's normal-flow helper: generated numbering and separators remain unplanned, selected actual supplement retains PreparedReferenceSupplement. Preserve all link fragments, counters, source anchors, geometry, destination, empty scopes, rollback and unplanned behavior. This adds no public API/default or phase.


## Laboratory release-mode lexical scope closure, 2026-11-12

### Evidence before decision

`lab/results/construction/diagnostics/watch-residual-2026-11-12/loremlocal-link-census/receipt.json` binds source/ELFs/UTC and already-produced main-edit PDFs: v2 and loremlocal have 5258 first-page link annotations (169 distinct dictionaries), followed by 60 annotations on each body page, targeting all twelve auto-toc destinations. The pinned vanilla64c5e8c02fe0394d913ead36e42ef8c1334d2107 has twelve first-page annotations and none on body pages. Counts alone are mechanical; clickable BODY areas targeting unrelated outline entries contradict lexical link membership and the existing nucleus. `link_capture.rs:256` places the mutating active.pop() INSIDE debug_assert_eq!, so release removes the mutation together with the assertion. This matches the release-only growth and explains why debug tests are insufficient. The baseline snapshot is evidence/freeze-loremlocal.json; the prior source text and the RED invocation must be preserved.

### Exclusive correction and gates

Laboratory only; no integration. End each real capture by unconditionally popping the active lexical scope, then use debug_assert_eq! only to inspect that already-computed value. Closing markers, target registry, body invocation, nested ordering, region/line transport, wrapping/bbox, checkpoint restore and full discovery/replay remain unchanged. Do not discard extra annotations at export, clear all scopes, change marker grammar, suppress errors or shorten lifecycle by destination equality. Nested links must restore the actual outer scope, and unrelated later content/pages must not inherit either link. No public signature, default, dependency, unsafe or pipeline phase changes. This is an internal language-parity correction under continuous ADR-0127.

Add actual capture/flow tests in this owner: a completed link releases the active scope and later page BODY has no link; a nested capture restores exactly the outer scope before final closure. Run those tests against the unchanged implementation with debug assertions disabled for typst-core and prove the expected failures. Run the existing local scope/geometry/artifact controls in the same profile. After correction require GREEN in both assertion profiles, the complete core/infra suites, workspace check, strict lint/Clippy/docs/fmt and the normal release build. Actual release PDFs must preserve existing text/geometry/raster and legitimate URL/destination regions while eliminating leaked BODY links; compare lexical clickable regions/destinations to the pinned oracle rather than requiring an old defective annotation census. Preserve prior raw observations and outline geometry debts. Retest nested scopes, pagination, subframes, repeat aliases, rollback and page-running cases. Measure nominal CPU only after build, with exact source and ELF provenance.


## Laboratory vector reconstruction reduction — 2026-11-17

### Evidence before decision

`lab/results/construction/diagnostics/full-flow-current-2026-11-17/summary.json`
binds exact source/ELF/dirty HEAD/UTC: references_500 costs475.09ms CPU per full
edit cycle. Physical layout accounts for72.54–72.59% of samples; final link
resolution13.31–15.07%, position projection5.87–7.16%. `resolve` currently takes
and rebuilds every vector, even those with no direct private link delimiter;
`append` extends from singleton arrays for unchanged items. `RawVec` growth and
libc calls beneath append/resolve appear in the stacks. Allocation/retention is
mechanics; lexical membership, destinations, source effects and coordinates are
language obligations. These fractions are not a recoverable-gain estimate.

### Exclusive internal experiment

Laboratory only; preserve all earlier font/SourcePlan/semantic optimizations.
This owner alone may avoid rebuilding the local vector if it has no directly
encoded link delimiter, while still recursively resolving descendants. The
predicate is the existing exact decoder, including unknown target ids; an empty
registry never authorizes ignoring real delimiters. Do not scan by text/destination
heuristics or infer origins from frame equality. Preserve direct vector allocation
and capacity on the no-direct-marker path; descend groups, semantic wrappers,
links and text replacements exactly as before. Recursive modifications may still
change child vectors. Vectors with delimiters keep the existing lexical stack,
missing/unclosed delimiter behavior, materiality, same-target redundancy removal,
link bounding boxes and destination rules. A direct-item push may replace singleton
IntoIterator append; no new retained capacity proportional to the entire document,
result cache, global state, unsafe, dependency or mutable source certificate.

Do not change position projection, phase/order, source cursors, callbacks, discovery
or replay, physical memo lifetime, invalidation or effect execution. No public API,
default or compatibility change. Internal ADR-0127 continuous flow: L0/pins first,
V15/V26 valid, reseal, genuine RED before implementation, then GREEN.

### Tests and complete-cycle gates

On the unchanged real resolver, prove RED for storage/capacity retention of a
nonempty marker-free vector and recursive marker-free children; pointer/capacity
are diagnostic mechanics, not language parity. Confirm unchanged descendant
resolution when the parent has no direct delimiter. Retain an unmodified local
resolver oracle for differential nested/equal/distinct targets, mismatched/unknown/
unclosed markers, empty metadata scopes, legitimate marker-like artifacts and
position carriers. Add direct lexical geometry/order checks, not timing asserts.
No new side effect or Location allocation is permitted.

Run full core/infra suites, focused tests in debug and disabled-debug-assertion
profiles, workspace build/check, strict Clippy/Rustdoc/fmt and unfiltered lineage.
Matched release baseline/candidate with existing opt=s/ThinLTO/codegen1/alignment3;
byte-identical baseline alias. Twelve full-cycle blocks, six orders twice,14 cases,
startup/same bytes/two comments/visible edits; seven CLI rounds, first discarded.
Separate controls for exact input fresh/watch PDFs, source/style/include restoration,
state/counter/query/aliases/error publication, font/Unicode/regex, nested links,
pagination and transformed subframes. Preserve language observables at0.001pt,
links/destinations/bookmarks/tags; raster is auxiliary. Preserve prior oracle debts.
Profile separately after nominal; no sampler/census/debugger/phase JSON in nominal.
A local retention win without robust complete-cycle benefit is not performance
acceptance; retain regressions, raw results and A/A uncertainty. No gain inferred
by summing inclusive costs. Report allocation retention and measured RAM honestly.

One build job,12GiB AS,8GiB process-tree RSS ceiling, abort MemAvailable<6GiB;
runtime AS1GiB,stack64MiB,files64MiB; disk only. Record source manifests, exact dirty
stat,ELFs and UTC at every source transition. Preserve previous sources/outputs.
No integration and no commits.


### Diagnostic gate accounting for inherited test Clippy failures

The unfiltered strict all-target Clippy invocation in
`link-capture-vectors-2026-11-17/clippy.log` reports duplicate test cfg, two generic
borrow warnings and two mutable-key warnings in other unchanged owners. Preserve
that failure. Before classifying it as inherited, run exactly the same check on
the frozen baseline and require equal diagnostics/locations. Do not suppress lints,
repair unrelated owners or present a reduced check as the unfiltered gate. Run
strict production Clippy separately. Existing all-target failures remain an open
technical gate; they do not prevent recording this isolated experiment's full-cycle
measurements and preservation evidence, and they do not authorize integration.


The same inherited-gate accounting applies to workspace fmt: the unfiltered
candidate check reports formatting differences in other unchanged owners. Require
the baseline's normalized output to be exactly identical, preserve both failures
and leave that global gate open. Check this owner independently with rustfmt
--check. Do not format unrelated sources or claim that the owner check closes
the workspace gate. This affects diagnostic accounting only, not the algorithm.

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
