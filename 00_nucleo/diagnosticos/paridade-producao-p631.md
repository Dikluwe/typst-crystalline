# Relatório — Passo 631 (P631): `boxed.rs` usa o helper partilhado `layout_sub_frame_inline`

**Data:** 2026-07-09  
**Commit base:** `435e0fe1b` (P630 — esclarecimento `place.rs`/`placement.rs`/`cursor.rs:650)  
**Foco:** Migrar `boxed.rs` para uma variante inline do helper `layout_sub_frame`, eliminando o save/restore manual da `current_line` e o alinhamento RTL isolado.

---

## Resumo executivo

P628 identificou que `boxed.rs` tratava o alinhamento RTL "à mão" — isolava os itens que o body adicionava à `current_line` do pai e chamava `align_current_line_rtl()` apenas sobre esses itens. P631 introduz `layout_sub_frame_inline` em `sub_frame.rs`, que encapsula exactamente esse padrão, e migra `boxed.rs` para a usar.

Resultado: menos código duplicado, comportamento preservado, todos os testes passam, linter limpo.

---

## Sonda mínima

Código anterior de `boxed.rs` (linhas 93–115 do estado pré-P631):

```rust
let parent_line_len_before = layouter.regions.current.current_line.len();
layouter.layout_content(body);
let body_tail: Vec<FrameItem> = layouter
    .regions
    .current
    .current_line
    .drain(parent_line_len_before..)
    .collect();
let saved_line = std::mem::take(&mut layouter.regions.current.current_line);
layouter.regions.current.current_line = body_tail;
layouter.align_current_line_rtl();
let aligned_body_tail = std::mem::take(&mut layouter.regions.current.current_line);
layouter.regions.current.current_line = saved_line;
layouter.regions.current.current_items.extend(aligned_body_tail);
```

A sonda confirmou que este padrão pode ser extraído sem alterar a semântica: o body do box continua na linha do pai, mas o alinhamento RTL aplica-se apenas à cauda de itens que o body produziu.

---

## Implementação

### `layout_sub_frame_inline` em `sub_frame.rs`

Nova função no `impl Layouter` de `sub_frame.rs`:

```rust
pub(super) fn layout_sub_frame_inline(
    &mut self,
    content: &Content,
    region: SubLayoutRegion,
) -> (f64, Vec<FrameItem>)
```

Semântica implementada:

- Não altera `regions.current.width` nem `line_start_x` — o caller mantém o controlo da largura (necessário para preservar o comportamento de `Boxed.width: None`).
- Executa `layout_content(content)` com a `current_line` do pai intacta.
- Isola os itens adicionados pelo body via `drain(parent_line_len_before..)`.
- Aplica `align_current_line_rtl()` apenas sobre esses itens, numa `current_line` temporária.
- Restaura a `current_line` do pai.
- Devolve `(height, items)`, onde `height` é a altura da linha resultante (fallback ao `line_height` do estilo activo).

### `boxed.rs` migrado

O bloco manual de save/restore/alinhamento foi substituído por:

```rust
let (_, body_items) = layouter.layout_sub_frame_inline(
    body,
    super::sub_frame::SubLayoutRegion {
        origin_x: 0.0,
        width: 0.0,
        height: None,
        align_rtl: true,
        unconstrained_height: true,
    },
);
layouter.regions.current.current_items.extend(body_items);
```

O save/restore de `regions.current.width` e `parent_bbox` permanecem em `boxed.rs` porque são específicos do container.

### Prompt L0 actualizado

A secção "Sub-layout isolado" de `00_nucleo/prompts/rules/layout.md` foi actualizada com a subsecção `layout_sub_frame_inline` e o hash correspondente (`935704d4`).

---

## Validação

### Testes automatizados

```bash
cargo test -p typst-core boxed
```

Resultado: **29 passed; 0 failed**.

```bash
cargo test --workspace
```

Resultado: **todos passam** (ver log completo; 0 falhas).

```bash
crystalline-lint .
```

Resultado: **No violations found**.

### Teste visual RTL

Documento de teste:

```typ
#box(width: 150pt)[
  #set text(lang: "ar", dir: rtl, size: 30pt)
  الكتاب على الطاولة
]
```

Comando usado:

```bash
./target/debug/typst /tmp/p631-box-arabe.typ /tmp/p631.pdf
mutool draw -o /tmp/p631.png -r 150 /tmp/p631.pdf
```

Observável: as três palavras árabes (`الكتاب`, `على`, `الطاولة`) aparecem alinhadas à direita do box, uma por linha — igual ao comportamento descrito no relatório de P625.

### Zero mudanças de snapshot

Nenhum teste automatizado exigiu regeneração de snapshot; o refactor foi passivo.

---

## Decisões e notas

- **Por que a função não aplica `regions.current.width`?** `boxed.rs` configura a largura do box antes de chamar o helper. Se o helper tentasse aplicar uma largura genérica, `Boxed.width: None` (que não deve restringir largura) seria afectado. Manter a configuração da largura no caller preserva a semântica exacta.
- **`origin_x`/`width` da `SubLayoutRegion` são ignorados** nesta variante inline. Foram reservados no tipo para manter consistência com `layout_sub_frame`, mas a semântica inline não os necessita.
- **A altura devolvida é informativa**; `boxed.rs` não a usa directamente porque a medição real do body para `clip` continua a usar `measure_content_constrained`.

---

## Próximo passo

Conforme o plano de P628: avaliação de `columns.rs` — decidir se merece um helper de nível superior próprio ou se fica fora da unificação por ter responsabilidades genuinamente diferentes (particionamento e ordenação regional, não só colocação de texto dentro de uma largura).

---

## Critérios de fecho

- [x] Sonda mínima completa: código actual de `boxed.rs` mapeado linha a linha.
- [x] `layout_sub_frame_inline` criada em `sub_frame.rs`.
- [x] `boxed.rs` migrado para a nova função.
- [x] Código manual de save/restore da `current_line` removido de `boxed.rs`.
- [x] Teste visual de RTL em box confirmado, igual ao resultado de P625.
- [x] Zero mudanças de snapshot inesperadas.
- [x] Sem regressão em `cargo test --workspace`.
- [x] `crystalline-lint .` limpo.
- [x] Prompt L0 actualizado com a nova variante.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p631.md`.
