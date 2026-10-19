# P1168 — materialização do primeiro lote de tags HTML tipadas

**Data:** 2026-08-25  
**HEAD de execução:** `9412ad4593608195b3946bf47d48e12ff7dc5f83` + working tree P1168  
**Vanilla:** pin ratificado `a51e02804`  
**Estado:** GREEN; nada staged

## Proveniência e RED

A execução iniciou em `2026-08-25T13:43:24-03:00`. SHA-256 do vanilla:
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`;
cristalino debug inicial:
`0d0cb57333432866b534aa063ed124266ab1c08403928a648fa0eb53679cb276`.
O índice estava e permaneceu vazio.

Depois do primeiro resselo, o teste
`p1168_html_module_expoe_exatamente_o_primeiro_lote_tipado` falhou em
`html.div` ausente: 0 aprovados, 1 falha. Esse foi o RED causal antes da
implementação.

## Implementação

`compiler/stdlib/html.rs` ganhou:

- exatamente 12 wrappers estáticos registrados por `TYPED_TAGS`;
- uma tabela fechada com exatamente 76 atributos globais únicos;
- 80 tokens exatos para `role` e enums menores pinados;
- casts de string, int, float, booleanos serializados, Presence, direção,
  none/auto, enums, unions e listas com shorthand;
- preservação da ordem de named args e omissão de Presence `false`;
- rejeição de named desconhecido, `data-*`, tipo e enum inválidos.

`ol`, `li`, `a`, `br` e `frame` continuam ausentes. O exporter permanece
genérico por `HtmlElem`; não ganhou branch por tag.

## Segundo gate descoberto pela medição

A primeira sonda GREEN parcial revelou:

```text
vanilla html.div()        -> elem(tag: "div", body: none)
vanilla html.elem("div") -> elem(tag: "div")
cristalino pré-correção   -> elem(tag: "div") nos dois casos
```

O L0 foi atualizado e o dono aprovou substituir o campo público por
`HtmlBody::{Unset, None, Content}`. Todos os consumers foram migrados:
`html.elem` preserva Unset; os constructors tipados materializam None; repr
distingue ambos; walkers/plain-text/export descem somente em Content.

## Correção contínua do exporter

Sondas genérica e tipada mostraram que valor de atributo vazio é emitido pelo
vanilla somente como nome (`hidden`), enquanto P1166 produzia `hidden=""`.
O L0 de export foi atualizado primeiro e a correção interna seguiu em fluxo
contínuo ADR-0127. A fixture compacta passou a ser byte-idêntica ao vanilla.

## GREEN observado

- 4 testes L1 P1168: PASS;
- 1 teste L3 de atributo vazio: PASS;
- 1 teste CLI P1168 de repr/DOM: PASS;
- `cargo build --workspace`: PASS;
- `cargo test --workspace`: PASS — 5.237 core, 848 infra, 55 shell, 2 wiring
  unit, 59 CLI e 2 testes do linter; doc-tests sem falha, 3 ignorados;
- após acrescentar o teste E2E P1168, a suíte CLI atual foi repetida
  integralmente: 60 aprovados, 0 falhas;
- fixture HTML compacta: `cmp` exit 0 contra o vanilla;
- feature off: diagnóstico gated e dois hints preservados;
- feature on: 12 bindings presentes; bindings excluídos continuam ausentes.

Warnings históricos do workspace não foram tratados como falhas.

## Resíduo medido e scope-outs

Uma fixture formatada com quebras/indentação entre expressões explícitas
produz espaços de texto no cristalino que o vanilla elimina. A fixture
compacta prova que isso é independente de constructor, casts e serialização
de nó. O eixo fica aberto para a auditoria de whitespace de P1171, junto da
interação texto/`br`; não é mascarado como MATCH integral de toda fonte HTML.

P1169 permanece dono de `ol`/`li`; P1170 de `a`; P1171 da tabela void, `br` e
whitespace adjacente. As demais tags, raw, CSS, MathML, `html.frame`, positions
e expansão rica continuam fora do corte.
