# Prompt L0 — `entities/elements/figure` — `FigureElem`
Hash do Código: 4c05df80

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
