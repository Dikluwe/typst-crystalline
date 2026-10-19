# Relatório — Passo 463: PDF `/GoTo` links internos

## Resumo

Conectou-se o segundo lado do par `label`/`ref` ao PDF: as referências
`ref(name)` / `@name` agora geram annotations `/Subtype /Link` com acção
`/S /GoTo /D /name`, navegando para os destinos nomeados `/Dests` criados por
`label(name, body)` (P460). A Trilha 2 — Referências cruzadas e navegação
interna — fica **completa**.

## Alterações

### 1. `01_core/src/entities/layout_types.rs`

- `FrameItem::Link` passou a usar `target: LinkTarget` em vez de `url: EcoString`.
- Novo enum `LinkTarget`:
  - `Url(EcoString)` — hyperlinks externos (P452);
  - `Destination(Label)` — destinos internos `/GoTo` (P463).

### 2. Refactor mecânico P452

Ficheiros actualizados para o novo campo `target`:
- `01_core/src/engine/layout/link.rs` — cria `LinkTarget::Url`.
- `01_core/src/engine/layout/cursor.rs` — propaga `target` nas duas funções de
translação.
- `01_core/src/engine/layout/helpers.rs` — propaga `target` em `translate_frame_item`.
- `01_core/src/engine/layout/slicing.rs` — propaga `target` em `rebase_item_y`.
- `01_core/src/engine/math/layout/mod.rs` — propaga `target` em `shift_item`.
- `01_core/src/engine/eval/tests.rs` — asserts actualizados para `LinkTarget::Url`.
- `03_infra/src/export/tests.rs` — teste P424 actualizado para `LinkTarget::Url`.

### 3. `01_core/src/engine/layout/references.rs`

- `layout_ref` resolve o texto (P462) e envolve os `FrameItem`s resultantes num
  `FrameItem::Link { target: LinkTarget::Destination(label), .. }`.
- Helper `resolve_ref_text` separado para clareza.
- Lida com flush do Layouter: `current_line` pode ter sido esvaziada durante o
  layout do texto; coleta robusta de `current_line` + `current_items`.

### 4. `03_infra/src/export/builder.rs`

- `collect_links` recolhe `(LinkTarget, Point, Size)`.
- `emit_link_annotations` gera:
  - `/A << /Type /Action /S /URI /URI (escaped_url) >>` para `LinkTarget::Url`;
  - `/A << /Type /Action /S /GoTo /D /name >>` para
    `LinkTarget::Destination(Label)`, reutilizando `escape_pdf_dest_name` (P460).
- Coordenadas `/Rect` continuam a converter Y-down → Y-up.

### 5. Tests

- `01_core/src/engine/layout/tests.rs::p462_ref_numeric`:
  - `ref_renders_as_frame_item_link_with_destination`
  - `external_link_still_uses_url_target`
- `03_infra/src/export/tests.rs`:
  - `pdf_ref_emite_goto_annotation`
  - `pdf_ref_unknown_label_emite_goto_without_valid_dest`
  - `pdf_external_link_still_emits_uri`

### 6. Specs L0

- `00_nucleo/prompts/entities/layout_types.md`
- `00_nucleo/prompts/engine/layout/link.md`
- `00_nucleo/prompts/engine/layout/ref.md`
- `00_nucleo/prompts/engine/layout_references.md`
- `00_nucleo/prompts/infra/export/builder.md`

## Verificação

```bash
cargo test --workspace -- --skip p350c
```

Resultado: **todos os testes passam** (typst-core 3255, typst-infra 496,
typst-wiring 21, crystalline_lint 2).

A excepção `p350c_flag_on_nao_convergente_classifica` (stack overflow) continua
pré-existente em HEAD.

## Scope-out / próximos passos

- Link styling (cor/sublinhado).
- Opção `link: false` para desactivar.
- `QuadPoints` para áreas clicáveis multi-linha.
- Escaping avançado de nomes de label inválidos.
- Trilha 2 marcada como COMPLETA; próximo passo sugerido: P464
  (consolidação `Content::Label` vs `Content::Labelled`) ou pivotar para outra
  trilha.
