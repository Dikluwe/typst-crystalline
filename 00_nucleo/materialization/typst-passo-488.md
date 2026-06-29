---

# P488 — Trilha 6: LoF/LoT page numbers (2-pass) + expansão corpus RTL

> **Passo:** 488
> **Data:** 2026-06-28
> **Foco:** (A) Fechar Trilha 6 5/5: LoF/LoT com page numbers reais via fixpoint TOC existente; (B) expansão do corpus `lab/parity/` com ficheiros RTL (árabe/hebraico) para validar P484.
> **Trilha:** 6 — item final (page numbers) + corpus RTL.
> **Tipo:** Materialização M + S.
> **Tamanho:** M (~40 min sub-item A) + S (~20 min sub-item B).
> **ADR-0072 ACEITE** — TOC fixpoint estruturalmente fechado (P192B). `LayouterRuntimeState.known_page_numbers` disponível.

---

## Contexto

P487-D fechou os bloqueadores operacionais (merge conflicts, stack overflow, sentinelas stale). O estado do projecto é agora:

- `cargo test --workspace` 100% verde.
- `crystalline-lint .` 0 violations.
- Trilha 6: 4/5 — LoF/LoT sem page numbers.
- Trilha 5: completa (P482–P486), mas sem corpus RTL para validar P484.

**Sub-item A:** P487 propôs a abordagem mas P487-D revelou que a tarefa era de housekeeping. O sub-item A de P488 é a implementação real de LoF/LoT com page numbers.

**Sub-item B:** P484 implementou RTL básico via `unicode-bidi`, mas o corpus `lab/parity/` não tem ficheiros árabe/hebraico. Sem corpus, os 6 testes unitários L3 de P484 são a única cobertura. Sub-item B adiciona pelo menos 2 ficheiros de corpus RTL e verifica que o shaper os processa correctamente.

---

## ADR-0108 — Medir antes de decidir

### Sub-item A

| Pergunta | Verificar em | Status |
|----------|-------------|--------|
| `native_lof()` localização e assinatura actual | `rules/stdlib/structural.rs` | 🟡 |
| `OutlineTarget::Figures` arm em `outline.rs` | `rules/layout/outline.rs` | 🟡 |
| `figure_captions_with_label()` existe no Introspector? | `entities/introspector.rs` | 🟡 |
| Figuras têm label registado em `label_pages`? | `rules/layout/references.rs` | 🟡 |
| Condição fixpoint inclui `ElementKind::OutlineFigures`? | `rules/layout/mod.rs:1515` | 🟡 |
| `known_page_numbers` populado para figuras/tabelas? | idem | 🟡 |

### Sub-item B

| Pergunta | Verificar em | Status |
|----------|-------------|--------|
| Corpus actual tem ficheiros com texto árabe/hebraico? | `lab/parity/corpus/` | 🟡 |
| Formato dos ficheiros corpus (`.typ`)? | corpus existentes | ✅ |
| Suite `structural_parity.rs` aceita SKIP para ficheiros sem query matches? | `lab/parity/tests/structural_parity.rs` | 🟡 |

**Todas as sondas com `grep`/`file:line`.**

---

## Sub-item A — LoF/LoT com page numbers

### A.1 — Diagnóstico pré-implementação

Confirmar via sonda:

1. `native_lof()` produz `Content::Outline` com `target: OutlineTarget::Figures` (P472).
2. `outline.rs` arm `Figures` produz entries sem page number (P472 scope-out confirmado).
3. A condição de activação do loop fixpoint — se apenas para `ElementKind::Outline` ou também para outros outline targets.
4. Como figuras são registadas em `label_pages` — se têm label sintético.

### A.2 — Labels sintéticos para figuras

Se figuras não têm label explícito, não aparecem em `label_pages`. Para que o fixpoint funcione para figuras, o walk de introspecção deve registar um label sintético para cada figura:

```rust
// Em rules/introspect.rs — arm Content::Figure:
// P488 — label sintético para figuras (para fixpoint page number resolution)
let synthetic_label = Label::from(format!("__figure_{}", n));
intr.runtime.label_pages.insert(synthetic_label.clone(), current_page);
// Armazenar mapeamento n → synthetic_label para uso no LoF
```

**Alternativa mais simples:** em vez de labels sintéticos, guardar directamente `figure_page_numbers: Vec<(usize, usize)>` (número de figura, número de página) no `Introspector` ou `TagIntrospector`. Não requer o mecanismo de fixpoint — basta que o Layouter L1 registe a página de cada figura durante o layout.

**Decisão (a confirmar via sonda):** se `label_pages` já contém figuras com labels sintéticos, usar o mecanismo existente. Se não, usar `figure_page_numbers` como campo novo no `Introspector`.

### A.3 — `figure_page_numbers` no `Introspector`

Se a abordagem de campo directo é adoptada:

**Ficheiro:** `entities/introspector.rs`

```rust
pub trait Introspector {
    // ... existentes ...
    /// **P488** — Pares (número de figura, página). Populado pelo Layouter durante layout.
    fn figure_page_numbers(&self) -> &[(usize, usize)];
    fn table_page_numbers(&self) -> &[(usize, usize)];

    /// Registar page number de figura (chamado pelo Layouter em runtime).
    fn record_figure_page(&mut self, figure_n: usize, page: usize);
    fn record_table_page(&mut self, table_n: usize, page: usize);
}
```

**Populado em:** arm `Content::Figure` no Layouter (ou no arm de label de figura em `references.rs`).

### A.4 — Layout de LoF com page numbers

**Ficheiro:** `rules/layout/outline.rs` — arm `OutlineTarget::Figures`

```rust
OutlineTarget::Figures => {
    let fig_pages = introspector.figure_page_numbers();
    for (label, n, caption) in introspector.figure_captions_with_label() {
        // Procurar page number
        let page_num = fig_pages.iter()
            .find(|(fn_, _)| *fn_ == n)
            .map(|(_, p)| *p)
            .unwrap_or(0);

        let entry = if page_num > 0 {
            format!("Figure {} {} . . . {}", n, caption, page_num)
        } else {
            format!("Figure {} {}", n, caption)
        };
        self.layout_content(&Content::text(entry));
        self.layout_newline();
    }
}
```

### A.5 — Activação do fixpoint para LoF/LoT

Se o loop fixpoint só activa para `ElementKind::Outline`, adicionar condição OR:

```rust
// layout/mod.rs:1515
if intr.kind_index.contains_key(&ElementKind::Outline)
|| intr.kind_index.contains_key(&ElementKind::OutlineFigures)
|| intr.kind_index.contains_key(&ElementKind::OutlineTables) {
    // fixpoint loop
}
```

Se `ElementKind::OutlineFigures`/`OutlineTables` não existem, adicionar ao enum.

---

## Sub-item B — Corpus RTL

### B.1 — Ficheiros de corpus a criar

**`lab/parity/corpus/rtl/arabic_basic.typ`:**

```typst
#set text(dir: rtl, lang: "ar")

مرحبا بالعالم

#heading[عنوان المستند]

هذا مستند بسيط باللغة العربية لاختبار
تشكيل النص من اليمين إلى اليسار.
```

**`lab/parity/corpus/rtl/hebrew_basic.typ`:**

```typst
#set text(dir: rtl, lang: "he")

שלום עולם

#heading[כותרת המסמך]

זהו מסמך פשוט בעברית לבדיקת
עיצוב הטקסט מימין לשמאל.
```

### B.2 — Integração na suite

Estes ficheiros são classificados como **INCLUDE** mas com query simples (e.g., `"heading"`) — verificam que o processamento não produz panic nem crash. A comparação estrutural com vanilla pode não ser possível se o vanilla render difere do cristalino em texto RTL — nesses casos, usar SKIP-feature com nota de explicação.

**`structural_parity.rs`** — adicionar ao corpus:

```rust
("rtl/arabic_basic.typ", Category::Include, vec!["heading"]),
("rtl/hebrew_basic.typ", Category::Include, vec!["heading"]),
```

Se a suite usa `typst query` no vanilla para comparar, e o vanilla não tiver suporte RTL completo na versão 0.14.2, usar `Category::Skip` com nota.

### B.3 — Critério de sucesso para corpus RTL

Não é paridade completa com vanilla (RTL no cristalino é subset). O critério é:
- Documentos RTL processam sem panic.
- `heading` queries retornam o número correcto de headings.
- Se vanilla retorna estrutura diferente → SKIP-feature com nota.

---

## Tests

### Sub-item A

- **L1:** `p488_introspector_figure_page_numbers_default_vazio` — `TagIntrospector::default().figure_page_numbers().is_empty()`.
- **L1:** `p488_record_figure_page_armazena_par` — após `record_figure_page(1, 3)`, `figure_page_numbers() == [(1, 3)]`.
- **L2:** `p488_lof_sem_pages_primeira_iteracao` — LoF sem `figure_page_numbers` → entries sem page number (sem crash).
- **L2:** `p488_lof_com_pages_inclui_numero` — `figure_page_numbers = [(1, 3), (2, 7)]` → "Figure 1  Caption A . . . 3".
- **L3 (E2E):** `p488_lof_page_numbers_correcto` — documento com 2 figuras + `#lof()` → após layout, entries têm page numbers.
- **Parity:** `p488_parity_incluindo_rtl` — corpus alargado incluindo RTL; todos INCLUDE passam ou SKIP-feature documentado.

### Sub-item B

- Não há novos testes de código — os ficheiros de corpus são os "testes".
- Sentinela `p488_parity_incluindo_rtl` valida corpus com RTL.

---

## Spec L0

### Sub-item A

- `entities/introspector.md` — métodos `figure_page_numbers`, `table_page_numbers`, `record_figure_page`, `record_table_page`.
- `rules/layout/outline.md` — §P488: page numbers em LoF/LoT.
- `rules/layout/mod.md` — condição fixpoint alargada (se aplicável).

### Sub-item B

- `lab/parity/SKIPS.md` — §RTL: estado dos ficheiros árabe/hebraico.

---

## Scope-out explícito

- **LoF/LoT formatação avançada** — pontos de preenchimento, alinhamento à direita de page numbers. Texto simples `"Figure N  Caption . . . P"`.
- **Figuras sem label/caption** — entries de figuras sem caption são omitidas.
- **Page numbers de tabelas em layouts multi-coluna** — `table_page_numbers` pode registar a página do topo da tabela. Célula exacta scope-out.
- **Corpus RTL com múltiplos elementos** — corpus minimal (1–2 headings por ficheiro). Documentos complexos árabe/hebraico scope-out.
- **`dir: rtl` em stdlib** — `#set text(dir: rtl)` pode não estar implementado. Se não, os ficheiros de corpus são SKIP-feature.
- **`y_offset` em emit** — scope-out permanente de P486.

---

## Critério de fecho

- [ ] Sondas sub-item A: `native_lof` localização; `OutlineTarget::Figures` arm; `figure_page_numbers` (ausente); condição fixpoint — todos com `file:line`.
- [ ] `figure_page_numbers` + `table_page_numbers` adicionados ao `Introspector`.
- [ ] `record_figure_page` + `record_table_page` chamados no Layouter.
- [ ] Layout de `OutlineTarget::Figures` usa `figure_page_numbers`.
- [ ] Condição fixpoint alargada (se necessário).
- [ ] 5+ testes verdes sub-item A (2 L1 + 2 L2 + 1 L3).
- [ ] 2 ficheiros de corpus RTL criados.
- [ ] Sentinela `p488_parity_incluindo_rtl` verde (ou SKIP-feature documentado).
- [ ] Spec L0 actualizada (3 ficheiros sub-A + 1 sub-B).
- [ ] `cargo test --workspace` verde; `crystalline-lint` zero violations.
- [ ] **Trilha 6: 5/5 COMPLETA** (se implementação convergir).
- [ ] **Todas as trilhas: COMPLETAS** (1–9).

---

## Próximo passo (P489)

Com P488, o projecto atinge o estado de conclusão do roteiro original:

| Indicador | Estado esperado pós-P488 |
|-----------|--------------------------|
| DEBTs activos | 0 |
| Todas as trilhas | COMPLETAS |
| Paridade | ≥73/73 matches (corpus alargado) |
| Corpus RTL | 2 ficheiros adicionados |

P489 candidatos:

| Opção | Descrição | Magnitude |
|-------|-----------|-----------|
| **P489-A** | `y_offset` em emit PDF (diacríticos RTL) | S |
| **P489-B** | Roteiro final e audit de consolidação | XS |
| **P489-C** | Expansão corpus para ≥60 ficheiros | M |
| **P489-D** | `smcp` via OpenType real | S |

---

## Estado pós-P487-D (para referência)

| Indicador | Estado |
|-----------|--------|
| `cargo test --workspace` | ✅ 3429+526+47 verde |
| `crystalline-lint .` | ✅ 0 violations |
| Paridade | 73/73 matches |
| DEBTs activos | 0 |
| Trilha 6 | 4/5 — **P488 fecha 5/5** |
| **P488** | LoF/LoT page numbers + corpus RTL | 🔄 EM PREPARAÇÃO |
