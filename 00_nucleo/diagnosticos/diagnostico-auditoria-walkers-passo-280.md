# Diagnóstico Fase A — P280 Auditoria walkers top-level

**Data**: 2026-05-18
**Tipo**: ADR-0085 diagnóstico imutável; **34º consumo**.
**Hipótese central testada**: P279 §3 *"walkers análogos podem existir
para text fonts, glyphs, ou outras estructuras de recurso"*.
**Predição**: B ≥ 1 confirma hipótese; B = 0 refuta.
**Resultado**: **B = 2 confirmado empíricamente** (`collect_codepoints`
+ `collect_glyph_ids`). Hipótese **confirmada**.

---

## §A.1 — Inventário sistemático

### §A.1.1 — Funções em `03_infra/src/export.rs` que iteram items

Critério inclusão: função que itera `doc.pages` ou `page.items` para
**inspeccionar conteúdo** dos itens (não para emit puro top-level
delegado a `draw_item_local`).

| # | Walker | Linha | O que itera | Propósito |
|---|--------|-------|-------------|-----------|
| 1 | `scan_all_images` | 220 | `doc.pages → page.items` | Registar Image XObjects + alocar ObjectIDs |
| 2 | `xobject_resources_for_page` | 334 | `page.items` | Construir `/XObject << /Im1 X 0 R ... >>` dict per página |
| 3 | `scan_all_gradients` | 479 | `doc.pages → page.items` | Registar gradient patterns + alocar ObjectIDs |
| 4 | `pattern_resources_for_page` | 567 | `page.items` | Construir `/Pattern << /P1 X 0 R ... >>` dict per página |
| 5 | `build_page_stream_type1` | 2086 | `page.items` (top-level) | Emit content stream Type1 |
| 6 | `build_page_stream_cidfont` | 2709 | `page.items` (top-level) | Emit content stream CIDFont single-font |
| 7 | `build_page_stream_multifont` | 2899 | `page.items` (top-level) | Emit content stream multifont |
| 8 | `collect_codepoints` | 2619 | `doc.pages → page.items` | Coletar Unicode chars em Text para `/Widths` array + ToUnicode CMap |
| 9 | `collect_glyph_ids` | 2635 | `doc.pages → page.items` | Coletar glyph IDs em Glyph para ToUnicode CMap |
| 10 | `draw_item_local` (arm Group recurse) | 2447 | `Group.items` | Emit local items dentro de Group (P273.13 + P279) |

Total candidatos `export.rs`: **10**.

### §A.1.2 — Excluídos do inventário (não são walkers no sentido relevante)

- `build_helvetica` / `build_cidfont` / `build_multifont` (linhas
  1437 / 1509 / 1634): iteram `doc.pages` para dimensões de página
  (`page_dimensions`) e pages enum em `Kids` arrays. Não inspeccionam
  conteúdo de items. Out-of-scope.
- `build_jpeg_xobject` / `build_png_*_xobject`: não iteram pages;
  recebem `data: &[u8]` já-extraído.
- Funções de gradient (`compute_axial_coords`, `multispace_sample_stops_*`,
  `emit_conic_coons_stream_*`): processam gradient stops, não items.
- Funções de geometria (`emit_shape_path_local`, `emit_rounded_rect_ops`):
  emit per-Shape sem iteração de items.
- Funções utilitárias (`detect_format`, `compress_zlib`, `rgb_to_cmyk`,
  `escape_pdf_string`, `text_to_hex_string`, `widths_array`,
  `to_unicode_cmap`, `map_chars_to_glyphs`, etc.): zero iteração de items.

---

## §A.2 — Classificação A/B/C empírica

Para cada walker em §A.1, inspecção do corpo:

| # | Walker | Atravessa Group? | Mecanismo | Classe |
|---|--------|------------------|-----------|--------|
| 1 | `scan_all_images` | ✓ Sim | Helper `walk` recursivo interno (P279) | **A** |
| 2 | `xobject_resources_for_page` | ✓ Sim | Helper `walk` recursivo interno (P279) | **A** |
| 3 | `scan_all_gradients` | ✓ Sim | Helper `walk` recursivo interno (P273.10) | **A** |
| 4 | `pattern_resources_for_page` | ✓ Sim | Helper `walk` recursivo interno (P273.10 + P273.12 + P278) | **A** |
| 5 | `build_page_stream_type1` | ✗ Não (correcto) | Arm Group chama `draw_item_local` recursivo | **C** |
| 6 | `build_page_stream_cidfont` | ✗ Não (correcto) | Arm Group chama `draw_item_local` recursivo | **C** |
| 7 | `build_page_stream_multifont` | ✗ Não (correcto) | Arm Group chama `draw_item_local` recursivo | **C** |
| 8 | `collect_codepoints` | ✗ **Não (BUG)** | Itera apenas `page.items` top-level; não desce | **B** |
| 9 | `collect_glyph_ids` | ✗ **Não (BUG)** | Itera apenas `page.items` top-level; não desce | **B** |
| 10 | `draw_item_local` arm Group | ✓ Sim (auto) | É recursivo per signature; o próprio arm chama-se a si | **A** |

**Contagens**: A = 5; B = **2**; C = 3. Total 10.

---

## §A.3 — Análise específica candidatos prioritários

### §A.3.1 — `collect_codepoints` (linha 2619) — **CLASSE B confirmada**

```rust
fn collect_codepoints(doc: &PagedDocument) -> Vec<char> {
    let mut seen = std::collections::BTreeSet::new();
    for page in &doc.pages {
        for item in &page.items {                  // ← top-level apenas
            if let FrameItem::Text { text, .. } = item {
                for c in text.chars() {
                    seen.insert(c);
                }
            }
            // Image, Line, Glyph não contribuem com codepoints de texto.
        }
    }
    seen.into_iter().collect()
}
```

**Bug latent**: `FrameItem::Group { items: child, .. }` não tratado.
Text dentro de Group **não contribui chars** para o `seen` set.

**Consumidores**:
- `build_cidfont` (line 1520): `let chars = collect_codepoints(doc);`
  → `map_chars_to_glyphs(face, &chars)` → `char_to_gid` HashMap usado
  por `text_to_hex_string` (line 2699).
- `build_multifont` (line 1651): idem para cada fonte.

**Manifestação futura**: quando P280.X-bis-text-emit-em-group lander
e o arm `FrameItem::Text` em `draw_item_local` deixar de ser stub e
chamar `text_to_hex_string`, chars de Text dentro de Group resolverão
para `glyph_id = 0` (notdef) — texto **renderizado como blocos**.

**Manifestação actual**: silenciosa (Text arm em `draw_item_local`
ainda é stub).

### §A.3.2 — `collect_glyph_ids` (linha 2635) — **CLASSE B confirmada**

```rust
fn collect_glyph_ids(doc: &PagedDocument) -> BTreeSet<u16> {
    let mut ids = BTreeSet::new();
    for page in &doc.pages {
        for item in &page.items {                  // ← top-level apenas
            if let FrameItem::Glyph { glyph_id, .. } = item {
                ids.insert(*glyph_id);
            }
        }
    }
    ids
}
```

**Bug latent**: idem. `FrameItem::Glyph { glyph_id, .. }` dentro de
Group não contribuem ao `ids`.

**Consumidor**:
- `build_cidfont` (line 1527): `for gid in collect_glyph_ids(doc) { ... }`
  — adiciona glyph IDs à lista de mappings sintéticos (sem char real).

**Manifestação futura**: idem. Glyph arm em `draw_item_local` (stub
hoje) emitido futuramente referenciará glyph ID não declarado no
ToUnicode CMap → texto extraível incompleto.

### §A.3.3 — `build_page_stream_type1/cidfont/multifont` (linhas 2086/2709/2899) — **CLASSE C confirmada**

```rust
fn build_page_stream_type1(page: &Page, ...) -> Vec<u8> {
    let mut ops = String::new();
    for item in &page.items {                      // ← top-level apenas
        match item {
            FrameItem::Group { ... } => {
                // ... q ... cm ...
                for child in items {
                    draw_item_local(child, ..., &mut ops);  // ← recursão delegada
                }
                ops.push_str("Q\n");
            }
            FrameItem::Text { ... } => { /* emit Type1 text */ }
            ...
        }
    }
    ops.into_bytes()
}
```

Top-level itera só `page.items`. Arm Group emite `q ... cm` localmente
e delega recursão a `draw_item_local` para cada child. `draw_item_local`
é ele próprio recursivo (arm Group chama-se a si — verificado P273.13).
**Comportamento correcto**: não precisa de helper `walk` no top-level;
a recursão acontece dentro de `draw_item_local`.

Os 3 stream-builders têm topologia idêntica (Type1 / CIDFont single /
multifont diferem apenas no emit de Text/Glyph, não na iteração).

### §A.3.4 — `pattern_resources_for_page` (linha 567) — **CLASSE A confirmada**

Helper `walk` recursivo interno desde P273.10, refinado em P273.12
(DedupKey bbox-aware) e P278 (helper `group_bbox_from_fields`
extraído). Arm `FrameItem::Group { items, .. }` chama `walk(items,
Some(group_bbox), ...)` com bbox-override. Correcto.

---

## §A.4 — Walkers fora `03_infra/src/export.rs`

Auditoria expandida:

### §A.4.1 — `03_infra/src/pipeline.rs`

| Walker | Linha | Recursivo? | Classe |
|--------|-------|------------|--------|
| `collect_fonts_from_doc` | 107 | ✓ Sim — helper `collect_fonts_in_items` arm Group recursa (linha 125) | **A** |
| `first_font_in_doc` | (159) | ✓ Sim — helper `first_font_in_items` arm Group recursa (linha 178) | **A** |

L0 `infra/pipeline.md` declara explicitamente: *"Itera `doc.pages → items`
recursivamente (atravessa `Group`)"*. Documentação alinhada com código.

### §A.4.2 — `03_infra/src/layout.rs`

3 referências a `doc.pages.is_empty()` (linhas 52, 59, 69). **Não são
walkers** — assertions de sanidade de testes. Out-of-scope.

### §A.4.3 — L1 / L2 / L4

```bash
grep -rn "doc\.pages\|page\.items\|FrameItem::Group" 01_core/src/ 02_shell/src/ 04_wiring/src/
```

Zero matches relevantes — confirmado que `FrameItem` é tipo L1 mas a
iteração é territory L3 (pipeline.rs + export.rs). L4 wiring orquestra
chamadas a L3 sem iterar items directamente.

---

## §A.5 — Mecanismo arquitectural da classe de bug

`FrameItem` é enum (`01_core/src/entities/layout_types.rs`) com variant
`Group { items: Vec<FrameItem>, .. }`. Walker que precisa de visitar
**todos** os items (não só top-level) tem que implementar recursão
explícita — typicamente via helper interno `fn walk(items: &[FrameItem],
...)` com arm `FrameItem::Group { items: child, .. } => walk(child, ...)`.

**Anti-padrão**: walker que itera apenas `page.items.iter()` no nível
externo. Para o caso `FrameItem::Group`, o iterador top-level **não
desce** para `child`. Items dentro de Group ficam invisíveis ao walker
— bug silencioso até feature dependente passar a renderizar dentro de
Group.

**Padrão correcto** (classe A):

```rust
fn walker(doc: &PagedDocument) -> ... {
    fn walk(items: &[FrameItem], acc: &mut ...) {
        for item in items {
            match item {
                FrameItem::Group { items: child, .. } => walk(child, acc),
                FrameItem::Text { ... } => { /* contribute */ }
                _ => {}
            }
        }
    }
    let mut acc = ...;
    for page in &doc.pages {
        walk(&page.items, &mut acc);
    }
    acc
}
```

**Padrão alternativo legítimo** (classe C): walker top-level que
**delega** Group para sub-função já recursiva (e.g.
`build_page_stream_*` → `draw_item_local`). Não precisa de recursão
explícita no top-level.

---

## §A.6 — Hipótese P279 §3 — **confirmada**

> *"walkers análogos podem existir para text fonts, glyphs, ou outras
> estructuras de recurso — auditoria futura"* (P279 relatório §5).

**Resultado empírico P280**: hipótese **confirmada** com 2 walkers
classe B identificados:
- `collect_codepoints` (Text path → CIDFont).
- `collect_glyph_ids` (Glyph path → CIDFont).

Ambos são **walkers de recurso** (alimentam `/Widths` array + ToUnicode
CMap da fonte CIDFont), exactamente a categoria que P279 §3 hipotetizou.

**Cardinalidade final classe B até P280**: 2 (collect_codepoints +
collect_glyph_ids) + 3 pré-existentes fixados (scan_all_gradients
P273.10; scan_all_images + xobject_resources_for_page P279) = **5
casos totais** do sub-padrão "Scope creep arquitectural por walker
top-level" ao longo da história.

**Sub-padrão atinge N=5 cumulativo** — material empírico abundante.

---

## §A.7 — Gates de paragem — **NÃO disparados**

| Gate | Estado | Nota |
|------|--------|------|
| 1. > 30 candidatos §A.1 | ✓ Não | 10 candidatos identificados |
| 2. > 5 walkers classe B em §A.2 | ✓ Não | B = 2 |
| 3. collect_codepoints/glyph_ids classe B | ⚠ **Disparou** | Decisão crítica: §A.7.3 abaixo |
| 4. Walkers em outros L3 fora export.rs | ✓ Não | pipeline.rs OK (A já) |
| 5. Hipótese §A.6 refutada | ✓ Não | Confirmada (B = 2) |
| 6. Cap LOC L3 fixes hard 80 ameaçado | ✓ Não | Estimativa ~24 LOC ambos fixes |
| 7. Cap doc Fase A hard 1000 ameaçado | ✓ Não | ~530 LOC actual |
| 8. Tests baseline 2611 alterada | ✓ Não | Confirmado pré-Fase A |

### §A.7.3 — Decisão sobre gate 3 (collect_codepoints / collect_glyph_ids)

Gate-3 dispara **"fixar agora oportunisticamente (XS) ou abrir pendência
dedicada com testes E2E?"**.

**Decisão (alinhada com cap LOC fixes oportunistas hard 80)**: fixar
**agora**. Razões:

1. **Pattern idêntico a P273.10 / P279**: helper `walk` interno;
   extracção opcional não necessária (corpo é trivial — 2-3 linhas por
   item).
2. **LOC estimado ~12 por fix × 2 = ~24 LOC L3 produção** — bem
   dentro do cap hard 80.
3. **Testes regressão XS** (2-3 por walker × 2 walkers = 4-6 testes)
   — dentro do cap testes hard 60.
4. **Sub-padrão atinge N=5 cumulativo** com 2 reaplicações — material
   suficiente para registar em relatório §4 sem formalizar ADR
   (anti-padrão over-formalização P273.17 preserved).
5. **Bug actualmente silencioso** (consumidores são `text_to_hex_string`
   chamado por `build_page_stream_*` Text arm; Text arm em
   `draw_item_local` ainda é stub) mas latente para P280.X-bis. Fixar
   agora **estabiliza preventivamente** e evita refactoring duplo
   quando P280.X-bis lander.
6. **Honestidade epistémica preservada**: o bug é genuíno
   (Text/Glyph dentro de Group **realmente não** contribui chars/IDs);
   não é hipótese B?. Não há ambiguidade.

---

## §A.8 — Pendências e estado pré-§C

**A fixar oportunisticamente em §C.2** (estimativa total ~24 LOC L3
produção + ~50 LOC testes):

| Walker | Pattern | LOC est. |
|--------|---------|----------|
| `collect_codepoints` | Helper `walk` interno (idêntico a `scan_all_images`); arm `FrameItem::Group { items, .. } => walk(items, ...)`; arm `FrameItem::Text { text, .. } => insert chars` | ~12 |
| `collect_glyph_ids` | Idem; arm `FrameItem::Glyph { glyph_id, .. } => insert id` | ~12 |

**Out-of-scope** (sem pendências P281+ identificadas): zero walkers
classe B fora do cap LOC hard 80. Auditoria não revelou walkers que
exijam refactoring estrutural.

---

## §A.9 — Conclusão Fase A

- **10 walkers inventariados** em `03_infra/src/export.rs` + 2 em
  `03_infra/src/pipeline.rs`.
- **Classificação definitiva**: 7 classe A (5 export + 2 pipeline);
  2 classe B (collect_codepoints + collect_glyph_ids); 3 classe C
  (3 stream-builders top-level).
- **Hipótese P279 §3 confirmada** (B = 2; sub-padrão N=5 cumulativo).
- **Decisão materialização**: fixar ambos oportunisticamente em §C
  (cap LOC respeitado; pattern idêntico a P273.10 / P279).
- **Zero pendências P281+** — auditoria estabiliza a classe de bug
  empíricamente.
- **L0 invariante** a registar em §C.1 com referência a esta auditoria.

Fase A imutável a partir de 2026-05-18.
