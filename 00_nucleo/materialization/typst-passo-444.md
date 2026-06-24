# P444 — Text decorações: `underline`, `overline`, `strike`

> **Passo:** 444  
> **Data:** 2026-06-24  
> **Foco:** Materializar as 3 funções nativas de decoração de texto (`underline`, `overline`, `strike`) em stdlib + layout + export PDF.  
> **Pré-requisitos:** P441-P443 (DEBT-42 fechado); ADR-0115/0116 `EM VIGOR`.  

---

## Contexto

O cristalino suporta `strong`/`emph` (bold/italic) desde os primeiros passos, mas **não tem** underline, overline nem strikethrough como elementos de texto. O Typst vanilla tem `#underline[text]`, `#overline[text]` e `#strike[text]` como elementos nativos de `text`, com parâmetros `stroke`, `offset`, `extent`, `evade` (underline/overline) e `background`.

Este passo materializa o **subset minimal** — as 3 funções com `body` e `stroke` básico (sem `evade`, `offset` avançado, ou `background`). Paridade vanilla para o caso comum.

---

## ADR-0108 — Medir antes de decidir

**FASE A.0 — Sonda:**

| Pergunta | Resultado | Status |
|----------|-----------|--------|
| `underline`/`overline`/`strike` existem em stdlib? | Não | ❌ |
| `FrameItem::Line` já existe (para math frac)? | Sim — P38 materializou `FrameItem::Line` | ✅ |
| Export PDF de linha já existe? | Sim — `q w m l S Q` para math frac line | ✅ |
| `Style` enum suporta novos campos? | Sim — ADR-0038 fundação extensível | ✅ |
| Bloqueadores? | Nenhum técnico | ✅ |

**Reclassificação:** S (~30 min; 3 funções nativas + 3 arms de layout + 3 arms de export + tests).

---

## ADR-0107 — Paridade linguagem

Contrato comportamental: `#underline[text]` desenha uma linha horizontal abaixo da baseline do texto; `#overline[text]` desenha acima; `#strike[text]` desenha através (meio da altura da linha). Paridade vanilla para o subset minimal (stroke padrão = cor do texto, 0.5pt).

---

## ADR-0109 — Atomização forma B

**Toques pontuais:**

1. **`entities/style.rs`** — Adicionar `Style::Underline`, `Style::Overline`, `Style::Strike` (variantes `bool`, análogo a `Bold`/`Italic`).
2. **`entities/style_chain.rs`** — `StyleDelta` ganha `underline: bool`, `overline: bool`, `strike: bool`; `fold_into` propaga.
3. **`entities/content.rs`** — Construtores `underline(body)`, `overline(body)`, `strike(body)` emitem `Content::Styled(body, Styles::from_iter([Style::Underline(true)]))` (análogo a `strong`/`emph`).
4. **`rules/stdlib/structural.rs`** — `native_underline`, `native_overline`, `native_strike` (3 funções nativas, análogo a `native_strong`/`native_emph`).
5. **`rules/eval/rules.rs`** — Selector `NodeKind::Underline`/`Overline`/`Strike` casam `Style::Underline`/`Overline`/`Strike` no `Content::Styled` (análogo a DEBT-50/P431).
6. **`rules/layout/mod.rs`** — Arms de `Content::Styled` com `Style::Underline`/`Overline`/`Strike` emitem `FrameItem::Line` posicionado:
   - Underline: `y = baseline + 0.2 * size` (abaixo da baseline)
   - Overline: `y = ascent - 0.1 * size` (acima do texto)
   - Strike: `y = (ascent - descent) / 2` (meio da altura da linha)
   - Linha: `start = (x, y)`, `end = (x + width, y)`, `thickness = 0.5pt` (default) ou valor do `stroke` se especificado.
7. **`03_infra/src/export.rs`** — `FrameItem::Line` já existe; nenhuma alteração necessária (reuso do operador PDF `q w m l S Q`).
8. **`rules/layout/tests.rs`** — 3 testes de integração: `underline_aparece_em_pdf`, `overline_aparece_em_pdf`, `strike_aparece_em_pdf`.
9. **`rules/eval/tests.rs`** — 3 testes unitários: `eval_underline_emite_styled`, `eval_overline_emite_styled`, `eval_strike_emite_styled`.
10. **Spec L0** — Adicionar secções em `00_nucleo/prompts/rules/stdlib/structural.md` (ou ficheiro dedicado `text_decoration.md`) para as 3 funções.

---

## Decisões arquiteturais

| Decisão | Opção escolhida | Justificativa |
|---------|----------------|---------------|
| Modelo de implementação | `Content::Styled` + `Style` enum (análogo a strong/emph) | Reusa infraestrutura existente; paridade com P431 |
| Render | `FrameItem::Line` (reuso do math frac) | Já existe em L1 e export; zero código novo de PDF |
| Stroke padrão | `0.5pt` na cor do texto | Paridade vanilla aproximada; vanilla lê da fonte, mas fallback é 0.5pt |
| Scope-out | `offset`, `extent`, `evade`, `background` | Subset minimal; paridade vanilla para caso comum |
| `Style: Copy` | Mantido — 3 novos `bool` = +3 bytes no enum | Sem alteração de derive |

---

## Scope-out explícito

- `offset` (posição relativa à baseline) — continua scope-out; usa posição hardcoded padrão.
- `extent` (extensão além do conteúdo) — continua scope-out; linha = largura exacta do texto.
- `evade` (evitar colisão com glyphs) — continua scope-out; linha é recta contínua.
- `background` (linha atrás do texto) — continua scope-out; linha é desenhada por cima (análogo ao vanilla default).
- `stroke` como `Stroke` object rico (thickness + paint + cap + join) — continua scope-out; aceita apenas `Length` (thickness) ou `Color` (paint) como subset.

---

## Critério de fecho

- [ ] `Style::Underline`/`Overline`/`Strike` adicionados ao enum `Style`.
- [ ] `StyleDelta` propaga as 3 flags; `TextStyle` e `StyleChain` atualizados.
- [ ] `Content::underline`/`overline`/`strike` construtores em `content.rs`.
- [ ] `native_underline`/`overline`/`strike` em `rules/stdlib/structural.rs`.
- [ ] Selectors `NodeKind::Underline`/`Overline`/`Strike` em `rules/eval/rules.rs`.
- [ ] Arms de layout em `rules/layout/mod.rs` emitem `FrameItem::Line` posicionado.
- [ ] 3 testes L1 (eval unit) + 3 testes L3 (layout E2E) verdes.
- [ ] Spec L0 atualizado (structural.md ou text_decoration.md).
- [ ] `cargo test --workspace` verde; `crystalline-lint` zero novas violações.
- [ ] `DEBT.md` sem alteração (não abre nem fecha DEBT).

---

**Próximo passo:** Com P444 fechado, continuamos com **text decorações avançadas** (offset, extent, evade — P445) ou **outras features de paridade** (smart quotes, smallcaps, sub/superscript). Indique se quer ajustar o escopo do P444.
