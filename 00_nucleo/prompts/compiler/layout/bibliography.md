# Prompt L0 — `compiler/layout/bibliography` — Layout de Bibliography e Cite (fallback numérico)
Hash do Código: 97f01adf

**Camada**: L1
**Ficheiros alvo**: `01_core/src/compiler/layout/bibliography.rs`, `01_core/src/compiler/layout/cite.rs`
**Criado em**: 2026-06-25 (P468 — Bibliography Phase 2: numeric citation style)
**ADRs**: ADR-0107 (paridade linguagem), ADR-0108 (anti-deriva), ADR-0109 (atomização forma B)

---

## Propósito

Este prompt cobre o **fallback local** de renderização de citações e
bibliografia no cristalino — o caminho que corre quando `bib_render_cache`
não está disponível (style não especificado ou hayagriva não activo).

P468 adiciona suporte ao estilo numérico como default: `[1]`, `[2]`
ordenados por primeira aparição no documento.

> **P1031 — natureza deste documento e achado escalado.**
>
> **Este fallback não tem correspondente na linguagem Typst.** O vanilla não tem caminho
> "sem CSL": omitir `style` significa o estilo `"ieee"`, não um renderizador próprio. Fonte:
> `crates/typst-library/src/model/bibliography.rs:159-163` —
> `#[default({ let default = ArchivedStyle::InstituteOfElectricalAndElectronicsEngineers; … })]`
> sobre `pub style: Derived<CslSource, CslStyle>`; doc comment da tabela de estilos em
> `bibliography.rs:79-95`. Página: `typst.app/docs/reference/model/bibliography/#parameters-style`.
> Logo as regras deste prompt são **decisões de implementação do cristalino**, não paridade —
> e as afirmações que soavam a "o vanilla faz assim" foram medidas e corrigidas abaixo.
>
> **Medição directa (2026-08-13)** — vanilla `/usr/local/bin/typst` (`typst 0.15.1
> (e0e8ca4d)`) vs cristalino `target/release/typst` (fonte em HEAD `4f64e4e69`, árvore de
> trabalho só com edições em `00_nucleo/prompts/**`). Documento:
> `A @netwok B @netwok C @other D @netwok` + `#bibliography("works.bib")`, com dois `@book`
> em `works.bib`.
>
> | Caso | Vanilla | Cristalino |
> |---|---|---|
> | `#bibliography("works.bib")` (sem `style`) | `A [1] B [1] C [2] D [1]` + entradas `[1] J. Doe, At what cost. Fake Press, 2020.` | `A [1] B ibid. C [2] D [1] Doe, op. cit.` + entradas `[1] Doe, Jane. At what cost. Fake Press (2020). ↑[1][2][4]` |
> | `#bibliography("works.bib", style: "ieee")` | idem acima | **byte-idêntico ao vanilla** |
>
> **Conclusão da medição**: o caminho CSL real (hayagriva) do cristalino **já bate com o
> vanilla**. A divergência inteira vem de o cristalino **não aplicar o default `"ieee"`
> quando `style` é omitido** — cai neste fallback em vez de resolver o estilo por defeito.
>
> **ACHADO ESCALADO (prioridade alta)**: aplicar `"ieee"` como default de
> `bibliography.style` faria o caso por omissão coincidir com o vanilla e tornaria este
> fallback inalcançável no uso normal. É mudança de comportamento por defeito → gate
> ADR-0127 e passo próprio. **Não implementado aqui.**

---

## `compiler/layout/bibliography.rs`

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
        // **P472** — back-refs: lista de posições onde a entry foi citada.
        let refs = layouter.introspector.back_refs_for_key(&e.key);
        let back_ref_str = if refs.is_empty() {
            String::new()
        } else {
            let cited: String = refs.iter().map(|p| format!("[{}]", p)).collect::<Vec<_>>().join("");
            format!(" ↑{}", cited)
        };
        let line = format!("[{}] {}{}", n, body, back_ref_str);
        layouter.layout_content(&Content::text(line));
        layouter.flush_line();
    }
}
```

---

## `compiler/layout/cite.rs` — P472 ibid.

Quando `CitationStyle::Numeric` + `CitationForm::Normal` e a key é idêntica à
última citada, o layouter emite `ibid.` em vez do número:

```rust
let is_ibid = style == CitationStyle::Numeric
    && form == CitationForm::Normal
    && layouter.last_cited_key.as_deref() == Some(key.as_str());
layouter.last_cited_key = Some(key.clone());
if is_ibid {
    layouter.layout_content(&Content::text("ibid.".to_string()));
    if let Some(s) = &e.supplement { layouter.layout_content(s); }
    return;
}
```

`last_cited_key: Option<String>` é campo de `Layouter` (inicializado `None`;
actualizado em cada citação, incluindo no caminho CSL cache).

**Scope-out**: ibid. apenas para `Numeric + Normal`; outros styles e forms não usam ibid.

> **P1031 — `ibid.` é invenção do cristalino, não comportamento do Typst.**
>
> Medido no mesmo documento da secção "Propósito": a repetição adjacente da mesma chave
> rende `[1]` no vanilla e `ibid.` no cristalino. Nenhum estilo CSL embutido do Typst
> substitui a citação numérica repetida por `ibid.`; o estilo por defeito (`"ieee"`,
> `bibliography.rs:159-163`) repete o número. Com `style: "ieee"` explícito o cristalino
> também rende `[1]` — ou seja, este ramo só é alcançado pelo fallback.
>
> Fica registado como **decisão de implementação sem base na linguagem**, não como
> paridade. Se o achado escalado (aplicar o default `"ieee"`) for implementado, este ramo
> passa a inalcançável e deve ser removido em vez de mantido.

---

## `compiler/layout/cite.rs` — P468 CitationStyle::Numeric

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

> **P1031 — a tabela acima é do fallback; nenhuma linha é paridade de linguagem.**
>
> As formatações `Author (Year)`, `Author [N]`, autor-só e ano-só **não** são o que os
> estilos CSL do Typst produzem; são a aproximação do fallback. O comportamento de
> linguagem correspondente é o do estilo CSL activo (`"ieee"` por defeito —
> `crates/typst-library/src/model/bibliography.rs:159-163`), e o cristalino já o reproduz
> byte-a-byte quando `style` é explícito (medição na secção "Propósito").
>
> **A última linha é uma divergência medida, não uma decisão neutra.** Citar uma chave sem
> bibliografia no documento:
>
> | Entrada | Vanilla ratificado | Cristalino |
> |---|---|---|
> | `A #cite(<x>) B` | `error: the document does not contain a bibliography` | — (`cite` não aceita label; ver `entities/elements/cite.md` §P1031) |
> | `A #cite("netwok") B` | `error: expected label, found string` | compila; renderiza `A [netwok] B` |
>
> Medição de 2026-08-13, vanilla `/usr/local/bin/typst` (`typst 0.15.1 (e0e8ca4d)`) vs
> cristalino compilado da fonte em HEAD `4f64e4e69` (árvore só com edições em
> `00_nucleo/prompts/**`). O vanilla **erra**; o cristalino **produz texto**. A regra
> "Entry `None` → `[key]`" é, portanto, uma falha silenciosa face à linguagem, não um
> fallback benigno. Fica ligada ao mesmo achado escalado da secção "Propósito".

---

## `compiler/layout/mod.rs` — `format_bib_entry_body`

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
| 2026-06-26 | P472: back-refs em bibliography.rs (` ↑[1][3]` via `back_refs_for_key`); ibid. em cite.rs (`last_cited_key` no Layouter + detecção Numeric+Normal) | `bibliography.rs`, `cite.rs`, `bibliography.md` |
