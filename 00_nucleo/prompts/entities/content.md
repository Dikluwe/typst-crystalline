# Prompt L0 — Content
Hash do Código: 3bf4e63d

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

## Variants `Content::Pad` + `Content::Hide` — Passo 156C (ADR-0061 Fase 1, sub-passo 1)

Primeira aplicação concreta da ADR-0061 (Layout Fase X
roadmap). Materializam dois containers simples user-facing:
`pad()` (margens internas) e `hide()` (placeholder
layout-aware). Análogos a vanilla `PadElem`/`HideElem` em
`lab/typst-original/crates/typst-library/src/layout/{pad,hide}.rs`.

### `Content::Pad { body, padding }`

```rust
Pad {
    body:    Box<Content>,
    padding: Sides<Length>,
}
```

**Atributos** (declarados em stdlib `pad()`, resolvidos antes
de criar o variant):
- `left` / `right` / `top` / `bottom` — específico por lado.
- `x` (cobre `left` + `right`) — atalho horizontal.
- `y` (cobre `top` + `bottom`) — atalho vertical.
- `rest` — fallback para qualquer lado não declarado.

**Precedência** (resolvida em `native_pad`): específico > eixo
> rest. Lados não declarados em qualquer nível ficam a
`Length::ZERO`.

**Comportamento `is_empty` / `plain_text` / `map_*`**:
- `is_empty` — proxy para `body.is_empty()`.
- `plain_text` — recurse no body (transparente).
- `map_content` / `map_text` — recurse no body; padding
  preservado como `Copy`.

**Renderização (layouter)**:
- `top` adicionado ao `cursor_y` antes do body.
- `left` adicionado ao `cursor_x` (e a `line_start_x` para
  que `flush_line` reinicie indentado).
- Body é layouted com cursor ajustado.
- Após body, `flush_line` força fim de linha pendente; `bottom`
  é adicionado a `cursor_y`; `cursor_x` e `line_start_x`
  restaurados.
- **`right` é scope-out neste passo** (perfil ADR-0054 graded):
  o Layouter actual não tem mecânica de "largura útil" por arm
  — width-aware wrap vive em `flush_line`/`layout_word` que
  consultam `page_config.width`. Aceitar como aproximação;
  refino quando refactor de Layouter para multi-region acontecer
  (DEBT-56 + Fase 3 Layout).

**Validação em `native_pad`**:
- Padding negativo é rejeitado com erro hard (perfil ADR-0054
  graded). Vanilla aceita-o (margens "negativas" produzem overlap);
  cristalino diverge intencionalmente até que layout overflow
  semantic esteja clara.
- Named args desconhecidos rejeitados (paridade com `assert`,
  `align`, `place` etc.).
- Body posicional obrigatório (Content ou Str).

### `Content::Hide { body }`

```rust
Hide {
    body: Box<Content>,
}
```

**Atributos**: apenas `body`. Sem named args.

**Comportamento `is_empty` / `plain_text` / `map_*`**:
- `is_empty` — proxy para `body.is_empty()`.
- `plain_text` — `String::new()` (não rende; texto plano vazio).
- `map_content` / `map_text` — recurse no body (transformações
  internas aplicam-se mesmo que o body não seja renderizado;
  permite que pipelines de pré-processamento funcionem).

**Renderização (layouter)**:
- Drena `current_items` e `current_line` para buffers
  temporários.
- Layouter executa o body normalmente (cursor avança).
- Items gerados pelo body são descartados (substituídos pelos
  buffers salvos).
- Resultado: zero `FrameItem`s emitidos, mas `cursor_x`/
  `cursor_y` preservam o avanço.

**Comportamento em introspect** (`materialize_time` + `walk`):
- Ambos descem no body. Hide preserva semantic de "presence":
  labels, contadores e refs dentro de `hide(...)` continuam a
  resolver. Apenas a renderização é suprimida.

### Construtores

- Stdlib: `#pad(body, left: ?, right: ?, top: ?, bottom: ?,
  x: ?, y: ?, rest: ?)` e `#hide(body)`.
- Construtores Rust: `Content::pad(body, padding)` e
  `Content::hide(body)`.

### Limitações conscientes (P156C)

- Sem show rules `#show pad: ...` ou `#show hide: ...` neste
  passo. Adiados a passo agregado futuro (análogo a P154B/P155
  para terms/divider/quote).
- `right` padding **scope-out** em layout — ver acima. Refino
  com refactor multi-region.
- Padding negativo **scope-out** (rejeitado com erro). Refino
  quando layout overflow semantic clara existir.
- `Content::Pad` e `Content::Hide` aninhados são suportados
  (cobertura recursiva em todos os arms); padding aninhado é
  cumulativo, hide aninhado é idempotente.

### Decisão arquitectural confirmada (per ADR-0061 Decisão 4)

Variants novos (não `Content::Styled`). Rationale: ambos têm
semantic estrutural (composição + cursor advance) que excede
styling visual. Coerente com vanilla `PadElem`/`HideElem`
serem `#[elem]` proper. Coerente com modelo ADR-0060 Fase 1
para terms/divider/quote.

## Variants `Content::HSpace` + `Content::VSpace` — Passo 156D (ADR-0061 Fase 1, sub-passo 2)

Segunda aplicação consecutiva de ADR-0061. Materializam
spacing primitives horizontal e vertical, análogos a vanilla
`HElem`/`VElem` em
`lab/typst-original/crates/typst-library/src/layout/spacing.rs`.

### `Content::HSpace { amount, weak }`

```rust
HSpace {
    amount: Length,
    weak:   bool,
}
```

**Atributos** (declarados em stdlib `h(amount, weak: false)`):
- `amount` — Length posicional obrigatório.
- `weak: bool` — armazenado mas comportamento de collapse
  adiado (perfil ADR-0054 graded). Refino futuro se priorizado.

**Comportamento `is_empty` / `plain_text` / `map_*`**:
- `is_empty` — `amount.is_zero()` (consistente com Sequence
  vazia).
- `plain_text` — `String::new()` (não rende texto).
- `map_content` / `map_text` — terminal (clone directo); leaf
  sem body.

**Renderização (layouter)**:
- Resolve `amount` em pt via `Length::resolve_pt(font_size_pt)`.
- Avança `self.cursor_x` por esse valor.
- `weak` ignorado neste passo.

**Validação em `native_h`**:
- Aceita `Length`, `Float` (interpretado em pt), `Int` (idem).
- `amount` negativo rejeitado (perfil ADR-0054 graded; vanilla
  aceita-o).
- Named arg desconhecido rejeitado.
- `weak` deve ser `Bool` (tipo errado → erro hard).

### `Content::VSpace { amount, weak }`

```rust
VSpace {
    amount: Length,
    weak:   bool,
}
```

**Atributos**: idênticos a `HSpace`.

**Comportamento `is_empty` / `plain_text` / `map_*`**: idênticos
a `HSpace`.

**Renderização (layouter)**:
- Resolve `amount` em pt.
- Se `cursor_x > line_start_x`, força `flush_line` (termina
  linha em curso para evitar texto meio-render).
- Avança `self.cursor_y` pelo valor resolvido.

**Validação em `native_v`**: idêntica a `native_h` (lógica
partilhada via helper `build_spacing` em `stdlib/layout.rs`).

### Construtores

- Stdlib: `#h(amount, weak: false)` e `#v(amount, weak: false)`.
- Construtores Rust: `Content::h_space(amount, weak)` e
  `Content::v_space(amount, weak)` (naming `_space` evita
  conflito com identificadores curtos `h`/`v` em scope Rust).

### Limitações conscientes (P156D)

- `amount` aceita apenas `Length` neste passo. Vanilla aceita
  `Fraction` (ex: `h(1fr)`) — refino futuro per ADR-0061 §6.3.
- `weak` armazenado mas semantic de collapse não implementada.
  Vanilla colapsa weak adjacentes; cristalino mantém ambos
  (over-spacing aceitável per ADR-0054 graded). Se priorizado,
  abrir DEBT.
- `amount` negativo rejeitado com erro. Vanilla aceita-o
  (gera overlap). Refino quando layout overflow semantic
  clara existir.
- `h` no fim de linha não força wrap; cursor.x apenas avança
  (pode exceder largura da página). Refino com refactor
  multi-region (DEBT-56 + Fase 3).
- `v` no início de página/coluna não colapsa contra margem
  (vanilla colapsa). Avanço simples de cursor.y.
- Sem show rules `#show h: ...` ou `#show v: ...` neste passo
  (consistente com adiamento P154B/P155/P156C).

### Decisão arquitectural confirmada (per ADR-0061 Decisão 4)

Variants novos (não `Content::Styled`). Rationale: spacing
primitives são structurais (afectam cursor, não rendem texto),
não estilo visual. Coerente com vanilla `HElem`/`VElem` serem
`#[elem]` proper. Coerente com modelo dos sub-passos
anteriores (terms, divider, quote, pad, hide).

## Variant `Content::Pagebreak` — Passo 156E (ADR-0061 Fase 1, sub-passo 3)

Terceira aplicação consecutiva de ADR-0061. Materializa
quebra de página manual, análoga a vanilla `PagebreakElem`
em `lab/typst-original/crates/typst-library/src/layout/page.rs`.

### `Content::Pagebreak { weak, to }`

```rust
Pagebreak {
    weak: bool,
    to:   Option<Parity>,
}
```

**Atributos** (declarados em stdlib `pagebreak(weak: false,
to: ?)`):
- `weak: bool` — armazenado mas comportamento de collapse
  adiado (consistente com P156D HSpace/VSpace).
- `to: Option<Parity>` — `None` == Auto (sem ajuste);
  `Some(Even)`/`Some(Odd)` força próxima página à paridade.

**Tipo `Parity`** novo em `01_core/src/entities/parity.rs`
(Even/Odd com método `matches(page_number)`); ver
`prompts/entities/parity.md`.

**Comportamento `is_empty` / `plain_text` / `map_*`**:
- `is_empty` — sempre **`false`** (event observável mesmo
  sem body; cf. Divider em P154B).
- `plain_text` — `String::new()` (event sem texto).
- `map_content` / `map_text` — terminal (clone directo);
  Pagebreak é leaf sem body.

**Renderização (layouter)**:
- Reusa `Layouter::new_page` (definido em `cursor.rs:128`)
  que commits `current_items` numa nova `Page`, push para
  `pages`, e reseta cursor.
- Sequência: `flush_line` (se houver linha em curso) →
  `new_page()` → se `to: Some(parity)`, verifica
  `pages.len() + 1` (próxima página) e insere segunda
  `new_page()` se paridade não bate.
- Página inserida usa `page_config` actual (mesmas dimensões;
  sem header/footer porque Page actual não os tem).
- `weak` ignorado neste passo.

**Validação em `native_pagebreak`**:
- Sem argumentos posicionais (rejeitado com erro hard).
- Named args válidos: `weak` (Bool), `to` (Str
  `"even"`/`"odd"`). Outros named args rejeitados.
- `weak` deve ser Bool; tipo errado → erro hard.
- `to` deve ser Str `"even"` ou `"odd"`; outro valor → erro
  hard (helper `extract_parity`). `to: None` (omitido)
  produz `Option::None`.

### Construtores

- Stdlib: `#pagebreak(weak: false, to: ?)`.
- Construtor Rust: `Content::pagebreak(weak, to)`.

### Limitações conscientes (P156E)

- `weak` collapse semantic não implementado (consistente
  P156D). Vanilla colapsa weak adjacentes; cristalino
  mantém ambos.
- Página vazia inserida para ajustar paridade não tem
  cabeçalho/rodapé (porque `Page` cristalino não os tem).
  Refino futuro com Page rico (Fase 3 ADR-0061).
- `to` aceita só string em stdlib (vanilla aceita
  `Symbol::even` sem aspas). Refino se priorizado.
- Pagebreak no início absoluto do documento cria página 1
  vazia + conteúdo na página 2; aceitável (case patológico
  raro).
- Sem show rules `#show pagebreak: ...` neste passo.

### Decisão arquitectural confirmada (per ADR-0061 Decisão 4)

Variant novo (não `Content::Styled`, não `Style::PageBreak`).
Rationale: pagebreak é "event" estrutural com semantic única
(força flush + verifica paridade) que excede styling. Coerente
com vanilla `PagebreakElem` ser `#[elem]` proper. Coerente
com modelo dos sub-passos anteriores.

### Tipo `Parity` (infraestrutura paralela)

`Parity { Even, Odd }` foi criado neste passo como
infraestrutura genérica reusável, análoga ao `Sides<T>`
criado em P156C. Vive em `01_core/src/entities/parity.rs`.
Reuso futuro previsível em refino Page rico (paridade per
header/footer).

Vanilla usa `Smart<Parity>` (Auto/Custom); cristalino
simplifica para `Option<Parity>` (`None` == Auto). Sem
perda funcional; ganho em clareza idiomática Rust.

## Skew via `TransformMatrix::skew` — Passo 156F (ADR-0061 Fase 1, sub-passo 4)

Quarta aplicação consecutiva de ADR-0061. Materializa
`skew(body, ax: ?, ay: ?)` análogo a vanilla `SkewElem`.

### Divergência face à spec do P156F

A spec do P156F propôs introduzir `enum TransformKind
{ Move, Rotate, Scale, Skew }` para "unificar" os 4
elementos vanilla num só variant `Content::Transform`.
**Inventário em 156F.1 revelou que essa unificação já
existia desde P78** — `Content::Transform { body, matrix:
TransformMatrix }` reusa a matriz cm (PDF) para todos os
4 tipos. `TransformKind` enum seria redundante.

**Decisão deste passo**: skew adicionado como método
estático novo `TransformMatrix::skew(ax_rad, ay_rad)` em
`entities/layout_types.rs`, análogo a `translate`,
`rotate`, `scale` já existentes. Zero refactor de
variant; zero mudança em consumers. **Risco de regressão
zero** (puramente aditivo).

### `TransformMatrix::skew(ax_rad, ay_rad)`

Forma da matriz cm:
```
| 1        tan(ax)   0 |
| tan(ay)  1         0 |
| 0        0         1 |
```

Aplicada a `(x, y)`:
- `x' = x + tan(ax) * y`
- `y' = tan(ay) * x + y`

### Stdlib `#skew(body, ax: ?, ay: ?)`

Implementado em `01_core/src/rules/stdlib/transforms.rs`
ao lado de `native_move`/`native_rotate`/`native_scale`
(coesão por domínio per ADR-0037). Atributos:
- `ax: Angle` — distorção horizontal (default 0).
- `ay: Angle` — distorção vertical (default 0).
- `body` posicional obrigatório.
- Aceita também `Float` em radianos (consistente com
  `native_rotate`).

**Validação**:
- Ângulos com magnitude ≥ `π/2 - 1e-3` rad (~89.94°)
  rejeitados (tan diverge); erro hard.
- Named args desconhecidos rejeitados (consistente
  pad/h/v/pagebreak).
- `origin` (ponto de pivot) **scope-out** — análogo a
  rotate/scale actuais que também não têm origin
  (refino futuro per ADR-0061 §6.3).

### Limitações conscientes (P156F)

- `origin` não suportado (alinhado com move/rotate/scale).
- Ângulos extremos rejeitados em vez de saturar (decisão
  de erro explícito vs comportamento indefinido).
- `Smart<Angle>` da vanilla simplificado para `Option`
  implícito (default 0 quando ausente).

### Decisão arquitectural confirmada

Sem TransformKind enum (per inventário 156F.1).
Arquitectura matriz cm já era a unificação correcta. P156F
**adiciona método ao tipo existente** em vez de refactorar
struct — padrão "menor mudança suficiente".
