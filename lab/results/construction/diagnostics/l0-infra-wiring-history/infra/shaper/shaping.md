# Archived pre-sanitation L0 — 00_nucleo/prompts/infra/shaper/shaping.md

This is historical evidence, not a materializable prompt or an approval.
Original SHA-256: `394500edb757c61d6f7e2172d53b5986eb32338d5f16a860635826a79ef5bbad`.
The original text below is preserved without rewriting; dates, line references,
working-tree inventories, proposals and old parity claims retain their original scope.
Dead or unpinned references are archival only and cannot prove current behavior.

---

---
Hash do Código: d5f389fe
prompt: infra/shaper/shaping
layer: L3
created: 2026-09-26
adr: ADR-0120, ADR-0127, ADR-0109, ADR-0129, ADR-0130
---

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/fonts/fallback-selection.toml sha256:faf6c20021b467fdb2a864625f4f6dd4386b5ef28c137b946e5cb4e4ce073d52

# Prompt L0 — infra/shaper/shaping

## Medição anterior à decisão

Extração estrutural de `03_infra/src/shaper.rs:119-129, 187-304, 375-684, 1124-1126` no HEAD `6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5`, working tree não commitado. Proveniência da auditoria registada em `00_nucleo/diagnosticos/shaper-atomizacao-2026-09-26.md` (hash da fonte anterior, diff HEAD --stat, untracked e instante). A suíte baseline shaper passou sem alterações. Estas referências indicam a posição pré-extração, não linhas dos arquivos novos.

## Decisão e contrato da unidade

Possuir shaped_width e try_shape, rustybuzz, variações, direção, tracking, ssty e materialização de TextShaped. Usar bidi e font_selection para segmentação e cache para reuse. Preservar largura máxima de parágrafos na medição, reinício de x e avanço vertical na emissão. Manter as diferenças atuais entre medição e emissão (features/tracking), sem correção algorítmica nesta atomização.

Separação mecânica em L3, sem alteração de contrato público, comportamento por defeito ou fase do pipeline (ADR-0127). A estrutura Rust é mecânica, não uma nova decisão de paridade linguística (ADR-0107/0108). Cada consumer tem este owner exclusivo; referências L0 são canônicas.

## P1133 — decisões semânticas partilhadas

O shaper consome `font_metrics::ssty_eligible_text`, proprietário canónico do
predicado de elegibilidade `ssty`; não mantém uma segunda implementação local.
Em todos os seus caminhos, o modo matemático é decidido exclusivamente por
`TextStyle::math`. O nome da família tipográfica, inclusive a ocorrência textual
de `"math"`, não altera a semântica do conteúdo.


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


## §P568 — Preservar espaços como `FrameItem::Text`

**Data:** 2026-07-05

`try_shape` retorna `None` (i.e., não shapeia e preserva `FrameItem::Text`)
quando o texto é composto apenas por whitespace (`text.trim().is_empty()`).

- Espaços entre palavras são assim emitidos pelo caminho primário de texto
  (`emit_text_pdf`) em vez de serem absorvidos no shaping de glyphs.
- O export PDF garante que os glyphs correspondentes a esses caracteres
  ainda são subsetados (ver §P568 em `builder.md`).


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


## P836 — eixos explícitos no shaping

`shaped_width` e o shaping principal passam a usar
`axis_variations_for_text_style(style)` (fusão dos eixos derivados de
`FontVariant` com `style.variations`, explícitos vencem por tag) em vez
de `axis_variations_for_font_variant(&variant)`. O
`rb_face.set_variations(&axis_vars)` existente aplica as coordenadas
explícitas ao run — eixo desconhecido em fonte não-variável é no-op
silencioso (paridade vanilla, medida em `temp/p836/s1_nonvar.typ`).

---


## P845 — Texto com `\n` interno: todos os parágrafos bidi (achado #55 de P831)

**Data:** 2026-07-22

O scope-out de P484 (`paragraphs[0]` assume 1 parágrafo por `FrameItem::Text`)
truncava qualquer texto com `\n` embutido na primeira linha — string literal
(`#"a\nb\nc"`), `read()` de ficheiro multilinha, `#eval` — porque o unicode-bidi
parte o texto em parágrafos nos separadores de classe B (`\n`, `\r`, U+2028, …)
e só o primeiro era shapeado. Revogado neste passo.

### Desenho

- `bidi_runs(text)` passa a devolver `Vec<Vec<BidiRun>>` — um entry por
  parágrafo bidi, na ordem do texto; parágrafos vazios (`\n\n` consecutivos)
  devolvem `Vec` vazio (a linha existe, só não tem runs). O separador de
  parágrafo (classe B) é excluído do range shapeado — o unicode-bidi mantém-no
  no parágrafo anterior (regra P1 do UAX#9) — para não gerar glifos `.notdef`
  nem poluir o texto extraível.
- `try_shape`: cada parágrafo é uma linha visível — `x` reinicia e `y` avança
  por `line_advance = top-edge + |bottom-edge| + leading`, a MESMA fórmula e
  os mesmos edges por omissão (`cap-height`/`baseline`, via
  `font_metrics::edge_offset_pt`, agora `pub(crate)`) do avanço de linha do
  Layouter em L1 (P762); leading default 0,65em. Métricas da primeira
  primária resolvida.
- `shaped_width`: largura de texto multilinha = **max** das larguras de linha
  (não a soma) — cada parágrafo é uma linha independente.

### Limitações (registadas, fora de escopo)

- O Layouter (L1) continua a tratar o texto com `\n` como UMA palavra/linha:
  a altura do frame e o cursor vertical não reflectem as linhas extra — texto
  seguinte pode sobrepor-se se o conteúdo exceder a linha reservada, e o
  salto de página não dispara por estas linhas. A correcção é de render
  (todas as linhas visíveis), não de fluxo vertical.
- `top-edge`/`bottom-edge` nomeados são respeitados via `edge_offset_pt`;
  o leading vem de `style.leading` (default 0,65em).

### Testes adicionados P845

- `p845_bidi_runs_tres_linhas_tres_paragrafos`: `a\nb\nc` → 3 parágrafos, sem `\n` nos runs.
- `p845_bidi_runs_linhas_vazias_consecutivas`: `a\n\nb` → 3 parágrafos, meio vazio.
- `p845_bidi_runs_sem_newline_um_paragrafo`: texto simples → 1 parágrafo, 1 run.
- `p845_try_shape_multilinha_empilha_linhas`: 3 `TextShaped` com y crescente.
- `p845_shaped_width_multilinha_max_das_linhas`: largura = max das linhas.

---


## P977 — feature `ssty` aplicada a texto math em tamanho de script

**Medição** (`typst-passo-975` Fase A): o vanilla aplica
`feat("ssty", 1)` a `MathSize::Script` e `feat("ssty", 2)` a
`ScriptScript` (`lab/typst-original/crates/typst-library/src/text/
mod.rs:1457-1460`); em NewCMMath a feature é AlternateSubst GSUB
(base → [`.st`, `.sts`]). Medição exacta: subscrito de `$ K_n $` —
cristalino 4.62pt (advance base 600du), vanilla 5.44pt (`.st`, 706du).

**Decisão**: no loop de shaping (`try_shape`), quando `style.math`,
`style.math_size` é `Script` ou `ScriptScript` **e o sub-run é
ssty-elegível** (`ssty_eligible_text` — ver correcção de escopo do
Passo 1129 abaixo), o `rustybuzz::shape` recebe a feature `ssty`
com o valor do nível (1 ou 2), em vez da lista vazia. Os glifos shaped
passam a ser os `.st`/`.sts` e seguem o caminho normal de subsetting
(`extended_glyph_ids`). `ShapeCache::key` ganha o nível ssty (o resultado
do shaping muda com ele). O fallback raro (`try_shape` → None, item fica
`Text` com glifo base) é residual aceite — só ocorre quando o rustybuzz
não lê a fonte, caso já degradado hoje. A métrica correspondente
(advance/ink com a substituição) está em `infra/font_metrics.md` §P977 —
os dois lados têm de concordar (lição P772o).

**Correcção de escopo (Passo 1129)** — a condição original só tinha sido
medida contra `$ K_n $` (subscrito de 1 carácter) e ficou registada sem
restrição de comprimento; a implementação pedia a feature `ssty` para
QUALQUER sub-run com `style.math` e `math_size` de script, incluindo
runs de texto multi-carácter (ex.: a legenda de
`underbrace(x, "cinco estrelas")`, que é `Content::Text`, não
`Content::MathIdent`/`MathText`). No vanilla, essa distinção já existe
na origem: um glifo matemático atómico (`GlyphItem`, `resolve_symbol`)
é shaped por `typst-layout/src/math/shaping.rs::shape_text`, que força
incondicionalmente `buffer.set_script(Tag(b"math"))` — só aí `ssty` (que
a NewCMMath-Book regista exclusivamente sob o script GSUB `math`,
confirmado via `fontTools`) chega a aplicar-se; texto multi-carácter
(`TextItem`, `resolve_text` — strings, legendas) é shaped por
`typst-layout/src/math/text.rs::layout_text` → `crate::inline::
layout_inline`, o shaper de parágrafo comum, que nunca força o script
`math`, pelo que o pedido genérico de `ssty` feito por `text/mod.rs::
tags()` fica inerte nesse run. Este ficheiro só tem UM ponto de decisão
(o pedido da feature, aqui em `try_shape`) — `buffer.set_script(...)`
permanece incondicional a `style.math` (linha separada): não há medição
que mostre outra feature GSUB math-only a ser indevidamente activada por
isso, e mexer no script forçado sem essa medição seria correcção
especulativa (ADR-0108).

**Excepção numérica (refinamento dentro do mesmo Passo 1129)** — a
primeira versão usava "sub-run de 1 carácter" como única condição e
regrediu `a_10` (corpus `.typ/sec_39.typ`): medição directa do vanilla
(`$ a_1 + a_10 = b $`) mostra CADA dígito de "10" em `BT…Tj…ET` próprio
com largura `.st` (569du, não a base 500du) — "10" não é um `TextItem`
de texto, é um `NumberItem` (`resolve_text`, `ir/resolve.rs:284-292`,
predicado `num`: dígitos ASCII + ≤1 ponto decimal + ≥1 dígito), cujo
`layout_number` (`math/text.rs`) chama `GlyphFragment::synthetic` por
carácter — cada dígito força o script `math` individualmente, como um
`GlyphItem`. Condição corrigida (final): a feature `ssty` é pedida
quando `ssty_eligible_text(&subrun.text)` — função local que replica o
predicado `num` (1 carácter **ou** dígitos ASCII com ≤1 ponto e ≥1
dígito; mesma regra que `font_metrics::ssty_eligible_text`, os dois
lados têm de concordar, lição P772o). Basta deixar de pedir `ssty` para
sub-runs não-elegíveis (texto multi-carácter não-numérico) para o
resultado do shaping coincidir com o vanilla (a substituição não ocorre
sem o pedido, independentemente do script activo) — `a_10` continua a
receber `.st` por dígito. Ver `infra/font_metrics.md` §P977 para a
correcção espelhada do lado da medição de largura/tinta.


## Proveniência `TextItem` no shaping matemático

`TextStyle.math_text_item=true` preserva a rota morfológica de texto inline
matemático. O shaper mantém `style.math=true`, a família e fallback
matemáticos, tamanho, idioma, direção, tracking, variações e todas as features
não relacionadas a `ssty`.

Para essa proveniência, o nível `ssty` é zero e a feature não é pedida,
mesmo em Script ou ScriptScript e mesmo para texto singular ou numérico. Isso
evita aplicar a variante `.st/.sts` que pertence às rotas de glifo e número.

Quando `math_text_item=false`, permanece o contrato de `ssty` já definido:
MathIdent, MathText, NumberItem, GlyphFragment e formas extensíveis pedem a
variante compatível com o tamanho quando `ssty_eligible_text` aceitar o
conteúdo. O predicado numérico continua equivalente ao de `NumberItem`.

A chave de `ShapeCache` distingue o nível efetivamente calculado. Não há
exceção por caractere, família ou fixture, nem alteração de `attach`, fonte,
fallback ou outra feature OpenType.


## Aceitação

TextItem em script não recebe `.st/.sts`; glifos e números continuam a
recebê-la. Família, posições, tamanho e shaping não-`ssty` permanecem iguais.
