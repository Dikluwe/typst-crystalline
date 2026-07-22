# Prompt — typst-passo-841: `layout::em` — `Sub` de `Length` ausente (achado #31)

**Origem**: achado #31 de P831 (lote 5)
**Estado**: aguardando execução

---

## Achado (medição de P831)

`#repr(2em - 5em)` — cristalino `error: cannot apply Sub to length and length` (exit 1); vanilla `-3em`. Falta o braço `Sub` para `Length` em `01_core/src/engine/eval/operators.rs` (só existe `Add`, em `:382`). Vanilla: `foundations/ops.rs:196`.

---

## Regra da linha de trabalho (obrigatória)
Comando exacto + saída literal (vanilla vs cristalino). Contagem de testes de `typst-core` a bater com os testes novos.

## Passo 1 — Sonda

1. Testar `2em - 5em`, `10pt - 3pt`, e combinações mistas de unidade (`1cm - 5mm`) nos dois binários.
2. Nota: este achado é da mesma família do achado #36 (`layout` define, `Angle - Angle` ausente) e #37 (`fr + fr` ausente), todos sintomas de `operators.rs` ter uma lista incompleta de braços aritméticos por tipo. Vale conferir, ao implementar este, se faz sentido tratar os três acasos numa revisão só de `operators.rs` em vez de três passos separados — mas isso é decisão de quem for executar, não assumida aqui.

## Passo 2 — Implementação

Adicionar o braço `Sub` para `Length` em `operators.rs`, replicando a semântica do vanilla (subtração numérica direta, preservando a unidade quando compatível).

## Passo 3 — Validação

1. Recompilar. Casos da sonda batendo com o vanilla.
2. Suíte completa, comando + contagem antes/depois.

## Relatório

`00_nucleo/diagnosticos/typst-passo-841-relatorio.md` com medição antes, código identificado, diff, medição depois, contagem de testes. Se decidir tratar junto com #36/#37, documentar isso explicitamente e não abrir os prompts 842 (que cobre #36/#37 dentro de `layout` define) redundantemente — avisar no relatório qual dos dois passos efetivamente implementou o quê.
