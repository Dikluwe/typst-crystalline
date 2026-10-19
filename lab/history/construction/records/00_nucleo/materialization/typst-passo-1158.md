# P1158 — materialização de `page.numbering` tipado e callbacks

**Data:** 2026-08-25
**Estado:** `EXECUTADO — contrato/transporte GREEN; callbacks concluídos em P1159`
**Baseline:** vanilla ratificado `a51e02804`
**Gate:** contrato e fase aprovados no P1157

## Objetivo

Materializar os L0s P1157 em testes-first: `Numbering::{Pattern, Func}`, delta
ternário em page/set/page-run, snapshots tipados, realização de callback com
dois números para margem e um para referência, e retorno cru por
`location.page-numbering()`.

## Ordem

1. adicionar testes RED de entidade/casts e transporte;
2. criar a entidade Numbering e migrar contratos públicos;
3. fechar o caminho visível com números lógicos corrente/final;
4. realizar a vista unária para referências;
5. expor Numbering cru por Location;
6. revalidar testes focados, typst-core, workspace, fmt e lint.

Não executar I/O em L1, não avaliar callbacks diretamente no layouter e não
reduzir Numbering funcional a string. Corrigir primeiro o L0 se a implementação
exigir contrato diferente do aprovado.

## Execução

Materializado:

- `entities::numbering::Numbering::{Pattern, Func}`;
- `page()` e `#set page` aceitam `Str | Func | None`;
- delta ternário em `Content::SetPage` e `PageRunElem`;
- snapshots tipados em `PageConfig`, `Page` e `PageStore`;
- `Introspector::page_numbering` tipado e wrapper L3 atualizado;
- consumers de pattern preservados; Func nunca é convertido para string.

Validação no estado não commitado:

```text
cargo test -p typst-core --lib: 5.224 passed; 0 failed
cargo check --workspace: exit 0
```

O total cresceu de 5.223 para 5.224 pelo teste novo da entidade Numbering.
Warnings históricos permanecem fora do escopo.

P1158 foi deliberadamente a primeira metade do constructor dividido. P1159
completou a realização de callbacks, vistas `visible/reference`, relayout,
`location.page-numbering()` e probes E2E.
