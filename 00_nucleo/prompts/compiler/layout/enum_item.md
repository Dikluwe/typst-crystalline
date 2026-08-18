# Prompt L0 — `compiler/layout/enum_item` — layout de `EnumItemElem`
Hash do Código: 1fdc5a12

**Camada**: L1 · **Alvo**: `01_core/src/compiler/layout/enum_item.rs`
**Origem**: atomização (ADR-0109, P380); campos `indent`/`body_indent`/`tight`
adicionados em **P505**.
**Prompts relacionados**: `entities/elements/enum_item.md`,
`entities/enum_numbering.md`, `rules/stdlib/structural.md`.

---

## Assinatura

```rust
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e:        &EnumItemElem,
)
```

## Semântica

Renderiza um item de lista ordenada num fluxo de bloco:

1. **Quebra de linha** se o cursor já estiver além de `line_start_x`
   (`cursor_x.0 > line_start_x.0`).
2. **Espaçamento entre grupos separados por Parbreak** (P864):
   - Se o item anterior na sequência foi um `EnumItem` e ocorreu um
     `Content::Parbreak` desde então
     (`layouter.parbreak_since_last_item == true` e
     `layouter.last_seen_item_group == Some(ItemGroup::Enum)`), avança
     `cursor_y` por um `paragraph_advance` (P762:
     `top_edge + |bottom_edge| + leading`) antes de posicionar o rótulo.
   - Reseta `layouter.last_was_loose_item = false` para não acumular com o
     espaçamento de itens soltos do mesmo grupo.
   - Reseta `layouter.enum_counter = None` para que o próximo item sem número
     reinicie a numeração em `1.`.
3. **Espaçamento entre itens soltos** (P505):
   - Se `e.tight == Some(false)` e o item anterior na sequência também foi um
     `ListItem`/`EnumItem` solto (`layouter.last_was_loose_item == true`),
     avança `cursor_y` por um `line_height` antes de posicionar o rótulo.
   - Actualiza `layouter.last_was_loose_item = (e.tight == Some(false))`.
4. **Resolve indentação**:
   - `indent` (Length) → deslocamento horizontal do rótulo numérico em relação
     à margem. Default `0pt`.
   - `body_indent` (Length) → deslocamento horizontal do corpo em relação ao
     fim do rótulo. Default `0.5em` (P1072 / paridade vanilla `enum.rs:152-154`).
   - `tight` (bool) → `true` (default) não adiciona espaço extra entre itens;
     `false` adiciona um espaçamento vertical equivalente a uma linha *entre*
     itens consecutivos soltos.

> **Fonte de paridade (P1031)** — doc comments dos campos de `EnumElem` no vanilla
> ratificado (`e0e8ca4d`), `crates/typst-library/src/model/enum.rs`, publicados em
> `typst.app/docs/reference/model/enum/`:
>
> | Campo | `file:line` | Doc comment / default |
> |---|---|---|
> | `indent` | `enum.rs:149-150` | *"The indentation of each item."* — sem `#[default]`, logo `0pt` ✅ confere |
> | `body_indent` | `enum.rs:152-154` | *"The space between the numbering and the body of each item."* — `#[default(Em::new(0.5).into())]` ❌ **`0.5em`, não `0pt`** |
> | `tight` | `enum.rs:89-90` | `#[default(true)]` ✅ confere no valor |
>
> **ACHADO CORRIGIDO (P1072) — default de `body_indent`** (mesmo achado que `layout/list_item.md`;
> um só passo de correcção cobre os dois). Medição de 2026-08-13, documento `+ Um` /
> (linha em branco) / `+ Dois`: vanilla dá `1. Um` / `2. Dois`; cristalino dá `1.Um` /
> `2.Dois`. Gate ADR-0127, passo próprio, **não implementado aqui**.
>
> **Sobre `tight`**: o vanilla define-o por espaçamento de parágrafo (`enum.rs:68-72`:
> *paragraph spacing* quando `{false}`, *paragraph leading* quando `{true}`) e fixa a regra
> de markup em `enum.rs:74-78`: *"In markup mode, the value of this parameter is determined
> based on whether items are separated with a blank line. […] The markup-defined tightness
> cannot be overridden with set rules."* A fórmula "equivalente a uma linha" é aproximação
> do cristalino, não citação.
5. **Formata o rótulo** com `EnumNumbering::format(number)`; se `number` for
   `None`, usa `"-"` como placeholder.
6. **Posiciona o rótulo** em:
   ```text
   label_x = margin + indent
   ```
   O rótulo é emitido como `FrameItem::Text` na posição corrente `cursor_y`.
7. **Mede a largura do rótulo** via `layouter.metrics.advance(label_str, font_size_pt)`.
8. **Posiciona o corpo** alterando temporariamente `line_start_x` e `cursor_x`
   para:
   ```text
   body_x = margin + indent + label_width + body_indent
   ```
   O corpo é renderizado com `layouter.layout_content(&e.body)`. A alteração de
   `line_start_x` garante que quebras de linha dentro do body mantenham a
   indentação.
9. **Restaura** `line_start_x` para o valor original após `flush_line`.

## Auto-incremento sequencial (mecanismo `enum_counter`)

O número de cada item é resolvido sequencialmente durante o layout
single-pass:

- A struct `Layouter` mantém o campo `enum_counter: Option<u32>` com o
  número actual da lista ordenada activa.
- No walk de sequência (`compiler/layout/sequence.rs`, governado por
  `compiler/layout.md`), qualquer item que não seja `EnumItem` faz reset do
  contador para `None`.
- **P864** — um `Content::Parbreak` entre itens `EnumItem` consecutivos
  também faz reset do contador para `None`, pelo que o próximo item sem
  número explícito reinicia a numeração em `1.`.
- Neste módulo: se o item não tiver `number` definido (`None`), o layouter
  calcula `enum_counter.unwrap_or(0) + 1`, actualiza o estado e formata o
  rótulo com o esquema de numeração do enum; se o item definir `number`
  explicitamente (`Some(n)`), o contador passa a `Some(n)`.

## Validação

- `enum(indent: 1.5em, body-indent: 0.5em, tight: false, [A], [B])` produz
  itens cujo rótulo está deslocado de `1.5em` e cujo corpo está deslocado de
  `1.5em + label_width + 0.5em` em relação à margem.
- `enum(tight: true, [A], [B])` mantém `cursor_y` inalterado entre itens.
- `enum(tight: false, [A], [B])` adiciona `line_height` de espaço entre itens.
- Itens com texto longo mantêm a indentação do corpo nas linhas quebradas.
- Dois `EnumItem` separados por `Content::Parbreak` (P864) **continuam** a numeração
  (`1.`, `2.`) e têm gap vertical de `2 * line_advance`; sem `Parbreak`, a numeração
  continua na mesma e o gap é `1 * line_advance`.

> **Correcção P1031 — o critério anterior estava errado nos dois lados.**
>
> A redacção anterior exigia que dois `EnumItem` separados por `Parbreak` *"reiniciam a
> numeração (ambos `1.`)"*. Isso contradiz a linguagem **e** o próprio cristalino:
>
> - **Linguagem**: uma linha em branco entre itens muda apenas a *tightness*, não a
>   numeração — `crates/typst-library/src/model/enum.rs:74-78`: *"In markup mode, the value
>   of this parameter is determined based on whether items are separated with a blank line.
>   If items directly follow each other, this is set to `{true}`; if items are separated by
>   a blank line, this is set to `{false}`."* O campo afectado é `tight`, não o contador.
> - **Medição (2026-08-13)**, documento `+ Um` / (linha em branco) / `+ Dois`:
>   vanilla `/usr/local/bin/typst` (`typst 0.15.1 (e0e8ca4d)`) → `1. Um` / `2. Dois`;
>   cristalino `target/release/typst` (fonte em HEAD `4f64e4e69`) → `1.Um` / `2.Dois`.
>   **Os dois continuam a numeração.**
>
> Ou seja: o comportamento actual do cristalino está certo e o critério de validação é que
> estava desactualizado (deriva desde P864). Critério reescrito acima. A única diferença
> que sobra neste caso é o espaço rótulo↔corpo (`body_indent`), tratada no bloco da secção
> "Resolve indentação".

## Scope-outs explícitos

- Renderização de `numbering` complexo (prefixos/sufixos arbitrários) — ver
  `enum_numbering.md`.
