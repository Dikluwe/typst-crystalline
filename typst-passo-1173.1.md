# P1173.1 — materializar segundo lote HTML global-only

**Data:** 2026-08-25  
**Estado:** `EM EXECUÇÃO`  
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
