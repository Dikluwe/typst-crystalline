# L0 — stdlib/collections/bytes — Métodos de Bytes
Hash do Código: 78e68100

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/eval/call-once.toml sha256:a922bbaa9c3b4c8435708af8e68b4557801f93ba09188768d48c711ba35a0c15

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/collections/bytes.rs`

## Medição antes da decisão

A working tree não commitada capturada em `lab/results/construction/diagnostics/atomizacao-stdlib-collections-2026-09-30-before.json.gz` contém as rotinas desta família em `01_core/src/compiler/stdlib/collections.rs`; a suíte completa do L1 passou com 6401 testes e 2 ignorados antes da extração. Esta é medição estrutural, não uma nova prova bilateral de paridade com o vanilla nem benchmark. O contrato histórico e as qualificações do pin vanilla permanecem em `00_nucleo/prompts/core/compiler/stdlib/collections.md`.

## Contrato de linguagem

len conta octetos; at aceita índice negativo e default apenas fora dos limites; slice preserva limites e count, recusando casts inválidos e mantendo âncoras de diagnóstico de CollectionCallSpans.

O despacho de método ligado e a exposição no tipo permanecem no owner pai; ambos devem chamar este comportamento sem cópia algorítmica. Preservar identidade das operações, sintaxe, morfologia, erros, spans e precedência de validação da implementação medida. A extração interna não altera contrato público, comportamento por defeito, fase do pipeline ou compatibilidade.

## Aceitação

Suíte do L1 e `crystalline-lint .` sem regressão. Comparação com o vanilla deve medir semântica, sintaxe e morfologia segundo ADR-0107/0108, sem pressupor igualdade de mecânica. Ownership 1:1 segundo ADR-0129/0130.


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

Manter try_dispatch_bytes_method apenas cfg(test) para seus controles existentes; o despacho produtivo único len/at/slice fica no pai, conforme responsabilidade de catálogo já vigente. Primitives bytes_len/bytes_at/bytes_slice e seus corpos permanecem intactos. Não duplicar validação/algoritmos.

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
