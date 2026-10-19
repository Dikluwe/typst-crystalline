# L0 — infra/shaper/cache
Hash do Código: f8a8d92b

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/fonts/shaped-face-identity.toml sha256:06ae067763e4c77876f7d999903f2e5e4144df34eb799eb62f2c52a31d8a030b
- 00_nucleo/prompts/_nuclei/fonts/shaper-primary-cache.toml sha256:bb9850c42a48ec519f605159d6aea76b305b0ccf6643ef6a4f8137720c76c898

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
- Preserve exact textual run keys: text, slot, direction, axis variations identified by their exact f32 bit patterns (no decimal rounding), effective tracking increment as an exact i32 in font units, calculated by the shaping owner before lookup effective ssty level, and optional explicit normalized language code. Keep absent language distinct from every explicit code, including explicit English. Values retain glyphs and i32 width in font units.
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
