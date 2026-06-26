# Prompt L0 — `entities/citation_style`
Hash do Código: 20c3966f

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

Default é `Numeric` — diferença deliberada face ao vanilla (que não tem um
default fixo; depende de `bibliography.style`). Cristalino usa Numeric
como default universal para citar sem especificar style explícito.

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
