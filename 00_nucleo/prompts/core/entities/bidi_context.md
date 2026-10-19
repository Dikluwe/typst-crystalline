# L0 — Immutable logical bidi context
Hash do Código: 1c529395

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:a4a76bdb3fa5069232468af724106217bcd1dc85231c9d2cf54b4da69e241425

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/bidi_context.rs`
**Status:** human-ratified; consumer implemented. Immutable receipt:
`lab/history/construction/records/00_nucleo/materialization/bidi-contexto-aprovacao-2026-10-05.json`.

## Evidence before logical context

`lab/results/construction/diagnostics/bidi-contexto-2026-10-05/initial.json.gz`
preserves HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b,complete tree,UTC,source/
binaries and 13 CLI inputs against 9f2b6e8715237cb086899a42873660fe744622e8.
rtl_auto_mixed placed ABC at x124.020510 locally versus 152.949220 upstream;
ltr_numeric placed Arabic left and reversed123/456 versus upstream numbers left.
layout_bidi.rs:73-82 chose first strong character per line;227-240 used any
RTL style as default level; shaper/bidi.rs:34 resolved each string afresh.
Private paragraph context did not reach these consumers. Upstream inline/mod.rs:
189,232 shares direction; collect.rs:156-178 embeds text whose direction differs.
probe.json.gz records recipe/rustc/unicode-bidi0.3.18 hashes/independent probe:
inside ABC123456789مرحبا, the 456789 line is LTR, while isolated 456789 with the
same RTL base produces 789,space,456. This refutes Dir-only transport and isolated
line/word resolution, not a candidate's full production geometry.

## Public pure data contract

ParagraphBidiSegment is Text{text:EcoString,dir:Dir},
Space{origin:TextSeparatorOrigin},or Linebreak.
ParagraphBidiInput and BidiSpan have private fields and implement Clone/Debug.
Input exposes new(base_dir:Dir,segments:Vec<ParagraphBidiSegment>)->Option<Self>,
base_dir(&self)->Dir and segments(&self)->&[ParagraphBidiSegment].
Span exposes new(input:Arc<ParagraphBidiInput>,segment:usize,range:Range<usize>)
->Option<Self>,input(&self)->&Arc<ParagraphBidiInput>,segment(&self)->usize,
range(&self)->Range<usize>,subspan(&self,relative:Range<usize>)->Option<Self>.
Use canonical Dir/EcoString/TextSeparatorOrigin and pure Arc/Range. No setters,
Hash/address/global identity requirement; clone preserves Arc/index/range.

Input requires LTR/RTL,nonempty segments and at least one nonempty Text;
vertical/no substantive text rejects. Do not normalize/reorder/trim Text.
Space is actual U+0020 with supplied origin, never inferred from geometry.
Linebreak is explicit source boundary; soft wrap adds no segment.
BidiSpan requires existing segment,start≤end,in-bounds valid UTF-8 boundaries.
Space spans index its U+0020 bytes and retain origin; Linebreak accepts no text
span. Invalid input returns None without panic/mutation/approximation.
subspan uses checked relative-offset addition,contained range and fresh UTF-8
validation, retaining input/segment. Repeated equal words remain separate
occurrences; never find subparts by string equality.

## Producer/consumer boundaries

Leaf text must exactly equal the referenced slice; splitting derives matching
subspans. Physical reordering/translation/clone/envelopes/rollback preserve
logical origin. Merge to one span only for contiguous ranges in the same
input/segment; otherwise retain separate fragments/spans. Lifetime is RAM,
not World/filesystem, with no retention across compilations/watch in this slice.
No coordinate identity,virtual controls in Content/copyable text,direction
overwrite or paragraph context in TextStyle/font keys.

This unit is pure data/validation, no Unicode algorithm/fonts/regions/I/O/
clock/mutable globals/dynamic dispatch/unsafe. Bidi remains L3 in existing
layout→bidi→shaping order, without a new L1 external whitelist entry.
Paragraph owner certifies/prepares complete segments/base/effective directions/
real spaces/explicit breaks before emission. It supplies deterministic original
UTF-8 ranges, keeps mutable cursors separate from immutable data and rejects
unmappable paths. Ending outer scope never erases accepted tail context.

L3 builds auxiliary Unicode input with virtual embeddings and original-byte
maps, never placing controls in source/glyphs/ToUnicode. Resolve full paragraph;
query line ranges with line-ending rules and fragment shaping ranges without
redetecting fragmented-string base. L3 owns auxiliary representation/library.
Any bidi cache is compilation-local and reuses immutable input, not x/y origin,
self-reference or unsafe. Owned strings and borrowed views may use normal
separate-variable lifetimes. No new FontMetrics method is authorized; a measured
public-trait need requires its own L0/gate.

## Acceptance and explicit exclusions

Require UTF-8/bounds/overflow,clone/subspan and repeated-word distinct-range
RED→GREEN; compare full context versus isolated 456789. Bidi Some requires
whole-path certification; out-of-scope paths use None without invented origins
or style-based matching. Objects/math/links/composite regions/hyphenation/
smallcaps/nonexact text transforms and the whole language are not certified.

Source→PDF probes against ratified pin cover opposite bases/first reversed run,
explicit overrides,numbers/neutrals,soft wrap/linebreak,whole scopes/parbreak,
restoration/checkpoints and prior controls including attached suffixes.
Check text/codepoints,word/baseline-grouped boxes,space origin,no auxiliary
controls in extraction and last-line spans. Distinguish space failures from
bidi order; total bbox equality is insufficient. Never use Rust Eq/PDF bytes
as language acceptance.

Each producer/pattern changes under its own L0 first. None in uncertified
producers preserves scope. Audit frame caches: invalidate/avoid reuse if
exterior context is not proved equal; equal geometry/text alone cannot certify
origin. Debug/release builds,appropriate L1/L3 tests and clean V15/V26 precede
restricted reseal. Approval excludes publication,upstream resync/new defaults.

## Historical record

`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/bidi_context.md` preserves the original evidence and chronology.
