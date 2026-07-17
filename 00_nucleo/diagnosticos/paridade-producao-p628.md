# Relatório — Passo 628 (P628)

**Data:** 2026-07-09  
**Commit base da sonda:** `0d76dbd22`  
**Foco:** Mapear o que pode ser unificado nos cinco caminhos de layout (fluxo principal, `grid.rs`, `placement.rs`, `columns.rs`, `boxed.rs`) e estimar o tamanho real de uma função partilhada.

---

## Resumo executivo

P579/P580, P625 e P626 confirmaram que correcções de layout aplicadas ao fluxo principal nem sempre chegavam aos sub-layouts (`grid`, `place`, `columns`, `box`). P593 começou a unificar a largura de palavra (`text_width`, `line_content_right`) com sucesso. P628 é a sonda obrigatória (ADR-0114) para decidir se o resto do núcleo de layout pode ser extraído numa função partilhada, sem escrever código.

Conclusão: **há um núcleo comum real**, já parcialmente materializado em `layout_sub_frame_with_width`, mas cada caminho tem especificidades que não devem ser absorvidas. A unificação total é **XL**; a abordagem segura é dividir em passos menores, começando pelos dois caminhos que já usam o helper existente (`grid.rs` e `placement.rs`).

---

## Sonda

### Comandos usados

```bash
# 1. Localizar os pontos de entrada de cada caminho e os helpers já partilhados
grep -n "fn layout\|fn measure\|text_width\|line_content_right\|align_current_line_rtl\|font_size_pt\|self.style.size" \
  01_core/src/engine/layout/mod.rs \
  01_core/src/engine/layout/cursor.rs \
  01_core/src/engine/layout/grid.rs \
  01_core/src/engine/layout/placement.rs \
  01_core/src/engine/layout/columns.rs \
  01_core/src/engine/layout/boxed.rs

# 2. Verificar quais helpers cada sub-layout consome directamente
grep -n "layout_word\|layout_chunk\|flush_line\|new_page\|text_width\|line_content_right\|align_current_line_rtl\|layout_sub_frame_with_width\|measure_content_constrained" \
  01_core/src/engine/layout/grid.rs \
  01_core/src/engine/layout/placement.rs \
  01_core/src/engine/layout/columns.rs \
  01_core/src/engine/layout/boxed.rs
```

### Tabela: mecanismos × caminhos

| Mecanismo | Fluxo principal | `grid.rs` | `placement.rs` | `columns.rs` | `boxed.rs` |
|---|---|---|---|---|---|
| Largura de palavra | Partilhado (`cursor.rs:word_width` → `text_width`) | Partilhado (via `layout_sub_frame_with_width`) | Partilhado (via `layout_sub_frame_with_width`) | Partilhado (via `layout_content` → `cursor.rs`) | Partilhado (via `layout_content` → `cursor.rs`) |
| Altura de linha | Partilhado (`cursor.rs:flush_line` → `vertical_metrics`) | Partilhado (via `layout_sub_frame_with_width`) | Partilhado (via `layout_sub_frame_with_width`) | Partilhado (via `layout_content` → `cursor.rs`) | Partilhado (via `layout_content` → `cursor.rs`) |
| Alinhamento RTL | Partilhado (`cursor.rs:align_current_line_rtl`) | Partilhado (`layout_sub_frame_with_width` com `align_rtl=true`) | Partilhado (`layout_sub_frame_with_width` com `align_rtl=false`, alinhamento feito depois pelo caller) | Partilhado (modo segmentado/fluido usa `layout_content`; RTL de colunas é ordem de preenchimento, não linha) | **Próprio** (`boxed.rs:112` chama `align_current_line_rtl` isoladamente sobre os itens do body na linha do pai) |
| Quebra de linha/página | Partilhado (`cursor.rs:flush_line` / `new_page`) | Partilhado (via `layout_sub_frame_with_width`) | Partilhado (via `layout_sub_frame_with_width`) | **Próprio** (decide quando passar à coluna seguinte vs. nova página) | Partilhado (via `layout_content` → `cursor.rs`) |
| Medição com largura máxima | Partilhado (`mod.rs:measure_content_constrained`) | **Usa** (`grid.rs` para `Auto` tracks) | Não usa | Partilhado (medição em `mod.rs`) | **Usa** (`boxed.rs:135` para decidir clip) |
| Ordem de colunas/direcção regional | N/A | N/A | N/A | **Próprio** (`columns.rs` inverte offsets em RTL; modo segmentado vs. fluido) | N/A |

Legenda:
- **Partilhado**: o caminho já consome um helper comum, sem replicar a lógica.
- **Usa**: o caminho chama um helper comum directamente, mas ainda pode ter lógica adicional à volta.
- **Próprio**: o caminho tem comportamento que não cabe no helper genérico.

### Núcleo já partilhado

O helper `layout_sub_frame_with_width` (`01_core/src/engine/layout/mod.rs:1546`) já encapsula o núcleo comum para sub-layouts:

- salva/restaura o estado do `Layouter`;
- inicializa um frame temporário com origem local;
- corre `layout_content`;
- faz flush da última linha e alinha RTL se pedido;
- devolve `(height, items)`.

`grid.rs` e `placement.rs` já usam este helper. O fluxo principal, `columns.rs` e `boxed.rs` não o usam porque têm restrições diferentes (ver abaixo).

---

## O que é genuinamente específico de cada caminho

### Fluxo principal
- Orquestração do documento: `Content::Space`, `Content::Parbreak`, `Content::Pagebreak`, `Content::Linebreak`, heading, sequence, etc.
- Gestão de estilo via `chain`/`TextStyle` e aplicação de `#set text`/`#set par`.
- Paginação física real (`new_page` com numeração, floats, footnotes).
- Não é um "sub-layout"; é o próprio contexto em que os outros correm.

### `grid.rs`
- Resolução de tracks (`Auto`, `Fixed`, `Fraction`).
- Algoritmo de colocação de células (`place_cells`, colspan/rowspan).
- Insets, fill, stroke, hlines/vlines por célula.
- Paginação entre linhas de grid.
- Overflow de células e `pending_cell_tails`.

### `placement.rs`
- Cálculo de posição absoluta a partir de `alignment`, `dx`, `dy`, `scope`.
- Não consome espaço no fluxo (`cursor_x`/`cursor_y` ficam intocados).
- Scope `Column` vs. `Parent` altera a área de ancoragem.

### `columns.rs`
- Divisão do body por `Content::Colbreak`.
- Modo segmentado vs. modo fluxo contínuo.
- Inversão da ordem de preenchimento das colunas em RTL.
- Coordenação de footnotes por coluna / acumuladas no contentor.
- Tradução horizontal dos items de cada coluna para offsets absolutos.

### `boxed.rs`
- Inline: avança `cursor_x`, não `cursor_y`.
- Outset/inset externo/interno, com shape fill/stroke e `radius`.
- `width`/`height` fixos com clip opcional.
- Alinhamento RTL isolado apenas sobre os itens produzidos pelo body dentro da `current_line` do pai.

---

## Interface esboçada da função partilhada

A função partilhada deve ser uma evolução controlada de `layout_sub_frame_with_width`, não um substituto total dos cinco caminhos. Esboço:

```rust
pub(super) fn layout_sub_frame(
    &mut self,
    content: &Content,
    region: SubLayoutRegion,
) -> (f64, Vec<FrameItem>)

pub(super) struct SubLayoutRegion {
    /// Origem x dentro do frame pai.
    pub origin_x: f64,
    /// Largura útil disponível para o conteúdo.
    pub width: f64,
    /// Altura útil disponível. `None` significa "sem limite" (decai
    /// VAlign::Bottom/Horizon para Top).
    pub height: Option<f64>,
    /// Se a última linha deve ser alinhada à direita quando o estilo for RTL.
    pub align_rtl: bool,
    /// Se a altura é ilimitada (afecta `Content::Align`).
    pub unconstrained_height: bool,
}
```

### Verificação contra os cinco casos

1. **Fluxo principal**: não é consumidor deste helper. O fluxo principal *define* o contexto (`page_config`, `regions`, etc.). Fora de scope.
2. **`grid.rs`**: encaixa perfeitamente. Células já usam `layout_sub_frame_with_width`; a nova versão adiciona `height: Some(cell_h)` e `unconstrained_height: false` sem alterar semântica.
3. **`placement.rs`**: encaixa. `place` precisa de altura ilimitada para alinhamento (`height: None`, `unconstrained_height: true`) e `align_rtl: false`, porque o posicionamento final é feito pelo caller.
4. **`columns.rs`**: **não encaixa directamente**. Modo segmentado precisa de salvar/restaurar `current_items` e de footnotes por coluna. Modo fluxo contínuo precisa de múltiplas colunas com `page_columns`. A função partilhada é demasiado simples para estes dois modos; `columns.rs` deve continuar a usar `layout_content` directamente.
5. **`boxed.rs`**: **encaixa parcialmente**. A medição e layout do body podem usar a função partilhada, mas o facto de ser inline e o alinhamento RTL isolado na linha do pai exigem uma variante ou um wrapper específico.

---

## Estimativa de tamanho honesta

| Escopo | Tamanho | Nota |
|---|---|---|
| Sonda (P628) | M | Concluída com este relatório. |
| Refactor de `layout_sub_frame_with_width` para `SubLayoutRegion` | S–M | Mudança mecânica; afecta `grid.rs` e `placement.rs`. |
| Aplicar a `boxed.rs` | M | Requer variante inline/isolamento de `current_line`. |
| Aplicar a `columns.rs` | L–XL | Provavelmente fora de scope da função partilhada; exigiria novo helper de nível superior. |
| Unificação total dos cinco caminhos | XL | Risco alto; quase certamente acaba por tocar em comportamentos específicos. |

---

## Proposta de divisão em passos menores

1. **P629 — Refactor do helper existente**  
   Transformar `layout_sub_frame_with_width` em `layout_sub_frame(SubLayoutRegion)`. Afecta `grid.rs` e `placement.rs`. Sem alteração de comportamento; só melhor estrutura e documentação.

2. **P630 — `boxed.rs` usa o helper partilhado**  
   Introduzir uma variante `layout_sub_frame_inline` (ou parâmetro `inline: true`) que faça o isolamento da `current_line` do pai e o alinhamento RTL isolado. Reduz a duplicação de save/restore de estado.

3. **P631 — Avaliação de `columns.rs`**  
   Decidir se merece um helper separado de nível superior (ex.: `layout_columns_region`) ou se fica fora da unificação. O modo segmentado e o modo fluxo contínuo são suficientemente distintos para não forçar encaixe na função genérica.

4. **P632+ — Unificação gradual de medição**  
   `measure_content_constrained` já é partilhado. Verificar se `grid.rs` e `boxed.rs` usam-na de forma consistente e se há mais braços que ainda replicam medição (ex.: shapes, stacks).

---

## Decisões e notas

- **Não se deve unificar os cinco caminhos numa só função.** O fluxo principal e `columns.rs` têm responsabilidades que não cabem no mesmo contrato.
- **O helper existente é o ponto de partida certo.** `grid.rs` e `placement.rs` já provam que o padrão funciona.
- **`boxed.rs` é o próximo candidato natural** porque é um sub-layout isolado, mas o facto de ser inline exige cuidado extra.
- **`columns.rs` deve ser tratado separadamente.** A lógica de colunas é sobre particionamento e ordenação regional, não sobre colocação de texto dentro de uma largura fixa.
- ADR-0108 aplicada: a tabela acima baseia-se em medições concretas (`file:line`) e não em suposição.

---

## Critérios de fecho do passo

- [x] Tabela completa: mecanismos × cinco caminhos, marcando partilhado/próprio.
- [x] Lista do que é genuinamente específico de cada caminho, a não unificar.
- [x] Interface da função partilhada esboçada e verificada contra os cinco casos.
- [x] Estimativa de tamanho revista com base na sonda.
- [x] Proposta de divisão em passos menores.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p628.md`, com hash do commit base.
