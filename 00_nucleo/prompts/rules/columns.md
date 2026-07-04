# Prompt L0 — `rules/columns` — Layout de colunas multi-página

Hash do Código: b4352a93

**Camada**: L1  
**Ficheiro alvo**: `01_core/src/rules/layout/columns.rs`  
**Ficheiros adjacentes**: `01_core/src/rules/layout/cursor.rs`, `01_core/src/rules/layout/mod.rs`, `01_core/src/rules/layout/footnote.rs`, `01_core/src/entities/elements/columns.rs`  
**Origem**: P537 (colunas reais), P537b (`#set page(columns:)`), P552 (correcção de footnotes)  
**ADRs**: ADR-0107 (paridade linguagem), ADR-0108 (medir antes de decidir), ADR-0109 (atomização forma B), ADR-0054 (graded / scope-outs).

---

## 1. Contexto

O consumer `Content::Columns` serve dois contextos sintacticamente distintos:

1. **`#columns(N)[body]`** — contentor de colunas no fluxo regular.
2. **`#set page(columns: N)`** — configuração de página que, após transformação AST (`wrap_page_columns`), produz um `Content::Columns` sintético envolvendo o resto do escopo.

A implementação base (P537) introduziu colunas reais para ambos os contextos, mas tratava as notas de rodapé de forma uniforme. Medições de P551/P552 mostram que o vanilla distingue os dois contextos para footnotes. Este L0 corrige essa distinção.

---

## 2. Medições que sustentam a decisão

- `01_core/src/rules/layout/columns.rs:126` e `:161` — em `layout_segmented`, `footnote_counter` é salvo e restaurado por segmento. Combinado com `flush_pending_footnote_bodies()` ao fim de cada coluna, isso faz com que o contador não avance e cada coluna reutilize o mesmo número.
- P551 — imagens de `#set page(columns: 2)` mostram cristalino com notas lado a lado mas numeradas ambas como `[1]`; vanilla 0.14.2/0.15.0 numera `1`, `2` e também coloca lado a lado.
- P552 — testes adicionais com vanilla:
  - `#columns(2)[...]` com notas em ambas as colunas: vanilla empilha as notas na coluna esquerda.
  - `#columns(2)[#colbreak() #lorem(80)#footnote[...]]`: vanilla coloca a nota na coluna esquerda, não na direita.
  - `#set page(columns: 2)` com notas em ambas as colunas: vanilla coloca cada nota no fundo da respectiva coluna.

---

## 3. Decisão

### 3.1 Contador de notas

O `footnote_counter` do `Layouter` é **monotónico por invocação de layout** e **nunca é reiniciado dentro de `columns::layout`**. Cada `Content::Footnote` consumido incrementa o contador exactamente uma vez, independentemente de `colbreak()` ou de quantas colunas existem.

### 3.2 Posicionamento das notas

| Contexto | Semântica de footnotes no vanilla | Comportamento cristalino a implementar |
|---|---|---|
| `#set page(columns: N)` | Notas no fundo da **coluna onde são referenciadas**; contador contínuo. | Preservar comportamento por coluna. |
| `#columns(N)[body]` | Notas acumuladas e renderizadas no **final do contentor**, alinhadas à **primeira coluna** (esquerda); contador contínuo. | Alterar `layout_segmented` para acumular `pending_footnote_bodies` durante os segmentos e flushar só no final do contentor, na posição x da primeira coluna. |

### 3.3 Distinguir os dois contextos no layout

O `Content::Columns` sintético de `#set page(columns:)` deve ser marcado como "colunas de página". A forma de menor blast radius é adicionar um campo `page_columns: bool` a `ColumnsElem`:

- `false` (default): comportamento `#columns(N)[...]` — notas empilhadas no final.
- `true`: comportamento `#set page(columns: N)` — notas por coluna.

A transformação `wrap_page_columns` (P537b) cria o `ColumnsElem` com `page_columns: true`. A função stdlib `columns()` cria com `page_columns: false`.

---

## 4. Alterações autorizadas

### 4.1 `ColumnsElem` — adicionar `page_columns`

Em `01_core/src/entities/elements/columns.rs`:

```rust
pub struct ColumnsElem {
    pub count:         usize,
    pub gutter:        Option<Length>,
    pub body:          Content,
    /// **P552** — `true` quando este Columns foi produzido por `#set page(columns:)`.
    /// Distingue a semântica de footnotes: por coluna (página) vs empilhadas (contentor).
    pub page_columns:  bool,
}
```

Default: `page_columns: false`. Todos os `map_content`/`map_text` e construtores existentes preservam o valor.

### 4.2 `Content::columns` — construtor com flag

Em `01_core/src/entities/content.rs`, manter `Content::columns(count, gutter, body)` como default `page_columns: false`. Adicionar `Content::columns_page(count, gutter, body)` ou parâmetro nomeado opcional para uso por `wrap_page_columns`.

### 4.3 `wrap_page_columns` — marcar colunas de página

Em `01_core/src/rules/eval/mod.rs` (ou onde viver a transformação), o `Content::Columns` produzido a partir de `Content::SetPage { columns: Some(n), .. }` deve usar `page_columns: true`.

### 4.4 `columns::layout_segmented` — corrigir contador e posicionamento

Em `01_core/src/rules/layout/columns.rs`:

1. **Remover save/restore de `footnote_counter`** (linhas 126 e 161). O contador avança naturalmente.
2. Se `e.page_columns` for `true`:
   - Manter flush de `pending_footnote_bodies` no fim de cada coluna (`layout_segmented` actual).
   - As notas são transladadas juntamente com os items da coluna.
3. Se `e.page_columns` for `false`:
   - **Não flushar** `pending_footnote_bodies` no fim de cada coluna.
   - Acumular os bodies de footnote num buffer local ao `layout_segmented`.
   - No final do loop, após recolher todos os items das colunas, posicionar as notas no fundo da primeira coluna (x = `column_x_offsets[0]`, y = altura máxima consumida pelas colunas ou `height - margin`, conforme já usado pelo flush).
   - Adicionar os items das notas a `all_column_items` (ou a `current_items` directamente) já transladados para a coluna esquerda.

### 4.5 `columns::layout_flow` — sem alterações de footnotes

O modo fluxo contínuo (sem `colbreak()`) é usado por `#columns(N)[body]` quando não há colbreaks suficientes. Neste modo, o flush de notas já acontece no fim de cada coluna preenchida pelo `Layouter`. Para `#columns()` isso ainda difere do vanilla (que acumula até ao fim do contentor), mas o caso sem `colbreak()` é secundário para P552; pode ser tratado na mesma alteração se o mecanismo de `page_columns` permitir.

**Scope-out consciente**: se a alteração de `layout_flow` para empilhar notas exigir refacção profunda do `Layouter`, fica para passo seguinte. O critério mínimo de P552 é corrigir `layout_segmented` (caso com `colbreak()`), que é o observado nas imagens.

### 4.6 Testes

- Teste do contador: duas notas em `#set page(columns: 2)` produzem `[1]` e `[2]`.
- Teste do contador: três notas em `#set page(columns: 2)` produzem `[1]`, `[2]`, `[3]`.
- Teste de `#columns(2)[...]` com duas notas: notas empilhadas na coluna esquerda.
- Regressão: `#columns(2)[...]` sem notas continua a funcionar.
- Regressão: `#set page(columns: 2)` continua com notas lado a lado.

---

## 5. Restrições

- Não se remove o `match` exaustivo em `layout_content`.
- Não se introduz despacho dinâmico (`dyn`/vtable/PropMap).
- A lógica permanece em L1 (sem I/O, sem estado global).
- `#set page(columns: N)` a meio do documento continua scope-out per ADR-0054.
- Gutter personalizado via `#set page(gutter: ...)` continua scope-out.

---

## 6. Critérios de verificação

- `#set page(columns: 2)` com duas notas: `[1]` na primeira coluna, `[2]` na segunda; notas no fundo de cada coluna.
- `#set page(columns: 2)` com três notas: `[1]`, `[2]`, `[3]` correctos.
- `#columns(2)[...]` com duas notas: notas empilhadas na coluna esquerda.
- `cargo test --workspace` limpo.
- `crystalline-lint .` limpo.
