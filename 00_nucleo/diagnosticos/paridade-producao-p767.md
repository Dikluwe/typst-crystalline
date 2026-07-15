# Diagnóstico P767 — L0: `Content::Shape` como bloco que quebra parágrafo + arqueologia

**Data da medição:** 2026-07-15T21:35:00-03:00  
**Commit base:** `f3eec288e91e29ce94264e4f6adf8c3090349676`  
**Working tree:** alterações não relacionadas pendentes (deleções em `00_nucleo/0.15.0.typ` e `00_nucleo/testing/fontes-padrao-teste.md`); este passo não faz alterações de código.  
**Passo:** P767  
**Objectivo:** Escrever o Prompt L0 para corrigir o modelo de fluxo de `Content::Shape` e rastrear onde a decisão original (fluxo contínuo) foi tomada.

---

## Parte A — Arqueologia

### Commits identificados

```bash
git log --all --oneline -S "native_rect" -- "01_core/src/rules/stdlib.rs" "01_core/src/rules/stdlib/shapes.rs" | tail -10
git log --all --oneline -S "Content::Shape" -- "01_core/src/entities/content.rs" | tail -10
```

| Passo (numeração da época) | Commit | O que decidiu (de facto) |
|---|---|---|
| **Passo 76** | `7a0a4315f` | Introdução de `Content::Shape`, `ShapeKind`, `Stroke`, `native_rect`/`native_line` e layout de formas no `Layouter`. As formas foram adicionadas ao fluxo contínuo existente (`cursor_x`/`cursor_y`, `flush_line()`), sem avaliação explícita da questão inline vs block. |
| **Passo 77-78** | `3c6a5fbf0` | Introdução de `ellipse`/`circle`, replicando o mesmo padrão de `rect`/`line`. |
| **Passo 96.5** | `d2faea55d` | Reestruturação de `stdlib.rs` em submódulos por domínio; `shapes.rs` criado. Nenhuma mudança semântica. |
| **Passo 326** | `dfef736af` | Migração modelo D: `Shape` passa a `ShapeElem` (leaf, não-locatável). Layout inalterado. |
| **P727** | `fa88c09be` | Fallback de stroke default em `curve`/`polygon`. Não toca no modelo de fluxo. |
| **P732** | `a2c26e4a2` | `polygon` com fallback de stroke e coordenadas `Length`. Não toca no modelo de fluxo. |
| **P734** | `be74c5e22` | `polygon` restringido a `Length`. Não toca no modelo de fluxo. |
| **P739** | `a6798f8fa` | Paridade `line(end:)`, `display Float`, etc. Não toca no modelo de fluxo. |
| **P741** | `17c825928` | Scope-out de `polygon` com ratio. Não toca no modelo de fluxo. |
| **P748/P750/P751/P752** | `72ad29a0a`, `c92d0622f`, `f6200771e`, `be3eaa85f` | Ajustes de baseline inicial, cap-height e margem para formas. Refinam posicionamento vertical, mas não alteram o facto de as formas estarem no fluxo contínuo. |
| **P763h** | `94fd4edea` | Divergência sistémica confirmada; scope-out registado; decisão de corrigir adiada para L0 dedicado. |
| **P767 (este passo)** | — | L0 escrito; arqueologia registada. |

### Confirmação da hipótese

Em `00_nucleo/materialization/typst-passo-76.md`, a secção de layout de `Content::Shape` (Tarefa 3b) especifica:

```rust
if self.cursor_y.0 + resolved_h > self.page_height.0 - self.margin.0 {
    self.new_page();
}
self.flush_line();
let pos = Point { x: self.cursor_x, y: self.cursor_y };
self.push_frame_item(pos, FrameItem::Shape { ... });
self.cursor_y += Pt(resolved_h);
```

Não há menção a "block", "parágrafo", "inline" ou "quebra de parágrafo". A forma foi simplesmente inserida no motor de fluxo contínuo já existente. A hipótese de P767 está confirmada: **a divergência não foi uma escolha consciente contra o vanilla — foi uma consequência não avaliada da arquitectura do Layouter**, que já era block-agnóstica desde a Fase 1 do projecto.

---

## Parte B — L0 escrito

**Ficheiro:** `00_nucleo/prompts/rules/layout/shape_block_behaviour.md`

Cobertura dos 5 pontos exigidos:

1. **Contrato**: `Content::Shape` força fecho de parágrafo antes e depois, replicando `BlockElem::single_layouter`. Decidido: não replicar o aviso do vanilla; quebrar silenciosamente como outros blocos cristalinos.
2. **Ponto de intercepção**: reutilizar o mecanismo de bloco existente (`block::layout`, `block_chain_active`, `prev_block_below_pending`, `above`/`below`). Três opções listadas (A/B/C), com A preferida.
3. **Espaçamento above/below**: por defeito `1.2em`, medido do vanilla (`BlockElem` em `container.rs:342`). Deve usar o colapso existente (P250).
4. **Impacto em `place()`**: formas dentro de `place()` não são afectadas (caminho absoluto, corrigido em P763f).
5. **Impacto nos testes de regressão**: busca em `01_core/src/rules/layout/tests.rs` não encontrou testes que misturem texto literal com formas no mesmo fluxo; testes dependentes do comportamento actual devem ser actualizados conscientemente.

**Hash do Código:** marcado como `P767-PENDING`, porque o Protocolo de Nucleação estabelece que o hash do código L1 é calculado pelo humano **antes** da implementação, e este passo é explicitamente "sem implementação".

---

## Validação

- `cargo test --workspace` — verde.
- `crystalline-lint .` — zero violações (apenas V7 esperado de `package_version_resolution.md`).

Nenhum código L1/L2/L3 foi escrito neste passo.

---

## Conclusão

- A decisão original de tratar formas como fluxo contínuo foi tomada implicitamente em **Passo 76** (`7a0a4315f`), ao absorver `Content::Shape` no motor de cursor único do Layouter sem avaliar inline vs block.
- O L0 `shape_block_behaviour.md` foi escrito e registado; a implementação fica para **P767a**, que deve escolher entre reutilizar `block::layout`, envolver implicitamente em `BlockElem`, ou introduzir estado de quebra de parágrafo no `Layouter`.
- O critério de fecho de P767 está satisfeito: arqueologia confirmada, tabela preenchida, L0 escrito, nenhum código implementado.
