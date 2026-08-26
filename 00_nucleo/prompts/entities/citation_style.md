# Prompt L0 — `entities/citation_style`
Hash do Código: bc87d8b4

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/citation_style.rs`
**Criado em**: 2026-06-25 (P468 — Bibliography Phase 2: numeric citation style)
**ADRs relevantes**: ADR-0037 (coesão por domínio), ADR-0054 (graded scope-out), ADR-0107 (paridade linguagem)

---

## Propósito

`CitationStyle` é enum entity que representa o estilo de formatação
bibliográfica. Distinto de `CitationForm` (que controla a *forma* da
citação — normal/prose/author/year); `CitationStyle` controla o *sistema*
de citação (numérico, autor-data, alfabético).

Adicionado em P468 (Bibliography Phase 2) para suportar citação numérica
`[1]`, `[2]` ordenada por primeira aparição no documento.

## Paridade

Vanilla Typst tem `CitationStyle` como enum derivado de `CslStyle`
(hayagriva). Cristalino usa subset de 3 variants universais independentes
de hayagriva:

- `Numeric` — `[1]`, `[2]` (default cristalino).
- `AuthorDate` — `(Kirsch, 1973)`.
- `Alphabetic` — `[Kir73]`.

Default é `Numeric` — diferença deliberada face ao vanilla. Cristalino usa Numeric
como default universal para citar sem especificar style explícito.

> **Correcção P1031 — o vanilla *tem* default fixo.**
>
> A redacção anterior dizia que o vanilla *"não tem um default fixo; depende de
> `bibliography.style`"*. Medido e refutado: `bibliography.style` **tem** default, e é
> `"ieee"`. Citação literal do vanilla ratificado (`e0e8ca4d`),
> `crates/typst-library/src/model/bibliography.rs:159-163`:
>
> ```rust
> #[default({
>     let default = ArchivedStyle::InstituteOfElectricalAndElectronicsEngineers;
>     Derived::new(CslSource::Named(default, None), CslStyle::from_archived(default))
> })]
> pub style: Derived<CslSource, CslStyle>,
> ```
>
> Página publicada: `typst.app/docs/reference/model/bibliography/#parameters-style`; o doc
> comment da tabela de estilos (`bibliography.rs:79-95`) lista `{"ieee"}` como o estilo
> típico de *"Engineering, IT"*.
>
> **Consequência**: `Numeric` como default do cristalino não é "preencher um vazio do
> vanilla" — é substituir um default que existe. O output difere: medido em 2026-08-13,
> `#bibliography("works.bib")` sem `style` dá `A [1] B [1] C [2] D [1]` no vanilla e
> `A [1] B ibid. C [2] D [1] Doe, op. cit.` no cristalino; com `style: "ieee"` explícito os
> dois binários coincidem byte-a-byte. Detalhe e escalonamento em
> `compiler/layout/bibliography.md` §"Propósito" (achado escalado: aplicar o default
> `"ieee"`).
>
> **Sobre `CitationStyle` ser "enum derivado de `CslStyle`"**: no vanilla não existe um tipo
> `CitationStyle`; o que existe é `CitationForm` (`cite.rs:133-147`, ortogonal — ver
> `entities/citation_form.md`) e o estilo CSL resolvido em `Derived<CslSource, CslStyle>`
> (`bibliography.rs:163`). `CitationStyle` é **construto do cristalino** para o fallback
> local, não um espelho de um tipo vanilla — a frase de paridade acima foi reescrita em
> conformidade.

## Representação

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CitationStyle {
    /// Estilo autor-data, ex.: `(Kirsch, 1973)`.
    AuthorDate,
    /// Estilo numérico, ex.: `[1]`, `[2]`.
    Numeric,
    /// Estilo alfabético, ex.: `[Kir73]`.
    Alphabetic,
}

impl Default for CitationStyle {
    fn default() -> Self { Self::Numeric }
}
```

## Interface pública

```rust
impl CitationStyle {
    pub fn as_str(&self) -> &'static str;
}
```

`as_str()` devolve strings lowercase canonical:
- `AuthorDate` → `"author-date"`
- `Numeric` → `"numeric"`
- `Alphabetic` → `"alphabetic"`

## Uso

- `CiteElem { style: Option<CitationStyle> }` — P468.
- `extract_citation_style` em `rules/stdlib/structural.rs` parseia `style:
  Str` named de `cite()` via strict matching (case-sensitive).
- `layout/cite.rs` resolve `e.style.unwrap_or_default()` para o arm de
  match do render.
- `Default` implementado explicitamente como `Numeric` — permite
  `unwrap_or_default()` em layout.

## Scope-out

- Variants adicionais de style CSL (específicas de hayagriva).
- Integração com `bibliography.style` para propagação automática de style.
- Serialização/desserialização completa.

## Critérios de verificação

- `CitationStyle::default()` == `CitationStyle::Numeric`.
- `CitationStyle::Numeric.as_str()` == `"numeric"`.
- `CitationStyle::AuthorDate.as_str()` == `"author-date"`.
- `CitationStyle::Alphabetic.as_str()` == `"alphabetic"`.
- `PartialEq`: variants diferentes não são iguais.

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-06-25 | P468: enum CitationStyle com 3 variants + Default::Numeric | `citation_style.rs`, `citation_style.md` |
