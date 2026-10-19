# L0 — infra/shaper/tests
Hash do Código: 94ec9918

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/fonts/shaped-face-identity.toml sha256:06ae067763e4c77876f7d999903f2e5e4144df34eb799eb62f2c52a31d8a030b
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:395a968c2fe49429b03b369cd5047df513132bfef6511ec21af466670ec2e147
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:a4a76bdb3fa5069232468af724106217bcd1dc85231c9d2cf54b4da69e241425
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:96c5b3ef9be3d171d3ca4c57f30fe826068c568fc17437f83b2b9b5d7372aeff



**Layer:** L3
**Exclusive consumer:** `03_infra/src/shaper/tests.rs`

## Current evidence

- `03_infra/src/shaper/tests.rs:38` — `p1404_fallback_interner_e_esparso_e_idempotente`.
- `03_infra/src/shaper/tests.rs:48` — `p1404_controles_de_formato_nao_pedem_glifo`.
- `03_infra/src/shaper/tests.rs:56` — `MockWorld`.
- `03_infra/src/shaper/tests.rs:61` — `library`.
- `03_infra/src/shaper/tests.rs:65` — `book`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/shaper/tests.rs`.

The language-demand diagnostic
`lab/results/construction/diagnostics/infra-performance-followup/language-control.json` and
pinned buffer setup at
`lab/typst-original/crates/typst-layout/src/inline/shaping.rs:980` motivate
language-sensitive regression controls. The versioned Noto Sans fixture contains
ru/sr substitutions with unequal advances, allowing width and glyph paths to be
tested without optional system fonts or fixture generation.

## Owned contract

- Own only the unit suite, mocks and real-font fixtures under shaper::tests. Preserve existing names and descendant helper access; tests do not define a new API.
- Cover text passthrough, all page layers/semantic groups, bidi/UTF-8 offsets/multiline paragraphs, script/font fallback, sparse local slot identity, variations without contamination, tracking, math TextItem/ssty and LTR/RTL reconciliation.
- Compare the existing fresh-metrics reconciliation entry point with the internal borrowed-metrics path on the same two-item document and World; their resulting x positions must agree.
- Require explicit-language measurement and emission to match configured same-face controls, preserving original text/clusters. Cover Russian/Serbian localized substitutions with the versioned Noto Sans fixture; independently assert distinct advances, fresh results, shared AB/BA/repeated runs and absent-language behavior. These are internal render/cache controls, not a claim of full upstream language parity.
- Distinguish dense-local candidate identity from global slot number; cardinality alone is insufficient. Cover scoring with sparse global slots and decoy metadata at the colliding local indices, and script boundaries within one face (including leading Common and inherited characters). Verify isolated versus mixed AB/BA shaping for tracking precision and absolute tracking at different sizes; retain same-effective-increment reuse controls. Mandatory new regressions use embedded/repository fixtures rather than silently skipping missing system fonts. Keep missing-font and no-panic controls and compare language text/positions where relevant rather than claim a new upstream parity measurement.

## Verification and boundary

Run `cargo test -p typst-infra --lib shaper::tests -- --test-threads=1` when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.

## Textual replacement and authored identity controls

initial.json/before/resultados.json/red.log in identidade-hifen-discricionario-
2026-09-28 retain text/bbox/SHY-loss evidence and approved layout_types carrier.
Exhaustive fixtures accommodate the variant: visual projections descend children,
text extractors consume replacement text once. Discretionary expectations become
SHY while literal U+002D stays; no relaxed bbox/break choice. Mandatory Noto and
missing-font controls preserve envelope and show replacement text neither changes
glyph/advance nor contaminates cache.



### Evidence

`lab/results/construction/diagnostics/shy-autoral-2026-09-28/{initial.json,before/resultados.json}`
retains state/binaries/commands and 30 bilateral probes. At 300pt,
repre<U+00AD>sentation extracts two SHYs in Tekt versus one in pinned vanilla;
spaces control extracts six versus one. shaping.rs:313–325 carries rustybuzz SHY;
export/fonts.rs:318–328 maps its substring to a space GID; builder/fonts.rs:523–535
keeps one identity per GID. Prior stream associates CID 0001 with 00AD then reuses
it for later space. This is observable text, not byte equality. Missing that
association or pre-shaper duplication would refute the causal hypothesis; no
historical intent inferred.

Mandatory Noto tests compare authored SHY glyphs/advances/positions/tracking
with raw shaper. Existing envelope never nests and absent font preserves Text;
glyph count cannot substitute for identity.

## Authored invisible-control shaping

### Evidence

`lab/results/construction/diagnostics/shy-autoral-2026-09-28/first-round/after/`
refutes the initial materialization: trailing/ligature preserve codepoints but
expand ActualText bbox 3.12pt/lose space in ordinary extraction; tracking adds a
placeholder-derived interval. first-round/final-receipt.json retains state/failures.
Rustybuzz0.20.1 src/lib.rs:32–35 documents default Default_Ignorable→zero-advance
space placeholder and REMOVE_DEFAULT_IGNORABLES removal after shaping. Shared
f+SHY+fi cluster can assign empty hex to a space GID, blocking later mapping.
Changed visible glyphs/ligatures under the documented flag or contaminated non-SHY/
non-bidi runs refute the fix.

`controles-bidi-pdf-2026-10-06/before.json` retains HEAD/tree/code/L0/old binary.
rtl_literal_embedding-cmap-before/rtl_forced_mark-cmap-before probes map CID 0001
to U+202C/U+200F, contaminating later spaces/word separation; block-align-2026-10-06/
verification.json.gz retains bilateral extraction/fonts/commands. Historical vanilla
`9f2b6e8715237cb086899a42873660fe744622e8` inline/shaping.rs:992–997 removes
placeholders; export/fonts.rs:307–328 retains controls per GID substring and
builder/fonts.rs:523–535 keeps first map. Observable text/geometry, not bytes/
upstream algorithm, matters. Persisting collision after placeholder removal/visual-
map filtering refutes the cause; no historical intent inferred.

### Contract and limits

Private configure_authored_ignorables_buffer applies REMOVE_DEFAULT_IGNORABLES
before shaping both measurement/emission subruns with author U+00AD or explicitly
pinned bidi control, preserving prior flags/full logical input. Do not remove
characters or filter GIDs/zero advances; runs without SHY/bidi keep behavior.
It removes placeholders, not visible selected-break glyphs; tracking uses remaining
clusters without an interval for removed placeholders. Cache uses full author text,
policy deterministic from text, no public signature/phase/fallback/OpenType-feature/
font change. Entirely invisible SHY and narrow break selection remain outside SHY
certification; later explicit bidi-only empty-result rule is separate.

SHY retains per-occurrence ActualText. Isolated bidi controls gain no replacement
envelope or copied space identity; only the mixed-cluster certificate below may
add one. Keep letters/spaces separated/Unicode clusters/ink controls. Extractors
may invent direction markers, so raw text alone proves no author identity. Internal
ADR-0127 adds no public field/signature/phase/flag/default/unsafe, consumes pinned
bidi-visual-identity and text-replacement without sharing productive ownership.

## Fallback, cached-font and bidi-empty controls

Declared family must win when covering text; Libertinus Serif must cover Hebrew
missing in primary before global Noto Sans Hebrew. Use correcao-rtl-restante-
2026-09-29 bilateral PDF corpus for face/text/position, not isolated names alone.

shaping.rs originally reloaded World::font for miss/hit despite loaded shaping
font. Cache owner now retains Font: repeated italic request with regular fallback
counts regular-source calls and checks Some carrier without additional output-
construction loads after cold face/shape. This is an upper bound, not a ban on
further valid reductions.

controles-bidi-pdf-2026-10-06/first-verification.json.gz preserves residual RLM
when empty items meant failure. Mandatory versioned-font/cache miss+hit isolated
bidi control emits no residual Text/fabricated envelope/advance, retaining missing-
font fallback and adjacent letters. Internal correction adds no public/phase/unsafe;
this owner owns tests only. Every explicit pinned bidi control bypasses coverage;
U+200B/ZWJ/ZWNJ/variation selectors are not generalized. Source→PDF probes certify
whole context/ink. Ratified bidi inventory/1ae7513e adaptation uses None for
uncertified fixture origin/read `..`, no phase/API addition.

## Certified mixed-cluster tests



`lab/results/construction/diagnostics/bidi-identidade-ocorrencia-2026-10-06/before.json`
retains HEAD/tree/code/L0/binaries/rustybuzz probe. SHA-versioned DejaVu with
REMOVE_DEFAULT_IGNORABLES gives LRM+ABC/PDI+ABC A cluster 0, B 4, C 5; 0..4 contains
control+A. fonts.rs:320–347 removes visual controls, text_identity.rs:15 certifies
only SHY; earlier controles-bidi-pdf verification.json.gz observes absent U+200E/
U+2069 in three occurrences with preserved ink. Historical vanilla 9f2b6e871
inline/shaping.rs:1052–1099 supplies full cluster ranges to export. Unicode/text
semantics/morphology, not GID/bytes/algorithm, determines acceptance. Missing
identity/duplication on another same letter/changed ink refutes the envelope fix;
no inferred intent.


Causal mandatory Noto tests put LRM/PDI with a letter in a real cluster range,
checking exact per-occurrence text plus one original child; ordinary repeated
letters gain no controls. Miss/hit preserve glyphs/advances/position/Font/origin.
Existing replacement gets no nested child envelope, SHY controls stay intact.
Repeated/wrapped source→PDF controls observe author identity and ink.

## Internal fixture maintenance

fixtures-ativos-ci-2026-10-13/before.json.gz retains sources/owners/diagnostics/
HEAD/dirty tree/UTC/status/diffstat; actual previous suite confirmed the separate
missing math test. tests.rs:113 computes unused fonts.len, :520 builds an unused
mutable FontBook before the actual World. Remove only these pure unused locals,
keeping World/source registration/fixture guards/assertions. No productive/lint
suppression change; pre-existing optional-host-font limitations remain open.
L0 first/affected-owner seals/shell+infra suites/all-target compile/fmt/lineage/
Clippy inventory; counts certify no parity.

ci-simplificacoes-2026-10-14/before.json.gz retains source/owner/spans/HEAD/tree/
UTC/diffstat at :1358, 1359, 1494, red.log selected errors/types checked under 1.92.0.
Isolated tracking styles a/b use std::slice::from_ref, keeping AB/BA/repeated/size/
tracking controls, necessary multi-style array clones, mandatory fixture and every
glyph/position assertion. Internal continuous ADR-0127 adds no public/default/
phase/unsafe/dependency/suppression; selected RED→GREEN/full suite/build/all-targets/
Rustdoc/fmt/lineage/full inventory/scoped seals, no syntax-mirror tests.

field-reassign-2026-10-15/before.json.gz/initial-diagnostics.json retains source/
spans/prior L0/HEAD/tree/UTC/diffstat for :174, 291, 454, 627, 1126. Five test/helper
TextStyle literals initialize contiguous font/size/weight with same families/sizes/
received weight/clones/later tracking mutation/system-font guards/documents/all
assertions. Remaining fields ..Type::default(), mut removed only without mutation.
No new type/builder/API/suppression/default semantics, L0 first/scoped seals,
selected RED→GREEN/all-target Clippy/workspace suite/build/fmt/Rustdoc/lineage,
unchanged names/input/expectations/tolerances; no initializer-mirror tests.

## Approved paint-root transport

### Evidence and scope

Manifesto `d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d`
was approved for the native-tiling package; receipt
`lab/history/construction/records/00_nucleo/materialization/tiling-transporte-2026-10-14/approval-receipt.json`.
Its bilateral Parent/Solid evidence cannot be replaced by structural inspection.
`lab/results/construction/diagnostics/paint-root-2026-10-14/before/results.json.gz`
retains sources/dirty tree/HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, UTC/
diffstat/binary hashes for containers/Place/Rotate with Solid/Gradient Parent.
This unit's before-source/syntax-nodes anchors: [569]. layout_types.rs:531–539
lacks technical-root distinction; helpers.rs:253–269 reconstructs Group. Box groups
background/body; Block stays flat except clipping. Reference-frame preservation is
language, envelope representation mechanics; prior general Rotate/Box geometry
divergences remain open.

### Contract and acceptance

Adapt Group construction/patterns to approved paint_root. New technical/legacy
fixture groups are false; reconstruction preserves received value, never inferring
root from dimensions/clipping/transforms. Keep other fields/drawing/resources.
Metadata migration alone does not certify exporter pattern integration or replace
Shape.fill/stroke with FramePaint. Independently authorized prepared-transport/
native-target/parity slices (historical T1c/T2/T3) stay incomplete until integrated
verification, no production/parity claim from a successful build. L0 first,
RED→GREEN/scoped resealing; preserve source→PDF/SVG text/geometry and revalidate
suite/build/doc/check/fmt/lineage.
