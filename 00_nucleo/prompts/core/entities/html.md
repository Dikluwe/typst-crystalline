# Prompt L0 — HTML content entities
Hash do Código: e98920cb

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/compiler-feature-gates.toml sha256:09b32666fb1b1fa7ba8f2ef7c979c60cbc64cc515b8d9c46818bec19d7471fb1

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/html.rs`

## Evidence before contract

Historical typst-html/src/lib.rs:47-93 defined HtmlElem with required tag,
attrs, internal properties and optional body. The earlier Content/exporter
could not preserve tag+attrs+body. Initial typed-constructor measurement found
12 constructors converting 76 global attributes to strings before HtmlElem;
this introduced no entity fields. A later binary probe refuted the hypothesis
that the existing entity preserved all observables: html.div() yielded
`elem(tag: "div", body: none)` upstream, html.elem("div") omitted body, while
both local forms omitted it. Option<Box<Content>> could not represent all
three states. The owner approved explicit tri-state replacement on 2026-08-25.

## Current HTML element contract

HtmlElem has public tag:EcoString, attrs:Option<HtmlAttrs>, body:HtmlBody.
HtmlBody is a closed enum Unset, None, Content(Box<Content>). HtmlAttrs
preserves ordered, already-converted name/value pairs. Names and values retain
semantic meaning; escaping belongs only to export. Tag is validated.

html.elem with omitted body produces Unset; typed normal constructors produce
None when omitted; supplied content produces Content. repr omits only Unset,
prints body:none for None and actual content otherwise. Export/plain text/
walkers treat Unset/None as empty bodies and descend only into Content.
Never encode these states through tag, empty content or hidden booleans.
Preserve omitted attrs versus an empty dictionary wherever upstream reflection
distinguishes them; the initial fine-grained fields measurement remains a
required prerequisite for acceptance, not an assumed result.

Pure L1, no I/O/L3/lab dependency. Content remains closed with exhaustive
map/repr/plain-text/reflection arms. html.elem roundtrip preserves tag/attrs/body;
export validates/escapes names/values without baking rendering style.
Rust equality/storage is mechanics; repr, present fields and serialized DOM
are observables. Typed signatures/casts belong to the HTML stdlib owner.
The 12 approved normal tags do not authorize void/raw booleans; declarative
classification must be specified before such constructors as br.

## Compiler feature boundary

The historical source html.rs:12-42 and 88-92 colocated Feature/Features and
tests with HTML. Upstream library lib.rs:272-305 has A11yExtras beside Html,
so the set is not HTML-owned. Its canonical owner is
`00_nucleo/prompts/core/entities/compiler_features.md`.
Keep explicit entities::html::{Feature,Features} compatibility re-exports as
long as the old public path requires them; no duplicated state/implementation/
tests. HTML binding/export remain governed by the pinned nucleus. This entity
does not activate feature, target or format.

## Ratified HTML frame carrier

Upstream FrameElem { body:Content } is distinct from HtmlElem and receives
SVG only after HTML-target layout. It has no tag, attrs or tri-state body.
The human approved public HtmlFrameElem { pub body:Content } on 2026-09-16.
Clone/Debug/equality preserve body; construction computes no layout/SVG/target/
styles and performs no I/O. No HtmlBody reuse, attrs or derived bytes.
Content incorporation is separately owned. Approval is bounded to that carrier.

## Deferred scope

Internal CSS, parent/location, expanded DOM, MathML and additional typed
attributes remain outside the initial entity scope; the ratified frame carrier
supersedes only its specific former exclusion. No broad closure follows from
construction or the historical probes.

## Historical record

`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/html.md` preserves the original evidence and chronology.
