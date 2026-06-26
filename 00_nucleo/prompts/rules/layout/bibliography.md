# Prompt L0 — `rules/layout/bibliography` — Layout de Bibliography e Cite (fallback numérico)
Hash do Código: 1350c0e0

**Camada**: L1
**Ficheiros alvo**: `01_core/src/rules/layout/bibliography.rs`, `01_core/src/rules/layout/cite.rs`
**Criado em**: 2026-06-25 (P468 — Bibliography Phase 2: numeric citation style)
**ADRs**: ADR-0107 (paridade linguagem), ADR-0108 (anti-deriva), ADR-0109 (atomização forma B)

---

## Propósito

Este prompt cobre o **fallback local** de renderização de citações e
bibliografia no cristalino — o caminho que corre quando `bib_render_cache`
não está disponível (style não especificado ou hayagriva não activo).

P468 adiciona suporte ao estilo numérico como default: `[1]`, `[2]`
ordenados por primeira aparição no documento.

---

## `rules/layout/bibliography.rs`

### Comportamento P468 — fallback numérico

Quando `bib_render_cache` não está disponível:

1. Obtém `citation_order: Vec<String>` via `layouter.introspector.citation_order()`.
2. Ordena as entries por posição em `citation_order` (ordem de primeira citação).
   Entries não citadas ficam no final, na ordem original.
3. Numera sequencialmente: `n = idx + 1` (1-based).
4. Formata cada entry como `[N] <body>` onde `<body>` é gerado por `format_bib_entry_body(e)`.

```rust
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    b: &BibliographyElem,
) {
    // title opcional
    if let Some(t) = &b.title { layouter.layout_content(t); layouter.flush_line(); }
    // CSL cache path
    if let Some(bib_content) = layouter.bib_render_cache.as_ref().and_then(|c| c.bibliography.clone()) {
        layouter.layout_content(&bib_content);
        return;
    }
    // Fallback numérico P468
    let citation_order = layouter.introspector.citation_order().to_vec();
    let mut ordered: Vec<_> = b.entries.iter().collect();
    ordered.sort_by_key(|e| {
        citation_order.iter().position(|k| k == &e.key).unwrap_or(usize::MAX)
    });
    for (idx, e) in ordered.iter().enumerate() {
        let n = idx + 1;
        let body = super::format_bib_entry_body(e);
        let line = format!("[{}] {}", n, body);
        layouter.layout_content(&Content::text(line));
        layouter.flush_line();
    }
}
```

---

## `rules/layout/cite.rs` — P468 CitationStyle::Numeric

O fallback local de `cite.rs` agora respeita `CitationStyle`:

```rust
let style = e.style.unwrap_or_default();  // None → Numeric

let numeric_n = |intr: &_| -> String {
    Introspector::citation_number_for_key(intr, key)
        .or_else(|| Introspector::bib_number_for_key(intr, key))
        .map(|n| format!("[{}]", n))
        .unwrap_or_else(|| format!("[{}]", key))
};

match (style, form, entry) {
    (Numeric, Normal, Some(_)) => numeric_n(...),        // [N]
    (Numeric, Normal, None)    => format!("[{}]", key),  // [key] sem bib
    (Numeric, Prose, Some(e))  => format!("{} [{}]", e.author, n),
    (Numeric, Author, Some(e)) => e.author.clone(),
    (Numeric, Year, Some(e))   => e.year.to_string(),
    (_, Normal, _)             => format!("[{}]", key),  // outros styles: placeholder
    (_, Prose, Some(e))        => format!("{} ({})", e.author, e.year),
    (_, Author, Some(e))       => e.author.clone(),
    (_, Year, Some(e))         => e.year.to_string(),
    (_, _, None)               => format!("[{}]", key),
}
```

**Regra de `citation_number_for_key` vs `bib_number_for_key`**:
- `citation_number_for_key`: posição por primeira aparição (P468, preferida).
- `bib_number_for_key`: numeração legado `assign_number` (fallback se ainda não citada).
- Entry `None` + Numeric/Normal → `[key]` (sem bib → sem número).

---

## `rules/layout/mod.rs` — `format_bib_entry_body`

Função auxiliar para formatar o corpo de uma entry sem prefixo `[key]`:

```rust
pub(super) fn format_bib_entry_body(e: &BibEntry) -> String {
    let mut out = format!("{}. {}", e.author, e.title);
    // campos opcionais: year, volume, pages, etc.
    out
}
```

Contrasta com `format_bib_entry` (inclui `[key]` no início — fallback original P159G).

---

## Integração

- `bibliography.rs` usa `layouter.introspector.citation_order()` via trait `Introspector`.
- `cite.rs` usa `layouter.introspector.citation_number_for_key(key)`.
- Ambos obtidos via `Layouter<M, S>` por descendência de módulo (ADR-0109 forma B).

---

## Scope-out

- Ordenação de entries não citadas (actualmente ficam no final, na ordem original).
- Numeração correcta em documentos multi-bibliografia (scope-out para P420).
- Formatação CSL completa (hayagriva). Ver `bib_csl.md`.

---

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-06-25 | P468: fallback numérico em bibliography.rs (ordenação por citation_order); CitationStyle match em cite.rs; format_bib_entry_body em mod.rs | `bibliography.rs`, `cite.rs`, `bibliography.md` |
