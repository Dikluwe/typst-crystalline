# L0 — compiler/eval/call_dispatch/call
Hash do Código: 9372377f

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/eval/call-once.toml sha256:a922bbaa9c3b4c8435708af8e68b4557801f93ba09188768d48c711ba35a0c15

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/eval/call_dispatch/call.rs`. No HEAD `d5c563dfab39f789e8e2f72c9a9364f5a761e3ca` e árvore limpa, `01_core/src/compiler/eval/call_dispatch.rs:1305-2125` contém o comportamento extraído. O SHA-256, horário e estado inicial estão em `lab/results/construction/diagnostics/atomizacao-call-dispatch-2026-09-30-before.json.gz`.

## Contrato

Resolver e interceptar chamadas de funções e métodos na prioridade existente; delegar métodos de instância sobre Value a `instance_methods.rs` após mutação e antes de coleções; preservar avaliação única de receiver e argumentos. Quando o método de instância não interceptar um acesso a campo, entregar o valor do receiver já avaliado ao despacho de coleções, sem reavaliar a expressão. A unidade delega a sequência de métodos de Selector para `selector_intercepts.rs` e as chamadas contextuais de medida/layout para `contextual_calls.rs`, métodos de coleção/`with` para `collection_methods.rs` e métodos de Location/Content/OutlineEntry para `content_methods.rs` e a invocação final do callee para `invoke.rs`, cujos Prompts L0 são proprietários individuais. Preservar assinaturas públicas, diagnósticos, prioridades de intercepção, rastros e semântica de argumentos especificados em `00_nucleo/prompts/compiler/eval/call_dispatch.md`. A extração permanece em L1 com despacho estático, sem I/O ou estado global mutável.

## Aceitação

Preservar a suíte L1, os testes de call_dispatch e `crystalline-lint .` com ownership e hashes corretos. Não inferir ganho de desempenho.


## Avaliação única em chamadas — 2026-10-17

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado.
lab/results/construction/diagnostics/call-once-2026-10-17/before.json.gz registra UTC,
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

Manter caches locais Option<Value> para receiver e callee e Option<Args> para argumentos do fallback mutante. Helpers internos resolve_receiver(access, cache, scopes, ctx, engine) e resolve_callee(call, cache_receiver, cache_callee, scopes, ctx, engine) devolvem referências aos valores guardados. Reusar eval_field_access_with_target para lookup completo, incluindo features/spans/estilos/deprecações. Preservar prioridade where/with, seletores, mutação, instância, coleção, contexto, conteúdo, erro de callee e invoke. Transportar cache por todos os fallbacks, sem duplicar lógica de lookup; erro de método ocorre antes de argumentos comuns. Where/with continua no ramo inicial único. No final nomear field_callee_error antes de resolver campo; invoke recebe callee e Args já obtidos. Testes co-localizados usam World em memória e APIs vigentes antes do código.

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


Matrizes de aceitação do complemento V19: call_once_unhandled_keeps_every_value_and_does_not_evaluate_arguments e call_once_callable_classification_covers_all_value_variants_before_arguments constroem 39 discriminantes distintos de Value (36 via linguagem e Path/Location/LocatedContent pelos construtores puros RootedPath/Locator/IntrospectedContent existentes). A primeira testa retorno Unhandled para nome desconhecido, igualdade do payload transferido, ponteiro do Vec original e callback de args que panic se executado; a segunda testa 37 rejeições antes de args e os dois caminhos Func/Type com panic real do argumento. São testes do transporte/classificação interna; não certificam que toda categoria tem a mesma capacidade de cast upstream, nem paridade geral de todos os schemas.
