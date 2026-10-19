# P449 — Highlight

> **Passo:** 449  
> **Data:** 2026-06-24  
> **Foco:** Materializar a função nativa `highlight` para texto com fundo colorido.

---

## Contexto

O Typst vanilla tem `#highlight[text]` (fundo amarelo por defeito) e `#highlight(fill: color)[text]`. O cristalino não tem. Último quick win tipográfico XS da sequência P444–P448.

---

## ADR-0108 — Medir antes de decidir

| Pergunta | Resultado | Status |
|----------|-----------|--------|
| `highlight` existe em stdlib? | Não | ❌ |
| Infra de retângulo colorido existe? | Sim — `FrameItem::Rect` (usado em `block.rs`, `table.rs`) | ✅ |
| Export PDF de filled rect existe? | Sim — `re f` | ✅ |
| Bloqueadores? | Nenhum técnico | ✅ |

**Reclassificação:** XS (~15 min; 1 função + layout + export + tests).

---

## Toques pontuais

1. **`entities/style.rs`** — `Style::Highlight(Option<Color>)` (None = sem highlight).
2. **`entities/style_chain.rs`** — `StyleDelta` propaga `highlight`; `TextStyle` ganha `highlight: Option<Color>`.
3. **`entities/content.rs`** — `highlight(body, fill: Option<Color>)` emite `Content::Styled`.
4. **`rules/stdlib/text.rs`** — `native_highlight` com parâmetro `fill: Option<Color>` (default `Some(Color::from_u8(0xFF, 0xF2, 0x36, 0xFF))` — amarelo Typst).
5. **`rules/eval/rules.rs`** — Selector `NodeKind::Highlight`.
6. **`engine/layout/text.rs`** — Ao renderizar `FrameItem::Text`, se `TextStyle.highlight` é `Some(color)`, emitir `FrameItem::Rect` (background) + `FrameItem::Text` (foreground) no mesmo posicionamento.
7. **`engine/layout/cursor.rs`** — O rect de fundo usa `cursor.y - descent` a `cursor.y + ascent`, `width = text_width`.
8. **`03_infra/src/export.rs`** — Reuso de `FrameItem::Rect` (zero alteração no export).
9. **Tests** — 2 L1 (eval: default amarelo, custom color) + 2 L3 (layout: rect emitido, cor correcta).
10. **Spec L0** — `text.md` secção `highlight`.

---

## Scope-out explícito

- `extent`, `top-edge`, `bottom-edge` — scope-out.
- `highlight` em math mode — scope-out.
- Gradient fill — scope-out; apenas `Color` sólida.

---

## Critério de fecho

- [ ] `Style::Highlight` adicionado.
- [ ] `StyleDelta` propaga `highlight`.
- [ ] `native_highlight` em stdlib com default amarelo.
- [ ] Selector `NodeKind::Highlight` em eval.
- [ ] Layout emite `FrameItem::Rect` + `FrameItem::Text` para texto highlighted.
- [ ] 4 tests verdes (2 L1 + 2 L3).
- [ ] Spec L0 atualizado.
- [ ] `cargo test --workspace` verde; `crystalline-lint` zero violations.

---

## Próximo passo

Com P449 fechado, a frente tipográfica XS está completa. O próximo passo natural é:
- **P450** — Bibliography #5: carregamento `.bib` de disco (frente estrutural maior)
- **P450** — Heading numbering
- **P450** — Links / hyperlinks

Aguardando indicação.
