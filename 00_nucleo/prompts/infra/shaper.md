---
prompt: infra/shaper
layer: L3
created: 2026-06-27
passo: P482
adr: ADR-0120
---

# Prompt L0 — `shaper.rs` (Trilha 5 Fase 1)
Hash do Código: 1fa8b958

## Propósito

Post-processing shaping pass: converte `FrameItem::Text` →
`FrameItem::TextShaped` via rustybuzz após layout e antes de export.
Executado em L3 (`03_infra/src/shaper.rs`).

Decisão arquitectural: ADR-0120 Opção A1 — o shaping não pode ocorrer
em `cursor.rs` (L1) porque os bytes de fonte vivem em L3.

## API pública

```rust
pub fn shape_document(world: &dyn World, doc: PagedDocument) -> PagedDocument
```

Converte todos os `FrameItem::Text` de um `PagedDocument` em
`FrameItem::TextShaped` via rustybuzz.

Itens sem fonte resolvida (`style.font == None` ou lookup falha) são
preservados como `FrameItem::Text` (fallback Helvetica).

## Pipeline interno

```
shape_document(world, doc) → shape_page → shape_item → try_shape
```

- `shape_item`: desce recursivamente em `Group` e `Link`.
  `Text` com `style.font.is_some()` é candidato a shaping.
- `try_shape`: resolve slot via `resolve_slot`, obtém bytes via
  `world.font(slot_idx)`, constrói `rustybuzz::Face::from_slice(data, 0)`,
  shape com `rustybuzz::shape(&rb_face, &[], buffer)`, mapeia
  glyph_infos + glyph_positions → `Vec<ShapedGlyph>`.

## Resolução de fonte

```rust
fn resolve_slot(world: &dyn World, font_list: &FontList) -> Option<usize>
```

Itera `font_list.as_slice()`, para cada família chama
`world.book().select_pattern(&family.name, &FontVariant::default())`.
Verifica que `world.font(idx)` devolve `Some` antes de aceitar o slot.
Mesma lógica de `resolve_font` em `pipeline.rs`.

## `byte_idx_to_char`

```rust
fn byte_idx_to_char(s: &str, byte_idx: usize) -> Option<char>
```

Devolve o codepoint que começa no byte `byte_idx` de `s` (UTF-8).
Usado para popular `ShapedGlyph.char_code` a partir de `cluster`.

## Scope-out (Fase 1)

- RTL/bidi — Fase 3 (ADR-0120).
- OpenType features explícitas — `features = &[]` usa GSUB/GPOS padrão.
- Font fallback multi-família além da primeira que resolve.
- Face index > 0 (TTC fonts) — Fase 1 usa sempre index 0.
- `FrameItem::TextShaped` shaping (recursão) — não re-shape já shaped.

## Testes (≥6)

- `p482_shape_document_preserves_text_sem_font`: sem `style.font` → Text preservado.
- `p482_shape_document_preserves_text_content`: conteúdo do Text preservado.
- `p482_byte_idx_to_char_ascii`: índices ASCII correctos.
- `p482_byte_idx_to_char_utf8`: multi-byte UTF-8 (é = 2 bytes).
- `p482_shaped_glyph_clone_eq`: ShapedGlyph clone+eq.
- `p482_shape_document_group_children_passthrough`: Text dentro de Group preservado sem font.

## P484 — Fase 3: RTL básico via unicode-bidi

**Data:** 2026-06-28

`unicode-bidi = "0.3"` adicionado ao workspace. `try_shape` agora divide o texto
em runs bidirectionais antes de shape via `bidi_runs(text) -> Vec<BidiRun>`.

### `BidiRun`

```rust
struct BidiRun {
    text:       String,  // substring do texto original para este run
    rtl:        bool,    // true se run RTL (árabe, hebraico, etc.)
    byte_start: usize,   // offset byte no string original (para cluster abs)
}
```

### `bidi_runs`

```rust
fn bidi_runs(text: &str) -> Vec<BidiRun>
```

Usa `unicode_bidi::BidiInfo::new(text, None)` + `visual_runs(para, line)`
para obter runs na ordem visual correcta. Para texto LTR puro: 1 run.
Para texto misto/RTL: múltiplos runs na ordem correcta de renderização.

### `try_shape` pós-P484

Substitui `buffer.guess_segment_properties()` por:

```rust
for run in bidi_runs(text) {
    buffer.set_direction(if run.rtl { Direction::RightToLeft }
                         else       { Direction::LeftToRight });
    // shape + ajustar abs_cluster = run.byte_start + info.cluster
}
```

### Scope-out P484

- Múltiplos parágrafos (`paragraphs[0]` assume 1 parágrafo por `FrameItem::Text`).
- Texto vertical (CJK rotated).
- Corpus RTL no lab/parity — testes unitários L3 cobrem.
- Remoção de `FrameItem::Text` — requer ADR nova (colisão ADR-0029).

### Testes adicionados P484

- `p484_bidi_runs_ltr_unico_run`: texto inglês → 1 run LTR.
- `p484_bidi_runs_vazio_zero_runs`: texto vazio → 0 runs.
- `p484_bidi_runs_arabico_rtl`: árabe → run RTL detectado.
- `p484_try_shape_rtl_sem_fonte_nao_panic`: árabe sem fonte → sem panic.
- `p484_bidi_runs_misto_ingles_arabico`: texto misto → ≥2 runs.
- `p484_bidi_runs_byte_start_correcto`: byte_start correcto.

---

## P483 — Fase 2: font padrão + cobertura ≥95%

**Data:** 2026-06-28

`try_shape` actua quando `style.font.is_some()`. Pós-P483, `From<&StyleChain>
for TextStyle` preenche sempre `font` com pelo menos `FontList("Helvetica")`
(fallback padrão). Assim o shaper tenta actuar em todo o texto, não apenas
em texto com `#set text(font:...)` explícito.

Comportamento defensivo preservado: se `resolve_slot` não encontra a fonte
no `FontBook` (fonte não carregada, Type1), `try_shape` retorna `None` e o
item permanece como `FrameItem::Text`.

### Cobertura esperada em produção

Em produção com `SystemWorld` e fontes do sistema carregadas: ≥95% do texto
normal será `TextShaped`. `FrameItem::Text` resta apenas para fontes ausentes.

### Testes adicionados P483

- `p483_text_com_font_helvetica_tenta_shape_mas_sem_fontes_preserva_text`
- `p483_text_sem_font_nao_tenta_shape`
- `p483_shaped_glyph_debug_display`

## §P485 — `units_per_em` em `try_shape`

**P485** adiciona extracção de `units_per_em` de `rb_face.units_per_em()` (retorna
`i32` via rustybuzz; cast `.max(1) as u16`). O valor é armazenado em
`FrameItem::TextShaped.units_per_em: u16` e usado em `emit_shaped_pdf` para
converter `x_advance` (font units) em unidades TJ do PDF.

```rust
// Em try_shape, após construir rb_face:
let units_per_em = rb_face.units_per_em().max(1) as u16;
// ...
Some(FrameItem::TextShaped { pos: *pos, glyphs: all_glyphs, style: style.clone(),
                              text: text.clone(), units_per_em })
```

### Testes adicionados P485

- `p485_shape_document_sem_fonte_nao_produz_textshaped`
- `p485_units_per_em_cast_seguro`

## §P486 — Features OpenType confirmadas (Sub-item A)

**P486** confirma via sonda do código-fonte de rustybuzz 0.20.1 (`ot_shape.rs:86-91`)
que `liga`, `kern` e `calt` estão em `HORIZONTAL_FEATURES` com flags `F_GLOBAL`/
`F_GLOBAL_HAS_FALLBACK` — activados por defeito para texto horizontal, independentemente
do parâmetro `user_features` passado a `rustybuzz::shape`.

```rust
// P486 — liga/kern/calt activados por defeito via HORIZONTAL_FEATURES
// (rustybuzz 0.20.1 ot_shape.rs:86-91). features = &[] é suficiente.
let output = rustybuzz::shape(&rb_face, &[], buffer);
```

Sub-item A é apenas documentação — nenhum código novo é necessário.

### Testes adicionados P486

- `p486_features_default_confirmado`: `features = &[]` (len=0) documenta invariante.

---

## §P515 — Font fallback por caractere

**Data:** 2026-06-30

O shaper passa a suportar fallback por caractere quando a fonte principal
não cobre todos os codepoints do texto. Isto é necessário para documentos
com scripts mistos (ex: latino + CJK + emoji).

### Heurística de cobertura

```rust
fn face_covers_char(face: &rustybuzz::Face, c: char) -> bool
```

- Um caractere é coberto se `face.glyph_index(c)` (via ttf_parser) devolver `Some`.
- Alternativa: iterar `face.glyph_count()` — usar `ttf_parser::Face` para o lookup.

### `shape_with_fallback`

Substituir `try_shape` por um pipeline que:

1. Resolve a fonte principal via `resolve_slot` (primeira família de `FontList` que resolva).
2. Para cada run bidireccional (P484), percorre os caractes e quebra o run em sub-runs sempre que a cobertura da fonte actual mudar.
3. Para cada sub-run, tenta resolver uma fonte que cubra todos os caracteres do sub-run, iterando `FontList` e, opcionalmente, fallback genérico do sistema (quando `fontdb` estiver activo).
4. Shape cada sub-run com a sua fonte.
5. Concatena os glifos na ordem visual, preservando `cluster` absoluto no texto original.

### Estrutura do resultado

`FrameItem::TextShaped` continua a ser uma única entidade por `FrameItem::Text` original. Os glifos podem vir de múltiplas faces; o PDF emit (multifont, P515+) usa o `glyph_id` e a fonte correcta para cada glifo.

Para a Fase 1 do P515, o shaper produz **um `TextShaped` por fonte** (i.e., o item original é substituído por múltiplos `FrameItem::TextShaped` consecutivos com o mesmo `pos` base e offsets acumulados) ou mantém um único `TextShaped` com metadados de fonte por glifo. A decisão concreta é delegada ao Prompt L0 `infra/export/font_subset` e ao refactor do `PdfBuilder` para multi-fonte per-glyph.

### Scope-out P515

- Fallback para fontes do sistema quando `FontList` não cobre — requer `fontdb` activo (Prompt L0 `infra/fontdb`).
- Escolha de peso/estilo no fallback — usa `FontVariant::default()`.
- Shape de texto vertical.

### Testes adicionados P515

- `p515_shape_document_latin_only_single_font`: latim coberto → 1 TextShaped.
- `p515_shape_document_mixed_fallback_splits`: latim + caractere ausente → ≥2 TextShaped ou glifos com fontes distintas.
- `p515_try_shape_missing_glyph_does_not_panic`: caractere sem cobertura em nenhuma fonte → preserva Text (não panic).

---

## P525 — Variation Fonts MVP

**Data:** 2026-07-01

Aplica coordenadas de eixo OpenType (`wght`, `ital`) ao `rustybuzz::Face`
antes do shape, usando `set_variations`. Resolve fontes candidatas com a
`FontVariant` real derivada do `TextStyle`, não mais `FontVariant::default()`.

### Decisão de arquitectura: gestão do `Face`

P525 sondou a pipeline e confirmou que **não existe cache de `rustybuzz::Face`**
no shaper: cada run de texto cria uma nova face via `Face::from_slice`. Portanto,
chamar `set_variations` logo após a criação do face é seguro e não há risco de
contaminação entre pesos diferentes no mesmo documento.

Se futuramente for introduzida uma cache de `Face`, a chave deve incluir a
variante (ou `set_variations` deve ser reaplicado antes de cada `shape`), para
não reintroduzir contaminação.

### `text_style_to_font_variant`

```rust
fn text_style_to_font_variant(style: &TextStyle) -> FontVariant
```

Deriva `FontVariant` de `TextStyle.weight`/`bold`/`italic`. `stretch` não está
exposto no `TextStyle` actual (rejeitado em P414); `Oblique(angle)` não existe
no modelo actual (`FontStyle::Oblique` é uma flag sem ângulo).

### `axis_variations_for_font_variant`

```rust
fn axis_variations_for_font_variant(variant: &FontVariant) -> Vec<rustybuzz::Variation>
```

Mapeamento:

- `weight` → `wght` (100–900). Omissão quando 400 (default).
- `FontStyle::Italic` → `ital` = 1.0.
- `stretch` → `wdth` (não activo até `TextStyle` expor stretch).
- `Oblique(angle)` → `slnt` (não activo até `FontStyle::Oblique` carregar ângulo).

### Aplicação no `try_shape`

```rust
let variant = text_style_to_font_variant(style);
let axis_vars = axis_variations_for_font_variant(&variant);
let candidates = resolve_candidates(world, font_list, &variant)?;
// ...
let mut rb_face = rustybuzz::Face::from_slice(font.as_slice(), 0)?;
if !axis_vars.is_empty() {
    rb_face.set_variations(&axis_vars);
}
```

### Limitações do MVP

- O shaper aplica variações nos avanços e posicionamentos, mas o **export PDF
  não as reflecte no output visual**. O `resolve_font` da pipeline usa
  `FontVariant::default()` e `collect_fonts_from_doc` agrupa por `FontList`
  (sem weight/style), pelo que todos os pesos partilham a mesma fonte subsetada
  na instância default. Um leitor de PDF não varia contornos embutidos.
- Portanto, `text(weight: 700)` numa fonte VF produz avanços de bold mas
  **contornos de regular** — regressão de linguagem no output visual.
- Fix real (P528) requer instanciar a VF estaticamente para cada combinação
  peso/estilo usada no documento e embutir cada instância separadamente.

### Testes adicionados P525

- `p525_axis_variations_weight_italic`: mapeamento `FontVariant` → eixos.
- `p525_shape_document_mixed_weights_no_contamination`: pipeline real com
  `wght=700 → 100 → 700`, confirmando que o terceiro shape reproduz o primeiro.

---

## P534 — Fallback de fonte por script/cobertura (multi-script)

**Data:** 2026-07-02

Correcção de P515: o fallback anterior limitava-se às famílias declaradas na
`FontList`; caracteres fora da cobertura dessas famílias perdiam-se. P534
segmenta o texto por script Unicode e, para cada segmento, procura primeiro nas
famílias declaradas e depois em todas as fontes disponíveis no `FontBook`
(fontdb), escolhendo a primeira que cubra todos os caracteres do segmento.

### Segmentação por script

```rust
fn script_runs(run: &BidiRun) -> Vec<ScriptRun>
```

Usa `unicode-script::{Script, UnicodeScript}`. Caracteres com script `Common` ou
`Inherited` herdam o script do segmento actual (`is_compatible`). Mudanças de
script forçam uma quebra de sub-run, mesmo que a mesma fonte cubra ambos.

### Escolha por cobertura

Para cada segmento de script:

1. Itera as `primary_candidates` (famílias da `FontList`, na ordem declarada).
2. Se alguma cobrir todos os caracteres do segmento, usa-a.
3. Caso contrário, itera `fallback_candidates` (todo o `FontBook`, na ordem de
   descoberta), escolhendo a primeira que cubra todos os caracteres.
4. Se nenhuma fonte cobrir o segmento inteiro, divide o segmento no ponto onde
   a cobertura muda e repete.

Implementação concreta: `split_run_by_font` percorre os caracteres de cada
`BidiRun`, mantendo o script efectivo e o índice da fonte candidata. Em cada
mudança de script ou de fonte, fecha a sub-run actual.

### Caching de cobertura

Para evitar re-parse da face para cada caractere, `try_shape` mantém um cache
`HashMap<char, usize>` do primeiro candidato (primary+fallback) que cobre cada
caractere. O cache é local a cada chamada de `try_shape`.

### Scope-out P534

- Fontes de cor para emoji (COLR/CPAL) — mecanismo de renderização, não escolha
  de fonte.
- Fundir blocos `BT...ET` consecutivos da mesma fonte — scope-out; cada
  `FrameItem::TextShaped` continua a gerar o seu próprio bloco.
- Ordenação sofisticada de fallback por script (fontique) — usa ordem do
  `FontBook`.

### Testes adicionados P534

- `p534_split_run_by_font_respects_script_boundaries`: segmentação por script.
- `p534_shape_mixed_script_system_fallback`: texto latim+CJK+árabe com system
  fonts produz múltiplos `TextShaped` com fontes distintas.

---

## §P568 — Preservar espaços como `FrameItem::Text`

**Data:** 2026-07-05

`try_shape` retorna `None` (i.e., não shapeia e preserva `FrameItem::Text`)
quando o texto é composto apenas por whitespace (`text.trim().is_empty()`).

- Espaços entre palavras são assim emitidos pelo caminho primário de texto
  (`emit_text_pdf`) em vez de serem absorvidos no shaping de glyphs.
- O export PDF garante que os glyphs correspondentes a esses caracteres
  ainda são subsetados (ver §P568 em `builder.md`).

## P555 — Fallback preserva a classe da fonte (serifa / sem serifa)

**Data:** 2026-07-03

P554 mudou a fonte por defeito do cristalino para `FreeSerif`. Se `FreeSerif`
não estiver disponível no ambiente, o shaper deve tentar outras serifas antes
de recair em sans-serif, preservando a classe visual escolhida.

### Listas de fallback por classe

```rust
const DEFAULT_FALLBACK_FONTS_SERIF: &[&str] = &[
    "FreeSerif", "DejaVu Serif", "Liberation Serif", "Bitstream Vera Serif",
];
const DEFAULT_FALLBACK_FONTS_SANS: &[&str] = &[
    "DejaVu Sans", "Noto Sans", "Liberation Sans", "FreeSans", "Arial",
];
```

### Heurística de classe

A classe é inferida a partir do nome da primeira família declarada em
`style.font`:

- Se o nome (case-insensitive) contiver `"serif"` → usar `DEFAULT_FALLBACK_FONTS_SERIF`.
- Se contiver `"sans"` → usar `DEFAULT_FALLBACK_FONTS_SANS`.
- Caso contrário → manter comportamento anterior (`DEFAULT_FALLBACK_FONTS_SANS`).

Nomes em regex ou sem indicação de classe usam a lista sans por defeito.

### Implementação

- Novo módulo `03_infra/src/fallback_fonts.rs` centraliza as constantes e a
  função `fallback_font_list_for(name: &str) -> &'static [&'static str]`.
- `shaper.rs` e `font_metrics.rs` importam deste módulo e removem as suas
  definições locais duplicadas.
- Em `try_shape`, quando as fontes primárias não resolvem, itera a lista
  escolhida antes de recair no fallback carácter-a-carácter do `FontBook`.

### Testes adicionados P555

- `p555_fallback_font_list_serif_for_freeserif`: `FreeSerif` → lista serif.
- `p555_fallback_font_list_sans_for_dejavu_sans`: `DejaVu Sans` → lista sans.
- `p555_fallback_font_list_default_for_unknown`: nome sem indicação → lista sans.

---

## §P582 — Redistribuição de posições x após shaping

**Data:** 2026-07-06

Corrige posições x de todos os itens de uma linha usando os advances reais dos glyphs obtidos no shaper. O Layouter usa `FallbackFontMetrics` (estimativa) para calcular o cursor_x de cada palavra; o shaper usa a fonte real (rustybuzz). Quando as duas fontes diferem (ex.: bold resolve face diferente), as posições ficam descasadas. Esta passagem redistribui as posições dentro de cada linha acumulando a diferença (width_real - width_est) de forma a manter todos os espaçamentos e layouts relativos intactos.

### Chamada na pipeline (`03_infra/src/pipeline.rs`)

```rust
let doc = crate::shaper::shape_document(world, doc);
let doc = crate::shaper::fix_line_positions(world, doc);
```

### Comportamento

1. **Agrupamento por linha**: Para cada página, percorre `page.items` e agrupa os itens por baseline y (dentro de tolerância 0.5pt).
2. **Redistribuição de x**: Para cada linha, ordena os itens por x original (esquerda para direita).
   - Se a linha for RTL (se algum item tiver direção RTL):
     - Ancoramos o item mais à direita (o início da linha RTL) na sua coordenada original.
     - Percorremos a linha de trás para frente (da direita para a esquerda) aplicando `x_new = x_orig + shift`.
     - O shift acumula negativamente a diferença: `shift -= w_real - w_est` do item à esquerda.
   - Caso contrário (linha LTR normal):
     - Ancoramos o primeiro item (mais à esquerda) na sua coordenada original.
     - Percorremos a linha da esquerda para a direita aplicando `x_new = x_orig + shift`.
     - O shift acumula positivamente a diferença: `shift += w_real - w_est` de cada item.

### Testes adicionados P582

- `p582_single_item_unchanged`: item único — posição inalterada.
- `p582_two_items_redistributed`: dois itens — segundo recebe o shift cumulativo de largura.
- `p582_two_lines_independent`: itens em linhas diferentes redistribuídos independentemente.
- `p582_zero_glyphs_zero_advance`: item com glyphs vazios não altera a posição do seguinte.
- `p582_y_unchanged`: coordenada y mantida inalterada.
- `p582_rtl_redistributed`: linha RTL com âncora à direita e shift para a esquerda.

---

## §P621 — Tracking aplicado nos `x_advance` dos glifos

**Data:** 2026-07-08

Aplica `text.tracking` aos avanços horizontais (`x_advance`) de cada glifo
produzido pelo shaper, convertendo o valor de pontos para unidades da fonte.

### Decisão

- O layout reserva a largura correcta via `FontMetrics::text_width`, que
  adiciona `tracking × (n_chars − 1)` à largura base.
- Para que o posicionamento pós-shaping (`fix_line_positions`) e o output PDF
  reflitam essa largura, o tracking tem de estar presente nos avanços reais dos
  glifos.
- Aplicar tracking no export (via operador PDF `Tc`) desalinha o layout em
  texto RTL, porque o `fix_line_positions_page` distribui as posições usando
  `width_real` calculado a partir dos `x_advance` dos glifos.
- A solução correcta é aumentar `x_advance` de cada glifo durante o shaping,
  garantindo que a largura real do run já inclui o tracking.

### Fórmula

```rust
let tracking_pt = style.tracking.resolve_pt(style.size.val());
let tracking_fu = (tracking_pt * candidate.units_per_em as f64 / style.size.val()).round() as i32;
```

Para cada glifo, o tracking é aplicado ao avanço actual quando o glifo
seguinte pertence a um cluster de caracteres diferente:

```rust
let is_last = idx + 1 == n_glyphs;
let next_cluster_differs = !is_last && clusters[idx + 1] != cluster;
let extra = if next_cluster_differs { tracking_fu } else { 0 };
x_advance: pos_g.x_advance + extra,
run_width += pos_g.x_advance + extra;
```

- `tracking_fu` pode ser negativo (tracking negativo).
- Tracking nulo produz `tracking_fu = 0` e mantém bit-exact com documentos sem
  tracking.
- O último glifo de cada sub-run não recebe tracking (não há glifo seguinte).
- Em scripts com ligaduras/conjuntos (devanágari, árabe contextual), vários
  glifos podem partilhar o mesmo `cluster`; nesses casos o tracking não é
  inserido dentro do cluster, evitando partir a forma visual do caractere.

### Consequências

- `emit_shaped_pdf` continua a usar o delta model de P520/P548 sem `Tc`;
  o tracking já está reflectido em `x_advance`, logo o TJ delta é correcto.
- `fix_line_positions_page` recebe runs cuja largura real inclui tracking,
  pelo que o reposicionamento RTL/LTR mantém-se coerente com a largura
  reservada pelo layout.
- Scripts complexos (devanágari, árabe) beneficiam do tracking em `x_advance`,
  mas diferenças visuais podem persistir devido a ligatures, marks e
  reordering — essas limitações são documentadas no relatório de paridade do
  passo.

### Testes adicionados P621

- `p621_tracking_aumenta_x_advance`: shape do mesmo texto com e sem tracking
  numa fonte real; verifica que a soma dos `x_advance` aumenta pelo valor
  esperado em unidades da fonte.

## §P783/P784 — Cadeia de fallback específica de modo matemático

**P783** implementou o mecanismo (nunca documentado aqui — débito próprio,
fechado retroactivamente por este L0). **P784** fez a verificação visual
real que P783 nunca fez, encontrou dois bugs, e corrigiu.

### Mecanismo (`try_shape`/`shaped_width`, após resolver `primary`)

Depois de resolver as primárias declaradas (`font_list`), decide-se se se
adiciona `fallback_fonts.rs::math_fallback_font_list()`
(`NewComputerModernMath → Libertinus Serif → fontes de emoji`, paridade
`math::families()` vanilla) como **primárias adicionais** — com prioridade
sobre o fallback global lazy (`CandidateSet::covering_all`, todo o
`FontBook` em ordem de índice):

```rust
let primary_has_math = /* primária já resolvida tem tabela MATH própria */;
if style.math || primary_has_math {
    for family in math_fallback_font_list() { /* resolve e adiciona a `primary` */ }
}
```

Dois gatilhos independentes (OR):
- **`style.math`** (P784) — `true` sempre que o texto vem do motor de layout
  matemático (`layout/equation.rs`, ver `entities/layout_types.md` §P784).
  **Este é o gatilho que importa na prática** — a fonte de corpo por omissão
  (`Libertinus Serif`) não tem tabela MATH, então `primary_has_math` sozinho
  nunca disparava no caso comum.
- **`primary_has_math`** (P783 original) — a fonte já resolvida declara MATH
  própria (ex.: utilizador define `font: "NewComputerModernMath"`
  explicitamente, mesmo fora de `$...$`). Mantido como gatilho adicional,
  não removido.

### Bug 1 (P783, corrigido por P784) — gate nunca disparava

P783 só tinha `primary_has_math` — **nunca verificado com um glifo
matemático real ausente em `Libertinus Serif`** (o caso de teste de P783,
`frac(a,b)`/`x^2_1`, só usa ASCII já coberto pela primária, então o
`mutool trace` nunca mudou — "correcto mecanicamente" não era evidência).
P784 mediu com `⨿` (U+2A3F, ausente em `Libertinus Serif`, presente em
`NewComputerModernMath`) e confirmou visualmente: o glifo embutido era
**errado** (`uni27F8`, uma seta dupla, de uma fonte de sistema aleatória —
`MathJax_Main-Regular` — apanhada pelo fallback global, porque a cadeia
math nunca chegava a ser tentada). Corrigido com `style.math` (acima).

### Bug 2 (independente, também descoberto por P784) — nome de família errado

Mesmo com `style.math` a disparar o bloco, `resolve_candidates` continuava
a devolver **zero candidatos** para `"New Computer Modern Math"` (o nome
que P783 hardcoded, com espaços). Medido por leitura directa da tabela
`name` do ficheiro embutido (`fontTools`/`ttf_parser`, nameID 1 `FAMILY`):
o nome real é **`"NewComputerModernMath"`, sem espaços** — inconsistência
do próprio ficheiro de fonte upstream (`Libertinus Serif` tem espaços;
`New Computer Modern *` não tem — confirmado nos dois). Corrigido em
`fallback_fonts.rs::DEFAULT_FALLBACK_FONTS_MATH` (`"NewComputerModernMath"`)
e em `embedded_fonts.rs::embedded_font_group` (ver `embedded_fonts.md`
§P784, mesma causa-raiz, mesma correcção).

### Critério de verificação (P784, medido)

```
$⨿$ (U+2A3F)
  → antes: glyph="uni27F8" (seta, errado), font embutido de MathJax_Main-Regular
  → depois: glyph="uni2A3F" (correcto), adv=".75" — bate exactamente com o
    adv=".75" do vanilla (mesma fonte, mesmo glifo)
  → confirmado visualmente: mutool draw mostra o glifo "⨿" correcto (não tofu,
    não glifo errado), mesma forma que o render vanilla
```


