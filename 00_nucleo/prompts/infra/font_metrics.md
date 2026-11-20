# L0 — infra/font_metrics
Hash do Código: 7daa94ae

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/fonts/complete-shaping-products.toml sha256:d612a88c4d14b13ea8e783dc06a984e26cc2c9978983a93dbabc6fac1277c2e3
- 00_nucleo/prompts/_nuclei/fonts/immutable-metric-epoch.toml sha256:2ae956d6a0693c848353f4edfc70aea0888231adbb30b23765917b2bbb57d60e
- 00_nucleo/prompts/_nuclei/fonts/primary-resolution-cache.toml sha256:34e78c2ed34beeefbbd0e6a025b94322f27dadce8641ae58fe6a074068a1f30a
- 00_nucleo/prompts/_nuclei/fonts/raw-shaping-cache.toml sha256:ba89868d23ef2304111b39efef27f781b39b74141f68b4abdc048c0088e0ae61
- 00_nucleo/prompts/_nuclei/paragraph-product-reuse.toml sha256:69dd4f797e17336227f56e038f9d833a0c6eff81bfc8e6beef4cf268fec27430



**Layer:** L3
**Exclusive consumer:** `03_infra/src/font_metrics.rs`

## Current evidence

- `03_infra/src/font_metrics.rs:27` — `FontBookMetrics`.
- `03_infra/src/font_metrics.rs:40` — `from_bytes`.
- `03_infra/src/font_metrics.rs:49` — `FallbackFontMetrics`.
- `03_infra/src/font_metrics.rs:68` — `new`.
- `03_infra/src/font_metrics.rs:80` — `clone`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/font_metrics.rs`.

## Owned contract

- Own FontBookMetrics/FallbackFontMetrics state, lifetimes, constructors, Clone and existing internal reexports only. Metric algorithms, caches, fallback, MATH and resolution belong to their child owners.
- FontBookMetrics::from_bytes parses face index zero, returns None for invalid bytes, uses 1000 for zero units-per-em and eagerly builds its reverse glyph map through the math child.
- FallbackFontMetrics::new borrows World and creates lazy Arc/Mutex caches, including the shared shaper face cache. Clone shares every cache and the same World; no global state or I/O enters L1.
- Preserve the L1 FontMetrics contract and existing crate-private edge_offset_pt/ssty/reverse-map access. ttf-parser/rustybuzz types stay inside L3; do not add public signatures or broaden private child visibility.

## Child owners

- `00_nucleo/prompts/infra/font_metrics/basic.md` owns only `03_infra/src/font_metrics/basic.rs`.
- `00_nucleo/prompts/infra/font_metrics/caches.md` owns only `03_infra/src/font_metrics/caches.rs`.
- `00_nucleo/prompts/infra/font_metrics/fallback.md` owns only `03_infra/src/font_metrics/fallback.rs`.
- `00_nucleo/prompts/infra/font_metrics/math.md` owns only `03_infra/src/font_metrics/math.rs`.
- `00_nucleo/prompts/infra/font_metrics/resolution.md` owns only `03_infra/src/font_metrics/resolution.rs`.

## Verification and boundary

Run `cargo test -p typst-infra --lib font_metrics:: -- --test-threads=1` when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.

## Bounded primary-resolution reuse

### Evidence

`lab/results/construction/diagnostics/content-cost-profile-2026-10-10/{resolution-calls,cpu-style-current,checked-phases}.json.gz`
retains HEAD/tree/UTC, scripts/sources/commands and release SHA
`9ae0a3d9285b8d0b4bae0da1db058483e7f5543003fabff46bb7d93b17e474f2`.
Tables of 1, 600 cells call select_pattern 48, 019 times, resolve_primary 6, 401,
text_edges/vertical_metrics 3, 200/3, 201; selection appears in 25–29% of inclusive
CPU samples. resolution.rs:27–85 forms the base chain from FontList/effective
variant/face readability, without size/variations; :105–124 adds MATH separately;
font_metrics.rs:49–86 already shares clone-local caches. Inference: last-resolution
reuse may amortize scans without growing retention. Identity/chain divergence
or lock/retention cost eliminating benefit refutes it.

### Contract and acceptance

Add only a private last-complete-primary-resolution carrier, initially empty in
new and shared by Clone. Key/entry types and algorithm belong exclusively to
`00_nucleo/prompts/infra/font_metrics/resolution.md`. Consume the pinned nucleus;
add no public fields/signatures. Continuous ADR-0127 requires repeated-query RED
through clones; family/regex/order/variant controls, independent instances,
transient failures and math/variations against fresh instances; quantitative
allocation/paired-time gate, documents equivalent to the preceding release and
many-identity refuters; core/infra/build/lint. Language acceptance observes
selection/metrics/origin/effects/geometry, never vanilla mechanics or inferred
hash hits. Remove on divergence or robust regression.


## R8 — approved font-backed superscript integration

Evidence before decision: the source-bound `footnote-layout-r8-2026-10-24/before/receipt.json`,
`font-tables.json` and `red-geometry.json` under `lab/results/construction/diagnostics/`
record wrong 0.65 scaling/positive baseline and font-dependent pinned geometry at
bd5ebec285505889e8ff1d4f68804ab147a699f3. `approval.json` binds the human-approved
FontMetrics::text_superscript_metrics addition. OS/2/whole-run GSUB are text metrics;
MATH constants and fixture/font-name coefficients are not substitutes. This private
integration follows ADR-0127 continuous parity flow, superseding only superscript
scaling/feature assumptions below. Keep subscript behavior and unrelated folds.
Expose the existing private single-face superscript helper within L3 for shaping;
no new public API. Preserve resolution, caches and all unrelated behavior.

Require actual RED→GREEN tests, fixed-pin PDF/SVG controls, unchanged geometry tolerance and complete workspace/lineage validation.

## Raw shaping reuse: evidence before decision

The shaping-input-overlap-lab-20261111 census on clean HEAD 48e5a2570159c6fc325babe883bac7d1c1fbd4b2 compared complete inputs and independently replayed 3,788 calls. In layout.typ, 111/111 emission misses had an earlier equal measurement among 113 measurement calls; math admitted only 2/9. Tracking coincidence applied to raw buffers, not final adjusted carriers. Counts do not prove CPU/memory benefit. See the preserved external census and the correction's measurements for exact states/times/ELFs.

Internal correction authorized by the owner; ADR-0127 continuous flow, no public/default/phase/compatibility change. The pinned raw-shaping-cache nucleus supplies shared invariants; this owner authorizes only its following responsibility.

Add only the crate-private adapter that clones the raw store under its own face-cache lock, releases that lock, and invokes traversal with self.world. Preserve constructors/Clone and all existing metric caches and algorithms.

Keep regression tests first and preserve source/frame/glyph/position/font/fallback semantics. Directed missing-key and bypass-reuse mutants must fail; compare allocation traffic, peak and CPU in the real pipeline including retention/key costs, without an arbitrary target. Compile only in own /dev/shm and preserve all existing checkout changes. Outline's semantic divergence and prior identity debts remain explicit. The retained eval/font-session prototype is a lab measurement base, not integrated by this correction.


## Laboratory first-query reuse correction — 2026-11-12

Evidence precedes decision: fontdemand-fallback-comparison/freeze.json and summary.csv record unchanged release d7c9024946a3ffde6f5ca0825829eeff29c7b17f60ce26b6ef69670a0e39c7a8 regressing in system and nonminimum-variant workloads. resolution.rs:first_primary and font_selection.rs:first_candidate query World again whenever no complete chain was populated; the selective successful path never populates that chain. phase-counts/receipt.json records 1115 certifications for 113 actual shapes on layout, not a timing attribution.

Keep a DISTINCT latest successful first-readable-query result, scoped exactly like the existing per-compilation face/metrics cache. Key by exact full ordered FontList (optional for metrics) and effective FontVariant. It answers only first-face queries and can never substitute for a complete primary/fallback chain, glyph coverage, bidi splitting or raw-shape certification. Store only key, slot and units-per-em; no additional bytes, global state, cross-World/compilation reuse, negative selection cache or public API. Query/read without holding the metrics selection lock. Do not publish if any earlier selected face failed loading/parsing; shaper conservatively refuses while any existing face-cache failure is present, preserving its old negative-face policy. Existing full-chain hits remain usable and take precedence. New starts empty; metrics Clone shares first-query state only with the same World. Size/color/language/axes retain existing downstream application.

Continuous ADR-0127 internal repair. Require counter RED before implementation; repeat hits do not query catalog, keys and Worlds remain isolated, failed faces keep prior retry policy, and subsequent full-chain resolution equals fresh output. Preserve whole-PDF no-acquisition control, all prior semantic tests and font freshness. Rebuild/lint/paired nominal and forced-full regressions before acceptance; laboratory only.

Exclusive obligation: Own the private shared optional first-resolution field, empty construction and same-World Clone; complete cache remains separate.


## Laboratory paragraph retention — 2026-11-12

Evidence first: subtree-layout-admission/receipt.json records102 preparations
and two refused complete plans per100-paragraph compilation at frozen subtree.
paragraph-validation records actual route geometry GREEN and cold-cache runtime
RED (22 vs11 calls). Prepared contains word/gap measurements and selectors,
without World, positions or callbacks. This experiment preserves all current
phase order and creates a bounded explicit product session. User waives gates
for laboratory work; no checkout integration.

Add an optional private ParagraphSession field, absent in new and cloned only
as a pure-product handle. Expose with_paragraph_session(world,&session) creating
otherwise wholly fresh metrics/selection/raw/face caches. Never retain these
World-bound caches in the paragraph session.


## Laboratory immutable metric epoch — 2026-11-13

Evidence before decision: physical-retain/conclusion.json and its exact release
freeze record six-to-three physical execution reduction but only about5% CPU gain
on the outline. physical-retain-deep-stacks/structural-summary.json measures
11.5% inclusive samples in Journal::validates, with additional metric request
recording/style copying. These are execution mechanics, not language semantics.
Source world/fonts.rs:168-216 routes every query beyond the immutable prefix
through font_catalog(); world.rs stores the full catalog in OnceLock. FontSlot
get at fonts.rs:179-216 keeps embedded bytes in owned Vec and disk fonts may be
Mmap. Fallback metrics and transitive shaper queries use only the font-facing
World methods. Therefore a still-uninitialized full catalog plus the owned
embedded-prefix identity can certify this restricted domain; full-catalog or
arbitrary provider identity alone cannot. Runtime tests must refute expiry,
provider replacement and a falsely complete journal. No performance gain is
assumed. The user explicitly waives approval stops for laboratory work; this
additive experiment supersedes prior no-new-API clauses only for named methods.
Integration remains excluded. Follow the pinned immutable-metric-epoch nucleus.

Exclusive owner obligation: Add a private Arc<()> provider-instance identity to FallbackFontMetrics, fresh in new and shared only by Clone. Do not retain World in any L1 token. Keep all existing metric caches and paragraph-session attachment.


## Laboratory complete shaping products — 2026-11-13

### Evidence before decision

The frozen fraction-demand release (f34f54de100fc63587f7611d042dc3d836b484f95283a2f5fc029c8afadce65e)
and fraction-demand-work-shift/causal-summary.json record 112 backend executions
per100-paragraph compilation and64 peroutline, unchanged by removing2004/2172
width queries. All three cycle kinds repeat this work. structural-summary.json
attributes18.39% inclusive samples to emitted shaping and5.31% to raw shaping
in100 paragraphs; these groups overlap and are not a speedup prediction.
font_metrics.rs:100-106 shares only raw buffers within a compilation;
shaper/traversal.rs:30-49 reconstructs faces/selection/output caches;
shaping.rs:262-346 rebuilds measured widths and:363-679 rebuilds emitted products.
world/fonts.rs:21,35-39,291 retains one immutable prefix owner across a font
session and revokes before full-catalog exposure. Current PdfCompileSession
at pipeline/pdf.rs:1133 has no shaping products. These are execution mechanics,
not a change to language semantics. The user explicitly authorizes laboratory
implementation/testing without further stops, including experimental APIs;
integration remains prohibited. No performance benefit is presumed.

### Exclusive responsibility

Add a laboratory constructor/adapter that attaches an explicit shaping product session alongside existing paragraph products, without retaining World-bound metric caches across compilations. Carry only that optional session with the existing raw-store adapter into emitted shaping; keep existing constructor and private facade behavior available.

Own with_product_sessions(world, paragraphs, shaping), an explicit laboratory constructor that delegates paragraph attachment and attaches the owned ShapeSession only to the fresh shaper faces. The existing shape_document_with_context facade forwards its optional product handle alongside the compilation-local raw store; its signature remains unchanged.

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

## Reúso de bordas tipográficas no fechamento

O censo em `lab/results/construction/diagnostics/references-500-reused-lines-2026-11-19/probe/observed.json` registra cerca de 25 mil
chamadas de text_edges por produção. Cada provedor real possui
uma entrada compartilhada só entre seus clones para bordas padrão
certificadas pelo domínio imutável da consulta de fonte. Seu filho fallback
define a chave e as condições de publicação. O cache começa vazio para
cada novo FallbackFontMetrics e tem capacidade unitária.

## Reúso de larguras repetidas

O censo de references_500 em lab/results/construction/diagnostics/references-500-item-width-2026-11-19/probe-summary.json
mostra que os cálculos de largura de item são majoritariamente Text, com
poucos textos distintos. Cada provedor real possui uma tabela adicional de
larguras shaped certificadas, compartilhada entre seus clones. Ela começa
vazia em novos provedores. O L0 de caches define sua chave, orçamento e
publicação. Mudança interna em fluxo contínuo ADR-0127.
