# L0 — infra/shaper/cache
Hash do Código: a434296e

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
