# P1174.1 — materializar terceiro lote HTML global-only

**Data:** 2026-08-25
**Estado:** `EXECUTADO — GREEN; PARADO ANTES DE STAGING`
**Baseline:** P1173.1 GREEN + P1174 aprovado
**Vanilla:** `a51e02804`

## Objetivo

Ressellar, obter RED e materializar exatamente `dd`, `dl`, `dt`, `figcaption`,
`figure`, `footer`, `header`, `hgroup`, `legend`, `main`, `mark` e `menu`, todos
com os 76 globais e body content opcional. Não alterar entidade nem exporter.

## Aceitação

- exatamente 12 bindings novos, zero específicos;
- tabela global única reutilizada;
- chamada vazia `body: none` e body Content;
- onze tags block e `mark` phrasing preservam a morfologia P1172–P1173;
- fixture tipada byte-idêntica ao vanilla ratificado;
- suíte, build, formato e lint GREEN;
- índice vazio e nenhum commit.

## Resultado

Os 12 bindings foram materializados pela via global-only existente, sem
alteração de entidade ou exporter. O RED de binding ausente passou a GREEN, a
integração CLI passou e a fixture tipada ficou byte-idêntica ao vanilla
ratificado. O fecho detalhado está em
`00_nucleo/diagnosticos/typst-p1174.1-materializacao-html-tags-global-only-lote3.md`.
