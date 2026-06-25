# P448 — Subscript e Superscript

> **Passo:** 448  
> **Data:** 2026-06-24  
> **Foco:** Materializar as funções nativas `sub` e `super` para texto subscrito e sobrescrito.  
> **Pré-requisitos:** P447 (cobertura + DSM audit fechado).  

---

## Contexto

O Typst vanilla tem `#sub[text]` e `#super[text]` como elementos nativos de texto que deslocam o baseline para baixo (subscrito) ou para cima (sobrescrito) e reduzem o tamanho da fonte (tipicamente 0.6×). O cristalino não tem estas funcionalidades. Quick win tipográfico XS, análogo a `smallcaps` (P446) e `smartquote` (P445).

---

## ADR-0108 — Medir antes de decidir

**FASE A.0 — Sonda:**

| Pergunta | Resultado | Status |
|----------|-----------|--------|
| `sub`/`super` existem no cristalino? | Não | ❌ |
| Vanilla implementa como `Style` ou `Content`? | `Content` — `SubElem`/`SuperElem` com `body` | ✅ |
| `FrameItem::Text` suporta offset vertical? | Sim — `TextStyle` tem baseline shift implícito via `y` offset em layout | ✅ |
| `Style` enum extensível? | Sim — ADR-0038 | ✅ |
| Bloqueadores? | Nenhum técnico | ✅ |

**Reclassificação:** XS (~15 min; 2 funções nativas + 2 styles + 2 layout arms + tests).

---

## ADR-0107 — Paridade linguagem

Contrato comportamental: `#sub[text]` desloca o baseline para baixo (`-0.2 * size`) e reduz o tamanho para `0.6 * size`; `#super[text]` desloca para cima (`+0.3 * size`) e reduz para `0.6 * size`. Paridade vanilla para o subset minimal (sem `offset`/`size` configuráveis).

---

## ADR-0109 — Atomização forma B

**Toques pontuais:**

1. **`entities/style.rs`** — Adicionar `Style::Subscript(bool)` e `Style::Superscript(bool)` ao enum `Style` (análogo a `Bold`/`Italic`/`Smallcaps`).
2. **`entities/style_chain.rs`** — `StyleDelta` ganha `subscript: bool` e `superscript: bool`; `fold_into` propaga; `TextStyle` ganha `subscript: bool` e `superscript: bool`.
3. **`entities/content.rs`** — Construtores `sub(body)` e `super(body)` emitem `Content::Styled(body, Styles::from_iter([Style::Subscript(true)]))`.
4. **`rules/stdlib/structural.rs`** — `native_sub` e `native_super` (análogo a `native_strong`/`native_emph`/`native_smallcaps`).
5. **`rules/eval/rules.rs`** — Selectors `NodeKind::Subscript` e `NodeKind::Superscript` casam `Style::Subscript`/`Superscript` no `Content::Styled`.
6. **`rules/layout/mod.rs`** — Arms de `Content::Styled` com `Style::Subscript`/`Superscript`:
   - Ajusta `TextStyle.size *= 0.6` e `TextStyle.baseline_offset = -0.2 * size` (sub) ou `+0.3 * size` (super).
   - O layout de texto existente em `layout/text.rs` aplica o offset vertical ao posicionamento do glyph.
7. **`rules/layout/tests.rs`** — 2 testes de integração: `subscript_desloca_baseline` e `superscript_desloca_baseline`.
8. **`rules/eval/tests.rs`** — 2 testes unitários: `eval_sub_emite_styled` e `eval_super_emite_styled`.
9. **Spec L0** — Adicionar secções em `00_nucleo/prompts/rules/stdlib/structural.md`.

---

## Decisões arquiteturais

| Decisão | Opção escolhida | Justificativa |
|---------|----------------|---------------|
| Modelo | `Content::Styled` + `Style` enum | Paridade com P444/P446; reusa infraestrutura existente |
| Render | Ajuste de `TextStyle.size` + `baseline_offset` | O layout de texto já suporta offset vertical; nenhum código novo de PDF |
| Size padrão | `0.6×` do tamanho actual | Paridade vanilla aproximada |
| Offset padrão | Sub: `-0.2em`; Super: `+0.3em` | Paridade vanilla aproximada |
| Scope-out | `offset`/`size` configuráveis | Subset minimal; paridade vanilla para caso comum |

---

## Scope-out explícito

- `offset` configurável (posição relativa à baseline) — continua scope-out; usa offset hardcoded padrão.
- `size` configurável (factor de escala) — continua scope-out; usa `0.6×` hardcoded.
- Nested sub/superscript (sub dentro de super, etc.) — continua scope-out; vanilla trata como composição simples.

---

## Critério de fecho

- [ ] `Style::Subscript`/`Superscript` adicionados ao enum.
- [ ] `StyleDelta` propaga as 2 flags; `TextStyle` atualizado.
- [ ] `Content::sub`/`super` construtores em `content.rs`.
- [ ] `native_sub`/`super` em `rules/stdlib/structural.rs`.
- [ ] Selectors `NodeKind::Subscript`/`Superscript` em `rules/eval/rules.rs`.
- [ ] Layout arms ajustam `TextStyle.size` e `baseline_offset`.
- [ ] 2 testes L1 (eval unit) + 2 testes L3 (layout E2E) verdes.
- [ ] Spec L0 atualizado.
- [ ] `cargo test --workspace` verde; `crystalline-lint` zero novas violações.

---

**Próximo passo:** Com P448 fechado, continuamos com **highlight** (P449, XS-S) ou **outline refinado** (P450, XS-S) ou **outras features de paridade**. Indique se quer ajustar o escopo do P448.
