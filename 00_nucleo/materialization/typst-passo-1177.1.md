# P1177.1 — materializar família HTML ruby

**Data:** 2026-08-25
**Estado:** `EXECUTADO — GREEN; PARADO ANTES DE STAGING`
**Baseline:** P1176.1 GREEN + P1177 aprovado
**Vanilla:** `a51e02804`

## Objetivo

Ressellar os L0s aprovados, obter RED e materializar exatamente
`html.ruby`, `html.rp` e `html.rt` pela via global-only existente. Corrigir no
exporter o descarte de espaço entre filhos consecutivos `rp`/`rt` dentro de
`ruby`, sem remover espaços junto à base ou a outros elementos inline.

## Sequência

1. registrar a aprovação e ressellar os consumers L1/L3;
2. acrescentar testes RED de binding, body/attrs, grouping e whitespace;
3. implementar os três constructors pelo dispatcher estático;
4. normalizar somente o espaço ruby nucleado no L0 L3;
5. comparar fixture tipada com o vanilla ratificado;
6. executar suítes, build, formato, lint e `git diff --check`;
7. fechar diagnóstico e parar antes de staging.

## Aceitação

- exatamente três bindings novos, totalizando 58;
- 76 globais e body none/Content reutilizados;
- `ruby` agrupável, `rp`/`rt` isolados boundaries;
- espaço entre `rp`/`rt` descartado dentro de `ruby`;
- espaço após base e em torno de `span` preservado;
- nenhuma validação parental, entidade, default ou fase nova;
- fixture tipada byte-idêntica ao vanilla;
- suíte/build/formato/lint GREEN;
- índice vazio e nenhum commit.

## Resultado

Os três bindings foram materializados pela via global-only, elevando o
dispatcher de 55 para 58 tags. O exporter recebeu somente a regra de contexto
`ruby` aprovada: espaço entre dois filhos `rp`/`rt` é descartado, enquanto
espaços junto à base e a `span` permanecem. A fixture tipada foi byte-idêntica
ao vanilla ratificado. O fecho está em
`00_nucleo/diagnosticos/typst-p1177.1-materializacao-html-familia-ruby.md`.
