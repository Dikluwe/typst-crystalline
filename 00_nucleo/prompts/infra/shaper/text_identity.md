# L0 — infra/shaper/text_identity
Hash do Código: 437903bb

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/fonts/bidi-visual-identity.toml sha256:b234a166cefeba71803da8bcc3cb5e3daf4d7bed8313f5883bcc7bf12bf95f15
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:395a968c2fe49429b03b369cd5047df513132bfef6511ec21af466670ec2e147

**Layer:** L3
**Exclusive consumer:** `03_infra/src/shaper/text_identity.rs`

## Authored identity evidence



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

## Exclusive per-occurrence protection

Own private protect_authored_identity(Vec<FrameItem>)->Vec<FrameItem> and
configure_authored_ignorables_buffer(&mut UnicodeBuffer,&str). Caller invokes
protection only outside an existing replacement. For TextShaped containing author
U+00AD wrap that exact item in TextReplacement with cloned occurrence text and
one original child. Other items stay identical. U+00AD is normative identity,
no fixture/family/GID/width exception; no font resolution/shaping/measurement/glyph
filtering/coordinate transform/wrapper traversal in this unit.



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


The existing renamed helper protect_authored_identity preserves SHY rule. For bidi
sort unique cluster offsets plus text.len() into ranges; out-of-bounds/non-UTF8
ranges certify nothing. Certify only a glyph-covered valid range containing at
least one pinned bidi control and one codepoint outside that set. No glyph/valid
range or controls only outside emitted ranges means no envelope. Never generalize
to ZWJ/ZWNJ/variation selectors. Certificate wraps whole TextShaped with exact text
clone and original sole child, identical Font/bidi/position/clusters/advances.
No global ToUnicode rewrite/relayout/reshaping; caller prevents nesting.

## Acceptance

Mandatory SHA-versioned Noto causal tests in shaper/tests prove one logical text,
unchanged glyphs/advances/positions/tracking and no nesting. Source→PDF retains
per-occurrence SHY/control identity, lexical spaces/literal hyphens in top/local
Compact/Verbose; repeated same letters cannot acquire another occurrence's controls.
Cache hit/miss and existing-wrapper scope stay correct. Missing-font/narrow SHY
selection remains uncertified. Require exclusive ownership/pinned invariants,
real RED→GREEN and ink/text/bbox controls, no GID-count or raw-extraction substitute.
