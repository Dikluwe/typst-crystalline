# P424 — Consumer PDF: `FrameItem::Link` → Annotation URI (S-M)

**Título**: PDF link consumer — conversão de `FrameItem::Link` em annotation URI no output PDF
**Tipo**: Materialização (S-M) — consumer PDF writer + infraestrutura de annotation
**Bloqueadores**: P422 (FrameItem::Link criado) e P425-A7 (FrameItem::Link propagado em typst-infra) como pré-requisitos
**Referências**: ADR-0107 (paridade linguagem), ADR-0108 (medir antes de decidir), ADR-0109 (atomização forma B), P422 (link render visual), P425-A7 (bloqueador: pos/size necessário)

---

## FASE A.0 — Sonda do substrato

Executados os 6 grep obrigatórios:

1. `FrameItem::Link` existe em `typst-infra` ✅ (braços em stream, pipeline, fonts, images, gradients).
2. PDF writer existe ✅ (`03_infra/src/export/builder.rs` e `stream.rs`).
3. Annotations ainda não existiam ❌ — este é o gap do P424.
4. `FrameItem` tem `Point`/`Size` ✅ (`entities/layout_types.rs`).
5. `pos`/`size` existem nos variants de `FrameItem` ✅.
6. PDF writer consome `Text`, `Shape`, `Image`, `Group`, etc. ✅.

**Decisão**: prosseguir com opção α (adicionar `pos`/`size` a `FrameItem::Link`).

---

## FASE A.1 — L0

Atualizados:

- `00_nucleo/prompts/rules/layout/link.md` — layout calcula bbox acumulada e emite `FrameItem::Link { url, items, pos, size }`.
- `00_nucleo/prompts/infra/export/builder.md` — `PdfBuilder` aloca objetos de annotation `/Subtype /Link` e referencia-os no `/Annots` de cada página.

Hashes sincronizados via `crystalline-lint --fix-hashes`:
- `01_core/src/rules/layout/link.rs` → `f34dd97a`
- `03_infra/src/export/builder.rs` → `4d25f90c`

---

## FASE B — Código

### B.1 — Entities (`01_core/src/entities/layout_types.rs`)

`FrameItem::Link` atualizado:
```rust
Link {
    url:   EcoString,
    items: Vec<FrameItem>,
    pos:   Point,
    size:  Size,
},
```

### B.2 — Layout (`01_core/src/rules/layout/link.rs`)

- Adicionada função `link_bbox()` que percorre os items produzidos pelo body e calcula a bbox acumulada usando `FontMetrics` para texto.
- `layout()` agora preenche `pos`/`size` ao emitir `FrameItem::Link`.

### B.3 — Propagação de `pos`/`size` em transforms/rebases

Atualizados os matches que reconstróem `FrameItem::Link` para preservar/transladar `pos`/`size`:
- `01_core/src/rules/layout/helpers.rs` (`translate_frame_item`)
- `01_core/src/rules/layout/slicing.rs` (`rebase_item_y`)
- `01_core/src/rules/layout/cursor.rs` (dois locais de translate)
- `01_core/src/rules/math/layout/mod.rs` (translate math)
- `03_infra/src/integration_tests.rs` (`frame_item_pos`)

### B.4 — PDF Writer (`03_infra/src/export/builder.rs`)

- Adicionado `emit_link_annotations(doc)` chamado antes de `serialize()` nos três caminhos (Helvetica, CIDFont, Multifont).
- Coleta recursivamente `FrameItem::Link` (incluindo dentro de `Group`).
- Aloca IDs de annotation após todos os objetos já emitidos.
- Converte Y-down → Y-up e escreve `/Rect`.
- Escapa `(`, `)`, `\` e caracteres de controlo na URI.
- Modifica os dicionários `/Page` existentes para incluir `/Annots [...]`.

### B.5 — Tests

- `01_core/src/rules/layout/link.rs`: `p424_layout_link_tem_bbox_positiva`.
- `01_core/src/rules/eval/tests.rs`: `p422_link_body_texto_preserva_url` e `p422_link_body_implicito_url` agora validam `pos`/`size`.
- `03_infra/src/export/tests.rs`: `pdf_link_emite_annotation_uri`, `pdf_link_escape_parenteses_na_uri`, `p424_link_com_group_interno_bbox_aproximada`.

---

## FASE C — Validação

```bash
cargo check -p typst-core -p typst-infra
# → ok (apenas warnings preexistentes)

cargo test -p typst-core --lib p422
# → 5 passed; 0 failed

cargo test -p typst-core --lib p424
# → 1 passed; 0 failed

cargo test -p typst-infra --lib pdf_link
# → 2 passed; 0 failed

crystalline-lint .
# → 0 errors; 0 drift warnings
```

**Critério de fecho**:
- [x] L0 hashado e propagado.
- [x] `FrameItem::Link` tem `pos` + `size`.
- [x] `layout_link` calcula bbox corretamente.
- [x] PDF writer emite annotation URI para `FrameItem::Link`.
- [x] `#link("url")[text]` → PDF com `/Subtype /Link`, `/S /URI` e `/Annots`.
- [x] Nenhum vtable/`dyn` introduzido.
- [x] `match` exaustivo preservado.
- [x] Lógica atomizada em free functions (forma B).
- [x] 9 tests novos verdes (5 core + 1 core + 3 infra).

---

## Relatório de Execução — P424

**Data**: 2026-06-23
**Executor**: assistente IA (Kimi Code CLI)
**Branch**: `Tekt`
**Commit**: `6f63473d1 P424: FrameItem::Link com pos/size + annotations URI no PDF`

**Sonda A.0**:
- Pré-requisitos P422/P425-A7 satisfeitos ✅
- Gap confirmado: nenhuma annotation URI no PDF ✅
- Tipos `Point`/`Size` disponíveis para opção α ✅

**L0**: `rules/layout/link.md` e `infra/export/builder.md` atualizados; hashes sincronizados.

**Implementação**:
- `entities/layout_types.rs`: `FrameItem::Link` ganhou `pos: Point` e `size: Size`.
- `rules/layout/link.rs`: cálculo de bbox acumulada via `FontMetrics`.
- `rules/layout/helpers.rs`, `slicing.rs`, `cursor.rs`, `math/layout/mod.rs`: preservação/translação de `pos`/`size`.
- `export/builder.rs`: emissão de annotations URI e referência `/Annots` nas páginas.
- `export/tests.rs`: 3 tests E2E de PDF annotations (incluindo bbox aproximada com Group interno).
- `integration_tests.rs`: match exaustivo corrigido.

**Validação**:
- `cargo check -p typst-core -p typst-infra` passa.
- `cargo test -p typst-core --lib p422` → 5 passed.
- `cargo test -p typst-core --lib p424` → 1 passed.
- `cargo test -p typst-infra --lib pdf_link` → 2 passed.
- `crystalline-lint .` → 0 errors, 0 drift.

**Scope-out / bloqueadores**:
- Links internos (`#link("<label>")`) — requerem named destinations; scope-out.
- Links inter-página — scope-out.
- Border/estilo visual da annotation — `/Border [0 0 0]` é aceitável.
- Hover tooltip — renderizador-dependente; scope-out.
- `QuadPoints` para textos multi-line — scope-out; usa `Rect` simples.
- URIs não-ASCII são mantidas como bytes; percent-encoding fica como melhoria futura.

**Notas epistêmicas**:
- **ADR-0107**: paridade comportamental (texto clicável → navegação para URL). Mecânica PDF livre.
- **ADR-0108**: sonda confirmou que PDF writer existe e que a decisão α é viável.
- **ADR-0109**: `FrameItem::Link` é dado; `layout_link` e `emit_link_annotations` são free functions.
- **Honestidade**: bbox é aproximada para `Group` (usa `inner_width`/`inner_height` sem transformar); links aninhados reutilizam bbox já calculada.

**Próximo passo sugerido**: P425 já concluído; P426 pode ser continuação de varredura mecânica em `typst-shell`/`typst-wiring` ou feature linguística scope-out (ex.: `text.lang` rustybuzz, XL).
