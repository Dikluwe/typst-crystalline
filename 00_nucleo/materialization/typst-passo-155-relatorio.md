# Relatório do Passo 155 — Fase 1 Model fechada (sub-passo 2: `quote`)

**Tipo**: substantivo escopo L (variant + parser markup + smart-quotes
lang-aware + módulo novo `rules/lang/`).
**Padrão**: 154A (diagnóstico) → 154B (sub-passo 1) → **155 (sub-passo 2;
fecha Fase 1)**.

## 1. Sumário executivo

Materializou-se a segunda e última sub-fase da Fase 1 do roadmap
ADR-0060:

- **`Content::Quote { body, attribution, block, quotes }`** —
  variant estrutural com 4 atributos do vanilla `QuoteElem`.
- **`native_quote`** em stdlib, expondo `#quote(body,
  attribution: ?, block: ?, quotes: ?)` em Typst-lang.
- **Módulo novo `01_core/src/rules/lang/quotes.rs`** com
  `localize_quotes(lang) → (open, close)` para 6 idiomas
  (`pt`/`en`/`de`/`fr`/`es`/`it`) + default ASCII.
- **Smart-quotes em markup `"..."`** via alternância open/close por
  sequência, emitindo glyph localizado como `Content::Text`. Distinção
  contextual code vs markup **preservada**: `"..."` em código continua
  a ser `Value::Str` (regression test obrigatório passa).

**ADR-0060 transita `PROPOSTO → IMPLEMENTADO`**. Fase 1 inteira
fechada. Plano Fase 2 (P156–P158) inalterado.

## 2. Inventário pré-materialização (155.1)

- `Content` enum: 42 variants pré-P155 (38 + Divider/Terms/TermItem
  per P154B).
- `Lang` em `01_core/src/entities/lang.rs`: 2-3 letras ASCII puro per
  ADR-0052; `as_str()` devolve `&str` lowercase. Sem region/country
  → lookup BCP47 com prefix simplifica para exact match.
- Parser em `Mode::Markup` vs `Mode::Code` confirmado
  (`SyntaxMode::{Markup,Code}`); lexer em `01_core/src/rules/lexer/markup.rs`
  já produz `SyntaxKind::SmartQuote` para `'` e `"` (per-character).
  `parse/markup.rs::markup_expr` consome SmartQuote como leaf
  (`p.eat()`); eval drop-down silencioso (caía em `_ => Value::None`).
- Módulo `rules/lang/` **não existia**; `rules/layout/hyphenation.rs`
  hospedava lang-aware hyphenation (ADR-0057).
- `make_stdlib`: 31 funcs nativas + módulo `calc` antes de P155.

## 3. Variant `Content::Quote` — forma final + diff

```rust
// 01_core/src/entities/content.rs
pub enum Content {
    // ... 42 variants existentes
    Quote {
        body:        Box<Content>,
        attribution: Option<Box<Content>>,
        block:       bool,
        quotes:      bool,
    },
}
```

**Decisões aplicadas**:
- 4 fields named (Decisão 2 do spec).
- `attribution: Option<Box<Content>>` (não-tomada de fonte).
- `block: bool` controla layout (block vs inline).
- `quotes: bool` controla aspas localizadas.

## 4. Cobertura exaustiva de arms (~7 sítios)

| Ficheiro | Função | Tratamento |
|----------|--------|------------|
| `entities/content.rs` | `is_empty()` | `body.is_empty()` |
| `entities/content.rs` | `plain_text()` | ASCII fallback (sem smart-quotes); attribution prefixada por " — " |
| `entities/content.rs` | `PartialEq::eq` | comparação par-a-par dos 4 fields |
| `entities/content.rs` | `map_content` | container; recurse em body + attribution |
| `entities/content.rs` | `map_text` | idem |
| `rules/introspect.rs` | `materialize_time` | recurse em body + attribution |
| `rules/introspect.rs` | `walk` | walk body; walk attribution se presente |
| `rules/layout/mod.rs` | `layout_content` | smart-quote insertion via `localize_quotes`; block (indent + line-attribution) vs inline (parágrafo + " — attribution") |
| `rules/layout/mod.rs` | `measure_content_constrained` | catch-all `_ => (0.0, 0.0)` cobre |

## 5. Módulo `rules/lang/quotes.rs` — assinatura + tabela

```rust
pub fn localize_quotes(lang: &Lang) -> (&'static str, &'static str);
pub const DEFAULT_QUOTES: (&str, &str) = ("\"", "\"");
```

**Tabela inicial** (6 idiomas + default ASCII):

| Lang | Open | Close |
|------|------|-------|
| `pt` | `«` (U+00AB) | `»` (U+00BB) |
| `en` | `"` (U+201C) | `"` (U+201D) |
| `de` | `„` (U+201E) | `"` (U+201C) |
| `fr` | `« ` (com U+00A0 NBSP) | ` »` (com U+00A0 NBSP) |
| `es` | `«` | `»` |
| `it` | `«` | `»` |
| (default) | `"` ASCII | `"` ASCII |

**Lookup**: exact match no `lang.as_str()`. Cristalino's `Lang` é
2-3 letras ASCII puro (ADR-0052) — sem necessidade de prefix-match
BCP47 sugerido na spec. ISO 639-2/3 codes (3 letras: ex. `por`)
caem em default a menos que sejam adicionados explicitamente à tabela.

## 6. `native_quote` — assinatura + registo

```rust
// 01_core/src/rules/stdlib/structural.rs
pub fn native_quote(...) -> SourceResult<Value> {
    let body = match args.items.first() { ... };
    let mut attribution = None;
    let mut block = false;
    let mut quotes = true;
    for (key, value) in args.named.iter() {
        match key.as_str() {
            "attribution" => { ... },
            "block" => { ... },
            "quotes" => { ... },
            other => return Err(unexpected_named(other)),
        }
    }
    Ok(Value::Content(Content::Quote {
        body: Box::new(body),
        attribution: attribution.map(Box::new),
        block, quotes,
    }))
}
```

Registado em `make_stdlib` (em `01_core/src/rules/eval/mod.rs`):

```rust
scope.define("quote", Value::Func(Func::native("quote", native_quote)));
```

Aceita posicional `body` (Content ou Str) e named `attribution` /
`block` / `quotes`. Argumentos nomeados desconhecidos → Err.
Body em falta → Err.

## 7. Markup `"..."` — diff + edge cases

**Decisão pragmática**: cristalino's lexer emite per-character
SmartQuote tokens (1 char = 1 SyntaxKind::SmartQuote). Refactorar o
lexer para parear `"..."` excederia escopo P155. Em vez disso,
`eval_markup` mantém estado de alternância open/close por sequência
markup e emite o glyph localizado como `Content::Text(glyph, style)`.

```rust
// 01_core/src/rules/eval/mod.rs::eval_markup
let mut double_open = true;  // próximo `"` é open?
let mut single_open = true;
for child in node.children() {
    match child.kind() {
        SyntaxKind::SmartQuote => {
            let is_double = child.text().as_str() == "\"";
            let lang = engine.styles.lang();
            let (open, close) = match &lang {
                Some(l) => localize_quotes(l),
                None    => DEFAULT_QUOTES,
            };
            let glyph = if is_double {
                let g = if double_open { open } else { close };
                double_open = !double_open;
                g
            } else {
                single_open = !single_open;
                "'"  // smart-apostrophes scope-out
            };
            parts.push(Content::Text(glyph.into(), style));
        }
        // ...
    }
}
```

**Edge cases tratados**:
- `"hello"` em PT → `«hello»`.
- `"hello"` em EN → `"hello"` (curly).
- `"..."` sem close: emite `«` no parágrafo (open ímpar). Aceitável
  per spec O Que Pode Sair Errado.
- `""` vazio: emite `«»` (PT) ou `""` (EN). Trivial.
- `"..."` em `Mode::Code` (e.g. `#let s = "hello"`): preservado como
  `Value::Str` (regression test obrigatório).

**Distinção contextual**: `eval_markup` só processa nós em modo
markup; `Expr::Str` em modo código resolve via `Expr::Str(node) =>
Ok(Value::Str(EcoString::from(node.get())))` em `eval_expr` (intacto).

**Limitações registadas**:
- Aspas aninhadas em markup: não suportadas (alternância simples).
- Aspas secundárias: produz `'` ASCII.
- Smart-apostrophes (`don't` → `don't`): scope-out.
- `Content::Quote` não emerge de markup `"..."`; exclusivamente de
  `#quote()` estrutural.

## 8. Tests adicionados (lista + contagens)

| Ficheiro | Testes | Total |
|----------|--------|-------|
| `01_core/src/rules/lang/quotes.rs::tests` | localize pt/en/de/fr/es+it/jp/por (3-letter) | 7 |
| `01_core/src/entities/content.rs::tests` | quote_constructor; plain_text com/sem attribution; quotes:false; is_empty; map_text recurse; partial_eq | 7 |
| `01_core/src/rules/eval/tests.rs` | eval_quote {default, attribution, block, quotes:false}; arg_invalido; sem_body; markup default ASCII; **regression code-vs-markup** | 8 |

**Total**: 1123 → **1145** (+22 = 7 + 7 + 8). Sem regressão.

Render tests (03_infra) em PDF: scope-out neste passo. Layouter
cobre via path comum (pipeline já testado em P140B/P141).

## 9. L0 prompts + hashes propagados

- `00_nucleo/prompts/entities/content.md` ganhou secção
  "Variant `Content::Quote` — Passo 155".
- **Ficheiro novo** `00_nucleo/prompts/rules/lang.md` regista
  módulo `rules/lang/` e mecanismo smart-quotes.
- Hashes recomputados via `crystalline-lint --fix-hashes`:
  - `01_core/src/entities/content.rs`: `43745b5d → 8413bb8d`.
  - `01_core/src/rules/lang/mod.rs`: novo, `4426dbc0`.
  - `01_core/src/rules/lang/quotes.rs`: novo, `4426dbc0`.
- "Hash do Código" L0:
  - `entities/content.md`: `a4244268 → 0f5177f7`.
  - `rules/lang.md` (novo): `6664c5f2`.
- Headers `@updated`:
  - `content.rs`: `2026-04-24 → 2026-04-25`.
  - `lang/{mod,quotes}.rs`: `2026-04-25` (novos).

## 10. ADR-0060 transição `PROPOSTO → IMPLEMENTADO`

```diff
- **Status**: `PROPOSTO`
+ **Status**: `IMPLEMENTADO` (Fase 1 fechada; Fase 2 e Fase 3
+ prosseguem como roadmap planeado e aplicam-se em passos
+ subsequentes — P156/157/158).
- **Validado**: Passo 154A — diagnóstico.
+ **Validado**: Passo 154A — diagnóstico; Passo 154B — sub-passo 1
+ (terms + divider); **Passo 155 — sub-passo 2 (quote); Fase 1 fechada**.
```

Anotação Passo 155 adicionada com sumário do que ficou; secção
"Plano de materialização" actualizada para registar transição
realizada (em vez de ser condicional ao fim de Fase 2 + Fase 3).

## 11. Inventário 148 actualizado

- **Tabela A Model**: 5/4/5/8/0 → **6/4/5/7/0=22** (`quote` transita
  `ausente → implementado`).
  - Cobertura Model: **41% → 45%** (10/22 → 11/22).
  - Total user-facing: 55% → ~55-56%.
- **Tabela B Content cristalino**: 42 → **43 variants** (+`Quote`).
  - Vanilla extra ausentes: ~12 → ~11.
  - Cobertura arquitectural: 75% → **75-76%**.
- **§7 entrada 7**: lista actualizada (Quote removido do agregado);
  refinamento P155 documentado registando fechamento da Fase 1.

## 12. README dos ADRs actualizado

- Tabela "Estado por ADR" linha 0060: `PROPOSTO → IMPLEMENTADO
  (Fase 1 fechada em P155; Fase 2/3 prosseguem em P156+)`.
- Distribuição: `PROPOSTO` 11→**10**; `IMPLEMENTADO` 18→**19**.
- Total inalterado (60 ADRs).
- Entrada nova em "Passos-chave da história dos ADRs" para P155
  (resumindo escopo, decisões pragmáticas, hashes propagados,
  testes, transição ADR-0060).

## 13. Próximo passo

**Passo 156 — Fase 2 sub-passo 1: `table` foundations**.
Mecânica análoga a P154B/P155: variant novo + cobertura exaustiva +
stdlib func. Particularmente:

- `Content::Table { ... }` — colunas, células, alinhamento.
- Possibilidade de delegar para `Content::Grid` em camada interna.
- DEBT-34d/e (grid cell layouting) podem ser tocadas; consultar
  estado actual antes.
- Decisão arquitectural: keep table separate ou converter para grid
  em eval (definir em P156.1).

Alternativa: P157 (figure kinds — table/equation figures) primeiro,
se priorizado pela humano. P158 (bibliography+cite) bloqueado por
DEBT-55 + ADR-0061 a criar.

## 14. Limitações registadas

- Sem show rules `#show quote: ...` neste passo.
- Sem aspas secundárias `'...'` em markup (produz `'` ASCII).
- Sem smart-apostrophes (`don't` → `don't`).
- Aspas aninhadas em markup não suportadas (alternância simples).
- 6 idiomas iniciais para smart-quotes; outras línguas (zh, ja,
  ar, ...) caem em default ASCII; expansível em passo futuro.
- Markup `"..."` produz `Content::Text(glyph)`, não `Content::Quote`
  (decisão pragmática para evitar refactor do lexer per-character).
- Render tests E2E PDF scope-out neste passo (cobertura via path
  comum).
- Hyphenation continua em `rules/layout/`; refactor unificador para
  `rules/lang/` adiado a passo separado se priorizado.

## 15. Verificação final

- ✅ `cargo build --workspace`: clean.
- ✅ `cargo test --workspace --lib`: **1145 passed; 0 failed; 6 ignored**
  (1123 → 1145 = +22 testes).
- ✅ `crystalline-lint .`: **No violations found** (incluindo V5
  PromptDrift após `--fix-hashes`).
- ✅ Hashes propagados consistentes:
  - `content.rs` ↔ `entities/content.md` = `8413bb8d`.
  - `lang/{mod,quotes}.rs` ↔ `rules/lang.md` = `4426dbc0`.
- ✅ Inventário 148 reflecte cobertura aumentada.
- ✅ ADR-0060 IMPLEMENTADO; Fase 1 fechada.
- ✅ README ADRs com distribuição + tabela + Passos-chave actualizados.
- ✅ Sem ADR nova; sem DEBT criado/fechado.
- ✅ Sem regressão (testes pré-P155 todos passam).

## Critério de conclusão

| # | Critério | Estado |
|---|----------|--------|
| 1 | `Content::Quote` compila + tests unit passam | ✅ |
| 2 | Stdlib `#quote(body, attribution: ?, block: ?, quotes: ?)` invocável | ✅ |
| 3 | Markup `"..."` em contexto markup produz aspas localizadas | ✅ (per-char alternância) |
| 4 | Markup `"..."` em código continua `Value::Str` (regressão evitada) | ✅ |
| 5 | Smart-quotes funcionam para 6 langs + default | ✅ (7 testes) |
| 6 | ADR-0060 IMPLEMENTADO; Fase 1 fechada | ✅ |
| 7 | Inventário 148 reflecte cobertura aumentada | ✅ |
| 8 | Próximo passo (156 = table foundations) tem âncora | ✅ |
| 9 | Sem regressão | ✅ |
| 10 | Relatório do passo escrito | ✅ |
