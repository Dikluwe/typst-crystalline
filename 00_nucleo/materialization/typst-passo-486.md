---

# P486 — OpenType features `liga`/`kern`/`calt` + `x_offset`/`y_offset` em emit

> **Passo:** 486
> **Data:** 2026-06-28
> **Foco:** (A) Activar features OpenType `liga`, `kern`, `calt` explicitamente em `shaper.rs`; (B) usar `x_offset` e `y_offset` de cada `ShapedGlyph` no emit PDF para posicionamento correcto de diacríticos e kern marks.
> **Trilha:** 5 — Shaping / rustybuzz — extensão final.
> **Tipo:** Materialização S + S.
> **Tamanho:** S (~45 min total; dois sub-itens pequenos e independentes).
> **ADR-0120 ACEITE**. **ADR-0019 EM VIGOR** — rustybuzz em L3.

---

## Contexto

P485 fechou o `TJ` operator com `x_advance` por glifo mas deixou dois items em scope-out:

1. **Features OpenType** — `features = &[]` passado ao `rustybuzz::shape()`. O relatório P485 diz explicitamente que `liga`, `kern`, `calt` ficaram como scope-out. Se estão activos por defeito em rustybuzz 0.20 com `features = &[]`, sub-item A é apenas confirmação e documentação. Se não, é necessário activá-los.

2. **`x_offset`/`y_offset`** — P485 usa apenas `x_advance` no número TJ; `ShapedGlyph.x_offset` e `y_offset` existem mas não são usados. `x_offset` afecta kern marks e posicionamento horizontal de diacríticos; `y_offset` afecta posicionamento vertical (ex: combinando diacríticos, superscript/subscript shaped).

---

## ADR-0108 — Medir antes de decidir

| Pergunta | Verificar em | Status |
|----------|-------------|--------|
| `features = &[]` em rustybuzz 0.20 activa `liga` por defeito? | doc rustybuzz + lab/krilla-reference | 🟡 |
| `features = &[]` activa `kern` por defeito? | idem | 🟡 |
| `features = &[]` activa `calt` por defeito? | idem | 🟡 |
| `rustybuzz::Feature::new` vs `rustybuzz::feature!` macro | rustybuzz 0.20 API | 🟡 |
| `ttf_parser::Tag::from_bytes(b"liga")` disponível? | `ttf-parser 0.25` | 🟡 |
| `ShapedGlyph.x_offset` usado no emit pós-P485? | `export/stream.rs` | ✅ não usado (P485 scope-out) |
| `ShapedGlyph.y_offset` usado no emit pós-P485? | idem | ✅ não usado (P485 scope-out) |
| Formato TJ com offset horizontal: `[(hex) num (hex)] TJ`? | PDF spec §9.4.3 | ✅ |
| Offset vertical em PDF: `x y Td` ou `Tm`? | PDF spec §9.4.2 | 🟡 |

**Sondas 🟡 com `grep`/`file:line` antes de qualquer código.**

---

## Sub-item A — Features OpenType `liga`/`kern`/`calt`

### A.1 — Verificar defaults de rustybuzz

A sonda deve confirmar qual é o comportamento com `features = &[]`:

- **Se rustybuzz activa `liga`, `kern`, `calt` por defeito:** sub-item A é apenas documentação. Adicionar comentário em `shaper.rs` e actualizar L0.
- **Se não activa por defeito:** implementar features explícitas.

```rust
// Para verificar defaults: consultar rustybuzz source
// lab/krilla-reference/crates/krilla/src/text/shape.rs
// krilla passa features explícitas? Se sim, rustybuzz não activa por defeito.
```

### A.2 — Features explícitas (se necessário)

**Ficheiro:** `03_infra/src/shaper.rs`

```rust
// P486 — features OpenType activadas explicitamente
// liga: standard ligatures (fi, fl, ff, ffi, ffl)
// kern: kerning via GPOS table
// calt: contextual alternates
use ttf_parser::Tag;
use rustybuzz::Feature;

const FEATURES: &[Feature] = &[
    Feature { tag: Tag::from_bytes(b"liga"), value: 1, start: 0, end: u32::MAX },
    Feature { tag: Tag::from_bytes(b"kern"), value: 1, start: 0, end: u32::MAX },
    Feature { tag: Tag::from_bytes(b"calt"), value: 1, start: 0, end: u32::MAX },
];

// Em try_shape:
let output = rustybuzz::shape(&rb_face, FEATURES, buffer);
```

**Nota:** se `Feature` struct não é construível com esta sintaxe em rustybuzz 0.20, usar a API disponível. A sonda resolve.

### A.3 — Impacto observável

Com `liga` activa:
- "fi" em fontes com ligature → 1 glifo em vez de 2.
- `glyphs.len()` para "fi" decresce de 2 para 1.
- O `cluster` do glifo de ligature aponta para o 'f' (byte 0).

Com `kern` activa:
- "AV" → o 'V' tem `x_offset` negativo (ou o 'A' tem `x_advance` reduzido).
- Visível como par de glifos mais próximos no PDF.

---

## Sub-item B — `x_offset`/`y_offset` em `emit_shaped_pdf`

### B.1 — Estado actual do TJ emit (pós-P485)

```
[ <GID0> -600 <GID1> -580 ] TJ
```

O número entre glifos é o `x_advance` convertido. O `x_offset` (deslocamento horizontal relativo do glifo dentro do seu avanço) não é aplicado.

### B.2 — `x_offset` horizontal

No operador PDF `TJ`, o número ajusta o posicionamento horizontal entre glifos. Para aplicar o `x_offset` do glifo actual, a abordagem é:

```
[ offset_adjustment <GID> advance_adjustment ... ] TJ
```

Onde:
- `offset_adjustment` (antes do gliph ID) = `-(x_offset / upm * 1000)` — move o glifo para a posição correcta.
- `advance_adjustment` (depois do glyph ID) = ajuste para o próximo — necessário para cancelar o offset antes de avançar.

A lógica completa por glifo:

```rust
// Para cada ShapedGlyph:
if glyph.x_offset != 0 {
    let offset_adj = -(glyph.x_offset as f64 / upm * 1000.0);
    tj.push_str(&format!("{:.0} ", offset_adj));
}
tj.push_str(&format!("<{:04X}>", glyph.glyph_id));
// advance: negativo de x_advance (já feito em P485)
let advance = -(glyph.x_advance as f64 / upm * 1000.0);
tj.push_str(&format!(" {:.0} ", advance));
```

**Simplificação:** se `x_offset = 0` para a maioria dos glifos (texto LTR sem kern marks), o output é equivalente ao P485 — zero regressão.

### B.3 — `y_offset` vertical

O operador `TJ` é puramente horizontal. Para aplicar `y_offset`, PDF requer uma mudança de baseline via `Td` ou `Tm` antes do glifo e restauração depois. Isso torna o emit mais complexo: cada glifo com `y_offset != 0` precisaria de uma sequência:

```
0 y_pt Td <GID> Tj 0 -y_pt Td
```

Onde `y_pt = y_offset / upm * font_size_pt`.

**Decisão de sub-item B:** implementar `x_offset` no TJ array (simples, sem mudança de operador). `y_offset` requer saída do `TJ` e sequências `Td` — mais complexo; avaliar via sonda se `y_offset != 0` é comum no corpus antes de implementar.

**Se sonda mostrar `y_offset = 0` para todo o corpus LTR:** scope-out declarado; apenas `x_offset` implementado em P486.

---

## Tests

### Sub-item A

- **L3:** `p486_features_array_tem_liga_kern_calt` — `FEATURES` tem 3 entradas; tags corretas.
- **L3:** `p486_shape_com_features_nao_panic` — `try_shape("hello", rb_face, FEATURES)` sem panic (MockWorld sem fonte → None; com fonte real → ShapedGlyph).
- **Documental:** se features são defaults, `p486_features_default_confirmado` — comentário in-code + L0.

### Sub-item B

- **L3:** `p486_emit_x_offset_zero_equivale_p485` — glifo com `x_offset=0` → TJ output igual ao P485.
- **L3:** `p486_emit_x_offset_nonzero_aplica_ajuste` — glifo com `x_offset=-50, upm=1000` → TJ inclui `"50 "` antes do glyph ID.
- **L3:** `p486_emit_x_offset_positivo` — glifo com `x_offset=30, upm=1000` → TJ inclui `"-30 "` antes do glyph ID.
- **Parity:** `p486_parity_73_73_mantido` — 73/73 matches preservados.

---

## Spec L0

### Actualizados

- `infra/shaper.md` — §P486: features `liga`/`kern`/`calt`; confirmação ou activação explícita.
- `infra/export/stream.md` — §P486: `x_offset` no TJ array; `y_offset` scope-out.
- ADR-0120 — anotação P486: extensão final shaping; features + kern marks.

---

## Scope-out explícito

- **`y_offset` em emit** — requer saída do TJ e sequência `Td`; scope-out a menos que sonda mostre `y_offset != 0` comum.
- **`dlig`/`hlig`/`clig`** — discretionary/historical/contextual ligatures; apenas `liga` standard neste passo.
- **`smcp` feature** — smallcaps via OpenType; P447+ confirmado como scope-out de Trilha 5.
- **`tnum`/`onum`** — tabular/old-style numerals; scope-out.
- **Ligatures em corpus de paridade** — sem corpus específico com "fi"/"fl"; verificação via testes unitários.
- **`x_advance` exacto vs `hmtx`** — P485 scope-out preservado; `x_advance` usado no número TJ mas sem correcção de `hmtx` base.

---

## Critério de fecho

- [ ] Sondas: defaults de features em rustybuzz 0.20; `Feature` API; `y_offset` no corpus — todos com `file:line` e conclusão.
- [ ] Sub-item A: features `liga`/`kern`/`calt` confirmadas activas (por defeito ou explícitas).
- [ ] Sub-item B: `x_offset` aplicado no TJ array.
- [ ] `y_offset`: decision documentada (implementar ou scope-out).
- [ ] 4+ testes verdes sub-item A + 3+ testes verdes sub-item B.
- [ ] `p486_parity_73_73_mantido` verde.
- [ ] Spec L0 actualizada (3 ficheiros + ADR-0120).
- [ ] `cargo test --workspace` verde; `crystalline-lint` zero violations.
- [ ] **Trilha 5: extensão final FECHADA** — features activas + posicionamento kern marks.

---

## Próximo passo (P487)

Com P486 fechado, Trilha 5 atinge o estado de paridade qualitativa máxima sem RTL avançado:

| Indicador Trilha 5 | Estado pós-P486 |
|--------------------|-----------------|
| `FrameItem::TextShaped` | ✅ P482 |
| shaper.rs pipeline | ✅ P482 |
| RTL básico | ✅ P484 |
| `x_advance` TJ | ✅ P485 |
| `liga`/`kern`/`calt` | ✅ P486 |
| `x_offset` kern marks | ✅ P486 |
| `y_offset` diacríticos | scope-out ou P487 |
| `smcp` OpenType | scope-out |
| `x_advance` exacto vs hmtx | scope-out |

Opções para P487:

| Opção | Descrição | Magnitude |
|-------|-----------|-----------|
| **P487-A** | `y_offset` em emit se sonda P486 mostrar necessidade | S |
| **P487-B** | Expansão corpus lab/parity com ficheiros RTL | S |
| **P487-C** | Fechar Trilha 6 (5ª funcionalidade: LoF/LoT page numbers) | M–L |
| **P487-D** | Audit final do projecto — estado consolidado | XS |

---

## Estado pós-P485 (para referência)

| Indicador | Estado |
|-----------|--------|
| Paridade | 73/73 matches |
| DEBTs activos | 0 |
| Trilha 5 | Fases 1–3 + P485 + **P486 em preparação** |
| ADR-0120 | ACEITE |
| **P486** | Extensão shaping final | 🔄 EM PREPARAÇÃO |
