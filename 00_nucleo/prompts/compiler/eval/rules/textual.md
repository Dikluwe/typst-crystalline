# L0 — compiler/eval/rules/textual
Hash do Código: 1d755b5f

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/eval/core.toml sha256:e7642a709c937928333439b2a78cdb3a6dbd6b56d67fcc67728efd2a26796e58

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/rules/textual.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Measurement before decision

At the measured working tree, `01_core/src/compiler/eval/rules.rs:3932-4435`
contains `TextualSnapshot`, text-node interception, grouping, rule slicing,
space normalization and their focal tests. `compiler/eval/mod.rs:1371,1408,
1416,1470,1550` calls the three entrypoints. The existing `p1494` test filter
passes 17/17 before movement; this is structural baseline, not a new bilateral
parity measurement. Source state and command are recorded in
`00_nucleo/diagnosticos/atomizacao-textual-2026-09-30.md`.

## Decision and ownership

Move that behavior to this descendant module without changing algorithms,
order, diagnostics, spans or language morphology. The parent `rules.rs` retains
set/show dispatch and one-node show application, and exposes this child to the
eval coordinator. No public signature, default, pipeline phase or compatibility
changes. The native dispatcher remains static and exhaustive; the child does
not import render or infra and does not create dynamic dispatch.

`TextualSnapshot` captures the causal rule/style/guard state. The two callable
entrypoints are `intercept_text_element` and `realize_textual_parts`; the
behavioral helpers and focused tests live in the two descendant owners. Calls
from `eval/mod.rs` refer to this coordinator; no copied implementation remains
in `rules.rs`. The shared one-node show application and selector semantics remain
owned respectively by `compiler/eval/rules.md` and
`compiler/eval/selector_matching.md`.

## Refinement measurement

At the working tree captured in
`00_nucleo/diagnosticos/atomizacao-textual-algoritmos-2026-09-30-before.json.gz`,
this consumer contains the coordinator, the space algorithm at lines 335-463
and the matching algorithm at lines 96-333. The `p1494` filter passed 17 tests
before this second movement. The distinct inputs and helpers support two
behavioral owners; no CPU reduction follows from that measurement.

## Behavioral delegation

The coordinator retains `TextualSnapshot`, `intercept_text_element` and
`realize_textual_parts` with their existing signatures and phase. It invokes
space normalization only when `normalize_spaces` is true, then returns early
when show is disabled, then invokes the matching owner. Snapshot grouping
retains pointer identity for rules and equality of collapsed style and guard
state. Element-selector interception still uses the one-node show application
owned by `compiler/eval/rules.md`; textual selectors remain deferred.

`compiler/eval/rules/textual/space.md` exclusively owns collapse of adjacent
spaces, newline origin, explicit linebreak consumption and the pinned Unicode
spacing predicate. `compiler/eval/rules/textual/matching.md` exclusively owns
realized group traversal, selector precedence, matched-slice transformation,
while its descendant `compiler/eval/rules/textual/matching/projection.md`
exclusively owns preservation of unmatched original content. These algorithms stay in the
eval layer as descendant modules. This coordinator must not duplicate either
algorithm or introduce dynamic dispatch.

## Acceptance

The coordinator tests snapshot group identity across shared rules, distinct
rule allocations, collapsed style changes and guard changes. The existing focal
tests remain green, including literal/regex slicing, Unicode space handling,
lexical snapshots and guard behavior. Build and
`crystalline-lint .` have no new violations. Language content, diagnostics,
ordering and public contract are unchanged. No performance gain is claimed
without separate measurement. Header ownership remains 1:1.


## Realização com avaliação única

Com defer_show_rules, capturar grupos por snapshot lexical e fronteira de estilo antes de executar receitas textuais. Capturar também grupos cujo texto só surge da realização de elementos; não decidir match pela forma pré-show. Em realização, reutilizar matching/splice vigente, sem executar programa comum novamente.


## Realização com avaliação única

Fachada interna realize_captured_text realiza elementos já resolvidos no grupo, conserva a origem causal do grupo e delega matching/splice ao owner existente. Sem perda de origens por substituição textual.


## Realização com avaliação única

Normalização de espaço vigente é reutilizada internamente após realização de receitas; não duplicar regra Unicode de inserção.


## Realização com avaliação única

Snapshot de grupo textual conserva watermark contextual, incluindo conteúdo lexical capturado não inserido na árvore visual inicial.
