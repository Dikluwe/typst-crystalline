# Prompt L0 — entities/font-list
Hash do Código: ddcbfc54

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/font_list.rs`
**ADRs relevantes**: ADR-0053 (materialização), ADR-0033 (paridade vanilla),
ADR-0037 (coesão por domínio), ADR-0107 (paridade linguagem), DEBT-52

## Contexto

`FontList` é a lista priorizada de famílias de fonte usada por
`#set text(font: ...)`. Materializado no Passo 132B para obter
paridade parcial com o Typst vanilla. P407 (fecho de DEBT-52)
activa a forma dict com regex keys, introduzindo `FontNamePattern`.

## Interface pública

```rust
/// Enum inabitado — reserva forma estrutural para futuro coverage filtering.
pub enum Covers {}

/// Padrão de nome de família: literal exacto ou regex.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum FontNamePattern {
    Literal(EcoString),
    Regex(Regex),  // entities::regex::Regex (P402)
}

impl FontNamePattern {
    pub fn is_match(&self, name: &str) -> bool;
    pub fn as_str(&self) -> Option<&str>;
}

pub struct FontFamily {
    pub name: FontNamePattern,       // lowercased se literal
    pub variants: Vec<EcoString>,    // variant names (scope-out usage)
    pub covers: Option<Covers>,      // inabitado; sempre None
}

impl FontFamily {
    /// Família literal, variants vazio, covers None; normaliza name para lowercase.
    pub fn new(name: EcoString) -> Self;
    /// Família literal explícita com variants.
    pub fn new_literal(name: EcoString, variants: Vec<EcoString>) -> Self;
    /// Família regex com variants.
    pub fn new_regex(regex: Regex, variants: Vec<EcoString>) -> Self;
}

pub struct FontList(Vec<FontFamily>);  // non-empty por construção

impl FontList {
    pub fn new(families: Vec<FontFamily>) -> Option<Self>;  // None se vazio
    pub fn single(name: EcoString) -> Self;                 // 1 literal, variants vazio
    pub fn as_slice(&self) -> &[FontFamily];
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;  // sempre false (invariante)
}
```

## Semântica

- **Name lowercased para literais**: `FontFamily::new("Arial")` →
  `name = FontNamePattern::Literal("arial")`. Paridade vanilla.
- **Regex**: `FontNamePattern::Regex` usa `Regex::is_match(name)`;
  pattern é preservado via `Arc<regex::Regex>` (clone O(1)).
- **Non-empty invariante**: `FontList::new(vec![])` devolve `None`.
- **Ordem preservada**: prioridade de fallback chain; primeiro match wins.
- **`Covers` inabitado**: `enum Covers {}` sem variantes.
  `Option<Covers>` só pode ser `None` por construção.

## Integração

- `FontBook::select_pattern(&self, pattern: &FontNamePattern, variant)`
  resolve literais exactos (case-insensitive) ou regex contra o catálogo.
- Layout/text.rs decodifica `"text.font"` da chain custom em `FontList`.
- `Style::Font(FontList)` continua `Clone + PartialEq`; `FontNamePattern`
  é `Clone + PartialEq + Eq + Hash`.

## Validação no eval (arm `"font"`)

Ver prompt `rules/style/font-dict.md`.

## Critérios de Verificação

```
Dado FontFamily::new("Arial")
Quando name.as_str()
Então Some("arial")               (lowercase normalizado)

Dado FontNamePattern::Regex(Regex::new("Name.*"))
Quando is_match("Name Bold")
Então true

Dado FontNamePattern::Literal("Name")
Quando is_match("Name Bold")
Então false                       (exacto)

Dado FontList::single("X")
Quando len()
Então 1

Dado FontList::new(vec![])
Quando called
Então None                        (non-empty obrigatório)

Dado Covers
Quando try to match on variant
Então compile error               (enum inabitado)
```

## Não incluído (deferido)

- `covers` concreto (keyword `LatinInCjk` ou regex coverage).
- `FontVariant` selection variant-aware (DEBT-53 / ADR-0054bis).
- Optimização O(1) de lookup para literais em dict com regex.
- Iteração `IntoIterator` — pode adicionar on-demand.
