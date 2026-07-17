> **Passo:** 449 — Highlight
> **Data:** 2026-06-24
> **Executor:** assistente IA (Kimi Code CLI)
> **Ficheiro de instrução:** `00_nucleo/materialization/typst-passo-449.md`

---

## Resumo

Materializada a função nativa `#highlight[...]` com fundo amarelo por defeito e
`fill: color` opcional. O modelo reaproveita `Content::Styled` +
`Style::Highlight(Option<Color>)`, como `#sub[...]` / `#super[...]` no Passo 448.

---

## Implementação

### Modelo

Escolheu-se `Content::Styled(body, Styles::from_iter([Style::Highlight(fill)]))`
em vez de variant próprio de `Content`, reaproveitando a infraestrutura de
estilos (ADR-0038/0039). Isto simplifica herança, selectors e layout.

### Ficheiros alterados

| Ficheiro | Alteração |
|----------|-----------|
| `01_core/src/entities/style.rs` | `Style::Highlight(Option<Color>)`, `fold_into`, construtor `Style::highlight`, tests actualizados para 13 variantes. |
| `01_core/src/entities/style_chain.rs` | `StyleDelta.highlight: Option<Option<Color>>`, accessor, `diff_styles`, `is_empty`, `From<&StyleChain> for TextStyle`. |
| `01_core/src/entities/layout_types.rs` | `TextStyle` ganha `highlight: Option<Color>`. |
| `01_core/src/entities/content.rs` | `Content::highlight(body, fill)`. |
| `01_core/src/entities/show.rs` | `NodeKind::Highlight`. |
| `01_core/src/engine/stdlib/text.rs` | `native_highlight` com `fill` default amarelo Typst (`rgba(255, 242, 54, 255)`), aceita `none`. |
| `01_core/src/engine/stdlib/mod.rs` | Re-export de `native_highlight`. |
| `01_core/src/engine/eval/mod.rs` | Registo `"highlight"` no stdlib scope. |
| `01_core/src/engine/eval/rules.rs` | Matching de selector `NodeKind::Highlight` sobre `Content::Styled` com `Style::Highlight`. |
| `01_core/src/engine/layout/text.rs` | Propaga `highlight` para `TextStyle` efectivo. |
| `01_core/src/engine/layout/cursor.rs` | `push_text` emite `FrameItem::Shape` rectangular com `fill` antes de cada `FrameItem::Text` quando `TextStyle.highlight` é `Some(color)`. |
| `01_core/src/engine/eval/tests.rs` | 3 testes L1: default amarelo, cor custom, `fill: none` desactiva. |
| `01_core/src/engine/eval/rules.rs` | 3 testes L1: selector casa highlight, não casa texto plano, `fill: none` continua a casar. |
| `01_core/src/engine/layout/tests.rs` | 4 testes L3: rect amarelo antes do texto, cor custom, `fill: none` sem shape, texto plano sem shape. |
| `00_nucleo/prompts/entities/show.md` | `NodeKind::Highlight` no conjunto completo. |
| `00_nucleo/prompts/entities/style.md` | Documentação da variante `Style::Highlight`. |
| `00_nucleo/prompts/entities/style_chain.md` | Documentação do campo `highlight`; contagens actualizadas para 13 campos. |
| `00_nucleo/prompts/engine/stdlib/text.md` | Secção `highlight(body, fill?)`. |

### Notas

- O Layouter emite **um rectângulo por run de texto** (`layout_word` /
  `layout_chunk`), cobrindo a altura da linha (`ascender → line_height`).
- `fill: none` traduz-se em `Style::Highlight(None)`, que o resolver achata para
  `TextStyle.highlight = None`, suprimindo o shape sem perder a origem sintática
  (o selector `NodeKind::Highlight` continua a casar).
- Nenhuma alteração no export PDF: `FrameItem::Shape` com `fill` já é suportado.
- Os hashes `@prompt-hash` dos ficheiros cujos prompts mudaram foram actualizados
  com `crystalline-lint --fix-hashes`.

---

## Verificações

- `RUST_MIN_STACK=8388608 cargo test --workspace` ✅
- `crystalline-lint .` ✅ (apenas os 2 warnings órfãos pre-existentes)

---

## Código de fecho

- `Style::Highlight` adicionado.
- `StyleDelta` propaga `highlight` (13 campos).
- `TextStyle` e `From<&StyleChain>` actualizados.
- `Content::highlight` e `native_highlight` registada como `"highlight"`.
- Selector `NodeKind::Highlight` em show rules.
- Layout emite `FrameItem::Shape` + `FrameItem::Text` para texto highlighted.
- 3 testes L1 (eval) + 3 testes L1 (selectors) + 4 testes L3 (layout) verdes.
- Spec L0 actualizado em 4 prompts.
