# L0 — stdlib/collections/arguments — Métodos de Arguments
Hash do Código: 423d9ed5

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/collections/arguments.rs`

## Medição antes da decisão

A working tree não commitada capturada em `lab/results/construction/diagnostics/atomizacao-stdlib-collections-2026-09-30-before.json.gz` contém as rotinas desta família em `01_core/src/compiler/stdlib/collections.rs`; a suíte completa do L1 passou com 6401 testes e 2 ignorados antes da extração. Esta é medição estrutural, não uma nova prova bilateral de paridade com o vanilla nem benchmark. O contrato histórico e as qualificações do pin vanilla permanecem em `00_nucleo/prompts/core/compiler/stdlib/collections.md`.

## Contrato de linguagem

len, pos, named, at, filter e map operam sobre a sequência causal de ocorrências; preservam nomes duplicados legais, spans, ordem, uma invocação por ocorrência e destacamento de value_span em map.

O despacho de método ligado e a exposição no tipo permanecem no owner pai; ambos devem chamar este comportamento sem cópia algorítmica. Preservar identidade das operações, sintaxe, morfologia, erros, spans e precedência de validação da implementação medida. A extração interna não altera contrato público, comportamento por defeito, fase do pipeline ou compatibilidade.

## Aceitação

Suíte do L1 e `crystalline-lint .` sem regressão. Comparação com o vanilla deve medir semântica, sintaxe e morfologia segundo ADR-0107/0108, sem pressupor igualdade de mecânica. Ownership 1:1 segundo ADR-0129/0130.
