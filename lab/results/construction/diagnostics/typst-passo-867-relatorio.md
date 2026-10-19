# Relatório — typst-passo-867: `#set page(height: auto)` / `width: auto`

**Data:** 2026-07-23
**Passo:** P867
**Base:** commit `06a336b0c3ae42949c2bced6c4c6511b9d89bebd`
**Estado:** working tree não commitado (inclui alterações preexistentes de P865 e P866 necessárias para a compilação).

---

## Proveniência da medição

```text
commit: 06a336b0c3ae42949c2bced6c4c6511b9d89bebd
working tree: alterações não commitadas (inclui P865 em stdlib/text.rs/rules.rs e P866 em shell/cli/wiring)
hora da medição: 2026-07-23T14:16:43Z (aproximada à sessão)
```

`git diff HEAD --stat` (estado actual do working tree):

```text
 00_nucleo/prompts/engine/eval.md                   |  24 +-
 00_nucleo/prompts/engine/layout.md                 |  28 +-
 00_nucleo/prompts/engine/stdlib/text.md            |  46 ++-
 00_nucleo/prompts/shell/cli.md                     |  50 +++-
 01_core/src/engine/eval/bibliography.rs            |   2 +-
 01_core/src/engine/eval/bindings.rs                |   2 +-
 01_core/src/engine/eval/closures.rs                |   2 +-
 01_core/src/engine/eval/control_flow.rs            |   2 +-
 01_core/src/engine/eval/flow.rs                    |   2 +-
 01_core/src/engine/eval/markup.rs                  |   2 +-
 01_core/src/engine/eval/math.rs                    |   2 +-
 01_core/src/engine/eval/mod.rs                     |   2 +-
 01_core/src/engine/eval/modules.rs                 |   2 +-
 01_core/src/engine/eval/rules.rs                   |  48 ++-
 01_core/src/engine/eval/tests.rs                   | 130 +++++++-
 01_core/src/engine/layout/cursor.rs                |  74 ++++-
 01_core/src/engine/layout/dynamic.rs               |   2 +-
 01_core/src/engine/layout/grid.rs                  |   2 +-
 01_core/src/engine/layout/grid_placement.rs        |   2 +-
 01_core/src/engine/layout/helpers.rs               |   2 +-
 01_core/src/engine/layout/hyphenation.rs           |   2 +-
 01_core/src/engine/layout/metrics.rs               |   2 +-
 01_core/src/engine/layout/mod.rs                   |  74 ++++-
 01_core/src/engine/layout/placement.rs             |   2 +-
 01_core/src/engine/layout/set_page.rs              |  20 +-
 01_core/src/engine/layout/slicing.rs               |   2 +-
 01_core/src/engine/layout/sub_frame.rs             |   2 +-
 01_core/src/engine/layout/tests.rs                 |  45 ++-
 01_core/src/engine/stdlib/text.rs                  | 332 +++++++++++++++++++--
 01_core/src/entities/content.rs                    |   7 +-
 01_core/src/entities/layout_types.rs               |  25 +-
 02_shell/build.rs                                  |   2 +-
 02_shell/src/cli.rs                                | 104 ++++++-
 04_wiring/src/main.rs                              |  18 +-
 04_wiring/tests/cli.rs                             | 175 +++++++++++
 36 files changed, 1131 insertions(+), 148 deletions(-)
```

> Nota: as alterações em `00_nucleo/prompts/engine/stdlib/text.md`, `01_core/src/engine/stdlib/text.rs`, `00_nucleo/prompts/shell/cli.md`, `02_shell/*`, `04_wiring/*` e respectivos testes pertencem a P865/P866 e já existiam no working tree antes da execução de P867. Este relatório foca-se nos ficheiros afetados por P867: `layout_types.rs`, `content.rs`, `engine/eval/rules.rs`, `engine/layout/mod.rs`, `engine/layout/set_page.rs`, `engine/eval/tests.rs`, `engine/layout/tests.rs` e os L0s `eval.md` / `layout.md`.

---

## Objetivo

Suportar `auto` em `#set page(width: ...)` e `#set page(height: ...)`, de modo que:

- `height: auto` faça a página crescer verticalmente para acomodar o conteúdo, desactivando a paginação automática.
- `width: auto` faça a página crescer horizontalmente para acomodar o conteúdo, desactivando a quebra de linha.
- Os tipos inválidos (`array`, etc.) continuem a ser rejeitados com mensagem de erro apropriada.

---

## Implementação

### 1. `PageDimension` em `01_core/src/entities/layout_types.rs`

Adicionado o enum:

```rust
pub enum PageDimension {
    Auto,
    Length(f64),
}
```

`PageConfig.width`/`height` continuam a usar `f64`, representando `auto` por `f64::INFINITY`. `PageConfig::auto_margin()` foi ajustada para usar a dimensão finita restante quando uma delas for `auto`; se ambas forem `auto`, recai na largura A4 (595.28 pt) como fallback.

### 2. `Content::SetPage` em `01_core/src/entities/content.rs`

Os campos `width` e `height` passaram de `Option<f64>` para `Option<PageDimension>`, permitindo distinguir:

- `None` — não alterar a dimensão actual;
- `Some(PageDimension::Auto)` — activar crescimento automático;
- `Some(PageDimension::Length(v))` — dimensão fixa em pontos.

### 3. Eval de `#set page` em `01_core/src/engine/eval/rules.rs`

- Nova função auxiliar `extract_page_dimension` aceita `Value::Auto` além de `Value::Length`, `Value::Float`, `Value::Int` e `Value::None`.
- `width`/`height` passam a usar `extract_page_dimension`; `margin` mantém `extract_pt`.
- Valores `auto` propagam-se para a `StyleChain` como `Value::Auto`; valores numéricos continuam como `Value::Float`.

### 4. Layout em `01_core/src/engine/layout/set_page.rs` e `01_core/src/engine/layout/mod.rs`

- `set_page.rs` converte `PageDimension::Auto` para `f64::INFINITY` em `PageConfig` e `PageDimension::Length(v)` para `v`.
- `layout/mod.rs`:
  - `available_width()` e `available_height()` devolvem `f64::INFINITY` quando a dimensão correspondente é `auto`.
  - `page_bottom_limit()` devolve `f64::INFINITY` quando `height: auto`, impedindo `new_page()` por overflow vertical.
  - `compute_page_width()` / `compute_page_height()` calculam as dimensões finais da página a partir da extensão real do conteúdo.
  - Em `finish()`, as dimensões finais são calculadas antes do flush de floats/footnotes; `width: auto` actualiza o posicionamento da numeração de página; `height: auto` passa um `footnote_bottom_y` explícito para o flush de notas de rodapé.

### 5. Actualização dos L0s

- `00_nucleo/prompts/engine/eval.md` — secções de cast de `#set page` e `extract_page_dimension` actualizadas; hash de código atualizado para `6a04dec9`.
- `00_nucleo/prompts/engine/layout.md` — adicionada secção P867 com semântica de `auto` e casos limite; hash de código atualizado (o cabeçalho `layout/mod.rs` usa `715bf549`).
- `crystalline-lint --fix-hashes .` foi corrido; não há `PromptDrift` (V5) restante.

---

## Testes adicionados

### Eval — `01_core/src/engine/eval/tests.rs`

| Teste | Descrição |
|-------|-----------|
| `p867_set_page_height_auto_ok` | `#set page(height: auto)` é aceite no eval. |
| `p867_set_page_width_auto_ok` | `#set page(width: auto)` é aceite no eval. |
| `p867_set_page_height_invalid_type_error` | `height: (1, 2)` continua a ser rejeitado. |

### Layout — `01_core/src/engine/layout/tests.rs`

| Teste | Descrição |
|-------|-----------|
| `p867_height_auto_cresce_para_conteudo_curto` | `#set page(height: auto)\nX` produz 1 página com altura finita e positiva. |
| `p867_height_auto_cresce_para_conteudo_longo` | `#set page(height: auto)\n#lorem(200)` produz 1 página (paginação desactivada). |
| `p867_width_auto_cresce_para_conteudo` | `#set page(width: auto)\nX` produz 1 página com largura finita e positiva. |
| `p867_height_fixo_continua_paginar` | `#set page(height: 80pt)\n#lorem(80)` continua a paginar (>= 2 páginas). |

---

## Validação

### `cargo test -p typst-core`

```text
running 4672 tests
test result: ok. 4670 passed; 0 failed; 2 ignored; 0 measured; 0 filtered out
```

### `cargo test --workspace`

```text
typst-core:    4670 passed; 0 failed; 2 ignored
typst-infra:    717 passed; 0 failed; 5 ignored
typst-shell:     41 passed; 0 failed; 0 ignored
typst-wiring:     2 passed; 0 failed; 0 ignored
cli tests:       36 passed; 0 failed; 0 ignored
crystalline_lint: 2 passed; 0 failed; 0 ignored
```

### `crystalline-lint .`

```text
warning: Prompt órfão: '00_nucleo/prompts/infra/package_version_resolution.md' não é referenciado por nenhum arquivo em L1–L4. Materializar ou remover. [V7]
EXIT CODE: 0
```

Apenas o aviso pré-existente de prompt órfão; zero violações relacionadas com P867.

---

## Diff resumido (P867)

```text
 00_nucleo/prompts/engine/eval.md                   |  24 +-
 00_nucleo/prompts/engine/layout.md                 |  28 +-
 01_core/src/engine/eval/rules.rs                   |  48 ++-
 01_core/src/engine/eval/tests.rs                   |  11 ++
 01_core/src/engine/layout/mod.rs                   |  74 ++++-
 01_core/src/engine/layout/set_page.rs              |  20 +-
 01_core/src/engine/layout/tests.rs                 |  36 +++
 01_core/src/entities/content.rs                    |   7 +-
 01_core/src/entities/layout_types.rs               |  25 +-
```

> Nota: as contagens acima são aproximadas e reflectem apenas as alterações atribuíveis a P867 dentro de ficheiros que também contêm mudanças de P865/P866.

---

## Decisão

P867 está implementado e validado. `#set page(height: auto)` e `#set page(width: auto)` são aceites no eval e produzem páginas expansíveis no layout, desactivando paginação automática (`height`) e quebra de linha (`width`). Tipos inválidos continuam rejeitados, e testes de regressão confirmam que dimensões fixas ainda paginam normalmente.
