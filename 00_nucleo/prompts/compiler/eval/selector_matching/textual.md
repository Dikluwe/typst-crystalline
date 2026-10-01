# L0 — compiler/eval/selector_matching/textual
Hash do Código: 51fd58ee

## Medição anterior à decisão

No HEAD ea7ae25ab (árvore limpa, 2026-09-30), a implementação vigente está em `01_core/src/compiler/eval/selector_matching.rs:370-481`. A medição identifica o limite do bloco a extrair; não mede desempenho nem altera a paridade de linguagem.

## Owner e contrato

Camada L1. Consumer produtivo único: `01_core/src/compiler/eval/selector_matching/textual.rs`. Casar seletores textuais e produzir intervalos e substituições para Text e Regex, preservando offsets UTF-8, ordem e a exclusão de matches vazios. Esta extração mantém contrato público, defaults, fase do pipeline, compatibilidade e pureza L1.

## Aceitação

O despacho permanece estático. Testes L1, build e crystalline-lint devem passar. Não inferir ganho de desempenho da separação.
