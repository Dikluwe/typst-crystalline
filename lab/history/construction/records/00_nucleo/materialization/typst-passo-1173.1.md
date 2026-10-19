# P1173.1 — materializar segundo lote HTML global-only

**Data:** 2026-08-25  
**Estado:** `EXECUTADO — GREEN; PARADO ANTES DE STAGING`
**Baseline:** P1172.1 GREEN + P1173 aprovado  
**Vanilla:** `a51e02804`

## Objetivo

Ressellar, obter RED e materializar exatamente `abbr`, `address`, `article`,
`aside`, `b`, `bdi`, `bdo`, `cite`, `code`, `dfn`, `i`, `kbd`, todos com 76
globais e body content opcional. Corrigir em L3 somente whitespace adjacente a
block siblings medido em P1173.

## Aceitação

- exatamente 12 bindings novos, zero específicos;
- tabela global única reutilizada;
- chamada vazia `body: none` e body Content;
- phrasing/block e wrappers P1172.1 preservados;
- `address/aside` formatados sem espaço estrutural;
- fixtures byte-idênticas ao vanilla;
- suíte, build, formato e lint GREEN;
- índice vazio e nenhum commit.

## Resultado

Os 12 bindings foram materializados por uma única via global-only. O exporter
passou a colapsar `Content::Space` adjacente a elementos block ou `br`, sem
classificar custom tags nem todos os void elements como block. Os REDs de L1 e
L3 passaram a GREEN, a integração CLI passou e a fixture foi byte-idêntica ao
vanilla ratificado. O fecho detalhado está em
`00_nucleo/diagnosticos/typst-p1173.1-materializacao-html-tags-global-only-lote2.md`.
