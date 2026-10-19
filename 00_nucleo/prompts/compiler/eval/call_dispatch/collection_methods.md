# L0 — compiler/eval/call_dispatch/collection_methods
Hash do Código: 011cc150

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/builtin-module-identity.toml sha256:2b8888460a1e6008abda0ff71603fe4dace79ff0a2a7aad598f8aabfab3e3776
- 00_nucleo/prompts/_nuclei/eval/call-once.toml sha256:a922bbaa9c3b4c8435708af8e68b4557801f93ba09188768d48c711ba35a0c15

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/eval/call_dispatch/collection_methods.rs`. Comportamento original em `01_core/src/compiler/eval/call_dispatch/call.rs:265-315` no HEAD `51a027f99`, árvore limpa, 2026-10-01 00:44:23 UTC. A classificação é extração interna, não nova prova bilateral de paridade ou desempenho.

## Contrato

Own collection method interception and Func.with partial application in the existing order. Receber o valor do receiver avaliado pelo despacho anterior para acesso a campo e consumi-lo uma vez, sem reavaliar a expressão; preservar argument spans, trace handling e fallthrough. Preservar contrato público, fase do pipeline, defaults, compatibilidade e pureza L1. Evidência e proveniência da duplicação em `lab/results/construction/diagnostics/arrays-dicts-item6-2026-10-09.md` e `instance_methods.md`.

## Aceitação

Suíte L1, `cargo build` e `crystalline-lint .` passam; a extração não implica ganho de desempenho.


## Continuação causal do gate — chamadas e set rules

Medição anterior à decisão: candidate-1/validation-green.log registra quatro
controles aprovados e falha real do teste de argumentos: std.html(panic(...))
executa panic antes do gate. before-set-probe.json.gz registra UTC, hashes,
fontes e saídas bilaterais no mesmo HEAD/árvore não commitada: std.html,
format.html, std.asset e format.bundle chamados avaliam argumentos antes do
gate; set std.html/std.format.html/alias apaga o erro do target e ignora os
argumentos. Variável/field ausente em set também é aceito indevidamente;
condition false conserva atalhamento. Html ativo em set por alias ignora panic
no Tekt e o executa no pin. São ordem causal, diagnóstico e efeito da linguagem.
collection_methods.rs:36 avalia argumentos antes de autenticar módulo; set.rs:
44 usa ok() na resolução e :326 autentica HTML pela sintaxe literal.

Invocar check_module_field_feature no receiver já resolvido/cacheado, antes de coletar/avaliar argumentos. Não reavaliar o receiver, mudar dispatch de coleção nem alterar os demais fallbacks.

L0 precede esta continuação; correção interna/paridade em fluxo ADR-0127.
O núcleo builtin-module-identity continua pinado e sem alteração. O contrato
público e os campos externos permanecem iguais; nenhum fallback novo ou
supressão. Os controles e a matriz dos oito perfis permanecem; acrescentar
verificação bilateral dos efeitos e de set com/sem feature. A suite completa
regressa após o source freeze novo, sem reaproveitar gates de fonte alterada.


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

Substituir rota eager por try_collection_method: retirar uma vez o receiver do cache, verificar gate, construir spans e passar avaliação adiada de args via closure estática FnOnce ao dispatcher stdlib. Handled conserva trace Args map/filter; Unhandled devolve o Value por ownership ao cache. Remover o ramo With tardio redundante: call.rs já trata exatamente essa forma primeiro. Não duplicar catálogo nem decidir por nome público da função.

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
