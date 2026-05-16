# Prompt L0 — Content
Hash do Código: d3f8a8c1

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

## Variant `Content::Block` — Passo 156G (ADR-0061 Fase 2, sub-passo 1)

Quinta aplicação consecutiva de ADR-0061; **primeira de
Fase 2** (containers ricos). Materializa
`block(body, width: ?, height: ?, inset: ?, breakable: true)`
análogo a vanilla `BlockElem`.

### Decisão arquitectural escolhida (Opção A modificada)

Per inventário 156G.1 + análise comparativa 156G.2:
**variant rico `Content::Block { body, width, height, inset,
breakable }`** em vez de Style cascade.

**Rationale**:
- `Style` enum cobre só **propriedades de texto** (Bold,
  Italic, Size, Fill, HeadingLevel) — vocabulário não-encaixa
  com width/height/inset/breakable que são **propriedades de
  container**.
- Coerente com `Content::Pad` (P156C) que também tem fields
  explícitos para padding (também propriedade de container).
- Reusa pattern emergente: containers com fields explícitos
  para atributos não-Style.

### `Content::Block { body, width, height, inset, breakable }`

```rust
Block {
    body:      Box<Content>,
    width:     Option<Length>,
    height:    Option<Length>,
    inset:     Sides<Length>,
    breakable: bool,
}
```

**Atributos** (subset Fase 1 per ADR-0054 graded; declarados
em stdlib `block(body, ...)`):
- `body` posicional opcional (Content ou Str; ausente →
  Empty).
- `width: Length` — largura explícita; default `None` (auto).
- `height: Length` — altura explícita; default `None` (auto).
- `inset: Length` — margem interna uniforme nos 4 lados
  (refino futuro: Sides completo via dict); default zero.
- `breakable: bool` — `true` permite quebra entre páginas;
  `false` é "atómico"; default `true`. **Semantic real
  adiada** — armazenado mas layouter não impede quebra ainda.

**Atributos scope-out** (refino futuro per ADR-0054 graded):
`outset`, `fill`, `stroke`, `radius`, `clip`, `spacing`,
`above`/`below`, `sticky`. **Rejeitados em `native_block`**
com erro hard até refino futuro.

**Comportamento `is_empty` / `plain_text` / `map_*`**:
- `is_empty` — proxy para `body.is_empty()` (atributos não
  fazem container deixar de ser vazio).
- `plain_text` — recurse no body (transparente).
- `map_content` / `map_text` — recurse no body; atributos
  preservados como Copy.

**Renderização (layouter)**:
- `flush_line` se houver conteúdo pendente (block ocupa
  nova "linha lógica").
- Aplica `inset.top` (avança cursor.y).
- Aplica `inset.left` (offset de `line_start_x`/cursor.x).
- Layout do body com cursor ajustado.
- `flush_line` no fim.
- Aplica `inset.bottom` (avança cursor.y).
- Se `height: Some(h)`, garante avanço mínimo de h vertical
  (caso body + inset_top + inset_bottom seja menor).
- Restaura `line_start_x`/cursor.x.
- `inset.right` é scope-out (Layouter actual sem largura
  útil por arm; refino com refactor multi-region per
  DEBT-56).
- `width` armazenado mas não consumido em layout actual
  (largura limitada por flush_line/word_wrap globais).
  Per ADR-0054 graded; refino futuro.

**Validação em `native_block`**:
- Width/height/inset negativos rejeitados (consistente com
  pad em P156C).
- Named arg desconhecido rejeitado com mensagem explicativa
  (incluindo lista de scope-outs).
- `breakable` deve ser Bool.
- Inset aceita `Length` uniforme apenas (refino futuro para
  dict).

### Construtores

- Stdlib: `#block(body, width: ?, height: ?, inset: ?,
  breakable: ?)`.
- Construtor Rust: `Content::block(body, width, height,
  inset, breakable)`.

### Limitações conscientes (P156G)

- **9 atributos vanilla scope-out** (outset, fill, stroke,
  radius, clip, spacing, above, below, sticky) — refino
  futuro per ADR-0054 graded.
- `inset` aceita Length uniforme apenas. Vanilla aceita
  dict ou número. Refino futuro.
- `width` armazenado mas não impõe limite real (Layouter
  actual sem mecânica de largura útil por arm).
- `breakable: false` armazenado mas semantic real defere
  (refactor multi-region exigido).
- `inset.right` scope-out em layout (mesma razão que
  `Pad.right` em P156C).
- Sem show rules `#show block: ...` neste passo.
- Block aninhado: suportado estruturalmente; insets
  cumulativos via cursor advance.

### Decisão arquitectural confirmada (per ADR-0061 Decisão 4)

Variant rico (não Styled). Coerente com:
- vanilla `BlockElem` ser `#[elem]` proper.
- `Content::Pad` (P156C) que usa fields explícitos.
- Princípio "container com atributos não-style usa variant".

**Padrão emergente Fase 2**: containers ricos preferem
variants explícitos quando atributos não são propriedades
de texto. Box (P156H) e Stack (P156I) provavelmente seguem
mesmo modelo.

## Variant `Content::Boxed` (box inline) — Passo 156H (ADR-0061 Fase 2, sub-passo 2)

Sexta aplicação consecutiva de ADR-0061; **segunda Fase 2**.
Materializa `box(body, width: ?, height: ?, inset: ?,
baseline: ?)` análogo a vanilla `BoxElem`.

### Decisão arquitectural reusada (Opção A modificada de P156G)

Padrão **variant rico** estabelecido em P156G aplicado
directamente — sem nova decisão arquitectural. P156G §15.5
recomendou explicitamente esta reaplicação.

### `Content::Boxed { body, width, height, inset, baseline }`

```rust
Boxed {
    body:     Box<Content>,
    width:    Option<Length>,
    height:   Option<Length>,
    inset:    Sides<Length>,
    baseline: Length,
}
```

**Naming**: variant Rust é `Boxed` (não `Box`) para evitar
ambiguidade com `std::boxed::Box`; stdlib expõe `#box(...)`
(paridade vanilla); construtor Rust: `Content::boxed(...)`.

**Atributos** (subset Fase 2 per ADR-0054 graded):
- `body` posicional opcional (Content ou Str; ausente → Empty).
- `width: Length` — explícita; default `None` (content-based).
- `height: Length` — explícita; default `None` (auto).
- `inset: Length` — uniforme nos 4 lados; default zero.
- `baseline: Length` — ajuste vertical; default zero;
  **negativo aceito** (move para cima).

**Atributos scope-out** (refino futuro per ADR-0054 graded):
`outset`, `fill`, `stroke`, `radius`, `clip`, `stroke-overhang`.
**Rejeitados em `native_box`** com erro hard.

### Distinção material face a `Block` (P156G)

| Aspecto | Block | Boxed (Box) |
|---------|-------|-------------|
| Posicionamento | structural (força flush_line) | **inline** (sem flush) |
| Largura default | full page width | content-based |
| Atributo único | `breakable: bool` | `baseline: Length` |
| Layouter | flush + inset_top + offset_left + body + flush + inset_bottom + height_min | append inline + inset_left + body + inset_right (top/bottom/baseline scope-out em layout actual) |

**Comportamento `is_empty` / `plain_text` / `map_*`**:
Análogo a `Block` (proxy body; recurse).

**Renderização (layouter)**:
- **NÃO força flush_line** (box é inline).
- Aplica `inset.left` como avanço de cursor.x.
- Layout body in-place na linha actual.
- Aplica `inset.right` como avanço de cursor.x final.
- `width`/`height`/`baseline` armazenados mas semantic real
  adiada per ADR-0054 graded:
  - `width`: limitar largura útil em contexto inline exigiria
    refactor multi-region (DEBT-56).
  - `height` em contexto inline alteraria line_height —
    refino futuro.
  - `baseline` exige offset vertical mid-linha — não
    suportado por cursor.rs actual.
- `inset.top`/`inset.bottom` em contexto inline são
  complexos; armazenados mas não aplicados (refino futuro).

**Validação em `native_box`**:
- Width/height/inset negativos rejeitados (consistente Block).
- **Baseline negativo aceito** (semantic legítima — move
  box para cima).
- Named arg desconhecido rejeitado.

### Construtores

- Stdlib: `#box(body, width: ?, height: ?, inset: ?,
  baseline: ?)`.
- Construtor Rust: `Content::boxed(body, width, height,
  inset, baseline)`.

### Limitações conscientes (P156H)

- 6 atributos vanilla scope-out (outset, fill, stroke,
  radius, clip, stroke-overhang). Refino futuro.
- `inset` aceita Length uniforme apenas (refino futuro
  para dict).
- `width`/`height` armazenados mas não impõem limite real
  (refino multi-region per DEBT-56).
- `baseline` armazenado mas semantic real adiada (cursor.rs
  actual sem mecânica de offset mid-linha).
- `inset.top`/`inset.bottom` armazenados mas não aplicados
  em layout inline (alterariam line_height).
- Sem show rules `#show box: ...` neste passo.

### Padrão emergente Fase 2 (reaplicação confirma)

P156G estabeleceu padrão "variant rico para containers"; P156H
reaplica directamente sem nova decisão arquitectural. **Padrão
consolidado**: P156I (stack) provavelmente segue mesmo modelo.

## Variant `Content::Stack` — Passo 156I (ADR-0061 Fase 2, sub-passo 3; **último Fase 2**)

**Sétima aplicação consecutiva** de ADR-0061; **último
sub-passo Fase 2**; **atinge target 72% Layout** declarado
em ADR-0061 §6.2.

### Decisão arquitectural reusada (Opção A modificada de P156G/H)

Padrão variant rico estabelecido em P156G+H aplicado
directamente. Adaptação para `Arc<[Content]>` (clone O(1)
per ADR-0026 revisão, consistente com `Sequence`/`MathSequence`).

### `Content::Stack { children, dir, spacing }`

```rust
Stack {
    children: Arc<[Content]>,
    dir:      Dir,
    spacing:  Option<Length>,
}
```

**Atributos**:
- `children: Arc<[Content]>` — variádicos posicionais (Content
  ou Str na stdlib).
- `dir: Dir` — direcção de empilhamento (LTR/RTL/TTB/BTT;
  default `TTB`). Tipo `Dir` novo em `entities/dir.rs`.
- `spacing: Option<Length>` — espaço entre children;
  `None` == zero (consistente com padrão Smart→Option
  N=5 aplicações).

**Distinção material face a Block/Boxed**:

| Aspecto | Block | Boxed | Stack |
|---------|-------|-------|-------|
| Body | único | único | **Vec (Arc<[Content]>)** |
| Tipo body | `Box<Content>` | `Box<Content>` | `Arc<[Content]>` |
| Posicionamento | structural | inline | **structural** |
| Atributos próprios | breakable | baseline | **dir + spacing** |

### Comportamento `is_empty` / `plain_text` / `map_*`

- `is_empty` — `children.iter().all(|c| c.is_empty())`
  (consistente com `Sequence`).
- `plain_text` — concatena plain_text de todos os children.
- `PartialEq::eq` — comparação 3-fields (Arc deep eq).
- `map_content` / `map_text` — mapear cada child; preservar
  dir/spacing.
- `materialize_time` (introspect) — recurse em cada child.
- `walk` (introspect) — walk em cada child em ordem.

### Renderização (layouter)

- **Structural**: força `flush_line` antes (se necessário).
- **TTB/BTT**: itera children; cada um em "linha" própria
  (flush_line após cada); spacing entre via `cursor_y +=
  Pt(spacing)` antes de cada child (excepto o primeiro).
- **LTR/RTL**: itera children inline; spacing via `cursor_x
  += Pt(spacing)` entre cada.
- **BTT/RTL**: implementadas como reverse iteration
  (`children.iter().rev()`) — geometricamente similar a
  TTB/LTR mas com order visualmente invertido. Refino futuro
  pode aplicar posicionamento absoluto reverso real per
  ADR-0054 graded.

### Validação em `native_stack`

- `dir` aceita só string `"ltr"`/`"rtl"`/`"ttb"`/`"btt"`
  (helper `extract_dir`). Outros valores ou tipos rejeitados.
- `spacing` aceita Length/Float/Int (em pt); negativo
  rejeitado.
- Named arg desconhecido rejeitado (sem scope-out adicional;
  vanilla stack tem apenas estes 3 atributos — lista
  pequena).
- Children variádicos: aceita Content ou Str; outros tipos
  rejeitados (estricto).

### Construtores

- Stdlib: `#stack(dir: ?, spacing: ?, ..children)`.
- Construtor Rust: `Content::stack(children: Vec<Content>,
  dir, spacing)` (Vec interno convertido para `Arc<[T]>`
  via `into()`).

### Limitações conscientes (P156I)

- `BTT`/`RTL` implementadas como reverse iteration em vez de
  posicionamento absoluto reverso real. Per ADR-0054 graded.
- Sem alignment per-child (vanilla `StackChild` tem
  alinhamento opcional). Refino futuro.
- Sem show rules `#show stack: ...` neste passo.
- Stack aninhado suportado estruturalmente; não testado E2E.

### Tipo `Dir` (infraestrutura paralela)

`Dir { LTR, RTL, TTB, BTT }` foi criado neste passo como
infraestrutura genérica reusável, análoga a `Sides<T>`
(P156C) e `Parity` (P156E). Vive em `01_core/src/entities/dir.rs`.
Reuso futuro previsível em refino bidi shaping ou
`Content::Columns { dir, ... }` quando Fase 3 for atacada.

### Padrão emergente "Smart<T> → Option<T> ou default" — N=5

P156I aplica novamente o padrão simplificador:
- P156E `Smart<Parity>` → `Option<Parity>`.
- P156F angles default 0 (em vez de Smart).
- P156G `Smart<Rel<Length>>` para width → `Option<Length>`.
- P156H idem Box.width.
- **P156I `Smart<Rel<Length>>` para spacing → `Option<Length>`;
  `Smart<Dir>` simplificado para `Dir` directo com Default
  natural (TTB).**

**N=5 aplicações** — patamar empírico forte. Candidato a
registo formal em ADR meta futuro.

---

## Variant `Content::Repeat` — Passo 156J (ADR-0061 Fase 3, sub-passo 1; **primeira Fase 3**)

P156J adiciona **um variant** ao enum `Content` cobrindo
repetição de body para preencher espaço, análoga a vanilla
`RepeatElem`. **Primeira aplicação Fase 3** (ADR-0061; activa
caminho 1 — materializar Fase 3).

**Decisão arquitectural reusada de P156G/H/I**: variant rico
(Opção A) — atributos `gap` e `justify` são propriedades
específicas de repetição; `Style` enum cobre só propriedades
de texto. Coerente com Block/Boxed/Stack.

### `Content::Repeat { body, gap, justify }`

```rust
Repeat {
    body:    Box<Content>,
    /// Espaço entre cópias; `None` == zero (padrão Smart→Option N=6).
    gap:     Option<Length>,
    /// `true` == distribuir espaço residual aumentando gap real.
    /// Default vanilla `true`. Distribuição real adiada per
    /// ADR-0054 graded.
    justify: bool,
}
```

**Atributos** (paridade vanilla):
- `body`: conteúdo a repetir (obrigatório).
- `gap: Option<Length>`: espaço entre cópias; `None` == zero.
- `justify: bool`: default vanilla `true` (paridade).

### Stdlib `repeat`

`#repeat[.]` ou `#repeat(body, gap: ?, justify: ?)`. Processado
por `native_repeat` em `stdlib/layout.rs`. Helper `extract_length`
reusado para `gap` (sexta aplicação consecutiva — N=6).

### Layout per ADR-0054 graded (paridade estrutural)

P156J implementa **paridade estrutural** (variant + stdlib +
medição estática + layout single-render). Algoritmo dinâmico
de quantidade-para-encher (`floor(available / (body_width +
gap))`) está diferido — exige refactor inline-region não
disponível no Layouter actual (mesma razão que `Block.width` /
`Boxed.width` em P156G/H).

**Layout actual**: emite o body uma vez no contexto actual.
**Walk**: percorre o body uma vez (counters/labels resolvem;
sem multiplicação de state — vanilla também só conta uma vez).

### `is_empty` / `plain_text` / `map_*`

- `is_empty()`: proxy via body (consistente com Block/Boxed).
- `plain_text()`: recurse no body sem multiplicar (paridade
  não visível em texto plano).
- `PartialEq`: cobre todos os fields (body, gap, justify).
- `map_content` / `map_text`: recurse no body; preserva
  gap/justify (Copy primitivos).

### Limitações conscientes (P156J)

- Algoritmo dinâmico de "quantidade-para-encher" diferido per
  ADR-0054 graded. Single-render é aproximação aceite.
- `justify: true` armazenado mas distribuição real adiada.
- Erro vanilla "infinite content" (se largura disponível for
  unbounded) não implementado — requer detecção de fr context
  no Layouter.

### Padrão emergente "Smart<T> → Option<T> ou default" — N=6

P156J aplica o padrão simplificador pela sexta vez consecutiva:
- P156E `Smart<Parity>` → `Option<Parity>`.
- P156F angles default 0 (em vez de Smart).
- P156G/H `Smart<Rel<Length>>` para width → `Option<Length>`.
- P156I `Smart<Rel<Length>>` para spacing → `Option<Length>`.
- **P156J `Length` (default zero vanilla) → `Option<Length>`
  para `gap`; `bool` directo para `justify` (default vanilla
  `true`).**

**N=6 aplicações** — patamar empírico forte e crescente.
Promoção a ADR meta segue como candidato P156K-meta documentado
em ADR-0061 §"Aplicações cumulativas".

### Helper `extract_length` reuso N=6

`extract_length` em `stdlib/layout.rs` foi reusado em P156C
(pad), P156D (h+v), P156G (block.width/height/inset), P156H
(box.width/height/inset/baseline), P156I (stack.spacing) e
agora **P156J (repeat.gap)**. Sexta aplicação consecutiva —
emergiu como vocabulário canónico para coerção de Length em
named args. Promoção a helper público em release futuro
(refactor scope-out).

## Variante `Content::StateDisplay` — Passo 240 (M9d/M7+1; ADR-0081 PROPOSTO P239 Opção γ)

```rust
Content::StateDisplay {
    key:      String,
    callback: Option<crate::entities::func::Func>,
}
```

Render-mediated state display vanilla `state.display(callback)`.
Walk emite Tag via `extract_payload` arm; `apply_state_displays`
pós-fixpoint (paralelo `apply_state_funcs` P191B) pre-renderiza
Content via `apply_func(callback, [value], ctx, engine)` e
armazena em `Introspector.state_displays[(key, loc)]`. Layout
arm consome via `Introspector::state_display_value(key, loc)`
— Layouter permanece puro (sem Engine+ctx em signature; paridade
arquitectural estrita Opção γ vs α/β/δ P239 audit).

**`callback: None`**: state value renderiza directo
(`Value::Content(c)` passa-through; `Value::Str(s)` via
`Content::text(s)`; outros tipos fallback `Content::Empty`).
**`callback: Some(func)`**: aplicada ao value via apply_func;
resultado convertido para Content pela mesma regra.

**Primeira excepção justificada à aplicação automática
ADR-0080 EM VIGOR pós-P229** — feature runtime nova + walk
integration merece L0 tocado partial (este bloco + bloco
`state_display` em `rules/stdlib.md` + bloco `apply_state_displays`
em `rules/introspect.md`).

## Variante `Content::CounterDisplayCallback` — Passo 241 (M9d/M7+2; ADR-0081 IMPLEMENTADO parcial paralelo absoluto P240)

```rust
Content::CounterDisplayCallback {
    key:      String,
    callback: Option<crate::entities::func::Func>,
}
```

Render-mediated counter display real walk-time vanilla
`counter.display(callback)`. **Paralelo absoluto** a
`Content::StateDisplay` P240 (mesmo pattern; mesma arquitectura
Opção γ; Layouter permanece puro).

Walk emite Tag via `extract_payload` arm; `apply_counter_displays`
pós-fixpoint (paralelo `apply_state_displays` P240) converte
`intr.counters.value_at(key, loc)` (Option<&[usize]>) para
`Value::Array(Vec<Value::Int>)` representando counter state e
chama `apply_func(callback, [array], ctx, engine)`. Resultado
Content armazenado em `intr.counter_displays[(key, loc)]`. Layout
arm consome via `Introspector::counter_display_value(key, loc)`.

**Forma do Value passado ao callback** (Decisão 4 P241): paridade
vanilla `CounterState = SmallVec<[u64; 3]>` representado como
`Value::Array(Vec<Value::Int>)`. Counter inexistente:
`Value::Array(vec![])` (vector vazio).

**`callback: None`** + counter populated: formato default "1.2.3"
via join "." (paridade `formatted_counter_at` P177).
**`callback: None`** + counter inexistente: `Content::Empty`.

**Coexiste com `Content::CounterDisplay { kind }` legacy
single-pass** — variant nova paralela preservada inalterada
(Decisão 1 P241 Opção α: variant nova vs refino legacy).

**Segunda excepção justificada ADR-0080 EM VIGOR pós-P229** —
N=1 (P240) → 2 (P241) cumulativo; pattern "L0 tocado para
features runtime novas + walk integration" promove-se a N=2.

## Refino `Content::Block.radius` + `Content::Boxed.radius` — Passo 242 (M9d/M7+5; ADR-0081 IMPLEMENTADO parcial 3/5)

P231 introduziu fields `radius: Option<Length>` + `clip: bool` em
ambos os variants como scope-out P156G/H graded ("semantic adiada").
P242 promove para semantic real via:

```rust
// Refino tipo radius:
- radius: Option<Length>,      // P231 — single Length OR None
+ radius: Corners<Length>,     // P242 — per-corner (top_left/top_right/bottom_right/bottom_left)

// Default migrado:
- radius: None,
+ radius: Corners::uniform(Length::ZERO),
```

Audit C1 P242 refinou hipótese spec: assumira "5 fields → 7 fields"
mas Block/Boxed já tinham 8 fields P231; ajuste real é "refine field
type" (`Option<Length>` → `Corners<Length>`) + materialize semantic
clip. Sem `P242.div-N` formal — paridade lição N=5 cumulativo
ajustes triviais audit precedentes.

**`clip` semantic materializada P242**:
- `clip: false`: comportamento inline original preservado (radius
  armazenado sem clip-mask emit; semantic radius isolada continua
  graded).
- `clip: true` + radius zero: Layouter emite `FrameItem::Group` com
  `clip_mask: Some(ShapeKind::Rect)` (paridade DEBT-30 P79).
- `clip: true` + radius non-zero: Layouter emite `FrameItem::Group`
  com `clip_mask: Some(ShapeKind::RoundedRect { radii: radius })`;
  PDF exporter desenha Bezier 4 corners path via
  `emit_rounded_rect_ops` (kappa = 0.552_284_749_831 paridade
  Ellipse same ficheiro).

**Promoção real graded ADR-0054 P156G/H → semantic concreta P242**:
sub-padrão emergente "promoção real scope-out ADR-0054 graded"
N=1 inaugurado P242 (Categoria A.4 P231 graded → A.4 materializado
parcial). Outset/fill/stroke restantes em Block/Boxed permanecem
scope-out (refino futuro).

stdlib `block(radius:)` / `box(radius:)` aceitam:
- `Length` uniforme (paridade pre-P242; `extract_length`).
- `Dict` por canto: `top-left` / `top-right` / `bottom-right` /
  `bottom-left` / `top` / `bottom` / `left` / `right` / `rest`.
  Precedência: canto específico > eixo > rest (paridade
  `extract_sides_lengths` per ADR-0064 Caso C).

## Promoção scope-outs Pad.right / Block.width / Boxed.width — Passo 243 (M9d / M7+3 fase (a); ADR-0081 IMPLEMENTADO parcial 4/5)

P156C declarou `Pad.right` scope-out ("Layouter actual não tem
mecânica de largura útil por arm"). P156G/H declararam
`Block.width` / `Boxed.width` semantic real adiada ("armazenado
mas não impõe limite real"). **P243 promove os 3 para semantic
real** via `regions.current.width` save/restore:

```rust
// Em layout/mod.rs Pad arm:
let saved_width = self.regions.current.width;
self.regions.current.width = (saved_width - right).max(0.0);
self.layout_content(body);
self.regions.current.width = saved_width;
```

```rust
// Em Block arm:
let saved_width = self.regions.current.width;
if let Some(w) = width {
    let w_pt = w.resolve_pt(font);
    self.regions.current.width = (line_start + w_pt).max(0.0);
}
self.layout_content(body);
self.regions.current.width = saved_width;
```

```rust
// Em Boxed arm (paralelo Block):
let saved_width = self.regions.current.width;
if let Some(w) = width {
    let w_pt = w.resolve_pt(font);
    self.regions.current.width = (cursor_x + w_pt).max(0.0);
}
self.layout_content(body);
self.regions.current.width = saved_width;
```

**Mecânica**: `layout_word` em `cursor.rs` consulta
`self.regions.current.width` para width-aware wrap; promoção
P243 garante que width efectiva reflecte constraint user-provided
durante body layout. **Save/restore LIFO** preserva semantic
cumulativo para Pad/Block aninhados (P243 test
`p243_pad_aninhado_largura_cumulativa_preservada`).

**Sub-padrão "promoção real scope-out ADR-0054 graded"** N=2
cumulativo (P242 radius/clip + P243 multi-region attrs).
**§"Limitações conscientes" P156C/G/H** secções relevantes
transitam de "scope-out" / "armazenado adiada" para
"materializado P243" (anotação cruzada).

## Promoção scope-outs Block/Boxed fill+stroke+outset — Passo 247 (M9d / M7+5; ADR-0079 Categoria A.4)

P156G/H declararam **9 atributos vanilla scope-out** em Block e
**6 em Boxed** (per ADR-0054 graded; refino futuro). P231
materializou `outset` armazenado (semantic adiada). P242 promoveu
`radius` + `clip` para semantic real (RoundedRect + clip_mask).
**P247 promove 3 cosméticos visuais em agregação**: `outset`
semantic real activado + **2 fields novos** `fill` + `stroke`
em Block + Boxed (paridade simétrica).

```rust
// Em Content::Block (P247):
Block {
    body, width, height, inset, breakable,
    outset, radius, clip,                                  // P231/P242
    fill:   Option<Color>,                                 // P247 NOVO
    stroke: Option<Stroke>,                                // P247 NOVO
}

// Em Content::Boxed (P247; paridade simétrica):
Boxed {
    body, width, height, inset, baseline,
    outset, radius, clip,                                  // P231/P242
    fill:   Option<Color>,                                 // P247 NOVO
    stroke: Option<Stroke>,                                // P247 NOVO
}
```

**Default**: `fill: None`, `stroke: None`, `outset: Sides::ZERO`
preservam output bit-equivalente a P246 (backward compat estrita).

**Types fixados** (per audit C1 §2.2 — `Color` Copy / `Stroke`
Clone existentes em `geometry.rs:24`; `Paint` enum não existe):

- `fill: Option<Color>` — `Color` Copy directo (Stroke já usa
  Color directo). Refactor para `Paint` enum é cross-cutting
  fora de scope P247 (futuro ADR dedicada).
- `stroke: Option<Stroke>` — reuso de struct `Stroke { paint:
  Color, thickness: f64 }`.

**Layouter activação Shape + outset semantic real (Decisão 3-5)**:

Quando `fill.is_some() || stroke.is_some() || outset != ZERO`,
Layouter emite `FrameItem::Shape { pos, kind, width, height,
fill, stroke }` ANTES do body (snapshot-and-insert via
`current_items.insert(items_before, ...)`):

```
outer bound:  pos.x - outset.left, pos.y - outset.top
shape bounds: outer_bound + (width + outset.left+right,
                              height + outset.top+bottom)
body origin:  pos.x + inset.left, pos.y + inset.top
```

- `kind`: `Rect` se radius == zero; `RoundedRect { radii: radius }`
  caso contrário (reuso P242).
- **Z-order**: Shape inserido em `items_before` para que
  fill+stroke renderizem por baixo do conteúdo body (paridade
  vanilla PDF z-order natural).
- **Outset semantic** (cenário A audit §2.4-§2.5 — outset zero-uso
  pré-P247): cursor.y avança `outset.top` antes do inset.top;
  `outset.bottom` após height min; bounds Shape expandem em
  todos os lados.
- **clip=true preserva semantic P242**: body items wrapped em
  `FrameItem::Group` com clip_mask; Shape fill+stroke + Group(body)
  coexistem (Shape primeiro, Group depois — z-order natural).

**stdlib `block(fill:, stroke:)` + `box(fill:, stroke:)` (P247)**:

- `fill` aceita `Value::Color` directo; tipos inválidos rejeitados
  com erro hard (paridade pattern Grid/Table P228).
- `stroke` reusa `extract_stroke` helper pré-existente (P227
  `stdlib/layout.rs:351`): aceita `Length` (Color preto + thickness
  resolvido), `Color` (thickness default 1pt), ou `Stroke` directo.

**Sub-padrão "promoção real scope-out ADR-0054 graded"** N=2 →
**N=3 cumulativo** P247 (P242 radius+clip = N=2; **P247
outset+fill+stroke = N=3 promoções reais agregadas**). Contando
granular: 5 promoções cumulativas (P242 radius + P242 clip +
P247 outset + P247 fill + P247 stroke).

**Sub-padrão emergente "agregar promoções scope-outs cosméticos
visuais"** N=1 inaugurado P247 (3 promoções num passo único;
magnitude controlada M-L; coesão semantic forte).

**§"Limitações conscientes" P156G fechadas em P247**: 5 dos 9
scope-outs originais Block fechados cumulativamente (outset
P231→P247 + radius P242 + clip P242 + fill P247 + stroke P247);
restam 4 (spacing + above + below + sticky).

**§"Limitações conscientes" P156H fechadas em P247**: 5 dos 6
scope-outs originais Boxed fechados cumulativamente (outset +
radius + clip + fill + stroke); resta 1 (stroke-overhang).

## Promoção graded → real semantic Block.breakable + Boxed.height + TableCell overflow — Passo 248 (M9d / M7+5; ADR-0079 Categoria A.4 cumulativa)

P156G declarou `Block.breakable` "semantic adiada per ADR-0054
graded — armazenado mas não impede quebra". P156H declarou
`Boxed.height` "semantic real adiada". P157B declarou
`TableCell.body` sem detecção de overflow vertical. **P248
activa as 3 semanticas em agregado** via mecanismo comum de
medição antecipada (`measure_content_constrained` puro
pré-existente, audit C1 §2.4 confirmado).

**Activação A — `Block.breakable`** (Layouter `mod.rs` Block arm):

```rust
if !*breakable {
    let avail_w = match width {
        Some(w) => w.resolve_pt(font),
        None    => self.available_width(),
    };
    let (_, body_h) = self.measure_content_constrained(body, avail_w);
    let height_min = height.map(|h| h.resolve_pt(font)).unwrap_or(0.0);
    let inner_h = body_h.max(height_min);
    let block_total_h = outset_top + inset_top + inner_h
                       + inset_bottom + outset_bottom;
    let page_usable_h = self.available_height();
    let remaining_h = self.page_bottom_limit()
                    - self.regions.current.cursor_y.0;
    if block_total_h <= page_usable_h && block_total_h > remaining_h {
        self.new_page();
    }
    // else: cabe na actual OU overlong (emit normal — paridade vanilla).
}
```

3 cenários distintos:
- Cabe na página actual → emit normal preservado.
- Cabe numa página nova mas não na actual → `new_page()`
  antecipado antes do emit.
- Overlong (excede página inteira) → emit normal (paridade
  vanilla "overlong atómico não quebra").

**Default `breakable: true` preserva comportamento P156G literal**
(zero overhead; sem medição antecipada).

**Activação B — `Boxed.height` overflow** (Layouter Boxed arm):

```rust
if let Some(h) = height {
    if *clip {
        let h_pt = h.resolve_pt(font);
        let (body_w_real, body_h_real) =
            self.measure_content_constrained(body, avail_w_box);
        if body_h_real > h_pt {
            let body_items = drain items emitidos pelo body;
            push FrameItem::Group {
                pos: top-left da caixa,
                clip_mask: Some(ShapeKind::Rect),
                inner_height: h_pt,
                items: body_items,
            };
        }
    }
}
```

- `height: None` → preservado P156H literal.
- `height: Some(h)` + body cabe → preservado.
- `height: Some(h)` + body excede + `clip: true` → wrap em Group
  com clip_mask Rect (reuso mecanismo P242).
- `height: Some(h)` + body excede + `clip: false` → emit normal
  (overflow visível; paridade vanilla default).

**Activação C — `TableCell.body` overflow clip implícito**
(Layouter `grid.rs` GridCell/TableCell arm):

```rust
let (cell_h_measured, cell_items) =
    self.layout_sub_frame_with_width(cell, body_x, body_w);
// ... translate cell_items para abs ...
let cell_overflow = cell_h_measured > body_h;
if cell_overflow {
    push FrameItem::Group {
        pos: (body_x, body_y),
        clip_mask: Some(ShapeKind::Rect),
        inner_width: body_w,
        inner_height: body_h,
        items: translated_items,
    };
} else {
    for item in translated_items { push directo; }  // P157B preservado
}
```

- Cell body cabe em `regions.cell.height` (P246) → preservado
  P157B literal.
- Cell body excede → **clip implícito ao limite cell** (paridade
  vanilla default).
- **Row break real é scope-out P248** (refino futuro per
  ADR-0054 graded; promoção candidata a passo dedicado;
  DEBT-34e preservado aberto cumulativo — distinto: DEBT-34e
  cobre colspan/rowspan placement, P248 cobre overflow Y).

**Sub-padrão "promoção graded → real semantic activação consumer"
N=1 → N=2 cumulativo P248**: P245 inaugurou N=1 (Place float
real); **P248 N=2 cumulativo agregado** (3 sub-activações
granulares em passo único: breakable + height + cell overflow).

**Sub-padrão emergente "agregar promoções graded → real
multi-consumer via mecanismo comum"** N=1 inaugurado P248:
distinto de P247 "agregar promoções cosméticos visuais"
(ortogonais aditivos) — P248 agrega semantic real com
mecanismo comum (medição antecipada).

**Promoções reais scope-outs ADR-0054 graded granular cumulativas
pós-P248**: 8 = (P242 radius + P242 clip) + (P247 outset + P247
fill + P247 stroke) + (P248 breakable + P248 height + P248 cell
overflow). Limiar conceptual sólido para ADR meta candidata
futura XS admin (N≥6 patamar atingido).

**§"Limitações conscientes" P156G/H/P157B fechadas em P248**:
- Block.breakable semantic real activada (resta 4/9 cumulativo:
  spacing + above + below + sticky).
- Boxed.height semantic real activada cumulativamente.
- TableCell overflow Y clip implícito (row break diferido).

## Promoção Block spacing + above + below + sticky — Passo 250 (M9d / M7+5; ADR-0079 Categoria A.4 Block COMPLETO; cita ADR-0082 PROPOSTO N=1 primeira aplicação citante)

P156G declarou 9 scope-outs originais Block; P247 + P248
fecharam cumulativamente 5/9 cosméticos visuais + breakable
semantic real. **P250 fecha os 4 scope-outs restantes** em
agregação (spacing + above + below + sticky) e marca **Block
A.4 COMPLETO 10/10** (incluindo breakable contado como décimo
elemento).

```rust
// Em Content::Block (P250):
Block {
    body, width, height, inset, breakable,        // P156G + P248
    outset, radius, clip,                         // P231 + P242
    fill, stroke,                                 // P247
    spacing: Option<Length>,                      // P250 NOVO
    above:   Option<Length>,                      // P250 NOVO
    below:   Option<Length>,                      // P250 NOVO
    sticky:  bool,                                // P250 NOVO
}
```

**Block fields: 10 → 14**. **Boxed fields: 10 preservado** —
estes 4 scope-outs são exclusivos Block (vanilla BlockElem
properties; BoxElem não os tem). **Asymetria intencional**;
sub-padrão "refino aditivo paralelo entre variants irmãos" N=5
P247 **não aplica P250**.

**Default values**:
- `spacing: None` (cristalina graded; vanilla `Em::new(1.2)`).
- `above: None` (fallback `spacing`).
- `below: None` (fallback `spacing`).
- `sticky: false`.

**Activação A — `spacing`/`above`/`below` cursor.y advance via
collapse semantic** (paridade vanilla CSS margin collapse):

- `above_pt = above.or(spacing).map(resolve).unwrap_or(0.0)`.
- `below_pt = below.or(spacing).map(resolve).unwrap_or(0.0)`.
- Entre Blocks consecutivos: `gap = max(prev.below, curr.above)`;
  cursor.y advance = `gap - prev.below_already_applied`.
- Primeiro Block do Sequence: `above` suprimido (`block_chain_
  active == false`).
- Non-Block intermediário quebra chain (`block_chain_active`
  reset).

**Layouter fields novos P250**:
- `prev_block_below_pending: f64` (default 0.0) — below pendente
  do prev Block para CSS-style collapse.
- `block_chain_active: bool` (default false) — chain state;
  reset entre Sequences (save/restore) + non-Block children.

**Activação B — `sticky` lookahead 1-block** (Sequence consumer):

```rust
if let Content::Block { sticky: true, .. } = part {
    if let Some(next) = iter.peek() {
        let part_h = measure_content_constrained(part, avail_w).1;
        let next_h = measure_content_constrained(next, avail_w).1;
        let combined = part_h + next_h;
        let remaining = page_bottom - cursor_y;
        let page_usable = available_height;
        if combined > remaining && combined <= page_usable {
            new_page();  // break antes do block sticky
        }
        // else: cabe OU overlong → emit normal.
    }
}
```

**Refactor Sequence consumer cross-arm P250** (pattern emergente
N=1 inaugurado):

```rust
Content::Sequence(parts) => {
    let saved_below = self.prev_block_below_pending;
    let saved_chain = self.block_chain_active;
    self.prev_block_below_pending = 0.0;
    self.block_chain_active       = false;
    let mut iter = parts.iter().peekable();
    while let Some(part) = iter.next() {
        // Sticky pre-layout lookahead.
        // ... see Activação B ...
        self.layout_content(part);
        if !matches!(part, Content::Block { .. }) {
            self.block_chain_active       = false;
            self.prev_block_below_pending = 0.0;
        }
    }
    self.prev_block_below_pending = saved_below;
    self.block_chain_active       = saved_chain;
}
```

**stdlib `block(spacing:, above:, below:, sticky:)` P250**:

- `spacing`/`above`/`below`: helper inline `extract_block_length`
  (paridade pattern P247); negativos rejeitados com erro hard.
- `sticky`: `Value::Bool` directo; tipos errados rejeitados.

**Citação ADR-0082 PROPOSTO N=1 (primeira aplicação citante)**:

Os 4 critérios operacionais ADR-0082 verificados:
1. **Storage prévio** ✓ — 4 fields scope-out P156G declarados
   originalmente.
2. **Consumer Layouter pre-promoção graded** ✓ — 4 args
   "rejeitados em `native_block` com erro hard" P156G.
3. **Paridade vanilla referência empírica** ✓ — audit C1 §2.4
   confirmou: vanilla `Em::new(1.2)` default; `above.or(spacing)`
   fallback; `max(prev.below, curr.above)` collapse; sticky
   default false.
4. **Backward compat literal** ✓ — defaults (None×3 + false)
   produzem output PDF bit-equivalente para Block sem estes
   args (sentinela `p250_block_defaults_preserva_output_pre_p250`).

**Sub-padrão "promoção real scope-out ADR-0054 graded"**
granular N=8 → **N=12 cumulativo P250** (P242 radius + P242
clip + P247 outset + P247 fill + P247 stroke + P248 breakable
+ P248 height + P248 cell_overflow + **P250 spacing + above +
below + sticky**).

**Sub-padrão "Refactor Sequence consumer cross-arm"** N=1
inaugurado P250 — primeira aplicação peekable + neighbour
context no Layouter. Pattern candidato a formalização N=3-4
futuro (hipóteses: pagebreak weak collapse; HSpace/VSpace
weak adjacent).

**§"Limitações conscientes" P156G fechadas em P250**: 10/10
scope-outs originais P156G + breakable contado = **Block A.4
COMPLETO**.

**Boxed continua 5/6 scope-outs** (resta stroke-overhang;
P250 não toca Boxed por assimetria intencional).

---

## Estado actual cumulativo (reconciliação P258 Cenário B1)

**P258 audit empírico Fase A** (`diagnostico-model-fase-a-passo-258.md`)
confirmou **cobertura Model ~73%** (ponderado linear; +25pp face
P154A 48%). Representação base inicial deste prompt (`Empty, Text,
Space, Sequence` + comentário "Variantes futuras") está
**desactualizada vs enum real pós-M3-M9 + P199B + P252/P257**:
o enum tem **~62 variants** cumulativos.

**Decisão arquitectural P258**: representação inicial preservada
como **histórico cumulativo** (paridade pattern ADR-0080 §"refactor
aditivo"); secções subsequentes deste prompt L0 (12+ anotações
cumulativas P154B/P155/P157A-C/P159A/P247/P250/P251/P252) cobrem
materializações reais variant-por-variant. **Não reconciliação
destructiva**.

### Sumário variants Content materializadas cumulativamente

Lista amostral (ordem aproximada de introdução):

- **Foundations** (P25-P101): `Empty`, `Text`, `Sequence`,
  `Styled`.
- **Markup básico** (P-M3): `Heading`, `Raw`, `ListItem`,
  `EnumItem`, `Link`, `Outline`.
- **Math** (P36-P40 + M3-M9): `Equation`, `MathSequence`,
  `MathIdent`, `MathText`, `MathFrac`, `MathAttach`, `MathRoot`,
  `MathDelimited`, `MathMatrix`, `MathCases`.
- **Introspector + Numbering** (P164-P204; P182C; P199B):
  `Labelled`, `Ref`, `SetHeadingNumbering`,
  `SetEquationNumbering`, `SetFigureNumbering`,
  `CounterDisplay`, `CounterUpdate`.
- **Figure** (P158): `Figure { body, caption, kind, numbering }`.
- **Visualize** (P25+): `Image`, `Shape`, `Transform`.
- **Grid + Table** (P82-P83; P157A-C; P227-P234): `Grid`,
  `GridHeader`, `GridFooter`, `GridCell`, `Table`, `TableCell`,
  `TableHeader`, `TableFooter`.
- **Bibliography + Cite** (P159A-G; Bloco B Model paridade
  manual): `Bibliography`, `Cite`.
- **Layout primitives** (P81-P96; P156C-L; P217-P252): `SetPage`,
  `Align`, `Place`, `Pad`, `Hide`, `HSpace`, `VSpace`,
  `Pagebreak`, `Colbreak`.
- **Markup compositivo** (P154B; P155): `Terms`, `TermItem`,
  `Quote`, `Divider`.
- **Block + Boxed** (P156G/H + P231/P242/P247/P248/P250/P252):
  `Block { body, width, height, inset, breakable, outset,
  radius, clip, fill, stroke, spacing, above, below, sticky }`
  (14 fields), `Boxed { body, width, height, inset, baseline,
  outset, radius, clip, fill, stroke }` (10 fields).
- **Stack + Repeat + Columns** (P156I-J; P217-P220): `Stack`,
  `Repeat`, `Columns`.

**~62 variants cumulativos total** (audit P258 Bloco 1).

### Variants PENDENTES pós-P258 (ausentes empíricos confirmados)

- **`Content::Footnote`** — Layout desbloqueio P156C preservado;
  variant Content + stdlib func não materializados. Candidata
  refino P-Footnote-N futuro (M; +10-15 tests).
- **`Content::Document`**, **`Content::Title`**,
  **`Content::Asset`** — Fase 3 condicional ADR-0060 §"Fase 3
  condicional"; sem prioridade designada; scope-out formal
  preservado.

### `parcial` pendentes pós-P258

- **link**, **list**, **enum**, **par** — refinos
  atributos vanilla (`marker`/`tight`/`indent`/`leading`/etc.)
  preservados como scope-out informal P258 (cobertura útil
  via paridade observable básica preservada).

### Bloco B hayagriva — scope-out implícito P258

Bibliography + Cite cumpridas **cumulativamente via paridade
manual P159A-G** (`bib_entry.rs` 413 LoC; 16 fields universais
paridade `hayagriva::Entry` sem dependência crate real).
ADR-0062 PROPOSTO preservada; promoção a IMPLEMENTADO diferida
até consumer real exigir CSL styling completo.

### Estado agregado P258

| Estado | P154A | Audit P258 | Δ |
|--------|-------|------------|---|
| implementado | 4 | 4 | 0 |
| implementado⁺ | 4 | 10 | **+6** |
| parcial | 5 | 4 | -1 |
| ausente | 10 | 4 (footnote, document, asset, title) | **-6** |

**Cobertura ponderada linear**: P154A 48% → Audit P258 **~73%**
(Δ +25pp).

**Cenário Fase B**: ☑ **B1 (≥75% — fecho conceptual Model)**
— Bloco A massivamente materializado cumulativamente; Bloco B
scope-out implícito documentado; Fase 3 + footnote refinos
futuros candidatos.
