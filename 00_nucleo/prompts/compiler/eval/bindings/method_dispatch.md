# L0 — compiler/eval/bindings/method_dispatch
Hash do Código: b7694a6d

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/eval/call-once.toml sha256:a922bbaa9c3b4c8435708af8e68b4557801f93ba09188768d48c711ba35a0c15

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/bindings/method_dispatch.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0127, ADR-0129, ADR-0130.

## Source evidence

The current exclusive consumer defines the following owned boundaries:

- `01_core/src/compiler/eval/bindings/method_dispatch.rs:29` — `fn is_accessor_method`.
- `01_core/src/compiler/eval/bindings/method_dispatch.rs:35` — `fn expect_positional`.
- `01_core/src/compiler/eval/bindings/method_dispatch.rs:52` — `fn finish_args`.
- `01_core/src/compiler/eval/bindings/method_dispatch.rs:74` — `fn has_readonly_method`.
- `01_core/src/compiler/eval/bindings/method_dispatch.rs:91` — `fn call_method_access`.
- `01_core/src/compiler/eval/bindings/method_dispatch.rs:202` — `fn is_mutating_method`.
- `01_core/src/compiler/eval/bindings/method_dispatch.rs:208` — `fn is_dict_mutating_method`.
- `01_core/src/compiler/eval/bindings/method_dispatch.rs:222` — `fn try_eval_mutating_method`.
- `01_core/src/compiler/eval/bindings/method_dispatch.rs:269` — `fn call_method_mut`.

## Contract

Own mutable-method classification, mutation and mutable accessor routing.
Array mutation names are push/pop/insert/remove; dictionary mutation names are
insert/remove. Mutable accessors are first/last/at where the receiver supports
them. Preserve readonly classification so temporary mutation fails with the
existing diagnostic rather than becoming a mutation of a detached copy.

Evaluate arguments, resolve the mutable place, then perform the mutation in the
existing order. The mutation interceptor returns None for an unrecognized
nonmutating call. Preserve array/dictionary bounds, empty-collection, missing
argument, default and unexpected-argument diagnostics and span priorities.
first/last/at return the actual mutable slot, never a cloned substitute.

Argument helpers consume via remove_positional(0)/remove_named and finish_args;
they keep Args views, occurrences, names, individual origins and aggregate span
coherent. Never retain an occurrence carrier after modifying its views directly.


## Avaliação única em chamadas — 2026-10-17

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado.
00_nucleo/diagnosticos/call-once-2026-10-17/before.json.gz registra UTC,
status, diff HEAD --stat, fontes/L0/config e hashes. Os 2124 hashes da etapa
anterior foram verificados exatamente antes do baseline. before-probe.json.gz
e before-math-probe.json.gz conservam binários/argv/fixtures/saídas bilaterais.
Pin vanilla 9f2b6e8715237cb086899a42873660fe744622e8, SHA-256 do binário
7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea.
calc.abs/array.len/str.from-unicode medem receiver=3/args=2 contra 1/1;
module.measure/layout/within/before/after/func/has/body chegam a receiver=4;
fallback mutante mede args=3 contra 1. Métodos ausentes executam panic de
argumento antes do erro causal. math.mat soma duas vezes; array prévio (1,)
fica (1,1,1) contra (1,1). Array vazio mascara o defeito com truncate.
call.rs:70–118 perde receiver; collection_methods.rs:43 avalia args antes de
reconhecer operação; contextual/content/selector reavaliam AST no fallthrough;
method_dispatch.rs:247 descarta args; invoke.rs:66–120 compensa por nome.
O L0 call_dispatch.md já exige receiver/args uma vez. pinned-source.json.gz
fixa call.rs/methods.rs upstream: interesse é ordem/efeito/diagnóstico da
linguagem, não copiar Vm, storage ou igualdade Rust.

A primeira fixture declarava palavra reservada or como função: é inválida e
está arquivada em invalid-fixture-attempt, sem crédito para efeitos de chamada.
Ela revelou panic 101 em import inválido; manter aberto para correção causal
própria no owner de parsing/import. O baseline válido substitui só a fixture.

### Decisão e responsabilidade exclusiva

Adicionar enum restrito a eval MutatingCall { Handled(Value), Fallback { target: Value, args: Args } }. try_eval_mutating_method passa a SourceResult<MutatingCall> mantendo os mesmos parâmetros. Preservar args antes de access e writeback real; sucesso mutante mapeia Handled; fallback de Module/Func/Type/Symbol/Content clona o valor do local uma única vez e transporta os Args. Não reexecutar access ou args. Isto supersede a aceitação histórica de dupla avaliação no comentário P717; a nova medição prova efeito observável e exige correção.

Consumir o núcleo eval/call-once.toml pinado. Mudança mecânica interna e
correção de paridade ADR-0127 em fluxo contínuo. Assinaturas pub fn/pub use e
structs públicos não mudam; os parâmetros novos/retornos/helpers acima têm
visibilidade crate/eval, sem novo trait público, campo público, default ou
fase. Esta cláusula supersede somente restrições históricas conflitantes de
assinatura interna e compensação por nome; manter extensões Tekt existentes.

### Aceitação e limites

Testes funcionais co-localizados antes do código: efeitos de receiver/args,
fallback mutante, erro antes de args, where/with/selector/coleções preservados
e matriz sem truncar efeito legítimo. Confirmar RED funcional e GREEN.
Comparar casos bilaterais válidos e controles; hits e misses de todos os
métodos de coleção, erros, spans/occurrences e complementos de Value. Não
mascarar dívida residual de diagnóstico como igualdade. Testar dispatch
Unhandled sem perder receiver ou executar callback de args; contar avaliações
pelo efeito observado, não por bytes de PDF, Rust Eq ou microbenchmark.

Ownership/V26 antes de resselo só dos owners do lote. Build, suíte workspace,
Clippy all-targets -D warnings/-D unfulfilled_lint_expectations, Rustdoc estrito,
fmt, V1/V5/V15/V26 e reverse, SARIF diferencial sem erro novo. V19 uniforme
continua informativo, com matriz de alternativas e relação de comportamento;
sem supressão nem mudança de configuração. Comandos pesados sequenciais com
12 GiB/high10GiB/swap0/jobs1. Não publicar/commit/merge/release nem declarar o
objetivo global fechado por esta etapa.


### Forma plana do complemento — medição V20

candidate-4/validation-architecture.log e differential-review.json medem nota
V20 nova por captura target/other @ (grupo de variantes) com profundidade 3.
A captura é dispensável: guardar o local mutável em variável target antes do
match e referenciá-lo no complemento, ou referenciar o callee original cujo
payload foi ignorado. Manter os grupos explícitos planos, os mesmos membros e
retornos, sem supressão de V20/V19 e sem duplicar classificação. As matrizes de
39 alternativas, os efeitos e o ponteiro original devem permanecer iguais.
