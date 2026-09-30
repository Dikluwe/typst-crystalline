# L0 — compiler/eval/repr/selector
Hash do Código: b91fa827

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/eval/repr/selector.rs`. No HEAD `6b147bd213e07e1dc896a8e798b84bf4d8dbbdcb` e árvore limpa, `01_core/src/compiler/eval/repr.rs:1467-1515` possui o comportamento separado. SHA-256, horário e estado estão em `00_nucleo/diagnosticos/atomizacao-repr-2026-09-30-before.json.gz`. O objetivo é pesquisa algorítmica por comportamento, sem ganho de desempenho presumido.

## Contrato

Formatar Selector de modo recursivo, incluindo Element.where com campos ordenados e Before/After com inclusive falso explícito. Preservar nomes de função, escaping e formatação dos Values.

O contrato detalhado e as evidências históricas de paridade continuam em `00_nucleo/prompts/compiler/eval/repr.md`; a extração não muda saída, assinatura pública ou fase. Não há I/O nem estado global mutável.

## Aceitação

Preservar testes de morfologia de linguagem, compilar e passar `crystalline-lint .` com owner e hash válidos.
