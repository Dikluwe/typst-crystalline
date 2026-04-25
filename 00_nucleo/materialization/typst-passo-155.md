# Passo 155 — `quote` (Fase 1 Model, segunda sub-fase; fecha ADR-0060)

**Série**: 155 (passo **substantivo**; segunda materialização
da Fase 1 do roadmap ADR-0060; fecha Fase 1 inteira).
**Precondição**: Passo 154B encerrado; ADR-0060 PROPOSTO
com Fase 1 primeira sub-fase materializada (`terms` +
`divider`); 1123 tests; zero violations; 60 ADRs; 13 DEBTs
abertos; cobertura Model 41% (10/22); cobertura arquitectural
Content 75% (42/56 estima).

**Numeração**: 155 segue 154B no padrão de Fase 1
(154B → 155 = duas sub-fases). Após 155, ADR-0060 transita
**`PROPOSTO → IMPLEMENTADO`**. Fase 2 abre depois.

**Natureza**: passo **substantivo escopo L** (decisão humana;
maior que P154B M). Toca:
- L1 (`01_core/`): novo variant `Content::Quote`; cobertura
  exaustiva de arms; `native_quote`; possível mecanismo de
  smart-quotes (interacção com `text.lang` per ADR-0057).
- L0 (prompts): spec do variant + smart-quotes.
- Parser cristalino: regra nova para `"..."` em contexto
  markup (distinto de string literal em código).
- Testes em `01_core/src/rules/` cobrindo construção,
  comparação, `plain_text`, eval do construtor, parse
  markup, smart-quotes por lang.
- ADR-0060: transição `PROPOSTO → IMPLEMENTADO` no fim.
- Inventário 148: cobertura Model 41% → ~45%; arquitectural
  Content 75% → ~77%.

**ADRs aplicáveis**:
- **ADR-0060** (PROPOSTO) — Fase 1 fecha aqui.
- **ADR-0026** + **ADR-0026-R1** — `Content` enum fechado.
- **ADR-0033** — paridade funcional.
- **ADR-0036** — atomização.
- **ADR-0037** — coesão por domínio.
- **ADR-0038** — sistema de estilos: `Quote` não usa
  `Content::Styled` (per regra 7.2 do diagnóstico 154A).
- **ADR-0054** — perfil observacional graded (smart-quotes
  podem ter cobertura inicial limitada se lang específico
  não tem dados; aceitável).
- **ADR-0057** — lang hyphenation (precedente de feature
  lang-aware; smart-quotes seguem padrão similar).

---

## Contexto

Diagnóstico 154A classificou `quote` como ausente; M;
médio valor; recomendação: variant novo (per regra 7.2).

Vanilla `QuoteElem` em
`lab/typst-original/crates/typst-library/src/model/quote.rs`
tem **4 atributos**:

- `body: Content` — conteúdo do quote.
- `attribution: Option<...>` — autor/fonte (Optional).
- `block: bool` — true = parágrafo dedicado; false = inline.
- `quotes: bool` ou `Smart<bool>` — controla aspas
  automáticas em torno do body.

Vanilla syntax markup `"..."` em contexto markup é distinto
de string literal `"..."` em código:
- **Em código**: `"hello"` é `Value::Str`.
- **Em markup**: `"hello"` é `QuoteElem` com smart-quotes
  aplicadas (`«hello»` em PT, `"hello"` em EN, etc.).

Smart-quotes dependem de `text.lang` (per ADR-0057
precedente). Tabela de aspas por lang típica:

| Lang | Open primary | Close primary |
|------|--------------|---------------|
| `pt`, `pt-PT`, `pt-BR` | `«` | `»` |
| `en`, `en-US`, `en-GB` | `"` (U+201C) | `"` (U+201D) |
| `de`, `de-DE` | `„` | `"` |
| `fr`, `fr-FR` | `« ` (com NBSP) | ` »` |
| `es` | `«` | `»` |
| `it` | `«` | `»` |
| default fallback | `"` (ASCII) | `"` (ASCII) |

Aspas secundárias (`'...'`) também variam — mas não
materializadas neste passo (scope-out).

**Decisões humanas confirmadas**:
1. **4 atributos**: `body`, `attribution`, `block`, `quotes`.
2. **Forma**: `Quote { body, attribution, block, quotes }` —
   4 fields fixos.
3. **Construtor**: `#quote()` stdlib **+** sintaxe markup
   `"..."` com **smart-quotes completos** (lang-aware).

**Escopo confirmado L**: aceitar custo de smart-quotes.

---

## Objectivo

Ao fim do passo:

1. **`Content` enum estendido**:
   ```rust
   Content::Quote {
       body: Box<Content>,
       attribution: Option<Box<Content>>,
       block: bool,
       quotes: bool,
   }
   ```
   Total +1 variant (42 → 43).

2. **Cobertura exaustiva** dos arms (~9 sítios, similar a
   154B):
   - `Content::plain_text()` — Quote produz
     `quotes_open + body.plain_text() + quotes_close +
     attribution_separator + attribution.plain_text()` (com
     `attribution_separator = " — "` se present).
   - `Content::is_empty()` — depende de `body.is_empty()`.
   - `Content::map_content` — recurse em body e
     attribution.
   - `Content::map_text` — análogo.
   - `PartialEq` — derivado.
   - Layouter (`03_infra/src/...` ou
     `01_core/src/rules/layout/mod.rs`) — Quote render:
     - Se `block: true`: parágrafo dedicado, indent left
       margin, attribution em linha separada (alinhada à
       direita, prefixada por "—").
     - Se `block: false` (inline): integrado no parágrafo
       circundante; sem quebra de linha.
     - Se `quotes: true`: insere aspas locale-apropriadas
       em torno do body.
   - `introspect::materialize_time` / `walk` — recurse em
     filhos.
   - Show rules em `eval/rules.rs` — sem show específico
     para Quote neste passo (consistente com P154B).

3. **Stdlib func** `native_quote` em
   `01_core/src/rules/eval/mod.rs` ou
   `01_core/src/rules/stdlib/structural.rs`:
   ```rust
   #[allow(dead_code)]
   pub fn native_quote(args: &Args) -> SourceResult<Value> {
       let body = expect_arg_content(args, 0)?;
       let attribution = take_named_optional(args, "attribution");
       let block = take_named_or(args, "block", false)?;
       let quotes = take_named_or(args, "quotes", true)?;
       Ok(Value::Content(Content::Quote {
           body: Box::new(body),
           attribution: attribution.map(Box::new),
           block,
           quotes,
       }))
   }
   ```
   Registar em `make_stdlib`. Total stdlib funcs:
   31 → 32 + módulo `calc`.

4. **Parser markup `"..."`**:
   - Adicionar regra em parser markup que reconhece
     `"texto"` como `QuoteSyntax { content: "texto" }` ou
     similar (forma sintáctica nova, separada de string
     literal em código).
   - Em `eval`, `QuoteSyntax` resolve para
     `Content::Quote { body, attribution: None,
     block: false, quotes: true }` — sempre inline; sempre
     com smart-quotes.
   - **Crítico**: distinção contextual code vs markup
     **preservada**. `"..."` em código continua a ser
     `Value::Str`.

5. **Smart-quotes mecanismo**:
   - Função `localize_quotes(body: &str, lang: &Lang) ->
     (open: &'static str, close: &'static str)` em
     `01_core/src/rules/lang/quotes.rs` (módulo novo) ou
     `01_core/src/entities/lang.rs` (extensão).
   - Tabela de pares `open`/`close` por lang. Inicial: 7
     langs (`pt`/`en`/`de`/`fr`/`es`/`it`/default).
   - Layouter consulta `text.lang` actual (per
     `StyleDelta.lang` per ADR-0057) e aplica em runtime.
   - **Forma escolhida**: tabela estática
     `&'static [(BCP47, (&str, &str))]` com lookup linear
     (7 entries → trivial). Lookup por prefixo BCP47 (e.g.
     `pt-BR` faz match em `pt`).

6. **Tests**:
   - **Unit em `content.rs::tests`**:
     - `quote_constructor_devolve_variant_correcto`.
     - `quote_plain_text_com_attribution`.
     - `quote_plain_text_sem_attribution`.
     - `quote_is_empty_proxy_para_body`.
     - `quote_map_content_recurse_em_body_e_attribution`.
   - **Unit em `lang/quotes.rs::tests`** (módulo novo):
     - `localize_quotes_pt_devolve_aspas_baixas`.
     - `localize_quotes_en_devolve_curly`.
     - `localize_quotes_de_devolve_par_germanico`.
     - `localize_quotes_lang_desconhecido_devolve_default_ascii`.
     - `localize_quotes_pt_BR_resolve_pt`.
   - **Unit em eval / parse**:
     - `eval_quote_construtor_typst_lang`.
     - `eval_quote_com_attribution_typst_lang`.
     - `eval_quote_block_true_typst_lang`.
     - `parse_markup_aspas_simples_devolve_quote_node`.
     - `parse_markup_aspas_em_codigo_continua_string_literal`
       (regression).
   - **Integração render** (opcional consoante layouter):
     - `quote_block_render_em_pdf`.
     - `quote_inline_render_em_pdf`.
     - `quote_pt_renderiza_aspas_baixas`.
     - `quote_en_renderiza_curly_quotes`.

7. **L0 prompts**:
   - `prompts/entities/content.md` ganha secção
     "Quote — Passo 155 (ADR-0060 Fase 1)".
   - `prompts/rules/lang.md` (ou ficheiro equivalente que
     governa `Lang`) ganha secção "Smart-quotes — Passo
     155".
   - Hashes recalculados; propagados via
     `crystalline-lint --fix-hashes`.

8. **ADR-0060 transita `PROPOSTO → IMPLEMENTADO`**:
   - Status alterado.
   - **Validado** atualizado: "Passo 154A — diagnóstico;
     Passo 154B — sub-passo 1; **Passo 155 — sub-passo 2;
     Fase 1 fechada**".
   - Anotação Passo 155 com sumário do que ficou.
   - Plano de Fase 2 mantido inalterado (P156/157/158).

9. **Inventário 148**:
   - Tabela A Model: 5/4/5/8/0 → **6/4/5/7/0** (quote sai
     de ausente para implementado).
   - Cobertura Model: 41% → **45%** (10/22 → 11/22).
   - Total user-facing: 55% → **56%**.
   - Tabela B Content: 42 → **43 variants**.
   - Cobertura arquitectural Content: 75% → **77%**.
   - §7 entrada 7: ~12 → ~11 elementos vanilla agregados
     ausentes.

10. **README dos ADRs**:
    - Distribuição: `PROPOSTO` 11 → 10; `IMPLEMENTADO` 18
      → 19.
    - Tabela "Estado por ADR": ADR-0060 muda de PROPOSTO
      para IMPLEMENTADO.
    - Entrada nova em "Passos-chave" para P155.

11. **Relatório do passo** em
    `00_nucleo/materialization/typst-passo-155-relatorio.md`.

Este passo **não**:

- Materializa show rules (`#show quote: ...`) — candidato
  P159+ ou passo agregado pós-Fase 1.
- Materializa aspas secundárias (`'...'`) — extensão
  futura.
- Materializa smart-apostrophes (`don't` → `don't`) —
  trabalho separado de smart-quotes; pode ser parte de P159+.
- Toca DEBT-55 ou outras DEBTs.
- Toca série paridade / `lab/parity/`.
- Materializa Fase 2 (table, figure-kinds, bibliography).
- Importa crates novas.

---

## Decisões já tomadas

1. **4 atributos** materializados.
2. **Forma**: `Quote { body, attribution, block, quotes }`,
   4 fields fixos.
3. **Construtor duplo**: stdlib `#quote()` + markup `"..."`
   com smart-quotes lang-aware.
4. **Smart-quotes para 7 langs iniciais** + default ASCII.
5. **`PROPOSTO → IMPLEMENTADO`** no fim (Fase 1 fechada).
6. **Sem show rules** neste passo.
7. **Sem aspas secundárias** ('') neste passo.
8. **Smart-apostrophes scope-out**.

## Decisões diferidas (resolvidas neste passo)

9. **Localização do módulo smart-quotes**:
   - Opção A: `01_core/src/rules/lang/quotes.rs` (módulo
     novo dedicado, agrupado com hyphenation).
   - Opção B: `01_core/src/entities/lang.rs` (extensão de
     ficheiro existente).
   - **Default A** — coesão por domínio (ADR-0037);
     smart-quotes é regra (rules/), não entidade.

10. **Lookup BCP47**:
    - Opção C: lookup exact match + fallback por prefixo
      (`pt-BR` → `pt-BR`? não → `pt`? sim → resolver).
    - Opção D: só prefixo (`pt-BR` → `pt`).
    - **Default C** (mais flexível para futuro
      `pt-PT` ≠ `pt-BR` se priorizado).

11. **Tabela de aspas para 7 langs**: confirmada acima.
    `pt`, `en`, `de`, `fr`, `es`, `it`, default. Outras
    langs (zh, ja, ar, ...) caem em default ASCII.

12. **Como `quotes: bool` interage com markup `"..."`**:
    - Markup `"..."` sempre produz `quotes: true` (aspas
      aplicadas).
    - Stdlib `#quote(body, quotes: false)` produz quote
      sem aspas adicionais (útil para citações já com aspas
      no body, ou layouts custom).
    - Se `quotes: true` mas body já começa/termina com
      aspas: **dupla aspas**. Aceite (não-trivial detectar
      e suprimir; ADR-0054 perfil graded cobre).

13. **`block` interaction com layouter**:
    - `block: true` → quote ocupa linhas próprias; indent
      `1.5em` à esquerda; spacing antes/depois `0.6em`.
      Attribution em linha separada à direita, prefixo
      `"— "`.
    - `block: false` → quote inline no parágrafo; sem
      quebra; attribution (se presente) inline também,
      prefixo `" — "`.
    - Métricas exactas (1.5em, 0.6em) confirmadas em 155.1
      ou aproximadas.

14. **Parser: como distinguir `"..."` em markup de string
    literal**:
    - Cristalino tem `Mode::Markup` vs `Mode::Code` (ou
      equivalente).
    - Em `Mode::Markup`: `"texto"` → token `MarkupQuote`
      ou similar; eval cria `Content::Quote`.
    - Em `Mode::Code`: `"texto"` → token `String` (forma
      actual); eval cria `Value::Str`.
    - Confirmar mode-tracking em parser em 155.1.

15. **Edge case: `"..."` vazio (`""`)** em markup:
    - Decisão: `Quote { body: Empty, ..., quotes: true }`
      → renderiza só as aspas (`«»` em PT, `""` em EN).
    - Trivial; não justifica decisão arquitectural.

16. **Edge case: aspas aninhadas** (`"frase com "interna""`):
    - Vanilla provavelmente alterna primary/secondary
      automaticamente.
    - Cristalino: **não materializado** neste passo
      (aspas secundárias scope-out). Aspas literais
      aninhadas em markup confundem parser (ambiguidade
      open/close). Documentar como limitação.

17. **Performance**: lookup tabela 7 entries é trivial;
    sem cache. Se langs aumentar para 50+ futuro,
    considerar HashMap.

---

## Escopo

**Dentro**:

- Edição de `01_core/src/entities/content.rs`: 1 variant
  novo + cobertura exaustiva de arms.
- Edição de `01_core/src/entities/content.rs::tests`: 5
  testes unit Quote.
- Criação de `01_core/src/rules/lang/quotes.rs` (módulo
  novo) com:
  - `localize_quotes(body_lang) -> (open, close)`.
  - Tabela `LANG_QUOTES` (7 entries).
  - 5 testes unit.
- Edição de `01_core/src/rules/lang/mod.rs` (ou ficheiro
  análogo) para expor o módulo.
- Edição de `01_core/src/rules/eval/mod.rs` ou
  `stdlib/structural.rs`: `native_quote` + registo em
  `make_stdlib`.
- Edição de `01_core/src/rules/parse/...` (parser markup):
  reconhecer `"..."` em `Mode::Markup` como token novo.
- Edição de `01_core/src/rules/eval/...` (eval markup):
  resolve token markup-quote para `Content::Quote`.
- Edição de Layouter para Quote rendering (block + inline
  + smart-quote insertion via `localize_quotes`).
- 4-6 testes unit eval/parse.
- Até 4 testes integração render (opcional consoante
  layouter).
- Edição de `prompts/entities/content.md` (Quote variant).
- Edição de `prompts/rules/lang.md` (smart-quotes).
- Hashes propagados via lint.
- ADR-0060: anotação + transição `PROPOSTO → IMPLEMENTADO`.
- Inventário 148: contagens recalculadas.
- README dos ADRs: tabela + distribuição + Passos-chave.
- Relatório do passo.

**Fora**:

- Show rules `#show quote`.
- Aspas secundárias (`'...'` em markup).
- Smart-apostrophes (`'` → `'`).
- Aspas aninhadas com alternância.
- Outras Fase 2 features (table, figure-kinds,
  bibliography).
- DEBT-55 ou modificação de outras DEBTs.
- Trabalho em `lab/parity/`.
- Importação de crates novas.
- Aspas para langs não listadas (zh, ja, ar) — fallback
  default ASCII; expansão futura.

---

## Sub-passos

### 155.1 — Inventário pré-materialização

**A.1.1 — Confirmar `Content` enum actual**:

```bash
view 01_core/src/entities/content.rs   # esperado 41 variants pós-P154B
grep -nE "^pub enum Content" 01_core/src/entities/content.rs
```

Esperado 42 variants (38 + Divider + Terms + TermItem
per P154B). Confirmar antes de adicionar Quote.

**A.1.2 — Verificar `Lang` actual + ADR-0057**:

```bash
view 01_core/src/entities/lang.rs   # se existir
grep -nE "pub struct Lang\|pub fn lang" 01_core/src/entities/
```

Confirmar:
- `Lang` tipo + métodos (e.g. `bcp47() -> &str` ou
  `as_str()`).
- Onde está o módulo `lang/` (per ADR-0057 hyphenation).

**A.1.3 — Verificar parser mode tracking**:

```bash
grep -rn "Mode::Markup\|Mode::Code\|markup_mode\|code_mode" \
  01_core/src/rules/parse/
```

Confirmar:
- Há distinção contextual? (quase certo que sim — Typst
  parser tem isso).
- Como é representada (enum, flag, função).
- Onde tokens markup são produzidos (e.g. `parse_markup`,
  `parse_paragraph`).

**A.1.4 — Verificar layout actual de outros block elements**:

```bash
grep -nE "fn layout_(heading|figure|paragraph)" 01_core/src/rules/layout/
```

Para Quote `block: true`, reusar padrões. Esperado:
funções similares com indent + spacing.

**A.1.5 — Verificar se `Content::Text` carrega lang**:

Smart-quotes precisam saber o lang em runtime. Lang vive
em `StyleDelta` (per ADR-0057). Confirmar se `layout_quote`
tem acesso a `StyleDelta` activo.

```bash
grep -nE "StyleDelta\|style_delta" 01_core/src/rules/layout/
```

### 155.2 — Adicionar variant `Content::Quote`

```diff
 pub enum Content {
     // ... 42 variants existentes
+    Quote {
+        body: Box<Content>,
+        attribution: Option<Box<Content>>,
+        block: bool,
+        quotes: bool,
+    },
 }
```

### 155.3 — Cobertura exaustiva de arms

Para cada match site identificado em 155.1:

**A.3.1 — `is_empty`**:

```rust
Content::Quote { body, .. } => body.is_empty(),
```

**A.3.2 — `plain_text`** (sem smart-quotes — texto plain
não interage com lang; usa ASCII fallback):

```rust
Content::Quote { body, attribution, block, quotes } => {
    let body_txt = body.plain_text();
    let with_quotes = if *quotes {
        format!("\"{}\"", body_txt)
    } else {
        body_txt
    };
    match attribution {
        Some(a) => format!("{} — {}", with_quotes, a.plain_text()),
        None => with_quotes,
    }
}
```

**A.3.3 — `map_content`**: recurse em body + attribution.
`block` e `quotes` são bool primitivos.

**A.3.4 — `map_text`**: análogo.

**A.3.5 — `PartialEq`**: derivado.

**A.3.6 — Layouter `layout_content`**:

```rust
Content::Quote { body, attribution, block, quotes } => {
    let lang = current_style_delta.lang.unwrap_or_default();
    let (open, close) = if *quotes {
        crate::rules::lang::quotes::localize_quotes(&lang)
    } else {
        ("", "")
    };
    if *block {
        layout_block_quote(body, attribution, open, close, ...)
    } else {
        layout_inline_quote(body, attribution, open, close, ...)
    }
}
```

`layout_block_quote` e `layout_inline_quote` são funções
novas no layouter. Forma mínima viável; ADR-0054 graded
cobre.

**A.3.7 — Introspect `materialize_time` / `walk`**:
recurse em body + attribution.

### 155.4 — Módulo `rules/lang/quotes.rs`

```rust
//! Smart-quotes lang-aware.
//! Materializado em Passo 155 (ADR-0060 Fase 1).

use crate::entities::lang::Lang;

const LANG_QUOTES: &[(&str, (&str, &str))] = &[
    ("pt",      ("«",     "»")),
    ("en",      ("\u{201C}", "\u{201D}")),
    ("de",      ("\u{201E}", "\u{201C}")),
    ("fr",      ("« ",    " »")),
    ("es",      ("«",     "»")),
    ("it",      ("«",     "»")),
];

const DEFAULT_QUOTES: (&str, &str) = ("\"", "\"");

/// Devolve par open/close para o lang dado.
/// Lookup por exact match + fallback por prefixo BCP47.
pub fn localize_quotes(lang: &Lang) -> (&'static str, &'static str) {
    let bcp47 = lang.bcp47();
    // Exact match
    for (key, pair) in LANG_QUOTES.iter() {
        if *key == bcp47 {
            return *pair;
        }
    }
    // Prefix match
    for (key, pair) in LANG_QUOTES.iter() {
        if bcp47.starts_with(&format!("{}-", key)) {
            return *pair;
        }
    }
    DEFAULT_QUOTES
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::lang::Lang;

    #[test]
    fn localize_quotes_pt_devolve_aspas_baixas() {
        assert_eq!(localize_quotes(&Lang::from("pt")), ("«", "»"));
    }

    // ... outros 4 tests
}
```

Registar em `01_core/src/rules/lang/mod.rs`:

```diff
 pub mod hyphenation;
+ pub mod quotes;
```

### 155.5 — `native_quote` em stdlib

Em `01_core/src/rules/stdlib/structural.rs`:

```rust
pub fn native_quote(args: &Args) -> SourceResult<Value> {
    let body = match args.items.first() {
        Some(Value::Content(c)) => c.clone(),
        Some(Value::Str(s)) => Content::text(s.as_str()),
        Some(other) => return Err(...),
        None => return Err(...),
    };
    let attribution = args.named.get("attribution")
        .and_then(|v| match v {
            Value::Content(c) => Some(c.clone()),
            Value::Str(s) => Some(Content::text(s.as_str())),
            _ => None,
        });
    let block = args.named.get("block")
        .and_then(|v| match v {
            Value::Bool(b) => Some(*b),
            _ => None,
        })
        .unwrap_or(false);
    let quotes = args.named.get("quotes")
        .and_then(|v| match v {
            Value::Bool(b) => Some(*b),
            _ => None,
        })
        .unwrap_or(true);
    Ok(Value::Content(Content::Quote {
        body: Box::new(body),
        attribution: attribution.map(Box::new),
        block,
        quotes,
    }))
}
```

Registar em `make_stdlib`:

```rust
scope.define("quote", Value::Func(Func::native("quote", native_quote)));
```

### 155.6 — Parser markup `"..."`

**A.6.1 — Identificar tokenizer markup**:

```bash
grep -rn "fn parse_markup\|tokenize_markup\|markup_token" \
  01_core/src/rules/parse/
```

**A.6.2 — Adicionar regra para `"..."`**:

Em parser markup, quando encontra `"` em `Mode::Markup`,
ler até próximo `"` (sem escape neste passo) e produzir
nó AST `MarkupQuote { content: ... }`.

**A.6.3 — Eval do nó markup**:

Em `eval`, `MarkupQuote { content }` resolve para
`Content::Quote { body: content, attribution: None, block:
false, quotes: true }`.

**A.6.4 — Edge cases**:

- `"` sem fechamento: erro de parse.
- `""` vazio: produz `Quote { body: Empty, ... }` per
  Decisão 15.
- `"` em código (`#let s = "..."`): comportamento
  anterior preservado (string literal).

**A.6.5 — Tests regression**:

`parse_markup_aspas_em_codigo_continua_string_literal`
**obrigatório**. Falha = regressão crítica.

### 155.7 — Tests

**A.7.1 — Unit Content**:

5 testes em `content.rs::tests` (esboço em §Objectivo
item 6).

**A.7.2 — Unit `quotes.rs`**:

5 testes (esboço em §Objectivo item 6).

**A.7.3 — Eval/parse**:

5 testes (esboço em §Objectivo item 6).

**A.7.4 — Integração render** (opcional):

Até 4 testes (esboço em §Objectivo item 6).

**Total estimado**: 1123 → 1138-1142 (+15 a +19).

### 155.8 — L0 prompts + hashes

**A.8.1 — `prompts/entities/content.md`**:

Adicionar secção:

```markdown
### Quote — Passo 155 (ADR-0060 Fase 1)

`Content::Quote { body, attribution, block, quotes }` —
representa citação estrutural.

**Atributos**:
- `body: Box<Content>` — conteúdo citado.
- `attribution: Option<Box<Content>>` — autor/fonte.
- `block: bool` — true = parágrafo dedicado; false = inline.
- `quotes: bool` — true = aspas locale-apropriadas em
  torno do body.

**Comportamento `plain_text`**:
- Sem smart-quotes (usa `"` ASCII fallback).
- Com attribution: `"body" — attribution`.
- Sem attribution: `"body"`.

**Renderização (layouter)**:
- Smart-quotes via `crate::rules::lang::quotes::localize_quotes`.
- `block: true`: indent + spacing dedicado.
- `block: false`: inline no parágrafo circundante.
```

**A.8.2 — `prompts/rules/lang.md`**:

Adicionar secção:

```markdown
### Smart-quotes — Passo 155

Função `localize_quotes(lang) -> (open, close)` em
`01_core/src/rules/lang/quotes.rs`.

**Tabela inicial** (7 entries):
| Lang | Open | Close |
| `pt` | `«` | `»` |
| `en` | `"` (U+201C) | `"` (U+201D) |
| `de` | `„` | `"` (U+201C) |
| `fr` | `« ` (com NBSP) | ` »` |
| `es` | `«` | `»` |
| `it` | `«` | `»` |
| default | `"` ASCII | `"` ASCII |

**Lookup**: exact match primeiro; fallback por prefixo
BCP47 (e.g. `pt-BR` → `pt`).
```

**A.8.3 — Hashes**:

```bash
sha256sum 00_nucleo/prompts/entities/content.md
sha256sum 00_nucleo/prompts/rules/lang.md
crystalline-lint --fix-hashes .
```

Headers actualizados em `01_core/src/entities/content.rs`,
`01_core/src/rules/lang/quotes.rs`, e qualquer ficheiro
com `@prompt-hash` afectado.

### 155.9 — ADR-0060 transição

```diff
- **Status**: `PROPOSTO`
+ **Status**: `IMPLEMENTADO`
- **Validado**: Passo 154A — diagnóstico.
+ **Validado**: Passo 154A — diagnóstico; Passo 154B —
+ sub-passo 1 (terms + divider); Passo 155 — sub-passo 2
+ (quote); **Fase 1 fechada**.
```

Anotação Passo 155 adicionada na secção "Materialização":

```markdown
**Anotação Passo 155 (2026-04-25)**: segundo sub-passo da
Fase 1 materializado — `Content::Quote { body, attribution,
block, quotes }` adicionado ao enum `Content`; `native_quote`
em stdlib; parser markup `"..."` reconhece QuoteSyntax em
`Mode::Markup` (string literal em `Mode::Code` preservado);
módulo `rules/lang/quotes.rs` com smart-quotes para 7 langs.

**Fase 1 fechada**. Status `PROPOSTO → IMPLEMENTADO`.
Cobertura Model 41% → ~45%; arquitectural Content 75% → ~77%.
Plano Fase 2 (P156/157/158 — table, figure-kinds,
bibliography+cite) inalterado.
```

### 155.10 — Inventário 148

`00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`:

- **Tabela A Model**: 5/4/5/8/0 → **6/4/5/7/0**.
  - Cobertura Model: 41% → **45%** (11/22).
- **Total user-facing**: 55% → **56%**.
- **Tabela B Content**: 42 → **43 variants** (+Quote).
  - Cobertura arquitectural Content: 75% → **77%**.
- **§7 entrada 7**: ~12 → ~11 elementos vanilla agregados
  ausentes.

### 155.11 — README dos ADRs

- Tabela "Estado por ADR": linha ADR-0060 muda
  `PROPOSTO → IMPLEMENTADO`.
- Distribuição: `PROPOSTO` 11 → 10; `IMPLEMENTADO` 18 → 19.
- Total inalterado (60 ADRs).
- Entrada nova em "Passos-chave da história dos ADRs"
  para P155 — Fase 1 fechada.

### 155.12 — Relatório do passo

Ficheiro:
`00_nucleo/materialization/typst-passo-155-relatorio.md`.

Secções:
1. Sumário executivo.
2. Inventário pré-materialização (155.1).
3. Variant Quote — forma final + diff.
4. Cobertura exaustiva de arms.
5. Módulo `rules/lang/quotes.rs` — assinatura + tabela.
6. `native_quote` — assinatura + registo.
7. Parser markup — diff + edge cases.
8. Tests — lista + contagens.
9. L0 prompts + hashes propagados.
10. ADR-0060 transição `PROPOSTO → IMPLEMENTADO`.
11. Inventário 148 actualizado.
12. README dos ADRs actualizado.
13. Próximo passo: 156 (Fase 2 — table foundations) ou
    decisão humana entre alternativas.
14. Limitações registadas:
    - Sem show rules.
    - Sem aspas secundárias.
    - Sem smart-apostrophes.
    - 7 langs iniciais (outras → fallback ASCII).
    - Aspas aninhadas não suportadas em markup.
15. Verificação final.

---

## Verificação

1. ✅ `Content::Quote` adicionado com 4 fields.
2. ✅ Cobertura exaustiva de arms (~9 sítios L1).
3. ✅ Módulo `rules/lang/quotes.rs` com 7 langs +
   default + lookup BCP47 + 5 tests.
4. ✅ `native_quote` registada em `make_stdlib`.
5. ✅ Parser markup `"..."` reconhece em `Mode::Markup`.
6. ✅ Parser markup `"..."` em `Mode::Code` continua
   string literal (regression test obrigatório).
7. ✅ Smart-quotes consultam `text.lang` em runtime.
8. ✅ Layouter cobre block + inline.
9. ✅ 5 unit Content + 5 unit quotes + 5 eval/parse +
   até 4 render = 15-19 testes novos.
10. ✅ L0 prompts actualizados (content.md + lang.md);
    hashes propagados.
11. ✅ ADR-0060 transita `PROPOSTO → IMPLEMENTADO`.
12. ✅ ADR-0060 anotada com sumário Passo 155.
13. ✅ Inventário 148 actualizado.
14. ✅ README dos ADRs com tabela + distribuição +
    Passos-chave actualizados.
15. ✅ Sem ADR nova; sem DEBT criado / fechado.
16. ✅ `cargo test --workspace --lib`: 1123 → 1138-1142.
17. ✅ `crystalline-lint .` zero violations.
18. ✅ Relatório do passo escrito.

---

## Critério de conclusão

1. `Content::Quote` compila + tests unit passam.
2. Stdlib func `#quote(body, attribution: ..., block: ...,
   quotes: ...)` invocável.
3. Markup `"..."` em contexto markup produz `Content::Quote`
   com smart-quotes lang-aware.
4. Markup `"..."` em código continua a ser
   `Value::Str` (regressão evitada).
5. Smart-quotes funcionam para 7 langs + default.
6. ADR-0060 IMPLEMENTADO; Fase 1 fechada.
7. Inventário 148 reflecte cobertura aumentada.
8. Próximo passo (156 = table foundations) tem âncora.
9. Sem regressão.
10. Relatório do passo escrito.

---

## O que pode sair errado

- **Parser sem `Mode::Markup` vs `Mode::Code` distinto**:
  improvável (Typst parser tem isso). Se ausente,
  **pausar**: feature exige refactor de parser que excede
  escopo P155. Abrir DEBT-56 ("Parser mode tracking") e
  scope-out markup `"..."` neste passo (manter só stdlib).
  ADR-0060 ainda transita IMPLEMENTADO se outros
  componentes ficam.

- **Smart-quotes layouter não tem acesso a `text.lang`**:
  improvável (StyleDelta carrega-o per ADR-0057). Se
  acontecer, fallback default ASCII em todos os contextos;
  documentar como limitação.

- **Cobertura exaustiva quebra compilação em sítios L3
  inesperados**: e.g. PDF export tem match sobre Content.
  Compilador guia. Esperado: 1-2 sítios extras em
  `03_infra/`. Adicionar arms; iterar.

- **Layouter `block: true` exige page model não-existente**:
  block quote pode exigir column flow ou column break que
  cristalino não tem. Solução: forma mínima viável (apenas
  spacing + indent dentro do parágrafo actual; sem column
  break). Documentar como limitação P159+.

- **Lookup BCP47 com prefix-match introduz bugs**: edge
  case `lang = "pt"` puro: exact match em "pt" funciona.
  Edge case `lang = "pt-BR"`: prefix match em "pt-"
  funciona. Edge case `lang = "p"`: nem exact nem prefix
  → default. Confirmar em testes.

- **`«` e `»` (PT) precisam font support**: fonts cristalino
  cobertas em P140B/146 incluem DejaVu (cobre Latin extended,
  inclui `«»`). Improvável falhar; se falhar, fallback
  glyph missing path documentado.

- **Markup `"..."` confunde com regex/escape em contexto
  específico**: cristalino não tem regex em L1 (gap 8 DEBT-52);
  sem confusão. Confirmar.

- **Aspas aninhadas em markup**: `"frase com "interna""` —
  parser pode interpretar primeiro `"` como open, segundo
  como close, depois `"interna"` como nova quote, último
  `"` órfão. **Comportamento aceite**: alternância manual
  via stdlib `#quote()` em Typst-lang; markup só suporta
  pares simples não-aninhados. Documentar limitação.

- **`native_quote` decimal de spec falha em parse**: ex,
  `args.named.get("block")` vs vanilla espera positional.
  Confirmar em P155.1; ajustar.

- **Diff em `content.rs` cresce >150 linhas**: aceitável
  (cobertura exaustiva é o custo). Se for >300, considerar
  refactor de helpers (e.g. `Content::recurse_children()`
  para reduzir duplicação em `map_content`/`map_text`/etc).
  Decisão futura.

- **Hashes L0 não propagam consistentemente**: 2 ficheiros
  prompt mudados; lint deve cobrir. Se algum sítio falha,
  manualmente `--fix-hashes`. Verificar.

- **Show rule built-in quote já existe** (e.g. desugar
  `"..."` markup é uma show rule): improvável dado que
  cristalino não tem show rules built-in; se descoberto,
  integrar. Ajustar arquitectura.

---

## Notas operacionais

- **Modelo: substantivo escopo L** (4-6h). Maior que
  P154B (M, ~2-3h). Justifica-se pela combinação de
  variant + parser + smart-quotes.

- **Fase 1 fecha aqui**. ADR-0060 transita
  `PROPOSTO → IMPLEMENTADO`. Cobertura Model 41% → ~45%
  (target Fase 1 era ~50% per diagnóstico 154A; ligeiramente
  abaixo porque footnote saiu da Fase 1).

- **Pós-155**:
  - ADR-0060 IMPLEMENTADO.
  - Fase 1 inteira: terms + divider + quote.
  - Fase 2 abre: table (P156), figure-kinds (P157),
    bibliography (ADR-0061 + P158).
  - Show rules para Fase 1 features: candidato a passo
    agregado (P159+ ou outro).

- **Smart-quotes precedente**: este passo materializa
  primeira tabela lang-aware no L1 fora de hyphenation
  (ADR-0057). Se outras features lang-aware surgirem (ex:
  date format, number format), padrão `localize_*(lang)`
  está estabelecido.

- **Limitações Fase 1**: registadas em ADR-0060 anotação
  + relatório 155 + inventário 148:
  - Sem syntax markup para terms (`/ term: desc`).
  - Sem syntax markup para divider (`---`) — exception:
    se 154B.1 confirmou que cristalino não usa `---`,
    **pode** ser implementado neste passo como bónus —
    decisão humana.
  - Sem show rules.
  - Sem atributos vanilla para terms (`tight`, etc).
  - Aspas secundárias para quote.
  - Smart-apostrophes.
  - Aspas aninhadas em markup.

- **Síntese da estratégia diagnóstico-primeiro Model**:
  154A diagnosticou; 154B materializou primeira sub-fase;
  155 materializa segunda sub-fase e fecha Fase 1. Padrão
  funcionou; antecipou todos os bloqueios materiais.

- **Reformulação 8 da série paridade**? Não — série
  paridade continua suspensa em P153. Esta numeração
  contínua segue Model. Quando paridade for retomada,
  P156 reservado para P4 cristalino-only; este P156
  Model conflicaria. **Solução**: P156 Model fica como
  está; P4 cristalino-only fica para numeração diferente
  (e.g. `P-paridade-4` ou similar) quando paridade for
  retomada. Decisão de numeração registada como dívida
  documental menor.

- **Quarentena vanilla**: continua opção 3 (princípio sem
  regra absoluta). Sem mudança neste passo.

- **DEBT-55** (bibliography + cite XL) inalterado.
  Atacável em P158 com ADR-0061.
