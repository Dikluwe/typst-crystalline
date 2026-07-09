# Prompt L0 — `rules/columns` — Layout de colunas multi-página

Hash do Código: 5f91e192

**Camada**: L1  
**Ficheiro alvo**: `01_core/src/rules/layout/columns.rs`  
**Ficheiros adjacentes**: `01_core/src/rules/layout/cursor.rs`, `01_core/src/rules/layout/mod.rs`, `01_core/src/rules/layout/footnote.rs`, `01_core/src/entities/elements/columns.rs`  
**Origem**: P537 (colunas reais), P537b (`#set page(columns:)`), P552 (correcção de footnotes), P553 (correcção geométrica de largura de coluna)  
**ADRs**: ADR-0107 (paridade linguagem), ADR-0108 (medir antes de decidir), ADR-0109 (atomização forma B), ADR-0054 (graded / scope-outs).

---

## 1. Contexto

O consumer `Content::Columns` serve dois contextos sintacticamente distintos:

1. **`#columns(N)[body]`** — contentor de colunas no fluxo regular.
2. **`#set page(columns: N)`** — configuração de página que, após transformação AST (`wrap_page_columns`), produz um `Content::Columns` sintético envolvendo o resto do escopo.

A implementação base (P537) introduziu colunas reais para ambos os contextos, mas tratava as notas de rodapé de forma uniforme. Medições de P551/P552 mostram que o vanilla distingue os dois contextos para footnotes. O L0 corrigiu essa distinção em P552.

A medição P553 identificou um segundo problema: as colunas são renderizadas com largura efetiva muito menor do que no vanilla, porque o cálculo de `column_width` parte da **largura total** da página em vez da **largura útil**. Este L0 corrige essa geometria.

---

## 2. Medições que sustentam a decisão

### 2.1 Footnotes — P552

- Em `layout_segmented` (`01_core/src/rules/layout/columns.rs`), `footnote_counter` era salvo e restaurado por segmento. Combinado com `flush_pending_footnote_bodies()` ao fim de cada coluna, isso fazia com que o contador não avance e cada coluna reutilizasse o mesmo número.
- P551 — imagens de `#set page(columns: 2)` mostram cristalino com notas lado a lado mas numeradas ambas como `[1]`; vanilla 0.14.2/0.15.0 numera `1`, `2` e também coloca lado a lado.
- P552 — testes adicionais com vanilla:
  - `#columns(2)[...]` com notas em ambas as colunas: vanilla empilha as notas na coluna esquerda.
  - `#columns(2)[#colbreak() #lorem(80)#footnote[...]]`: vanilla coloca a nota na coluna esquerda, não na direita.
  - `#set page(columns: 2)` com notas em ambas as colunas: vanilla coloca cada nota no fundo da respectiva coluna.

### 2.2 Geometria de colunas — P553

- Em `columns::layout` (`01_core/src/rules/layout/columns.rs`), a variável `full_width` era atribuída a `layouter.regions.current.width`, usando a largura total da página (`595.28 pt` em A4) e produzindo `column_width = (595.28 - 23.81) / 2 = 285.73 pt`.
- O layout interno de cada coluna (`layout_word` / `layout_chunk` em `cursor.rs`) impõe `right_margin = regions.current.width - page_config.margin`, e o cursor começa em `page_config.margin` (`70.87 pt` em A4). Isso reduz a área útil efetiva da coluna para `285.73 - 2 × 70.87 ≈ 144 pt`.
- P553 — para `#set page(columns: 2)\n#lorem(1200)`:
  - Cristalino: **5 páginas**; primeira linha da coluna esquerda ocupa ~124 pt.
  - Vanilla 0.14.2/0.15.0: **2 páginas**; primeira linha da coluna esquerda ocupa ~212 pt.
  - Vanilla 0.14.2/0.15.0 com `#set text(font: "Liberation Sans")`: **3 páginas**.
- Interpretação: ~2 das 3 páginas extra do cristalino são causadas pela geometria errada; a diferença restante (3 vs 2) é da fonte padrão sans-serif do cristalino vs serif do vanilla.

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

### 3.4 Geometria de colunas

As colunas devem ocupar a **largura útil** da página, definida como `page_width - 2 × margin`. A largura de cada coluna é calculada a partir dessa largura útil, com gutter default ainda proporcional à **largura total** da página (para paridade com a medição vanilla).

Formalmente, para `count` colunas:

```text
usable_width = page_width - 2 × margin
gutter       = page_width × COLUMNS_DEFAULT_GUTTER_RATIO   (ou valor explícito)
column_width = (usable_width - (count - 1) × gutter) / count
x_i          = margin + i × (column_width + gutter)         (início absoluto da coluna i)
```

Durante o layout de cada coluna, o sistema vê uma "mini-página" de largura `column_width + 2 × margin`, com cursor iniciando em `margin`. Isso faz com que o `right_margin` interno (`width - margin`) termine exactamente no fim da área útil da coluna (`x_i + column_width`), preenchendo toda a largura disponível.

A translação horizontal dos items da mini-página para a posição absoluta na página mantém-se: `dx = x_i - margin`.

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

Em `01_core/src/entities/content.rs`, o `Content::Columns` produzido a partir de `Content::SetPage { columns: Some(n), .. }` deve usar `page_columns: true`.

### 4.3.1 `wrap_page_columns` — separar por `#pagebreak()` (P627)

O body de um `Content::Columns` sintético pode conter `#pagebreak()` no seu interior (ex.: documento bilingue com um único `#set page(columns:)` no topo e mudança de `text.dir` apenas por `#pagebreak()`). Nesse caso, cada secção de página deve ser o seu próprio `ColumnsElem`, para que a direcção de preenchimento (P626) seja avaliada por secção e não uma única vez para o grupo inteiro.

Implementação:

1. Após determinar o body implícito `parts[i + 1..j]` (já delimitado por `SetPage`/`Pagebreak` ao nível da sequência exterior), examinar o body:
   - Se for uma `Sequence`, iterar pelos seus filhos.
   - Cada vez que se encontrar um `Content::Pagebreak`, terminar o `ColumnsElem` actual, emitir o próprio `Pagebreak` (preservando `weak`/`to`), e iniciar um novo `ColumnsElem` para os filhos seguintes.
   - Filhos que não são `Pagebreak` acumulam no segmento actual.
2. Se o body não for uma `Sequence`, criar um único `ColumnsElem` (comportamento anterior).
3. Segmentos vazios não produzem `ColumnsElem`.
4. Cada `ColumnsElem` resultante mantém `count`, `gutter: None` e `page_columns: true`.

Esta alteração é local a `wrap_page_columns` e não muda a semântica dos `Pagebreak` (continuam a forçar nova página); apenas garante que a direcção de preenchimento é decidida por página.

### 4.4 `columns::layout` — cálculo a partir da largura útil

Em `01_core/src/rules/layout/columns.rs`:

1. Guardar `page_width = layouter.regions.current.width` e `margin = layouter.page_config.margin`.
2. Calcular `usable_width = page_width - 2.0 * margin`.
3. Resolver `gutter_pt` (default `page_width * COLUMNS_DEFAULT_GUTTER_RATIO`).
4. Calcular `column_width = (usable_width - (count_f - 1.0) * gutter_pt) / count_f`.
5. Calcular `column_x_offsets[i] = margin + i as f64 * (column_width + gutter_pt)`.

### 4.5 `columns::layout_segmented` — região de coluna

- Configurar `regions.current.width = column_width + 2.0 * margin` (em vez de `column_width`).
- Manter `cursor_x = margin` e `line_start_x = margin`.
- Manter translação `dx = column_x_offsets[idx] - margin`.
- Manter semântica de footnotes de P552.

### 4.6 `columns::layout_flow` e `start_column` — região de coluna

- Configurar `regions.current.width = column_width + 2.0 * margin`.
- Manter `cursor_x = margin` e `line_start_x = margin`.
- Manter translação `dx = column_x_offsets[idx] - margin` em `close_current_column`.

### 4.7 `columns::layout_segmented` — correcção de contador e posicionamento

Em `01_core/src/rules/layout/columns.rs`:

1. **Remover save/restore de `footnote_counter`** em `layout_segmented`. O contador avança naturalmente.
2. Se `e.page_columns` for `true`:
   - Manter flush de `pending_footnote_bodies` no fim de cada coluna (`layout_segmented` actual).
   - As notas são transladadas juntamente com os items da coluna.
3. Se `e.page_columns` for `false`:
   - **Não flushar** `pending_footnote_bodies` no fim de cada coluna.
   - Acumular os bodies de footnote num buffer local ao `layout_segmented`.
   - No final do loop, após recolher todos os items das colunas, posicionar as notas no fundo da primeira coluna (x = `column_x_offsets[0]`, y = altura máxima consumida pelas colunas ou `height - margin`, conforme já usado pelo flush).
   - Adicionar os items das notas a `all_column_items` (ou a `current_items` directamente) já transladados para a coluna esquerda.

### 4.8 `columns::layout_flow` — sem alterações de footnotes

O modo fluxo contínuo (sem `colbreak()`) é usado por `#columns(N)[body]` quando não há colbreaks suficientes. Neste modo, o flush de notas já acontece no fim de cada coluna preenchida pelo `Layouter`. Para `#columns()` isso ainda difere do vanilla (que acumula até ao fim do contentor), mas o caso sem `colbreak()` é secundário para P552; pode ser tratado na mesma alteração se o mecanismo de `page_columns` permitir.

**Scope-out consciente**: se a alteração de `layout_flow` para empilhar notas exigir refacção profunda do `Layouter`, fica para passo seguinte. O critério mínimo de P552 é corrigir `layout_segmented` (caso com `colbreak()`), que é o observado nas imagens.

### 4.9 `flush_pending_footnote_bodies` — bottom opcional

- `01_core/src/rules/layout/cursor.rs` — a função passa a aceitar `bottom_y: Option<f64>`, permitindo ao `columns.rs` posicionar footnotes de contentor abaixo do conteúdo, em vez de no fundo da página.

### 4.10 `layout_sub_frame_with_width` — altura de linha não-flushed

- `01_core/src/rules/layout/mod.rs` — corrigido cálculo de `cell_height` para incluir a altura da linha quando há itens em `current_line` não flushed. Sem esta correção, footnotes de uma única linha eram medidas com altura 0 e empilhadas sobrepostas.

### 4.11 Testes

- Teste do contador: duas notas em `#set page(columns: 2)` produzem `[1]` e `[2]`.
- Teste do contador: três notas em `#set page(columns: 2)` produzem `[1]`, `[2]`, `[3]`.
- Teste de `#columns(2)[...]` com duas notas: notas empilhadas na coluna esquerda.
- Regressão: `#columns(2)[...]` sem notas continua a funcionar.
- Regressão: `#set page(columns: 2)` continua com notas lado a lado.
- Regressão P553: `#set page(columns: 2)` com `#lorem(1200)` reduz de 5 para 3 páginas (fonte actual), mantendo 1200 palavras.

---

## 5.1. Direcção de preenchimento em RTL — P626

### 5.1.1. Medições

- P625 — `#set page(columns: 2)` com texto árabe (`#set text(lang: "ar", dir: rtl)`) preenche a coluna **esquerda** primeiro no cristalino, enquanto o vanilla 0.15.0 preenche a coluna **direita** primeiro.
- Sonda P626 com `mutool trace` no vanilla 0.15.0:
  - 2 colunas: primeiro `fill_text` aparece em `x ≈ 344` (coluna da direita).
  - 3 colunas: primeiro `fill_text` aparece em `x ≈ 416` (coluna mais à direita).
- Interpretação: a ordem de preenchimento segue a direcção de leitura do texto. Em RTL a "primeira" coluna é a mais à direita; o layout deve avançar da direita para a esquerda.

### 5.1.2. Decisão

- `columns::layout` determina a direcção efectiva do body consultando o canal `"text.dir"` nos estilos que o envolvem.
- Como o `Content::Columns` sintético de `#set page(columns:)` é criado num contexto onde o `text.dir` viaja no `Content::Styled` do body (e não na chain activa do `Layouter` no momento em que `columns::layout` é invocado), a direcção é lida percorrendo o body do elemento: o primeiro `Content::Styled` (incluindo recursão por `Sequence`) que defina `"text.dir"` vence.
- Quando a direcção for `Dir::RTL`, o vector `column_x_offsets` é invertido antes de ser usado em `layout_segmented`/`layout_flow`. Isto faz com que o índice 0 corresponda à coluna mais à direita e a progressão `start_next_column` avance para a esquerda.
- Documentos LTR mantêm a ordem original (esquerda para a direita).
- Documentos mistos (secções com `#set page(columns:)` e `text.dir` diferentes) funcionam porque cada `ColumnsElem` lê a direcção do seu próprio body.

---

## 5.2. Direcção de preenchimento por página em documentos bilingues — P627

### 5.2.1. Medições

- Documento bilingue com um único `#set page(columns: 2)` no topo:
  ```typst
  #set page(columns: 2)
  #set text(lang: "en", dir: ltr, size: 16pt)
  #lorem(150)
  #pagebreak()
  #set text(lang: "ar", dir: rtl, size: 16pt)
  #lorem(150)
  ```
- Cristalino P626: página 1 LTR começa à esquerda (`x ≈ 70.9`); página 2 RTL começa também à esquerda (`x ≈ 108`), herdando a direcção da primeira página.
- Vanilla 0.15.0: página 1 LTR começa à esquerda (`x ≈ 70.9`); página 2 RTL começa à direita (`x ≈ 343`).
- Causa: `wrap_page_columns` cria um único `ColumnsElem` cujo body inclui ambas as secções; `body_dir` encontra o primeiro `text.dir` (LTR) e aplica-o a todas as páginas do grupo.

### 5.2.2. Decisão

- `wrap_page_columns` separa o body de um `ColumnsElem` sintético nos `Content::Pagebreak` que aparecem ao nível da `Sequence` do body.
- Cada segmento entre `Pagebreak`s torna-se um `ColumnsElem` independente, com o seu próprio `body_dir`.
- Os `Pagebreak` originais são preservados entre os novos `ColumnsElem`, mantendo a semântica de quebra de página.
- Com esta separação, um documento bilingue com LTR → `#pagebreak()` → RTL produz dois `ColumnsElem`: um LTR (preenche da esquerda) e outro RTL (preenche da direita).

---

## 5. Restrições

- Não se remove o `match` exaustivo em `layout_content`.
- Não se introduz despacho dinâmico (`dyn`/vtable/PropMap).
- A lógica permanece em L1 (sem I/O, sem estado global).
- `#set page(columns: N)` a meio do documento continua scope-out per ADR-0054.
- Gutter personalizado via `#set page(gutter: ...)` continua scope-out.
- Não se altera `layout_word` / `layout_chunk` directamente; a geometria é corrigida ajustando a largura da região de coluna em `columns.rs`.

---

## 6. Critérios de verificação

- `#set page(columns: 2)` com duas notas: `[1]` na primeira coluna, `[2]` na segunda; notas no fundo de cada coluna.
- `#set page(columns: 2)` com três notas: `[1]`, `[2]`, `[3]` correctos.
- `#columns(2)[...]` com duas notas: notas empilhadas na coluna esquerda.
- **P626** — `#set page(columns: 2)` + `#set text(lang: "ar", dir: rtl)` + `#lorem(200)`: primeira coluna preenchida é a da direita.
- **P626** — `#set page(columns: 3)` + `#set text(lang: "ar", dir: rtl)` + `#lorem(300)`: ordem de preenchimento é direita → meio → esquerda.
- **P626** — Documento LTR (`dir: ltr` implícito) com 2 colunas continua a encher da esquerda para a direita.
- **P626** — Documento misto: secção LTR seguida de secção RTL com `#set page(columns:)` distintos; cada secção preenche na direcção correcta.
- **P627** — Documento bilingue com um único `#set page(columns: 2)` e mudança de `text.dir` por `#pagebreak()`: página LTR preenche da esquerda; página RTL preenche da direita.
- **P627** — Documento com múltiplas transições (LTR → RTL → LTR) usando `#pagebreak()` dentro do mesmo grupo de colunas: cada página preenche na direcção correcta.
- **P627** — Testes de P626 (`p626_set_page_columns_rtl_preenche_direita_primeiro`) continuam a passar sem alteração.
- `cargo test --workspace` limpo.
- `crystalline-lint .` limpo.
- P553: `#set page(columns: 2)\n#lorem(1200)` produz 3 páginas no cristalino (com a fonte padrão actual); vanilla com `Liberation Sans` também produz 3 páginas.
