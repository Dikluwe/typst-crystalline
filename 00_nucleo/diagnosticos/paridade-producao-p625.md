# Relatório — Passo 625 (P625)

**Data:** 2026-07-09  
**Commit de implementação:** `94dea03df`  
**Foco:** Alinhamento RTL não se propaga para `box`, `columns`, `grid`, `placement`.

---

## Resumo executivo

`align_current_line_rtl()` (P576/P592) corrigia o alinhamento RTL no fluxo principal, mas os quatro sub-layouts (`box`, `grid`, `place`, `columns`) continuavam a alinhar à esquerda. P625 propaga o mecanismo de forma selectiva:

- `layout_sub_frame_with_width` aprende a forçar a largura do sub-frame e a opcionalmente alinhar RTL (`align_rtl: bool`).
- `boxed.rs` alinha o body RTL dentro da largura do box.
- `grid.rs` e `cursor.rs` (footnote) activam o alinhamento no sub-frame.
- `place.rs` / `placement.rs` (`#align` e `#place`) **não** alinham no sub-frame, para não quebrar o posicionamento do bloco — o alinhamento visual continua a ser garantido pela passagem bidi.
- A heurística de reflow bidi em `layout_bidi.rs` passa a usar o limite direito real do conteúdo (`content_right`), para não espalhar palavras pela largura da página em colunas/caixas.

---

## Sonda — onde `align_current_line_rtl` é chamado

```bash
grep -n "align_current_line_rtl()" 01_core/src/rules/layout/
```

Resultado antes da correcção:

| Ficheiro | Linha | Contexto |
|----------|-------|----------|
| `cursor.rs` | 238 | `flush_line()` |
| `mod.rs` | 1167 | `finish()` |

`layout_sub_frame_with_width`, `boxed.rs`, `grid.rs`, `placement.rs`, `columns.rs` **não** chamavam a função — confirmado o padrão P579/P580.

---

## Implementação

### Ficheiros alterados

- `01_core/src/rules/layout/mod.rs` — `layout_sub_frame_with_width`:
  - guarda/restaura `width` e `height`;
  - configura `width = cell_x + cell_width + margin` durante o sub-layout para que `align_current_line_rtl` e `flush_line` usem o limite direito do sub-frame;
  - configura `height = 1_000_000_000.0` para evitar quebras de página dentro do sub-frame (regressão descoberta em `p595_nota_enorme_colunas_gera_warning`);
  - novo parâmetro `align_rtl: bool` controla se `align_current_line_rtl()` é chamado.

- `01_core/src/rules/layout/boxed.rs` — após `layout_content(body)`, isola os itens do body na linha actual, alinha-os com `align_current_line_rtl()` e move-os para `current_items`.

- `01_core/src/rules/layout/grid.rs` — passa `align_rtl: true` nas duas chamadas a `layout_sub_frame_with_width`.

- `01_core/src/rules/layout/cursor.rs` — passa `align_rtl: true` no layout do body de footnote.

- `01_core/src/rules/layout/place.rs` e `placement.rs` — passam `align_rtl: false`; o bloco posicionado pelo caller não deve ser puxado para a margem direita.

- `03_infra/src/layout_bidi.rs` — `same_paragraph` e `try_fuse_paragraph` usam `metrics.line_content_right(...)` em vez de `page.width`, para que o reflow RTL respeite a largura real de sub-layouts. O teste `p565_reflow_merges_rtl_lines_when_fits` foi ajustado para posicionar o limite direito da linha no `content_right` real (anteriormente usava a largura da página como proxy).

---

## Validação

### Testes automatizados

```bash
cargo test --workspace
```

Resultado: **todos passam** (605 tests, 0 falhas, 5 ignorados).

```bash
crystalline-lint .
```

Resultado: **No violations found**.

### Casos visuais RTL

| Caso | Comportamento observado | Divergência do vanilla |
|------|--------------------------|------------------------|
| `#box(width: 150pt)` com texto árabe | Palavras alinhadas à direita do box, uma por linha. | Posição absoluta do box ligeiramente deslocada (problema de origem do box inline, não do alinhamento). |
| `#grid(columns: 1)` com texto árabe | Três palavras em linha única, ordem visual RTL correcta. | Nenhuma significativa. |
| `#place(top + left)` com texto árabe | Bloco no canto superior-esquerdo, texto RTL na ordem visual correcta. | Nenhuma significativa. |
| `#set page(columns: 2)` com texto árabe | Palavras agrupadas numa coluna (duas linhas), alinhadas à direita dentro da coluna. | Direcção de preenchimento das colunas em RTL (vanilla começa pela coluna da direita; cristalino pela esquerda). A estrutura de linha dentro da coluna está corrigida. |

### Casos LTR de regresso

Foram testados os mesmos quatro documentos com texto latino (`The book on the table`). Não houve alteração de comportamento em relação ao estado pré-P625; os documentos LTR continuam alinhados à esquerda.

---

## Decisões e notas

- **Por que `height` ilimitado no sub-frame?** A correção da largura fez a nota de rodapé gigante medir altura maior; contudo, a altura da página física fazia `flush_line` emitir uma nova página dentro do sub-frame, perdendo as linhas anteriores. Sub-frames são conceptualmente caixas sem quebra de página, pelo que `height` foi elevado para evitar `new_page` automático.
- **Por que `align_rtl: false` em `place`/`align`?** Se o sub-frame alinhar RTL à largura útil da página, o bloco inteiro é deslocado para a direita, quebrando `#place(top + left)`. O alinhamento visual RTL destes casos é garantido pela passagem bidi posterior sobre o documento completo.
- **Padrão documentado:** mecanismos corrigidos no fluxo principal precisam agora de uma checklist explícita de propagação aos quatro sub-layouts (`box`, `columns`, `grid`, `place`), para evitar que o mesmo bug reapareça.

---

## Critérios de fecho do passo

- [x] Sonda completa, alcance confirmado nos quatro casos.
- [x] Correcção aplicada e testada em todos os sub-layouts afectados.
- [x] Fluxo principal de RTL sem regressão.
- [x] Sem regressão em `cargo test --workspace`.
- [x] `crystalline-lint .` limpo.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p625.md`, com hash do commit.
- [x] Nota adicionada ao padrão P579/P580/P625 sobre checklist de propagação aos sub-layouts.
