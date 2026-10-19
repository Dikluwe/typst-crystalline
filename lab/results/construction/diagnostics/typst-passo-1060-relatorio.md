# Relatório de Execução — Passo 1060

**Data**: 2026-08-17
**Passo**: 1060 — Investigação do Mecanismo Real de Margin Collapsing e Auditoria de Nomes de Arquivo Duplicados em Prompts L0
**Gate**: `ADR-0127` (Classificação: Investigação Pura / Sem alteração de código)
**Status**: CONCLUÍDO COM ÊXITO (Mapeamento fidedigno dos campos e pontos de colapso no código real L1 + Auditoria quantitativa de 350 prompts L0 com 35 colisões identificadas e opções estruturadas para o dono)

---

## 1. Parte 1 — Mecanismo Real de Margin Collapsing no Código L1

A investigação no código-fonte real (`01_core/src/compiler/layout/`) substituiu suposições e documentações antigas pelos nomes e fluxos exatos implementados.

### 1.1 Campos do `Layouter` Envolvidos

Em `01_core/src/compiler/layout/mod.rs` (linhas 324–329):

```rust
/// **P250 (M9d / M7+5; ADR-0079 Categoria A.4 cumulativa; cita
/// ADR-0082 PROPOSTO N=1)** — below pendente do bloco anterior
/// para CSS-style margin collapse `max(prev.below, curr.above)`
/// entre blocks consecutivos. Reset por Sequence consumer +
/// non-Block arms.
pub(super) prev_block_below_pending: f64,

/// **P250** — `true` se o elemento previamente laid out foi
/// `Content::Block`. Used by `prev_block_below_pending` collapse
/// logic + first-block-in-sequence above suppression. Reset por
/// non-Block arms (via Sequence consumer ou directamente).
pub(super) block_chain_active: bool,
```

### 1.2 Mecanismo em `Content::Block` (`01_core/src/compiler/layout/block.rs`)

1. **Entrada do Bloco (Resolução e Colapso de `above`)**:
   ```rust
   // 1. Termina linha em curso.
   if layouter.regions.current.cursor_x.0 > layouter.regions.current.line_start_x.0 {
       layouter.flush_line();
   }

   // P250 — spacing/above collapse (paridade vanilla CSS margin
   // collapse `max(prev.below, curr.above)` entre Blocks
   // consecutivos; `above` suprimido no primeiro Block dum
   // Sequence — sinalizado via `block_chain_active == false`).
   let above_pt = above.or(*spacing).map(|l| l.resolve_pt(font)).unwrap_or(0.0);
   let gap = if layouter.block_chain_active {
       layouter.prev_block_below_pending.max(above_pt)
   } else {
       0.0
   };
   let advance = (gap - layouter.prev_block_below_pending).max(0.0);
   layouter.regions.current.cursor_y += Pt(advance);
   layouter.prev_block_below_pending = 0.0;
   ```

2. **Saída do Bloco (Avanço e Registo de `below`)**:
   ```rust
   // P250 — below cursor.y advance + state update para
   // collapse com próximo Block consecutivo.
   let below_pt = below.or(*spacing).map(|l| l.resolve_pt(font)).unwrap_or(0.0);
   layouter.regions.current.cursor_y += Pt(below_pt);
   layouter.prev_block_below_pending = below_pt;
   layouter.block_chain_active = true;
   ```

### 1.3 Mecanismo em `Content::Sequence` (`01_core/src/compiler/layout/sequence.rs`)

No Sequence consumer com iterador `peekable`:
```rust
let saved_below = layouter.prev_block_below_pending;
let saved_chain = layouter.block_chain_active;
layouter.prev_block_below_pending = 0.0;
layouter.block_chain_active = false;

let mut iter = parts.iter().peekable();
while let Some(part) = iter.next() {
    // ... lookahead sticky e agrupamentos P864 ...
    layouter.layout_content(part);
    if !matches!(part, Content::Block { .. } | Content::Shape(_)) {
        // P250 — non-Block child quebra chain.
        // **P767a** — `Content::Shape` também é block-level, logo
        // mantém o estado de colapso de margem.
        layouter.block_chain_active = false;
        layouter.prev_block_below_pending = 0.0;
    }
    // ...
}

layouter.prev_block_below_pending = saved_below;
layouter.block_chain_active = saved_chain;
```

### 1.4 Mecanismo em `Content::Parbreak` (`01_core/src/compiler/layout/mod.rs`)

```rust
Content::Parbreak => {
    let had_items = !self.regions.current.current_line.is_empty();
    self.flush_line();
    if had_items {
        let font_size = self.style.size.val();
        use crate::entities::value::Value;
        use super::layout::vanilla_defaults::{PAR_LEADING, PAR_SPACING};
        let spacing_pt = match self.chain.custom("par.spacing") {
            Some(Value::Length(l)) => l.resolve_pt(font_size),
            // PAR_SPACING, ver vanilla_defaults.rs
            _ => font_size * PAR_SPACING,
        };
        let leading_pt = match self.chain.custom("par.leading") {
            Some(Value::Length(l)) => l.resolve_pt(font_size),
            // PAR_LEADING, ver vanilla_defaults.rs
            _ => self.style.leading.map(|l| l.resolve_pt(font_size)).unwrap_or(font_size * PAR_LEADING),
        };
        let extra_spacing = (spacing_pt - leading_pt).max(0.0);
        self.regions.current.cursor_y += Pt(extra_spacing);
    }
}
```

### 1.5 Conformidade com `00_nucleo/prompts/entities/content.md`
- **Validação**: A seção P250 de `00_nucleo/prompts/entities/content.md` (linhas 1959–1981) **está rigorosamente alinhada** com a implementação real de `block.rs` e `sequence.rs`, descrevendo fielmente os campos `prev_block_below_pending` e `block_chain_active`.

---

## 2. Parte 2 — Auditoria de Nomes de Arquivo Duplicados em `00_nucleo/prompts/`

### 2.1 Estatísticas Gerais
- **Total de arquivos `.md` em `00_nucleo/prompts/`**: 350
- **Total de nomes de arquivos distintos**: 302
- **Nomes com colisões/duplicatas (> 1 ocorrência)**: **35 nomes**
- **Total de arquivos envolvidos em colisões**: **83 arquivos** (23.7% da base de prompts)

### 2.2 Inventário Completo das Colisões por Nome

#### 7 Ocorrências (1 nome, 7 arquivos)
* **`mod.md`**:
  * `00_nucleo/prompts/entities/mod.md`
  * `00_nucleo/prompts/entities/ast/mod.md`
  * `00_nucleo/prompts/compiler/mod.md`
  * `00_nucleo/prompts/compiler/lexer/mod.md`
  * `00_nucleo/prompts/compiler/math/mod.md`
  * `00_nucleo/prompts/infra/export/mod.md`
  * `00_nucleo/prompts/infra/export/gradients/mod.md`

#### 3 Ocorrências (8 nomes, 24 arquivos)
* **`layout.md`**:
  * `00_nucleo/prompts/compiler/layout.md`
  * `00_nucleo/prompts/compiler/stdlib/layout.md`
  * `00_nucleo/prompts/infra/layout.md`
* **`heading.md`**:
  * `00_nucleo/prompts/entities/elements/heading.md`
  * `00_nucleo/prompts/compiler/layout/heading.md`
  * `00_nucleo/prompts/compiler/stdlib/structural/heading.md`
* **`footnote.md`**:
  * `00_nucleo/prompts/entities/elements/footnote.md`
  * `00_nucleo/prompts/compiler/layout/footnote.md`
  * `00_nucleo/prompts/compiler/stdlib/structural/footnote.md`
* **`bibliography.md`**:
  * `00_nucleo/prompts/entities/elements/bibliography.md`
  * `00_nucleo/prompts/compiler/layout/bibliography.md`
  * `00_nucleo/prompts/compiler/stdlib/structural/bibliography.md`
* **`table.md`**:
  * `00_nucleo/prompts/entities/elements/table.md`
  * `00_nucleo/prompts/compiler/eval/table.md`
  * `00_nucleo/prompts/compiler/layout/table.md`
* **`label.md`**:
  * `00_nucleo/prompts/entities/label.md`
  * `00_nucleo/prompts/entities/elements/label.md`
  * `00_nucleo/prompts/compiler/stdlib/label.md`
* **`color.md`**:
  * `00_nucleo/prompts/entities/color.md`
  * `00_nucleo/prompts/compiler/stdlib/color.md`
  * `00_nucleo/prompts/compiler/stdlib/foundations/color.md`
* **`_comum.md`**:
  * `00_nucleo/prompts/entities/elements/_comum.md`
  * `00_nucleo/prompts/compiler/math/layout/_comum.md`
  * `00_nucleo/prompts/compiler/stdlib/_comum.md`

#### 2 Ocorrências (26 nomes, 52 arquivos)
* **`cast.md`**: `compiler/eval/cast.md`, `compiler/stdlib/foundations/cast.md`
* **`columns.md`**: `entities/elements/columns.md`, `compiler/columns.md`
* **`counter.md`**: `entities/counter.md`, `compiler/stdlib/counter.md`
* **`counter_update.md`**: `entities/counter_update.md`, `entities/elements/counter_update.md`
* **`divider.md`**: `entities/elements/divider.md`, `compiler/stdlib/structural/divider.md`
* **`enum_item.md`**: `entities/elements/enum_item.md`, `compiler/layout/enum_item.md`
* **`equation.md`**: `entities/elements/equation.md`, `compiler/layout/equation.md`
* **`eval.md`**: `compiler/eval.md`, `compiler/stdlib/eval.md`
* **`fonts.md`**: `infra/fonts.md`, `infra/export/fonts.md`
* **`image-sizer.md`**: `entities/image-sizer.md`, `infra/image-sizer.md`
* **`lang.md`**: `entities/lang.md`, `compiler/lang.md`
* **`link.md`**: `entities/elements/link.md`, `compiler/layout/link.md`
* **`list_item.md`**: `entities/elements/list_item.md`, `compiler/layout/list_item.md`
* **`markup.md`**: `entities/ast/markup.md`, `compiler/stdlib/structural/markup.md`
* **`math.md`**: `entities/ast/math.md`, `compiler/stdlib/structural/math.md`
* **`math_style.md`**: `entities/math_style.md`, `compiler/stdlib/math_style.md`
* **`operators.md`**: `entities/operators.md`, `compiler/eval/operators.md`
* **`outline.md`**: `entities/elements/outline.md`, `compiler/stdlib/structural/outline.md`
* **`plugin_host.md`**: `contracts/plugin_host.md`, `infra/plugin_host.md`
* **`quote.md`**: `entities/elements/quote.md`, `compiler/stdlib/structural/quote.md`
* **`ref.md`**: `entities/elements/ref.md`, `compiler/stdlib/ref.md`
* **`regex.md`**: `entities/regex.md`, `compiler/stdlib/text/regex.md`
* **`smartquote.md`**: `entities/elements/smartquote.md`, `compiler/stdlib/text/smartquote.md`
* **`state.md`**: `entities/elements/state.md`, `compiler/stdlib/state.md`
* **`state_update.md`**: `entities/elements/state_update.md`, `entities/state_update.md`
* **`world.md`**: `contracts/world.md`, `world.md`

### 2.3 Comparativo das Opções de Decisão

| Aspecto | Opção (a): Prefixar Camada no Nome | Opção (b): Exigir Caminho Completo Canónico |
| :--- | :--- | :--- |
| **Descrição** | Renomear os 83 arquivos (ex.: `compiler-layout.md`, `stdlib-layout.md`). | Manter nomes 1:1 com os arquivos Rust; exigir caminhos completos nos prompts e relatórios (ex.: `00_nucleo/prompts/compiler/layout.md`). |
| **Arquivos afetados** | **83 arquivos `.md` renomeados** + centenas de arquivos `.rs` com anotação `//! @prompt`. | **0 arquivos renomeados**. Preserva correspondência 1:1 de nomes entre especificação e módulo Rust. |
| **Risco de Quebra de Linhagem** | Alto (exigiria resselo em massa de `@prompt` no `crystalline-lint`). | Baixo (regra de disciplina documental imediata sem churn de código). |

---

## 3. Conclusão e Próximos Passos

1. O mecanismo de colapso vertical existente e seus pontos de atuação estão totalmente mapeados no código-fonte real (`prev_block_below_pending` e `block_chain_active`).
2. O inventário de prompts L0 está concluído e pronto para balizar a reescrita do L0 de margin collapsing sem qualquer suposição indevida.
3. Nenhuma alteração de código foi realizada, em estrita conformidade com o enunciado do Passo 1060.
