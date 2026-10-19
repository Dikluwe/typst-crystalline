---

# P483 — Trilha 5 Fase 2: migração completa `Text → TextShaped` + `FrameItem::Text` deprecated

> **Passo:** 483
> **Data:** 2026-06-27
> **Foco:** Fechar o dual-path de P482: todos os emit sites de `FrameItem::Text` passam a preencher a fonte activa; `export/stream.rs` usa `TextShaped` como path principal; `FrameItem::Text` marcado `#[deprecated]`.
> **Trilha:** 5 — Shaping / rustybuzz — Fase 2.
> **Tipo:** Materialização M.
> **Tamanho:** M (~1.5–2h).
> **ADR-0120 ACEITE** (P482). **ADR-0039 EM VIGOR** — `TextStyle` como struct resolvido.

---

## Contexto

P482 criou o dual-path:
- `cursor.rs` (e outros write sites) emitem `FrameItem::Text` como antes.
- `shaper.rs` converte para `FrameItem::TextShaped` quando encontra `style.font` preenchido.
- `export/stream.rs` tem arm `TextShaped` funcional, mas o path principal continua a ser `Text` (fallback string plana).

O problema de P482 é que o shaper resolve a fonte via `world.book().select_pattern()` — mas se `style.font = None` (fonte padrão, Helvetica), o shaper não consegue shape e preserva `Text`. O resultado é que **texto com fonte padrão (a maioria) permanece não-shaped**.

P483 resolve este problema em dois sub-itens:

**Sub-item A:** Garantir que todos os `FrameItem::Text` emitidos pelo Layouter têm `style.font` preenchido com pelo menos a família padrão — preenchendo o gap que impede o shaper de actuar.

**Sub-item B:** Após o shaper actuar na totalidade dos items, `FrameItem::Text` sem conversão representa genuinamente um fallback (fonte não carregada, erro de shaping) — marcá-lo `#[deprecated]` e garantir que export/stream usa `TextShaped` como path primário.

---

## ADR-0108 — Medir antes de decidir

| Pergunta | Verificar em | Status |
|----------|-------------|--------|
| `TextStyle.font` é `None` para texto com fonte padrão? | `01_core/src/entities/layout_types.rs` — `TextStyle::default()` | 🟡 |
| Qual é a fonte padrão quando `font = None`? | `03_infra/src/pipeline.rs` `default_text_style` ou similar | 🟡 |
| `StyleChain::font()` retorna `None` ou `Some(FontList::default())`? | `entities/style_chain.rs` | 🟡 |
| `From<&StyleChain> for TextStyle` preenche `font`? | `entities/layout_types.rs` | 🟡 |
| Em P482, qual % dos items é `TextShaped` vs `Text` pós-shaper? | teste com documento simples | 🟡 |
| `select_pattern` em shaper tem que corresponder à fonte padrão? | `03_infra/src/shaper.rs::resolve_slot` | 🟡 |

**Todas as sondas com `grep`/`file:line` antes de escrever código.**

---

## Sub-item A — Garantir `style.font` preenchido para texto com fonte padrão

### A.1 — Diagnóstico esperado

Se `StyleChain::font()` retorna `None` quando nenhum `#set text(font: ...)` foi aplicado, então `TextStyle::from(&chain).font = None`, e o shaper não consegue resolver a fonte.

A solução depende do resultado da sonda:

**Caso 1 — `StyleChain::font()` retorna `None` para fonte padrão:**
Adicionar fonte padrão explícita como fallback em `From<&StyleChain> for TextStyle`:

```rust
// Em From<&StyleChain> for TextStyle:
font: Some(chain.font().unwrap_or_else(|| FontList::single("Helvetica"))),
```

**Caso 2 — `StyleChain::font()` já retorna `Some(FontList)` mas shaper falha:**
O problema está em `resolve_slot` — `select_pattern` não encontra match. Verificar se `FontBook` tem a fonte registada e o nome corresponde.

**Caso 3 — `shaper.rs::resolve_slot` retorna `None` para fonte válida:**
Bug em `resolve_slot` — corrigir o match de família vs nome de slot.

A sonda determina qual caso se aplica.

### A.2 — Implementação (Caso 1 mais provável)

Se `TextStyle::default().font = None` é a causa:

**Ficheiro:** `01_core/src/entities/layout_types.rs` — `From<&StyleChain> for TextStyle`

```rust
// P483 — garantir que font nunca é None em TextStyle produzido de StyleChain
font: Some(
    chain.font().cloned().unwrap_or_else(|| FontList::single("Helvetica"))
),
```

**Impacto:** `shaper.rs::resolve_slot` passa a encontrar sempre uma família para tentar shape. Se a fonte não estiver carregada na `FontBook`, `resolve_slot` retorna `None` → item preservado como `Text` (fallback defensivo mantido).

### A.3 — Verificação da cobertura pós-fix

Após o fix, instrumentar `shaper.rs` para contar `Text` preservados vs `TextShaped` convertidos num documento de teste simples. Critério: ≥ 95% dos items de texto do corpo (excluindo math e casos especiais) devem ser `TextShaped`.

---

## Sub-item B — `FrameItem::Text` deprecated + export path primário

### B.1 — Deprecar `FrameItem::Text`

**Ficheiro:** `01_core/src/entities/layout_types.rs`

```rust
pub enum FrameItem {
    #[deprecated(since = "P483", note = "Use FrameItem::TextShaped. \
        FrameItem::Text é preservado como fallback para fontes não carregadas.")]
    Text {
        pos:   Point,
        text:  EcoString,
        style: TextStyle,
    },
    TextShaped { ... },
    // ...
}
```

**Nota:** a deprecação `#[deprecated]` em variants de enum requer `#![allow(deprecated)]` nos sites de match legítimos. Verificar se Rust estável suporta `#[deprecated]` em enum variants (suportado desde Rust 1.81). Caso não suportado: usar doc comment `// DEPRECATED P483` + lint manual.

### B.2 — `export/stream.rs` — path primário `TextShaped`

**Ficheiro:** `03_infra/src/export/stream.rs`

Nos dois sites de match (`264` e `675`), reordenar os arms para que `TextShaped` apareça primeiro e `Text` seja o arm de fallback com aviso:

```rust
FrameItem::TextShaped { pos, glyphs, style, text } => {
    emit_shaped_pdf(writer, pos, glyphs, style, units_per_em_map, font_selector)?;
}
#[allow(deprecated)]
FrameItem::Text { pos, text, style } => {
    // Fallback: shaping não disponível (fonte não carregada, Type1, etc.)
    // Emite string plana — comportamento de P280.
    emit_text_pdf(writer, pos, text, style)?;
}
```

### B.3 — Suprimir warnings `deprecated` nos ~21 sites de match existentes

Nos sites de passthrough (cursor.rs, slicing.rs, etc.), adicionar `#[allow(deprecated)]` ao bloco de match ou ao arm específico:

```rust
#[allow(deprecated)]
match item {
    FrameItem::Text { pos, .. } => { /* passthrough */ }
    FrameItem::TextShaped { pos, .. } => { /* idem */ }
    // ...
}
```

**Alternativa:** se `#[deprecated]` em enum variants não suprimir warnings limpos, usar uma constante de lint local no módulo: `#![allow(deprecated)]` em cada ficheiro afectado. A sonda determina qual é necessário.

---

## Tests

### Sub-item A

- **L3 (`shaper::tests`):**
  - `p483_texto_com_font_none_resulta_em_textshaped` — documento simples com `style.font = None` no `FrameItem::Text` de entrada; após P483, `resolve_slot` encontra Helvetica e produz `TextShaped`.
  - `p483_cobertura_shaping_95_pct` — documento de 100 palavras; ≥95 items convertidos para `TextShaped`.

- **L1 (`entities::tests`):**
  - `p483_textstyle_from_chain_font_nunca_none` — `TextStyle::from(&StyleChain::default_chain()).font == Some(FontList::single("Helvetica"))`.

### Sub-item B

- **L3 (`export::tests`):**
  - `p483_export_usa_textshaped_como_primario` — documento com `TextShaped` items → export emite CID hex strings (não strings planas).
  - `p483_export_fallback_text_preservado` — documento com `Text` items (sem shape) → export emite string plana sem panic.

- **Regressão:**
  - `p483_parity_73_73_mantido` — 73/73 matches preservados (export com dual-path não deve introduzir diffs).

---

## Spec L0

### Actualizados

- `entities/layout_types.md` — `FrameItem::Text` marcado deprecated P483; nota sobre `style.font` preenchido.
- `infra/shaper.md` — secção P483: `resolve_slot` cobre fonte padrão pós-fix.
- `infra/export/stream.md` — path primário `TextShaped`; fallback `Text` documentado.
- ADR-0120 — anotação P483: Fase 2 executada; cobertura de shaping ≥95%.

---

## Scope-out explícito

- **Remoção de `FrameItem::Text`** — apenas deprecated, não removido. Remoção é Fase 3 (P484) após RTL implementado.
- **`#[deprecated]` em todos os construtores de `FrameItem::Text`** — se Rust não suportar em enum variants, usar `// DEPRECATED` doc comment.
- **Type1 / CFF shaping** — `FrameItem::Text` permanece o path para fontes que rustybuzz não suporta.
- **Avanço horizontal em pt no emit** — `emit_shaped_pdf` usa glyph IDs hex; cálculo de advance em pt para posicionamento preciso é P485+ (OpenType metrics).
- **Font substitution** — se a fonte pedida não está na FontBook, fallback para Helvetica. Sem font-matching avançado.

---

## Critério de fecho

- [ ] Sondas: `TextStyle::default().font`, `StyleChain::font()` default, `resolve_slot` trace num documento simples — todos com `file:line` e conclusão.
- [ ] Causa raiz de `style.font = None` identificada e corrigida (Caso 1, 2, ou 3).
- [ ] `TextStyle::from(&StyleChain::default_chain()).font` nunca é `None`.
- [ ] `shaper.rs` cobre ≥95% dos items de texto de um documento padrão.
- [ ] `FrameItem::Text` marcado `#[deprecated(since = "P483")]` (ou doc comment equivalente).
- [ ] `#[allow(deprecated)]` adicionado nos ~21 sites de match legítimos.
- [ ] `export/stream.rs` tem `TextShaped` como arm primário; `Text` como fallback.
- [ ] 5+ testes verdes (1 L1 + 4 L3).
- [ ] `p483_parity_73_73_mantido` verde.
- [ ] Spec L0 actualizada (4 ficheiros).
- [ ] `cargo test --workspace` verde; `crystalline-lint` zero violations.
- [ ] **Trilha 5: Fase 2 FECHADA.** Shaping cobre ≥95% do texto normal.

---

## Estado pós-P482 (para referência)

| Indicador | Estado |
|-----------|--------|
| Paridade | 73/73 matches |
| DEBTs activos | 0 |
| ADR-0120 | ACEITE (P482) |
| Trilha 5 | Fase 1 ✅; **Fase 2 em preparação** |
| **P483** | Fase 2 shaping — migração completa | 🔄 EM PREPARAÇÃO |
