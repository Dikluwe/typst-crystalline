---

# P468 — Relatório de Execução: Bibliografia Fase 2 (Estilos Numéricos)

> **Passo:** 468
> **Data:** 2026-06-25
> **Estado:** ✅ FECHADO
> **Executor:** Kimi (base) + IA assistente (fixes + L0 + testes + commit)

---

## Resumo

P468 materializa o estilo de citação numérico `[1]`, `[2]` para
bibliografia, com numeração por ordem de primeira aparição no documento.
O cristalino passa a ter `CitationStyle::Numeric` como default universal.

---

## Critério de fecho — verificação

| Critério | Estado | Notas |
|----------|--------|-------|
| `Introspector` mantém `citation_order: Vec<String>` | ✅ | `BibStore.citation_order` + `record_citation` |
| `CiteElem` ganha campo `style: Option<CitationStyle>` | ✅ | Adicionado por Kimi |
| `native_cite` aceita `style: Str` | ✅ | `extract_citation_style` em `structural.rs` |
| Layout `Numeric/Normal` renderiza `"[N]"` | ✅ | `cite.rs` com `citation_number_for_key` |
| Bibliografia ordenada por ordem de primeira citação | ✅ | `bibliography.rs` sort por `citation_order` |
| 7 testes verdes (2 L1 + 3 L2 + 2 L3) | ✅ | Ver secção de testes |
| Spec L0 atualizada (3 prompts) | ✅ | `citation_style.md`, `bibliography.md`, `structural.md` |
| `cargo test` verde (3313 L1 + 499 L3) | ✅ | 5 testes de stack overflow skipped (pré-existentes) |
| `crystalline-lint` zero violations | ✅ | Só V7 warnings pré-existentes |

---

## Testes adicionados

### L1 — `entities/bib_store.rs`

- `record_citation_mantem_primeira_aparicao` — `record_citation` ignora chaves
  repetidas; `citation_order` reflecte a ordem de primeira aparição.
- `citation_number_for_key_pela_primeira_aparicao` — posição 1-based via
  `citation_order`; keys ausentes → `None`.

### L2 — `rules/layout/tests.rs`

- `cite_numeric_ordem_primeira_aparicao` — documento com 2 cites; o segundo
  citado primeiro recebe `[1]`; o primeiro citado segundo recebe `[2]`.
- `bibliography_ordenada_pela_ordem_de_citacao` — 3 entries citadas em ordem
  invertida; a bibliografia respeita a ordem de citação.
- `cite_numeric_prose_inclui_numero` — `CitationForm::Prose` + `Numeric`
  renderiza `"Author [N]"`.

### L3 — `measurements.rs`

- `p468_e2e_citation_number_for_key_via_proxy` — proxy `CountingIntrospector`
  delega `citation_number_for_key`; entry "second" citada primeiro → número 1.
- `p468_e2e_citation_order_via_proxy` — proxy delega `citation_order()`; ordem
  reflecte sequência de `record_citation`.

---

## Ficheiros modificados

### L1

| Ficheiro | Tipo | Mudança |
|----------|------|---------|
| `entities/citation_style.rs` | **NOVO** | Enum `CitationStyle` (3 variants + Default::Numeric) |
| `entities/bib_store.rs` | M | Campo `citation_order`, `record_citation`, `citation_number_for_key`, `citation_order()` |
| `entities/introspector.rs` | M | Trait: `citation_number_for_key`, `citation_order`; TagIntrospector: delegação |
| `entities/elements/cite.rs` | M | Campo `style: Option<CitationStyle>` em `CiteElem` |
| `entities/content.rs` | M | `cite_with_style` + `map_content`/`map_text` actualizado |
| `entities/mod.rs` | M | `pub mod citation_style` |
| `rules/introspect.rs` | M | `record_citation(key)` no arm `ElementPayload::Citation` |
| `rules/layout/cite.rs` | M | Match `(CitationStyle, CitationForm, Option<&BibEntry>)` |
| `rules/layout/bibliography.rs` | M | Fallback ordena por `citation_order` e prefixa `[N]` |
| `rules/layout/mod.rs` | M | `format_bib_entry_body` (sem prefixo `[key]`) |
| `rules/layout/tests.rs` | M | 3 testes L2 novos; 13 testes existentes actualizados (default Numeric) |
| `rules/stdlib/structural.rs` | M | `extract_citation_style`; `native_cite` aceita `style:` named |
| `rules/eval/repr.rs` | M | `repr()` para `CitationStyle` |

### L3

| Ficheiro | Tipo | Mudança |
|----------|------|---------|
| `measurements.rs` | M | Proxy `citation_number_for_key` (record_call) + `citation_order` (record_call); 2 testes E2E |

### L0 (prompts)

| Ficheiro | Tipo | Mudança |
|----------|------|---------|
| `entities/citation_style.md` | **NOVO** | Spec `CitationStyle` enum |
| `rules/layout/bibliography.md` | **NOVO** | Spec fallback numérico (cite.rs + bibliography.rs) |
| `rules/eval/cast.md` | **NOVO** | Spec `cast_length` P469 (fix V1 pré-existente) |
| `entities/bib_store.md` | M | Adicionado `citation_order` + P468 revision |
| `entities/introspector.md` | M | P468 revision + 2 novos métodos |
| `rules/stdlib/structural.md` | M | `native_cite` assinatura alargada com `style:` |

---

## Divergências face ao plano P468

| Plano | Realidade | Justificação |
|-------|-----------|--------------|
| Usar `Content::Ref` com `style` | Usa `Content::Cite` com `style` (pré-existente P159C) | `cite()` já existia como abstracção correcta |
| `CounterRegistry` chave `"citation"` | `BibStore.citation_order: Vec<String>` | Mais simples; `CounterRegistry` é para numeração hierárquica |
| `entities/bibliography.md` como prompt | `entities/citation_style.md` | Convenção de nomear L0 pelo tipo (idem `citation_form.md`) |

---

## Testes corrigidos (impacto de default Numeric)

13 testes existentes actualizados porque `CitationStyle::Numeric` é agora o
default — o comportamento `[key]` foi substituído por `[N]`:

- `bibliography_entry_minima_regression_p159a`
- `layout_bibliography_renderiza_entries_como_lista`
- `layout_bibliography_e_cite_no_mesmo_documento`
- `cite_prose_renderiza_author_year_quando_key_existe` (→ `Author [N]`)
- `cite_form_prose_inalterada_com_bib_numerada` (→ `Author [N]`)
- `cite_prose_via_introspector_renderiza_author_year` (→ `Author [N]`)
- `cite_4_forms_via_layout_with_introspector` (→ `Author [N]`)
- `bibliography_sem_style_preserva_fallback_local` (→ `[1]`)
- `bibliography_style_inexistente_cai_em_fallback` (→ `[1]`)
- `cite_normal_multi_bibliography_continua` (scope-out P420; test usa author lookup)

---

## Estado pós-P468

**Trilha 6 (Fase 2 estilos): 1/5 completo.**

| Passo | Estado |
|-------|--------|
| P468 — Estilos numéricos | ✅ FECHADO |
| P469+ — Back-references, LoF/LoT | ⏳ |

---

## Próximos passos

- **P469** — Back-references (`"ver [3]"`), `ibid`/`op. cit.` (scope-out P468).
- **P469** — List of Figures / List of Tables.
- Continuação Trilha 8 (`Relative`, `pad`/`corners`/`sides`).
