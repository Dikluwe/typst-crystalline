# Relatório de Produção — P632

**Passo:** 632  
**Data:** 2026-07-09  
**Foco:** Avaliação de `columns.rs` para a sequência de unificação de helpers de sub-layout.  
**Commit de referência (medidas e alterações):** `d0077b35e`

---

## 1. Nota rápida resolvida: `SubLayoutRegion` vs `layout_sub_frame_inline`

`SubLayoutRegion` possui campos `origin_x` e `width` que a variante inline ignorava sem aviso. Foi adicionada uma `debug_assert!` em `01_core/src/engine/layout/sub_frame.rs:155-159` que falha em debug se `origin_x` ou `width` forem diferentes de `0.0` quando `layout_sub_frame_inline` é chamada.

A variante inline opera sobre a linha horizontal do pai; se o caller precisar de uma região geométrica própria, deve usar `layout_sub_frame`.

---

## 2. Sonda principal — `columns.rs`

### 2.1 Mapeamento das responsabilidades (com `file:line`)

| # | Responsabilidade | Local | Classificação |
|---|---|---|---|
| 1 | Divisão do body pelos `Content::Colbreak` | `01_core/src/engine/layout/columns.rs:31-64` (`split_by_colbreak`) | **Estrutura de colunas** |
| 2 | Escolha entre modo segmentado (`colbreak` presente) e modo fluxo contínuo | `columns.rs:182-186` (`layout`) | **Estrutura de colunas** |
| 3 | Inversão da ordem de preenchimento em RTL | `columns.rs:66-97` (`body_dir`), `columns.rs:175-180` (inversão de `column_x_offsets`) | **Estrutura de colunas** |
| 4 | Divisão por página em documentos LTR/RTL mistos | Partição em `01_core/src/entities/content.rs:2305-2321` (`wrap_page_columns` / `page_column_segments`); direcção consumida em `columns.rs:175` (`body_dir`) | **Estrutura de colunas** |
| 5 | Coordenação de notas de rodapé por coluna vs. empilhadas | `columns.rs:209-287` (`layout_segmented`: save/restore de `pending_footnote_bodies`, flush por coluna ou acumulado na primeira coluna) | **Estrutura de colunas** |
| 6 | Translação horizontal dos items de cada coluna para offsets absolutos | `columns.rs:258-263`, `columns.rs:276-279`; helper `translate_item_x` em `columns.rs:351-357` | **Estrutura de colunas** (consequência do sistema de coordenadas mini-página) |

### 2.2 Uso actual de `layout_sub_frame` / `layout_sub_frame_inline`

Nenhuma das seis responsabilidades invoca `layout_sub_frame` ou `layout_sub_frame_inline`. O layout do conteúdo *dentro* de cada coluna é feito directamente por `layouter.layout_content(segment)` (`columns.rs:235` e `columns.rs:327`), depois de configurar a região da coluna.

A única reutilização de helper genérico é `translate_item_x`, que já delega para funções partilhadas em `engine/layout/helpers`.

### 2.3 Porque não unificar?

- **`split_by_colbreak`, modo segmentado/fluxo, RTL e divisão por página** são decisões de *topologia* de colunas; não têm contrapartida no helper de sub-layout.
- **Coordenação de footnotes** é semântica específica de colunas (`page_columns: true` vs `false`) e depende do estado `column_mode` / `column_width` do `Layouter`; o helper genérico não tem conhecimento deste estado.
- **Translação horizontal** é uma consequência do sistema de coordenadas mini-página (`cursor_x = margin`, depois `dx = x_i - margin`). Embora seja “colocação”, está ligada à geometria de coluna e não pode ser absorvida por `layout_sub_frame_inline` (que não opera em região geométrica) nem por `layout_sub_frame` sem alterar a semântica de quebras de página e de footnotes.
- O *save/restore* manual em `layout_segmented` (`columns.rs:215-255`) parece superficialmente similar ao de `layout_sub_frame`, mas o conjunto de campos guardados/restaurados e a activação de `column_mode` tornam-no uma variante especializada, não uma duplicação genérica.

### 2.4 Decisão final

`columns.rs` **fica completamente fora da unificação**. Não foi criado nem alterado nenhum helper de nível superior para colunas, nem foi forçada a reutilização de `layout_sub_frame`. O arquivo já está atomizado (forma B da ADR-0109) e as suas responsabilidades são genuinamente distintas das dos helpers genéricos.

---

## 3. Validação

Foram executados no commit `d0077b35e`:

```text
cargo test -p typst-core columns       → 44 passed, 0 failed
cargo test -p typst-core p552          → 2 passed, 0 failed
cargo test -p typst-core p553          → 0 matched (nenhum teste nomeado com p553)
cargo test -p typst-core p595          → 3 passed, 0 failed
cargo test -p typst-core p626          → 1 passed, 0 failed
cargo test -p typst-core p627          → 1 passed, 0 failed
cargo test --workspace                 → all passed
crystalline-lint .                     → No violations found
```

Não houve alterações de snapshot nem regressões nos passos anteriores de colunas.

---

## 4. Estado da sequência de unificação de layout

| Passo | Caminho | Estado |
|---|---|---|
| P593 | Largura de texto (todos) | Fechado |
| P625 | Alinhamento RTL (grid, box, place, footnote) | Fechado |
| P626/P627 | Ordem/direcção de colunas | Fechado |
| P629 | Helper base (`sub_frame.rs`, `SubLayoutRegion`) | Fechado |
| P631 | `boxed.rs` migrado | Fechado |
| **P632** | `columns.rs` avaliado | **Fechado — fica fora da unificação** |

A sequência de unificação de layout está assim completa e explicitamente delimitada: `columns.rs` permanece como arquivo autónomo por ter responsabilidades estruturais próprias.
