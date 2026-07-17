# Passo 431 — relatório: fecho DEBT-50 (show selector Strong/Emph distingue origem)

**Tipo:** refactor comportamental em L1 (zero I/O; alterações localizadas em
`Style`, `StyleDelta`, `StyleChain`, eval e layout).  
**Data:** 2026-06-23. **HEAD base:** `632f67f19`.  
**Caveat de stack:** suíte completa corre com `RUST_MIN_STACK=8388608`.

## O que se fez

Fechou-se a dívida técnica **DEBT-50**: o selector `NodeKind::Strong`/`Emph`
não deve disparar para texto afectado por `#set text(bold: true)` /
`#set text(italic: true)` — apenas para `*bold*` / `_italic_` sintáticos.

A decisão do P431 (Opção α) foi introduzir uma **flag de origem no enum
`Style`**:

- `Style::Bold { value: bool, from_strong: bool }`
- `Style::Italic { value: bool, from_emph: bool }`

Para que essa origem sobreviva ao folding de `Styles`/`StyleChain`, o backing
`StyleDelta` ganhou campos `bold_from_strong` / `italic_from_emph`, e
`StyleChain::collapse`/`StyleDelta::fold_into` passaram a propagá-los.

### Ficheiros alterados

- **`01_core/src/entities/style.rs`**:
  - `Style::Bold(bool)` → struct variant com `value` + `from_strong`.
  - `Style::Italic(bool)` → struct variant com `value` + `from_emph`.
  - Adicionados helpers `Style::bold`, `Style::italic`, `Style::strong`,
    `Style::emph` para construção canónica.
  - `fold_into` propaga as flags de origem para o `StyleDelta`.

- **`01_core/src/entities/style_chain.rs`**:
  - `StyleDelta` ganha `bold_from_strong` e `italic_from_emph`.
  - `empty()` / `is_empty()` actualizados.
  - `collapse()` propaga as flags de origem.
  - Adicionado `StyleDelta::diff_styles` para construir uma `Styles` com
    apenas os campos que mudaram — usado no transporte de `#set` em
    `eval_markup`.

- **`01_core/src/engine/eval/rules.rs`**:
  - `#set text(bold: true)` emite `Style::bold(true)` (`from_strong: false`).
  - `#set text(italic: true)` emite `Style::italic(true)` (`from_emph: false`).
  - `selector_matches` para `NodeKind::Strong`/`Emph` verifica a origem no
    delta de um eventual `Content::Styled`, mantendo o match das variantes
    próprias `Content::Strong`/`Emph`.

- **`01_core/src/engine/eval/mod.rs`**:
  - A detecção de fronteiras de `#set` em `eval_markup` passou a observar o
    **delta completo** (não só o canal `custom`), de modo que alterações em
    `text.bold`/`text.italic` sejam transportadas num `Content::Styled`
    aninhado, preservando a origem.

- **`01_core/src/engine/layout/mod.rs`**:
  - Braços `Content::Strong`/`Emph` empurram `Style::strong()` /
    `Style::emph()` (`from_strong`/`from_emph: true`).

- **`01_core/src/engine/layout/term_item.rs`**:
  - Negrito do termo usa `Style::bold(true)` (origem genérica).

- **`01_core/src/engine/eval/tests.rs`**:
  - Helpers de show-set ajustados para lerem o campo tipado `bold` em vez do
    canal `custom` `"text.bold"`.

- **`01_core/src/engine/layout/tests.rs`**:
  - Teste `debt_50_show_strong_nao_apanha_set_text_bold_porque_bake_in`
    renomeado para `debt_50_show_strong_nao_apanha_set_text_bold` e
    actualizado para o cenário pós-bake-in (wrapping).

- **`00_nucleo/diagnosticos/debt/DEBT.md`**:
  - DEBT-50 movido para a Secção 2 (ENCERRADO) com critérios todos assinalados.

## Decisão de engenharia

- **Opção α do P431** (flag no enum `Style`) foi a escolhida porque distingue
  semanticamente a *origem* do estilo, independentemente da forma do wrapper
  (`Content::Strong` vs `Content::Styled`).
- **Backward compat**: `value` continua a ser o campo observável pelo layout;
  `from_strong`/`from_emph` só influenciam selectors.
- **Scope-out preservado**: a migração real de bake-in para wrapping continua
  fora do P431; este passo apenas prepara o mecanismo de distinção.

## Verificação

| Critério | Resultado |
|----------|-----------|
| `Style::Bold`/`Italic` com campos `from_strong`/`from_emph` | ✓ |
| `StyleDelta` propaga origem | ✓ |
| `#set text(bold: true)` emite `from_strong: false` | ✓ |
| Selector `NodeKind::Strong`/`Emph` distingue origem | ✓ |
| Teste `debt_50_show_strong_nao_apanha_set_text_bold` passa | ✓ |
| `cargo test --workspace` verde | ✓ |
| `crystalline-lint` sem novas violações | ✓ |

## Artefactos

- Código:
  - `01_core/src/entities/style.rs`
  - `01_core/src/entities/style_chain.rs`
  - `01_core/src/engine/eval/rules.rs`
  - `01_core/src/engine/eval/mod.rs`
  - `01_core/src/engine/eval/tests.rs`
  - `01_core/src/engine/layout/mod.rs`
  - `01_core/src/engine/layout/term_item.rs`
  - `01_core/src/engine/layout/tests.rs`
- Débito:
  - `00_nucleo/diagnosticos/debt/DEBT.md`
- Plano:
  - `00_nucleo/materialization/typst-passo-431.md`
- este relatório.

## Próximo passo

Com DEBT-50 fechado, o bloqueador futuro para a migração de bake-in para
wrapping desaparece. Pode-se retomar **DEBT-57** (outro subset stdlib L0) ou
**DEBT-55** (probe hayagriva/CSL).
