> **Passo:** 464 — Consolidação: `Content::Label` vs `Content::Labelled`  
> **Data:** 2026-06-25  
> **Executor:** assistente IA (Kimi Code CLI)  
> **Ficheiro de instrução:** `00_nucleo/materialization/typst-passo-464.md`

---

## Resumo

Unificados os dois tipos semânticos de label (`Content::Label` de P460 e
`Content::Labelled` de P329) num único `Content::Label`, adicionando o campo
`auto: bool` para distinguir a origem (user-created vs auto-generated). Todos os
consumidores (`eval`, `layout`, `introspect`, `export`) foram migrados e as
specs L0 actualizadas.

---

## Implementação

### Modelo

Escolheu-se manter `Content::Label` como tipo único (nome alinhado com Typst
vanilla) e introduzir `LabelElem.auto: bool` em vez de dois variants/structs.
A distinção é de origem, não de tipo, o que simplifica todos os consumers.

- `auto: false` — `#label("nome", body)` (user-created).
- `auto: true` — `<label>` em headings/figures/equations (auto-generated).

### Ficheiros alterados

| Ficheiro | Alteração |
|----------|-----------|
| `01_core/src/entities/elements/label.rs` | `LabelElem` ganha campo `auto: bool`; `map_content`/`map_text` preservam `auto`; testes actualizados; teste L1 novo. |
| `01_core/src/entities/elements/labelled.rs` | **Removido** (ficheiro órfão). |
| `01_core/src/entities/elements/mod.rs` | Removido `pub mod labelled;`. |
| `01_core/src/entities/content.rs` | Removido `LabelledElem`/`Content::Labelled`; adicionado `Content::label_auto`; ajustados `is_empty`, `plain_text`, `PartialEq`, `map_content`, `map_text`. |
| `01_core/src/rules/eval/mod.rs` | Emissão de `<label>` passa a usar `Content::label_auto`. |
| `01_core/src/rules/eval/repr.rs` | Removido braço `Content::Labelled`; `Content::Label` com `auto: true` representa-se sem body. |
| `01_core/src/rules/eval/tests.rs` | Ajustados matches e acesso a campos; teste `eval_label_anexa_ao_bloco_anterior` verifica `Content::Label { auto: true }`. |
| `01_core/src/rules/introspect.rs` | Unificados braços `Content::Label` (user) e `Content::Labelled` (auto) num só braço `Content::Label` com lógica condicional a `auto`; ajustados helpers e todos os tests legados; adicionado teste L2. |
| `01_core/src/rules/introspect/locatable.rs` | Ajustados tests para `Content::label_auto`. |
| `01_core/src/rules/introspect/fixpoint.rs` | Ajustados tests para `Content::label_auto`. |
| `01_core/src/rules/layout/mod.rs` | Removido braço `Content::Labelled`; layout de `Content::Label` único. |
| `01_core/src/rules/layout/references.rs` | Simplificação: `layout_label` regista destino directamente; `layout_labelled` removido. |
| `01_core/src/rules/layout/tests.rs` | Ajustados todos os `Content::labelled`/`labelled_prod` para `Content::label_auto`. |
| `01_core/src/entities/resolved_label_store.rs` | Comentário actualizado para referenciar `Content::Label` com `auto: true`. |
| `03_infra/src/export/tests.rs` | Adicionado teste L3: figure auto-labelled + user label geram `/Dests` correctos. |
| `00_nucleo/prompts/entities/elements/label.md` | Actualizado com campo `auto` e semântica de origem. |
| `00_nucleo/prompts/entities/elements/labelled.md` | Marcado como CONSOLIDADO em P464; remete para `label.md`. |
| `00_nucleo/prompts/entities/content.md` | Adicionada secção `Content::Label`; removidas referências a `Labelled`. |
| `00_nucleo/roteiro-conclusao-typst-cristalino-atualizado.md` | Dívida técnica `Content::Label` vs `Content::Labelled` marcada como RESOLVIDA. |
| `00_nucleo/materialization/typst-passo-464.md` | Critérios de fecho marcados como concluídos. |

### Notas

- `ElementPayload::Labelled` (usado para tags de introspeção pós-recursão) foi
  **mantido** intencionalmente — não é `Content::Labelled`; é o payload emitido
  pelo walk arm de labels auto-gerados.
- O `label_to_counter_key` duplicado entre `Introspector` e `TagIntrospector`
  foi deixado como scope-out explícito, conforme plano P464.
- O teste L2 requer o helper `labelled_prod` para levantar o `Styled` de
  numbering para fora do label, garantindo que `compute_labelled` inspecciona o
  `Heading` puro.

---

## Verificações

- `RUST_MIN_STACK=8388608 cargo test --workspace` ✅ (3269 passed; 0 failed)
- `cargo test --test crystalline_lint` ✅ (zero violations)

---

## Código de fecho

- `Content::Labelled` removido do enum `Content`.
- `LabelElem` ganhou campo `auto: bool`.
- Todos os consumidores migrados para `Content::Label` único.
- Ficheiro `entities/elements/labelled.rs` removido; `labelled.md` marcado como
  consolidado.
- 3 tests verdes (1 L1 + 1 L2 + 1 L3).
- Specs L0 actualizadas.
- Dívida técnica `Content::Label` vs `Content::Labelled` marcada como RESOLVIDA
  no roteiro.
