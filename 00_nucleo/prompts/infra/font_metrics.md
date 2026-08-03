# Prompt L0 — `infra/font_metrics` — Parser de Métricas TrueType/OpenType
Hash do Código: d77eab5e

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
    fn advance(&self, text: &str, size: Pt, style: &TextStyle) -> Pt

    /// Retorna (ascender_pt, line_height_pt)
    /// line_height = size * ((ascender + |descender| + line_gap) / upem)
    /// |descender|: fontes "incorrectas" devolvem negativo → .abs()
    /// Recebe `style` para que implementações com resolução de fonte (ex:
    /// `FallbackFontMetrics`) usem a face efectivamente renderizada.
    fn vertical_metrics(&self, size: Pt, style: &TextStyle) -> (Pt, Pt)

    /// Distância da baseline ao topo das maiúsculas (cap-height) em Pt.
    /// Usado pelo layout para posicionar a primeira baseline a
    /// `margem + cap-height` (paridade com vanilla `top-edge: cap-height`).
    /// Fallback: se `face.capital_height()` for None ou ≤ 0, usa o ascender.
    /// Recebe `style` para resolução da face correcta em implementações L3.
    fn cap_height(&self, size: Pt, style: &TextStyle) -> Pt

    /// **P813** — limites de tinta do texto `(ascent, descent)` em Pt,
    /// ambos ≥ 0, medidos da união das bboxes reais dos glyphs.
    fn text_ink_bounds(&self, text: &str, size: Pt, style: &TextStyle) -> (Pt, Pt)

    /// **P922** — limites de tinta do texto `(top, bottom)` em Pt, com
    /// sinal. `top` positivo = acima da baseline; `bottom` positivo = abaixo.
    /// Necessário para acentos cujo ink fica inteiramente acima da baseline
    /// (combining marks), onde `bottom` é negativo.
    fn text_ink_bounds_signed(&self, text: &str, size: Pt, style: &TextStyle) -> (Pt, Pt)

    /// Variantes verticais extensíveis para um caractere (ex: '(', '[', '√')
    /// Retorna GlyphVariants::default() se a fonte não tem tabela MATH
    fn vertical_glyph_variants(&self, c: char) -> GlyphVariants

    /// Assembly por partes para delimitadores extensíveis grandes
    fn vertical_glyph_assembly(&self, c: char) -> GlyphAssembly

    /// Constantes globais da tabela OpenType MATH.
    /// Fallback para MathConstants::fallback() se a tabela não existir.
    /// **P893** — `style` para que `FallbackFontMetrics` resolva qual face
    /// activa fornece as constantes (mesmo motivo de `math_kern`, P891).
    fn math_constants(&self, style: &TextStyle) -> MathConstants

    /// Kern matemático para os 4 quadrantes de um glyph (top-right/left, bottom-right/left)
    /// Retorna MathGlyphKern::default() se não houver tabela kern_infos
    /// **P891** — `style` para que `FallbackFontMetrics` resolva a face activa.
    fn math_kern(&self, c: char, style: &TextStyle) -> MathGlyphKern

    /// Resolve glyph_id → char base (para ToUnicode no PDF).
    /// Usa o mapeamento pré-computado em from_bytes.
    fn glyph_to_char(&self, glyph_id: u16) -> Option<char>
}
```

---

## Funções Internas Chave

### `extract_variants(face, c)` → `GlyphVariants`

Acede a `face.tables().math.variants.vertical_constructions.get(glyph_id)`.
Mapeia cada `ttf_parser::math::GlyphVariantRecord` → `GlyphVariant { glyph_id, advance,
hor_advance }`. `advance` vem de `record.advance_measurement` (eixo de esticamento, só para
`select`); `hor_advance` vem de `face.glyph_hor_advance(record.variant_glyph).unwrap_or(0) as
f64` (hmtx, avanço horizontal nativo do glifo — **P917**, ver `entities/glyph_variants.md`
§P917). Retorna `GlyphVariants::default()` se qualquer nível da cadeia for `None`.

### `extract_assembly(face, c)` → `GlyphAssembly`

Acede à `assembly` da construção vertical e mapeia cada
`ttf_parser::math::GlyphPart` → `GlyphPart { glyph_id, start_connector, end_connector,
full_advance, is_extender, hor_advance }`. `hor_advance` vem de
`face.glyph_hor_advance(part.glyph_id).unwrap_or(0) as f64` — mesmo motivo de
`extract_variants` acima (**P917**).

**P917 — `extract_variants_horizontal`/`extract_assembly_horizontal` (abaixo, §P906) ganham o
mesmo campo `hor_advance`, populado da mesma forma** (`face.glyph_hor_advance(glyph_id)`).
Mesmo para construções horizontais — onde `advance`/`full_advance` (eixo X) coincide
conceptualmente com o avanço nativo na maioria das fontes — o vanilla (`glyph.rs:293`) lê
sempre `font.x_advance(glyph_id)` para posicionar, nunca a medida do eixo de esticamento;
`hor_advance` uniformiza os quatro extractores com essa fonte única de verdade, em vez de só
corrigir o eixo vertical (onde o desvio é grande) e deixar o horizontal a depender de uma
coincidência não garantida pela spec OpenType MATH.

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

**P915** — `math_constants_from_face` ganha a leitura de
`superscript_shift_up_cramped` (`ttf_parser::math::Constants::
superscript_shift_up_cramped().value`), mesmo mecanismo dos outros 14
campos — método já existia em `ttf_parser` 0.25 (confirmado antes de
propor o campo novo), só nunca tinha sido consumido pelo cristalino. Ver
`entities/math_constants.md` §P915.

**P920** — `math_constants_from_face` ganha a leitura de mais 2 campos,
mesmo mecanismo, métodos já existentes em `ttf_parser` 0.25 (confirmado em
`ttf-parser-0.25.1/src/tables/math.rs:378,390`, nenhum campo novo na
crate externa): `fraction_numerator_shift_up`
(`Constants::fraction_numerator_shift_up().value`),
`fraction_denominator_shift_down`
(`Constants::fraction_denominator_shift_down().value`). Ver
`entities/math_constants.md` §P920 para a fórmula que os consome.

**P922** — `math_constants_from_face` ganha a leitura de
`accent_base_height` (`Constants::accent_base_height().value`) e,
adicionalmente, `flattened_accent_base_height`
(`Constants::flattened_accent_base_height().value`). Ambos os métodos já
existiam em `ttf_parser` 0.25 (`ttf-parser-0.25.1/src/tables/math.rs:214-221`).
`accent_base_height` é consumido pela fórmula real do gap de acento do
vanilla (`typst-layout/src/math/accent.rs:56-65`). `flattened_accent_base_height`
só é usada para decidir a variante "flattened" do acento quando a base é
muito alta — funcionalidade não implementada neste passo, mas o campo é
adicionado para paridade de dados. Ver `entities/math_constants.md` §P922.

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

### `text_ink_bounds` / `text_ink_bounds_signed` — limites de tinta reais

Ambos os métodos leem a bounding box de cada glyph (`face.glyph_index(c)` +
`face.glyph_bounding_box(gid)`) e convertem `y_max`/`y_min` de design units
para pontos.

**`text_ink_bounds` (P813)** — contrato `>= 0`:
```rust
ascent = max(0.0, size * (y_max / upem))
descent = max(0.0, size * (-y_min / upem))
```
Usado para o tamanho de caixa de texto math e para a extensão agregada da
equação.

**`text_ink_bounds_signed` (P922)** — contrato com sinal, para geometria de
acentos:
```rust
top = size * (y_max / upem)       // positivo = acima da baseline
bottom = size * (y_min / upem)    // positivo = abaixo da baseline
```
Note que `bottom` usa `y_min` directamente (não `-y_min`). Para um combining
mark como `hat`/`tilde`, `y_min > 0` (tinta acima da baseline), logo `bottom`
é **negativo** — exactamente o sinal que a fórmula do vanilla espera para
`accent.descent()`. Fallback para stubs sem bbox: `(cap_height(size, style),
Pt(0.0))` (mesmo default conservador de `text_ink_bounds`, agora interpretado
como `top >= 0`, `bottom <= 0`).

Em `FallbackFontMetrics`, ambos os métodos usam a mesma resolução de face
(`resolve_primary_with_math_fallback` + `covering`) para garantir que o acento
é medido na mesma fonte MATH que o shaper/PDF efectivamente usa.

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

// vertical_metrics (style = TextStyle::default())
ascender_pt > 0
line_height > ascender_pt
line_height em 12pt < 24pt
// escala com font_size:
|vertical_metrics(24pt, &style).line_height - 2 × vertical_metrics(12pt, &style).line_height| < 0.5pt

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

### `typo_metrics(face)` — métricas verticais tipográficas

Função auxiliar privada em `03_infra/src/font_metrics.rs` que devolve
`(ascender, descender_abs, line_gap)` em unidades de design da face,
preferindo os valores tipográficos do OS/2 (`sTypoAscender`,
`sTypoDescender`, `sTypoLineGap`) quando pelo menos um de
`sTypoAscender`/`sTypoDescender` é não-zero. Caso contrário, cai para as
métricas do `hhea` (`ascender`, `descender`, `line_gap`). O descender é
devolvido como valor absoluto.

Usada tanto por `FontBookMetrics::vertical_metrics` como por
`FallbackFontMetrics::vertical_metrics` para garantir consistência na
forma como as métricas verticais são lidas das faces.

### `FallbackFontMetrics::vertical_metrics` — resolução por estilo

`vertical_metrics(size, style)` resolve a fonte primária declarada no
`style` (ou a lista de fallback do shaper se não houver fonte
explicitamente declarada) e lê as métricas da primeira face que resolve.
Isto alinha o `line_height` usado pelo layout com a face que o shaper e
o PDF efectivamente usarão para renderizar, evitando diferenças
verticais quando a primeira fonte do `FontBook` não coincide com a fonte
do estilo (P760).

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
- `covering(c, primary, variant)`: primárias primeiro (P912: quando `style.math` é
  verdadeiro / na resolução de candidatos, realiza um primeiro passe procurando
  qualquer face primária que possua a tabela OpenType MATH (`face.tables().math.is_some()`)
  e cubra `c`; se encontrada, devolve essa face imediatamente; caso contrário,
  retorna a primeira face que cobre `c`);
  se nenhuma cobrir, recolhe os candidatos devolvidos por
  `self.world.candidates_for_char(c)` (P880 — `SystemWorld` calcula a
  cobertura lazy; `MockWorld`s usam o `FontBook` directamente) e escolhe via
  `FontBook::select_fallback(like, variant, ids)` (P838 — scoring de
  similaridade do vanilla; `like` = `FontInfo` da primeira primária). Antes
  de P838 era a primeira por ordem de índice. Isto alinha a fonte usada na
  medição com a usada no shaping (que aplica o mesmo scoring), eliminando
  divergências de geometria (glifos com tamanho/errado e overlap de palavras
  medidos em P831).

  **P942** — os candidatos de `candidates_for_char` (coverage exacta, runs de
  codepoints) já são definitivos: `covering` **não** carrega a face de cada
  candidato para re-verificar `glyph_index(c)`. A versão anterior fazia essa
  re-verificação (carregava dezenas de faces `.ttc` de CJK, ~10-20 MB cada),
  medida como o custo dominante do fallback de layout (~480 ms em `utf8-cjk`,
  `layout_ms` 728→25 ms após a correcção). O vanilla confia na coverage em
  `select_fallback` (`book.rs:106-114`) e não re-verifica. Só a face da fonte
  vencedora é carregada (para `units_per_em`).

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

### Variação de eixo aplicada à medição (P772o)

**Achado (P772m/P772o):** `advance()` lia sempre `glyph_hor_advance` da
face **por omissão** (`CachedFace::face()`, nunca variada) —
independentemente do `wght`/`ital` pedido no `TextStyle`. A chave de
cache (P659, `advance_width_key`) já diferenciava por `axis_hash`, mas o
próprio cálculo nunca aplicava essas coordenadas à face. Para fontes
variáveis cujo desenho muda visivelmente o avanço dos glifos ao longo do
eixo de peso, isto desalinhava a largura usada pelo *layout* (posição de
cada palavra/espaço, sempre à instância por omissão) da largura realmente
desenhada pelo *shaper* (`03_infra/src/shaper.rs`, que já aplica
`rb_face.set_variations` correctamente). O erro acumulava carácter a
carácter e podia exceder a largura do espaço entre palavras — medido e
confirmado em `Ubuntu Sans` (eixo `wght` 100–800): a partir de peso ≈770
o erro acumulado excedia o espaço, colando palavras (`P772m`, `P772o`).

**Correcção:** dentro do loop carácter-a-carácter de `advance()`, clona-se
a `Face` cacheada (`Face: Clone`, ~2KB, sem reler bytes — não é um
re-parse) e aplica-se `face.set_variation(tag, value)` para cada entrada
de `axis_variations_for_font_variant(&text_style_to_font_variant(style))`
— as mesmas coordenadas já usadas por `shaper.rs` e já usadas para
diferenciar a chave de cache (P659). `set_variation` devolve `None`
silenciosamente para fontes não-variáveis ou eixos ausentes — sem efeito,
comportamento preservado para o caso não-variável.

**Verificado, não corrigido nesta ronda:** o mesmo padrão (`cached_face()`
sem variação aplicada) está presente em `vertical_metrics`/`cap_height`/
outras leituras de métricas verticais desta struct — não confirmado se
têm o mesmo tipo de impacto observável (a lacuna original media-se em
largura horizontal, não altura de linha); não alterado neste passo por
falta de medição própria (ADR-0108).

**Verificado e não resolvido:** o mesmo teste com `Cantarell-VF.otf`
(fonte variável CFF2/HVAR, ao contrário do `glyf`/`gvar` de `Ubuntu
Sans`) continua a colar palavras no peso 800 **mesmo com a correcção
acima** — a medição de `advance()` para essa fonte já bate com o valor
esperado (`fontTools.varLib.instancer`, confirmado por instrumentação),
mas o resultado renderizado continua sobreposto. Causa não isolada nesta
ronda — ver `paridade-producao-p772o.md` §5.

### Invariantes adicionais

| Invariante | Detalhe |
|-----------|---------|
| Zero re-parses por carácter | Cada `slot_idx` é parseado no máximo uma vez por instância (`face.clone()` em P772o clona a estrutura já parseada, não relê bytes) |
| `CachedFace` estável | Alocado em `Arc`; bytes não são movidos após criação do `Face` |
| Kerning intra-fonte | Só aplica entre glifos da mesma fonte candidata |
| Fallback de largura | Glifo ausente: `size * 0.6` (mesmo valor de `FontBookMetrics` proporcional) |
| `ttf-parser` não escapa | L1 continua a receber apenas `Pt` e tipos puros |
| Variação aplicada por carácter | `advance()` aplica `set_variation` a uma cópia da face cacheada antes de medir (P772o) — a face partilhada em `Arc<CachedFace>` nunca é mutada |

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
| 2026-07-14 | P760 — `FontMetrics::vertical_metrics` e `cap_height` recebem `style`; `FallbackFontMetrics` resolve a fonte do estilo; métricas tipográficas do OS/2 preferidas via `typo_metrics` | `font_metrics.md`, `font_metrics.rs`, `layout.md` |
| 2026-07-16 | P772o — `advance()` passa a aplicar `set_variation` (eixos `wght`/`ital`) a uma cópia da face antes de medir; corrige colapso de espaço entre palavras em fontes variáveis de peso alto (`Ubuntu Sans`, confirmado; `Cantarell-VF` continua afectado por causa não isolada — ver relatório) | `font_metrics.md`, `font_metrics.rs` |
| 2026-07-22 | P838 — `covering()` recebe `variant` e escolhe o fallback global via `FontBook::select_fallback` (scoring vanilla), alinhando a fonte medida com a do shaping | `font_metrics.md`, `font_metrics.rs` |
| 2026-07-31 | P942 — `covering()` deixa de carregar a face de cada candidato para re-verificar `glyph_index` (coverage exacta já é definitiva); elimina o custo dominante do fallback CJK (~480 ms em `utf8-cjk`) | `font_metrics.md`, `font_metrics.rs` |

## P836 — eixos explícitos nas métricas

Os quatro call sites que calculam `axis_variations_for_font_variant(&
variant)` a partir de um `TextStyle` passam a usar
`axis_variations_for_text_style(style)`, para que larguras/métricas de
glifos reflitam as variações explícitas de `#text(variations:)` —
coerência com o shaper (avanços idênticos em medida e shaping).

## P845 — `edge_offset_pt` passa a `pub(crate)`

O shaper (`infra/shaper.md` §P845) reutiliza `edge_offset_pt` para calcular
o avanço vertical entre as linhas de um texto com `\n` interno
(`top-edge + |bottom-edge| + leading`, mesma fórmula e defaults
`cap-height`/`baseline` do avanço de linha do Layouter, P762). Semântica da
função inalterada — só a visibilidade muda (de privada do módulo para
`pub(crate)`).

## P890 — `advance()` falta a injecção da cadeia de fallback matemático (P783/P784), `text_ink_bounds()` já tem

**Sintoma medido** (`typst-passo-889-relatorio.md`, `typst-passo-890-relatorio.md`): qualquer
identificador matemático de letra única (`i`, `n`, auto-itálico) ou letra grega (`alpha`, `beta`)
dispara um custo fixo de ~5s (uma vez por compilação, não por ocorrência), atribuído a
`World::candidates_for_char` (`world.rs`, P880) — a primeira chamada computa `Coverage` para
**todos** os slots do `FontBook` (potencialmente 1000+, incluindo `.ttc` CJK de 20-27MB cada) antes
de devolver os candidatos, mesmo quando a fonte certa já está disponível sem precisar desse scan.

**Causa exacta, confirmada por instrumentação temporária** (`eprintln!` em `covering()`,
`candidates_for_char()` e `text_ink_bounds()`, corridos e revertidos no mesmo passo — não ficou no
código): `layout_equation_measured` (`01_core/src/engine/math/layout/mod.rs:380-388`) chama, na
mesma iteração e para o mesmo `FrameItem`, primeiro `self.metrics.advance(...)` (linha 383) e depois
`self.metrics.text_ink_bounds(...)` (linha 386). **`text_ink_bounds()` já tem a injecção de
`math_fallback_font_list()` em `primary` quando `style.math` é verdadeiro** (secção "Resolução de
fontes" desta L0 não documentava isto — lacuna corrigida agora). **`advance()` (linha 830 de
`font_metrics.rs`) chama só `self.resolve_primary(style)`, sem a mesma injecção** — `resolve_primary`
nunca consulta `math_fallback_font_list()` nem o campo `style.math` (confirmado por leitura directa,
secção "Resolução de fontes" acima). Por `advance()` correr primeiro, é ele quem primeiro chama
`covering()` para `𝑖`/`α`; como `primary` só tem a fonte de corpo (`Libertinus Serif`, sem tabela
MATH), `covering()` cai no scan caro de `candidates_for_char` — que **acha** `New Computer Modern
Math` (a fonte certa já estava lá, só não foi consultada primeiro) mas paga o custo total de o
achar. Confirmado via `fontTools` (P889 secção 2.4) que a fonte certa tem o glifo directo — o scan
nunca foi necessário, só inevitável dado que `advance()` não olhou para o sítio certo primeiro.

**Correcção**: `advance()` passa a aplicar a mesma injecção condicional a `style.math` que
`text_ink_bounds()` já tem, antes de chamar `covering()` pela primeira vez. Mesma cadeia
(`math_fallback_font_list()`), mesma condição (`style.math`), evitando duplicar o texto — extraída
para uma função privada partilhada pelas duas (`push_math_fallback_candidates` ou nome equivalente),
reduzindo o par de implementações divergentes a uma só.

**Fora de escopo desta correcção**: o mecanismo de `candidates_for_char` em si (ler o `FontBook`
inteiro na primeira chamada, independentemente de qual fonte acaba por ser necessária) continua
caro **na primeira vez que genuinamente for preciso** (ex.: um carácter que nenhuma fonte da cadeia
math cobre) — este passo elimina o gatilho desnecessário para os casos em que a cadeia math já
resolveria sem scan, não o custo do scan em si quando ele é mesmo preciso.

## P891 (achado colateral de P889, "i²" com gap indevido) — `FallbackFontMetrics` nunca implementou `math_kern`

**Confirmado por leitura directa do trait** (`01_core/src/engine/layout/metrics.rs::FontMetrics::
math_kern`, default): retorna `MathGlyphKern::default()` (kern zero em todos os 4 quadrantes)
incondicionalmente quando não sobreposto. `FallbackFontMetrics` (`impl FontMetrics for
FallbackFontMetrics<'_>`, este ficheiro) **nunca sobrepôs `math_kern`** — só `FontBookMetrics` (a
variante de face única, usada em testes isolados) tem uma implementação real (linhas 349-405,
leitura da tabela OpenType MATH via `ttf_parser`). Confirmado com instrumentação temporária
(`eprintln!` num override temporário, revertido) que `FallbackFontMetrics::math_kern` — via o
default do trait — é de facto chamado em produção para `𝑖` (base de `i^2` em `04-math.typ`),
devolvendo sempre kern zero.

**Efeito**: `attach.rs::layout_attach` usa `base_kern.top_right`/`.bottom_right`/etc. para "encaixar"
scripts (sub/super-índices) mais perto de bases itálicas inclinadas — com kern sempre zero, o
expoente de `i^2` fica posicionado sem essa aproximação, produzindo o gap visível confirmado em
P889 (`i  ²` em vez de `i²`).

**Achado colateral, não corrigido neste passo (fora de âmbito, registado para futuro)**: o trait
`FontMetrics` tem mais três métodos math-específicos com default e sem override em
`FallbackFontMetrics` — `math_constants` (usa `MathConstants::fallback()`, genérico, não a tabela
MATH real da fonte activa), `vertical_glyph_variants` e `vertical_glyph_assembly` (sem variantes de
tamanho nem montagem por partes — delimitadores extensíveis podem não crescer correctamente). Estes
três não foram tocados nem instrumentados neste passo (P891 só cobre `math_kern`, o único
confirmado a afectar o sintoma medido) — candidatos a um passo dedicado futuro, dado o impacto
potencial (constantes MATH aproximadas afectam proporções de toda a equação, não só um kern
pontual).

**Correcção**: `FallbackFontMetrics::math_kern` ganha uma implementação real. Como a variante
multi-fonte precisa de saber **qual** face activa cobre o carácter (ao contrário de
`FontBookMetrics`, que só tem uma face fixa), a assinatura do trait ganha um parâmetro
`style: &TextStyle` (mesmo padrão já usado por `advance`/`text_ink_bounds`) — `FallbackFontMetrics`
resolve a face via `resolve_primary_with_math_fallback` + `covering` (mesmo mecanismo de P890) e lê
a tabela MATH dessa face; `FontBookMetrics` ignora o novo parâmetro (só tem uma face). A lógica de
leitura da tabela (antes só em `FontBookMetrics::math_kern`) é extraída para uma função livre
partilhada (`math_kern_from_face(face, c)`), reusada pelas duas implementações.

## P893 (primeiro dos 3 achados colaterais de P891) — `FallbackFontMetrics` nunca implementou `math_constants`

**Medido** (`typst-passo-893-relatorio.md`, `fontTools` sobre `NewCMMath-Regular.otf`, mesmo `rev`
embutido em produção): das 13 constantes de `MathConstants` (excl. `upem`), `axis_height` diverge
2× do fallback STIX (`500` vs `250` real), `subscript_shift_down` +90% (`130` vs `247`),
`upper_limit_gap_min` +100% (`100` vs `200`); só `script_percent_scale_down`/`script_script_
percent_scale_down` (idênticos) e `superscript_shift_up` (+0.3%) estão próximos do fallback. Não é
uma imprecisão pontual — `axis_height` sozinho afecta o centro vertical de **toda** fracção/
delimitador/raiz (`MathLayouter::apply_axis_offset`).

**Correcção**: `FallbackFontMetrics::math_constants(&self, style: &TextStyle) -> MathConstants`
ganha implementação real. Como `math_constants` não recebe `char` (é uma propriedade por-fonte, não
por-glifo, ao contrário de `math_kern`), não usa `covering(c, ...)` — em vez disso resolve
`primary = resolve_primary_with_math_fallback(style, &variant)` e escolhe a **primeira face com
tabela MATH presente** (`face.tables().math.is_some()`), não simplesmente `primary.first()` (que no
caso comum, sem `#set text(font:)` explícito, seria a fonte de corpo genérica sem tabela MATH,
resultando em fallback sempre — sem correcção nenhuma no caso comum). Sem candidato com tabela MATH:
`MathConstants::fallback()` (mesmo comportamento de hoje). Lógica de leitura extraída de
`FontBookMetrics::math_constants` para uma função livre partilhada `math_constants_from_face(face:
&ttf_parser::Face) -> MathConstants` (mesmo padrão de `math_kern_from_face`, P891).

**Efeito na integração**: `MathLayouter::new(metrics, block)` (`math/layout/_comum.md`,
`math/layout/mod.rs`) não recebia `style` — os `constants` resultantes são cacheados uma única vez
por equação em `self.constants`. Ganha um terceiro parâmetro `style: &TextStyle`, chamando
`metrics.math_constants(style)`. Único call site de produção: `engine/layout/equation.rs` (já
constrói `math_style` antes desta chamada — ver `engine/layout/equation.md` §P893).

**Fora de âmbito deste achado, à data** (revisitado e corrigido em P906, ver secção abaixo):
`vertical_glyph_variants`/`vertical_glyph_assembly` — problema de crescimento de glifo, não de
proporção contínua, tratado à parte por decisão explícita de P891/P893.

## P906 — `horizontal_glyph_variants`/`horizontal_glyph_assembly` (extracção da tabela MATH)

Ver `engine/layout.md` §P906 (trait, decisão de design, dados da fonte confirmados via `fontTools`).

**Implementação**: duas funções privadas novas, `extract_variants_horizontal`/
`extract_assembly_horizontal`, espelhando byte-a-byte `extract_variants`/`extract_assembly`
existentes — única diferença: lêem `variants_table.horizontal_constructions` em vez de
`.vertical_constructions` (campo simétrico da mesma struct `ttf_parser`, confirmado no vanilla).
`extract_variants`/`extract_assembly` (verticais) **não tocadas** — decisão aditiva de
`engine/layout.md` §P906 aplicada também aqui: duplicação deliberada em vez de parametrizar por
eixo, para não arriscar o caminho vertical já em produção.

`FontBookMetrics::horizontal_glyph_variants`/`::horizontal_glyph_assembly` (impl do trait) chamam
as novas funções privadas — mesmo padrão dos métodos verticais correspondentes.

`build_math_glyph_reverse_map` — decisão original (não estender) **revertida** ao investigar a
confirmação visual (ver subsecção "cadeia de bugs pré-existentes" abaixo): sem entradas para os
chars horizontais, o subsetting (P45/DEBT-9, `builder.md` §P906) nunca incluía os glyph_ids de
esticamento horizontal no subset embutido. Estendida com `STRETCHY_BASES_HORIZONTAL = ['⏟', '⏞',
'⎵', '⎴', '⏝', '⏜', '\u{0302}', '\u{0303}']`, extensor mapeia para `_` (análogo ao `|` vertical).

### Cadeia de bugs pré-existentes descoberta ao confirmar visualmente (não introduzidos por P906)

`underbracket(a+b+c)` produzia PDF com glifo `.notdef` apesar do layout (largura, posição) estar
correcto e testado ao nível de unidade. Investigação (instrumentação directa no binário, não
inferência) revelou uma cadeia de **4 bugs pré-existentes**, cada um só visível depois de corrigir
o anterior, todos partilhados com o eixo vertical (nunca antes exercitados com dados reais porque
`vertical_glyph_variants`/`assembly` não estavam implementados em `FallbackFontMetrics` — bug nº1):

1. **Este ficheiro**: `FallbackFontMetrics` (única implementação usada no pipeline real,
   `pipeline.rs:126`) nunca implementava `vertical_glyph_variants`/`vertical_glyph_assembly` —
   herdava o default vazio do trait. Gap já confirmado e deliberadamente adiado em P891/P893 (nota
   acima), nunca corrigido. Corrigido agora com o mesmo mecanismo de `math_kern` (P891): resolve
   `primary = resolve_primary_with_math_fallback(style, &variant)`, `covering(c, &primary, &variant)`
   → `cached.face()` → `extract_variants`/`extract_assembly` (verticais) e `extract_variants_
   horizontal`/`extract_assembly_horizontal` (horizontais, novas). `&dyn FontMetrics` (`engine/
   layout.md`, delegação P858) tinha o MESMO gap (nunca delegava `vertical_glyph_variants`/
   `assembly`) — corrigido em paralelo.
2. `emit_glyph_pdf` (`export/stream.md` §P906) hardcodava `/F1` + não aplicava remap de subsetting.
3. `collect_fonts_in_items` (`infra/pipeline.md` §P906) ignorava `FrameItem::Glyph` na selecção de
   quais fontes embutir — corrigido com o novo método abaixo.
4. `layout_underover`/`layout_accent` (`math/layout/_comum.md` §P906) usavam a convenção "topo do
   box" em vez de baseline-relativa (mesmo padrão já corrigido em `frac.rs` P905 e `root.rs` P901,
   nunca antes auditado aqui) — só visível ao aninhar `MathUnderover` (caso `underbrace`/`overbrace`
   COM anotação).

### `resolve_font_combo` — novo método, para (3) acima

```rust
pub(crate) fn resolve_font_combo(&self, c: char, style: &TextStyle)
    -> Option<(FontList, FontVariant, FontVariations)>
```

Mesmo mecanismo de `covering`/`resolve_primary_with_math_fallback`, mas devolve a IDENTIDADE da
fonte (via `FontInfo` no `slot_idx` resolvido: `book.infos().get(slot_idx)` → `(FontList::single
(info.family), info.variant, FontVariations::default())`) em vez de dados de glifo. Consumido por
`pipeline::collect_fonts_in_items` para que `FrameItem::Glyph` entre na selecção Cidfont-vs-
Multifont como se fosse mais um span de texto.

### Achados relacionados, fora de âmbito de P906 (registados, não corrigidos)

- **Resolução de face para caracteres em múltiplas fontes candidatas** (ex. parênteses `(`/`)`,
  presentes tanto na fonte de corpo como na MATH): `covering()` pode preferir a fonte de corpo
  (sem tabela MATH) mesmo em contexto math, se `resolve_primary` já a encontrar como primária antes
  do fallback matemático ser consultado — achado incidental durante a investigação da cadeia acima,
  não confirmado em profundidade nem corrigido.
- **`layout_stretchy_glyph_horizontal`/`layout_stretchy_delimiter`, ramo "variante encontrada +
  `glyph_to_char` mapeado"**: mapeia o `glyph_id` da variante GRANDE de volta ao MESMO `base_char`
  pequeno (por construção de `build_math_glyph_reverse_map`) e renderiza via `layout_text_node`
  do `base_char` — isto descarta a largura (`advance_du`) e o glifo da variante encontrada,
  medindo antes o avanço natural (pequeno) do char base. Sintoma confirmado: `hat(a+b)` produz um
  item de largura zero (combining mark, `hmtx` advance=0) em vez de esticado. Achado novo,
  candidato a passo dedicado — possivelmente requer que o `FrameItem::Text` resultante carregue
  um `x_advance` explícito em vez de depender de `self.metrics.advance(text, ...)` remedir o char
  base.

---

### §P914 — Expansão de `STRETCHY_BASES` para Mapeamento ToUnicode

Em `build_math_glyph_reverse_map`:
- Adicionados todos os caracteres delimitadores matemáticos verticais: `‖` / `∥`, `⌊` / `⌋`, `⌈` / `⌉`, `⟨` / `⟩`, `/`, `\`, `↑`, `↓`, `↕`, `⇑`, `⇓`, `⇕`.
- Garante que variantes de glifos verticais produzidas para esses delimitadores sejam incluídas no mapa reverso `glyph_id → char`, viabilizando o mapeamento ToUnicode em exportação PDF.

## P945 — `extract_assembly`/`extract_assembly_horizontal` leem `min_connector_overlap`

**Medição**: `engine/math/layout/assembly.md` §P945 — o algoritmo de assembly
do vanilla consome `minConnectorOverlap` (NewCMMath: 20du).

`extract_assembly` e `extract_assembly_horizontal` passam a preencher o novo
campo `GlyphAssembly::min_overlap` (`entities/glyph_variants.md` §P945) a
partir de `face.tables().math.variants.min_connector_overlap` (campo já
exposto por ttf_parser 0.25 — mesmo sítio onde o vanilla o lê,
`lab/typst-original/crates/typst-layout/src/math/fragment/glyph.rs:542-548`).
Faces sem tabela MATH/variants mantêm o default `0` (comportamento anterior).

## P946 — reverse map: peças de assembly/variantes com codepoint real da cmap (divergência deliberada)

**Medição** (`typst-passo-946` Fase A, commit `019dfbd5c`): o render das peças
está correcto (charstrings byte-a-byte idênticos ao vanilla e à fonte original —
verificado via fontTools/pikepdf; os dois sintomas visuais reportados — `binom`
"com dois delimitadores" e gancho inferior da chave de `cases` "com glifo
errado" — são **artefactos de rasterização a 72–150dpi**, presentes também no
vanilla à mesma DPI; a 300dpi os dois compiladores renderizam correctamente e de
forma equivalente). No texto extraído, contudo, o cristalino emitia os
caracteres de conveniência de P906 (`{`, `|` para vertical; `{`, `_` para
horizontal) — 5 caracteres para 1 delimitador lógico.

**Comportamento do vanilla, medido** (ToUnicode CMap de PDF real): o vanilla
agrupa os glifos da assembly por cluster no char de origem — a chave de `cases`
extrai como **um único `{`** (extensores/peça do meio não mapeados). Ou seja, o
vanilla **não** emite os codepoints reais das peças (a premissa inicial deste
passo foi refutada por esta medição e corrigida antes da decisão).

**Decisão do dono (divergência deliberada, ADR-0107 — a mecânica diverge de
propósito)**: mapear cada peça para o seu **codepoint real** quando existir na
cmap da face (NewCMMath cobre U+239B–U+23A0 e U+23A1–U+23AA, verificado via
fontTools) — extracção semanticamente mais rica e acessível (`⎧⎪⎨⎪⎩`), mesmo
divergindo do vanilla na extracção.

**Implementação**: `build_math_glyph_reverse_map` constrói primeiro o mapa
inverso da **cmap da face** (glyph_id → char, um passe sobre os codepoints) e,
para cada glifo de variante/peça de assembly, prefere o **codepoint real**
quando existe na cmap; só cai no fallback anterior (`base_char` para peças sem
codepoint — ex.: variantes `.v1`–`.v7`, que não são codificadas — `|`/`_` para
extensores sem codepoint, ex.: `braceleft.ex`, glifo distinto de `uni23AA` e sem
codepoint próprio). A pesquisa `or_insert` (primeira base char ganha para peças
partilhadas) mantém-se.

**Consumidores e invariâncias** (verificados por leitura):
- `builder.rs` ToUnicode de `FrameItem::Glyph` — passa a emitir os codepoints
  reais das peças quando cobertos pela cmap (divergência deliberada, acima).
- `stretchy.rs` (caminho de **variante** com mapeamento): variantes `.vN`
  não têm codepoint na cmap → continuam mapeadas para `base_char` —
  comportamento inalterado.
- `assembly.rs` emite peças como `FrameItem::Glyph` directamente (nunca via
  `glyph_to_char`) — inalterado.
- Subsetting (`builder.rs`) indexa por glyph_id — inalterado.

## P952 — `math_constants_from_face` lê `display_operator_min_height`

`math_constants_from_face` passa a preencher o novo campo
`MathConstants::display_operator_min_height` (`entities/math_constants.md`
§P952) a partir de `c.display_operator_min_height()` (devolve `u16` directo em ttf_parser 0.25 — sem `.value`) (método já
exposto por ttf_parser 0.25, mesmo padrão dos outros campos).
