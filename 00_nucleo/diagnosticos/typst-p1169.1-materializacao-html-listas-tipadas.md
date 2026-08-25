# P1169.1 — materialização de `html.ol` e `html.li`

**Data:** 2026-08-25  
**Estado:** GREEN; parado antes de staging  
**HEAD:** `9412ad4593608195b3946bf47d48e12ff7dc5f83`  
**Vanilla ratificado:** upstream `a51e02804`  
**Binário vanilla medido:** `/usr/local/bin/typst`, SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`

## Proveniência

A execução começou em `2026-08-25T14:04:58-03:00` sobre working tree não
commitado que já continha P1168 GREEN e os documentos P1169. O índice estava
vazio. Antes desta materialização, o binário cristalino debug tinha SHA-256
`f59bfc98fd6d5c95cda155208e0ab4dce0ed0a777a5f27dd0417587bc13f3a72`.

À medição final, `2026-08-25T14:09:41-03:00`, continuavam alterados os L0s de
stdlib/entities/export HTML, os módulos cristalinos correspondentes, o teste
CLI e os documentos P1168/P1169. O estado exato é recuperável pelo HEAD acima
mais o working tree desta entrega; nenhum ficheiro estava staged.

## L0 e RED

O contrato público já tinha aprovação explícita no gate ADR-0127. Antes do
RED, `crystalline-lint --fix-hashes .` ressellou
`01_core/src/compiler/stdlib/html.rs` com o hash L0 `073860ae` e terminou sem
drift.

O primeiro teste `p1169_1_html_module_expoe_ol_e_li` falhou causalmente porque
`html.ol` ainda não existia: `0 passed; 1 failed; 5237 filtered out`. Só depois
desse RED foi escrita a implementação.

Ao fechar o passo, a marcação documental de materializado alterou o L0 e um
segundo resselo atualizou a linhagem do mesmo módulo para `c81569d2`; a nova
análise terminou com zero drift.

## Materialização

Foram acrescentados exatamente os bindings `ol` e `li`. Ambos reutilizam a
única tabela dos 76 atributos globais. Uma fatia estática por tag acrescenta:

- `ol`: `reversed: Presence`, `start: Int` e `type` restrito a `"1"`, `"a"`,
  `"A"`, `"i"`, `"I"`;
- `li`: `value: Int`.

O lookup consulta primeiro a fatia específica e depois os globais, mas a
ordem emitida continua sendo a ordem dos argumentos nomeados. `Presence`
omite `false` e grava string vazia para `true`; os inteiros aceitam todo o
domínio já suportado. O helper de enum reproduz a classe diagnóstica medida,
inclusive `found TYPE` para tipo errado. Body omitido continua
`HtmlBody::None`. O exporter de produção não foi alterado.

## GREEN e decalque

Os três testes L1 focados passaram (`3 passed; 5237 filtered out`) e o teste
CLI focado passou (`1 passed; 60 filtered out`). A fixture compacta aninhada
com `ol`, dois `li`, atributos globais/específicos e `strong` compilou nos dois
binários; `cmp` terminou com exit code 0. A sonda inválida de `ol.type`
reproduziu a enumeração esperada e `html.a` permaneceu ausente.

`cargo build --workspace` e `cargo test --workspace` terminaram com exit code
0. Entre os totais observados na suíte integral: infra `848`, shell `55`,
wiring unitário `2`, CLI `61` e testes do linter `2`, todos sem falhas; os três
doctests de core permaneceram ignorados.

Na barreira final, `cargo fmt --all -- --check`, `git diff --check` e
`crystalline-lint .` terminaram com exit code 0. O linter conservou apenas os
avisos históricos V16–V20; não houve violation nem drift. O índice continuou
vazio.

## Scope-outs

P1169.1 não materializa `html.a`, reservado a P1170; não materializa `br`, a
tabela void nem a correção de whitespace, reservados a P1171. As demais tags,
raw, `frame`, CSS, MathML e positions continuam abertas. Nenhum contrato além
dos dois bindings aprovados foi ampliado.
