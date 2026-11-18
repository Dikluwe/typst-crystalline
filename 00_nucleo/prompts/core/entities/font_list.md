# L0 — core/entities/font_list
Hash do Código: f275bbe5

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/fonts/immutable-family-list.toml sha256:f33ff33fa820aa3914d7de26e3abe47e4c453971161347e6c71ab028ebbac468



**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/font_list.rs`
**References:** ADR-0053, ADR-0033, ADR-0037, ADR-0107, DEBT-52.

## Measured basis and laboratory decision — 2026-11-13

shared-font-list-validation/prepared.json binds source/binary freezes, HEAD,
exact dirty checkout list, UTC and audited hashes. In frozen text-products,
font_list.rs:137-177 has a private Vec<FontFamily>, derived Clone and only
immutable list access; layout_types.rs:172-267 puts FontList in cloned TextStyle,
and paragraph_paint.rs:172 clones retained FrameItems again on a paint hit.
The existing test named font_list_clone_o1_via_ecow only compares values and does
not establish O(1) list cloning. The corrected release profile has44/135 direct
TextStyle::clone samples in100paragraphs/outline and94 FrameItem::clone samples
in outline. It identifies a hypothesis about repeated list allocation, not the
fraction of time that sharing will save. Baseline main edit is13.129/21.875ms
versus vanilla3.697/4.241ms in the recorded campaign.

Pinned vanilla64c5e8c0 text/mod.rs:1073 also uses Vec<FontFamily>; its exact blob
was compared with git show and hashed in prepared.json. Sharing here is an
intentional mechanical choice for this compiler's clone-heavy ownership path,
not a claim that vanilla uses Arc or that vector shape is language parity.
ADR-0029 permits immutable RAM sharing; ADR-0107 leaves representation free.
The hypothesis is refuted as a useful performance candidate if the paired
release campaign shows no material gain or unacceptable startup/memory costs.

Only in the laboratory, store the nonempty family slice in a private Arc.
Clones of FontList, TextStyle and FrameItem reuse the same immutable family
and nested metadata storage, with ownership surviving destruction of the
original value. Constructors own the supplied values; independently built equal
lists still compare/hash by ordered contents, never by storage identity.
No mutation API, global interner, unsafe code, semantic cache, new public
signature or pipeline change. Field access remains immutable. This does not
change source/font invalidation because all family values remain exact and
immutable and independent current values retain content-based equality.
User authorizes laboratory work without another approval. No integration.

## Contract

FontList is the prioritized family list for `#set text(font: ...)`, including
dictionary regex keys. Preserve this interface:

```rust
pub enum Covers {}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum FontNamePattern { Literal(EcoString), Regex(Regex) }
impl FontNamePattern {
    pub fn is_match(&self, name: &str) -> bool;
    pub fn as_str(&self) -> Option<&str>;
}
pub struct FontFamily {
    pub name: FontNamePattern,
    pub variants: Vec<EcoString>,
    pub variant: Option<EcoString>,
    pub weight: Option<EcoString>,
    pub style: Option<EcoString>,
    pub covers: Option<Covers>,
}
impl FontFamily {
    pub fn new(name: EcoString) -> Self;
    pub fn new_literal(name: EcoString, variants: Vec<EcoString>) -> Self;
    pub fn new_regex(regex: Regex, variants: Vec<EcoString>) -> Self;
    pub fn new_named(name: FontNamePattern, variants: Vec<EcoString>, variant: Option<EcoString>, weight: Option<EcoString>, style: Option<EcoString>) -> Self;
}
pub struct FontList(Arc<[FontFamily]>); // private representation; public methods unchanged
impl FontList {
    pub fn new(families: Vec<FontFamily>) -> Option<Self>;
    pub fn single(name: EcoString) -> Self;
    pub fn as_slice(&self) -> &[FontFamily];
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;
}
```

Literal family constructors lowercase names. new supplies empty variants and
covers None; new_literal/new_regex preserve supplied variants. Existing new_named
transports name/pattern, ordered variants, variant, weight and style unchanged;
it does not add normalization or interpretation. These fields/signature already
exist in the measured code; documenting them is not a public API addition. Regex uses the
domain Regex::is_match and Arc<regex::Regex> backing with O(1) clone; its pattern
is preserved. Covers is uninhabited, so covers is always None. FontList is
nonempty by construction: new(empty) returns None; single creates one literal
with empty variants; is_empty always returns false. Preserve order/first-match
fallback priority. Literal matching is exact, not a prefix match.

FontBook::select_pattern resolves case-insensitive literals or regex against
the catalog. The layout text owner decodes custom-chain `"text.font"` into
FontList. Style::Font remains Clone+PartialEq; FontNamePattern keeps
Clone+PartialEq+Eq+Hash. Parsing/validation belongs exclusively to
`00_nucleo/prompts/core/compiler/eval/font_dict.md`.

## Acceptance

FontFamily::new("Arial").name.as_str() is Some("arial"). Regex("Name.*") matches
"Name Bold"; Literal("Name") does not. FontList::single("X").len() is 1;
new(vec![]) is None. Attempting to match a Covers variant cannot compile.

## Deferred scope

Concrete covers (LatinInCjk or regex coverage), interpretation of variant-aware
selection, O(1) literal lookup in regex dictionaries and IntoIterator are outside
this owner. Family metadata is transported; this experiment does not alter the
current consumers that interpret it. The historical partial-parity claim does not
certify current upstream parity.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/font_list.md`. Historical observations are not a new parity certification.


## Laboratory validation

Tests precede representation change. Real nonempty clones must fail storage
sharing assertions while the old Vec implementation remains; positive runtime
failures are mechanical performance evidence, not language parity. Cover cloned
list/style/frame owners, destruction/replacement independence, nonempty input,
family order, regex/literal distinction, all named metadata, separate equal
owners and HashMap equality/hash behavior. Do not add a public identity accessor.
Language acceptance uses existing compiler/PDF/SVG/raster/font refresh and warm
revision controls, not Rust equality or byte identity. Run full core/infra suites,
release build, lint, strict Clippy/docs, format and all-target checks; record
exact source/binary provenance and one paired campaign against frozen base and
pinned vanilla, plus independent clone/allocation work evidence and profile.

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
