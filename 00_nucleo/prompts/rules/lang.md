# Prompt L0 — Regras lang-aware (`rules/lang/`)
Hash do Código: 6664c5f2

## Módulo
`01_core/src/rules/lang/`

## Propósito

Agrupa regras cujo comportamento depende do `text.lang` activo
(per ADR-0057). Materializado em **Passo 155** como parte da Fase 1
do roadmap ADR-0060.

Inicialmente contém apenas `quotes` (smart-quotes lang-aware).
Hyphenation continua em `rules/layout/hyphenation.rs` — refactor
de unificação adiado a passo separado se priorizado.

---

## `rules/lang/quotes.rs` — Smart-quotes (Passo 155)

### Função pública

```rust
pub fn localize_quotes(lang: &Lang) -> (&'static str, &'static str);
pub const DEFAULT_QUOTES: (&str, &str) = ("\"", "\"");
```

Devolve par `(open, close)` para o `Lang` dado. Lookup por exact
match no código ISO 639-1/2/3 (Lang em cristalino é 2-3 letras
ASCII puro per ADR-0052; sem region/country como `pt-BR`).

Línguas não cobertas → `DEFAULT_QUOTES` (ASCII).

### Tabela inicial (6 idiomas + default)

| Lang | Open | Close |
|------|------|-------|
| `pt` | `«` (U+00AB) | `»` (U+00BB) |
| `en` | `"` (U+201C) | `"` (U+201D) |
| `de` | `„` (U+201E) | `"` (U+201C) |
| `fr` | `« ` (com U+00A0 NBSP) | ` »` (com U+00A0 NBSP) |
| `es` | `«` | `»` |
| `it` | `«` | `»` |
| (default) | `"` ASCII | `"` ASCII |

### Pontos de consumo

1. **Layouter** (`rules/layout/mod.rs`) em `Content::Quote { quotes:
   true, .. }`: consulta `self.chain.lang()` e aplica `localize_quotes`
   antes de renderizar body.

2. **Eval markup** (`rules/eval/mod.rs::eval_markup`) em
   `SyntaxKind::SmartQuote`: cristalino's lexer produz 1 token por
   `"` ou `'` (per-character). O eval mantém estado de alternância
   open/close dentro de cada sequence markup, e emite o glyph
   correspondente como `Content::Text`.

### Critérios de verificação

- `localize_quotes(Lang::from_str("pt").unwrap()) == ("«", "»")`.
- `localize_quotes(Lang::from_str("en").unwrap()) == ("\u{201C}", "\u{201D}")`.
- `localize_quotes(Lang::from_str("de").unwrap()) == ("\u{201E}", "\u{201C}")`.
- `localize_quotes(Lang::from_str("fr").unwrap())` contém NBSP em ambos.
- `localize_quotes(Lang::from_str("jp").unwrap()) == DEFAULT_QUOTES`.
- `localize_quotes(Lang::from_str("por").unwrap()) == DEFAULT_QUOTES`
  (3-letter ISO; `por` ≠ `pt`; cai em default).

### Limitações registadas

- Sem aspas secundárias (`'...'` em markup produz `'` ASCII).
- Sem smart-apostrophes (`'` em meio de palavra).
- Sem aspas aninhadas com alternância primary/secondary.
- 6 idiomas iniciais; outras línguas (zh, ja, ar, ...) caem em
  default ASCII. Expansível em passo futuro sem breaking change.

### Decisões registadas

- **Localização do módulo**: `rules/lang/quotes.rs` (módulo novo).
  Hyphenation permanece em `rules/layout/` por ora — refactor
  unificador adiado.
- **Lookup**: exact match. Cristalino's `Lang` é 2-3 letras ASCII
  puro (ADR-0052) — sem necessidade de prefix-match BCP47.
- **Tabela estática** `&'static [(&str, (&str, &str))]` com lookup
  linear (6 entries → trivial; sem cache).
