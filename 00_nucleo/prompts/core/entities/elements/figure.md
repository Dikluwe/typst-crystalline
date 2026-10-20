# Prompt L0 — `entities/elements/figure` — `FigureElem`
Hash do Código: c8036146

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/caption-projection.toml sha256:b57c37587932a65caedce0b0df5691463ad336b90126a978ea88a7c369432738

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/figure.rs`
**Architecture:** ADR-0105 model D; locatable Figure.

## Evidence before contract

Historical upstream e0e8ca4d model/figure.rs:203-204 makes caption optional;
296-299 defaults numbering to Some(pattern"1") and permits a pattern/function
of one number. Its count update at 437-445 depends only on numbering, not on
caption. The 2026-08-13 English probe with explicit figure numbering"1", two
uncaptioned figures and captions C1/C2 produced Figure3:C1 and Figure4:C2 in
both vanilla and local HEAD 4f64e4e69. Uncaptioned figures therefore consumed
numbers in both. Crystalline index-based figure_number_at_index, rather than
the payload's caption gate, supplied that language result. Do not describe
caption presence as the language counting rule. The historical missing
numbering default remains separately owned by figure layout.

## Contract

FigureElem derives Debug/Clone/PartialEq/Hash with public body:Content,
caption:Option<Content>,kind:Option<String>; Content::Figure holds Arc.
The legacy four-argument Content::figure(body,caption,kind,numbering) remains
an adapter: the entity has three fields and numbering belongs in the chain.
No restored numbering field is authorized.

plain_text joins nonempty body/caption text with one space, omitting empty
pieces. is_empty is body.is_empty() and absent-or-empty caption.
Both content/text maps recurse symmetrically into body and caption, preserving
kind. Equality/hash include all three fields. element_kind returns Figure;
to_payload returns Figure { kind:kind.clone(),counter_update:Step,
is_counted:caption.is_some() }. Preserve payload-based from_tags/walk consumers;
the flag's mechanical role does not redefine language counting.
infer_kind_from_body still inspects Image/Raw body; native_figure construction
uses the Content constructor.

## Present-caption reflection: evidence and contract

caption-before.json and more-cast-after.json in
`lab/results/construction/diagnostics/tekt-v16-figure-casts-2026-10-17`
record UTC/source/binaries and pin9f2b6e8715237cb086899a42873660fe744622e8.
Construction preserved caption but fields.rs:73 projected only body and the
entity getter defaulted None. Pinned upstream returns FigureCaption for
present caption including empty, null for explicit none, and
`not known at this point` for omission. Option<Content> cannot distinguish
the last two: FG1 remains incomplete, needs a public proposal, and must not
be simulated with Empty or strings.

Override existing Element::get_field only for body/caption. body returns its
original cloned Content. Some(caption) yields Value::Content of
Content::figure_caption(caption.clone(),None,None), preserving origin without
resolving defaults, synthesizing kind/supplement/counter or executing layout.
None caption and unknown fields still return None: never invent presence.
No new trait method/public signature/default/phase follows from this internal
ADR-0127 language-reflection correction.

## Acceptance

L0 precedes real caption-projection RED→GREEN; replay six probes and symbolic
cases, retaining body, Source and absent fields. Revalidate the whole package
under 12GiB/no swap/jobs 1, resealing only owners after valid V15/V26; no release.
Keep symmetric traversal, empty/text behavior, locatability and all three-field
identity. Reflection does not close FG1 or authorize synthesized fields.

## Historical record

`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/elements/figure.md` preserves the original evidence and chronology.


## Approved Figure/Grid occurrence amendment (2026-10-19)

### Evidence before specification

Baseline `731f0ffdf4c7c152f590551e72ff63f7bd85ef85` preserves the measured proposal in `lab/results/v16-corrections-2026-10-19/figure-kind/review.md`, its `agents/contracts/design.md`, semantic partition, independent audit and counter receipts. The fixed oracle is commit `bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree `cac836c07e4dec2a89219370428292393a3216b5`. Constructor string kind loses function identity; constructor inference precedes occurrence styles; separated Grid fields lose source order. Typed identity, lexical presence, field traversal and counter results are language semantics/morphology. The chosen Rust carriers are implementation mechanics. The owner explicitly approved the linked public fields, contracts, phases and compatibility changes on 2026-10-19 under ADR-0127; this amendment materializes that bounded decision.

### Exclusive contract

The approved public declarations are:
```rust
#[derive(Debug, Clone)]
pub enum FigureKind { Element(crate::entities::func::Func), Name(ecow::EcoString) }
#[derive(Debug, Clone, PartialEq, Hash)]
pub enum FigureSupplement {
    Auto, None, Content(crate::entities::content::Content),
    Func { function: crate::entities::func::Func, span: crate::entities::span::Span },
}
pub struct FigureElem {
    pub body: Content,
    pub caption: Option<Content>,
    pub kind: Option<Option<FigureKind>>,
    pub resolved_kind: Option<FigureKind>,
    pub supplement: Option<FigureSupplement>,
    pub resolved_supplement: Option<Option<Content>>,
}
```
FigureKind has manual PartialEq/Eq/Hash: Element and Name remain nominally distinct. Native uses typed native call pointer identity; Format uses its FormatElementKind and typed pointer; NativeWithEngine uses its distinct typed ABI pointer. Dynamic Element uses constructor Arc identity. Synthetic invalid Closure/Plugin/With inputs retain their representation and Arc identity, are never executed for identity, and remain invalid language kind arguments. No name/Debug/hash/pointer-ordering fallback is semantic identity; global Func equality is unchanged.

Outer kind None means omission, Some(None) explicit auto, Some(Some(k)) explicit kind. Resolved fields belong to one prepared occurrence; resolved supplement Some(None) means prepared explicit-none. No numbering field is restored. Preserve plain_text/is_empty and existing body/caption reflection. Project known typed lexical or resolved kind and supplement, but omitted raw fields remain unknown. Kind Element projects Value::Func, Name projects Value::Str; explicit auto projects Value::Auto. Supplement Auto/None/Content/Func projects its exact lexical value, while prepared supplement projects its resolved Content/None. Caption omission versus explicit-none and caption synthesis remain incomplete.

Both maps recurse body, caption, and one selected supplement Content in that order. When resolved_supplement is Some, map only its Content, keeping lexical supplement immutable as origin; otherwise map lexical FigureSupplement::Content. Preserve every presence bit, function/span, resolved kind, and alias metadata; never execute functions or transform lexical/resolved aliases twice. Structural identity includes all six fields. to_payload carries resolved kind (lexical explicit kind only for synthetic unprepared input), resolved supplement, CounterUpdate::step and initial is_counted=true. Effective numbering alone gates counting in the specialized consumer, independent of caption. These clauses supersede old three-field/Option<String>/constructor-inference/caption-count clauses only.

### Acceptance and limits

Preserve all unrelated prior obligations and nucleus pins. This amendment supersedes only conflicting Figure/Grid clauses stated above. Require independent semantic RED→GREEN, exhaustive decision extension builds, existing workspace/build/tests/all-targets/Clippy/Rustdoc/fmt, canonical paths, strict forward/reverse lineage and valid V15/V26 before scoped resealing. No suppression, severity downgrade, weakened assertion or global hash repair. The bounded package does not certify full Figure/Grid parity: standalone HTML rendering, complete caption synthesis, supplement-dependent selector preview, external Dynamic enumeration, Grid footer phase and complex Table supplemental target order remain visible debt.
