# L0 — compiler/eval/rules/pdf — Validação de opções PDF
Hash do Código: 4dcc845d

Layer: L1
Exclusive consumer: `01_core/src/compiler/eval/rules/pdf.rs`

## Medição antes da decisão

A working tree não commitada capturada em `lab/results/construction/diagnostics/atomizacao-eval-rules-2026-09-30-before.json.gz` contém as rotinas desta família em `01_core/src/compiler/eval/rules.rs`. A suíte L1 passou com 6401 testes e 2 ignorados antes da extração. É medição estrutural, não benchmark nem prova bilateral nova contra o vanilla. As medidas históricas e ressalvas do pin continuam em `00_nucleo/prompts/core/compiler/eval/rules.md`.

## Contrato de linguagem

Preservar validação de pages e standard, nomes fechados, ranges, conflitos tipados PdfStandards, ordem do primeiro erro, hints, spans e o Value original validado. Não realizar export PDF.

As assinaturas, identidades, casts, defaults, precedência de erros e fase de avaliação observados permanecem. O parent mantém diagnóstico compartilhado e bindings estáticos; outros owners de selector matching, terminação e textual realization continuam exclusivos. Não duplicar esses algoritmos.

## Aceitação

Suíte L1 e `crystalline-lint .` sem regressão. Paridade em semântica, sintaxe e morfologia (ADR-0107/0108), ownership exclusivo e pins válidos (ADR-0129/0130).
