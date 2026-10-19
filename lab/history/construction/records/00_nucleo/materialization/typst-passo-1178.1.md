# P1178.1 — materializar família HTML de documento

**Data:** 2026-08-25
**Estado:** `EXECUTADO — GREEN; PARADO ANTES DE STAGING`
**Baseline:** P1177.1 GREEN + P1178 aprovado
**Vanilla:** `a51e02804`

## Objetivo

Ressellar e materializar exatamente `html.html`, `html.head`, `html.body` e
`html.title`. Implementar em L3 adoção/exclusividade de raiz `html`/`body` e
codificação escapable-raw de `title`, sem alterar entidade, default ou fase.

## Sequência

1. registrar aprovação e ressellar consumers L1/L3;
2. obter RED de bindings, composição documental e `title`;
3. acrescentar os quatro constructors pela via global-only;
4. implementar adoção/exclusividade antes do envelope automático;
5. implementar body textual pre/escapable-raw de `title`;
6. comparar fixtures tipadas com o vanilla ratificado;
7. executar suítes, build, formato, lint e diff;
8. fechar diagnóstico e parar antes de staging.

## Aceitação

- quatro bindings novos, totalizando 62;
- 76 globais e body none/Content reutilizados;
- `html` único substitui o envelope e `body` único é adotado;
- `html`/`body` com siblings retornam o erro medido;
- `head` permanece nó comum;
- `title` preserva whitespace/newline, escapa `&`/`<`, mantém `>`/aspas e
  rejeita filhos não textuais;
- fixtures tipadas byte-idênticas ao vanilla;
- suíte/build/formato/lint GREEN;
- índice vazio e nenhum commit.

## Resultado

Os quatro bindings foram materializados pela via global-only, elevando o
dispatcher de 58 para 62 tags. O exporter passou a adotar raízes `html`/`body`,
validar sua exclusividade e serializar `title` como escapable-raw textual. Nove
fixtures válidas foram byte-idênticas ao vanilla e três diagnósticos inválidos
coincidiram. O fecho está em
`00_nucleo/diagnosticos/typst-p1178.1-materializacao-html-familia-documento.md`.
