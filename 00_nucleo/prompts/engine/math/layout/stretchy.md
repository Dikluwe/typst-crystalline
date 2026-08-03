# Prompt L0 — `math/layout/stretchy` — operadores extensíveis
Hash do Código: efba337a

**Camada**: L1 · **Alvo**: `01_core/src/engine/math/layout/stretchy.rs`
**Origem**: fatiado de `rules/math/layout.md` em **P314** (ADR-0104). Núcleo
partilhado: ver `math/layout/_comum.md`.

---

Operadores extensíveis. Consome `GlyphVariants` via
`self.metrics.vertical_glyph_variants(c)` com `.select(min_advance)`
(P255 §2 item 2; `stretchy.rs:22`).

**Critério**: `stretchy.rs` selecciona variant via `select(min_advance)`.

## P906 — `layout_stretchy_glyph_horizontal` (esticamento no eixo X)

Ver `engine/layout.md` §P906 (contexto, dados da fonte, decisão de design).

Segundo método em `stretchy.rs`, espelhando `layout_stretchy_delimiter` — mesmo algoritmo
(variante única via `select_with_advance`; se insuficiente, `horizontal_glyph_assembly` via
`layout_assembly_horizontal`, ver `assembly.md` §P906; sem variantes nem assembly, fallback ao
glifo base via `layout_text_node`), só troca `vertical_glyph_variants`/`vertical_glyph_assembly`
por `horizontal_glyph_variants`/`horizontal_glyph_assembly` e `min_height_du` por `min_width_du`.
Adicionado ao mesmo ficheiro (não um ficheiro novo) — é a mesma unidade conceptual
("esticar um glifo extensível"), só o eixo muda; ADR-0109 não exige separação por eixo.

```rust
pub(super) fn layout_stretchy_glyph_horizontal(
    &self,
    c: char,
    min_width_du: f64,
    style: &TextStyle,
) -> MathBox
```

**Chamador**: `layout_underover`/`layout_accent` (`math/layout/mod.rs`) — ver `_comum.md` §P906.
`min_width_du` = largura da base convertida para design units (`base_box.width * upem /
style.size.val()`), mesma conversão já usada em `layout_root` para `min_height_du`.

**Critério**: para um char com dados horizontais na fonte (`⏟`/`⏞`/`⎵`/`⎴`/`hat`/`tilde`
combinantes), `layout_stretchy_glyph_horizontal(c, min_width_du, style).width >= min_width_du`
convertido para pt (dentro da tolerância normal de granularidade discreta de variantes/assembly);
para um char sem dados horizontais, comportamento idêntico ao `layout_node` anterior (fallback
`layout_text_node`, sem regressão).

## P912 — Subtração de DELIM_SHORT_FALL (0.1em)

Tanto em `layout_stretchy_delimiter` como em `layout_stretchy_glyph_horizontal`, subtrai-se
`DELIM_SHORT_FALL = 0.1em` (`0.1 * upem` em design units) da dimensão alvo antes de consultar
`variants.select_with_advance(target_du)`. Esto permite que uma variante ligeiramente menor
que a dimensão estrita seja selecionada se estiver dentro do raio de 0.1em.

## P918 — só a subtração de `DELIM_SHORT_FALL` é duplicação real; o resto NÃO se unifica

**Achado** (P918 Fase A, leitura directa dos dois corpos — revisão de uma primeira leitura mais
grosseira que os tinha marcado como "quase idênticos"): as duas linhas do bloco P912 acima
(`short_fall_du = 0.1 * upem`; `target_du = (min_*_du - short_fall_du).max(0.0)`) são idênticas
byte-a-byte nos dois métodos e foram extraídas para uma função privada local
`apply_delim_short_fall(target_du: f64, upem: f64) -> f64`. **O resto dos dois métodos NÃO foi
unificado** — divergem de propósito, não por descuido: `layout_stretchy_delimiter` centra a
caixa em `axis_height` (`shift_y`, ver P914 abaixo), `layout_stretchy_glyph_horizontal` assenta
na baseline normal via `vertical_metrics` (sem `shift_y`). Forçar uma função partilhada para o
resto teria exigido esconder essa diferença atrás de parâmetros/closures — exactamente o risco
que a ADR-0123 nomeia (confundir duas convenções geométricas por semelhança superficial de
estrutura de controlo). Mantidos como dois métodos completos, só as 2 linhas comprovadamente
idênticas são partilhadas.

---

## P914 — Centralização no Eixo Matemático (`axis_height`)

Em `layout_stretchy_delimiter`:
- O `MathBox` do delimitador tem seu `ascent` e `descent` ajustados em torno de `axis_height`: `ascent = axis_pt + height / 2`, `descent = height / 2 - axis_pt`.
- O glifo do delimitador vertical (variante ou montagem) é deslocado por `shift_y = axis_pt - height / 2`, alinhando perfeitamente o centro do delimitador com o eixo matemático.

## P917 — `x_advance`/largura da caixa usa `hor_advance`, nunca `advance_du`

**Achado (medido, não hipótese)**: instrumentação directa (`eprintln!`, revertida) com a fonte
real do pipeline (`NewCMMath-Regular.otf`, embutida via `typst_assets`) nos 4 casos fixos de
P911/P916 confirmou que `vertical_glyph_variants` não é vazia e `select_with_advance` escolhe a
variante correcta e crescente com o conteúdo (`(1/2)` → advance 1793; `(1/2/3/4)` → 2991,
tecto das 8 variantes) — as três hipóteses do L0 antigo de P917 (lista vazia, fórmula do alvo,
ordem variante-vs-assembly) **não se confirmaram**. A causa real: no ramo "sem mapeamento —
emitir como Glyph" de `layout_stretchy_delimiter`, `x_advance` era computado como
`style.size * (advance_du / upem)` — `advance_du` é `GlyphVariant.advance`, a medida ao longo
do eixo de esticamento (**altura**, para construções verticais), reaproveitada como avanço
**horizontal**. Confirmado quantitativamente contra `fontTools`/`hmtx` na fonte real: para
`parenleft.v4` (variante seleccionada em `(1/2)`), `advance` = 1793 du (→ 19.72pt a 11pt, usado
antes) vs. `hor_advance` real = 597 du (→ 6.57pt) — o valor correcto, confirmado também pelo
`/Widths` da fonte embutida no PDF exportado (`mutool trace` → `adv=".597"`). O bug não afecta
QUAL glifo é desenhado (a selecção sempre esteve correcta) — afecta só o espaço horizontal
reservado para ele, criando um gap grande a seguir a cada delimitador esticado.

**Correcção**: `x_advance` (e `MathBox.width`) no ramo "emitir como Glyph" usa
`GlyphVariant.hor_advance` (avanço nativo do glifo, `entities/glyph_variants.md` §P917), nunca
`advance`/`advance_du`. `advance_du` continua a ser usado **só** para `height_pt`/`ascent`/
`descent`/`shift_y` (eixo vertical, correcto) e para o argumento de `select_with_advance`
(decisão de qual variante é grande o suficiente) — nada nesses usos muda.

```rust
if let Some(variant) = variants.select_variant(target_du) {
    // variant: &GlyphVariant — select_variant substitui select_with_advance quando o
    // caller precisa de hor_advance além de (glyph_id, advance); ver entities/glyph_variants.md.
    let height_pt = style.size.val() * (variant.advance / self.constants.upem);
    // ...ascent/descent/shift_y inalterados, a partir de height_pt...
    let x_advance = style.size * (variant.hor_advance / self.constants.upem);
    // MathBox.width = x_advance.val() — não advance_du
}
```

**Critério**: para `(1/2)` com a fonte de produção, `layout_stretchy_delimiter('(', ...).width`
aproxima-se do `hor_advance` nativo da variante seleccionada (dentro de arredondamento de
ponto flutuante), nunca da sua `advance` (medida de altura) — teste com métricas reais
(`fontTools` como oráculo), não `FixedMetrics`/stub sem dados de `hor_advance`.

`layout_stretchy_glyph_horizontal` (P906, eixo X) recebe a mesma correcção por uniformidade
(ver `infra/font_metrics.md` §P917) — ainda que o desvio numérico seja tipicamente pequeno
nesse eixo, a fonte de verdade passa a ser sempre `hor_advance`, nunca a medida do eixo de
esticamento, em ambos os métodos deste ficheiro.

## P952b — variante de delimitador centrada pela tinta real, não por metade simétrica

**Medição** (`typst-passo-952`, teste de guarda P945 2×2 a falhar após a
correcção de ancoragem da grelha): o caminho de variante de
`layout_stretchy_delimiter` assumia tinta simétrica na variante
(`half_h = advance/2` acima e abaixo da baseline do glifo), mas a tinta real
das variantes de NewCMMath é assimétrica (`parenleft.v4`: 1146du acima vs
646du abaixo). A grelha (agora ancorada como o vanilla, `_comum.md` §P952b)
ficava a descoberto em baixo. O vanilla usa as métricas reais da variante
(`update_glyph` → bbox real) e centra a tinta no eixo (`center_on_axis`:
`baseline = h/2 + axis`, `table.rs:186-188`).

**Correcção**: novo método `FontMetrics::glyph_ink_bounds(glyph_id, size,
style) -> (Pt, Pt)` (acima/abaixo da baseline, em pt — `infra/font_metrics.md`
§P952b; default `cap_height`/0 para compatibilidade). O braço `Glyph` do
caminho de variante passa a usar a tinta real:
`shift_y = (up − down)/2 − axis_pt`, `ascent = (up+down)/2 + axis_pt`,
`descent = (up+down)/2 − axis_pt` — o centro da tinta aterra no eixo, como o
vanilla. O braço com mapeamento Unicode (`layout_text_node`) mantém a
convenção anterior (o shaper renderiza o glifo base; limitação registada).
Assemblies não são afectadas (peças NewCMMath têm `yMin = 0` — validado em
P949).
