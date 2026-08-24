# Diagnóstico P1140.20.1 — gate da geometria de página

**Data:** 2026-08-24  
**Estado:** Fase A fechada; aguarda confirmação ADR-0127  
**Código funcional:** não alterado

## Decisão preparada

- novo owner puro `entities/page_geometry.rs` para `Paper`, `PageBinding` e
  semântica folded de `PageMarginSpec`;
- tabela completa do vanilla ratificado em milímetros e conversão normativa
  `mm × 72 / 25.4`;
- remoção obrigatória dos literais empíricos `595.28`/`841.89` do default;
- paper fornece dimensões por eixo, overrides vencem e flip ocorre depois;
- binding auto deriva de direção; inside/outside resolve somente ao fechar
  cada página usando paridade física;
- `SetPage` e `PageRunElem` ganham paper/flipped/binding e continuam
  deliberadamente incompletos até P1140.20.2–.4;
- `page` e `std.page` continuam ausentes até P1140.21.

## Contratos públicos afetados

P1140.20.1 acrescentará campos públicos a `Content::SetPage`, `PageRunElem` e
`PageConfig`, além de estender `PageMarginSpec`. Também altera o comportamento
padrão ao derivar A4 da tabela normativa e completa argumentos públicos de
`#set page`. Portanto o gate ADR-0127 é obrigatório.

## L0s e hashes SHA-256

| L0 | SHA-256 |
|---|---|
| `entities/page_geometry.md` | `fa95ffd3c1bdfc9a3ea13eb6dea03ff643225163903635638b76d2b208337453` |
| `entities/layout_types.md` | `f7c5eb021e0096c567ceafd3188af02a1780cdbf3c5f8722dd5603eea5fdb7df` |
| `entities/content.md` | `ed1e98e8da3c96db3a71c281449cde0ee1ff5e24f780067e06bc018f36da5ea0` |
| `entities/elements/page_run.md` | `0015948dfdcee9c67d13e02b89a84e0c8cbe217383b09429b6af2a09fd9afc6b` |
| `compiler/eval.md` | `f12a49ac3eb0f58064caa1334246af6ced836100b9c5ddcf21e764e5ae011de4` |
| `compiler/layout.md` | `e86b497366dfaaa756b0adec8b0f9a05ad4182ed74d483b7e313bd70be742108` |
| `compiler/stdlib/layout.md` | `7a9a65c01056888f0581ce7527b3fd4b98c963e794fc5e61b5720f130222950d` |

`crystalline-lint --fix-hashes .` atualizou 27 headers que apontam para os L0s
partilhados: somente valores `@prompt-hash`, sem lógica.

## Proveniência

- HEAD `45b547073d7686cdd5d3e3030c82de3e22ec395f`;
- working tree não commitada;
- medição final dos hashes em `2026-08-24T14:20:29-03:00`;
- stat dos ficheiros rastreados nessa hora: `90 files changed, 1079
  insertions(+), 511 deletions(-)`; esse total inclui trabalho anterior;
- alterações próprias desta Fase A: sete L0s (um novo, seis atualizados), este
  diagnóstico, atualização do passo e 27 headers de linhagem.

## Próxima ação após confirmação

Escrever REDs para tabela/precedência/flip, binding/paridade, fold/conflitos e
restauração; confirmar falha; implementar; executar suítes, build e lint. Sem
confirmação, não escrever testes ou código funcional.
