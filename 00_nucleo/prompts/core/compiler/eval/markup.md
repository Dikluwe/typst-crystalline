# L0 — compiler/eval/markup
Hash do Código: 6aade97a

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/eval/core.toml sha256:9a193bdaa08a4eb5b41e9d4124388d0de24bd66776fedf04ef9273c1390462ec
- 00_nucleo/prompts/_nuclei/set/declared-properties.toml sha256:b4b40ae19e8c22e62a22af00459e257fa4a6c9d8c518d101cd0798cbf4f97744

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/markup.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0127, ADR-0129, ADR-0130.

## Source evidence

The current exclusive consumer defines the following owned boundaries:

- `01_core/src/compiler/eval/markup.rs:30` — `fn eval_body_with_delta`.
- `01_core/src/compiler/eval/markup.rs:52` — `fn eval_strong`.
- `01_core/src/compiler/eval/markup.rs:71` — `fn eval_emph`.
- `01_core/src/compiler/eval/markup.rs:90` — `fn eval_heading`.
- `01_core/src/compiler/eval/markup.rs:116` — `fn eval_raw`.
- `01_core/src/compiler/eval/markup.rs:132` — `fn eval_link`.
- `01_core/src/compiler/eval/markup.rs:141` — `fn eval_list_item`.
- `01_core/src/compiler/eval/markup.rs:151` — `fn eval_enum_item`.

## Contract

Own evaluation of strong, emph, heading, raw, link, list and enum items.
Preserve AST body, destination, level, marker, semantic style and causal spans.
Reborrow Engine and push local styles without leaking them into the caller;
route resulting content through the applicable show interception owned by
00_nucleo/prompts/core/compiler/eval/rules.md. Do not perform layout.

eval_raw preserves extraction of text/lang/block and passes the RawElem to
intercept_content with current Scopes, EvalContext and Engine. Line synthesis,
matching and recomposition belong to rules, not this syntax adapter.

## Causal span for recursive markup headings

markup-span-red.json and span-red.log record Tekt rejection without an origin for
show heading: it => [= Z], while historical pin 9f2b6e871 anchors the error at = Z.
At the measured markup.rs:112–113, eval_heading propagates interception without
heading.span. When propagating those failures preserve known spans and assign
the real heading span only to detached diagnostics. Never infer origin from the
message, body or approximate position; do not change other adapters, matching
or recursion limits. These historical observations retain their original
provenance and do not establish current-pin parity.

## Declared set properties

Approved on 2026-11-24 (“Aprovo A–F”): `00_nucleo/diagnostics/set-contract-package-2026-11-24.pt-BR.md`.
A markup heading's level is its depth plus the declared heading.offset at evaluation.

## List containers (2026-11-24)

Markup items stay loose values with their own identity. A show rule that matches a
list, enum or term item is applied when it is created. A number is kept as written.

Approved on 2026-11-24: `00_nucleo/diagnostics/lists-package-2026-11-24.pt-BR.md`.
