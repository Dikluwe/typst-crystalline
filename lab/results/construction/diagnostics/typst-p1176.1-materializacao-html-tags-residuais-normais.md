# P1176.1 — materialização das tags residuais normais

**Data:** 2026-08-25
**Estado:** GREEN, antes de staging
**Vanilla:** `a51e02804`

## Proveniência

O fecho foi medido em `2026-08-25T15:34:33-03:00`, sobre o commit
`ce49041de76eb64c011e990e9774a0339f979211`, com working tree não commitado.
`git diff HEAD --stat` reportou 6 ficheiros rastreados, 809 inserções e 24
remoções; documentos untracked não aparecem. O vanilla tinha SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`; o
cristalino reconstruído, `e68cd9e8e59fcd0c4ba86a49d16268d7dd3e4a6004ff381d4b9e10905fe42ce3`.

## RED → GREEN

O primeiro resselo propagou `a71836aa` ao L1 e `f1fe441f` ao L3. Os REDs
falharam em `html.datalist` ausente (1 falha, 5247 filtrados) e na fixture de
boundary top-level/summary (1 falha, 853 filtrados).

Foram acrescentados exatamente `datalist`, `noscript` e `summary` como
wrappers global-only e entradas em `TYPED_TAGS`. O teste L1 passou e cobre
globals, body e rejeição de específico.

Em L3, `summary` foi acrescentado a `BLOCK_TAGS`. `block_sequence` ganhou uma
análise de espaço própria: HtmlElem não agrupável encerra o lado do parágrafo,
mas inline vazio já encontrado continua elegível à proteção P1175.1. Assim,
espaços junto a `datalist`/`summary` top-level desaparecem; dois `noscript`
vazios ainda recebem pre-wrap; `datalist` dentro de body permanece inline. Os
seis testes HTML P117x passaram juntos. O teste CLI P1176.1 passou 1/1, com 67
filtrados.

## Decalque

As fixtures principal e vazia foram recompiladas com constructors tipados nos
dois binários. Ambos os `cmp` terminaram com exit 0, cobrindo grouping de
`datalist`/`noscript`, `summary` block, `details` e boundaries vazias.

O resselo final propagou `48e8382f` ao L1 e `e4bdb72a` ao L3, com zero drift
warnings.

## Escopo

Nenhuma família documento, tabela ou ruby foi exposta. `HtmlElem`, target,
pipeline, CSS e scripting não mudaram. Índice vazio; nenhum commit criado.

## Validação integral

`cargo test --workspace` passou com 5248 testes core, 854 infra, 55 shell, 2
wiring, 68 CLI e 2 testes do lint; 3 doctests core permaneceram ignorados.
`cargo build --workspace`, `cargo fmt --all -- --check`, `crystalline-lint .`
e `git diff --check` terminaram com exit 0; apenas warnings e infos
preexistentes não bloqueantes permaneceram.
