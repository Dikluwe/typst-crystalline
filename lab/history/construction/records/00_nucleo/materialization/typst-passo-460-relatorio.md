# Relatório — Passo 460: `label<x>` — Destinos nomeados para referências cruzadas

## Resumo

Materializou-se a função nativa `label(name, body)` e o destino nomeado no
PDF, abrindo a **Trilha 2 — Referências cruzadas e navegação interna**.
`label` é um wrapper transparente: renderiza o `body` inalterado e regista a
página + posição para emissão de `/Dests` no catalogo PDF.

A implementação aproveita a infraestrutura de `Content::Labelled` (P329) e do
braço de referências em layout, separando claramente:

- **L1 (eval):** `native_label` produz `Content::Label { name, body }`.
- **L2 (layout):** `layout_label` delega a `layout_labelled`, registando a
  posição antes do layout e a página depois do layout.
- **L3 (export PDF):** o builder lê `PagedDocument.extracted_label_positions`
  e emite `/Names /Dests << /name [page_ref /XYZ x y null] >>`.

## Alterações

### 1. `01_core/src/entities/elements/label.rs` (novo)

- `LabelElem { name: EcoString, body: Content }`.
- Implementação transparente de `Element`: `plain_text`, `is_empty`,
  `map_content`, `map_text` delegam ao `body`.
- 4 testes L1 de identidade do elemento.
- Prompt L0 `00_nucleo/prompts/entities/elements/label.md` criado.

### 2. `01_core/src/entities/content.rs`

- Adicionada variante `Content::Label(Arc<LabelElem>)` (P460).
- Construtor `Content::label(name, body)`.
- Braços `map_content`, `map_text`, `plain_text`, `is_empty`, `repr`,
  `materialize_time` e walk do introspector actualizados para tratar `Label`.

### 3. `01_core/src/entities/layout_types.rs`

- `PagedDocument` ganhou `extracted_label_positions: HashMap<Label, Point>`.
- Actualizado o prompt L0 `00_nucleo/prompts/entities/layout_types.md` e o
  hash `@prompt-hash` no código.

### 4. `01_core/src/entities/layouter_runtime_state.rs`

- Adicionado `label_positions: HashMap<Label, Point>` ao runtime do layouter.

### 5. `01_core/src/engine/stdlib/label.rs` (novo)

- Extraído `native_label` de `structural.rs` para módulo próprio, conforme
  ADR-0117 (um prompt por ficheiro dono).
- Valida argumentos nomeados vazios e exige `name: string` + `body: content`.
- Prompt L0 `00_nucleo/prompts/engine/stdlib/label.md` criado.

### 6. `01_core/src/engine/stdlib/mod.rs`

- Declara `mod label;` e re-exporta `native_label`.
- Remove a implementação anteriormente inline em `structural.rs`.

### 7. `01_core/src/engine/layout/mod.rs`

- Braço `Content::Label(e)` chama `references::layout_label(self, &e.body, Label(...))`.
- Ao final do layout, copia `runtime.label_positions` para
  `doc.extracted_label_positions`.

### 8. `01_core/src/engine/layout/references.rs`

- `layout_labelled` passou a guardar também `label_positions` no runtime
  (já guardava `label_pages`).
- `layout_label` é o novo braço P460, simplesmente delegando a
  `layout_labelled`.

### 9. `01_core/src/engine/layout/tests.rs`

- `layout_label_renderiza_body_sem_alteracao_visual`
- `layout_label_registra_pagina_e_posicao`

### 10. `03_infra/src/export/builder.rs`

- `emit_named_destinations` adicionado: itera `PagedDocument.extracted_label_pages`
  e `extracted_label_positions`, converte Y-down interno para Y-up PDF
  (`page.height - pos.y`) e escreve `/Names /Dests << /name [3 0 R /XYZ x y null] >>`.
- Prompt L0 `00_nucleo/prompts/infra/export/builder.md` actualizado e hash
  `@prompt-hash` no código.

### 11. `03_infra/src/export/tests.rs`

- `pdf_label_emite_named_dests` — verifica `/Names`, `/Dests`, `/sec1`, `/XYZ`.
- `pdf_label_posicao_y_up_no_dests` — compara a coordenada Y do array `/XYZ`
  com `page.height - pos.y`.

## Verificação

```bash
cargo test --workspace
# Resultado: todos os crates passaram excepto um teste pré-existente:
#   rules::eval::tests::tests::p350c_flag_on_nao_convergente_classifica
#   stack overflow (já falhava no commit 6cf6e00ef, antes de P460).
# Total filtrado por label: 12 testes verdes em typst-core.
# Testes L3 de /Dests: 2 verdes.

crystalline-lint --fail-on error .
# Resultado: exit 0.
# Restam apenas warnings pré-existentes:
#   - V5 de deriva em eval/rules.rs (P461)
#   - V7 de prompts órfãos (adr-stub-vs-fallback, show-regex, font-dict)
```

## Notas

- O label é um **wrapper transparente**: não altera o layout visual do `body`.
- A posição do destino é capturada **antes** do layout do `body` (ponto de
  inserção), enquanto a página é registada **depois** do layout, evitando
  desvio quando o `body` força uma quebra de página.
- O sistema de coordenadas do PDF usa Y-up; o cristalino usa Y-down interno,
  logo o export faz `page.height - pos.y`.
- `Content::Label` (P460) e `Content::Labelled` (P329) coexistem: o primeiro
  é criado pelo utilizador via `#label<...>`, o segundo é o wrapper de
  referência interno usado por figures/equations.

## Scope-out explícito (conforme spec P460)

- `ref` / `@x` — P461.
- Links internos `/GoTo` — P462.
- Sintaxe sugar `#label<sec1>` (lexer/parser) — passo futuro.
- Label automático em headings/figures/equations/tables — passo futuro.
- Destinos de região (rectângulo) — apenas ponto `/XYZ`.
- Zoom em `/Dests` — `null` (default).

## Próximo passo

- **P461** — `ref<x>` / `@x`: resolução de destino + texto da referência.
- **P462** — PDF links internos `/GoTo`: conectar `ref` a `/Dests` via
  annotations `/GoTo`.
