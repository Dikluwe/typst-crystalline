# L0 — compiler/eval/closures
Hash do Código: f6f4523f

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/closures.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0127, ADR-0129, ADR-0130.

## Source evidence

The current exclusive consumer defines the following owned boundaries:

- `01_core/src/compiler/eval/closures.rs:40` — `fn apply_closure`.
- `01_core/src/compiler/eval/closures.rs:186` — `fn eval_closure_expr`.

Prior source readings supporting these clauses (snapshot provenance is
retained in the assigned history diagnostic, not asserted as a fresh measurement):

- `lab/typst-original/crates/typst-eval/src/call.rs:693-715,728-735`.
- `typst-eval/src/call.rs:638`.

## Contract

Own closure construction and application. Capture an eager immutable lexical
snapshot. Preserve parameter patterns, positional/named/default/spread handling,
definition-time evaluation of defaults, recursive self binding and the existing
maximum call depth of 80. Use isolated local scopes and a reborrowed Engine with
the causal route, styles and sink. Consume Return and reject residual Break or
Continue with their owned forbidden diagnostic.

Missing required parameters fail as missing arguments, not None; extra arguments
are errors, with leftover positional errors preceding leftover named errors.
Named-default parameters are keyword-only and do not consume positional values.
Recursive self is bound before parameters, so a same-named parameter shadows it.
Pattern parameters consume one positional value. Consume named and
positional arguments through remove_named/remove_positional(0), keeping Args
occurrences and views coherent. Preserve all residual sink occurrences, their
names, ordering, duplicates, individual spans and aggregate span. The current
sink order is residual named occurrences first, then residual positional ones,
preserving order within each group. Nonterminal-sink limitations are not silently
changed.

Keep the causal parameter/body origin in the private Func origin metadata;
known origins survive aliases, With and factories and are never replaced merely
because a later expression yields the same value. Do not execute closures to
compare identities or recover a source by searching equal values.

Under cfg(p1339_observation), record the real function/body SyntaxNode and actual
result/flow along the existing callback ledger path. Do not manufacture a Func
for a body, re-evaluate a body for instrumentation, or expose ledger behavior in
normal builds. Preserve recursion, style and context semantics in both builds.


## Context — identidade das regras locais, 2026-10-18

Medição anterior: green-text1.log e seu recibo em
00_nucleo/diagnosticos/context-paridade-2026-10-18, HEAD b7e7c83223e5daa67bbdf124c36b5b7f93200731,
working tree do candidato1. O controle local-inside-context regrediu de L para R.
context_stabilization.rs:257 cria EvalContext fresco; rules/show.rs:455–456
aloca RuleId começando do mesmo zero da regra herdada. textual.rs:86–103 compõe
por identidade, portanto o delta local colide com a regra herdada. before.json.gz
registra L no vanilla bd5ebec285505889e8ff1d4f68804ab147a699f3. Inferência causal
refutável pelo controle local e por callbacks/produtos de regras distintas.

Antes de executar o corpo de uma closure Capturer::Context, reservar no
next_rule_id privado todos os RuleIds do snapshot eval.context.recipe presente
no Engine causal. Usar max(id.saturating_add(1)), tal como o owner apply já faz
para receitas; não executar regra nem antecipar corpo, não instalar regras
globais na avaliação, não alterar assinatura pública. Closures Function e
snapshot ausente mantêm o comportamento vigente. Fluxo contínuo ADR-0127:
correção de colisão de identidade interna, dentro da fase existente.
