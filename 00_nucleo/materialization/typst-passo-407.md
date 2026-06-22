# Passo 407 — `text.font` dict: pattern-matching por regex (S-M)

**Tipo**: Refinamento de feature existente (L1 — stdlib + eval; zero tipo novo; zero I/O; reusa `Value::Regex` P402 + `FontList` P140B/P141/P146).
**Data**: 2026-06-22.
**Padrão**: diagnóstico-primeiro (DEBT-52 + Tabela A.3 linha 376); medir-antes-de-decidir (ADR-0108).
**ADRs relevantes**: ADR-0033 (paridade vanilla), ADR-0107 (paridade linguagem), ADR-0054 (graded scope-out — `FontVariant` selection variant-aware, DEBT-53 shaping XL).
**Sonda fonte**: Tabela C linha 376 — `text.font` dict bloqueado por `Value::Regex` ausente em L1 (DEBT-52, condicional ADR-0054bis).

> **Nota de numeração.** Um passo só. Não numerar à frente.
> **Nota de marco.** Este passo fecha **DEBT-52** (`text.font` dict) — último débito técnico aberto que travava uma feature user-facing. Após P407, saldo de DEBTs abertos: 11 → **10**.

---

## 1. Contexto

DEBT-52 documentava que `text.font` dict (pattern-matching por regex em font names) estava bloqueado porque `Value::Regex` não existia como tipo de primeiro-cidadão em L1. P402 resolveu isso — `Value::Regex(Regex)` é agora tipo L1 dedicado com `PartialEq` por pattern, `is_match`, e `Arc<regex::Regex>`.

No vanilla:
```typ
#set text(font: ("Name": ("Regular", "Bold")))
#set text(font: ("Name": ("Regular", "Bold", "Italic")))
#set text(font: ("Name": ("Regular", "Bold", "Italic", "Bold Italic")))
// ou com regex:
#set text(font: (regex("Name.*"): ("Regular", "Bold")))
```

O dict de fontes mapeia **font family name** (string ou regex) para **array de variant names** (strings). O `FontList` (P140B/P141/P146) itera sobre as entradas do dict e resolve cada uma via `FontBook::select`.

No cristalino, `FontList` é `Vec<FontFamily>` onde `FontFamily` contém `name: EcoString` e `variants: Vec<EcoString>`. Este passo adiciona suporte para **regex keys** no dict.

---

## 2. Decisão de engenharia

### 2.1 — Estrutura `FontFamily` refino

```rust
// entities/font.rs (ou onde FontFamily está definido)
pub struct FontFamily {
    pub name: FontNamePattern,  // antes: EcoString
    pub variants: Vec<EcoString>,
}

pub enum FontNamePattern {
    Literal(EcoString),
    Regex(Regex),  // entities::regex::Regex (P402)
}
```

**Decisão ADR-0107**: a paridade é com a **forma** do vanilla — dict key pode ser string literal ou regex. A mecânica interna (enum vs string com flag) é invisível.

**Decisão**: `FontNamePattern` é enum novo em L1. Alternativa: `EcoString` + `bool is_regex` — menos type-safe. Enum é preferido (ADR-0029 pureza L1, type safety).

**Impacto em `FontList`**: `Vec<FontFamily>` permanece; apenas `FontFamily.name` muda de `EcoString` para `FontNamePattern`.

### 2.2 — Parsing de `text.font` dict

Atualmente, `text.font` aceita:
- `Str` → `FontList` com um `FontFamily` (name literal, variants vazio ou default).
- `Array[Str]` → `FontList` com múltiplos `FontFamily` (cada um name literal, variants vazio).

Este passo adiciona:
- `Dict` → `FontList` com múltiplos `FontFamily`:
  - Key `Str` → `FontNamePattern::Literal(name)`.
  - Key `Regex` → `FontNamePattern::Regex(regex)`.
  - Value `Str` → `vec![variant]` (single variant).
  - Value `Array[Str]` → `variants` (multiple variants).

**Decisão**: se o vanilla aceita `Dict` com value como `Str` ou `Array[Str]`, implementar ambos. Se só aceita `Array[Str]`, implementar só isso. Verificar vanilla behavior.

**Simplificação**: assumir que o vanilla aceita `Str` e `Array[Str]` como value (flexibilidade comum em Typst). Implementar ambos; se o vanilla for mais restrito, ajustar em passo futuro (XS).

### 2.3 — Resolução de fonte com regex

`FontBook::select(name: &str)` (ou similar) itera sobre fontes disponíveis e retorna a que matcha o nome. Com regex:

```rust
impl FontNamePattern {
    pub fn is_match(&self, name: &str) -> bool {
        match self {
            FontNamePattern::Literal(lit) => lit == name,
            FontNamePattern::Regex(re) => re.is_match(name).unwrap_or(false),
        }
    }
}
```

**Decisão**: `FontBook::select` (ou `FontList::resolve`) itera sobre `FontFamily` entries em ordem. Para cada entry, se `FontNamePattern::is_match(name)` retorna true, retorna as variants. Primeiro match wins (ordem do dict preservada).

**Nota**: `FontBook::select` atual provavelmente faz lookup exato (`HashMap` ou similar). Com regex, não pode usar HashMap — deve iterar linearmente. Performance: O(n) por lookup vs O(1) anterior. Aceitável para S-M (font lists são pequenas, tipicamente < 10 entries).

### 2.4 — `FontList` clone e PartialEq

`FontList` (P140B/P141/P146) é usado em `Style::Font(FontList)`. Se `FontList` é `Clone` (provavelmente é, via derive), `FontNamePattern` deve ser `Clone` — `Regex` já é `Clone` (P402, Arc interno).

`PartialEq` de `FontList`: se usado em `Style` para equality, `FontNamePattern::PartialEq` deve ser consistente. `Regex::PartialEq` é por pattern (P402) — suficiente.

---

## 3. FASE A — L0 (redação; checkpoint obrigatório)

### A.1 — Prompt L0 `font-dict.md`

Extensão de `00_nucleo/prompts/entities/font.md` (ou novo `00_nucleo/prompts/rules/style/font-dict.md`):

- **Paridade**: `text.font: ("Name": ("Regular", "Bold"))` ≡ vanilla dict font pattern-matching.
- **Substrato**: reusa `FontList` (P140B/P141/P146) + `Value::Regex` (P402); refino de `FontFamily.name` para `FontNamePattern` enum.
- **Sem tipo novo**: reusa `Regex` (P402), `EcoString`, `FontList`.
- **Parsing de `text.font`**:
  - `Str` → `FontList` com 1 `FontFamily` (Literal, variants vazio).
  - `Array[Str]` → `FontList` com N `FontFamily` (Literal cada, variants vazio).
  - `Dict` → `FontList` com N `FontFamily`:
    - Key `Str` → `FontNamePattern::Literal(name)`.
    - Key `Regex` → `FontNamePattern::Regex(regex)`.
    - Value `Str` → `vec![variant]`.
    - Value `Array[Str]` → `variants`.
- **Resolução**: `FontNamePattern::is_match(name)` → Literal (exacto) ou Regex (pattern match). Primeiro match wins.
- **Performance**: O(n) linear scan para regex keys; O(1) hash lookup para literals (otimização futura, scope-out).
- **Erros**: key não-Str/não-Regex → Err; value não-Str/não-Array → Err; Array com não-Str → Err.
- **Teste**: `#set text(font: ("Name": ("Regular", "Bold")))` parse+eval → `FontList` com 1 entry; `FontNamePattern::is_match("Name")` → true; `FontNamePattern::is_match("Other")` → false.

### A.2 — CHECKPOINT

Parar. Apresentar extensão `font.md` (ou `font-dict.md`) ao dono. **Só prosseguir para Fase B quando confirmar que guardou e computou hash.**

---

## 4. FASE B — Código (após confirmação humana)

### B.1 — `FontNamePattern` enum

Em `entities/font.rs` (ou onde `FontFamily` está definido):

```rust
use crate::entities::regex::Regex;
use ecow::EcoString;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum FontNamePattern {
    Literal(EcoString),
    Regex(Regex),
}

impl FontNamePattern {
    pub fn is_match(&self, name: &str) -> bool {
        match self {
            FontNamePattern::Literal(lit) => lit.as_str() == name,
            FontNamePattern::Regex(re) => re.is_match(name).unwrap_or(false),
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            FontNamePattern::Literal(lit) => Some(lit.as_str()),
            FontNamePattern::Regex(_) => None,
        }
    }
}
```

### B.2 — Refino `FontFamily`

```rust
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FontFamily {
    pub name: FontNamePattern,
    pub variants: Vec<EcoString>,
}

impl FontFamily {
    pub fn new_literal(name: EcoString, variants: Vec<EcoString>) -> Self {
        Self {
            name: FontNamePattern::Literal(name),
            variants,
        }
    }

    pub fn new_regex(regex: Regex, variants: Vec<EcoString>) -> Self {
        Self {
            name: FontNamePattern::Regex(regex),
            variants,
        }
    }
}
```

### B.3 — Ajuste `FontList` parsing

Atualmente, `text.font` provavelmente é parseado em `eval/rules.rs` (set rule) ou `stdlib/text.rs`. Localizar o parsing atual e ajustar:

```rust
// Antes (provavelmente):
// Value::Str(name) => FontList::from_single(name.into()),
// Value::Array(arr) => FontList::from_array(arr.iter().map(|v| v.cast::<Str>().map(|s| s.into())).collect::<Result<_,_>>()?),

// Depois:
Value::Str(name) => Ok(FontList::from_single(FontFamily::new_literal(name.into(), vec![]))),
Value::Array(arr) => {
    let families = arr.iter()
        .map(|v| {
            let name = v.cast::<Str>()?;
            Ok(FontFamily::new_literal(name.into(), vec![]))
        })
        .collect::<SourceResult<Vec<_>>>()?;
    Ok(FontList::from_families(families))
},
Value::Dict(dict) => {
    let families = dict.iter()
        .map(|(key, value)| {
            let name = match key {
                Value::Str(s) => FontNamePattern::Literal(s.clone().into()),
                Value::Regex(re) => FontNamePattern::Regex(re.clone()),
                _ => bail!("chave de font dict deve ser string ou regex"),
            };
            let variants = match value {
                Value::Str(s) => vec![s.clone().into()],
                Value::Array(arr) => arr.iter()
                    .map(|v| v.cast::<Str>().map(|s| s.clone().into()))
                    .collect::<Result<Vec<_>, _>>()?,
                _ => bail!("valor de font dict deve ser string ou array de strings"),
            };
            Ok(FontFamily { name, variants })
        })
        .collect::<SourceResult<Vec<_>>>()?;
    Ok(FontList::from_families(families))
},
```

**Nota**: `FontList::from_single`, `FontList::from_families` — verificar se existem. Se `FontList` é `Vec<FontFamily>`, construir diretamente.

### B.4 — Ajuste `FontBook::select` (ou equivalente)

Localizar onde a fonte é resolvida (provavelmente em `layout/text.rs` ou `rules/layout/text.rs`):

```rust
// Antes (provavelmente lookup exato):
// let font = font_book.select(&family.name);

// Depois:
let font = match &family.name {
    FontNamePattern::Literal(name) => font_book.select(name),  // O(1) se hash map
    FontNamePattern::Regex(re) => {
        // O(n) linear scan
        font_book.iter()
            .find(|(name, _)| re.is_match(name).unwrap_or(false))
            .map(|(_, font)| font.clone())
    }
};
```

**Simplificação**: se `FontBook::select` já itera (não é hash map), apenas ajustar para usar `FontNamePattern::is_match`:

```rust
// Em FontBook::select ou FontList::resolve:
pub fn select(&self, pattern: &FontNamePattern) -> Option<Font> {
    self.fonts.iter()
        .find(|(name, _)| pattern.is_match(name))
        .map(|(_, font)| font.clone())
}
```

**Decisão**: se o cristalino usa `FontBook` como `HashMap<EcoString, Font>`, regex requer iteração linear. Implementar iteração linear para regex; literals continuam com hash lookup (se possível). Se `FontBook` não é HashMap, apenas ajustar `is_match`.

### B.5 — Ajuste `Style::Font(FontList)`

`Style::Font(FontList)` (P292) deve continuar funcionando sem mudanças — `FontList` é `Clone`, `FontFamily` é `Clone`, `FontNamePattern` é `Clone`. Verificar se `Style` derive `Clone` e se `FontList: Clone`.

### B.6 — Testes

1. **Unit `entities/font.rs`** (4-5 tests):
   - `font_name_pattern_literal` — `FontNamePattern::Literal("Name")`.
   - `font_name_pattern_regex` — `FontNamePattern::Regex(Regex::new("Name.*"))`.
   - `font_name_pattern_is_match_literal` — `Literal("Name").is_match("Name")` → true; `Literal("Name").is_match("Other")` → false.
   - `font_name_pattern_is_match_regex` — `Regex("Name.*").is_match("Name Bold")` → true; `Regex("Name.*").is_match("Other")` → false.
   - `font_family_new_literal` — `FontFamily::new_literal("Name", vec!["Regular"])`.
   - `font_family_new_regex` — `FontFamily::new_regex(Regex::new("Name.*"), vec!["Regular"])`.
   - `font_family_clone` — clone compartilha Regex Arc.
   - `font_family_partial_eq` — equality por name pattern + variants.

2. **Unit eval / stdlib** (6-8 tests em `stdlib/text.rs` ou `eval/rules.rs`):
   - `text_font_str` — `#set text(font: "Name")` → `FontList` com 1 Literal.
   - `text_font_array` — `#set text(font: ("Name1", "Name2"))` → `FontList` com 2 Literals.
   - `text_font_dict_literal_key` — `#set text(font: ("Name": ("Regular", "Bold")))` → `FontList` com 1 Literal + variants.
   - `text_font_dict_regex_key` — `#set text(font: (regex("Name.*"): ("Regular", "Bold")))` → `FontList` com 1 Regex + variants.
   - `text_font_dict_mixed_keys` — `#set text(font: ("Name": ("Regular"), regex("Other.*"): ("Bold")))` → 2 entries.
   - `text_font_dict_str_value` — `#set text(font: ("Name": "Regular"))` → variants=["Regular"].
   - `text_font_dict_invalid_key` — `#set text(font: (123: ("Regular")))` → Err.
   - `text_font_dict_invalid_value` — `#set text(font: ("Name": 123))` → Err.
   - `text_font_dict_invalid_array_value` — `#set text(font: ("Name": (123)))` → Err.

3. **Integration / E2E** (2-3 tests):
   - `text_font_dict_pipeline` — parse + eval + `#set text(font: ("Name": ("Regular", "Bold")))` → `FontList` no Style.
   - `text_font_dict_regex_pipeline` — parse + eval + `#set text(font: (regex("Name.*"): ("Regular")))` → `FontList` com Regex.
   - `text_font_dict_repr` — `repr(font dict)` ou style inspection.

### B.7 — Linhagem

- `@prompt` aponta para extensão `font.md` (ou `font-dict.md`).
- `@prompt-hash` via `--fix-hashes`.
- Referência cruzada: P402 (`Value::Regex` tipo L1), P140B/P141/P146 (`FontList` baseline), P292 (`Style::Font`), DEBT-52 (fechamento), ADR-0107 (paridade linguagem).

---

## 5. O que NÃO fazer (scope-out)

- **Não** implementar `FontVariant` selection variant-aware (Tabela A.3 linha 376) — scope-out ADR-0054bis condicional (requer `FontVariant` tipo + shaping XL).
- **Não** implementar otimização de lookup O(1) para literals em dict com regex — scope-out ADR-0054 graded (futuro XS; performance).
- **Não** implementar `text.font` com `Dict` aninhado (ex.: `("Name": ("weight": "Regular"))`) — não é o vanilla; o vanilla usa array de variant names.
- **Não** implementar fallback chain para regex (múltiplos matchs) — scope-out ADR-0054 graded (primeiro match wins é suficiente).
- **Não** adicionar tipo novo — reusa P402 Regex.
- **Não** quebrar invariantes de camada (L1 puro, zero I/O).

---

## 6. Critérios de aceitação

1. `#set text(font: ("Name": ("Regular", "Bold")))` parseia e evalua para `FontList` com `FontFamily` Literal.
2. `#set text(font: (regex("Name.*"): ("Regular", "Bold")))` parseia e evalua para `FontList` com `FontFamily` Regex.
3. `FontNamePattern::is_match("Name")` → true para Literal("Name").
4. `FontNamePattern::is_match("Name Bold")` → true para Regex("Name.*").
5. `FontList` continua `Clone` + `PartialEq` (nenhuma regressão em P292 `Style::Font`).
6. Zero tipo novo; zero I/O; zero variant novo.
7. Testes verdes (≥12 unit + 6-8 stdlib + 2-3 integration); lint zero; hashes propagados.
8. Inventário 148: `text.font` dict transita `ausente` (DEBT-52) → `implementado`; `text.font` dict com variant selection permanece `scope-out` (ADR-0054bis condicional).
9. DEBT-52 **fechado** — saldo DEBTs: 11 → **10**.
10. L0 salvo e hashado antes do código (protocolo de nucleação).
11. Ritmo S-M: tempo de ciclo comparável a P404-P406 (baseline de activação de tipo S modelado).

---

## 7. O que pode sair errado

- **`FontFamily` não é `Clone` ou `PartialEq` — ou depende de `Font` que não é.** Mitigação: verificar P140B/P141/P146. `FontNamePattern` é `Clone` + `PartialEq`; se `FontFamily` derive estes traits, deve funcionar. Se `Font` (o tipo de fonte real) não é `Clone`/`PartialEq`, `FontFamily` não pode derive — verificar e ajustar manualmente.
- **`FontBook::select` usa `HashMap<EcoString, Font>` — regex requer refactor para iterável.** Mitigação: se `FontBook` é HashMap, adicionar método `iter()` que retorna iterator de `(name, font)`. Se não existe, adicionar. Se `FontBook` já é iterável, apenas ajustar `select` para aceitar `FontNamePattern`.
- **`text.font` parsing atual não está em um único lugar — está espalhado entre set rule e stdlib.** Mitigação: localizar todos os call sites que constroem `FontList` a partir de `Value`. Grep por `FontList::` ou `FontFamily::` ou `text.font` ou `font:` em `eval/` e `stdlib/`.
- **`FontList` é usado em `Style::Font` que é `Copy` (ou era `Copy` antes de P292).** Mitigação: P292 removeu `Copy` de `Style` porque `FontList: !Copy`. Verificar se `Style` já é `Clone` (não Copy) — deve ser, desde P292.
- **Tentação de já implementar `FontVariant` selection.** Mitigação: ADR-0054bis condicional; requer `FontVariant` tipo + shaping XL (DEBT-53). Este passo é S-M, não XL.
- **Tentação de otimizar lookup para literals.** Mitigação: scope-out ADR-0054 graded; O(n) linear é suficiente para listas pequenas (< 10 entries).

---

## 8. Referências

- P402 — `Value::Regex` tipo L1 (baseline para regex keys).
- P140B/P141/P146 — `FontList` / `FontFamily` / `Style::Font` baseline.
- P292 — `Style::Font(FontList)` (último variant Style; perdeu Copy).
- DEBT-52 — `text.font` dict bloqueado por regex ausente em L1.
- ADR-0107 — paridade linguagem vs mecânica (dict key string ou regex).
- ADR-0054bis condicional — `FontVariant` selection variant-aware (scope-out).

---

## 9. Nota sobre o Tekt

Este passo é **fecho de débito técnico** — DEBT-52 era a última trava real de uma feature user-facing (`text.font` dict). A dependência (`Value::Regex` L1) foi resolvida em P402; este passo consome a dependência.

O padrão é: **tipo modelado (S) → ativação de tipo (S-M) → consumer que dependia do tipo (S-M)**. P402 (Regex tipo) → P407 (font dict consumer). O ritmo de P407 deve ser comparável a P404-P406 (ativação de tipo) — se for significativamente mais lento, o consumer tem complexidade oculta não mapeada.

Registar o tempo de ciclo de P407 como **baseline de fecho de débito técnico** — comparar com outros DEBTs futuros (DEBT-53 shaping, DEBT-56 column flow, etc.).
