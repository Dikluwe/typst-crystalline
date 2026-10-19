# P431 — Fecho de débito: DEBT-50 (show selector Strong/Emph distingue origem)

---

## Contexto

**DEBT-50** está latente desde o Passo 103 (ADR-0041). O teste `debt_50_show_strong_nao_apanha_set_text_bold_porque_bake_in` passa hoje porque `#set text(bold: true)` usa bake-in (`TextStyle` capturado em `Content::Text`), não wrapping (`Content::Styled`).

Quando o bake-in migrar para wrapping (pré-requisito de `Introspection` real e `morph_canon` vanilla-faithful), o selector `NodeKind::Strong` começará a apanhar `#set text(bold: true)` como false positive. O fecho agora evita que essa migração futura seja bloqueada.

---

## ADR-0108 — Medir antes de decidir

**FASE A.0 — Sonda:**

| Pergunta | Resultado | Status |
|----------|-----------|--------|
| `Content::Styled(_, ss)` com `Style::Bold(true)` é usado por `*bold*`? | Sim — `Content::strong()` emite `Styled([Bold(true)])` | ✅ |
| `#set text(bold: true)` emite `Styled` hoje? | Não — usa bake-in `TextStyle` em `Content::Text` | ✅ (latente) |
| Selector `NodeKind::Strong` casa `Styled` com `Bold(true)`? | Sim — `ss.iter().any(|s| matches!(s, Style::Bold(true)))` | ✅ |
| Teste de guarda existe? | Sim — `debt_50_show_strong_nao_apanha_set_text_bold_porque_bake_in` | ✅ |
| Bloqueadores? | Nenhum externo | ✅ |

**Reclassificação:** S (1 enum modificado + 2 arms de match + 1 teste atualizado).

---

## ADR-0107 — Paridade linguagem

O contrato é **comportamental**: `#show strong: it => [HIT]` deve disparar apenas para `*bold*` sintático, nunca para `#set text(bold: true)`. Paridade vanilla literal.

---

## ADR-0109 — Atomização forma B

**Opções analisadas:**

| Opção | Mecanismo | Toques | Risco |
|-------|-----------|--------|-------|
| α | `Style::Bold { value: bool, from_strong: bool }` | ~15 sítios (`Style` enum, `push_styles`, `TextStyle::from`, tests) | Médio — toca `Style` central |
| β | Marcador em `Content::Styled(body, styles, origin: Option<ElementKind>)` | ~40 sítios (struct `Styled` usado em dezenas de arms) | Alto — toca `Content` central |
| γ | Selector rigoroso: casa apenas `Styles` com exatamente `[Bold(true)]` | 2 LOC no selector | Baixo — mas frágil: `#set text(bold: true)` pode emitir `[Bold(true)]` exatamente |

**Decisão: Opção α** — flag no enum `Style`. É a única que distingue semanticamente a *origem* do estilo, independente da forma do `Styles` wrapper. O custo é localizado ao enum `Style` e seus consumers diretos.

**Toques pontuais:**
1. `entities/style.rs` — `Style::Bold(bool)` → `Style::Bold { value: bool, from_strong: bool }` (análogo a `Style::Italic { value: bool, from_emph: bool }`).
2. `entities/style_chain.rs` — `push_styles` arm `Style::Bold` preserva `from_strong` na propagação.
3. `rules/eval/rules.rs` — `eval_set_rule` para `text.bold` emite `Style::Bold { value: true, from_strong: false }`.
4. `entities/content.rs` — `Content::strong()` emite `Style::Bold { value: true, from_strong: true }`.
5. `rules/eval/rules.rs` — selector `NodeKind::Strong` casa `Style::Bold { from_strong: true, .. }` (não `value: true` genérico).
6. Teste `debt_50_show_strong_nao_apanha_set_text_bold_porque_bake_in` atualizado para simular cenário pós-bake-in (wrapping) e assert que HIT não aparece.

---

## Decisões arquiteturais

| Decisão | Opção escolhida | Justificativa |
|---------|----------------|---------------|
| Mecanismo de distinção | Flag `from_strong`/`from_emph` no enum `Style` | Semântica de origem preservada através de toda a cadeia `eval → StyleChain → Styled → layout` |
| Default da flag | `from_strong: false` / `from_emph: false` | Bake-in e `#set text` usam default; sintaxe `*bold*`/`_italic_` seta `true` |
| Backward compat | Preservada — `value: bool` continua a ser o campo observável | `text.bold` e `strong` produzem ambos `value: true` no layout |
| `Style: Copy` | Mantido — `bool` + `bool` = 2 bytes, ainda `Copy` | Sem alteração de derive |

---

## Scope-out explícito

- Migração real de bake-in para wrapping — não é parte deste fecho; só prepara o mecanismo de distinção.
- `morph_canon` vanilla-faithful — continua adiado (DEBT-61 histórico).
- Outros selectors (`NodeKind::Emph` análogo; `NodeKind::Heading` já distingue por `HeadingLevel`).

---

## Critério de fecho

- [ ] `Style::Bold` e `Style::Italic` com campo `from_strong`/`from_emph`
- [ ] `Content::strong()`/`emph()` emitem `from_strong: true` / `from_emph: true`
- [ ] `#set text(bold: true)` emite `from_strong: false`
- [ ] Selector `NodeKind::Strong` casa apenas `from_strong: true`
- [ ] Selector `NodeKind::Emph` casa apenas `from_emph: true`
- [ ] Teste `debt_50_show_strong_nao_apanha_set_text_bold` asserte paridade vanilla pós-bake-in
- [ ] `cargo test --workspace` verde; `crystalline-lint` zero violations
- [ ] DEBT-50 reclassificado como **FECHADO** em `DEBT.md`

---

**Próximo passo:** Com P431 fechado, continuamos com **DEBT-57** (outro subset stdlib L0, ex: `layout.rs` ou `calc.rs`) ou **DEBT-55** (probe hayagriva). Indique se quer ajustar o escopo do P431.
