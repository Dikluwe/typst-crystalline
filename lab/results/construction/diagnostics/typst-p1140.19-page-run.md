# Diagnóstico P1140.19 — fronteira de page-run

**Data:** 2026-08-24  
**Estado:** fechado  
**Binding público `page`:** deliberadamente ausente

## Resultado

Foi materializado `Content::PageRun`, um contentor interno que aplica a
configuração de página somente ao seu body e restaura o snapshot anterior ao
terminar. A implementação não expõe `page(...)`; ela resolve o pré-requisito
estrutural identificado em P1140.18.

## Medição anterior à decisão

No vanilla ratificado, as sondas produziram:

| Caso | Resultado |
|---|---|
| run entre conteúdo externo | `200×200 → 100×120 → 200×200` |
| body vazio no meio | 3 páginas, central vazia `100×120` |
| body vazio sozinho | 1 página `100×120` |
| runs consecutivos | `100×120 → 140×160` |
| runs aninhados | `180×180 → 100×120 → 180×180 → 240×240` |
| pagebreak dentro do run | duas `100×120`, depois `200×200` |
| run em block/columns | erro de configuração dentro de container |

A restauração LIFO refutou eventos soltos start/end e push/pop de `SetPage`:
ambas as formas permitem estado desequilibrado. Foi escolhida a variante α,
`PageRun`, que contém body e configuração numa unidade fechada.

## RED→GREEN

Depois da confirmação explícita do gate ADR-0127, o primeiro RED falhou na
compilação:

```text
no variant or associated item named `page_run` found for enum `Content`
```

O GREEN cobre 8 testes:

- isolamento e restauração simples;
- body vazio sem página-cauda;
- restauração LIFO aninhada;
- pagebreak explícito interno;
- numeração local sem vazamento;
- diagnóstico em container;
- `is_empty` estrutural do elemento;
- `map_text` recursivo preservando configuração.

## Implementação

- `01_core/src/entities/elements/page_run.rs`: entidade pura e `Element`;
- `01_core/src/entities/content.rs`: variante e constructor interno;
- `01_core/src/compiler/layout/page_run.rs`: consumer atomizado forma B;
- `Layouter::install_page_config`: sincronização de configuração numa página
  vazia, sem semântica progressiva de `SetPage`;
- `page_run_boundary_empty`: distingue a página corrente criada pela boundary
  final de uma página vazia exigida por `pagebreak()`;
- introspecção e transformações descem no body;
- queries L3 descem no body sem interpretar configuração.

O consumer usa snapshot local por chamada. Runs aninhados restauram em LIFO
sem estado global, identificador, `dyn` ou mapa genérico. Em sub-frame, a
configuração produz o diagnóstico ratificado em vez de ser ignorada.

## Descoberta no build workspace

A suíte L1 ficou verde antes do build completo, mas `cargo build --workspace`
encontrou dois matches exaustivos em `03_infra/src/query_helpers.rs`. O L0
`infra/query-helpers.md` foi atualizado primeiro; depois, `has_any_text` e
`count_variant` passaram a descer em `PageRun.body`. Nenhuma decisão de layout
foi movida para L3.

## Validação

- testes focados: 8 aprovados;
- `cargo test -p typst-core --lib`: 5.170 aprovados, zero falhas;
- `cargo build --workspace`: aprovado;
- `crystalline-lint --fix-hashes .`: zero drift após corrigir 2 headers finais;
- `crystalline-lint .`: exit 0;
- `git diff --check`: aprovado;
- `repr(type(page))` no release cristalino continua falhando com
  `unknown variable page`, comportamento esperado até P1140.21.

Hashes finais:

- `entities/elements/page_run.md` → `5db04263`;
- `entities/content.md` → `a3f5a55b`;
- `compiler/layout.md` → `7962f4b2`;
- `infra/query-helpers.md` → `3e7c7067`.

## Proveniência

- HEAD: `45b547073d7686cdd5d3e3030c82de3e22ec395f`;
- estado: working tree não commitada;
- hora final: `2026-08-24T13:57:11-03:00`;
- `git diff HEAD --stat`:
  `90 files changed, 1027 insertions(+), 511 deletions(-)`;
- vanilla: `/usr/local/bin/typst`, ratificado `a51e02804`;
- cristalino: código e testes da mesma working tree.

P1140.20 pode agora completar propriedades de página sem precisar inventar a
fronteira lexical; P1140.21 continua dono do constructor e binding públicos.
