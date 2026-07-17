# P446 — Smallcaps

> **Passo:** 446  
> **Data:** 2026-06-24  
> **Foco:** Materializar a função nativa `smallcaps` para converter texto em small capitals.  
> **Pré-requisitos:** P445 (smart quotes fechado).  

---

## Contexto

O Typst vanilla tem `#smallcaps[text]` como elemento nativo que converte letras minúsculas em small capitals (maiúsculas reduzidas) preservando maiúsculas e não-letras. O cristalino não tem esta funcionalidade. Quick win tipográfico XS.

---

## ADR-0108 — Medir antes de decidir

**FASE A.0 — Sonda:**

| Pergunta | Resultado | Status |
|----------|-----------|--------|
| `smallcaps` existe no cristalino? | Não | ❌ |
| Vanilla implementa como `Style` ou `Content`? | `Style` — `Smallcaps(bool)` no `TextStyle` | ✅ |
| `Style` enum suporta novo campo? | Sim — ADR-0038 extensível | ✅ |
| Fonte OpenType tem smallcaps? | Sim — via `font.smallcaps()` ou `font.variant(SmallCaps)` | ✅ |
| Bloqueadores? | Nenhum técnico | ✅ |

**Reclassificação:** XS (~15 min; 1 função nativa + 1 style + 1 layout arm + tests).

---

## ADR-0107 — Paridade linguagem

Contrato comportamental: `#smallcaps[Hello World]` produz texto onde letras minúsculas (`ello`, `orld`) são renderizadas como small capitals; maiúsculas (`H`, `W`) permanecem maiúsculas regulares. Não-letras (espaços, pontuação) são preservadas.

---

## ADR-0109 — Atomização forma B

**Toques pontuais:**

1. **`entities/style.rs`** — Adicionar `Style::Smallcaps(bool)` ao enum `Style` (análogo a `Bold`/`Italic`/`Underline`).
2. **`entities/style_chain.rs`** — `StyleDelta` ganha `smallcaps: bool`; `fold_into` propaga; `TextStyle` ganha `smallcaps: bool`.
3. **`entities/content.rs`** — Construtor `smallcaps(body)` emite `Content::Styled(body, Styles::from_iter([Style::Smallcaps(true)]))`.
4. **`rules/stdlib/structural.rs`** — `native_smallcaps` (análogo a `native_strong`/`native_emph`).
5. **`rules/eval/rules.rs`** — Selector `NodeKind::Smallcaps` casa `Style::Smallcaps` no `Content::Styled`.
6. **`engine/layout/mod.rs`** — Arm de `Content::Styled` com `Style::Smallcaps`:
   - Se `smallcaps == true`, o layout usa `font.variant(SmallCaps)` para glyphs de letras minúsculas.
   - Ou, alternativa: converte texto minúsculo para maiúsculo e aplica factor de escala 0.8x (fallback se fonte não tiver smallcaps nativo).
   - Vanilla usa `font.smallcaps()` quando disponível; fallback para scaling.
7. **`engine/layout/tests.rs`** — Teste de integração: `smallcaps_aparece_em_pdf`.
8. **`rules/eval/tests.rs`** — Teste unitário: `eval_smallcaps_emite_styled`.
9. **Spec L0** — Adicionar secção em `00_nucleo/prompts/engine/stdlib/structural.md`.

---

## Decisões arquiteturais

| Decisão | Opção escolhida | Justificativa |
|---------|----------------|---------------|
| Modelo | `Content::Styled` + `Style::Smallcaps` | Paridade com vanilla; reusa infra strong/emph |
| Fallback | Scaling 0.8x se fonte não tiver smallcaps | Vanilla faz o mesmo; garante funcionalidade universal |
| Scope | Apenas `smallcaps`; não `upper`/`lower` | Paridade vanilla; `upper`/`lower` são funções de string, não style |

---

## Scope-out explícito

- `upper`/`lower` como funções de string — scope-out; são métodos de `str`, não elementos de estilo.
- `font.smallcaps()` nativo do OpenType — usa se disponível, mas fallback por scaling é aceitável.
- Kerning ajustado para smallcaps — scope-out; usa kerning padrão da fonte.

---

## Critério de fecho

- [ ] `Style::Smallcaps` adicionado ao enum.
- [ ] `StyleDelta` propaga `smallcaps`; `TextStyle` atualizado.
- [ ] `Content::smallcaps` construtor em `content.rs`.
- [ ] `native_smallcaps` em `rules/stdlib/structural.rs`.
- [ ] Selector `NodeKind::Smallcaps` em `rules/eval/rules.rs`.
- [ ] Layout arm emite texto com smallcaps (fonte nativa ou scaling).
- [ ] 1 teste L1 (eval unit) + 1 teste L3 (layout E2E) verdes.
- [ ] Spec L0 atualizado.
- [ ] `cargo test --workspace` verde; `crystalline-lint` zero novas violações.

---

**Próximo passo:** Com P446 fechado, continuamos com **sub/superscript** (P447, XS-S) ou **outras features de paridade**. Indique se quer ajustar o escopo do P446.
