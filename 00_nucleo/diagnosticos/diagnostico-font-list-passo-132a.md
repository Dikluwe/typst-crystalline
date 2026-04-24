# Diagnóstico de `FontList` — Passo 132A

**Data**: 2026-04-24
**ADR alvo**: **ADR-0053** — "Font como tipo composto em L1"
(criado em 132A.6).
**Motivação**: `text.font` é a propriedade residual mais complexa
da lista DEBT-1. Vanilla aceita 3 formas (string / array / dict
com `covers`). Captura raw violaria ADR-0033 como no caso lang
(resolvido via ADR-0052). Este diagnóstico cumpre ADR-0034 antes
de materializar tipo composto em L1.

---

## 1. Localização no vanilla

- **Ficheiro principal**: `lab/typst-original/crates/typst-library/src/text/mod.rs`
- **Linhas-chave**:
  - `170`: `pub font: FontList` em `TextElem`.
  - `835-884`: `FontFamily` struct + `impl` + `cast!`.
  - `886-939`: `Covers` enum + `impl` + `cast!`.
  - `941-976`: `FontList` struct + `impl IntoIterator` + `cast!`.

## 2. Campos / variantes

### `FontFamily`

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct FontFamily {
    name: EcoString,          // lowercased
    covers: Option<Covers>,
}
```

- `name`: EcoString **lowercased** no construtor.
- `covers`: optional — filtro de coverage.

### `Covers`

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub enum Covers {
    LatinInCjk,               // keyword "latin-in-cjk"
    Regex(Regex),             // regex::Regex from crate externa
}
```

**1 keyword especial + regex arbitrário** (restrito a character
classes + dot + literal via AST validation).

### `FontList`

```rust
#[derive(Debug, Default, Clone, PartialEq, Hash)]
pub struct FontList(pub Vec<FontFamily>);
```

- Simples newtype sobre `Vec<FontFamily>`.
- **Não vazio obrigatório**: `FontList::new` emite
  `"font fallback list must not be empty"`.

## 3. Operadores / métodos

### `FontFamily`

```rust
impl FontFamily {
    pub fn new(string: &str) -> Self;                     // covers=None
    pub fn with_coverage(s: &str, covers: Option<Covers>) -> Self;
    pub fn as_str(&self) -> &str;                         // name lowercased
    pub fn covers(&self) -> Option<&Regex>;               // requires Regex
}
```

### `Covers`

```rust
impl Covers {
    pub fn as_regex(&self) -> &Regex;    // LatinInCjk → singleton regex
}
```

### `FontList`

```rust
impl FontList {
    pub fn new(fonts: Vec<FontFamily>) -> StrResult<Self>;
}

impl<'a> IntoIterator for &'a FontList { /* ... */ }
impl Default for FontList { /* ... */ }
```

### `cast!` — accepts 3 forms

```rust
cast! {
    FontList,
    family: FontFamily => Self(vec![family]),             // "Arial"
    values: Array => Self::new(values.cast...)?,          // ("A","B")
}

cast! {
    FontFamily,
    string: EcoString => Self::new(&string),              // "A"
    mut v: Dict => with_coverage(                         // (name:..,covers:..)
        &v.take("name")?.cast::<EcoString>()?,
        v.take("covers").ok().map(|v| v.cast()).transpose()?
    ),
}

cast! {
    Covers,
    "latin-in-cjk" => Covers::LatinInCjk,
    regex: Regex => { /* AST validation */ Covers::Regex(regex) },
}
```

## 4. Dependências

### Crates externas usadas em vanilla

- **`ecow::EcoString`** — ✓ autorizado em L1 (ADR-0024).
- **`regex::Regex`** — ❌ **NÃO autorizado** em L1. Nem está
  declarado em workspace `Cargo.toml`. `crystalline.toml`
  `[l1_allowed_external]` lista: `thiserror`, `comemo`,
  `unicode_ident`, `unicode_math_class`, `unicode_script`,
  `unicode_segmentation`, `rustc_hash`, `time`, `indexmap`,
  `ecow`. **Sem `regex`**.
- **`regex_syntax`** — idem, não autorizado. Usado apenas no
  `cast!` Covers para AST validation.

### Tipos internos vanilla referenciados

- `Packed`, `PlainText`, `Content`, `Engine`, `Dict`, `Array`,
  `cast!`, `bail!`, `dict!`, `hint!`, `StrResult`,
  `HintedStrResult` — toda a infra foundations de vanilla.
- Cristalino tem `Value::Dict`, `Value::Array`, `Value::Str`
  nativos.

## 5. Semântica

### Forma 1 — string simples

```
#set text(font: "Arial")
// → FontList of 1 FontFamily { name: "arial", covers: None }
```

Lowercase normalization no construtor. Qualquer string não-vazia
aceite.

### Forma 2 — array

```
#set text(font: ("A", "B", "C"))
// → FontList of 3 FontFamilies, prioridade na ordem
```

Cada elemento cast para `FontFamily`. **Array vazio** → Err
"font fallback list must not be empty".

### Forma 3 — dict (com ou sem covers)

```
#set text(font: (name: "Arial"))
// → FontList of 1 { name, covers: None }

#set text(font: (name: "Arial", covers: "latin-in-cjk"))
// → FontList of 1 { name, covers: Some(LatinInCjk) }

#set text(font: (name: "Arial", covers: regex("\d")))
// → FontList of 1 { name, covers: Some(Regex(...)) }
```

### Forma 4 — array misto

```
#set text(font: ((name: "X", covers: "latin-in-cjk"), "Y"))
// Array que mistura dict e string.
```

### Casos especiais

- Dict sem `name` → `v.take("name")?` falha com erro do cast
  macro.
- Dict com keys desconhecidas → `v.finish(&["name", "covers"])?`
  erra.
- `covers: "other-keyword"` → erro (keyword não registada).
- `covers: regex(".*")` → erro AST validation ("regex may only
  use dot, letters, and character classes").

### Fallback chain

Array define prioridade. Consumer (shaping engine, vanilla
Typst runtime) tenta cada família em ordem; se `covers`
definido, só aplica para codepoints que match.

## 6. Mensagens de erro

**FontList::new**:
`"font fallback list must not be empty"`

**Dict sem name**:
via `v.take("name")?` — mensagem do cast macro:
`"expected 'name' key"` ou similar (variação do cast framework).

**Dict com key desconhecido**:
via `v.finish(&["name", "covers"])?` — mensagem:
`"unexpected key ..."`.

**Covers regex inválido**:
```
"coverage regex may only use dot, letters, and character classes"
hint: "the regex is applied to each letter individually"
```

## 7. Divergências propostas para L1

### Decisão chave sobre `regex`

**Opção B estrita (recomendada)**: **deferir `covers` por
completo neste passo**. Capturar apenas formas string e array de
strings. Dict com ou sem `covers` → erro hard.

**Justificação**:
1. `regex` não autorizada em L1 e ADR complementar para
   autorizar seria passo substancial próprio (4-6 dependências
   transitivas: `aho-corasick`, `memchr`, `regex-automata`,
   `regex-syntax`).
2. `covers` é funcionalidade **avançada** — mix de fontes por
   script. Zero consumers hoje (sem shaping engine ainda).
3. **Sempre emitimos erro** quando vanilla aceita (diff menos
   grave que aceitar quando vanilla erra).
4. Quando consumer (shaping) for implementado num passo
   dedicado, reabrir decisão sobre `regex` com contexto real.

**Opção A descartada**: autorizar `regex` em L1 é decisão
grande para feature sem consumer.

**Opção C descartada**: suportar `LatinInCjk` keyword mas não
regex seria materialização parcial — `covers` fica num estado
meio-funcional confuso.

### Forma interna proposta

```rust
// 01_core/src/entities/font_list.rs

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FontFamily {
    name: EcoString,          // lowercased
}

impl FontFamily {
    pub fn new(name: &str) -> Self;
    pub fn as_str(&self) -> &str;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FontList(Vec<FontFamily>);

impl FontList {
    pub fn new(families: Vec<FontFamily>) -> Result<Self, &'static str>;
    pub fn families(&self) -> &[FontFamily];
}

impl<'a> IntoIterator for &'a FontList { /* ... */ }
```

**Sem** `covers`. **Sem** `Covers` enum. **Sem** dependência
`regex`. Campo `covers` fica ausente — `FontFamily` simplifica
para só `name`. Se consumer futuro implementar `covers`,
reabrir ADR-0053 ou criar ADR-0054 complementar.

### Validação / parser

Sem `impl FromStr` (não é string → struct um-a-um como `Lang`).
Usa **função construtora** `FontList::from_value(&Value) ->
Result<Self, SourceDiagnostic>`:

- `Value::Str(s)` → `Ok(FontList(vec![FontFamily::new(&s)]))`.
- `Value::Array(items)` onde cada item é `Value::Str` →
  `Ok(FontList(vec![items...]))`. Array vazio → Err.
- `Value::Array(items)` com algum item não-string → Err.
- `Value::Dict(_)` → Err **"dict form not supported — use string or array"**
  (divergência explícita vs vanilla).

### Mensagens de erro propostas

- **Tipo errado**: `"expected string or array of strings"`
  — quando valor não cabe em nenhuma forma suportada.
- **Array vazio**: `"font fallback list must not be empty"`
  — réplica literal vanilla.
- **Dict não suportado**: `"dict form of font not yet supported — use string or array of strings"`
  — divergência explícita.
- **Item de array inválido**: `"font family must be a string"`.

### Constantes

**Nenhuma**. Nomes de fonte são infinitos; não há sentido em
listar canónicas em L1.

### Localização em L1

**`01_core/src/entities/font_list.rs`** (**NOVO**).

**Razão (ADR-0037)**:
- `font_book.rs` é **catálogo** (`FontInfo`, `FontWeight`,
  `FontStretch`, `FontBook::select`).
- `font_list.rs` é **selector** (user-facing: família + fallback).
- Diferentes domínios; ficheiros separados. Futuro consumer
  de shaping une os dois.

---

## Itens adicionais

### Impacto em call-sites

#### `01_core/src/entities/style_chain.rs`

- `use crate::entities::font_list::FontList;` — adicionar.
- `StyleDelta.font: Option<FontList>` — **novo campo**.
- `StyleDelta::empty()` → `font: None`.
- `const fn empty()` — `FontList` é `Clone` não `Copy`;
  `Option::<FontList>::None` é const OK.

#### `01_core/src/rules/eval/rules.rs`

- `use crate::entities::font_list::FontList;` — adicionar.
- **Arm `"font"` novo** com validação:

```rust
"font" => {
    match FontList::from_value(&val) {
        Ok(fl) => delta.font = Some(fl),
        Err(msg) => {
            return Err(vec![SourceDiagnostic::error(
                named.expr().span(),
                msg.to_string(),
            )]);
        }
    }
}
```

### Plano de teste para 132B

#### Unit tests em `entities/font_list.rs` (10-12)

- `font_family_new_lowercases`: `FontFamily::new("Arial")` →
  `as_str() == "arial"`.
- `font_list_new_single_aceita`.
- `font_list_new_multiple_aceita`.
- `font_list_new_vazio_devolve_erro`.
- `font_list_from_value_string_aceita`.
- `font_list_from_value_array_de_strings_aceita`.
- `font_list_from_value_array_vazio_devolve_erro`.
- `font_list_from_value_array_com_item_nao_string_devolve_erro`.
- `font_list_from_value_dict_nao_suportado_devolve_erro`.
- `font_list_from_value_tipo_errado_devolve_erro`.
- `font_list_iter_preserva_ordem`.
- `font_list_preserva_case_original_do_token`: (opcional —
  lowercase normalization visível).

#### Integration tests em `rules/eval/tests.rs` (4-5)

- `eval_set_text_font_string_simples_passo_132b`: `"Arial"`
  → Ok, captura.
- `eval_set_text_font_array_passo_132b`: `("A", "B")` → Ok.
- `eval_set_text_font_array_vazio_erro_passo_132b`.
- `eval_set_text_font_dict_erro_passo_132b`.
- `eval_set_text_font_tipo_errado_erro_passo_132b`.

### Canary migration — lista completa

**`font` deixa de emitir warning em 132B**. Todos os tests que
usam `#set text(font: ...)` como canary precisam migrar para
**`hyphenate: true`** (ou outro unknown — ver decisão abaixo).

#### L1 — 5 canary tests em `rules/eval/tests.rs`

| Linha | Nome | Acção 132B |
|------:|------|------------|
| 1259 | `eval_set_text_font_canary_passo_126` | renomear → `hyphenate_canary_passo_132b_x126`; input `hyphenate: true`; assert `'hyphenate'` |
| 1309 | `eval_set_text_font_canary_passo_127` | idem |
| 1360 | `eval_set_text_font_canary_passo_128` | idem |
| 1463 | `eval_set_text_font_canary_passo_129` | idem |
| 1566 | `eval_set_text_font_canary_passo_131b` | idem |

Alternativa mais simples: **consolidar 5 tests duplicados num
único `hyphenate_canary_passo_132b`**, renomeando os 4
anteriores como removed (ou deixando só 1 pois todos fazem a
mesma asserção).

#### L3 — 3 tests em `integration_tests.rs`

| Linha | Nome | Acção 132B |
|------:|------|------------|
| 2180 | `debt49_set_text_font_emite_warning` | **INVERTER** — `font` agora captura Ok. Renomear → `debt49_set_text_font_valido_captura_passo_132b`. Assert `result.is_ok()` e `warnings.is_empty()`. |
| 2229 | `debt49_set_text_multiplas_propriedades_desconhecidas` | Rotar: `font: "A"` → `hyphenate: true`. Trio passa a `hyphenate/alignment/stroke`. |
| 2285, 2295 | `debt49_dedup_warnings_identicos` | Input usa `#set text(font: "X")` 2× para testar dedup; **rotar** para `#set text(hyphenate: true)` 2×. |

#### L4 — 2 tests em `04_wiring/tests/cli.rs`

| Linha | Nome | Acção 132B |
|------:|------|------------|
| 65 | `cli_sucesso_com_warning` | rotar `font: "Arial"` → `hyphenate: true`. Asserção muda de `stderr contém "font"` para `stderr contém "hyphenate"`. |
| 591 | `disciplina_warnings_antes_de_errors` | rotar input `font: "X"` → `hyphenate: true`. |

#### Total canary migration impact

- **5 L1 + 3 L3 + 2 L4 = 10 testes adaptados**.

**Substituto do canary**: **`hyphenate`**. Confirmado:
- Vanilla `pub hyphenate: Smart<bool>` — aceita valor `true`/`false`.
- Cristalino não tem `Smart<_>` nem captura `hyphenate`.
- `#set text(hyphenate: true)` é parseável (bool literal),
  eval processa, arm default emite warning "'hyphenate' ainda
  não suportada". **Canary válido**.
- Verificado que `hyphenate` não aparece em nenhum teste actual
  (zero risco de colisão).

### Plano de migração DEBT-49 L3 pool

Post-132B pool muda:
- **Antes**: `font, alignment, stroke`.
- **Depois**: `hyphenate, alignment, stroke`.

`hyphenate` entra no pool; `font` sai (capturado).
Saudável para mais alguns passos.

### ADR-0038 ganha nota?

**Recomendação: NÃO** neste passo. ADR-0038 já tem 4 notas.
132B é refactor por paridade como 131B — ADR-0053 dedicada
absorve. Quarta nota de 131B explica que futuras paridades
seguem padrão "ADR própria", não agregam em 0038.

### Implicações arquitecturais

- **Primeira materialização de tipo composto** (`FontList`
  contém `FontFamily`). Precedente para `Stroke` (width +
  paint) e outros tipos compostos futuros.
- **Primeira rejeição explícita de form suportada vanilla**:
  dict não suportado. ADR-0033 semi-violada: rejeita mais
  strict que vanilla. **Aceite** como trade-off (reduz
  surface vs. adição de `regex` não autorizada).
- **Divergência reportada**: ADR-0053 documenta como
  "paridade limitada a string + array; dict com `covers`
  deferido". Futuro passo quando shaping chegar reabre.

---

## Resumo executivo

**Decisões-chave**:
1. `FontList(Vec<FontFamily>)` simples, com `FontFamily {
   name: EcoString }`.
2. **Sem `covers`** — dict form rejeitado com mensagem clara.
3. Suporte para 2 das 3 formas vanilla: string + array.
4. `FontList::from_value(&Value) -> Result<Self, &str>`.
5. Arm `"font"` emite Err hard em inválido (padrão 131B).
6. Ficheiro dedicado `entities/font_list.rs`.
7. Canary DEBT-50 migra `font` → `hyphenate` em 10 testes
   distribuídos L1/L3/L4.

**Estimativa 132B**: **S-M**. 1 ficheiro novo (`font_list.rs`
~120 linhas + tests), 2 ficheiros modificados L1, 10 testes
adaptados em 3 camadas. 2-3h implementação + validação.

**Bloqueios**: nenhum. Zero crates novas. Decisão estratégica
de deferir `covers` elimina pressão sobre `regex`
authorization.
