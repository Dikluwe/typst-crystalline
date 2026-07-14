# Prompt L0 — `infra/font_metrics` — Parser de Métricas TrueType/OpenType
Hash do Código: 55d1dcbc

**Camada**: L3
**Ficheiro alvo**: `03_infra/src/font_metrics.rs`
**Criado em**: 2026-03-28 (Passo 19)
**Atualizado em**: 2026-04-12 (restauro — expandido com math_constants, math_kern, glyph_variants, build_math_glyph_reverse_map)
**ADRs relevantes**: ADR-0019 (`ttf-parser`, `rustybuzz` → L3 exclusivo), ADR-0011 (delegação de MathClass)

---

## Contexto e Objetivo

Para que o layouter (L1) componha parágrafos e equações, precisa de dimensões
microscópicas de caracteres: largura de glyph, ascender, descender, constantes
MATH OpenType. Este módulo (L3) abre o binário `.ttf`/`.otf` via `ttf-parser`
e extrai esses dados, **convertendo-os para os tipos neutros de L1** —
`MathConstants`, `GlyphVariants`, `GlyphAssembly`, `MathGlyphKern`.

### Fronteira de Arquitectura

- `ttf_parser::Face` **não escapa** a fronteira de L3. O L1 recebe apenas:
  - `Pt` (valores em pontos tipográficos, já convertidos de design units)
  - `GlyphVariants`, `GlyphAssembly`, `MathGlyphKern` (tipos puros de L1)
  - `MathConstants` (struct de L1 com valores em design units)
- Implementa o trait `FontMetrics` (definido em L1) para injeção de
  dependências — o `MathLayouter` (L1) nunca sabe que existe `ttf-parser`.

---

## Interface

### `FontBookMetrics<'a>` — struct principal

```rust
pub struct FontBookMetrics<'a> {
    face: Face<'a>,          // ttf-parser — não escapa este módulo
    upem: f64,               // units_per_em (tipicamente 1000 ou 2048)
    glyph_to_unicode: HashMap<u16, char>,  // mapeamento reverso preemptivo
}

impl<'a> FontBookMetrics<'a> {
    /// Constrói a partir de bytes de fonte OpenType/TrueType.
    /// Retorna None se bytes inválidos ou upem == 0 (fallback: 1000.0).
    pub fn from_bytes(data: &'a [u8]) -> Option<Self>
}
```

### Implementação de `FontMetrics` (trait de L1)

```rust
impl FontMetrics for FontBookMetrics<'_> {
    /// Avanço horizontal em Pt: size * (Σ glyph_units / upem)
    /// Fallback para glifos ausentes: upem * 0.6 (largura monospace)
    fn advance(&self, text: &str, size: Pt) -> Pt

    /// Retorna (ascender_pt, line_height_pt)
    /// line_height = size * ((ascender + |descender| + line_gap) / upem)
    /// |descender|: fontes "incorrectas" devolvem negativo → .abs()
    fn vertical_metrics(&self, size: Pt) -> (Pt, Pt)

    /// Distância da baseline ao topo das maiúsculas (cap-height) em Pt.
    /// Usado pelo layout para posicionar a primeira baseline a
    /// `margem + cap-height` (paridade com vanilla `top-edge: cap-height`).
    /// Fallback: se `face.capital_height()` for None ou ≤ 0, usa o ascender.
    fn cap_height(&self, size: Pt) -> Pt

    /// Variantes verticais extensíveis para um caractere (ex: '(', '[', '√')
    /// Retorna GlyphVariants::default() se a fonte não tem tabela MATH
    fn vertical_glyph_variants(&self, c: char) -> GlyphVariants

    /// Assembly por partes para delimitadores extensíveis grandes
    fn vertical_glyph_assembly(&self, c: char) -> GlyphAssembly

    /// Constantes globais da tabela OpenType MATH.
    /// Fallback para MathConstants::fallback() se a tabela não existir.
    fn math_constants(&self) -> MathConstants

    /// Kern matemático para os 4 quadrantes de um glyph (top-right/left, bottom-right/left)
    /// Retorna MathGlyphKern::default() se não houver tabela kern_infos
    fn math_kern(&self, c: char) -> MathGlyphKern

    /// Resolve glyph_id → char base (para ToUnicode no PDF).
    /// Usa o mapeamento pré-computado em from_bytes.
    fn glyph_to_char(&self, glyph_id: u16) -> Option<char>
}
```

---

## Funções Internas Chave

### `extract_variants(face, c)` → `GlyphVariants`

Acede a `face.tables().math.variants.vertical_constructions.get(glyph_id)`.
Mapeia cada `ttf_parser::math::GlyphVariantRecord` → `GlyphVariant { glyph_id, advance }`.
Retorna `GlyphVariants::default()` se qualquer nível da cadeia for `None`.

### `extract_assembly(face, c)` → `GlyphAssembly`

Acede à `assembly` da construção vertical e mapeia cada
`ttf_parser::math::GlyphPart` → `GlyphPart { glyph_id, start_connector, end_connector, full_advance, is_extender }`.

### `build_math_glyph_reverse_map(face)` → `HashMap<u16, char>`

Itera sobre `STRETCHY_BASES = ['(', ')', '[', ']', '{', '}', '|', '√']`.
Para cada base, recolhe variantes e peças de assembly.
Extensores mapeiam para `'|'`. Usa `or_insert` para não sobrescrever
se uma peça for partilhada por vários caracteres base.
**Usado por `export.rs`** para incluir glifos matemáticos no ToUnicode do PDF.

### `math_constants` — conversão de unidades

Todos os campos `MathLeadingValue { value: i16 }` do `ttf-parser` são
convertidos para `f64` (design units). A conversão para Pt é feita pelo
caller via `MathConstants::to_pt(value, font_size)` no L1.

### `math_kern` — leitura de tabela kern por quadrante

A tabela tem `n` alturas e `n+1` valores de kern:
- `kern[i]` aplica-se a alturas ≤ `height[i]`
- `kern[n]` (último, sem `correction_height`) aplica-se a tudo acima

```rust
for i in 0..count {
    height = kern.height(i).map(|v| v.value as f64)   // Some(h) ou None
    kv     = kern.kern(i).map(|v| v.value as f64)
    records.push(MathKernRecord { correction_height: height, kern_value: kv })
}
// último: correction_height: None
```

---

## Invariantes

| Invariante | Detalhe |
|-----------|---------|
| `upem == 0` | Fallback para `1000.0` em `from_bytes` — nunca divide por zero |
| `descender` pode ser negativo | Usar `.abs()` antes de somar |
| Fallback de advance | Glifos ausentes: `upem * 0.6` (monospace) |
| `ttf-parser` não escapa | L1 nunca importa `ttf_parser::*` |
| `script_percent_scale_down` | Convertido de `%` inteiro → `f64` dividindo por `100.0` |

---

## Critérios de Verificação

```
// from_bytes
FontBookMetrics::from_bytes(b"not a font") = None
FontBookMetrics::from_bytes(b"")           = None

// advance proporcional (fonte proporcional, fixture .ttf real)
advance("iiii", 12pt) < advance("WWWW", 12pt)
advance("A", 12pt) ∈ [3pt, 12pt]

// vertical_metrics
ascender_pt > 0
line_height > ascender_pt
line_height em 12pt < 24pt
// escala com font_size:
|vertical_metrics(24pt).line_height - 2 × vertical_metrics(12pt).line_height| < 0.5pt

// math_constants (tabela MATH real via fixture)
upem > 0
fraction_rule_thickness > 0
script_percent_scale_down ∈ (0.0, 1.0]
// fallback:
MathConstants::fallback().axis_height = 500.0

// math_kern
// fonte sem tabela kern → MathGlyphKern::default() (todos isEmpty)

// glyph_to_char
// char em STRETCHY_BASES → glyph_id mapeado de volta ao char base
// extensor → '|'
```

---

## `FallbackFontMetrics<'a>` — métricas com fallback multi-script

### Motivação

`FontBookMetrics` assume uma única fonte por instância. O layout de texto
normal (P544) precisa de medir strings que podem conter caracteres de
múltiplos scripts, recorrendo às fontes primárias declaradas no estilo e,
se necessário, a um conjunto padrão de fallbacks. Esta struct resolve o
mesmo conjunto de candidatos que o shaper (`03_infra/src/shaper.rs`) e
mede cada caractere com a face correcta.

### Interface

```rust
pub struct FallbackFontMetrics<'a> {
    world: &'a dyn World,
    cache: Arc<Mutex<HashMap<usize, Arc<CachedFace>>>>,
    shaped_width_cache: Arc<Mutex<HashMap<ShapedWidthKey, Pt>>>,
    shaper_face_cache: Arc<Mutex<crate::shaper::FaceCache>>,
    advance_width_cache: Arc<Mutex<HashMap<AdvanceWidthKey, Pt>>>,
}

impl<'a> FallbackFontMetrics<'a> {
    /// Constrói a partir de um `World`. As caches são lazy e partilhadas
    /// entre clones (fixpoint loop de TOC, etc.).
    pub fn new(world: &'a dyn World) -> Self
}
```

Implementa `FontMetrics` (trait de L1) e `Clone`. O clone partilha as mesmas
caches, de modo que faces parseadas e larguras medidas num layouter são
reutilizadas em layouters posteriores.

**P677** — `advance_width_cache`: guarda resultados de `advance(text, style)`
para evitar re-medir o mesmo texto+estilo repetidamente no layout (palavras e
espaços repetidos). A chave inclui texto, tamanho, fonte, variações de eixo e
outros campos do `TextStyle` que afectam métricas; `tracking` é aplicado fora
do cache em `text_width`, pelo que não entra na chave.

### `CachedFace` — cache do `Face` parseado

```rust
struct CachedFace {
    data: Font,            // bytes owned (Vec<u8> wrapper de L1)
    face: Face<'static>,   // ttf-parser — empresta dos bytes internos
}
```

**Decisão de desenho (lifetime):** `Face<'a>` exige bytes com lifetime `'a`.
Em vez de reconstruir o `Face` a cada chamada, guardamos os bytes e o
`Face` juntos numa struct alocada em `Arc` (estabilidade no heap). O
`Face` é criado com um slice `'static` obtido via `std::slice::from_raw_parts`
sobre o ponteiro dos bytes.

**Justificativa do `unsafe`:**
- O `Font` (campo `data`) nunca é movido depois de `CachedFace` construído
  (a struct vive dentro de `Arc`, logo a alocação no heap é estável).
- O `Face<'static>` não escapa do módulo como `'static`; é devolvido apenas
  como `&Face<'_>` em métodos internos.
- Este é o mesmo padrão usado pelo Typst vanilla em
  `typst-library/src/text/font/mod.rs` (`FontInner { ttf, data }`).

A cache é indexada por `slot_idx` do `FontBook`. Cada fonte é parseada uma
única vez por instância de `FallbackFontMetrics`; chamadas subsequentes a
`advance`, `resolve_primary` e `covering` reutilizam o `CachedFace`.

### Resolução de fontes

- `resolve_primary(style)`: fontes declaradas em `style.font`, na ordem;
  se nenhuma resolver, usa a lista de fallback apropriada à classe da primeira
  família (serif vs sans), consistente com `shaper.rs` (P555). Se não for
  possível inferir a classe, usa a lista sans.
- `covering(c, primary)`: procura a primeira fonte que cobre o caractere
  (primárias primeiro, depois todo o `FontBook`), usando o `Face` cacheado.

### Kerning na medição de largura

**Decisão:** aplicar kerning a partir das tabelas legacy `kern` e `kerx`
do TrueType/OpenType.

**Razão:** o shaper (`rustybuzz`) aplica kerning por omissão. A medição
anterior somava apenas `glyph_hor_advance`, o que reservava mais espaço do
que o shaping usava para pares com kerning negativo (ex.: `Te`, `To`, `Ty`,
`Ye` em DejaVu Sans). O excesso aparecia como espaço real no PDF e era
extraído pelo `pdftotext`.

**Algoritmo:**
- Itera caractere a caractere, resolvendo a fonte para cada um.
- Para o par `(anterior, actual)` na mesma fonte, consulta as subtables
  `kern` e depois `kerx`; se encontrar valor, adiciona-o (em design units)
  ao total, convertido para `Pt` com `units_per_em` da fonte.
- Pares que mudam de fonte não aplicam kerning (o shaper parte em sub-runs
  separados, onde o kerning inter-fonte não existe).

**Limitação conhecida:** kerning via GPOS (`GPOS` lookup type 2, feature
`kern`) não é consultado nesta iteração. A maioria das fontes com kerning
inclui também a tabela legacy `kern`; quando não incluir, a medição pode
continuar ligeiramente acima do shaping. Esta limitação é aceite e
registada para extensão futura.

### Invariantes adicionais

| Invariante | Detalhe |
|-----------|---------|
| Zero re-parses por carácter | Cada `slot_idx` é parseado no máximo uma vez por instância |
| `CachedFace` estável | Alocado em `Arc`; bytes não são movidos após criação do `Face` |
| Kerning intra-fonte | Só aplica entre glifos da mesma fonte candidata |
| Fallback de largura | Glifo ausente: `size * 0.6` (mesmo valor de `FontBookMetrics` proporcional) |
| `ttf-parser` não escapa | L1 continua a receber apenas `Pt` e tipos puros |

### Critérios de Verificação adicionais

```
// Cache — sem re-parse redundante
resolve_primary(style) seguido de advance(text) não chama Face::parse
mais do que uma vez por slot_idx

// Kerning — casos de P546
advance("Texto", 12pt) medido ≈ largura shaped (sem corte em pdftotext)
advance("Type", 12pt) < advance("Ti" em espaçamento)  // kerning aplicado

// Sem regressão
advance("Ti", 12pt) continua correcto
vertical_metrics(12pt) retorna valores positivos e escaláveis
```

---

## Histórico de Revisões

| Data | Motivo | Ficheiros afetados |
|------|--------|--------------------|
| 2026-03-28 | Criação — Passo 19: `advance`, `vertical_metrics`, `from_bytes` | `font_metrics.rs` |
| 2026-04-12 | Restauro — expandido: `math_constants`, `math_kern`, `vertical_glyph_variants`, `vertical_glyph_assembly`, `glyph_to_char`, `build_math_glyph_reverse_map` | `font_metrics.md` |
| 2026-07-03 | P548 — documentação de `FallbackFontMetrics`, cache do `Face` parseado e kerning via tabelas `kern`/`kerx` | `font_metrics.md`, `font_metrics.rs` |
| 2026-07-10 | P677 — adicionada `advance_width_cache` para reutilizar larguras `advance` entre chamadas do layout | `font_metrics.md`, `font_metrics.rs` |
