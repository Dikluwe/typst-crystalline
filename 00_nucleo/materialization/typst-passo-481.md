---

# P481 — Sonda profunda Trilha 5: arquitectura de shaping rustybuzz

> **Passo:** 481
> **Data:** 2026-06-27
> **Foco:** Sonda arquitectural completa antes de qualquer código de shaping. Mapear `FrameItem::Text` → `FrameItem::TextShaped`, impacto em export.rs, e redigir ADR nova que autorize a mudança.
> **Trilha:** 5 — Shaping / rustybuzz.
> **Tipo:** Sonda arquitectural + ADR.
> **Tamanho:** S (sonda + ADR; ~25 min). Zero código de produção neste passo.
> **ADR-0039 EM VIGOR** — `TextStyle` como struct resolvido; `FrameItem::Text` usa `TextStyle`. Qualquer mudança a `FrameItem` requer ADR nova.

---

## Contexto

P480 encerrou o ciclo de paridade estrutural (73/73 matches). O único épico genuinamente pendente é Trilha 5: shaping com `rustybuzz`.

P476 confirmou que `rustybuzz` está em `03_infra/Cargo.toml` mas sem uso activo. O shaping actual é um stub sequencial: `FrameItem::Text { pos, text: EcoString, style: TextStyle }` posiciona strings planas sem kern, ligatures, ou advance per-glyph.

Antes de escrever código de shaping, é necessário:
1. Mapear todos os sites que lêem/escrevem `FrameItem::Text`.
2. Decidir a forma do novo `FrameItem::TextShaped`.
3. Redigir ADR nova que autorize a mudança e declare a estratégia de migração.
4. Identificar os bloqueadores reais e a magnitude corrigida.

Este passo é exclusivamente de sonda e ADR. Zero código de produção.

---

## Sondas obrigatórias (ADR-0108)

### Grupo 1 — `FrameItem::Text` (escrita)

| Pergunta | Verificar em | Status |
|----------|-------------|--------|
| Onde `FrameItem::Text { .. }` é construído? | `grep -rn "FrameItem::Text {"` em `01_core/` | 🟡 |
| `cursor.rs` usa `FrameItem::Text`? | `cursor.rs` | 🟡 |
| `math/layout.rs` usa `FrameItem::Text`? | `math/layout.rs` | 🟡 |
| `FrameItem::Text` em `03_infra/`? | `grep -rn "FrameItem::Text"` | 🟡 |

### Grupo 2 — `FrameItem::Text` (leitura / pattern match)

| Pergunta | Verificar em | Status |
|----------|-------------|--------|
| `export.rs` faz match de `FrameItem::Text`? | `03_infra/src/export.rs` | 🟡 |
| `parity/` faz match de `FrameItem::Text`? | `lab/parity/` | 🟡 |
| Testes fazem match de `FrameItem::Text`? | `grep -rn "FrameItem::Text"` em `tests/` | 🟡 |
| Total de sites de leitura? | contagem | 🟡 |

### Grupo 3 — rustybuzz integration points

| Pergunta | Verificar em | Status |
|----------|-------------|--------|
| `rustybuzz = { .. }` em `Cargo.toml` — versão? | `03_infra/Cargo.toml` | 🟡 |
| `rustybuzz::Face::from_slice` disponível? | doc rustybuzz crate | 🟡 |
| `FontBook` dá acesso a bytes da fonte? | `03_infra/src/fonts.rs` | 🟡 |
| `ttf-parser` versão compatível com rustybuzz? | workspace `Cargo.toml` | 🟡 |
| `rustybuzz::shape(face, features, buffer)` retorna `GlyphBuffer`? | doc rustybuzz | 🟡 |
| `GlyphInfo.glyph_id` e `GlyphPosition.x_advance` disponíveis? | doc rustybuzz | 🟡 |

### Grupo 4 — impacto `unicode-bidi` (RTL)

| Pergunta | Verificar em | Status |
|----------|-------------|--------|
| `unicode-bidi` crate no workspace? | `Cargo.toml` raiz | 🟡 |
| `Dir::RTL` referenciado em layout? | `layout/mod.rs` | 🟡 |
| Impacto de RTL em `cursor.rs`? | `cursor.rs` — advance direction | 🟡 |

**Todas as sondas com `grep`/`file:line` e resultado explícito.**

---

## Análise arquitectural esperada

### Forma actual de `FrameItem::Text`

```rust
// ADR-0039 — TextStyle como struct resolvido
FrameItem::Text {
    pos:   Point,
    text:  EcoString,     // string plana — sem per-glyph advances
    style: TextStyle,     // bold, italic, size, fill, heading_level, ...
}
```

### Forma proposta de `FrameItem::TextShaped`

```rust
/// P481 — Proposta (a confirmar via sonda)
#[derive(Debug, Clone)]
pub struct ShapedGlyph {
    pub glyph_id:  u16,       // ID do glifo na fonte
    pub x_advance: f32,       // advance horizontal em unidades de fonte
    pub x_offset:  f32,       // offset horizontal (kern, marks)
    pub y_offset:  f32,       // offset vertical (diacríticos, etc.)
    pub cluster:   u32,       // índice byte no string original (bidi)
    pub char_code: char,      // codepoint Unicode para ToUnicode CMap
}

pub enum FrameItem {
    // ... variantes existentes ...
    Text {
        pos:   Point,
        text:  EcoString,      // preservado para paridade/fallback
        style: TextStyle,
    },
    TextShaped {               // P481 PROPOSTA — shaping real
        pos:    Point,
        glyphs: Vec<ShapedGlyph>,
        style:  TextStyle,
    },
}
```

**Questão central:** manter `Text` e adicionar `TextShaped` (migração gradual, mais segura) vs substituir `Text` por `TextShaped` (limpeza, mas ripple maior). A sonda de sites de leitura determina qual é viável.

### Pipeline de shaping (proposta)

```
layout/cursor.rs — emit de palavra:
  1. Obter bytes da fonte actual via FontBook.
  2. Construir rustybuzz::Face from bytes.
  3. Construir UnicodeBuffer com string + language + direction.
  4. shape(face, features=[], buffer) → GlyphBuffer.
  5. Iterar GlyphBuffer.glyph_infos() + GlyphBuffer.glyph_positions().
  6. Emitir FrameItem::TextShaped { pos, glyphs, style }.

export.rs — render de TextShaped:
  1. Para cada glyph: emit PDF /F{i} glyph_id como hex string.
  2. Usar x_advance (units_per_em → pt) para posicionar.
  3. Emitir mapa ToUnicode via char_code (preservar acessibilidade).
```

---

## ADR nova — `ADR-0120: FrameItem::TextShaped e pipeline rustybuzz`

A ADR deve cobrir:

### Secção 1 — Contexto

- `FrameItem::Text` (ADR-0039) usa string plana sem shaping.
- rustybuzz já está em `Cargo.toml` (ADR-0019) mas inactivo.
- Shaping real é necessário para: kern pairs, ligatures, scripts não-latinos, RTL.

### Secção 2 — Decisão

Uma de três opções:

| Opção | Descrição | Prós | Contras |
|-------|-----------|------|---------|
| **A** | `FrameItem::TextShaped` novo variant; `Text` preservado como fallback | Migração gradual; zero ripple imediato | 2 paths em export; complexidade permanente |
| **B** | Substituição completa `Text → TextShaped`; `text: EcoString` preservado em `TextShaped` como source | Limpeza; único path | Ripple em todos os sites de leitura |
| **C** | Shaping em `cursor.rs` inline sem novo FrameItem; calcular x_advance mas emitir `Text` com glyph_ids encoded | Menor ripple | Perde informação estrutural; hack |

**Recomendação:** Opção A para a primeira fase; migração para B numa segunda fase após estabilização.

### Secção 3 — Estratégia de migração

1. Fase 1 (P482): `ShapedGlyph` + `FrameItem::TextShaped` + pipeline shaping em `cursor.rs`. Export usa `TextShaped` quando disponível; `Text` como fallback.
2. Fase 2 (P483+): `export.rs` migrado completamente para `TextShaped`. `Text` deprecated.
3. Fase 3 (futura): RTL via `unicode-bidi`; eliminar `Text`.

### Secção 4 — Scope-out desta ADR

- RTL/bidi — Fase 3.
- OpenType features (kern tables, liga) — `rustybuzz::Feature` na Fase 1 mas sem activação explícita; kern implícito via GPOS.
- Fallback de fonte (`FontList` com múltiplas fontes) — shaping da Fase 1 usa apenas a fonte primária.
- Shaping em `math/layout.rs` — math já usa `FrameItem::Glyph`; não afectado.

---

## Output deste passo

1. **Relatório de sonda** — tabela com todos os `file:line` para os grupos 1–4.
2. **ADR-0120** redigida e em estado PROPOSTO (aguarda materialização em P482).
3. **Magnitude corrigida** — com base na contagem real de sites, corrigir estimativa XL (8–12h) ou confirmar.
4. **Plano de passos** — decomposição do épico em P482–P484 (ou equivalente).

---

## Critério de fecho

- [ ] Grupo 1: todos os sites de escrita de `FrameItem::Text` localizados com `file:line`.
- [ ] Grupo 2: todos os sites de leitura localizados com `file:line`; total contado.
- [ ] Grupo 3: versão rustybuzz confirmada; `FontBook` dá acesso a bytes de fonte; API `shape()` confirmada.
- [ ] Grupo 4: `unicode-bidi` ausente/presente determinado; impacto RTL em cursor avaliado.
- [ ] ADR-0120 redigida (PROPOSTO): forma `ShapedGlyph`, `FrameItem::TextShaped`, pipeline, estratégia de migração, scope-out.
- [ ] Magnitude corrigida do épico documentada.
- [ ] Plano P482–P484 (ou equivalente) esboçado.
- [ ] Zero código de produção produzido.
- [ ] `crystalline-lint` não precisa de correr (sem modificações de código).

---

## Próximo passo (P482)

Com ADR-0120 PROPOSTO e sonda completa:

- **P482 — Fase 1 shaping:** `ShapedGlyph` + `FrameItem::TextShaped` + pipeline rustybuzz em `cursor.rs` para texto latino LTR. Export lê `TextShaped` com fallback para `Text`. Magnitude estimada: L.
- **P483 — Fase 2 shaping:** migração completa de `export.rs` para `TextShaped`; `Text` deprecated. Magnitude estimada: M.
- **P484 — RTL básico:** `unicode-bidi` + re-order de runs. Magnitude estimada: L.

---

## Estado pós-P480 (para referência)

| Indicador | Estado |
|-----------|--------|
| Paridade | 73/73 matches; 0 diffs; 0 errors |
| DEBTs activos | 0 |
| Trilhas 1–4, 7, 8 | COMPLETAS |
| Trilha 5 | Épico XL — P481 é a sonda de arranque |
| Trilha 6 | 4/5 |
| **P481** | Sonda arquitectural Trilha 5 | 🔄 EM PREPARAÇÃO |
