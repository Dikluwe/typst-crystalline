# Paridade de Produção — Passo 552

**Data:** 2026-07-03  
**Repositório:** `typst-crystalline`  
**Binários:**

- Cristalino: `./target/release/typst` (após P552)
- Vanilla 0.14.2: `/usr/local/bin/typst`
- Vanilla 0.15.0: `lab/typst-original/target/release/typst`
- Ferramentas: `mutool` 1.23.10, `pdftotext`

---

## 1. Objetivo

Corrigir dois problemas reais de footnotes em colunas identificados em P551:

1. **Numeração duplicada:** `#set page(columns: 2)` com notas em ambas as colunas repetia `[1]`/`[1]` em vez de `[1]`/`[2]`.
2. **Posicionamento de `#columns(2)`:** o cristalino colocava notas lado a lado no fundo de cada coluna; o vanilla empilha-as na coluna esquerda.

---

## 2. Diagnóstico

### 2.1 Causa da numeração duplicada

`01_core/src/rules/layout/columns.rs:126` e `:161` salvavam e restauravam `footnote_counter` em cada segmento de coluna dentro de `layout_segmented`. Como o flush de footnotes acontecia ao fim de cada coluna, o contador era reiniciado antes de processar a coluna seguinte, fazendo com que todas as colunas reutilizassem o mesmo número.

### 2.2 Causa do posicionamento de `#columns(2)`

O consumer `Content::Columns` não distinguia entre a forma-função `#columns(N)[...]` e o contentor sintético produzido por `#set page(columns:)`. Ambos usavam `layout_segmented`, que colocava footnotes no fundo de cada coluna. O vanilla, no entanto, trata `#columns()` como um contentor no fluxo: as footnotes são acumuladas e renderizadas no final do contentor, alinhadas à primeira coluna.

---

## 3. Alterações implementadas

### 3.1 `ColumnsElem` — distinção de origem

- `01_core/src/entities/elements/columns.rs` — adicionado campo `page_columns: bool`.
- `01_core/src/entities/content.rs` — construtor `Content::columns(...)` usa `page_columns: false`; a transformação `wrap_page_columns` (para `#set page(columns:)`) usa `page_columns: true`.

### 3.2 `columns::layout_segmented` — semântica correta

- `01_core/src/rules/layout/columns.rs`:
  - Removido save/restore de `footnote_counter`; o contador avança monotonicamente.
  - Com `page_columns == true`: flush de footnotes no fim de cada coluna (comportamento de página).
  - Com `page_columns == false`: acumula footnotes durante os segmentos e flush único no final, posicionado na primeira coluna (comportamento de contentor).
  - Restaurado o estado de `pending_footnote_bodies` do contexto exterior após o flush.

### 3.3 `flush_pending_footnote_bodies` — bottom opcional

- `01_core/src/rules/layout/cursor.rs` — a função passou a aceitar `bottom_y: Option<f64>`, permitindo ao `columns.rs` posicionar footnotes de contentor abaixo do conteúdo, em vez de no fundo da página.

### 3.4 `layout_sub_frame_with_width` — altura de linha não-flushed

- `01_core/src/rules/layout/mod.rs` — corrigido cálculo de `cell_height` para incluir a altura da linha quando há itens em `current_line` não flushed. Sem esta correção, footnotes de uma única linha eram medidas com altura 0 e empilhadas sobrepostas.

### 3.5 Testes

- `01_core/src/rules/layout/tests.rs`:
  - Renomeado/actualizado `p537_footnotes_columns_colbreak_posicionam_por_coluna` → `p537_footnotes_columns_colbreak_empilham_na_primeira_coluna`.
  - Adicionado `p552_footnotes_set_page_columns_colbreak_posicionam_por_coluna`.
  - Adicionado `p552_footnote_counter_avanca_em_set_page_columns`.

---

## 4. Validação

### 4.1 Documentos de teste

Documento A (`#set page(columns: 2)`):

```typst
#set page(columns: 2)
#lorem(80)#footnote[Nota da primeira coluna.]
#colbreak()
#lorem(80)#footnote[Nota da segunda coluna.]
```

Documento B (`#columns(2)`):

```typst
#columns(2)[
  #lorem(80)#footnote[Nota da primeira coluna.]
  #colbreak()
  #lorem(80)#footnote[Nota da segunda coluna.]
]
```

### 4.2 Resultados

**Documento A:**

- Texto extraído: `[1]` na primeira coluna, `[2]` na segunda.
- Notas de rodapé lado a lado, numeradas `[1]` e `[2]`.
- Alinhado com vanilla 0.14.2/0.15.0.

**Documento B:**

- Texto extraído: `[1]` na primeira coluna, `[2]` na segunda.
- Notas de rodapé empilhadas na coluna esquerda.
- Alinhado com vanilla 0.14.2/0.15.0.

### 4.3 Comandos de validação

```bash
cargo test --workspace      # 572 passed; 1 flaky pré-existente (p307b_07_multi_feature)
crystalline-lint .          # 0 violations
```

O teste `p307b_07_multi_feature` continua flaky (ordem não-determinística no dicionário de bookmarks); não foi introduzido nem agravado por P552.

---

## 5. Ficheiros de verificação

- `/tmp/p551-footnote-cols.typ`, `/tmp/p551-footnote-cols2.typ`
- `/tmp/p552-a.pdf`, `/tmp/p552-a.png`
- `/tmp/p552-b.pdf`, `/tmp/p552-b.png`

Temporários, não commitados.

---

## 6. Conclusão

- **Corrigido:** numeração duplicada de footnotes em `#set page(columns: 2)`.
- **Corrigido:** posicionamento de footnotes em `#columns(2)` passa a empilhar na primeira coluna, como o vanilla.
- **Inventário actualizado:** itens de colunas de P551 marcados como corrigidos.
- **L0s actualizados:** `rules/columns.md`, `entities/elements/columns.md`, `passo-537b-set-page-columns.md`.
