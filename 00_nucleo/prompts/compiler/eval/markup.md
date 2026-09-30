# L0 — compiler/eval/markup
Hash do Código: 942969a5

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/eval/core.toml sha256:e7642a709c937928333439b2a78cdb3a6dbd6b56d67fcc67728efd2a26796e58

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
00_nucleo/prompts/compiler/eval/rules.md. Do not perform layout.

eval_raw preserves extraction of text/lang/block and passes the RawElem to
intercept_content with current Scopes, EvalContext and Engine. Line synthesis,
matching and recomposition belong to rules, not this syntax adapter.
