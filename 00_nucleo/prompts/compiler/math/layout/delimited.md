# Prompt L0 — `math/layout/delimited` — `MathDelimited`
Hash do Código: 6cce714e

**Camada**: L1 · **Alvo**: `01_core/src/compiler/math/layout/delimited.rs`
**Origem**: fatiado de `rules/math/layout.md` em **P314** (ADR-0104). Núcleo
partilhado: ver `math/layout/_comum.md`.

---

`MathDelimited` — par de delimitadores fixos. **P919**: deixou de chamar
`MathLayouter::apply_axis_offset` (ver `_comum.md` §P919 e secção abaixo). Test regressão
`delimitado_com_axis_height_nao_regride` (`tests.rs:520+`).

## P912 — Cálculo de Altura de Delimitadores Balanceados

Em `layout_delimited`, a altura-alvo do delimitador balanceado (quando `balanced = true`)
é calculada simetricamente em relação ao eixo matemático como `2.0 * (ascent - axis).max(descent + axis)`,
onde `axis` é a altura do eixo (`axis_height`) em pontos tipográficos, em vez de soma simples.
Isto dimensiona o glifo do delimitador (via `layout_stretchy_delimiter`, que já o centra no eixo
internamente — `stretchy.md` §P917) — não desloca o corpo nem o conjunto final.

**P1132g — binômio não balanceia a altura-alvo.** O `MathFracElem` com
`line=false`, forma semântica interna produzida por `binom`, seleciona o braço
`balanced=false` da mesma fórmula vanilla: a altura-alvo é `body.height()`.
As demais expressões delimitadas continuam a usar o cálculo balanceado acima.
Essa distinção deriva do conteúdo e das suas métricas reais; não usa deslocamento
ou constante medidos da fixture.

> **Fonte de paridade**: vanilla `lab/typst-original/crates/typst-layout/src/math/fenced.rs:93-99`
> (`if balanced { 2.0 * (f.ascent() - axis).max(f.descent() + axis) } else { f.height() }`).
> Guarda de regressão em `01_core/src/compiler/math/layout/tests.rs:4071`
> (`axis_ok_delimitado_corpo_nao_ganha_deslocamento_de_axis_height`).

## P919 — remoção da chamada a `apply_axis_offset` (nunca devia estar aqui)

**Achado** (`typst-passo-919-relatorio.md` Fase A, vanilla `fenced.rs::layout_fenced`): a função
não faz nenhuma centragem do grupo delimitado como um todo — `open`/`body`/`close` são apenas
empilhados na linha (`ctx.push`/`ctx.extend`), sem `set_baseline` nenhum sobre o conjunto. Os
delimitadores (`open`/`close`) já chegam pré-centrados no eixo (dimensionados via P912 acima,
posicionados internamente por `layout_stretchy_delimiter`, que já desloca os seus próprios
`items` — `stretchy.md` §P917); o corpo (`body_box`) mantém a sua baseline própria, sem
alteração. Chamar `apply_axis_offset` sobre o `result` já concatenado (`hconcat`) nunca teve
fundamento no vanilla — voltaria a centrar algo que já está correcto, uma vez corrigido o bug de
omissão de `apply_axis_offset` (`_comum.md` §P919). Confirmado também empiricamente: `mutool
trace` de `x + (a) + y` mostra `x`/`a`/`y` já exactamente ao mesmo Y sem qualquer correcção. A
chamada final `self.apply_axis_offset(result, style.size)` é removida — `layout_delimited`
devolve `result` directamente.

**Critério de regressão**: `x + (a) + y` — todos os três elementos partilham exactamente a mesma
baseline (mesmo Y), antes e depois desta mudança (remoção de um no-op, não correcção de
comportamento visível).

## P1436 — layout de `MathLrElem` (APROVADO NO GATE ADR-0127 EM 2026-09-17)

### Medição anterior à decisão

No pin, `resolve_lr` resolve o body sem o mutar, usa `size` explícito/cascata
ou `100%`, constrói stretch vertical com short-fall `0.1em`, escala cercas nas
extremidades e preserva item único. Um `LrElem` interior implícito só é
absorvido quando seu tamanho efetivo é `100%`.

### Decisão proposta

Este owner recebe uma free function para `MathLrElem`, mantendo a forma B da
ADR-0109. Resolver tamanho por explícito > `math.lr.size` cascaded > `100%` e
aplicar short-fall `0.1em`, nunca uma constante derivada de fixture. O body sem
cercas continua visível; cercas exteriores e itens únicos seguem as regras do
pin; nesting com tamanho não-default não é achatado. Reutilizar helpers de
stretch e as fórmulas P912/P919 sem deslocar novamente o grupo ou assar o
tamanho no carrier. `layout_delimited` mecânico existente permanece estável.

Aceitação geométrica discrimina `100%`, `150%`, `2em` e cascata com witnesses
independentes de altura/posição; igualdade de boxes internas ou bytes de
render não é requisito.
