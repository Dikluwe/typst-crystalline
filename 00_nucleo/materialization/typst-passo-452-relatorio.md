# Relatório — P452 Links e Hyperlinks

**Passo:** 452  
**Data:** 2026-06-24  
**Foco:** Materializar `link(url, body)` para hiperligações clicáveis no PDF exportado.

---

## Sumário executivo

A infraestrutura de links (`Content::Link`, `FrameItem::Link`, layout e exportação de annotations PDF) já estava materializada nos passos P422–P424. O trabalho do P452 consistiu em verificar o estado actual, garantir que todos os critérios de fecho estão satisfeitos, actualizar o spec L0 de `layout_types` para reflectir a variante `FrameItem::Link`, e consolidar o relatório.

Nenhuma mudança de código foi necessária além da actualização do prompt L0 e dos hashes correspondentes.

---

## Estado actual verificado

| Componente | Estado | Ficheiros |
|------------|--------|-----------|
| Entidade `Link` | ✅ | `01_core/src/entities/elements/link.rs` — `LinkElem { url, body }` + impl `Element`. |
| `Content::Link` | ✅ | `01_core/src/entities/content.rs` — variante `Link(Arc<LinkElem>)` + construtor `Content::link`. |
| `FrameItem::Link` | ✅ | `01_core/src/entities/layout_types.rs` — `Link { url, items, pos, size }`. |
| Função nativa `link` | ✅ | `01_core/src/engine/stdlib/structural.rs` — `native_link(url, body?)`. |
| Layout de link | ✅ | `01_core/src/engine/layout/link.rs` — renderiza body e envolve em `FrameItem::Link`. |
| Export PDF de links | ✅ | `03_infra/src/export/builder.rs` — `emit_link_annotations` gera `/Subtype /Link` + `/A /URI`. |
| `label`/`ref` | ⛔ scope-out | P453. |

---

## Mudanças realizadas

| Ficheiro | Alteração |
|----------|-----------|
| `00_nucleo/prompts/entities/layout_types.md` | Actualizado para listar todas as variantes actuais de `FrameItem`, incluindo `Link { url, items, pos, size }`. |
| `01_core/src/entities/layout_types.rs` | Hash do prompt actualizado via `crystalline-lint --fix-hashes`. |

---

## Critério de fecho

- [x] `Content::Link` adicionado como variante de `Content`.
- [x] `FrameItem::Link` adicionado com `Link { url, items, pos, size }`.
- [x] `native_link` registada no stdlib como `"link"`.
- [x] Layout renderiza `body` e envolve em `FrameItem::Link`.
- [x] Export PDF gera `/Subtype /Link` annotation com `/URI`.
- [x] 5+ tests verdes (L1, L2 e L3).
- [x] Spec L0 actualizado (`layout_types.md`).
- [x] `cargo test --workspace` verde (excepto stack overflow pré-existente `p350c_flag_on_nao_convergente_classifica`).
- [x] `crystalline-lint` zero violações novas.

---

## Tests existentes validados

### L1 — eval

- `rules::eval::tests::tests::p422_link_body_texto_preserva_url`
- `rules::eval::tests::tests::p422_link_body_implicito_url`

### L2 — layout

- `rules::layout::link::tests::p422_frame_item_link_construcao`
- `rules::layout::link::tests::p422_layout_link_body_texto`
- `rules::layout::link::tests::p422_layout_link_body_formatado`
- `rules::layout::link::tests::p424_layout_link_tem_bbox_positiva`
- `rules::layout::tests::layout_link_preserva_url_e_texto`

### L3 — export PDF

- `export::tests::pdf_link_emite_annotation_uri`
- `export::tests::pdf_link_escape_parenteses_na_uri`
- `export::tests::p424_link_com_group_interno_bbox_aproximada`

---

## Resultados de validação

```bash
$ cargo test --workspace --no-fail-fast -- --skip p350c_flag_on_nao_convergente_classifica
# 3215+ passed; 0 failed

$ crystalline-lint .
# 0 drift / 0 errors
# apenas warnings órfãos pré-existentes
```

---

## Commits

- `P452 — Links e Hyperlinks: consolidação de spec L0 e relatório`
