---

# P487 — Trilha 6: LoF/LoT com page numbers + audit final do projecto

> **Passo:** 487
> **Data:** 2026-06-28
> **Foco:** (A) Fechar Trilha 6 5/5: List of Figures e List of Tables com page numbers reais, via infraestrutura fixpoint já existente (`known_page_numbers`); (B) Audit final — actualizar roteiro, estado de ADRs, e paridade.
> **Trilha:** 6 — Bibliografia Fase 2 — item final.
> **Tipo:** Materialização M + Documental.
> **Tamanho:** M (~30 min sub-item A) + XS (sub-item B).
> **ADR-0072 ACEITE** — TOC fixpoint estruturalmente fechado (P192B). `LayouterRuntimeState.known_page_numbers` disponível.

---

## Contexto

**Trilha 6 estado:** 4/5 completo. O único item pendente é LoF/LoT com page numbers reais.

P472 materializou `native_lof()` e `native_lot()` que geram listas de figuras/tabelas com número e caption, mas **sem page numbers** — scope-out declarado como "requer 2-pass convergente".

A infra de 2-pass **já existe** desde P192B (ADR-0072):
- `LayouterRuntimeState.known_page_numbers: HashMap<Label, usize>` — populado entre iterações do fixpoint TOC.
- Loop fixpoint em `layout/mod.rs:1515` — activo quando `kind_index.contains_key(&ElementKind::Outline)`.
- `MAX_ITERATIONS = 5` — convergência via comparação de `extracted_label_pages == known_page_numbers`.

O mecanismo que resolve page numbers em TOC (`outline.rs` lê `runtime.known_page_numbers.get(&label)`) pode ser reaproveitado para LoF/LoT.

**Sub-item B** é um audit final do projecto pós-P486 — actualizar o roteiro `roteiro-conclusao-typst-cristalino-atualizado.md` para reflectir o estado real.

---

## ADR-0108 — Medir antes de decidir

| Pergunta | Verificar em | Status |
|----------|-------------|--------|
| `native_lof()` existe e gera `Content::Outline` com `target: Figures`? | `rules/stdlib/structural.rs` | 🟡 |
| `layout/outline.rs` trata `OutlineTarget::Figures`? | `rules/layout/outline.rs` | 🟡 |
| Figuras têm label registado em `extracted_label_pages`? | `rules/layout/references.rs` | 🟡 |
| `known_page_numbers` populado para figuras? | `rules/layout/mod.rs:1515` — loop fixpoint | 🟡 |
| `outline.rs` usa `runtime.known_page_numbers` para page numbers? | `rules/layout/outline.rs:51` | ✅ (ADR-0072) |
| Fixpoint activo para `OutlineTarget::Figures`? | `layout/mod.rs:1515` — condição `contains_key(Outline)` | 🟡 — pode não incluir `lof`/`lot` |
| `extracted_label_pages` inclui page numbers de figuras? | `layout/mod.rs:1139` | 🟡 |

**Sondas 🟡 com `grep`/`file:line` antes de escrever código.**

---

## Sub-item A — LoF/LoT com page numbers

### A.1 — Diagnóstico esperado

O problema mais provável: o loop fixpoint TOC (`layout/mod.rs:1515`) só é activado quando `kind_index.contains_key(&ElementKind::Outline)`. Um documento com `#lof()` (sem `#outline()`) pode não activar o fixpoint, logo `known_page_numbers` fica vazio.

**Sonda:** verificar a condição de activação do loop fixpoint.

Se a condição é `ElementKind::Outline` apenas, adicionar `ElementKind::OutlineFigures` (ou equivalente) como condição OR:

```rust
// layout/mod.rs:1515 — condição de activação fixpoint
if intr.kind_index.contains_key(&ElementKind::Outline)
|| intr.kind_index.contains_key(&ElementKind::OutlineFigures)
|| intr.kind_index.contains_key(&ElementKind::OutlineTables) {
    // loop fixpoint
}
```

### A.2 — Labels de figuras em `extracted_label_pages`

`extracted_label_pages` é populado em `references.rs` quando `Content::Labelled` ou `Content::Label` é encontrado com uma posição de página. Figuras geradas por `Content::Figure` devem ter labels se foram registadas com `#label(...)`. Se não têm label explícito, a figura não aparece em `extracted_label_pages`.

**Abordagem alternativa:** o `Introspector` já tem `kind_index[ElementKind::Figure]` com posições. Em vez de depender de labels, o layout de LoF pode ler as posições de figuras do `kind_index` e consultar o `figure_label_numbers` (ou `resolved_labels`).

**Sonda:** verificar qual é o mecanismo que `outline.rs` usa para saber o page number de uma entry do TOC — se é via `known_page_numbers` + label, ou via posição no `kind_index`.

### A.3 — Layout de LoF/LoT com page numbers

**Ficheiro:** `rules/layout/outline.rs` — arm `OutlineTarget::Figures`

Actualmente (P472):

```rust
OutlineTarget::Figures => {
    for (n, caption) in introspector.figure_captions() {
        let entry = format!("Figure {}  {}", n, caption);
        self.layout_content(&Content::text(entry));
        self.layout_newline();
    }
}
```

Com page numbers:

```rust
OutlineTarget::Figures => {
    for (label, n, caption) in introspector.figure_captions_with_label() {
        let page_num = self.runtime.known_page_numbers
            .get(&label)
            .copied()
            .unwrap_or(0);  // 0 = não resolvido ainda (primeira iteração)
        let entry = if page_num > 0 {
            format!("Figure {}  {} . . . {}", n, caption, page_num)
        } else {
            format!("Figure {}  {}", n, caption)  // sem page num na 1ª iteração
        };
        self.layout_content(&Content::text(entry));
        self.layout_newline();
    }
}
```

**Convergência:** na 1ª iteração, `known_page_numbers` está vazio → entries sem page num. Na 2ª iteração, `extracted_label_pages` do layout da 1ª iteração alimenta `known_page_numbers` → entries com page num. Na 3ª iteração (se stable), page numbers iguais → convergência.

### A.4 — `figure_captions_with_label` no `Introspector`

O `Introspector` precisa de expor labels de figuras junto com os seus números e captions. Verificar se `figure_captions()` já existe (P472) e se inclui o label.

Se não inclui label, adicionar ao `BibStore` ou ao `Introspector`:

```rust
// Trait Introspector:
fn figure_captions_with_label(&self) -> Vec<(Label, usize, String)>;
```

### A.5 — Activação do fixpoint para LoF

A condição de activação do loop fixpoint deve incluir documentos com `#lof()` ou `#lot()`. A sonda determina como estender a condição sem alterar o comportamento de documentos sem LoF/LoT.

---

## Sub-item B — Audit final do projecto

### B.1 — Estado das trilhas pós-P486

| Trilha | Estado |
|--------|--------|
| 1 — Numeração | COMPLETA |
| 2 — Referências cruzadas | COMPLETA |
| 3 — Selectors | COMPLETA |
| 4 — Visuais | COMPLETA |
| 5 — Shaping | COMPLETA (P482–P486) |
| 6 — Bibliografia Fase 2 | 4/5 → **5/5 pós-P487** |
| 7 — Layout multi-região | COMPLETA |
| 8 — Refinos stdlib | COMPLETA |
| 9 — DEBT-2 | FECHADA |

### B.2 — ADR-0120 anotação final

Anotar em ADR-0120 o estado final de Trilha 5:
- Fases 1–4 executadas (P482–P486).
- Scope-outs permanentes: `y_offset` em emit, `smcp` OpenType, `x_advance` exacto vs hmtx, remoção de `FrameItem::Text`.
- `FrameItem::Text` como tipo pré-shaping permanente (ADR-0029 colisão declarada).

### B.3 — Roteiro actualizado

Produzir versão P487 do roteiro com:
- Todas as trilhas no estado final.
- Paridade: 73/73 matches + sentinel P486.
- DEBTs: 0 activos.
- Scope-outs declarados permanentes.
- Recomendação para P488+: expansão corpus RTL; `y_offset`; refinos menores.

---

## Tests

### Sub-item A

- **L2:** `p487_lof_sem_page_numbers_primeira_iteracao` — LoF sem `known_page_numbers` → entries sem page number (sem crash).
- **L2:** `p487_lof_com_known_page_numbers_inclui_pagina` — `known_page_numbers` preenchido → entries com "... 3".
- **L2:** `p487_lot_com_known_page_numbers` — análogo para tabelas.
- **L3 (E2E):** `p487_lof_converge_em_2_iteracoes` — documento com 2 figuras + `#lof()` → após fixpoint, entries têm page numbers correctos.
- **Parity:** `p487_parity_73_73_mantido` — 73/73 matches.

### Sub-item B

Nenhum teste de código. Documentação.

---

## Spec L0

### Sub-item A

- `rules/layout/outline.md` — §P487: page numbers em LoF/LoT via `known_page_numbers`.
- `entities/introspector.md` — `figure_captions_with_label` método.
- `rules/layout/mod.md` — condição fixpoint alargada para LoF/LoT.

### Sub-item B

- Roteiro `roteiro-conclusao-typst-cristalino-atualizado-p487.md` — estado final.
- ADR-0120 — anotação cumulativa P487 (estado final Trilha 5).

---

## Scope-out explícito

- **LoF/LoT com separadores / formatação avançada** — entries são texto simples `"Figure N  Caption . . . P"`. Formatação CSS-like (pontos, alinhamento à direita) é refino futuro.
- **Convergência em mais de 2 iterações** — se page numbers de figuras causam reflow que muda page numbers de outras figuras, pode ser necessária 3ª iteração. `MAX_ITERATIONS = 5` cobre este caso.
- **Figuras sem label explícito** — se figuras não têm `#label(...)`, não aparecem em `extracted_label_pages`. Scope-out; requer que `Content::Figure` registe automaticamente uma label sintética.
- **LoT com tabelas criadas via `#table()`** — figuras de tipo "table" têm label diferente de figuras de tipo "image". Ambos cobertos pelo mesmo mecanismo se `figure_captions_with_label` incluir `kind`.
- **`y_offset` em emit** — Trilha 5 scope-out preservado.

---

## Critério de fecho

- [ ] Sondas: `native_lof` existe; `OutlineTarget::Figures` tratado em `outline.rs`; fixpoint condição de activação; `figure_captions_with_label` (ausente/presente) — todos com `file:line`.
- [ ] `figure_captions_with_label` implementado no `Introspector` (ou mecanismo equivalente).
- [ ] Layout de `OutlineTarget::Figures` usa `known_page_numbers` para page numbers.
- [ ] Condição fixpoint alargada para activar em documentos com LoF/LoT.
- [ ] 4+ testes verdes (3 L2 + 1 L3).
- [ ] `p487_parity_73_73_mantido` verde.
- [ ] Spec L0 actualizada (3 ficheiros).
- [ ] Roteiro actualizado produzido.
- [ ] `cargo test --workspace` verde; `crystalline-lint` zero violations.
- [ ] **Trilha 6: 5/5 COMPLETA.**
- [ ] **Todas as trilhas: COMPLETAS** (1–9 inclusive).

---

## Estado pós-P486 (para referência)

| Indicador | Estado |
|-----------|--------|
| Paridade | 73/73 matches |
| DEBTs activos | 0 |
| Trilhas completas | 1, 2, 3, 4, 5, 7, 8 |
| Trilha 6 | 4/5 — **P487 fecha 5/5** |
| **P487** | LoF/LoT page numbers + audit final | 🔄 EM PREPARAÇÃO |
