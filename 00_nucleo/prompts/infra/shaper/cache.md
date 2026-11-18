# L0 — infra/shaper/cache
Hash do Código: 3a1e105b

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/fonts/complete-shaping-products.toml sha256:d612a88c4d14b13ea8e783dc06a984e26cc2c9978983a93dbabc6fac1277c2e3
- 00_nucleo/prompts/_nuclei/fonts/primary-selection-proof.toml sha256:5a0fb32f391d73057b0b38d83c211b834b23ce2f8055ffee25ee3be671b5adef
- 00_nucleo/prompts/_nuclei/fonts/raw-shaping-cache.toml sha256:ba89868d23ef2304111b39efef27f781b39b74141f68b4abdc048c0088e0ae61
- 00_nucleo/prompts/_nuclei/fonts/shaped-face-identity.toml sha256:06ae067763e4c77876f7d999903f2e5e4144df34eb799eb62f2c52a31d8a030b
- 00_nucleo/prompts/_nuclei/fonts/shaper-primary-cache.toml sha256:4b54a922e743e872fd4806c103368a9ccd3b2b736a54eeb7de8f5d1a4b9630ab



**Layer:** L3
**Exclusive consumer:** `03_infra/src/shaper/cache.rs`

## Current evidence

- `03_infra/src/shaper/cache.rs:19` — `ShapeCache`.
- `03_infra/src/shaper/cache.rs:24` — `CachedRun`.
- `03_infra/src/shaper/cache.rs:30` — `new`.
- `03_infra/src/shaper/cache.rs:34` — `key`.
- `03_infra/src/shaper/cache.rs:61` — `FaceCache`.

These are local source anchors, not a new upstream parity measurement.
`lab/results/construction/diagnostics/reuse-key-followup/results.json` supplies discriminating
public shaping controls: adjacent wght coordinates in Ubuntu Sans and Cantarell
produce different fresh advances but alias under four-decimal axis formatting.
Mixed runs then reuse the first result in either order. This local correction
addresses that demonstrated alias; it makes no new vanilla parity claim.
`lab/results/construction/diagnostics/shaper-prompt-audit/results.json` additionally demonstrates tracking aliases caused by decimal rounding and by changing size with an absolute tracking length.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/shaper/cache.rs`.

The language-demand controls in
`lab/results/construction/diagnostics/infra-performance-followup/language-control.json` and
buffer setup at `03_infra/src/shaper/shaping.rs:85` / `:249` identify a missing
semantic input. Once buffers honor language, cached runs must distinguish it;
otherwise same-face/text runs can reuse another language's glyphs and advances.
This is an internal cache representation correction, not a public API change.

## Owned contract

- Own ShapeCache/CachedRun and FaceCache/CachedFace. Run cache is local to a document; face cache is per instance/compilation and may be shared by the metrics facade.
- Preserve exact owned structured run keys: text, slot, direction, axis variations identified by their exact f32 bit patterns (no decimal rounding), effective tracking increment as an exact i32 in font units, calculated by the shaping owner before lookup effective ssty level, and optional explicit normalized language code. Keep absent language distinct from every explicit code, including explicit English. Values retain glyphs and i32 width in font units.
- Cache faces by slot including absence. Owned Font bytes keep parsed face borrows valid while CachedFace exists; preserve face index zero and existing encapsulated unsafe. No global cache or new eviction is authorized. The tracking key must identify the actual increment applied to cached glyphs; raw decimal lengths must not be rounded into a key. This correction covers explicit language identity; sufficiency of untested script dimensions remains unproved.

## Verification and boundary

Verify that distinct axis f32 bit patterns cannot alias through decimal formatting,
and identical run inputs still reuse their key. For the discriminating Ubuntu Sans
and Cantarell fixtures, mixed AB and BA document results must match isolated fresh
shaping at the corresponding coordinates. Keep these regression tests in this
consumer's test module. Language-sensitive emission and width regressions belong to the test owner `00_nucleo/prompts/infra/shaper/tests.md`; shared AB/BA and repeated runs must match corresponding fresh language controls. This is an internal cache-identity correction under
ADR-0127, with no public API, default mode, compatibility or pipeline phase change.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.

## Cache-carrier evidence and contracts

### Evidence

Compiler RED context-red.log in the 2026-10-05 implementation receipt identifies
a fixture missing the ratified bidi field. The fixture has no certified origin.
`identidade-fonte-shaped-implementacao-2026-10-05/second-benchmark.json` shows the
larger regression removed, but shaping.rs still calls World::font on carrier
construction, including hits, despite already loading it for CachedRun. The
public TextShaped font was ratified by the owner with manifesto
`f4a17206500baf5952f898135944d55f9fca905285eafc995315ead1121dd409`.

`shaper-primary-reuse-2026-10-10/resolution-calls-before.json.gz` records HEAD/
uncommitted tree/UTC, binary 5e66c09d/argv: local-set/no-fill-1600 calls
primary_candidates 1, 602 times and resolve_candidates 9, 612, among 9, 619
select_pattern lookups. font_selection.rs:16–91 rebuilds primary/class/MATH/common
chains; cache.rs:64–87 retains faces but no family resolution. GDB counts are not
times. Order/selection are semantics; repeating lookup is mechanics. Last-complete-
resolution benefit/safety is refuted by lookup/output/time/RSS controls.

### Current private contracts

Add bidi None only to the uncertified test fixture, preserving language assertions.
CachedRun retains a clone of the actually loaded Font with glyphs/width; hit/miss
carriers use that same font without another World lookup at output. Preserve keys,
document scope/invalidation and FaceCache; no added unsafe/public API. The public
carrier's approval does not enlarge this internal correction. Require fallback/
hit/FontSome controls and CLI revalidation.

Under the pinned shaper-primary-cache nucleus, FaceCache holds one optional,
initially empty primary resolution. Private carrier stores FontList/FontVariant/
math and candidate slots/units-per-em. Preserve slot map/negative caching and
CachedFace byte ownership; no globals/new unsafe/face-map eviction or sharing
across distinct instances/Worlds.

## Useful map-entry maintenance

`lab/results/construction/diagnostics/clippy-locais-2026-10-16/before.json.gz`
retains sources/L0/hashes/HEAD/uncommitted tree/UTC/diffstat for cache.rs:88;
consumer/use inspection preceded deciding and selected gate must fail before
implementation. This is representation/observation, no new upstream intent/parity.

FaceCache::get uses HashMap::entry(slot_idx).or_insert_with, executing World::font/
CachedFace::new only on miss and retaining None as a negative result. Return the
retained face borrow. No eager/duplicate lookup, loss of borrow-owning bytes,
key/lifetime/scope/API change. Continuous ADR-0127: no public/default/phase contract or lint suppression.
L0 first/scoped resealing, exact inventory reduction/no new warnings, selected
Clippy RED→GREEN/unfulfilled_lint_expectations as error; workspace suite/build/
fmt/Rustdoc/lineage under 12 GiB, zero swap, one job. No derive/Entry/module-
placement mirror tests. Any historical bbox control still requires actual
observation in its own scope; other controls preserve fixtures/expectations.


## Owned structured run identity

### Evidence before decision

The frozen source at `03_infra/src/shaper/cache.rs:22` stores run entries in
`HashMap<String, CachedRun>`; `:45–55` formats axes and every input into an owned
string before each lookup. `03_infra/src/shaper/shaping.rs:425–443` supplies the
same seven inputs and appends an effective native-sups decision conditionally.
This is observed private representation, not language semantics or upstream
intent. The approved representation changes these private construction and
lookup mechanics. Any cache contamination or changed observable shaping refutes
preservation; key microcosts alone do not establish an end-to-end benefit.

### Exclusive contract

Replace only the private run-key representation with an owned `ShapeKey` deriving
`PartialEq`, `Eq`, `Hash` and `Debug`. Retain the existing standard HashMap and
its default RandomState; do not add a hasher, cache, intern table, arena, borrowed
lookup, dependency or unsafe. `ShapeCache::key` retains its seven arguments and
returns this key. The key has private fields containing exact UTF-8 text as
String, slot as usize, rtl as bool, an ordered Vec of (raw tag u32, value.to_bits()
u32), tracking_fu as i32, ssty level as Option<u8>, explicit language as
Option<String>, and effective native-sups as Option<bool>, initially None.
Equality and hashing use every complete field, never only a digest. Preserve axis
order, repeated tags, adjacent bits, signed zero and every NaN payload without
normalization or rounding. Language None, Some(""), Some("en") and every other
code remain distinct. Sups None, Some(false), Some(true) remain distinct.

Construct owned text and optional language by exact copies, without intermediate
formatting or serialization. Construct the ordered axis vector directly from
the supplied slice; do not grow serialization buffers. Key construction for
zero, one and four axes must perform no reallocations under the System allocator;
allocations required by the owned fields remain permitted.

Expose only a pub(super) `set_native_sups(bool)` method on the key, recording
Some of the supplied effective decision. Field visibility stays private to this
cache owner. The shaping owner calls this method under the identical previous
suffix condition. No other shaping logic, CachedRun/CachedFace/FaceCache,
document lifetime, font load, carrier, clone, fallback, style, glyph or cluster
behavior changes. The existing tests remain intact.

This approved private representation follows continuous ADR-0127 flow:
no public contract, default behavior, compatibility or pipeline-phase change.
L0 precedes both consumer changes; validate V15/V26 before scoped resealing.
Require independent complete-field identity and fresh/shared shaping controls,
targeted field-loss mutations and existing infra regressions. Verify the
construction reallocation boundary for zero, one and four axes with a disposable
System-allocator gate; do not add a global allocator to productive tests.
Hash/key microcosts do not certify eval/layout/shaping/export or close existing
language-parity debts.

## C06: immutable glyph storage

Evidence at baseline `0a15ccebf861e3cc0ea16d0397d55dc057664556`: `shaper/cache.rs:45` stores CachedRun glyphs as Vec and derives Clone. Use SharedGlyphs so cached clones retain the frozen payload. Preserve full structured key, width, source Font and document lifetime. No global interning or key change. This supersedes storage/clone exclusions only for glyph payloads.


CachedRun.glyphs uses SharedGlyphs; derived Clone shares it. Preserve all other cache behavior. Test helpers may explicitly copy to Vec.

## Raw shaping reuse: evidence before decision

The shaping-input-overlap-lab-20261111 census on clean HEAD 48e5a2570159c6fc325babe883bac7d1c1fbd4b2 compared complete inputs and independently replayed 3,788 calls. In layout.typ, 111/111 emission misses had an earlier equal measurement among 113 measurement calls; math admitted only 2/9. Tracking coincidence applied to raw buffers, not final adjusted carriers. Counts do not prove CPU/memory benefit. See the preserved external census and the correction's measurements for exact states/times/ELFs.

Internal correction authorized by the owner; ADR-0127 continuous flow, no public/default/phase/compatibility change. The pinned raw-shaping-cache nucleus supplies shared invariants; this owner authorizes only its following responsibility.

Own RawShapeInput/RawShapeKey/RawShapeStore alongside existing caches. Construct faces/buffers and store Arc<GlyphBuffer> plus the owning Font. FaceCache::new creates its raw store; raw_store clones only that handle, with_raw_store keeps parsed faces and primary resolution initially empty. Test-only backend observation occurs only at an actual miss.

Keep regression tests first and preserve source/frame/glyph/position/font/fallback semantics. Directed missing-key and bypass-reuse mutants must fail; compare allocation traffic, peak and CPU in the real pipeline including retention/key costs, without an arbitrary target. Compile only in own /dev/shm and preserve all existing checkout changes. Outline's semantic divergence and prior identity debts remain explicit. The retained eval/font-session prototype is a lab measurement base, not integrated by this correction.


## One configured engine face inside raw shaping — RAM experiment (2026-11-12T07:51:11.406374+00:00)

Evidence precedes decision. face-construction/receipt.json binds the existing
ELF, source freeze, inputs, symbol maps and UTC. Full CLI layout has 112 engine
face constructions, all underneath RawShapeCache::shape, out of 222 raw requests;
plain has 4 constructions/6 requests. These counts are not timings. The separate
kernel-provenance.json records 112 synthetic requests with an identified embedded
font: fresh parse/configuration 1.888 ms, clone 0.937 ms, immutable borrow 0.548 ms.
The exact requests are synthetic, not captured CLI arguments; capture/lookup/
retention costs are excluded. Raw glyph info/position equality passed and an
axis-changing variable-font refuter was discriminant. This does not certify
nominal performance or upstream language parity.

Private internal scheduling under ADR-0127 section 2 continuous item 4; no public
signature/field, mode/default, phase/order, dependency, new unsafe or compatibility
change. Supersede the prior no-cache/unchanged-CachedFace exclusions solely for
this one private engine-face slot. The updated pinned nucleus permits this raw
owner-private resource; all caller parsed-face and selection caches remain fresh.
Retain at most one successful configured engine face per RawShapeCache, using
exact immutable Font owner variant/address held alive by the owned CachedFace
and exact ordered axis tag/value bits. Neither slot alone, digest, byte equality,
font family nor pointer without an owner is sufficient. Full raw-result keys,
including slot, remain unchanged. Compare all axes before using a face. On owner
or axes change create a fresh face at index zero, set the complete requested axes
once and replace the last slot only after successful parsing. A failed parse must
return the original None/fallback, never cache a failure or use the old face.
Never mutate a configured retained engine face. Raw hits bypass engine construction
as before. Text, buffer flags, language, script, direction, ordered features and
raw/adjusted result boundaries remain unchanged and per request.

Reuse the existing CachedFace owned-byte/lifetime enclosure, adding only an
optional boxed configured engine face and private constructor/accessor. Use its
existing encapsulated slice conversion; add no unsafe block or widened reference
lifetime. Engine references must borrow the CachedFace; no static reference may
escape. Existing callers of CachedFace::new retain the same behavior, with no
engine construction. Lifetime ends with this compilation, never watch/global.

Before implementation, real crate tests must fail the construction bound for
different texts with the same owner and axes. Verify against independent fresh
stores for owner changes, slot changes, directions, languages, scripts, features,
reordered/repeated/adjacent axes, variable-font default restoration and invalid
font then recovery. Confirm one slot retained on churn and distinguish raw-result
hits from face reuse. Bypass-reuse, missing-owner and missing-axis mutants must
fail their respective gates. Preserve existing raw reuse/cache/font-metrics tests.
L0 first, V15/V26 before sealing, release build and strict lint; source-to-PDF
text/pages/geometry, paired CPU/startup/wall/RSS, allocations and peak determine
utility. No percentage target, no sum of different campaigns, no outline closure.
Laboratory only; this experiment does not integrate previous candidates.


## Laboratory first-query reuse correction — 2026-11-12

Evidence precedes decision: fontdemand-fallback-comparison/freeze.json and summary.csv record unchanged release d7c9024946a3ffde6f5ca0825829eeff29c7b17f60ce26b6ef69670a0e39c7a8 regressing in system and nonminimum-variant workloads. resolution.rs:first_primary and font_selection.rs:first_candidate query World again whenever no complete chain was populated; the selective successful path never populates that chain. phase-counts/receipt.json records 1115 certifications for 113 actual shapes on layout, not a timing attribution.

Keep a DISTINCT latest successful first-readable-query result, scoped exactly like the existing per-compilation face/metrics cache. Key by exact full ordered FontList (optional for metrics) and effective FontVariant. It answers only first-face queries and can never substitute for a complete primary/fallback chain, glyph coverage, bidi splitting or raw-shape certification. Store only key, slot and units-per-em; no additional bytes, global state, cross-World/compilation reuse, negative selection cache or public API. Query/read without holding the metrics selection lock. Do not publish if any earlier selected face failed loading/parsing; shaper conservatively refuses while any existing face-cache failure is present, preserving its old negative-face policy. Existing full-chain hits remain usable and take precedence. New starts empty; metrics Clone shares first-query state only with the same World. Size/color/language/axes retain existing downstream application.

Continuous ADR-0127 internal repair. Require counter RED before implementation; repeat hits do not query catalog, keys and Worlds remain isolated, failed faces keep prior retry policy, and subsequent full-chain resolution equals fresh output. Preserve whole-PDF no-acquisition control, all prior semantic tests and font freshness. Rebuild/lint/paired nominal and forced-full regressions before acceptance; laboratory only.

Exclusive obligation: Own the private optional first-query record and FaceCache field, empty at new/with_raw_store; never copy it with the raw store or into another World.


## Current-compilation positive primary proof reuse (laboratory only)

Evidence precedes decision. controlprimary-segmentation-counts/receipt.json
(2026-11-12T21:39:58.227369Z–21:40:16.650035Z) binds frozen ELF/source and
observes 1115 certificate calls plus 2230 bidi analyses per hundred-paragraph
compilation: 1115 inside the proof and 1115 in consumers. There is one compile
entry per cycle; the same work recurs on unchanged-byte rewrites. The separate
CPU profile attributes about12% inclusive nearest-project-frame samples to the
proof (not exclusive CPU or guaranteed savings). The external allocator records
116764 requests/40837726 requested bytes for this workload; bidi is the largest
sampled request-count origin. Sources: evidence/freeze-controlprimary.json,
controlprimary-cpu-stacks/ and familyshare-allocation-before/. These measurements
are mechanisms, not language requirements; nominal benefit remains unproved.

Internal reversible reuse under ADR-0127 continuous flow: update L0 before tests
and code, reseal, no public/default/phase/compatibility change, no integration.
The pinned primary-selection-proof nucleus fixes the shared exact key, guard,
positive-only result, lifetime and immutable partition obligations. Preserve all
existing raw-cache/face/selection/axes/language/source-identity controls, fresh
fallback after success, late font demand/replacement, PDF/SVG/query/raster, and
strict workspace gates. Measure CPU and allocation traffic separately; retain
negative results, RSS and retained-memory costs. No performance claim from counts.

Exclusive responsibility: own the private exact proof key and positive map in FaceCache, initialized empty in with_raw_store. Value retains Font and Arc of the bidi partition; keep every existing map and raw store unchanged.

Represent unresolved direction absence structurally as Option of the exact four-way discriminant, with an exhaustive inner match. The first complete validation rejected the neutral zero encoding at cache.rs:132 (V16); preserve that failed gate and do not suppress it. This private key representation keeps identical equality partitions; no new source, selection or shaping behavior.


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

Add one optional ShapeSession handle to FaceCache, absent in new/with_raw_store. A private constructor can attach that handle while all faces, first/primary proofs and raw results remain fresh or share only the existing same-compilation raw store. Do not extend ShapeCache key/map lifetime.

Own crate-private attach_products and product_session carrier accessors. They transport only the bounded session; do not copy parsed faces, negative entries or primary resolution across Worlds.

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

## Strict test qualification for adoption

Preserve every existing test assertion, reference comparison and negative case.
Remove redundant cfg/borrows, use a named private snapshot type and array, and
pass configured raw-shaping inputs as the existing named RawShapeInput value
instead of a nine-argument helper. Test FontList HashMap/HashSet contracts through
generic Eq+Hash helpers so immutable regex backend caches do not trigger a
concrete mutable-key lint; keep actual map lookup, deduplication and field coverage.
No lint suppression or productive algorithm change is permitted by this cleanup.
