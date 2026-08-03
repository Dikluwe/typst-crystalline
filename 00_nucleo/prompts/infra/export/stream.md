# Prompt L0 — `infra/export/stream` — PageContext + emit unificado
Hash do Código: 42f91896

**Camada**: L3
**Ficheiro alvo**: `03_infra/src/export/stream.rs`
**Criado em**: 2026-05-19 (P307c)
**Atualizado em**: 2026-07-03 (P548 — correção do sinal do delta TJ)
**ADRs**: ADR-0098 (SSoT helpers unificados pós-P281)

---

## Contexto

Cluster central de emit PDF — implementação do "single source of
truth" pós-P281: helpers unificados invocados tanto pelo caminho
top-level (`build_page_stream`) como pelo caminho local em Group
(`draw_item_local`).

- `FontScenario` enum dispatch para `emit_text_pdf` / `emit_glyph_pdf`.
- `PageContext` agregador de `ptr_to_idx`, `img_refs`, `pat_ptr_to_idx`, `pat_refs`, `font_scenario`.
- `build_page_stream` orquestra emit page-level.
- `draw_item_local` emit recursivo em Groups (clip masks, transforms).
- Shape primitives: `emit_shape_path_local` + `emit_rounded_rect_ops` + `emit_stroke_paint` + `line_rg_prefix`.

## Restrições estruturais

- L3. Usa formatadores `format!`/`String` — não toca FS.
- `FontScenario` e `PageContext` são `pub(crate)` — instanciados em `super::builder` mas atravessam tipo nas chamadas.
- Helpers de emit são `pub(super)` — chamados por `super::builder` e self.
- `FrameItem::Image` inclui `orientation` (P776); o emit compõe a matriz `cm`
  com a transformação EXIF correspondente, preservando os bytes originais do
  JPEG/PNG (paridade `typst-pdf/src/image.rs::exif_transform`). A matriz é emitida
  com precisão de 5 casas decimais (P777), replicando o vanilla e evitando desvios
  de sub-pixel nas bordas em orientações com flip/rotate.
- Excede limite 800 LOC ADR-0037 Regra 2 (~685 LOC). Sub-divisão futura em `stream/{page,text,shape,draw}.rs` em P-stream-decomp dedicado se justificado.

## Interface

```rust
pub(crate) enum FontScenario<'a> {
    Type1,
    Cidfont { char_to_gid, glyph_mapping, glyph_to_nominal },
    Multifont { fonts, per_font_char_to_gid, per_font_glyph_mapping, per_font_glyph_to_nominal },
}
pub(crate) struct PageContext<'a> { /* ptr_to_idx, img_refs, pat_*, font_scenario */ }
impl<'a> PageContext<'a> {
    pub(crate) fn type1(...) -> Self;
    pub(crate) fn cidfont(..., char_to_gid, glyph_mapping, glyph_to_nominal) -> Self;
    pub(crate) fn multifont(..., fonts, per_font_char_to_gid, per_font_glyph_mapping, per_font_glyph_to_nominal) -> Self;
}

pub(super) fn emit_text_pdf(ops, pos_x, base_y, text, style, scenario);
pub(super) fn emit_glyph_pdf(ops, pos_x, base_y, glyph_id, size, scenario);
pub(super) fn emit_stroke_paint(...);
pub(super) fn line_rg_prefix(color: &Option<Color>) -> String;
pub(super) fn build_page_stream(page: &Page, ctx: &PageContext) -> Vec<u8>;
pub(super) fn emit_shape_path_local(ops, kind, w, h);
pub(super) fn emit_rounded_rect_ops(...);
pub(super) fn draw_item_local(ops, item, ctx, ...);
```

## Invariantes (ADR-0098)

- `emit_text_pdf` / `emit_glyph_pdf` / `line_rg_prefix` chamados em ambos os caminhos top-level e local — alteração simétrica garantida por construção.
- Para `color: None` em Line, `line_rg_prefix` retorna `""` (bit-exact pré-P285).
- Hash de `stream.rs` é métrica de aderência ao padrão (P282-P306, 23 passos cumulativos).

## Critérios de verificação

- Tests P281+ em `tests.rs` validam paridade entre caminhos.
- Test `p282_line_stroke_color_simetric` valida que `RG` injecção é simétrica.
- Snapshot binário em `p307b_snapshot_tests.rs` valida invariante observable.

## §P486 — `x_offset` no TJ array (Sub-item B)

**P486** adiciona suporte a `ShapedGlyph.x_offset` no operador PDF `TJ` em
`emit_shaped_pdf`. Aplicado simetricamente em `Cidfont` e `Multifont`.

### Fórmula por glifo com x_offset != 0

```
[ {-x_offset_tu} <GID> {advance_tu} ... ] TJ
```

Onde:
- `x_offset_tu = -(x_offset / upm * 1000)` — pré-glifo: desloca cursor
- `advance_tu` — ver §P520 (delta model)

O `x_offset` é uma translação visual do glifo actual; não altera o avanço
para o glifo seguinte.

### `y_offset` — scope-out confirmado

`y_offset` requer sequências `Td` (saída do array TJ) e não há corpus LTR com
`y_offset != 0`. Scope-out declarado — não implementado em P486.

### Testes adicionados P486

- `p486_emit_x_offset_zero_equivale_p485`: x_offset=0 → sem número antes do GID
- `p486_emit_x_offset_nonzero_aplica_ajuste`: x_offset=-50, upm=1000 → "50 " antes do GID
- `p486_emit_x_offset_positivo`: x_offset=30, upm=1000 → "-30 " antes do GID

## §P520 — Kerning via delta model no TJ

**P520** corrige o avanço do operador `TJ` para reflicta kerning aplicado
pelo shaper (rustybuzz). O CIDFont declara `/W` com larguras nominais
(`hmtx`), pelo que o `TJ` deve conter apenas o *delta* entre a largura
declarada e o avanço real.

### Fórmula por glifo

```
nominal    = glyph_to_nominal.get(glyph_id).unwrap_or(x_advance)
advance_tu = (nominal - x_advance) as f64 / upm * 1000.0
[ {-x_offset_tu} <GID> {advance_tu} ... ] TJ
```

No operador PDF `TJ`, cada número é **subtraído** da coordenada horizontal
antes de desenhar o próximo glifo. Portanto:

- `advance_tu` **positivo** quando `x_advance < nominal` (kerning negativo —
  aproxima o próximo glifo).
- `advance_tu` **negativo** quando `x_advance > nominal` (kerning positivo —
  afasta o próximo glifo).
- `glyph_to_nominal` é construído em `builder.rs` a partir do `hmtx` da fonte
  original, para todos os `glyph_id` usados no documento (`collect_glyph_ids`
  + codepoints mapeados).

### Testes adicionados P520

- `p520_emit_shaped_kerning_delta`: x_advance=599, nominal=639, upm=1000 → delta = +40

---

## Histórico de Revisões

| Data | Motivo | Ficheiros afetados |
|------|--------|--------------------|
| 2026-05-19 | Criação — P307c: PageContext + emit unificado | `stream.rs` |
| 2026-07-03 | P548 — correção do sinal do delta TJ: `advance_tu = (nominal - x_advance)` em vez de `(x_advance - nominal)` | `stream.rs`, `stream.md`, `builder.md`, `tests.rs` |

---

## §P788 — `draw_item_top`: flip Y para filhos de `FrameItem::Link` ao nível da página

**Decisão:** a emissão top-level por item foi extraída de `build_page_stream`
para `draw_item_top(ops, item, page_height, ctx) -> ops` (recebe/devolve
`ops` por valor — braço `Link` chama recursivamente sem conflito de borrow).
O braço `FrameItem::Link` passa a desenhar os filhos **pelo caminho
top-level com flip Y** (`pdf_y = page_height - pos.y`).

**Causa raiz (medida, P786 A9 + `#link` genérico):** os filhos de Link ao
nível da página eram desenhados por `draw_item_local` — que NÃO aplica flip
(assume a matriz `cm` invertida de um `Group` envolvente). Sem Group, os
filhos apareciam com `pos.y` crua → fundo da página (medido: y≈754 em vez
de y≈68; vanilla: coordenadas idênticas após a correção). `draw_item_local`
mantém o seu papel dentro de `Group` (matriz já invertida) — inalterado.

**Critério de aceitação:** teste `p788_link_top_level_filho_tem_flip_y` —
filho `Text` a (70,100) em página 800 → stream contém `70.0 700.0 Td`
(nunca `70.0 100.0 Td`).

## P836 — selecção de fonte por variações

`FontScenario::Multifont` e `font_index_for_style` passam a chavear por
`(FontList, FontVariant, FontVariations)`: o índice `/F{n}` de cada
run de texto é resolvido comparando também `style.variations`
(`unwrap_or_default`), coerente com a chave da pipeline/builder.

## P906 — `emit_glyph_pdf`: dois bugs em cadeia, `/F1` hardcoded + remap de subsetting em falta

**Contexto**: ao confirmar visualmente o mecanismo de esticamento horizontal
(`engine/layout.md` §P906), `underbracket(a+b+c)` produzia PDF com glifos
invisíveis/`.notdef`, apesar do layout (largura, posição) estar correcto e
testado. Medido directamente no PDF exportado (`pdftotext -bbox`,
instrumentação temporária), não inferido.

**Achado 1**: `emit_glyph_pdf` escrevia sempre `/F1 Tf` incondicionalmente,
mesmo no ramo `Multifont` (várias fontes embutidas, `/F1`.._`/Fn`). Corrigido
usando `per_font_glyph_reverse` (mapa reverso glyph_id→char por fonte, o
MESMO `build_math_glyph_reverse_map` já usado no subsetting DEBT-9/P45,
partilhado — ver `builder.rs` abaixo) para encontrar em qual fonte o
`glyph_id` está efectivamente presente. `FontScenario::Cidfont` não muda
(só uma fonte candidata, `/F1` sempre correcto aí).

**Achado 2, só visível depois de corrigir o 1º**: nem `Cidfont` nem
`Multifont` aplicavam `remap_glyph_id`/`glyph_mapping` (P516, já aplicado em
`emit_text_pdf`/`emit_shaped_pdf`) ao `glyph_id` recebido — desenhava sempre
o índice ORIGINAL (pré-subsetting) da fonte completa, mas a fonte
efectivamente embutida está subsetada (renumerada) sempre que `glyph_mapping`
não é vazio. `<XXXX> Tj` referenciava um slot de glifo errado/`.notdef` na
fonte embutida mesmo com `/Fn` já correcto. Corrigido aplicando o mesmo
`remap_glyph_id` já usado no caminho de texto.

**Pré-existente, partilhado com o eixo vertical**: nenhum dos dois bugs é
específico ao esticamento horizontal — `layout_stretchy_delimiter`
(vertical, delimitadores altos) usa exactamente o mesmo `emit_glyph_pdf`.
Nunca antes exercitado com dados reais porque `FallbackFontMetrics` não
implementava `vertical_glyph_variants`/`assembly` (gap adiado desde P891/
P893, corrigido no mesmo passo — `infra/font_metrics.md` §P906) — sem isso,
`emit_glyph_pdf` nunca recebia um `glyph_id` de uma fonte diferente de `/F1`
nem precisava de remap, então os dois bugs ficaram latentes.


## P953 — `cm` do `FrameItem::Group`: VERIFICADO correcto (`[a, -b, -c, d]`)

**Investigação** (`typst-passo-953` Fase A): a direcção de `#rotate` foi
suspeita de estar invertida, mas a medição final (zoom a alta resolução do
render + transformação efectiva do texto nos dois PDFs) confirmou que a forma
original `[a, -b, -c, d]` (negação de `b` e `c`) está **correcta** — o
efectivo no espaço de ecrã é idêntico ao do vanilla (`[cos, sin, -sin, cos]`
para `rotate(45deg)`, rotação horária descendente nos dois). Uma alteração
experimental (negar `b`/`d` + inverter o sinal em L1) foi medida como
incorreta e **revertida sem deixar rasto**. Registo: a convenção actual está
validada contra o vanilla — não mudar sem uma medição equivalente.

## P956 — `StreamMode`: modo verboso (vanilla-espelhado, novo padrão) vs compacto (formato Passo 20)

**ADR-0126 (emendada P956)**: o exportador passa a ter dois modos de emissão
de texto. **Verboso = padrão de produção** (espelha a semântica do vanilla
operador a operador); **Compacto = o formato actual** (Passo 20), preservado
atrás da flag `--compact` sem alteração de bytes. `BDC`/`EMC` (PDF tagueado)
**não** entra neste passo — eixo separado (ADR-0126 §1.3).

### O padrão vanilla (medido — typst 0.15.1, `temp/p956/min-vanilla.pdf`)

Por run de texto, o vanilla emite (stream descomprimido, `mutool clean -d`):

```pdf
q 1 0 0 -1 70.86614 763.78564 cm
/c0 cs 0 scn
BT 0 Tr /f0 11 Tf 1 0 0 -1 0 0 Tm [(…)] TJ
ET
Q
```

- `q` + `cm 1 0 0 -1 x y`: isola o bloco e carrega posição + flip Y (o frame
  vanilla é y-down; o flip é por bloco, não pré-calculado).
- `/cN cs … scn`: colour space nomeado (em `/Resources/ColorSpace`) + cor de
  preenchimento, declarado **por bloco** (preto = `/c0 cs 0 scn`, com c0
  ICCBased gray; cores = ICCBased sRGB, 3 componentes).
- `BT 0 Tr /f0 11 Tf 1 0 0 -1 0 0 Tm […] TJ ET`: `Tr` explícito, `Tm` com
  duplo flip (glifos direitos; a posição está toda no `cm`), kerning no array
  `TJ`. Depois `Q`.

### Emissão verbose cristalina (alvo deste passo)

```pdf
q 1 0 0 -1 {x} {y} cm
/c0 cs {r} {g} {b} scn
BT 0 Tr [/F{n} {size} Tf] [{tc}] 1 0 0 -1 0 0 Tm [{…}] TJ ET
Q
```

Regras:

- **Posição**: `x,y` = `pos` do item nas coordenadas locais (y-down) — **sem**
  `page_height − y` no emit; o `cm` faz o flip. O MESMO bloco serve top-level
  (`draw_item_top`) e local (`draw_item_local` em Groups): a `cm` do Group já
  compõe — o verbose unifica os dois caminhos (hoje divergem no cálculo de y).
- **`Tm` sempre `1 0 0 -1 0 0`** — posição inteiramente no `cm`.
- **`0 Tr` explícito** em todos os blocos; faux-bold (P139) usa `2 Tr` +
  `{stroke} w` no mesmo envelope.
- **`cs`/`scn` por bloco**: fill = `style.fill` ou preto `0 0 0` por omissão.
  `/c0` é o colour space sRGB ICCBased declarado nos recursos da página
  (ver `builder.md` §P956).
- **Conteúdo do array `TJ` inalterado** (P485/P486/P520/P548 — delta model,
  `x_offset`, remap de subsetting): o helper que constrói o array é
  partilhado pelos dois modos.
- **`emit_text_pdf`** (Type1 fallback / `Text` não-shaped): mesmo envelope,
  `(…) Tj`. **`emit_glyph_pdf`** (stretchy): mesmo envelope, `<gid> Tj`.
- **`/F1..N` mantidos** (nomes de recurso são arbitrários em PDF; o decalque
  compara geometria e operadores, não nomes).
- Line/Image/Shape/Group: **inalterados** neste passo (já usam `q…Q`/`cm`);
  o escopo verbose é a emissão de texto (`Text`/`TextShaped`/`Glyph`).

### Dispatch

`PageContext` ganha `mode: StreamMode` (construtores `type1`/`cidfont`/
`multifont` recebem-no — ver `builder.md` §P956). `draw_item_top` e
`draw_item_local` despacham: `Compact` → os helpers actuais, **byte-inalterados**;
`Verbose` → variantes verbose dos mesmos helpers (partilham o construtor do
array TJ e o `fill`/font selection).

### Regra para testes (Fase B.3 de P956)

Todo o teste que assecre bytes/operadores do stream declara **explicitamente**
o modo que espera (`StreamMode::Verbose` ou `StreamMode::Compact`) — nenhum
teste fica ambíguo sobre qual formato está a verificar. Os testes actuais do
formato Passo 20 passam a declarar `Compact` (o formato que verificam não
muda); testes novos do verbose verificam o envelope `q/cm` + `Tm` + `0 Tr` +
`cs`/`scn` e a equivalência de posição final (`Td` compacto vs `cm`+`Tm`
verbose → mesma baseline).
