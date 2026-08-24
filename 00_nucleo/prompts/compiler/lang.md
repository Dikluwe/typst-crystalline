# Prompt L0 — Regras lang-aware (`rules/lang/`)
Hash do Código: 8828444b

## Módulo
`01_core/src/compiler/lang/`

## Propósito

Agrupa regras cujo comportamento depende do `text.lang` activo
(per ADR-0057). Materializado em **Passo 155** como parte da Fase 1
do roadmap ADR-0060.

Inicialmente contém apenas `quotes` (smart-quotes lang-aware).
Hyphenation continua em `compiler/layout/hyphenation.rs` — refactor
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

1. **Layouter** (`compiler/layout/mod.rs`) em `Content::Quote { quotes:
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
  Hyphenation permanece em `compiler/layout/` por ora — refactor
  unificador adiado.
- **Lookup**: exact match. Cristalino's `Lang` é 2-3 letras ASCII
  puro (ADR-0052) — sem necessidade de prefix-match BCP47.
- **Tabela estática** `&'static [(&str, (&str, &str))]` com lookup
  linear (6 entries → trivial; sem cache).

---

## P1034 — o default da linguagem é `en`, e vive nos sítios que geram texto

**Achado 6 do P1031**, medido contra o vanilla ratificado (`a51e02804`) em 2026-08-13:
um documento sem `#set text(lang:)` produzia `Figura`/`Índice` no cristalino e
`Figure`/`Contents` no vanilla. Com `#set text(lang: "en")` explícito, coincidiam — logo o
defeito era do **default**, não do caminho de localização.

### Causa medida, e não é ambiente de build

`figure_supplement.rs` tinha `DEFAULT_SUPPLEMENTS_PT` e a razão estava escrita no próprio
código (P158B §2/§8.2): *"usa PT (não EN) para preservar backwards compat com tests
pré-existentes que esperam 'Figura'"*. Isto é, o fallback estava alinhado com os **testes**,
não com a linguagem. O título do outline era pior: `"Índice"` **fixo** em
`layout/outline.rs`, sem consultar língua nenhuma — e o vanilla, mesmo em `pt`, diz
`Sumário`.

### Decisão

O fallback de língua ausente ou desconhecida passa a **`en`**, nos dois sítios:

1. `figure_supplement::DEFAULT_SUPPLEMENTS_EN` — `image`→`Figure`, `table`→`Table`,
   `raw`→`Listing`.
2. `outline_title::outline_title_for_lang` (**módulo novo**) — tabela medida contra o
   vanilla, `#set text(lang: X)` + `#outline()`, texto extraído do PDF:

   | lang | vanilla | | lang | vanilla |
   |---|---|---|---|---|
   | en | Contents | | es | Índice |
   | pt | Sumário | | it | Indice |
   | de | Inhaltsverzeichnis | | zh | 目录 |
   | fr | Table des matières | | | |

   Fallback `en` = `Contents`. Confirmado no vanilla que uma língua sem localização
   (`lang: "jp"`) cai em **inglês**, não na língua do ambiente — medido também para o
   supplement de figura (`#set text(lang: "jp")` → `Figure 1`).

### Onde o default **não** vai, e porquê (medição que refutou a primeira tentativa)

A via aparentemente óbvia — `StyleChain::lang()` devolver `Some(Lang::ENGLISH)` em vez de
`None` — foi tentada e **refutada por medição**: liga a hifenização em todos os documentos.
`compiler/layout/cursor.rs:137` decide hifenizar **só** por `style.lang` ser `Some`, sem
qualquer porta de `justify`/`hyphenate`. Medido: `The extraordinary characteristics of this
remarkable phenomenon.` numa coluna de 100pt sem `#set text(lang:)` → vanilla **0** hífenes,
cristalino **3** com o default na chain.

Logo `StyleChain::lang()` mantém `None` = "não definido", que é o estado verdadeiro, e o
default de língua aplica-se em quem **gera texto**. Mover o default para a chain exige
primeiro corrigir a porta de hifenização — **passo próprio**, com medição do par
`justify`/`hyphenate` do vanilla.

### Consequência nos testes

Quatro testes fixavam o comportamento antigo e passam a esperar o novo
(`fallback_lang_none_devolve_en`, `fallback_lang_desconhecido_devolve_en`,
`figure_label_lang_unknown_fallback_en`, e os três de outline em `layout/tests.rs`). Um
quinto, `ref_supplement_explicit_overrides_default`, **passava por acidente**: procurava
`"Figura 1"`, que existia na *legenda* (supplement por defeito PT), não na referência; com o
default em EN a legenda diz `Figure 1` e o teste passou a testar o que diz testar.

## P1140.4-C — nome local de equação

### Medição antes da decisão

Probes no vanilla pinado em 2026-08-24 mediram o suplemento automático:
`en Equation`, `pt Equação`, `de Gleichung`, `fr Équation`, `es Ecuación`,
`it Equazione`. A língua é a capturada na equação; ausência usa inglês.

### Decisão

Adicionar helper atomizado `equation_supplement_for_lang` ao módulo de língua,
com a tabela medida e fallback inglês para língua ausente/desconhecida. O
helper é puro, recebe `Option<&Lang>` e não conhece introspecção ou layout.
