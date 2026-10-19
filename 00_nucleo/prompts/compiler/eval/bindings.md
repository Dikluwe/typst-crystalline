# L0 — compiler/eval/bindings
Hash do Código: 05fcaec3

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/builtin-module-identity.toml sha256:2b8888460a1e6008abda0ff71603fe4dace79ff0a2a7aad598f8aabfab3e3776
- 00_nucleo/prompts/_nuclei/eval/call-once.toml sha256:a922bbaa9c3b4c8435708af8e68b4557801f93ba09188768d48c711ba35a0c15

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/bindings/mod.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0127, ADR-0129, ADR-0130.

## Source evidence

The current exclusive consumer defines the following owned boundaries:

- `01_core/src/compiler/eval/bindings/mod.rs:1` — `exclusive lineage header`.

## Contract

This module is a static module and reexport boundary, not a semantic dispatcher.
Declare access, binding, field_access, method_dispatch and value_methods and keep
the existing bindings::<function> call paths and visibility. Each child has its
own exclusive prompt under 00_nucleo/prompts/compiler/eval/bindings/.

Delegate let/destructuring/assignment to binding; mutable-place lookup and
unknown-variable diagnostics to access; field/content/snapshot access and
symbol-deprecation adapters to field_access; mutation and mutable accessors to
method_dispatch; AST adapters and the native function.where helper to
value_methods. Reexport the existing crate-internal content, outline-entry,
element-field and locatability helpers without making them public.
Do not duplicate algorithms, register bindings or absorb font_dict, rules or
show_rule_termination behavior.

## Imports de produção e teste — 2026-10-13

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado. 01_core/src/compiler/eval/bindings/mod.rs:28; 01_core/src/compiler/stdlib/tests/introspection.rs:163 registra a composição e os usos existentes. O inventário Clippy distingue imports sem uso na compilação produtiva; os usos de teste foram conferidos separadamente. Fontes, L0, UTC, status e diffstat estão em `lab/results/construction/diagnostics/imports-ci-2026-10-13/before.json.gz`. Não é medição de paridade ou desempenho.

### Decisão e aceitação

A reexportação de eval_content_method_at neste hub atende somente aos testes e deve existir sob cfg(test). A definição e a rota produtiva direta em field_access permanecem; conservar as demais reexportações.

Correção de composição interna, sem mudança de assinatura pública, defaults, fase ou comportamento da linguagem. Preservar cenários/assertivas, executar a suíte core, build, Clippy e linhagem. Não mascarar avisos com allow/expect.


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

Somente reexportar check_module_field_feature com visibilidade interna a eval, sem absorver lógica.

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

Reexportar eval_field_access_with_target e MutatingCall somente para eval; manter demais rotas internas, sem lógica.

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

## R2 — construção e seleção de Sequence, fluxo contínuo ADR-0127

### Medição anterior à decisão

HEAD b1b486f3d42431f8ea360bc5712637ee1e3d4f6c, working tree não commitado.
lab/results/construction/diagnostics/sequence-constructor-2026-10-17/{before,expanded}.json.gz
e before-sources.json.gz conservam UTC, diffstat/status, binários/fontes e casos
bilaterais no pin bd5ebec285505889e8ff1d4f68804ab147a699f3. Vanilla
foundations/content/mod.rs:724–728 declara children requerido;
foundations/value.rs:644–650 define cast Content/string/symbol/none e
typst-macros/src/elem.rs:573–604 gera o constructor. Tekt
01_core/src/compiler/eval/bindings/field_access/content_methods.rs:358–410
ainda projeta fallback não chamável; o catálogo element_where não tem Sequence
e selector_matching.rs:272–337 não tem sua identidade. rules/set.rs:55–76
só recusa With de transforms e não reconhece set da Sequence. São semântica,
morfologia e diagnósticos de língua, não exigência de copiar storage upstream.
A mensagem de children() marcada pendente em R1 já coincide no baseline atual: o
getter recém-corrigido permite que o owner existente produza o erro correto.
A medição nova retifica essa pendência; não editar código de diagnóstico.

### Responsabilidade exclusiva

Somente reexportar o callback native_sequence do hub field_access com
visibilidade crate-interna; o callback mantém o owner específico. Não expor
função pub externa nem binding global ou duplicar seu algoritmo.

### Aceitação e limites

Esta seção completa a responsabilidade R2 nomeada no L0 de R1. Corrigir
continuamente tabelas/composição/callback interno, sem novos campos/traits/
assinaturas Rust externas, modo padrão, fase ou quebra de compatibilidade.
L0 antes de teste e código, RED funcional, GREEN e corpus bilateral. Não
certificar guardas globais de show, todos os casts/elementos, nem paridade
universal de render. Qualquer refutador residual deve permanecer no relatório
e ganhar owner/L0 antes de intervenção, sem mudar testes para esconder o caso.

Validar build/workspace/doctests em invocação separada, Clippy/Rustdoc estritos,
fmt, V1/V5/V15/V26 e reversa, SARIF diferencial sem erro/warning novo. V19 pode
registrar agrupamento nominal prescrito sem supressão, testando alternativas.
Heavy jobs sequenciais, MemoryMax12GiB/High8GiB/swap0/jobs1, debug0/incremental0
só no ambiente de validação. Resselo após V15/V26 válido e proveniência exata.
