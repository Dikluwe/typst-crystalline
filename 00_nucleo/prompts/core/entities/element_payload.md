# Prompt L0 — `entities/element_payload`
Hash do Código: aaa67a95

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/element_payload.rs`

## Base design and evidence

Own closed typed data for indexed elements. The initial Content inventory established Heading level/body, Figure body/caption/kind/numbering and Cite key/supplement/form; extract_payload subsequently filled Heading body_hash through hash_content instead of placeholder zero. Upstream spreads equivalent information across native elements; the local closed enum follows Content/ElementKind without proc-macro/vtable dispatch. Numbering/supplement projection belongs to its designated owners, not speculative payload fields.

The original `desenho-introspection-fixpoint.md` section 2.1 was referenced but not located/produced according to `lab/results/construction/diagnostics/inventario-tipos-introspection-vanilla.md` (2026-04-30). Keep that source-provenance gap visible rather than claiming it was read. The original three-variant/passive-consumer/lightweight-only descriptions are superseded by subsequent payload additions below.

## Base payload schema and mechanics

The following original schema is retained as a base declaration, supplemented by the explicitly specified additions below. It is not permission to delete later variants/fields.

```rust
use crate::entities::counter_update::CounterUpdate;

#[derive(Debug, Clone, PartialEq)]
pub enum ElementPayload {
    Heading {
        depth: u8,

        body_hash: u128,

        counter_update: CounterUpdate,
    },

    Figure {
        kind: Option<String>,

        counter_update: CounterUpdate,

        is_counted: bool,
    },

    Citation {
        key: String,
    },

    Metadata {
        value: Box<Value>,
    },

    State {
        key:  String,
        init: Box<Value>,
    },

    StateUpdate {
        key:    String,
        update: StateUpdate,
    },

    Outline,

    Bibliography {
        entries: Vec<BibEntry>,
    },

    Equation {
        block:          bool,
        counter_update: CounterUpdate,
    },

    Labelled {
        label:         Label,
        resolved_text: Option<String>,
        figure_number: Option<usize>,
    },
}
```

Metadata/State Box<Value> breaks recursive physical size. Preserve derived Debug/Clone/PartialEq, manual Eq marker and manual Debug-format Hash required by Value's f64 limitations; the older derived Eq/Hash excerpt is superseded by the original manual-implementation clause. Clone transports payload values across walk/tag/registry; no I/O/compiler/registry/property map or callback execution belongs here. Hash is convergence evidence, never a collision-free language identity comparator.

Heading depth is clamped 1..6; body_hash comes from hash_content(body) and detects body changes across iterations without cloning the entire body. counter_update retains its effective heading action. Figure kind distinguishes independent counters, with None/Auto using its historically specified consumer fallback `kind.as_deref().unwrap_or("image")`; is_counted is the numbering-present-and-caption-present predicate. Citation retains key for bibliography lookup. Metadata feeds MetadataStore; State carries key/init; StateUpdate carries data and uses its separately owned runtime, superseding the obsolete Set-only/apply_update-stub description.

Outline's base unit payload supports location queries; richer depth/title observations were deferred, not inferred into fields. Bibliography carries complete Vec<BibEntry> for BibStore concatenation and separate number assignment. Equation retains block/action and effective numbering gate; it does not use Figure's caption predicate. The original legacy population predicate is `block && state.value_at("numbering_active:equation", loc) == Some(Bool(true))`, under counter key `"equation"`; the historical dormant-gate window used substitution-with-fallback rather than fabricating a counter. Later effective Equation capture remains with its producer owner. The old dormant-production/fallback schedule is historical. Unit-Step spelling follows the later ratified CounterUpdate positive-level shape.

Labelled is emitted after target recursion because resolved_text/figure_number depend on updated state. It is not produced by pure pre-recursion extract_payload. label populates resolved labels, optional resolved_text retains precomputed reference text or None for unsupported targets, and figure_number is present only for numbered captioned Figure, feeding its number store. Original temporary redundant legacy writes/stub-to-consumer migration are chronology, not a mandate for permanent duplicate mutation.

## Heading and Equation additions

Heading additionally carries numbering_active:bool, captured from the chain during walk, historically `heading.numbering == Bool(true)`. Heading counter advancement alone does not prove numbering; preserve `cannot reference heading without numbering`. HeadingElem::to_payload initializes false, then walk overrides it. Label stays in ElementInfo; do not add speculative Heading label/numbering-pattern fields.

Upstream equation.rs:173–188 synthesizes supplement using target styles/locale before reference consumption. Equation payload carries effective supplement (auto/empty-none/content/function), captured language and sufficient content to reconstruct the public callback argument. Materialized supplement belongs to the Location substore, not payload. Upstream query measures effective defaults/alt: absent/none→null, empty→empty string, set-rule→inherited string. Payload additionally retains alt:Value restricted to Str|None and effective number alignment for query/semantic emission, without adding fields to EquationElem. Update exhaustive matches/hash/equality/tests to include all these additions. The inherited text specifies some additions semantically without exact Rust field names/types; that schema-completeness debt remains, and this editorial change does not invent them from code.

## Native occurrence evidence

At HEAD `2f42d64253547734564513a1159ee6b584c1c4b4`, the unchanged payload enum lacked Strong/Emph; introspect.rs:1278–1363 allocated Location/content only when extract_payload returned Some and 1398–1401 traversed them without own occurrence. The locatable contract requires is_locatable/payload agreement, so a function-preserving selector alone cannot create an occurrence.

Historical uncommitted-tree probes at 2026-09-10T00:18:55.842594+00:00–00:18:59.537177+00:00 and 00:20:17.650961+00:00–00:20:19.215281+00:00 against ratified `a51e02804` accept Strong/Emph query/counter/body filters and reject nonlocatable Text. Upstream strong.rs:21/emph.rs:26 mark Locatable/Tagged; counter.rs:338–357 counts filtered fields. Local introspect.rs:1944–1948 already emits Tag::End content hash and convergence.rs:25–28 includes it, so unit Start payload does not remove body-change evidence. ADR-0069's exceptional post-recursion precedent does not apply to these pure nodes.

## Separately approved NativeElement contract

The original active L0 explicitly records separate owner approval under ADR-0127 for the unit `NativeElement` variant, independently of Selector/ShowSelector approval. Preserve that approval assertion; it is not a blanket approval of the whole observation package.

NativeElement denotes a native occurrence whose complete content is held in the canonical Location store. The bounded new producers are actual Content::Strong and Content::Emph. Promoting an alternative Styled representation requires measured occurrence equivalence and owned extraction/walk specifications for cardinality/order/parent/no double allocation. Show-matcher origin flags alone, simultaneous Strong/Emph flags or Styled wrapping native Strong do not authorize extra Locations. Baked render styles are not semantic origin. Never promote Text/dynamic/future types implicitly.

Do not add ElementKind::NativeElement or per-function kinds. Use ordinary Tag::Start/End, Location and ElementInfo; derive language identity/fields from the stored Content/snapshot, never unit payload/hash/name/fake Heading or Metadata. Causal label remains in ElementInfo. Do not duplicate the body hash or substitute a constant. NativeElement alone supersedes the original one-payload-per-Kind assumption; retain all existing variants/fields/implementations.

## Acceptance and open scope

Require construction/equality/hash/clone for every declared variant and all-field retention. Integration uses ElementInfo(payload,label), the separately ratified boxed Tag payload, extraction/walk/store owners; this prompt is not their additional owner.

Native promotion requires their extraction/locatability/walk/query/count/layout-location contracts before code. Keep callback phase unchanged. Require distinct ordered Strong/Emph occurrences, body match/miss queries, filtered counters/keyed updates and unchanged labels/parents/positions; Tag::End must still detect body changes. Unit clone/hash tests alone do not establish language behavior. Missing canonical identity/fields/final hash, phase changes or a required new public datum refute the unit-payload sufficiency inference and reopen the design. Bare Content hash does not prove stability of chain-only derived fields; capture owner must measure/preserve causal evidence, and duplicating the same hash cannot fix it. No general query/counter parity or closure of the original P1339 scope is claimed.

## Historical record

`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/element_payload.md` preserves the original evidence and chronology.


## Approved Figure/Grid occurrence amendment (2026-10-19)

### Evidence before specification

Baseline `731f0ffdf4c7c152f590551e72ff63f7bd85ef85` preserves the measured proposal in `lab/results/v16-corrections-2026-10-19/figure-kind/review.md`, its `agents/contracts/design.md`, semantic partition, independent audit and counter receipts. The fixed oracle is commit `bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree `cac836c07e4dec2a89219370428292393a3216b5`. Constructor string kind loses function identity; constructor inference precedes occurrence styles; separated Grid fields lose source order. Typed identity, lexical presence, field traversal and counter results are language semantics/morphology. The chosen Rust carriers are implementation mechanics. The owner explicitly approved the linked public fields, contracts, phases and compatibility changes on 2026-10-19 under ADR-0127; this amendment materializes that bounded decision.

### Exclusive contract

ElementPayload::Figure changes kind to Option<FigureKind> and adds supplement: Option<Option<Content>>. Preserve counter_update, is_counted, caption_text and every other payload variant. Kind/supplement carry resolved occurrence values; None is permitted only for documented synthetic unprepared data. is_counted is effective numbering present, independent of caption, superseding caption conjunction and string-kind fallback clauses. No productive `figure:<kind>` user-string alias substitutes for typed identity. Clone/equality/hash retain new fields; migrate fixtures to explicit Name versus actual Element intent.

### Acceptance and limits

Preserve all unrelated prior obligations and nucleus pins. This amendment supersedes only conflicting Figure/Grid clauses stated above. Require independent semantic RED→GREEN, exhaustive decision extension builds, existing workspace/build/tests/all-targets/Clippy/Rustdoc/fmt, canonical paths, strict forward/reverse lineage and valid V15/V26 before scoped resealing. No suppression, severity downgrade, weakened assertion or global hash repair. The bounded package does not certify full Figure/Grid parity: standalone HTML rendering, complete caption synthesis, supplement-dependent selector preview, external Dynamic enumeration, Grid footer phase and complex Table supplemental target order remain visible debt.

### Typed Figure identity in convergence hashing

Integrated review at `element_payload.rs:277–284` observes the inherited Hash
implementation formats Debug, while `entities/func.rs:392–395` deliberately
prints every Func as `<function>`. Distinct Element(table)/Element(raw) kinds
then have the same hash input even though their reflected Figure kind and typed
counter keys differ. Figure payload hashing must additionally hash its typed
FigureKind using that owner's exact nominal Hash implementation. Preserve all
other payload fields and inherited hashing behavior. This is a bounded internal
hash correction within the already approved typed payload contract; no global
Content hash rewrite or equality change follows. A same-Location Figure-kind
convergence RED must become GREEN, and equal pointer aliases must keep equal
hashes. Hash remains probabilistic convergence evidence, never language equality.
