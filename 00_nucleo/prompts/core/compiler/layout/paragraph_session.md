# L0 — Retained paragraph products
Hash do Código: 81742093

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/paragraph-line-products.toml sha256:9e606cf207ae72ade678378bb4fe6c0aa4d1daa77b9c8fc2817b7c2afd4c8ea6
- 00_nucleo/prompts/_nuclei/paragraph-product-reuse.toml sha256:69dd4f797e17336227f56e038f9d833a0c6eff81bfc8e6beef4cf268fec27430


**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/paragraph_session.rs`
**Status:** Laboratory only. User waived laboratory approvals. No integration.


## Evidence before decision

subtree-layout-admission/receipt.json: current frozen source prepares102 segments
and rejects both complete plans in every 100-paragraph compilation. The route
RED in paragraph-validation/red-core.log proves this at runtime. Existing
paragraph_prepare.rs:234–337 and paragraph_plan.rs:145–205 already separate pure
word/gap/line products from regional emission. Cache accepted-only old plans
would miss this workload entirely. None of these mechanics defines language.

## Owned contract

Expose ParagraphSession::new/Default/Clone and stats(hits,misses,entries), backed
by a field-owned Arc/Mutex bounded memo, never globals or I/O. Expose opaque owned
MetricIdentity::new/Default/Clone for provider generations and a named-provider
owned-font constructor for verified immutable Font owners. Identity compares
owned Arc instance and variant, never borrowed address alone, bytes hash or name.
A custom metric identity must change when measurement behavior changes.

Own private Request/Key and Product containing immutable Prepared, selected Line
ranges and optional prepared hyphen lines. The plan owner constructs them using
its unchanged selectors. Request inspects actual input; initial retained subset
is uniform Text/Space/SpaceFromNewline/Empty through Source and Sequence only.
Decline Styled/Located/Realized/context/math/opaque values, active bidi/smallcaps
and unknown structure. Their original preparation still runs cold, so refusal
is not a new language restriction. Source spans are not cached in this existing
span-free Prepared product; fresh placement retains its own current provenance.

Key ordered typed atoms (preserve adjacency/separator class), full effective
TextStyle and numeric bits including lengths/edges/axes/spot tint, first and
continuation width, strategy, font size, justification, effective hyphenate and
overhang, and exact resolved line costs. Obtain identity from current metrics
for combined text plus spaces/hyphen before lookup. Cache only successful fully
prepared products, never errors, World reads or partial emission. LRU bounds:
512 entries,8MiB charged product/key payload,16KiB input per entry,64KiB key
payload per entry,4 distinct font owners each at most16MiB. Charge actual vectors
and retained text/style logical payload; bound arithmetic conservatively. These
are declared logical budgets, not exact allocator RSS. Unknown or oversized
inputs use cold path. Drop entries with session, expose count only for tests.

## Tests and acceptance

Cold stub must fail positive reuse at runtime. Instrument actual FontMetrics
advance calls: identical products across fresh Layouters must avoid measurements,
not merely increment a counter. Compare pages/text/word coordinates to cold
layout. Changed text/style/width/strategy/costs/metric identity must miss;
missing proof declines; fresh page position and provider generation remain
current. Independent sessions and retention bounds must hold. Refute omissions
with negative mutations. Check full language suites, PDF/SVG/query/geometry,
fonts/replacement/axes and actual watch controls before freezing. Measure CPU
paired with subtree and pinned vanilla; no claim of large gain from cache hits.
This stage does not retain final line carriers or skip regional placement.

Bound clarification before implementation: at most2048 typed atoms per key and
at most32 nested spot-color fallbacks; reject deeper/oversized input conservatively.
Hyphenation's owner reports its immutable vector payload with checked arithmetic;
retaining hyphenated products uses the same logical total budget.

Test correction from first-core-green.log: final alignment still invokes direct
advance per emitted word in finish.rs:138,255. Count actual text_width preparation
calls separately and require their elimination on hit; require reduced total
advance calls, not zero final-placement work. Layouter::new's legacy size argument
is ignored; set current StyleDelta.size to exercise actual style invalidation.

Further counter audit: metrics.rs:379 line_content_right also invokes text_width
for current FrameItem::Text during line alignment. Thus even the width counter
cannot demand zero in complete layout. The positive test must compare actual
cold versus warm invocation deltas and require a strict reduction while
geometry remains equal. The cold-stub/disabled-lookup refuter must still fail.
This replaces the preceding zero-text_width counter requirement; no claim that
this stage eliminates current placement measurements.

Admission and provider-identity matches must enumerate the full current domain
complement explicitly; adding a new Content/Identity variant must force review
of its admission rather than silently inherit a neutral wildcard.


## Full-suite refutation and adjacency certificate

Evidence: paragraph-validation/invisibility-core-all-before-connection.log
records8435 passes and three unchanged metadata/state/function invisibility
failures. With adjacent Text atoms, the new implicit single-line plan joins
"X"+"Y" into "XY", while the opaque route still yields plain_text "X Y".
This is an observable regression, not permission to weaken those tests.

Own a private pure preserves_atom_boundaries(parts) certificate using the same
ordered typed input collector, independent of a metric/session. Empty atoms are
neutral; a Text ending non-space followed by a Text starting non-space without
a Space/Newline separator is uncertified. Unsupported structure/bounds decline.
Each immutable Product records this certificate, valid only for its exact input
key. No changes to preparation's existing merging or public plain_text helper.

Charge the Product header too, conservatively in addition to its prepared/vector payload.


## Laboratory direct-body and context product reuse — 2026-11-13

Evidence before decision: paragraph-layout-boundaries/receipt.json, bound to
freeze-paragraph, records60 refused try_parts calls but78 try_layout calls
(72 accepted) on each outline compilation. The older statement that all outline
plans refuse applied only to try_parts and is superseded by this wider census.
paragraph-native-phases/summary.json records29.509ms elapsed inside context
preparation, not exclusive CPU or a promised saving. paragraph_plan.rs:89-136
prepares/selects the direct body without using the product session;
context_stabilization.rs:923 constructs compilation-local metrics without it.
This is execution mechanics, not a new language rule. No gain is claimed until
paired release measurement. User permits laboratory work; no integration.

Extend tests through direct try_layout: actual width measurement must fall on
a warm multiline body while text/pages/coordinates match cold. Changed width,
size, metric identity, hanging shape and current position must remain fresh.
Keep key/product/limits and typed admission unchanged; do not widen to Styled,
Located, Realized, callbacks or final frames. Test helper may choose direct
versus parts route explicitly, with existing tests remaining on parts.


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

Make MetricIdentity::same and style_payload visible only to sibling layout
modules. Keep their algorithms and guarantees unchanged. The physical-result
journal uses same solely to revalidate a captured horizontal certificate and
style_payload solely for logical owned-input charge, never as a new vertical
or complete-layout identity. No public signature, key or paragraph behavior change.

Expose the existing private MetricIdentity::font ownership/size projection only
to sibling layout modules for bounded retained-owner accounting. No new public
surface or metric guarantee is introduced by this visibility change.

## Laboratory line products — 2026-11-13

### Measurement before decision

Frozen shaping-products c2096c812761f5ab5278eb5376142e406990ec4b234615c9fbf6ce3be32feba6,
freeze 2026-11-13T06:06:24.083725Z, eliminates backend112->1/64->1 on edits,
but nominal CPU remains14.997/25.187ms versus vanilla3.683/4.234ms. Its twelve
raw profiles attribute30.45% inclusive layout to100 paragraphs and39.41% physical
coordination tooutline; overlapping groups are not a promised speedup.
paragraph_plan.rs:143-174,222-286 re-emits retained Prepared/Line products;
paragraph_emit.rs:90-125 walks fragments again; word_layout.rs:209-224 clones
styles and invokes baseline/indent for every fragment. cursor.rs:94-191 and
region_progress.rs:156-187 show that, with initial_baseline_pending=false,
block_chain_active=false and already material current_line, repeated admission
cannot advance region. paragraph_indent.rs:348-372 returns immediately once
origin_applied is true. bibliography.rs:30-33 returns when no prefix is pending.
word_layout.rs:89-159 then only appends Text/Replacement (plus Shape for
highlight), sets optical policy and records tracking. These are mechanics,
not language acceptance. No whole-Layouter snapshot is justified by these facts.
User authorizes laboratory work without further approval; no integration.

### Owned responsibility

Keep existing paragraph keys/admission/statistics unchanged. Add a private shared PaintSession carrier and a fresh detached identity in each immutable Product. Expose only a private carrier accessor. Paint has its own bounded payload; no new public signature or broader horizontal metric certificate.

Unit GREEN4 and actual parts/direct pipeline RED0pass2fail are preserved before-pipeline-connection. Product owns a fresh paint identity and a once-computed no-highlight certificate across its prepared/hyphenated fragments. Product-aware wrappers retain old emission facades as the refused path; keep the exact first-word/cold-tail/flush order.

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
