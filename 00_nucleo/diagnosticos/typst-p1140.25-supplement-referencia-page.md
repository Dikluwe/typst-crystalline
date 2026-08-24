# P1140.25 — Fecho de supplement e referência de página

**Data:** 2026-08-24  
**Estado:** fechado

## Proveniência

- HEAD: `45b547073d7686cdd5d3e3030c82de3e22ec395f`.
- Medição final: `2026-08-24T17:35:47-03:00`.
- Estado: working tree não commitado.
- `git diff HEAD --stat`: `126 files changed, 3213 insertions(+), 658 deletions(-)`.
- O stat cobre a árvore acumulada da série P1140, não apenas P1140.25; a lista
  exata de ficheiros desse estado é reproduzível com `git status --short` e o
  conteúdo com `git diff HEAD`.
- Baseline vanilla ratificado: `a51e02804`.

## Medição e decisão

As fontes medidas antes da decisão foram:

- `lab/typst-original/crates/typst-library/src/layout/page.rs:315-329`:
  contrato auto/none/content;
- `lab/typst-original/crates/typst-layout/src/pages/run.rs:145-149`:
  resolução localizada e captura por page-run;
- `lab/typst-original/crates/typst-layout/src/document.rs:98-103` e
  `typst-layout/src/introspect.rs:29-56,122-129`: snapshot e sealing;
- `lab/typst-original/crates/typst-library/src/model/reference.rs:135-257,334-355`:
  `form: "page"`, precedência explícita e NBSP.

A tabela localizada foi derivada dessas fontes, não de números empíricos no
código: inglês `page`, português e espanhol `página`, alemão `Seite` e francês
`page`, incluindo os aliases de idioma já reconhecidos pelo domínio.

## Contrato implementado

| Entrada | Snapshot de página | Store selado | `ref(form: "page")` |
|---|---|---|---|
| omitido/auto | nome localizado resolvido | conteúdo resolvido | supplement + NBSP + número |
| none | conteúdo vazio | conteúdo vazio | somente número |
| content/string | conteúdo explícito | conteúdo explícito | conteúdo + NBSP + número |
| supplement explícito em ref | sem alteração | sem alteração | sobrescreve o valor da página alvo |

`PageSupplement` é o owner do domínio. `PageConfig`, `PageRunElem`,
`Content::SetPage` e `Page` transportam o valor; `PageStore` da iteração
anterior é carregado no estado puro do layouter. A referência consulta a
página do alvo, mantém o destino interno clicável e emite erro com hint se a
página não tiver numbering.

## RED → GREEN

O RED contratual foi estabelecido pela ausência dos campos públicos
`page.supplement` e `RefForm`, que impedia compilar os testes novos antes do
transporte. Depois da implementação, o filtro `p1140_25` executou 6 testes:

- localização e estado none;
- transporte no eval;
- validação de `form` na stdlib;
- supplement e número da página alvo;
- supressão por supplement none;
- diagnóstico/hint sem numbering e conservação do link interno.

Resultado final: **6 passed; 0 failed**.

## Gates finais

- `cargo test -p typst-core p1140_25 -- --nocapture`: 6 passed.
- `cargo test --workspace --quiet -- --test-threads=1`: suites com
  5193, 835, 53, 2, 55 e 2 testes passaram; 0 falhas; 3 testes documentais
  permaneceram ignorados.
- `cargo build --workspace --quiet`: exit 0.
- `crystalline-lint .`: exit 0, sem violations; avisos preexistentes mantidos.
- `git diff --check`: exit 0.

## Divergência restante

P1140.25 não expõe deliberadamente o constructor/binding público
`page`/`std.page`. Essa superfície, o rebaseline diferencial e a decisão final
de fecho da série pertencem a P1140.26.
