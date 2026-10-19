# Relatório — Passo 626 (P626)

**Data:** 2026-07-09  
**Commit de implementação:** `8368c56fc`  
**Foco:** Direcção de preenchimento de colunas em RTL.

---

## Resumo executivo

P625 encontrou e documentou que `#set page(columns: 2)` com texto árabe preenchia a coluna esquerda primeiro no cristalino, enquanto o vanilla 0.15.0 preenche a coluna direita primeiro. P626 corrige a ordem de preenchimento para seguir a direcção de leitura do texto.

A direcção efectiva (`text.dir`) viaja no `Content::Styled` que envolve o body do `ColumnsElem`; no momento em que `columns::layout` é invocado, a chain activa do `Layouter` ainda não incluiu esses estilos. A correcção lê `"text.dir"` percorrendo o body do elemento e, quando é `Dir::RTL`, inverte o vector `column_x_offsets` antes de o usar em `layout_segmented`/`layout_flow`.

---

## Sonda — comportamento do vanilla

### Comandos

```bash
cat > /tmp/p626-tres-colunas.typ <<'EOF'
#set page(columns: 3)
#set text(lang: "ar", dir: rtl, size: 20pt)
#lorem(300)
EOF
lab/typst-original/target/release/typst compile /tmp/p626-tres-colunas.typ /tmp/p626-vanilla.pdf
mutool trace /tmp/p626-vanilla.pdf | grep -A2 -m1 'fill_text'
```

### Resultados

| Caso | Primeiro texto observado (vanilla 0.15.0) | Interpretação |
|------|------------------------------------------|---------------|
| 2 colunas | `transform="1 0 0 1 344.18946 84.02612"` | Coluna da direita preenchida primeiro. |
| 3 colunas | `transform="1 0 0 1 416.72947 84.02612"` | Coluna mais à direita preenchida primeiro; ordem completa é direita → meio → esquerda. |

---

## Implementação

### Ficheiros alterados

- `00_nucleo/prompts/engine/columns.md`:
  - Nova secção 5.1 (P626) com as medições da sonda, a decisão arquitetural e os critérios de verificação.
  - Hash actualizado para `05d7626d` via `crystalline-lint --fix-hashes`.

- `01_core/src/engine/layout/columns.rs`:
  - Adicionadas as funções auxiliares `body_dir` e `styles_dir` para extrair `"text.dir"` do `Content::Styled` do body (incluindo recursão por `Sequence`).
  - Em `columns::layout`, a direcção RTL é determinada por `body_dir(&e.body) == Some(Dir::RTL)` em vez de `layouter.chain.custom("text.dir")` (que estava `None` neste ponto).
  - Quando RTL, `column_x_offsets` é invertido com `.into_iter().rev().collect()`, fazendo com que o índice 0 seja a coluna mais à direita.

- `01_core/src/engine/layout/tests.rs`:
  - Adicionado `p626_set_page_columns_rtl_preenche_direita_primeiro`: verifica que o primeiro texto de um documento 2-colunas RTL aparece na coluna da direita (`x > centro da página`).

---

## Validação

### Testes automatizados

```bash
cargo test --workspace
```

Resultado: **todos passam**.

```bash
crystalline-lint .
```

Resultado: **No violations found**.

### Casos visuais

| Caso | Primeiro texto observado (cristalino) | Comportamento |
|------|---------------------------------------|---------------|
| 2 colunas RTL | `x = 346.093` | Coluna da direita primeiro. Paridade com vanilla (`x ≈ 344.19`). |
| 3 colunas RTL | `x = 417.206` | Coluna mais à direita primeiro. Paridade com vanilla (`x ≈ 416.73`). |
| 2 colunas LTR | `x = 70.867` | Coluna da esquerda primeiro. Sem regressão. |
| Misto (LTR + RTL, cada um com `#set page(columns:)`) | Página 1 LTR começa à esquerda; página 2 RTL começa à direita. | Direcção resolvida por `ColumnsElem`. |

### Nota sobre o caso misto com `#pagebreak()` dentro do mesmo grupo

Se um documento contiver um único `#set page(columns:)` no topo e depois alternar `text.dir` usando apenas `#pagebreak()`, o `wrap_page_columns` agrupa todo o conteúdo num único `ColumnsElem`; neste cenário a direcção de preenchimento é a do primeiro `text.dir` encontrado no body. Este comportamento é coerente com a arquitectura actual de `wrap_page_columns` e não é específico de P626.

---

## Decisões e notas

- **Por que ler do body em vez da chain?** A chain activa do `Layouter` em `columns::layout` ainda não recebeu os estilos do `Content::Styled` do body. O `text.dir` está empacotado no próprio body, pelo que a leitura tem de ser feita no AST do elemento.
- **Por que não adicionar um campo `dir` a `ColumnsElem`?** A leitura directa do body tem menor blast radius e cobre o caso principal (`#set page(columns:)` com `text.dir` no body). Se no futuro for necessário suportar `#columns()` dentro de um `#set text(dir: rtl)` ambiente, poder-se-á capturar a direcção no `native_columns` e propagá-la como campo.
- **Heurística de busca:** `body_dir` percorre `Sequence` e `Styled` de forma recursiva limitada; o primeiro `"text.dir"` encontrado vence. Elementos como `Pagebreak`, `Text`, etc., são folhas sem direcção.

---

## Critérios de fecho do passo

- [x] Sonda completa, comportamento do vanilla confirmado com duas e três colunas.
- [x] Ordem de preenchimento corrigida para RTL.
- [x] LTR sem regressão.
- [x] Documento misto testado (com separação por `#set page(columns:)`).
- [x] Sem regressão nos testes de colunas já existentes.
- [x] Sem regressão em `cargo test --workspace`.
- [x] `crystalline-lint .` limpo.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p626.md`, com hash do commit.
