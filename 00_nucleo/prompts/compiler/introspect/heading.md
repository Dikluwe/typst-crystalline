# L0 — introspect/heading
Hash do Código: f934ac2c

```text
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/introspection/content-snapshot.toml sha256:5a0270231de70be1212dbd17298cce34b7161b527d74b4b589e3f4c69d35ce24
- 00_nucleo/prompts/_nuclei/layout/element-form-b.toml sha256:6dbf8faa56960845c60734f5e047685ec5b3a6f14c0ce3a48d333b8982d0baa5
```

## Ownership and evidence

Layer: L1. Sole consumer: `01_core/src/compiler/introspect/heading.rs`.
The current snapshot_fields body at `:22` and TOC projections at `:93`,
`:122` implement the helpers below. The parent calls snapshot_fields while
the causal chain/label is available (`01_core/src/compiler/introspect.rs:2016`);
the Heading branch applies inclusion gates after walking the body.
`01_core/src/compiler/stdlib/structural/heading.rs:127` and
`01_core/src/compiler/eval/rules.rs:1620` delimit the existing producers.
Implemented snapshots must not be described as an awaiting-materialization
proposal, nor as evidence of support for every parser-accepted heading option.

## Walk-branch refinement measurement

At the uncommitted state captured in
`00_nucleo/diagnosticos/atomizacao-introspect-heading-label-2026-09-30-before.json.gz`,
`01_core/src/compiler/introspect.rs:2083-2220` owns the Heading walk branch,
while this consumer already owns snapshot and TOC computations. The `introspect`
test filter passed 263 tests before movement. This is evidence of separable
behavior, not a new timing or bilateral vanilla measurement.

## Complete modeled snapshot

Keep the pure descendant helper receiving HeadingElem, StyleChain and optional
causal Label and returning the ordered public field map:

| Field | Captured value |
| --- | --- |
| level | The modeled heading level as an integer. |
| depth | The modeled level when HEADING_SET_DEPTH is present; otherwise 1. |
| offset | 0, without inventing an unmodeled custom offset. |
| numbering | Exact string only when heading.numbering is Bool(true) and heading.numbering.pattern is Str; otherwise None. |
| supplement | Text Content from the canonical heading language supplement owner at the causal language. |
| outlined | The stored boolean. |
| bookmarked | Explicit boolean, or Auto when absent; not is_bookmarked(). |
| hanging-indent | Auto for the currently modeled producer. |
| body | The frozen semantic body Content, not plain text. |
| label | Causal Label, appended last only when present. |

A stale ancestral pattern does not override a disabled gate.
For a synthetic active gate without a recoverable language string pattern,
capture numbering None; do not derive a pattern from a rendered prefix or
claim parity of that synthetic input. Function/internal non-string patterns
are not fabricated into strings.

Use only `compiler::lang::heading_supplement_for_lang`, owned by
`00_nucleo/prompts/compiler/lang/heading_supplement.md`. Missing language
uses English without mutating StyleChain or activating hyphenation.
Do not load translations at runtime, infer an environmental region or maintain
a competing table. French is Section through the canonical owner.
The pinned snapshot nucleus owns authoritative presence and causal immutability.

## TOC projections

Compute sequential auto-toc labels and location-aware heading numbers without
mutating the introspector. When numbering is active, the auto-label text and
outline prefix are the hierarchical number with a trailing dot; when inactive,
auto-label text is empty and the optional outline number is absent.
Do not add a heading supplement to the outline number.
Return the supplied frozen body and level with the auto label and optional
number. The parent walk allocates the real Heading Location and emits its real Start/End.
The Heading descendant owns its complete walk branch: increments the threaded
auto-label counter, captures numbering from the causal StyleChain, freezes the
body at the Heading Location, walks descendants once through the parent walk,
then emits derived auto-label, TOC and bookmark tags in that order at the same
Location. The descendant independently gates outlined and bookmarked records.
It calls the parent tag-store population helper before each derived Start and
keeps each paired zero-hash End. The parent match remains exhaustive and
delegates this branch by a static free-function call.
Body references' supplemented text belongs to
`00_nucleo/prompts/compiler/introspect/labelled.md`.

## Acceptance and modeled boundary

Cover default/markup/explicit levels, transported string numbering and set none,
causal language/label, snapshot reuse after style changes, auto-label sequencing
and numbered/un-numbered TOC projection. Preserve counter/TOC behavior.

A complete snapshot means complete fields of the modeled producer, not universal
heading parity. Custom supplement/Func/none, numbering Func, custom offset or
hanging indent, untransported region and all set-rule heading fields are not
made supported by this helper. Constructor outlined/bookmarked and explicit
level already exist; do not incorrectly classify those as future features.
Precedence of constructor numbering:none under an ancestral set and show-set
language effects across phases require separate evidence/owner decisions.
No new constructor, callback API, public field or phase is authorized here.
