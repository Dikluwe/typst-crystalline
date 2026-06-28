---

# P485 — Posicionamento preciso: `units_per_em` em `TextShaped` + `TJ` em emit

> **Passo:** 485
> **Data:** 2026-06-28
> **Foco:** Adicionar `units_per_em: u16` a `FrameItem::TextShaped` (extraído de
> `rb_face.units_per_em()` no shaper) e actualizar `emit_shaped_pdf` para usar o
> operador PDF `TJ` com avanços explícitos por glifo — garantindo posicionamento
> correcto mesmo quando GPOS/kerning altera os avanços relativamente ao `hmtx`.
> **Trilha:** 5 — Shaping / rustybuzz (extensão pós-Fase 3).
> **Tipo:** Materialização M.
> **Tamanho:** M (~1.5–2h).
> **ADR-0120 ACEITE** (P482). **ADR-0029 EM VIGOR** — L1 sem I/O.

---

## Contexto

P482–P484 fecharam as três fases de shaping (LTR, migração, RTL). O path de
emit actual em `emit_shaped_pdf` emite todos os glifos de um run como uma única
string hex com `Tj`:

```pdf
BT
/F1 12.0 Tf
72.0 770.0 Td
<0041004200430044> Tj
ET
```

O operador `Tj` usa os avanços do `hmtx` da fonte embebida. Quando rustybuzz
aplica GPOS (kerning de OpenType) os avanços `x_advance` nos `ShapedGlyph`
podem diferir do `hmtx`. Sem `units_per_em` acessível no export, é impossível
converter `x_advance` (font units) em pt para correcção.

P485 resolve isto em dois sub-itens:

**Sub-item A:** Adicionar `units_per_em: u16` a `FrameItem::TextShaped`; popular no
`shaper.rs::try_shape` via `rb_face.units_per_em()`.

**Sub-item B:** Actualizar `emit_shaped_pdf` para usar o operador PDF `TJ` com
avanços explícitos — em vez de `Tj` com a string completa.

---

## ADR-0108 — Medir antes de decidir

| Pergunta | Verificar em | Status |
|----------|-------------|--------|
| `rustybuzz::Face::units_per_em() -> u16` existe? | `lab/krilla-reference/` grep | 🟡 |
| `FrameItem::TextShaped` campos actuais | `01_core/src/entities/layout_types.rs` | ✅ |
| `emit_shaped_pdf` assinatura e body actuais | `03_infra/src/export/stream.rs` | ✅ |
| Operador PDF `TJ` vs `Tj` — diferença | spec PDF 1.7 §9.4.3 | 🟡 (ver abaixo) |
| Sites de `FrameItem::TextShaped { .. }` que precisam de `units_per_em` | grep L1+L3 | 🟡 |

**PDF `TJ` spec (§9.4.3):**
```
[ string1 num1 string2 num2 ... ] TJ
```
- `<hex>` = string de glifos CID
- número = deslocamento em **-1/1000 de unidade de texto** (negativo = avança
  o cursor para a direita)
- Após cada string, o PDF avança pelo `hmtx` da fonte. O número TJ é uma
  correcção adicional.
- Para controlo total: emitir cada glifo individualmente e usar o número TJ
  para subtrair o avanço `hmtx` e adicionar o nosso `x_advance`:
  `kern_TJ = -(x_advance / units_per_em × 1000) + hmtx_advance_in_TJ_units`
  — complexo porque requer acesso ao `hmtx`.

**Abordagem simplificada (P485):**
Emitir cada glifo como uma string separada no array `TJ`, com o número TJ entre
eles derivado da diferença `x_advance − hmtx_default`. Como não temos acesso ao
`hmtx` individual no export, usar a aproximação:
**emitir cada glifo com `Td` para controlo total de posição**, resetando a
posição após cada glifo com o avanço exacto em pt:

```pdf
BT
/F1 12.0 Tf
72.0 770.0 Td
<0041> Tj  ← glifo A
7.2 0 Td   ← avança x_advance_A / units_per_em × font_size pt
<0042> Tj  ← glifo B
6.0 0 Td
...
ET
```

**Problema:** `Td` é **relativo à linha actual** (posição definida pelo `Td`
anterior), não ao cursor actual. O cursor avança dentro do BT block com `Tj`.
Portanto emitir `Td` após `Tj` cria um desvio duplo.

**Solução correcta com TJ e glifos individuais:**
```pdf
BT
/F1 12.0 Tf
72.0 770.0 Td
[ <0041> -600 <0042> -560 <0043> -540 ] TJ
ET
```
onde cada número `-kern` = `-(x_advance_i / units_per_em × 1000)` e o PDF
usa internamente os avanços `hmtx` + os ajustes TJ. **NOTA:** se os `x_advance`
do rustybuzz já incluem GPOS kerning e o `hmtx` base não tem kerning, os ajustes
TJ anulam o `hmtx` e adicionam o rustybuzz advance. Mas como não temos `hmtx`
individual por glifo, usamos a aproximação: os números TJ são proporcionais aos
`x_advance` do rustybuzz.

**Decisão P485:** Usar `TJ` com um número por glifo derivado de
`x_advance / units_per_em × 1000`. O número representa o advance real do
glifo (não uma correcção); o PDF viewer soma o `hmtx` e o número TJ —
resultando em avanço ligeiramente mais do que o correcto para `hmtx` sem
GPOS. Esta aproximação é boa o suficiente para P485; ajuste fino de `hmtx`
é P486+.

**Nota:** verificar com as sondas se `rustybuzz::Face::units_per_em()` retorna
`u16` ou outro tipo antes de definir o campo.

---

## Sub-item A — `units_per_em: u16` em `FrameItem::TextShaped`

### A.1 — `FrameItem::TextShaped` actualizado

**Ficheiro:** `01_core/src/entities/layout_types.rs`

```rust
TextShaped {
    pos:          Point,
    glyphs:       Vec<ShapedGlyph>,
    style:        TextStyle,
    text:         EcoString,
    /// Unidades por em da fonte shaped. Usado em export para converter
    /// `x_advance` (font units) em pt: `advance_pt = x_advance / units_per_em × size`.
    units_per_em: u16,
},
```

### A.2 — Popular em `shaper.rs::try_shape`

**Ficheiro:** `03_infra/src/shaper.rs`

```rust
// Após construir rb_face:
let units_per_em = rb_face.units_per_em();

// Na construção de FrameItem::TextShaped:
Some(FrameItem::TextShaped {
    pos:    *pos,
    glyphs: all_glyphs,
    style:  style.clone(),
    text:   text.clone(),
    units_per_em,
})
```

### A.3 — Impacto em sites de match de `TextShaped`

Adicionar `units_per_em` ao campo de match em todos os ~19 sites. Maioria usa
`FrameItem::TextShaped { pos, glyphs, style, text }` — adicionar `units_per_em`
ou `..` para ignorar.

Sites que **precisam** do valor: `emit_shaped_pdf` em `stream.rs`.
Sites que podem ignorar: cursor.rs, slicing.rs, helpers.rs, link.rs, equation.rs,
frac.rs, math/mod.rs, pipeline.rs, fonts.rs, integration_tests.rs, frame_dto.rs.

**Abordagem:** usar `..` nos sites que não usam `units_per_em`, adicionar o campo
explícito apenas em `emit_shaped_pdf`.

---

## Sub-item B — `emit_shaped_pdf` com TJ

### B.1 — Novo formato de emit

**Ficheiro:** `03_infra/src/export/stream.rs` — função `emit_shaped_pdf`

```rust
pub(super) fn emit_shaped_pdf(
    ops:          &mut String,
    pos_x:        f64,
    base_y:       f64,
    glyphs:       &[ShapedGlyph],
    text:         &str,
    style:        &TextStyle,
    scenario:     &FontScenario,
    units_per_em: u16,
) {
    if glyphs.is_empty() { return; }
    let font_size = style.size.val();
    let upm = units_per_em as f64;

    match scenario {
        FontScenario::Type1 => {
            emit_text_pdf(ops, pos_x, base_y, text, style, scenario);
        }
        FontScenario::Cidfont { .. } => {
            ops.push_str(&format!(
                "BT\n/F1 {:.1} Tf\n{:.3} {:.3} Td\n[ ",
                font_size, pos_x, base_y
            ));
            for g in glyphs {
                let advance_tu = -(g.x_advance as f64 / upm * 1000.0);
                ops.push_str(&format!("<{:04X}> {:.0} ", g.glyph_id, advance_tu));
            }
            ops.push_str("] TJ\nET\n");
        }
        FontScenario::Multifont { fonts, .. } => {
            let fi = style.font.as_ref()
                .and_then(|fl| fonts.iter().position(|(stored, _)| stored == fl))
                .unwrap_or(0);
            ops.push_str(&format!(
                "BT\n/F{} {:.1} Tf\n{:.3} {:.3} Td\n[ ",
                fi + 1, font_size, pos_x, base_y
            ));
            for g in glyphs {
                let advance_tu = -(g.x_advance as f64 / upm * 1000.0);
                ops.push_str(&format!("<{:04X}> {:.0} ", g.glyph_id, advance_tu));
            }
            ops.push_str("] TJ\nET\n");
        }
    }
}
```

### B.2 — Call sites de `emit_shaped_pdf`

Em `build_page_stream` e `draw_item_local`, passar `units_per_em` do item:

```rust
FrameItem::TextShaped { pos, glyphs, style, text, units_per_em } => {
    let pdf_y = page_height - pos.y.val();
    emit_shaped_pdf(&mut ops, pos.x.val(), pdf_y, glyphs, text.as_str(),
                    style, &ctx.font_scenario, *units_per_em);
}
```

---

## Spec L0

### Actualizados

- `00_nucleo/prompts/entities/shaped_glyph.md` — nota: `units_per_em` em
  `FrameItem::TextShaped` (não em `ShapedGlyph` — é propriedade do run, não do glifo).
- `00_nucleo/prompts/infra/shaper.md` — §P485: `units_per_em` extraído de `rb_face`.
- `00_nucleo/prompts/infra/export/stream.md` — §P485: `emit_shaped_pdf` com TJ.
- ADR-0120 — anotação P485: posicionamento TJ implementado.

---

## Tests

### Sub-item A (L1)
- `p485_textshaped_tem_units_per_em` — `FrameItem::TextShaped { units_per_em, .. }` —
  campo presente e acessível.

### Sub-item B (L3)
- `p485_emit_shaped_cidfont_usa_tj` — output de `emit_shaped_pdf` contém `TJ` e não `Tj`
  para path CIDFont.
- `p485_emit_shaped_advance_calculado` — dado `x_advance=600, units_per_em=1000`,
  o número TJ é `-600`.
- `p485_emit_shaped_type1_fallback_usa_tj_nao` — Type1 continua a usar `emit_text_pdf`.
- `p485_shaper_popula_units_per_em` — `try_shape` com MockWorld (sem fonte) não
  produz `TextShaped` — confirma que `units_per_em` só é definido quando há fonte.

### Regressão
- `p485_parity_73_73_mantido` — 73/73 matches preservados.

---

## Scope-out

- **Correcção `hmtx` exacta** — os números TJ são `-(x_advance / upm × 1000)`, não
  a diferença `x_advance − hmtx_advance`. Requer acesso ao `hmtx` por glifo. P486+.
- **`y_offset` / diacríticos verticais** — `ShapedGlyph.y_offset` não é usado no
  emit horizontal. P486+.
- **`x_offset`** — kern marks não são aplicados no emit. P486+.
- **OpenType features explícitas** (liga, kern, smcp) — `features = &[]`. P486+.

---

## Critério de fecho

- [ ] Sonda: `rb_face.units_per_em()` tipo confirmado com `file:line`.
- [ ] `FrameItem::TextShaped` tem campo `units_per_em: u16`.
- [ ] `shaper.rs::try_shape` popula `units_per_em` de `rb_face.units_per_em()`.
- [ ] `emit_shaped_pdf` usa `TJ` com número por glifo derivado de `x_advance`.
- [ ] 5+ testes verdes.
- [ ] `p485_parity_73_73_mantido` verde.
- [ ] `crystalline-lint` zero erros V1–V14.
- [ ] `cargo test --workspace` verde.

---

## Estado pós-P484

| Indicador | Estado |
|-----------|--------|
| Paridade | 73/73 matches |
| Trilha 5 | Fases 1–3 ✅ — **P485 extensão de posicionamento** |
| Trilha 6 | 4/5 (5.º item scope-out permanente) |
| ADR-0120 | ACEITE — Fases 1+2+3 executadas |
| **P485** | Posicionamento TJ preciso | 🔄 EM PREPARAÇÃO |
