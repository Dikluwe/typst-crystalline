# Prompt L0 — `math/layout/assembly` — assembly de delimitadores grandes
Hash do Código: 0d454514

**Camada**: L1 · **Alvo**: `01_core/src/engine/math/layout/assembly.rs`
**Origem**: fatiado de `rules/math/layout.md` em **P314** (ADR-0104). Núcleo
partilhado: ver `math/layout/_comum.md`.

---

Assembly por partes para delimitadores grandes. Consome `GlyphAssembly` via
`self.metrics.vertical_glyph_assembly(c)` (P255 §2 item 2; `assembly.rs:14,20`).

**Critério**: `assembly.rs` constrói assembly de partes para delimitadores.

## P906 — `layout_assembly_horizontal` (empilhamento no eixo X)

Ver `engine/layout.md` §P906 (contexto, dados da fonte, decisão de design).

`layout_assembly` (vertical) acumula `y_cursor` por peça (`part.full_advance` menos sobreposição de
conector com a peça seguinte) e emite `FrameItem::Glyph` com `x=0, y=y_in_box` — **não é genérico
por eixo**, está hardcoded a empilhamento vertical (confirmado por leitura antes de decidir: `y_
cursor`/`total_height`/inversão topo↔fundo são todos específicos de Y). Espelho novo, não reuso:
`layout_assembly_horizontal` acumula `x_cursor` (mesma fórmula de sobreposição de conector, eixo
trocado) e emite `FrameItem::Glyph` com `x=x_cursor, y=0` — sem a inversão bottom→top (partes
horizontais assumem-se esquerda→direita na fonte, mesma convenção OpenType usada pelo vanilla, não
verificada por reordenação explícita — se uma fonte listar partes na ordem inversa, é uma limitação
partilhada com o caminho vertical, que também assume a ordem da fonte sem reordenar).

```rust
pub(super) fn layout_assembly_horizontal(
    &self,
    c: char,
    assembly: GlyphAssembly,
    _target_advance: f64,
    style: &TextStyle,
) -> MathBox
```

`ascent`/`descent` da `MathBox` resultante: `ascent = vertical_metrics(style.size, style).0`,
`descent = 0.0` — mesma convenção já usada por `layout_stretchy_delimiter` no ramo de variante única
sem mapeamento de char (`stretchy.rs`), não um valor novo inventado.

**Critério**: para `⏟`/`⏞`/`⎵`/`⎴` (assembly de 5 partes confirmado na fonte via `fontTools`),
`layout_assembly_horizontal` produz uma `MathBox` com `width` próxima do `target_advance` pedido
(dentro da granularidade de conectores) e pelo menos 2 `FrameItem::Glyph` (não 1 — testar que
realmente compôs peças, não caiu no fallback de glifo único).

## P912/P913 — Repetição de Peças Extensoras (`is_extender`)

Tanto em `layout_assembly` como em `layout_assembly_horizontal`, consome-se o parâmetro `target_advance`
(em design units) executando o algoritmo de montagem por partes do vanilla (`MAX_REPEATS = 1024`):
1. Um laço calcula quantas repetições (`repeat`) das peças marcadas com `is_extender = true` são necessárias
   para atingir ou superar `target_advance` (convertido para Pt). Peças não-extensoras aparecem exatamente 1 vez;
   peças extensoras aparecem `repeat` vezes.
2. Calcula-se a razão de espalhamento `ratio` entre sobreposição máxima de conectores e sobreposição mínima
   para ajustar a dimensão total quando `full < target`.
3. As peças são posicionadas usando a sobreposição ajustada `max_overlap - ratio * max_overlap`.

---

## P914 — Centralização no Eixo Matemático (`axis_height`)

Em `layout_assembly` (vertical):
- O `MathBox` da montagem tem seu `ascent` e `descent` ajustados em torno de `axis_height`: `ascent = axis_pt + total_height / 2`, `descent = total_height / 2 - axis_pt`.
- Cada peça de glifo é deslocada verticalmente por `shift_y = axis_pt - total_height / 2`, centralizando a montagem com o eixo matemático.

## P917 — `x_advance`/`MathBox.width` de cada peça usa `hor_advance`, nunca `full_advance`

Mesmo achado e mesma correcção de `stretchy.md` §P917 (ver lá o achado medido completo — a
causa não é a repetição de extensores, já correcta desde P913, nem a selecção de peças; é só
a métrica usada para posicionar/medir largura). `GlyphPart.full_advance`/`start_connector`/
`end_connector` são medidas ao longo do **eixo de empilhamento** (vertical em `layout_assembly`,
horizontal em `layout_assembly_horizontal`) — correctas para o cálculo de `y_cursor`/`x_cursor`,
sobreposição de conectores e `repeat`/`ratio` (P912/P913, inalterado). Não são avanço horizontal.

Em `layout_assembly`, o `FrameItem::Glyph.x_advance` de cada peça (antes: `Pt(advance_pt)`,
onde `advance_pt = part.full_advance as f64 * scale`) passa a `Pt(part.hor_advance * scale)`
(`entities/glyph_variants.md` §P917). `max_advance`/`MathBox.width` (antes: `max` dos
`advance_pt`) passa a `max` dos `part.hor_advance * scale` — a largura da caixa que contém a
montagem é a largura real da peça mais larga, não a altura da peça mais alta. A posição
vertical de cada peça (`y_cursor`, `y_in_box`) **não muda** — continua a somar
`full_advance`/conectores, eixo de empilhamento inalterado.

Em `layout_assembly_horizontal`, `full_advance` já é a medida do eixo X (empilhamento e
avanço coincidem conceptualmente), mas por uniformidade com o vanilla (`font.x_advance`
sempre, nunca a medida do eixo de esticamento — `infra/font_metrics.md` §P917) o
`FrameItem::Glyph.x_advance` de cada peça também passa a `part.hor_advance * scale`; `x_cursor`
(posição/avanço cumulativo do cursor, que determina onde a peça seguinte começa) continua a
usar `full_advance`/conectores — só o `x_advance` **reportado no FrameItem** (usado por
consumidores fora deste ficheiro, ex.: cálculo de bounding box) muda de fonte de verdade.

**Critério**: para um assembly sintético com peças de `full_advance` grande mas `hor_advance`
pequeno (caso construído no teste, não dependente de fonte real), `layout_assembly(...).width`
aproxima-se da soma/máximo dos `hor_advance` das peças, nunca dos `full_advance` — mesmo
padrão de teste sintético já usado por P913 para o algoritmo de repetição.

## P918 — laço de `repeat`/`ratio` extraído para `resolve_assembly_repeat` (interno a este ficheiro)

**Achado** (P918, achado fora do escopo original dos 5 candidatos, incorporado por decisão do
dono): o laço descrito em "P912/P913" acima (determinação de `repeat`/`ratio`, `MAX_REPEATS =
1024`) mais a reconstrução final de `parts_vec` com `repeat` cópias de cada peça extensora —
~53 linhas — é **idêntico byte-a-byte** entre `layout_assembly` e `layout_assembly_horizontal`
(confirmado por leitura directa dos dois corpos). Categoria "mecânica" (ADR-0107) — reorganização
de código comum, sem implicação de fidelidade geométrica (ADR-0123) — extraída para função privada
local `resolve_assembly_repeat(assembly: &GlyphAssembly, scale: f64, target_pt: f64) -> (Vec<&GlyphPart>,
f64)` (devolve `parts_vec` já expandido + `ratio`), chamada por ambos os métodos. Fica **neste
ficheiro** (não em `mod.rs`/`_comum.md`) — os dois consumidores já vivem no mesmo módulo, sem
partilha entre ficheiros. Comportamento inalterado; `repeat` deixa de ser retornado (só era usado
para reconstruir `parts_vec`, já devolvido expandido).

## P945 — `minConnectorOverlap` no laço `repeat`/`ratio` e no posicionamento

**Medição** (`typst-passo-945` Fase A, leitura cruzada + fontTools): o laço de
P913 e o posicionamento das peças ignoram o `minConnectorOverlap` da tabela
MATH da fonte (NewCMMath: **20du**). O vanilla
(`lab/typst-original/crates/typst-layout/src/math/fragment/glyph.rs:575-641`):
- no laço: `growable += max(0, max_overlap − min_overlap)` (linha 610) — o
  cristalino acumulava `max_overlap` inteiro;
- no posicionamento: `advance −= max_overlap; advance += ratio ×
  (max_overlap − min_overlap)` (linhas 639-640) — o cristalino aplicava
  `overlap = max_overlap × (1 − ratio)` (equivale a `min_overlap = 0`).

Efeito: com `ratio > 0` (assemblies grandes, ex.: matriz 3×3+), as juntas
ficam mais abertas que o vanilla — parte do sintoma "traços desencontrados".

**Correcção**: `GlyphAssembly` ganha `min_overlap: u16` (design units, default
0 — ver `entities/glyph_variants.md` §P945), extraído em L3 de
`math.variants.min_connector_overlap` (ver `infra/font_metrics.md` §P945) e
consumido em `resolve_assembly_repeat` e nos dois posicionamentos
(`layout_assembly` e `layout_assembly_horizontal`, mesma fórmula — o laço já é
partilhado desde P918, a fórmula de overlap do posicionamento também passa a
ser partilhada ou replicada byte-a-byte).

**Confirmado irrelevante para NCM, não implementar** (anti-deriva): o
`y_offset = bbox descent` por peça do vanilla (glyph.rs:657-659) — as peças de
assembly de NewCMMath têm `yMin = 0` (medido via fontTools BoundsPen em
`uni239B/239C/239D/239E/239F/23A0`), logo a compensação é no-op nesta fonte.

## P952b — centragem da tinta da assembly no eixo (shift correcto)

**Medição** (`typst-passo-952`): a fórmula de P914 (`shift_y = axis_pt -
total_height/2`) centrava as **baselines** das peças, não a **tinta** — como
as peças de NewCMMath têm a tinta toda acima da baseline (yMin=0), a tinta
da assembly ficava deslocada `advance_topo - 2×axis` para cima face ao
centro declarado (`axis ± total/2`), e a `ascent`/`descent` declarada não
batia com a tinta real (exposto quando a ancoragem da grelha foi corrigida
em `_comum.md` §P952b e o teste de guarda 2×2 de P945 falhou).

**Correcção**: `shift_y = -axis_pt - (total_height - advance_topo -
advance_fundo)/2` — o centro da tinta (calculado dos avanços das peças
extremas) aterra em `-axis_pt`, e a tinta final coincide com a caixa
declarada (`[-(axis+half), half-axis]`). `ascent`/`descent` da MathBox
inalterados. Limitação registada (P949): peças com `yMin < 0` (outras fontes)
não são compensadas (o vanilla compensa via `y_offset` por peça).


## P957 — baseline das peças no FUNDO do slot (não no topo): sequência de passos estava rodada

**Medição** (`typst-passo-957` Fase A; confirmação visual do dono: peça
inferior do assembly com forma de canto reto em `{`, `(` de matriz
aumentada e `(` de matriz de reticências): a identidade das peças estava
correcta (subset CFF contém os charstrings certos, cruzados por hash com o
subset do vanilla), mas a **sequência de passos entre baselines
consecutivas** saía **rodada de uma posição** face ao vanilla:

- `(` (matriz aumentada, 4 peças, NewCMMath-Book): cristalino
  [429, 376, 1426]du vs vanilla [1426, 376, 429]du.
- `{` (cases de 3 ramos, 5 peças): cristalino [555, 1308, 556, 557]du vs
  vanilla [558, 556, 1308, 556]du.

Os valores do vanilla batem exactamente com a fórmula do seu `assemble`
(`lab/typst-original/crates/typst-layout/src/math/fragment/glyph.rs:631-641`:
passo após a peça `i` = `advance_i − overlap_i + ratio×(overlap_i −
min_overlap)`, baseline da peça `i` na **origem acumulada** — fundo do slot).
O cristalino emitia `y_in_box = total_height − y_from_bottom − advance_pt`
(`assembly.rs`), colocando a baseline de cada peça no **topo** do seu slot —
o passo entre baselines consecutivas saía `advance_{i+1} − overlap_i`
(próxima peça) em vez de `advance_i − overlap_i` (peça actual) — daí a
rotação. Efeito visual: cada peça desenhada `advance_i` acima do sítio
certo; a peça inferior (gancho curvo) ficava coberta pelo extensor e o fundo
do delimitador mostrava a haste recta do extensor — o "canto reto".

**Correcção**: `y_in_box = total_height − y_from_bottom` (baseline no fundo
do slot, convenção do vanilla; as peças de NewCMMath têm `yMin = 0`, logo o
`y_offset = descent` por peça do vanilla — `glyph.rs:657-659` — é zero aqui;
a limitação P949 para fontes com `yMin < 0` mantém-se registada).

**Simplificação associada (substitui a fórmula de P952b)**: com baselines no
fundo dos slots, a tinta da assembly ocupa exactamente `[0, total_height]`
(a peça do topo tem altura de tinta = `advance` e a do fundo `yMin = 0`),
logo a centragem da tinta no eixo é simplesmente
`shift_y = −axis_pt − total_height / 2` — a tinta passa a coincidir
exactamente com a caixa declarada (`ascent = axis + total/2`,
`descent = total/2 − axis`, P914). A fórmula de P952b
(`−axis − (total − adv_topo − adv_fundo)/2`) era a compensação necessária
enquanto as baselines estavam no topo dos slots; deixa de ser usada.

**Não muda**: `resolve_assembly_repeat` (laço repeat/ratio, P913/P945), o
cálculo de overlaps com `minConnectorOverlap` (P945), `hor_advance` para
`x_advance`/largura (P917), a ordem das peças (fonte: fundo→topo), e
`layout_assembly_horizontal` (eixo X, sem `y_in_box`).
