# P1224 — stroke complexo materializado internamente; CLI em novo gate

O “Execute” do dono foi registrado como aprovação explícita do contrato P1223
em `entities/geometry.md`. Baseline em `2026-08-26T17:24:41-03:00`: HEAD
`0a0fabe05cd001802c6dfcfffb183f7651ab17d1`, árvore não commitada acumulando
P1222/P1223, vanilla ratificado `a51e02804`.

O RED inicial falhou por ausência de `DashPattern`, `LineCap`, `LineJoin` e
defaults. A implementação adicionou os quatro campos complexos, preservou
`LineWidth` para `"dot"` e atualizou mecanicamente 147 construtores literais
com defaults. `native_stroke` agora aceita cap, join, nove presets de dash,
arrays/dicts com phase e miter limit positivo finito. `FrameItem::Shape` já
transportava `Stroke`, portanto não houve duplicação.

O SVG serializa tardiamente os cinco atributos de stroke. Os testes focais
passaram: 2 testes P1224 em `typst-core` e 1 em `typst-infra`; o check conjunto
de core+infra também passou. Sete mutantes focais têm testemunhas diretas, mas
o gate integral de 25+9 mutantes não foi executado e não se declara score 1.0.

Durante o A/B público surgiu uma segunda mudança de comportamento padrão:
`typst eval` vanilla converte `Stroke` em string JSON, enquanto o L0 vigente da
CLI cristalina ordena rejeitar tipos fora do modelo JSON. Uma tentativa local
foi retirada antes do veredito. A emenda P1224 foi escrita em
`00_nucleo/prompts/shell/cli.md` e aguarda confirmação ADR-0127.

Por isso paths gerais, corpus integral de ataques, readjudicação DSM e gates
workspace não foram executados. O resultado proporcional é:

```text
COMPLEX STROKE INTERNAL+SVG GREEN — PUBLIC EVAL GATED — GENERAL PATHS PARTIAL
```

EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO.
