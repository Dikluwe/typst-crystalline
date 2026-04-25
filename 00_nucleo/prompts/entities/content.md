# Prompt L0 — Content
Hash do Código: 0f5177f7

## Módulo
`01_core/src/entities/content.rs`

## Propósito
`Content` representa a estrutura declarativa do documento Typst produzida
por `eval()`. É puramente declarativa — não desenha, não mede, não renderiza.
Qualquer operação que precise de métricas de fonte ou I/O pertence a L3.

## Divergência do original (Opção D)
O `Content` original (`typst-library/foundations/content/`) usa:
- `pub struct Content(raw::RawContent)` com vtable `unsafe trait NativeElement`
- Proc macros `#[elem]` que geram implementações de `NativeElement`
- Arc manual (fat pointer com ref counting customizado, não `std::sync::Arc`)
- Styles como camada separada via `StyledElem` wrapper

Replicar esta metaprogramação em L1 traria toda a complexidade de
`typst_macros` sem benefício arquitectural. O cristalino diverge
intencionalmente: usa um enum linear com variantes declarativas.

Decisão registada em ADR-0026 (a criar).

## Representação
```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Content {
    Empty,
    Text(EcoString),         // TextElem mínimo
    Space,                   // SpaceElem / espaço entre palavras
    Sequence(Vec<Content>),  // sequência de elementos
    // Variantes futuras — NÃO implementar sem ADR:
    // Styled(Box<Content>, Styles),             // requer Styles real
    // Heading { level: u8, body: Box<Content> },
    // Strong(Box<Content>),
    // Emph(Box<Content>),
    // Raw { text: EcoString, lang: Option<EcoString> },
    // Elem(Arc<dyn NativeElement>),              // vtable — Passo 20+
}
```

## Interface pública obrigatória
```rust
impl Content {
    pub fn text(s: impl Into<EcoString>) -> Self;
    pub fn empty() -> Self;
    pub fn sequence(parts: Vec<Content>) -> Self;
    pub fn is_empty(&self) -> bool;
    pub fn plain_text(&self) -> String;
}
```

### `sequence()` — normalização
- 0 partes → `Empty`
- 1 parte → desembrulha (evita `Sequence([x])`)
- n > 1 partes → `Sequence(parts)`

### `plain_text()` — para verificação em testes
- `Empty` → `""`
- `Text(s)` → `s.to_string()`
- `Space` → `" "`
- `Sequence(v)` → concatenação recursiva

## Método `map_content` (Passo 69 — DEBT-19)

Percorre a árvore AST de baixo para cima (bottom-up), aplicando uma closure
a cada nó após processar os seus filhos.

```rust
pub fn map_content<F>(&self, transform: &mut F) -> SourceResult<Self>
where
    F: FnMut(&Content) -> SourceResult<Option<Content>>;
```

Semântica:
- `transform` retorna `Some(new_content)` → substituir; o novo nó NÃO é reavaliado.
- `transform` retorna `None` → manter o nó processado (com filhos já transformados).
- O `match` lista explicitamente containers (recursão) e terminais (clone). Sem `_ =>`.

Containers (recursão bottom-up): `Sequence`, `Strong`, `Emph`, `Heading`,
`ListItem`, `EnumItem`, `Link`, `Labelled`, `Figure`, `Equation`, `MathSequence`,
`MathFrac`, `MathAttach`, `MathRoot`, `MathDelimited`, `MathMatrix`, `MathCases`.

Terminais (clone directo): `Text`, `Space`, `Empty`, `Linebreak`, `Outline`,
`Raw`, `Ref`, `SetHeadingNumbering`, `CounterUpdate`, `CounterDisplay`,
`MathAlignPoint`, `MathIdent`, `MathText`.

## Variante `Content::Image` (Passo 71 — DEBT-24)

```rust
Image {
    path:   String,
    data:   std::sync::Arc<Vec<u8>>,
    width:  Option<Box<Value>>,   // Box quebra ciclo Content→Value→Content
    height: Option<Box<Value>>,
},
```

Terminal — sem filhos Content. `Arc<Vec<u8>>` partilhado: clones do AST não copiam bytes.
- `plain_text` → `""`
- `is_empty` → `false`
- `map_content` → terminal: `clone()`
- `map_text` → terminal: `clone()`
- Layouter: placeholder 100×100 pt (DEBT-24b).

## Método `get_field` (Passo 68)

Acesso a campos de elementos estruturados — usado pelas show rules.
Suporta `.body` e `.level` em `Heading`, `.body` em `Figure`.

## Critérios de verificação
- `Content::text("hello").plain_text() == "hello"`
- `Content::empty().is_empty() == true`
- `Content::sequence(vec![]).is_empty() == true`
- `Content::sequence(vec![Content::text("a")]) == Content::text("a")` (desembrulha)
- `Content::sequence(vec![Content::text("a"), Content::Space, Content::text("b")]).plain_text() == "a b"`
- `Content::Empty` e `Content::Space` — clone e PartialEq funcionam

## Variantes estruturais — Passo 154B (ADR-0060 Fase 1)

Materializadas em P154B como primeira sub-fase da Fase 1 do roadmap
ADR-0060. **Sem ADR nova** — apenas adições ao enum.

### `Content::Divider`

Singleton estrutural sem dados. Representa um separador horizontal.

- `plain_text()` → `""` (sem texto; representação visual é distinta).
- `is_empty()` → `false` (singleton estrutural conta como conteúdo).
- `map_content` / `map_text` → terminal (clone directo).
- Layouter: emite `FrameItem::Shape::Line` à largura do conteúdo,
  espessura 0.5pt, traço preto.

### `Content::Terms { items: Vec<Content> }`

Lista de pares termo-descrição. Tipicamente `items` é uma sequência
de `Content::TermItem`. A ordem é preservada.

- `plain_text()` → `items.iter().map(plain_text).join("\n")`.
- `is_empty()` → `items.is_empty()`.
- `map_content` / `map_text` → container; recurse em cada item.
- Layouter: itera items, layout sequencial.

### `Content::TermItem { term: Box<Content>, description: Box<Content> }`

Par individual term/description. Surge tipicamente dentro de `Terms`,
mas pode também aparecer standalone (e.g. show rules futuras).

- `plain_text()` → `format!("{}: {}", term.plain_text(), description.plain_text())`.
- `is_empty()` → `term.is_empty() && description.is_empty()`.
- `map_content` / `map_text` → container; recurse em term e description.
- Layouter: term em negrito + ": " + description, com indent 1.5em.

### Stdlib funcs (Passo 154B)

- `terms(named: descrição, ...)` em Typst-lang produz `Content::Terms`.
  Aceita só argumentos nomeados; descrição pode ser content ou string.
  Forma: `#terms(apple: [fruit], banana: [yellow])`.
- `divider()` produz `Content::Divider`. Sem argumentos.

### Limitações conscientes (P154B)

- Sem syntax markup nova (`/ term: desc` ou `---`) — trabalho de parser
  diferido a passo separado.
- Sem atributos vanilla `tight`/`separator`/`indent`/`hanging-indent`
  para `terms` — extensíveis sem breaking change (passar a
  `Terms { items, tight, ... }`).
- Sem show rules `#show terms: ...` neste passo.

## Variant `Content::Quote` — Passo 155 (ADR-0060 Fase 1, sub-passo 2)

Materializado em P155 como segunda sub-fase da Fase 1; **fecha
ADR-0060** (`PROPOSTO → IMPLEMENTADO`).

```rust
Content::Quote {
    body:        Box<Content>,
    attribution: Option<Box<Content>>,
    block:       bool,
    quotes:      bool,
}
```

**Atributos**:
- `body` — conteúdo citado.
- `attribution` — autor/fonte opcional.
- `block: true` → parágrafo dedicado, indent + spacing; `block: false`
  → inline no parágrafo circundante.
- `quotes: true` → aspas locale-apropriadas via
  `crate::rules::lang::quotes::localize_quotes(lang)` em torno do body.

**Comportamento `plain_text`**:
- Sem smart-quotes: usa `"` ASCII fallback (texto plano não interage com lang).
- Com attribution: `"body" — attribution`.
- Sem attribution: `"body"`.
- Se `quotes: false`: aspas omitidas.

**Renderização (layouter)**:
- Smart-quotes via `text.lang` activo (per ADR-0057).
- `block: true`: indent 1.5em à esquerda; attribution em linha separada
  prefixada por "— ".
- `block: false`: inline; attribution prefixada por " — ".

**Construtores**:
- Stdlib: `#quote(body, attribution: ?, block: false, quotes: true)`.
- Markup: `"..."` em `Mode::Markup` produz aspas localizadas
  open/close por alternância (NÃO produz `Content::Quote`; produz
  `Content::Text(glyph)`). Cristalino usa o lexer vanilla
  (1 char = 1 SmartQuote token). `Content::Quote` é exclusivamente
  para construções estruturais via `#quote(...)`.

**Tabela de smart-quotes** (per `rules/lang/quotes.rs`):
| Lang | Open | Close |
|------|------|-------|
| `pt` | `«` | `»` |
| `en` | `"` (U+201C) | `"` (U+201D) |
| `de` | `„` | `"` (U+201C) |
| `fr` | `« ` (NBSP) | ` »` (NBSP) |
| `es` | `«` | `»` |
| `it` | `«` | `»` |
| (default) | `"` ASCII | `"` ASCII |

### Limitações conscientes (P155)

- Sem show rules `#show quote: ...` neste passo.
- Sem aspas secundárias (`'...'`) em markup — produz `'` ASCII.
- Sem smart-apostrophes (`don't` → `don't`).
- Aspas aninhadas em markup não suportadas (alternância simples
  open/close).
- Markup `"..."` produz `Content::Text` com glyph localizado; **não**
  produz `Content::Quote` (esse fica reservado para `#quote()`
  estrutural). Decisão pragmática: cristalino's lexer já é
  per-character, e refactor para parear `"..."` excederia escopo P155.
