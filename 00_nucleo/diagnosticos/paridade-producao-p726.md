# Paridade Produção — P726 — `fill:`/`stroke: none` em `block`/`box`/`grid`/`table` (+cells)

**Data:** 2026-07-13
**Passo:** `00_nucleo/diagnosticos/typst-passo-726.md`
**Hash do commit (implementação):** `f9ae244e7`.
**HEAD base:** `e20cf50b7` (fim de P725, branch `Tekt`).
**Estado:** FECHADO — `fill: none` e `stroke: none` aceites em 12 pontos
(block/box/grid/table × fill/stroke + table.cell/grid.cell), paridade
medida com o vanilla. 12 testes novos verdes, workspace sem regressão,
`crystalline-lint .` limpo. **`cetz` compila end-to-end (exit 0) pela
primeira vez — a cadeia de bloqueios de eval P678–726 está fechada.** A
paridade de pixels ainda não fecha: o bug de render de `curve` (P723)
produz página em branco — confirmado, isolado e registado para P727.

**Proveniência das medições (regra de proveniência):** todas as medições
deste relatório foram corridas em **working tree não commitado** sobre
`e20cf50b7`, com exactamente estes ficheiros alterados
(`git diff HEAD --stat`):

```
 00_nucleo/diagnosticos/achados-adiados-cetz.md |   9 +-
 00_nucleo/prompts/rules/stdlib/layout.md       |  59 +++++++++-
 00_nucleo/prompts/rules/stdlib/structural.md   |  25 +++-
 01_core/src/rules/eval/tests.rs                |  16 +++
 01_core/src/rules/stdlib/layout.rs             |  20 ++--
 01_core/src/rules/stdlib/mod.rs                | 157 +++++++++++++++++++++++++
 01_core/src/rules/stdlib/structural.rs         |  24 +++-
```

---

## 1. Sonda ampla (ADR-0108 — medir antes de decidir, cumprida)

### 1.1 Vanilla — todas as funções da família aceitam `none`

Documento da sonda do passo (`/tmp/p726-none.typ`), binário
`lab/typst-original/target/release/typst`, **exit 0**:

```typst
#block(fill: none)[x]   #block(stroke: none)[x]
#box(fill: none)[x]     #box(stroke: none)[x]
#rect(fill: none, stroke: none)[x]
#grid(fill: none, stroke: none, [a], [b])
#table(fill: none, stroke: none, [a], [b])
```

Sondas adicionais (mesmo binário):

| Caso | Vanilla |
|---|---|
| `table.cell(fill/stroke: none)`, `grid.cell(fill/stroke: none)` | exit 0 |
| `table.hline/vline(stroke: none)`, `grid.hline/vline(stroke: none)` | exit 0 (linha não desenhada) |
| `stroke(paint: none)` | **exit 1** — "expected color, gradient, tiling, or auto, found none" |

`none` = "sem preenchimento"/"sem traço", equivalente a omitir o
argumento. `stroke(paint: none)` é erro **no vanilla também** — o erro
cristalino nesse ponto é paridade, não lacuna.

### 1.2 Cristalino antes — 12 pontos a rejeitar `none` (medido função a função)

```
block(fill: none)  → error: block(fill): espera Color, recebeu none
block(stroke: none)→ error: block(stroke): espera Length / Color / Stroke, recebeu none
box(fill|stroke: none)   → idem (box)
grid(fill|stroke: none)  → idem (grid)
table(fill|stroke: none) → idem (table)
rect(fill|stroke: none)  → exit 0  (já aceite — parse_paint/parse_color via and_then, shapes.rs:90-94)
table.cell / grid.cell (fill|stroke: none) → erro (loops structural.rs:876, 1103)
table.hline(stroke: none) → erro (structural.rs:2527 etc.)
```

Pontos localizados no código (grep `espera Color` / `extract_stroke`):

- **fill** (6): `layout.rs:317` (grid), `:858` (block), `:1094` (box);
  `structural.rs:680` (table), `:876` (table_cell), `:1103` (grid_cell).
- **stroke** (6 neste escopo): todos via `extract_stroke`
  (`layout.rs:431`) — call sites grid `layout.rs:311`, block `:869`,
  box `:1104`, table `structural.rs:675`, table_cell `:875`,
  grid_cell `:1102`.
- **hline/vline** (4): `structural.rs:2527,2560,2593,2626` — de outra
  espécie (entidade com `Stroke` não-opcional) → scope-out, ver §2.2.

## 2. Implementação (L0: `prompts/rules/stdlib/layout.md` e `structural.md`, secções P726)

Braço `Some(Value::None)` junto a `None => None` em cada ponto (mesmo
idiom já usado para `caption`, `structural.rs:694`):

```rust
// fill — 6 pontos
Some(Value::Color(c)) => Some(*c),
Some(Value::None) | None => None,
Some(other) => return Err(...),   // tipos inválidos continuam erro

// stroke — 6 pontos (call sites de extract_stroke)
Some(Value::None) | None => None,
Some(val) => Some(extract_stroke(val, fn_name, "stroke")?),
```

Nos loops de cell, `"stroke" => stroke = if matches!(value, Value::None)
{ None } else { Some(extract_stroke(...)?) }` e braço `Value::None =>
fill = None` no match de fill.

**`extract_stroke` não mudou** — os call sites tratam `none` antes de o
invocar; blast radius zero nos consumidores não-escopo (hlines/vlines
mantêm o erro actual).

**L0 actualizado antes do código** (Regra de Ouro): secção P726 em
`layout.md` e `structural.md`; hashes recalculados (`layout.rs` →
`39dc5feb`, `structural.rs` → `edbe2de6`).

### 2.1 Paridade mantida — o que NÃO mudou

- `stroke(paint: none)` continua erro — o vanilla também erra (medido,
  §1.1). ADR-0107: paridade ao nível "é erro"; texto diverge, mecânica
  aceite.
- Tipos inválidos (`fill: true`, `stroke: true`) continuam rejeitados —
  teste de regressão dedicado.

### 2.2 Scope-out medido — hline/vline `stroke: none`

Vanilla aceita (linha não desenhada). O cristalino guarda `Stroke`
**não-opcional** em `GridHLineElem`/`TableHLineElem` (+vlines) e o render
desenha sempre a linha (`rules/layout/grid.rs:668-689`). Aceitar `none`
exige entidade `Option<Stroke>` + salto no render — mudança de outra
espécie, sem consumidor em cetz. Atalho zero-thickness rejeitado por
design (width 0 em PDF é hairline, não "invisível"). Registado em
`achados-adiados-cetz.md`.

### 2.3 Testes (12 novos; fail-first: 11/12 FAILED antes — o 12º é o de
regressão de tipos inválidos, verde trivialmente antes do fix)

11 unitários em `stdlib/mod.rs` (block/box/grid/table × fill/stroke
none → campos `None`; cells fill+stroke none; tipos inválidos ainda
erro) + 1 E2E em `eval/tests.rs` com o padrão exacto do cetz
(`block.with(breakable: false)(fill: none, stroke: none)[x]`,
canvas.typ:111,129). Nenhum teste existente alterado.

## 3. Validação

- `cargo test -p typst-core --lib p726` → **12 passed, 0 failed**.
- `cargo test --workspace` → **0 failed** (core 3977 = 3965 de P725 + 12
  novos; infra 630; resto verde).
- `crystalline-lint --fix-hashes .` + `crystalline-lint .` → **0
  violations**.
- Documento da sonda completo (`/tmp/p726-none.typ`): cristalino agora
  **exit 0** (antes falhava na primeira linha).

## 4. `cetz` — campos fixos: cadeia de eval FECHADA; paridade de pixels aberta

```
$ time ./target/release/typst /tmp/p726-cetz.typ /tmp/p726-cetz.pdf
real    0m52,066s
Exit code: 0        ← primeira vez na cadeia P678–726
```

Vanilla: exit 0 em 0.044s (referência; o ~52s de eval cristalino é
pré-existente, medido em P724/P725).

**Diff de pixels (medido, `mutool draw -r 150`, diff via script
lab/.venv):**

| Documento | px não-brancos vanilla | px não-brancos cristalino | diff |
|---|---|---|---|
| cetz canvas (line + circle) | 1451 | **0** | 1545 px (0.071%) |
| `#curve(...)` directo | 1043 | **0** | 1043 px (0.048%) |
| `#rect(...) + #circle(...)` | 972 | 979 | ≈par |

**O eval fecha; o render de paths não.** O PDF cristalino é uma página
em branco (PNG de 10122 bytes — exactamente o tamanho da página em
branco medida em P723).

### Bug de `curve` (item da lista de controlo) — confirmado: ainda bloqueia

Reproduzido com `#curve(curve.move, curve.line, curve.close)` directo
(sem cetz): página em branco no cristalino, forma visível no vanilla.
`rect`/`circle` renderizam correctamente (979 vs 972 px) — o bug é
**específico de paths**, não de formas vectoriais em geral. Local do
render: `01_core/src/rules/layout/curve.rs` (`CurveElem` →
`FrameItem::Shape { kind: Path(...) }`, `content.rs:406-411`). É o
**único bloqueio restante** para paridade de pixels no cetz — candidato
natural a P727. Registado em `achados-adiados-cetz.md` com as medições.

Achado lateral da sonda: `line(end:)` rejeitado ("argumento nomeado
inesperado") — vanilla aceita; registado na lista (baixa prioridade,
sem consumidor em cetz).

## 5. ADRs

Grep a `00_nucleo/adr` em vigor pelos termos centrais (`fill`, `stroke`,
`none`): **nenhuma menção** — nenhuma decisão vigente rege validação de
fill/stroke. ADR-0107: `stroke(paint: none)` classificado como paridade
"é erro" (observável é a rejeição, não o texto). ADR-0108: toda a
classificação precedida de medição (§1, §4); proveniência no cabeçalho.
Nada a atualizar nas ADRs.

## 6. Critério de fecho do passo

- [x] Sonda ampla completa — block/box/rect/grid/table + cells +
  hline/vline testados individualmente contra o vanilla (§1.1-1.2).
- [x] Implementado e testado nas funções confirmadas (12 pontos;
  hline/vline scope-out medido e registado).
- [x] Sem regressão em `cargo test --workspace`.
- [x] `crystalline-lint .` limpo.
- [x] `cetz` re-testado — **exit 0 (cadeia de eval fechada)** + diff de
  pixels registado (§4); paridade visual pendente do bug de `curve`.
- [x] Grep às ADRs (§5).
- [x] Bug de render de `curve` confirmado — ainda bloqueia; isolado como
  específico de paths, local registado para P727.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p726.md`
  (hash do commit `f9ae244e7`).
