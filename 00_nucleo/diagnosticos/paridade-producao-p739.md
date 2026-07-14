# P739 — Paridade de produção: hline/vline `stroke: none`, `line(end:)`, display de Float, NaN em `Length / Float`

**Data:** 2026-07-13 (21:51 -03:00)
**Commit base:** `5636bdf4d17540dd05325f7a3992153ff671d2b0` ("P738: preenche hash do commit no relatório")
**Commit da implementação:** `a6798f8fa7d2b09bf614513456eac6a478538255`
**Estado no momento das medições E2E:** working tree não commitado; `git diff HEAD --stat`:

```
 00_nucleo/prompts/entities/elements/grid_hline.md  |   5 +-
 00_nucleo/prompts/entities/elements/grid_vline.md  |   5 +-
 00_nucleo/prompts/entities/elements/table_hline.md |   5 +-
 00_nucleo/prompts/entities/elements/table_vline.md |   5 +-
 00_nucleo/prompts/rules/eval.md                    |  29 ++++-
 00_nucleo/prompts/rules/eval/ops.md                |   9 +-
 00_nucleo/prompts/rules/stdlib/grid_hline.md       |   1 +
 00_nucleo/prompts/rules/stdlib/grid_vline.md       |   1 +
 00_nucleo/prompts/rules/stdlib/shapes.md           |  23 +++-
 00_nucleo/prompts/rules/stdlib/table_hline.md      |   1 +
 00_nucleo/prompts/rules/stdlib/table_vline.md      |   1 +
 01_core/src/entities/content.rs                    |   8 +-
 01_core/src/entities/elements/grid_hline.rs        |   8 +-
 01_core/src/entities/elements/grid_vline.rs        |   7 +-
 01_core/src/entities/elements/table_hline.rs       |   7 +-
 01_core/src/entities/elements/table_vline.rs       |   7 +-
 01_core/src/rules/eval/bibliography.rs             |   2 +-
 01_core/src/rules/eval/closures.rs                 |   2 +-
 01_core/src/rules/eval/control_flow.rs             |   2 +-
 01_core/src/rules/eval/flow.rs                     |   2 +-
 01_core/src/rules/eval/markup.rs                   |   2 +-
 01_core/src/rules/eval/math.rs                     |   2 +-
 01_core/src/rules/eval/mod.rs                      |  13 +-
 01_core/src/rules/eval/modules.rs                  |   2 +-
 01_core/src/rules/eval/operators.rs                |  14 ++-
 01_core/src/rules/eval/rules.rs                    |   2 +-
 01_core/src/rules/eval/tests.rs                    | 133 ++++++++++++++++++++-
 01_core/src/rules/layout/grid.rs                   |  22 ++--
 01_core/src/rules/stdlib/shapes.rs                 |  59 ++++++++-
 01_core/src/rules/stdlib/structural.rs             |  24 ++--
 30 files changed, 336 insertions(+), 67 deletions(-)
```

Vanilla de referência: `lab/typst-original/target/release/typst` (build local do
Typst original). Render para pixels: `pdftoppm -png -r 150`; comparação:
`python3 /tmp/pngdiff.py` (conta pixels não-brancos e bytes com diff > 8).

---

## Parte A — `hline`/`vline` com `stroke: none` (achado P726)

### Sonda (medição prévia, ADR-0108)

`/tmp/p739a-hline.typ`:

```typst
#table(
  columns: 2,
  table.hline(stroke: none),
  [a], [b],
)
#grid(
  columns: 2,
  grid.vline(stroke: none),
  [a], [b],
)
```

- **Vanilla:** compila (exit 0). Ampliando o render a 150 dpi: a caixa da
  table aparece **sem a linha do topo** — o `hline(stroke: none)` em y=0
  remove exactamente essa posição; o resto do stroke default das células
  permanece. Morfologia medida: `stroke: none` é aceite e a linha não é
  desenhada (achado de P726 confirmado como língua, não mecânica).
- **Cristalino (antes):** o constructor rejeitava `stroke: none`
  (`expected stroke` / tipo errado) — o campo `stroke` era `Stroke`
  obrigatório nas 4 entidades (`grid_hline`, `grid_vline`, `table_hline`,
  `table_vline`).

### Decisão

Zero-thickness seria hairline em PDF (achado P726) — a representação
correcta é `Option<Stroke>`, com `None` = linha não desenhada. Mudança de
contrato interno (4 entidades + factories + constructors + trait
`LayoutHLine/LayoutVLine`), mantendo o render existente: os dois loops de
`rules/layout/grid.rs` saltam linhas com `stroke: None`
(`let Some(stroke) = h.stroke() else { continue };`).

### E2E (após)

- Cristalino compila (exit 0) e **não desenha linhas** — texto "a b"/"a b"
  sem caixa. O cristalino não desenha o stroke default das células da
  table (achado pré-existente, fora deste passo); por isso a contagem de
  pixels não-brancos diverge do vanilla (282 vs 640) — a diferença é a
  caixa default da table, **não** as linhas da sonda. No vanilla, a
  remoção exacta da linha do topo pelo `hline(stroke: none)` foi
  confirmada visualmente no crop ampliado.

---

## Parte B — `line(start:, end:)` (achado P726)

### Sonda

`/tmp/p739b-line-end.typ`: `#line(start: (0pt, 0pt), end: (50pt, 50pt))`.

- **Vanilla:** compila (exit 0) — linha diagonal de (0,0) a (50pt,50pt).
- **Cristalino (antes):** `unexpected argument: end` — a whitelist de
  named args do `native_line` só admitia `dx`, `dy`, `stroke`.

### Decisões (com medição)

- `end` aceite como array de 2 coordenadas; `dx = end.x − start.x`,
  `dy = end.y − start.y` (semântica vanilla).
- `end` combinado com `dx`/`dy` → erro; `start` sem `end` → erro.
- **Scope-out medido:** `start` ≠ `(0pt, 0pt)` — `ShapeKind::Line` do
  cristalino não carrega posição absoluta (o offset de layout é outro
  mecanismo); aceitar `start` não-zero exigiria mudar o shape de render.
  Rejeitado com erro explícito de scope-out.
- **Scope-out:** `angle:`/`length:` — não existem na interface legada do
  constructor (outra forma de especificar a linha no vanilla); a
  whitelist continua a rejeitá-los.
- Não-regressão: `dx`/`dy` directos inalterados.

### E2E (após)

Cristalino: 523 px não-brancos; vanilla: 523 px; diff 0.0481%
(anti-aliasing). Paridade de pixels.

---

## Parte C — display de Float em interpolação de markup (achado P713)

### Sonda

`/tmp/p739c-float.typ`: `#(4/2) #(1.0) #(2.5) #(0.1) #(100.0) #(1.5e3) #repr(1.0)`.

- **Vanilla (`pdftotext`):** `2 1 2.5 0.1 100 1500 1.0` — Float usa o
  Display de f64 (inteiros exactos sem `.0`); `repr` mantém `1.0`.
- **Cristalino (antes):** `2.0 1.0 2.5 0.1 100.0 1500.0 1.0` — o braço
  P545 de `eval/mod.rs` usava `repr_value` para todos os tipos,
  incluindo Float.

### Decisão

Divergência de língua (morfologia do texto produzido). O braço ganha
`Value::Float(f) => format!("{f}")` (Display de f64 do Rust — paridade
com o vanilla); os restantes tipos mantêm `repr_value`. `repr` é outro
observável (forma literal) e fica inalterado.

### E2E (após)

`pdftotext` cristalino: `2 1 2.5 0.1 100 1500 1.0` — **idêntico** ao
vanilla.

---

## Parte D — NaN em `Length / Float` (achado P725, divergência latente)

### Sonda

A dúvida prévia era se NaN era sequer **alcançável** no cristalino sem
`float.nan` (que não existe — `float.nan` erra "type float não tem
campos"). Medição: `calc.inf - calc.inf` → NaN (o cristalino expõe
`calc.inf`). Com essa via:

- **Vanilla:** `repr(1pt / (calc.inf - calc.inf))` → `0pt`.
- **Cristalino (antes):** NaN propagava (`float.nan * 1pt + ...`).

Controlo de não-regressão medido nos dois compiladores: divisão por zero
**literal** (`0.0/0.0`, `1pt/0.0`) → erro "cannot divide by zero" em
ambos — paridade exacta (o erro zero-divisor fica intacto).

### Decisão

`sanitize_length_nan` (introduzida em P725 para `Length * Float`)
estendida aos dois braços `Div` de `Length` em `operators.rs`:
componente NaN sanitiza para zero (`0pt`). O saneamento fica no braço do
eval (não em `Length::div`), por disciplina um-bug-por-passo.

### E2E (após)

`#repr(1pt / (calc.inf - calc.inf))` → `0pt` — idêntico ao vanilla.

### Achado novo registado (não corrigido)

`repr(calc.inf - calc.inf)`: cristalino → `NaN.0`; vanilla →
`float.nan`. A forma literal do NaN em `repr` diverge — item adicionado
a `achados-adiados-cetz.md`.

---

## Validação global

- `cargo test --workspace`: **4764 passed, 0 failed** (4755 em P738 + 9
  testes novos: 2 de A, 4 de B, 2 de C, 1 de D).
- `crystalline-lint .`: 0 violations (`--fix-hashes` aplicado; 12 headers
  actualizados).
- `cargo build --release`: limpo.
- Não-regressão cetz: `/tmp/p734-cetz.typ` a 150 dpi vs referência
  vanilla `/tmp/p736-van150-1.png` — 1535/1478 px não-brancos, diff
  0.1477% (B−A=+57) — **idêntico** aos números de P736/P737.

## Scope-outs declarados neste passo

- `line(start:)` com `start` ≠ `(0pt, 0pt)` — `ShapeKind::Line` não
  carrega posição absoluta.
- `line(angle:, length:)` — interface alternativa vanilla não
  materializada (whitelist rejeita).
