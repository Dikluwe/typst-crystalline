# Diagnóstico P1140.20.1 — geometria lógica de página

**Data:** 2026-08-24  
**Estado:** implementado e validado

## Resultado

- `Paper` puro com os 107 nomes da tabela ratificada, incluindo Unicode;
- A4 deriva de 210×297 mm por `mm × 72 / 25.4`; removidos os defaults
  empíricos em pontos de `PageConfig`, layout callback e export defaults;
- `PageBinding::{Auto,Left,Right}` com direção e paridade física;
- `PageMarginSpec.two_sided` preserva físico versus inside/outside e fold;
- paper/width/height/flipped seguem precedência por eixo com dimensões-base
  separadas, evitando dupla troca;
- `SetPage` e `PageRunElem` transportam paper/flipped/binding;
- parser rejeita papel, tipo, binding, propriedade e chave de margem inválidos;
- `compiler/layout/page_geometry.rs` é o owner da aplicação compartilhada;
- margens lógicas são resolvidas novamente para cada página física;
- page-run mantém snapshot e restauração LIFO;
- snapshots PDF foram regenerados porque o MediaBox A4 passou do arredondado
  `595.28×841.89` ao normativo `595.2756×841.8898` serializado;
- `.gitattributes` classifica PDFs como binários para o gate de whitespace.

## Testes

Novos testes cobrem tabela/nome inválido, conversão A4, binding LTR/RTL,
paridade, fold, precedência paper/override/flip e margem lógica ímpar/par.

Validação final:

- core: 5175 passed;
- infra: 832 passed;
- demais suítes do workspace: 53 + 2 + 55 + 2 passed; 3 ignored;
- `cargo build --workspace`: aprovado;
- `crystalline-lint .`: exit 0;
- `git diff --check`: aprovado.

## Proveniência

- HEAD `45b547073d7686cdd5d3e3030c82de3e22ec395f`;
- working tree não commitada e contendo passos anteriores;
- medição final em 2026-08-24;
- stat observado antes do relatório: `106 files changed, 1463 insertions(+),
  577 deletions(-)`; não atribuir esse total exclusivamente a P1140.20.1.

## Próximo

P1140.20.2 deve executar sua própria Fase A e gate para bleed, fill,
background e foreground. `page` público permanece ausente até P1140.21.
