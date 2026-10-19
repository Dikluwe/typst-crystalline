# L0 — svg/glyphs
Hash do Código: 7790632c

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/svg/glyphs.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/export/svg-glyph-font-context.toml sha256:fe41319c4bf27b69b635c5666e46f5b8b9b99160323926fb83889b0f09b97417

## Consumer evidence

`03_infra/src/export/svg/glyphs.rs:20` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Font context and glyph definitions

Own GlyphFontRequest/SvgGlyphFontContext, immutable request normalization/lookup,
GlyphDefs, outline extraction/cache/path builder, direct/shaped glyph painting and
the existing HTML textual writer. Follow the pinned glyph-font-context nucleus;
its shared resolution invariants are not redeclared by the façade.

Use the resolved FontKey's supplied bytes for glyph_id, verifying the key exists
in the collection. Deduplicate definitions by font identity and GID. Apply size /
units_per_em, position and Y inversion once; preserve shaped advances/offsets,
fill/alpha and supplied styles without reshaping or moving the layout cursor.
Keep OutlineBuilder methods as trait methods and current visibility.
Absent/ambiguous mapping, invalid face, zero units_per_em or missing outline remains
Unknown with the existing fallback, not a fabricated glyph. Fontless wrappers do
not synthesize text/outlines or probe incidental/system fonts or network.

## Verification

Exercise reordered fonts, same-GID distinct fonts, mathematical assemblies,
fill/alpha, transforms and missing/invalid/fontless contexts.


## Immutable coordinate and effective outline inputs

### Evidence

`avisos-clippy-tekt-2026-10-15/before.json.gz` retains sources/owners/HEAD/tree/UTC/
diffstat; selected-red.json records glyphs.rs:319 mutable y without reassignment.
This is equivalent Rust expression maintenance, no new parity. Preserve y value/
offsets/advances/Y inversion/fill/definition identity, make only y immutable.
Continuous ADR-0127: no public/default/phase, suppression or syntax-mirror tests;
selected GREEN, existing suite/build/all-target Clippy/fmt/doc/lineage/scoped seals.

`too-many-arguments-2026-10-15/before.json.gz`/classification.json retains full
source/prior L0/signatures/hashes/UTC/status/diffstat, HEAD
06848cdcb5450f2c555626bf116bc7acd82bd82b, dirty tree captured
2026-10-15T16:39:18.873477+00:00, reconfirmed spans/contracts. render_text_shaped
:286 has nine arguments (threshold seven), historical C066; outline-only leaf
ignores _text, HTML textual route is chosen earlier with separate writer. This
is private structure, no functional-defect/new parity claim.
clippy-aridade-interna-2026-10-16/before.json.gz retains checked sources/callers/L0/
spans/28-warning inventory/hashes/HEAD/tree/UTC/diffstat. It materializes the six
named corrections, superseding earlier pending status only for them.

### Current private correction and acceptance

Remove only _text from pub(super) outline helper and its dispatcher argument,
keeping eight effective face/outline/scale/definition inputs and unchanged body.
Then only this leaf may use
`#[expect(clippy::too_many_arguments, reason="C066: Leaf SVG usa oito inputs efetivos de contorno, face, escala e definições; texto HTML segue writer separado")]`.
Never remove text from FrameItem/public API/HTML writer; preserve upem==0/fallback/
GIDs/defs/scale/offsets/advances. Residual expectation depends on dead-argument
removal, no unconditional suppression. Item/lint only, no module/impl/crate/global
threshold, other lint or unfulfilled suppression; remove/review if no longer needed.

Continuous ADR-0127 no external public/default/phase/dependency/semantics change.
New carriers/constructors private; generic bans are superseded only for the two
historical conditional C047/C066 residuals after _text removal. Other five correction
arities remain visible. L0 first/caller-L0 before code/scoped seals. Public/external
compatibility needs fresh concrete approval. Real selected RED before code/GREEN
after, preserve all functional fixtures/expectations/tolerances and values/calls/
PNG-SVG-PDF/preference/diagnostic/resource/MCID behavior, no global mutation or
carrier-mirror tests. Full workspace suite/build/all-target Clippy/unfulfilled
expectations/Rustdoc/fmt/strict+reverse lineage under12GiB/zero swap/one job.
