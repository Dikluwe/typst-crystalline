# Diagnóstico Fase A1 — P282 Paridade emit local vs top-level

**Data**: 2026-05-18
**Tipo**: ADR-0085 diagnóstico imutável; **36º consumo**.
**Hipótese central**: P281 unificação β-completa pode ter introduzido
divergências bit-exact entre emit top-level (em `build_page_stream`) e
local (em `draw_item_local`) para Text/Glyph/Line.
**Resultado**: **paridade total confirmada**; **zero fixes pendentes**.
Todas as 6 suspeitas listadas em P282 §A1 refutadas empíricamente.

---

## §A1.1 — Text emit: comparação top-level vs local

### Localização (`03_infra/src/export.rs`)

| Camada | Linha | Implementação |
|--------|-------|---------------|
| Top-level (em `build_page_stream`) | 2251–2255 | `let pdf_y = page_height - pos.y.val(); emit_text_pdf(...)` |
| Local (em `draw_item_local`) | 2649–2652 | `emit_text_pdf(ops, pos.x.0, pos.y.0, ...)` |

**Observação arquitectural crítica**: **ambos invocam o mesmo helper
`emit_text_pdf`** (linhas 2122–2179). A única diferença é o `base_y`:
top-level usa `page_height - pos.y.val()` (Y-inversion); local usa
`pos.y.0` directo (matriz `cm` do Group já inverteu Y).

### Resultado por scenario

| Aspecto | Top-level (Type1) | Local (Type1) | Paridade? |
|---|---|---|---|
| `BT ... ET` envelope | ✓ via helper | ✓ via helper | ✓ |
| `Tf` (font select + size) | ✓ via helper | ✓ via helper | ✓ |
| `Td` (positioning) | ✓ usando `pdf_y` | ✓ usando `pos.y.0` | ✓ (diferença legítima) |
| `Tj` (text operator) | ✓ via helper | ✓ via helper | ✓ |
| `escape_pdf_string` (Type1) | ✓ via helper | ✓ via helper | ✓ |
| **Faux-bold (`Tr 2` + `w`)** | ✓ helper l.2147–2153 | ✓ helper l.2147–2153 | **✓ paridade** (suspeita refutada) |
| **Tracking (`Tc`)** | ✓ helper l.2139–2146 | ✓ helper l.2139–2146 | **✓ paridade** (suspeita refutada) |
| **Style.bold/italic seleciona /F2/F3** (Type1) | ✓ helper l.2134–2138 | ✓ helper l.2134–2138 | **✓ paridade** (suspeita refutada) |

Para CIDFont (helper l.2159–2166):
- Hex string Identity-H via `text_to_hex_string`: ✓ paridade.
- `/F1` hardcoded: ✓ paridade.

Para Multifont (helper l.2167–2178):
- `style.font` lookup + `/F{fi+1}` dispatch: ✓ paridade.
- `per_font_char_to_gid[fi]` indexação: ✓ paridade.

**Síntese §A1.1**: **paridade total Text em todos os 3 scenarios**.
Mecanismo arquitectural: P281 centralizou emit num único helper —
divergência é estructuralmente impossível por construção.

---

## §A1.2 — Glyph emit: comparação top-level vs local

### Localização

| Camada | Linha | Implementação |
|--------|-------|---------------|
| Top-level | 2266–2270 | `let pdf_y = page_height - pos.y.val(); emit_glyph_pdf(...)` |
| Local | 2653–2656 | `emit_glyph_pdf(ops, pos.x.0, pos.y.0, ...)` |

**Idêntico padrão arquitectural**: ambos invocam `emit_glyph_pdf`
(linhas 2185–2210). Única diferença: `base_y`.

### Resultado por scenario

| Aspecto | Top-level | Local | Paridade? |
|---|---|---|---|
| **Type1**: silenciosamente ignored | ✓ helper l.2199–2201 | ✓ helper l.2199–2201 | ✓ (ambos sem-op) |
| **CIDFont**: `BT /F1 size Tf x y Td <gid> Tj ET` | ✓ helper l.2202–2207 | ✓ helper l.2202–2207 | ✓ |
| **Multifont**: `/F1` hardcoded (não selecciona por fonte) | ✓ helper l.2202–2207 | ✓ helper l.2202–2207 | **✓ paridade** (suspeita refutada) |

**Observação spec §A1.2**: a suspeita "Multifont Glyph seleciona /F{i+1}
baseado em algum critério" é **refutada** — código confirma `/F1`
hardcoded para Glyph em **ambos** scenarios CIDFont e Multifont
(decisão arquitectural pré-existente preservada por P281; glyphs são
delimitadores matemáticos que tipicamente usam a primeira fonte
disponível).

### Síntese §A1.2

**Paridade total Glyph em todos os 3 scenarios**. Mesmo mecanismo
arquitectural que Text — helper único partilhado.

---

## §A1.3 — Line emit: comparação top-level vs local

### Localização

| Camada | Linha | Implementação |
|--------|-------|---------------|
| Top-level | 2256–2265 | `q {t} w {x1} {y1} m {x2} {y2} l S Q\n` com Y-inversion explícita |
| Local | 2657–2664 | `q {t} w {x1} {y1} m {x2} {y2} l S Q\n` com `pos.y.0` directo |

### Análise literal

**Top-level**:
```rust
let x1 = start.x.val();
let y1 = page_height - start.y.val();
let x2 = end.x.val();
let y2 = page_height - end.y.val();
ops.push_str(&format!(
    "q {:.3} w {:.1} {:.1} m {:.1} {:.1} l S Q\n",
    thickness, x1, y1, x2, y2
));
```

**Local**:
```rust
ops.push_str(&format!(
    "q {:.3} w {:.1} {:.1} m {:.1} {:.1} l S Q\n",
    thickness, start.x.0, start.y.0, end.x.0, end.y.0
));
```

### Resultado

| Aspecto | Top-level | Local | Paridade? |
|---|---|---|---|
| `q` (push state) | ✓ | ✓ | ✓ |
| `w` (line width) | ✓ thickness | ✓ thickness | ✓ |
| `m` (moveTo) | ✓ x1, y1 (Y-inv) | ✓ start.x, start.y (pós-cm) | ✓ (diferença legítima) |
| `l` (lineTo) | ✓ x2, y2 (Y-inv) | ✓ end.x, end.y (pós-cm) | ✓ (diferença legítima) |
| `S` (stroke paint) | ✓ | ✓ | ✓ |
| `Q` (pop state) | ✓ | ✓ | ✓ |
| **`RG` (stroke colour)** | **✗ NÃO emitido** | **✗ NÃO emitido** | **✓ paridade simétrica** |

### Síntese §A1.3

**Paridade estructural ✓**. Spec hipotetizou que `RG` estaria em
top-level e em falta em local — refutado: **ambos** não emitem `RG`.
Mesma limitação simétrica pré-existente (Line usa graphics state
default — preto + `w` default 1.0 implícito quando não especificado).

**Esta NÃO é uma falha de paridade P281**; é uma limitação
arquitectural pré-P281 que permanece simetricamente em ambos os
caminhos. Resolução requer:
1. Adicionar campo `color: Option<Color>` a `FrameItem::Line` em
   L1 (mudança L1 estrutural).
2. Top-level + local emitirem `{r:.3} {g:.3} {b:.3} RG\n` quando
   color é Some.

Pendência **fora-do-escopo P282** (passo P283+ se prioritizado).

---

## §A1.4 — Síntese paridade emit

Tabela conclusão:

| Variante | Scenario | Paridade bit-exact? | Acção §C.1 |
|---|---|---|---|
| Text | Type1 | ✓ paridade total | nenhuma |
| Text | CIDFont | ✓ paridade total | nenhuma |
| Text | Multifont | ✓ paridade total | nenhuma |
| Glyph | Type1 | ✓ (ambos ignored) | nenhuma |
| Glyph | CIDFont | ✓ paridade total | nenhuma |
| Glyph | Multifont | ✓ paridade total (`/F1` hardcoded ambos) | nenhuma |
| Line | (scenario-independent) | ✓ paridade estructural | nenhuma (`RG` falta simetricamente) |

**6/6 suspeitas spec §A1 refutadas empíricamente**:

1. ~~Faux-bold local pode estar simplificado~~ → refutado: helper único.
2. ~~Tracking local pode estar em falta~~ → refutado: helper único.
3. ~~/F2/F3 hardcoded para /F1 em local~~ → refutado: helper único.
4. ~~Glyph multifont local pode divergir~~ → refutado: helper único (`/F1` hardcoded ambos).
5. ~~Line local pode faltar `RG` que top-level tem~~ → refutado: **ambos** sem `RG`.
6. ~~Multifont Glyph dispatch diferente~~ → refutado: comportamento pré-P281 preservado.

---

## §A1.5 — Mecanismo arquitectural P281 garante paridade

**Win arquitectural P281**: ao centralizar emit num único helper por
variante (`emit_text_pdf` / `emit_glyph_pdf`), a paridade é
**estructuralmente garantida** — qualquer mudança futura ao emit
afecta automaticamente top-level **e** local.

Esta é a **invariante de single source of truth** documentada em
L0 `infra/export.md` §"Pipeline unificado (P281)":

> "Single source of truth para emit PDF: modificações futuras a arms
> de FrameItem editam um sítio (helpers emit_text_pdf + emit_glyph_pdf
> + arms inline em draw_item_*)."

Tests funcionais P281 (9 verdes) já cobrem casos representativos.
Suite de regressão 2 615 bit-exact preserved valida indirectamente
que top-level continua correcto.

---

## §A1.6 — Conclusão Fase A1

**Paridade emit local vs top-level**: **total para Text/Glyph; estrutural
para Line** (com limitação simétrica `RG`).

**§C.1 NÃO dispara** — zero fixes pendentes neste passo.

**Pendência fora-do-escopo identificada** (não-bloqueante):
- `P-line-color-rg-emit` — Adicionar suporte a stroke colour em
  `FrameItem::Line` (requer mudança L1; magnitude S+M).

Fase A1 imutável a partir de 2026-05-18.
