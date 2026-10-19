# Nota retroativa — P452 Links e Hyperlinks

**Data:** 2026-06-24  
**Passo:** 452  
**ADR-0114:** Aplica-se — reclassificação retroativa por sonda A.0.

---

## Reclassificação

O P452 foi originalmente classificado como materialização de hiperligações (`link(url, body)` e exportação de Link Annotation PDF). A sonda A.0 realizada em P453 revelou que **todo o substrato já existia** nos passos P422–P424:

- `Content::Link` e `LinkElem` — materializados em P422.
- `FrameItem::Link { url, items, pos, size }` — materializado em P422/P424.
- Layout de link (`engine/layout/link.rs`) — materializado em P422/P424.
- Export PDF de Link Annotation (`emit_link_annotations` em `export/builder.rs`) — materializado em P424.
- `native_link` e spec L0 de layout — materializados em P422/P424.

Portanto, o P452 é reclassificado de **S-M → XS (consolidação/verificação retroativa)**, não materialização.

---

## Reconciliação de forma

A spec P452 propunha uma forma alternativa para `FrameItem::Link`:

```rust
// Proposto na spec P452 (não implementado)
Link { url: EcoString, body: Frame }
```

A forma efectivamente materializada em P424 é:

```rust
// Implementação real em 01_core/src/entities/layout_types.rs
Link { url: EcoString, items: Vec<FrameItem>, pos: Point, size: Size }
```

A diferença é superficial: em vez de guardar um `Frame` completo, a implementação guarda a lista de `FrameItem`s do body já layoutados, mais a bounding box (`pos`/`size`). Esta forma é consistente com o resto do pipeline de layout e evita criar um `Frame` artificial para um elemento inline. A semântica é equivalente: os itens de `items` são os mesmos que estariam dentro de `body.items`, e a bbox total é `size` deslocada por `pos`.

---

## Conclusão

Nenhum código de produção foi alterado no P452. O único trabalho realizado foi:

- Verificação de que todos os critérios de fecho estavam satisfeitos.
- Actualização do spec L0 `00_nucleo/prompts/entities/layout_types.md` para reflectir `FrameItem::Link`.
- Actualização do hash do prompt em `01_core/src/entities/layout_types.rs`.
- Emissão do relatório de consolidação `typst-passo-452-relatorio.md`.
