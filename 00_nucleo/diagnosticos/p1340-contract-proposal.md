# P1340 — proposta de contrato do recorte de estabilização

Estado: proposta para revisão independente; não é selo nem veredito. Regime:
executado sem atestação de isolamento. Autor `/root/p1340_contract`, sem leitura
do patch candidato e sem escrita de produção/L0. Skill aplicada:
`/home/dikluwe/.codex/skills/tekt-materializacao-segregada/SKILL.md` e suas duas
referências. Nenhuma pasta materialization/context foi lida.

## Evidência e proveniência

O JSON homônimo registra UTC, HEAD `2f42d64253547734564513a1159ee6b584c1c4b4`,
working tree não commitado, diff stat, baseline P1340 e SHA-256 das autoridades.
Os pins históricos selecionados coincidem com o selo P1339; isso identifica
entradas, não certifica isolamento ou execução.

O L0 `infra/pipeline/context_stabilization.md` tem SHA-256
`62f27ec604a0b9b0d4121ecf43ac556a18a59cd53947744c29187c4d49c1781c`.
Suas cláusulas de descoberta, seed e A_k/I_k, junto de `compiler/eval.md`
§P1339, definem a obrigação. As assinaturas públicas existentes foram localizadas
em `eval/mod.rs:5661,5786,5812` e `pipeline.rs:222,303`; não se leu implementação
posterior para escolher os testes.

## Seleção, sem redução de obrigação

Reutilizar as definições imutáveis de `p1339-contract-r3.json`,
`p1339-ab-batch2-positive-oracles.json` e o mapa final
`p1339-ab-batch1-coverage-map-r2.json`. O JSON enumera cada caso selecionado,
fixture/hash, política de referência e perfil/ordem. A seleção pública é o
conjunto cujo `obligation_ids` intersecta W05/W07/W08/W09. Inclui controles de
counter/runtime necessários à causalidade, erro independente e preservação.

O recorte implementa W05, a integração causal W07, W08 e W09; W06 e W10 são
dependências obrigatórias. Isto organiza o trabalho P1340 e não declara fechadas
nem remove as outras obrigações P1339. Controles string ordinários conservam
baseline: não transformar o ancestral inteiro em requisito vanilla.

Reutilizar integralmente `p1339-ab-batch1-retention-lifecycle-typed-r3.json`:
retained-sibling-growth, replaced-capture-descendant, replaced-chain-descendant,
request-chain-points, increasing_set, oscillating_set, nested_producer,
fresh_callback_stable, nan_key_stable_zero, provisional-assert-final-panic,
legacy-sibling-error e selected-stable-error. Não mudar fontes, predicados,
DTO ou a conjunção dos checkers v3 → v2 → v1.

As testemunhas estruturais já congeladas continuam obrigatórias. Em especial,
`SW-F08-shared-pagination-budget` exige enumerar inicializações, ciclos e arestas
de retry por contexto/pages/positions/nested; `SW-F09-sink-error-lineage`
exige caminhos reais de criação, descarte, publicação e retorno Err. Contagem
de páginas ou saída pública isolada não substitui estas testemunhas.

## Interfaces e observabilidade

Inferência: os três métodos aprovados de EvalContext e as fachadas existentes
bastam para a coordenação produtiva. A necessidade de novo contrato público ou
de observação sem owner existente refutaria essa inferência.

Os resultados públicos não revelam identidade de captura retida nem o transcript
integral real de tentativas/sinks. O contrato histórico já fixa as portas
test-only `retention_actual` e `attempt_transcript_actual`, com ligação tardia
mecânica auditada. Elas devem observar o mesmo caminho usado em produção;
não construir um segundo orquestrador nem fabricar eventos/identidades a partir
do esperado. `Ok(false)` da API não distingue Different de Unproven.

A individualização para `03_infra/src/pipeline/context_stabilization.rs`
exige sucessor explícito de autoridade/manifesto. O selo P1339 fica histórico e
intocado; não se trata de simples resselo da linha Hash do Código. Mapear o owner
antigo das portas/testemunhas para o novo owner L3 é vínculo mecânico somente
se nenhum predicado ou requisito mudar, sob revisão independente.

## Gates a executar

1. Congelar baseline P1340 e os L0s individualizados no sucessor de autoridade;
   verificar transferência sem perda e pins dos oráculos.
2. RED público real no WIP anterior à implementação, com controles positivos,
   negativos e de preservação. Falha de build/API ausente não é RED semântico.
3. Mesmas fixtures/predicados públicos no produto real, perfis default/html/a11y/
   html+a11y e ordens normal/repeat/reverse, conservando diagnóstico integral.
4. Todos os cenários de lifecycle, testes dependentes F04/F05 e testemunhas
   estruturais: descoberta selecionada descartada; I0 vazio seletivo; erros
   independentes preservados; A1..A5 com orçamento único paginado; D5 lê I4;
   sem A6; descendentes substituídos; markers/updates sem acúmulo; sinks
   conservados publicados uma vez; erro final impede exportação.
5. Revisão independente da ligação e discriminação afetadas; gates finais
   históricos continuam devidos, incluindo build/test, fmt/diff, lint zero,
   V5/V15/V26 e integridade dos inputs.

`Unknown`, binding ausente ou testemunha incompleta não recebem sucesso.
Nenhum build, teste, RED/GREEN, selo ou veredito foi executado por este autor.
