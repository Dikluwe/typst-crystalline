> **Passo:** 448 — Subscript e Superscript  
> **Data:** 2026-06-24  
> **Executor:** assistente IA (Kimi Code CLI)  
> **Ficheiro de instrução:** `00_nucleo/materialization/typst-passo-448.md`

---

## Resumo

Materializadas as funções nativas `#sub[...]` e `#super[...]` (subscrito e
sobrescrito) com paridade vanilla para o subset minimal: redução de corpo para
`0.6×` e deslocamento vertical da baseline (`-0.2em` para sub, `+0.3em` para
super).

---

## Implementação

### Modelo

Escolheu-se `Content::Styled(body, Styles::from_iter([Style::Subscript(true)]))`
em vez de variants próprios de `Content`, reaproveitando a infraestrutura de
estilos (ADR-0038/0039). Isto é consistente com a forma como `*bold*` / `_italic_`
trabalham internamente e simplifica o layout.

### Ficheiros alterados

| Ficheiro | Alteração |
|----------|-----------|
| `01_core/src/entities/style.rs` | `Style::Subscript(bool)`, `Style::Superscript(bool)`, `fold_into`, construtores e tests. |
| `01_core/src/entities/style_chain.rs` | `StyleDelta.subscript`/`superscript`, accessors, `diff_styles`, `From<&StyleChain> for TextStyle`. |
| `01_core/src/entities/layout_types.rs` | `TextStyle` ganha `subscript`, `superscript`, `baseline_offset`; `Default for Length`. |
| `01_core/src/entities/content.rs` | `Content::sub(body)`, `Content::superscript(body)`. |
| `01_core/src/entities/show.rs` | `NodeKind::Subscript`, `NodeKind::Superscript`. |
| `01_core/src/engine/stdlib/text.rs` | `native_subscript`, `native_superscript`. |
| `01_core/src/engine/stdlib/mod.rs` | Re-export dos nativos. |
| `01_core/src/engine/eval/mod.rs` | Registo "sub" / "super" no stdlib scope. |
| `01_core/src/engine/eval/rules.rs` | Matching de selectors `NodeKind::Subscript`/`Superscript` sobre `Content::Styled`. |
| `01_core/src/engine/layout/text.rs` | Aplicação de `size *= 0.6` e `baseline_offset` em função dos flags. |
| `01_core/src/engine/layout/cursor.rs` | Aplicação do `baseline_offset` ao posicionamento Y de `FrameItem::Text`. |
| `01_core/src/engine/eval/tests.rs` | 2 testes L1: `eval_sub_emite_styled`, `eval_super_emite_styled`. |
| `01_core/src/engine/layout/tests.rs` | 2 testes L3: baseline desloca para baixo/cima e tamanho reduzido. |
| `00_nucleo/prompts/entities/style.md` | Documentação das novas variantes. |
| `00_nucleo/prompts/entities/style_chain.md` | Documentação dos novos campos. |
| `00_nucleo/prompts/engine/stdlib/text.md` | Documentação de `sub` / `super`. |

### Notas

- As funções nativas foram colocadas em `rules/stdlib/text.rs` (ao lado de
  `smallcaps` e decorações de texto), em vez de `structural.rs`, porque o
  domínio é tipográfico. Os prompts L0 foram actualizados em `text.md`.
- O nome do construtor `Content::super` foi evitado porque `super` é keyword
  em Rust; usou-se `Content::superscript`.
- Os hashes `@prompt-hash` dos ficheiros cujos prompts mudaram foram
  actualizados com `crystalline-lint --fix-hashes`.

---

## Verificações

- `RUST_MIN_STACK=8388608 cargo test --workspace` ✅
- `crystalline-lint .` ✅ (apenas os 2 warnings órfãos pre-existentes)

---

## Código de fecho

- `Style::Subscript`/`Superscript` adicionados.
- `StyleDelta` propaga as 2 flags; `TextStyle` actualizado.
- `Content::sub`/`Content::superscript` construtores.
- `native_subscript`/`native_superscript` registadas como `sub`/`super`.
- Selectors `NodeKind::Subscript`/`Superscript` em show rules.
- Layout ajusta `TextStyle.size` e `TextStyle.baseline_offset`.
- 2 testes L1 + 2 testes L3 verdes.
- Spec L0 actualizado.
