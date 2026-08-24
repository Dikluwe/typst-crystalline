# Prompt L0 — `stdlib/layout` — módulo `layout`

## P1140.20.2 — canvas preparatório

#set page transporta bleed/fill/background/foreground. Não registrar
page/std.page; .3/.4 completam e P1140.21 expõe constructor/reflexão.

## P1140.20.1 — preparação da geometria de página

Completar somente transporte de `#set page` para paper, flipped, binding e
margem lógica. Não registrar `page`/`std.page`, função nativa ou constructor
parcial. P1140.20.2–.4 completam argumentos; P1140.21 cria função/reflexão.
Named desconhecido é rejeitado, nunca ignorado.
Hash do Código: 39cd4476

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/stdlib/layout.rs`
**Origem**: Passo 96.5 (extraído de `stdlib.rs` conforme ADR-0037), com marcos P82, P83, P84.5, P84.6, P156C–P156L, P156H–P156J, P156E, P218–P220, P222, P223, P224, P227, P228, P231, P242, P247, P250, P252, P335.
**ADRs**: ADR-0033 (divergência intencional), ADR-0037 (coesão por domínio), ADR-0054 (perfil graded / scope-outs), ADR-0061 (layout roadmap), ADR-0064 (smart option default), ADR-0078 (multi-region flow), ADR-0079 (stroke), ADR-0109 (atomização forma B).
**Convenções partilhadas**: ver `00_nucleo/prompts/compiler/stdlib/_comum.md`.

---

## Módulo `layout` — funções nativas de layout

Este módulo implementa as funções globais de layout e espaçamento de Typst: alinhamento, colocação absoluta/flutuante, grid, padding, ocultação, espaçamentos horizontais/verticais, containers block/box, composição em stack, repetição, colunas, quebras de coluna, medição de content, construtor de stroke e quebras de página.

Todas as funções partilham a assinatura padrão de `native_*`:

```rust
fn native_X(
    ctx: &mut EvalContext,
    args: &Args,
    world: &dyn World,
    current_file: FileId,
) -> SourceResult<Value>
```

A maioria ignora `ctx`/`world`/`current_file`.

---

### `native_align(alignment, body)`

**Assinatura**: `align(alignment: Align | Str, body: Content | Str) -> Content`

**Argumentos**:
- 1º posicional `alignment`: `Value::Align` (sintaxe preferida, ex: `center + bottom`) ou `Value::Str` (legacy, ex: `"center"`).
- `body`: primeiro argumento posicional do tipo `Content` ou `Str`.
- Não aceita argumentos nomeados.

**Semântica**: Cria `Content::Align { alignment, body }` via `Content::align(alignment, body)`.

**Paridade vanilla**: Equivalente a `align(center + bottom, [body])` / `align("center", [body])`.

**Limitações / scope-outs**: Nenhuma conhecida; DEBT-36 (legacy strings) está encerrado.

**Testes canónicos**:
```
align(center, [x]) -> Content::Align { alignment: center, body: "x" }
align("right", [x]) -> Content::Align { alignment: right, body: "x" }
align([x]) -> Content::Align { alignment: left (default), body: "x" }
align(center) -> Err "align() exige um bloco de conteúdo"
align(center, [x], named:{x:1}) -> Err "argumento nomeado inesperado"
```

---

### `native_place(alignment, body, dx?, dy?, scope?, float?, clearance?)`

**Assinatura**: `place(alignment, body, dx: Length?, dy: Length?, scope: "column" | "parent"?, float: bool?, clearance: Length?) -> Content`

**Argumentos**:
- 1º posicional `alignment`: `Value::Align` ou `Value::Str`; default `"top-left"`.
- `body`: argumento posicional `Content` ou `Str`.
- `dx`, `dy`: deslocamento em pt (aceitam `Length`, `Float`, `Int`). Default `0`.
- `scope`: `"column"` (default) ou `"parent"`; `scope: "parent"` exige `float: true`.
- `float`: booleano, default `false` (P223).
- `clearance`: `Length` opcional (P223), não negativo.

**Semântica**: Cria `Content::Place { alignment, dx, dy, scope, float, clearance, body }`.

**Paridade vanilla**: Equivalente a `place(top + right, [body])`; floats reais são scope-out.

**Limitações / scope-outs**:
- Float real (P245) e posicionamento flutuante avançado scope-out per ADR-0054 graded.
- `scope: "parent"` sem `float: true` produz erro (paridade vanilla).

**Testes canónicos**:
```
place(center, [x]) -> Content::Place { alignment: center, body: "x", ... }
place(center, [x], dx: 5pt) -> dx: 5
place(center, [x], scope: "parent") -> Err "scope \"parent\" requer float: true"
place(center, [x], clearance: -1pt) -> Err "valor negativo"
```

---

### `native_grid(columns?, rows?, ...cells)`

**Assinatura**: `grid(columns: TrackSizing[]?, rows: TrackSizing[]?, ..cells: Content[], gutter: Length?, align: Align?, inset: Length?, header: Content?, footer: Content?, stroke: Stroke?, fill: Color?) -> Content`

**Argumentos**:
- `columns` / `rows`: array de tracks (`Length`, `Fraction`, `"auto"`, ou `Int` > 0 expandindo para N tracks `auto`). Default: `[auto]`.
- `cells`: variádicos posicionais `Content`.
- `gutter`: `Length` uniforme, não negativo.
- `align`: `Value::Align`, default top-left.
- `inset`: `Length` uniforme (default `0pt`); per-side é refino futuro.
- `header` / `footer`: `Content` opcional.
- `stroke`: `Length` / `Color` / `Stroke` shorthand (P227).
- `fill`: `Color` (P228).

**Semântica**: Cria `Content::Grid(GridElem { columns, rows, cells, gutter, align, inset, header, footer, stroke, fill })`.

**Paridade vanilla**: Equivalente a `#grid(columns: 2, ..)`; layout multi-region real scope-out per ADR-0078.

**Limitações / scope-outs**:
- `inset` per-side via dict é refino futuro.
- Stroke/fill usam subconjuntos simplificados.

**Testes canónicos**:
```
grid([a], [b], columns: 2) -> Grid 1×2
grid(columns: (1fr, 2fr), rows: 3, [a],[b],[c]) -> Grid 3 linhas
grid([a], gutter: -2pt) -> Err "negativo"
```

---

### `native_pad(body, left?, right?, top?, bottom?, x?, y?, rest?)`

**Assinatura**: `pad(body: Content | Str, left: Length?, right: Length?, top: Length?, bottom: Length?, x: Length?, y: Length?, rest: Length?) -> Content`

**Argumentos**:
- 1º posicional `body`: `Content` ou `Str`.
- Lados: `left`, `right`, `top`, `bottom`.
- Eixos: `x` (left+right), `y` (top+bottom).
- `rest`: aplica-se aos quatro lados.
- **Precedência**: específico > eixo > `rest`.

**Semântica**: Cria `Content::Pad { body, sides: Sides<Option<Length>> }` via `Content::pad(body, sides)`. Lados não especificados ficam `None` (default zero em uso).

**Paridade vanilla**: Equivalente a `pad(left: 1em, rest: 5pt, [body])`.

**Limitações / scope-outs**: Padding negativo rejeitado (vanilla aceita; divergência intencional per ADR-0054).

**Testes canónicos**:
```
pad([x], rest: 5pt) -> Pad { sides: all 5pt }
pad([x], x: 1em, left: 2em) -> left 2em, right 1em
pad([x], left: -1pt) -> Err "negativo"
pad(123) -> Err "espera content ou string"
```

---

### `native_hide(body)`

**Assinatura**: `hide(body: Content | Str) -> Content`

**Argumentos**:
- 1º posicional `body`: `Content` ou `Str`.
- Sem argumentos nomeados.

**Semântica**: Cria `Content::Hide { body }` via `Content::hide(body)`. O contento é avaliado para side-effects (counters/labels) mas não contribui para o layout visível.

**Paridade vanilla**: Equivalente a `hide([body])`.

**Limitações / scope-outs**: Nenhuma conhecida.

**Testes canónicos**:
```
hide([x]) -> Content::Hide { body: "x" }
hide("x") -> Content::Hide { body: "x" }
hide(123) -> Err "espera content ou string"
hide([x], named:{x:1}) -> Err "argumento nomeado inesperado"
```

---

### `native_h(amount, weak?)`

**Assinatura**: `h(amount: Length | Fraction, weak: bool = false) -> Content`

**Argumentos**:
- 1º posicional `amount`: `Length` (ou `Float`/`Int` coagidos para pt) **ou
  `Fraction` (P842, #38)**. Não negativo.
- `weak`: booleano, default `false`.

**Semântica**: Cria `Content::HSpace { amount: Spacing, weak }` —
`Spacing::Absolute` via `Content::h_space(amount, weak)`; `Spacing::Fractional`
via `Content::h_space_fraction(fr, weak)` (P842). A expansão da fração
acontece no layout (`flush_line`/`finish`) — ver `entities/elements/h_space.md`
e `compiler/layout.md`.

**P1140.9 — presença de `weak`:** o extrator conserva `(valor, presença)`.
O caminho da nativa cria o elemento com `weak_explicit = true` quando a chave
`weak` existe em `Args::named`, mesmo para `weak: false`; omissão produz
`weak = false, weak_explicit = false`. O bit serve somente a `repr`/identidade
observável e não participa do layout.

**Paridade vanilla**: `#h(1fr)` distribui o espaço restante da linha
proporcionalmente (medido em P842, `temp/p842/l7_h_*.typ`: `A#h(1fr)B`
encosta B à margem direita; `#h(1fr)#h(2fr)` divide na razão 1:2;
`#h(10pt)#h(1fr)` combina fixo e fração).

**Limitações / scope-outs**:
- Comportamento de `weak` collapse adiado.
- Ratio (`h(50%)`) continua truncado para zero em `extract_length`
  (scope-out P475 — resolução percentual requer contexto de layout).

**Testes canónicos**:
```
h(1em) -> Content::HSpace { amount: Absolute(1em), weak: false }
h(1fr) -> Content::HSpace { amount: Fractional(1.0), weak: false }   // P842
h(10pt, weak: true) -> HSpace { weak: true }
h(-5pt) -> Err "negativo"
h(-1fr) -> Err "negativo"                                            // P842
h(1em, strong: true) -> Err "argumento nomeado inesperado"
```

---

### `native_v(amount, weak?)`

**Assinatura**: `v(amount: Length, weak: bool = false) -> Content`

**Argumentos**:
- Análogo a `native_h`, vertical. **P842**: `Fraction` continua rejeitado
  (distribuição vertical fracionária é outro mecanismo — scope-out
  registado no relatório de P842); a mensagem pré-P842
  (`"v() espera amount como length, recebeu fraction"`) preserva-se.

**Semântica**: Cria `Content::VSpace { amount, weak }` via `Content::v_space(amount, weak)`.

**P1140.9:** aplica a mesma preservação `(weak, weak_explicit)` de `h`.

**Paridade vanilla / limitações**: Idênticas a `h`, exceto `Fraction`
(scope-out P842).

**Testes canónicos**:
```
v(12pt) -> Content::VSpace { amount: 12pt, weak: false }
v(1em, weak: true) -> VSpace { weak: true }
```

---

### `native_linebreak(justify?)` — P1140.10

**Medição anterior à decisão (vanilla pinado `a51e02804`, 2026-08-24):**
`type(linebreak)` → `function`; `repr(linebreak())` → `"linebreak()"`;
true/false explícitos aparecem como `linebreak(justify: true|false)`. A sintaxe
markup `\` usa o mesmo elemento com o campo omitido e é sempre não justificada
(`typst-library/src/text/linebreak.rs:23-37`,
`typst-eval/src/markup.rs:106-110`).

**Assinatura:** `linebreak(justify: bool = false) -> Content`.

- zero posicionais;
- somente named `justify`;
- omissão cria `justify = false, justify_explicit = false`;
- named bool cria `justify_explicit = true`, inclusive para false;
- tipo inválido ou named desconhecido produz erro.

A nativa vive neste módulo, não em `stdlib/text`: o L0 do hub `text` já decide
que o homólogo vanilla `text/linebreak.rs` pertence no cristalino às fronteiras
de eval/layout. `make_stdlib` registra `Value::Func`, nunca `Value::Type`.

**Estado dividido:** P1140.10 entrega binding e transporte. O efeito visual de
`justify: true` é P1140.11; até lá o consumer continua a quebra não justificada
histórica. Esta incompletude é deliberada e não pode ser omitida de relatórios.

Testes canônicos:

```text
linebreak() -> Linebreak { justify: false, justify_explicit: false }
linebreak(justify: false) -> Linebreak { justify: false, justify_explicit: true }
linebreak(justify: true) -> Linebreak { justify: true, justify_explicit: true }
linebreak(1) -> erro
linebreak(justify: 1) -> erro
linebreak(foo: true) -> erro
```

---

### `native_block(body, width?, height?, inset?, breakable?, outset?, radius?, clip?, fill?, stroke?, spacing?, above?, below?, sticky?)`

**Assinatura**: `block(body: Content | Str?, width: Length?, height: Length?, inset: Length?, breakable: bool = true, outset: Length?, radius: Length | Dict?, clip: bool?, fill: Color?, stroke: Stroke?, spacing: Length?, above: Length?, below: Length?, sticky: bool?) -> Content`

**Argumentos**:
- `body`: posicional opcional (`Content`, `Str`, ou omitido → `Empty`).
- `width` / `height`: `Length` (ou `Float`/`Int` coagidos), ausente = auto.
- `inset`: `Length` uniforme, `Relative` (parte abs), ou `Dict {left?, right?, top?, bottom?, x?, y?, rest?}` per-side (P475). Default `0pt`.
- `breakable`: bool default `true`.
- `outset`: `Length` uniforme, `Relative` (parte abs), ou `Dict` per-side (P475). Default `0pt`.
- `radius`: `Length` uniforme ou dict por canto (P242).
- `clip`: bool default `false`.
- `fill`: `Color` (P247).
- `stroke`: `Length` / `Color` / `Stroke` (P247).
- `spacing`, `above`, `below`: `Length` opcionais (P250).
- `sticky`: bool default `false` (P250).

**Semântica**: Cria `Content::Block(BlockElem { body, width, height, inset, breakable, outset, radius, clip, fill, stroke, spacing, above, below, sticky })`. `inset`/`outset` resolvidos como `Sides<Length>` via `extract_sides_from_value`; parte `rel` de `Relative` truncada (scope-out: requer contexto de layout).

**Paridade vanilla**: Equivalente a `#block(width: 5cm, [body])`; scope-outs listados acima.

**Limitações / scope-outs**: `inset: (left: 50%)` — parte relativa truncada para `Length::ZERO`; fills complexos scope-out per ADR-0054 graded.

**Testes canónicos**:
```
block([x], width: 5cm) -> Block { width: 5cm, body: "x" }
block([x], inset: 1em, fill: red) -> Block com inset/fill
block([x], inset: (left: 3pt, right: 5pt)) -> Block { inset: {left:3, right:5, top:0, bottom:0} }
block([x], outset: (x: 2pt)) -> Block { outset: {left:2, right:2, top:0, bottom:0} }
block([x], width: -1cm) -> Err "negativo"
block([x], foo: 1) -> Err "argumento nomeado inesperado"
```

---

### `native_stack(dir?, spacing?, ..children)`

**Assinatura**: `stack(dir: "ttb" | "ltr" | "rtl" | "btt", spacing: Length?, ..children: Content[]) -> Content`

**Argumentos**:
- `dir`: string, default `"ttb"`.
- `spacing`: `Length` opcional, default zero.
- `children`: variádicos posicionais `Content` ou `Str`.

**Semântica**: Cria `Content::Stack { children, dir, spacing }` via `Content::stack(children, dir, spacing)`.

**Paridade vanilla**: Equivalente a `#stack(dir: ltr, [a], [b])`.

**Limitações / scope-outs**: `spacing` negativo rejeitado.

**Testes canónicos**:
```
stack([a], [b]) -> Stack { dir: TTB, children: ["a","b"] }
stack(dir: "ltr", spacing: 5pt, [a], [b]) -> Stack LTR 5pt
stack([a], spacing: -1pt) -> Err "negativo"
stack(123) -> Err "children devem ser content ou string"
```

---

### `native_box(body, width?, height?, inset?, baseline?, outset?, radius?, clip?, fill?, stroke?)`

**Assinatura**: `box(body: Content | Str?, width: Length?, height: Length?, inset: Length?, baseline: Length?, outset: Length?, radius: Length | Dict?, clip: bool?, fill: Color?, stroke: Stroke?) -> Content`

**Argumentos**:
- `body`: posicional opcional (`Content`, `Str`, omitido → `Empty`).
- `width`, `height`, `radius`, `clip`, `fill`, `stroke`: idênticos a `block`.
- `inset`, `outset`: `Length` | `Relative` (parte abs) | `Dict` per-side — idêntico a `block` (P475).
- `baseline`: `Length` (default zero); negativos aceites.

**Semântica**: Cria `Content::Boxed(BoxedElem { body, width, height, inset, baseline, outset, radius, clip, fill, stroke })`. `inset`/`outset` via `extract_sides_from_value` (idêntico a block).

**Paridade vanilla**: Equivalente a `#box(width: 1fr, [body])`.

**Limitações / scope-outs**: Parte `rel` de `Relative` truncada (scope-out: requer contexto de layout). Outras limitações idênticas a `block`.

**Testes canónicos**:
```
box([x]) -> Boxed { body: "x" }
box(width: 1fr, repeat[.]) -> Boxed { width: 1fr, body: Repeat }
box([x], baseline: -2pt) -> baseline: -2pt (aceite)
box([x], inset: (left: 3pt, top: 5pt)) -> Boxed { inset: {left:3, right:0, top:5, bottom:0} }
```

---

### `native_repeat(body, gap?, justify?)`

**Assinatura**: `repeat(body: Content | Str, gap: Length?, justify: bool = true) -> Content`

**Argumentos**:
- 1º posicional `body`: `Content` ou `Str`.
- `gap`: `Length` opcional, default zero.
- `justify`: bool default `true`.

**Semântica**: Cria `Content::Repeat { body, gap, justify }` via `Content::repeat(body, gap, justify)`.

**Paridade vanilla**: Equivalente a `#box(width: 1fr, repeat[.])` / `#repeat(gap: 2pt, [x])`.

**Limitações / scope-outs**:
- Cálculo runtime de `floor(available / (body_width + gap))` diferido; Layouter faz single-render do body.
- `gap` negativo rejeitado.

**Testes canónicos**:
```
repeat([.]) -> Content::Repeat { body: ".", gap: 0, justify: true }
repeat([.], gap: 2pt, justify: false) -> Repeat gap 2pt justify false
repeat(gap: -1pt) -> Err "negativo"
```

---

### `native_columns(count, body, gutter?)`

**Assinatura**: `columns(count: Int, body: Content | Str, gutter: Length?) -> Content`

**Argumentos**:
- 1º posicional `count`: `Int >= 1`.
- 2º posicional `body`: `Content` ou `Str`.
- `gutter`: `Length` opcional, não negativo.
- Apenas 2 posicionais.

**Semântica**: Cria `Content::Columns { body, count, gutter }`.

**Paridade vanilla**: Equivalente a `#columns(2, [body])`; multi-region flow real scope-out per ADR-0078.

**Limitações / scope-outs**: Layout de colunas reais é stub/transparente; `colbreak` faz downgrade a pagebreak.

**Testes canónicos**:
```
columns(2, [body]) -> Content::Columns { count: 2, body: "body" }
columns(0, [body]) -> Err "count >= 1"
columns(2, [body], gutter: 1em) -> Columns gutter 1em
columns(2, [a], [b]) -> Err "aceita 2 posicionais"
```

---

### `native_colbreak(weak?)`

**Assinatura**: `colbreak(weak: bool = false) -> Content`

**Argumentos**:
- `weak`: bool default `false`.
- Sem posicionais.

**Semântica**: Cria `Content::Colbreak { weak }` via `Content::colbreak(weak)`.

**P1140.9:** a nativa preserva `weak_explicit` pela presença da chave named.
Omissão e `weak: false` continuam iguais para layout, mas distintos para
`repr` e identidade de conteúdo.

**Paridade vanilla**: Equivalente a `#colbreak()`; downgrade a pagebreak sem multi-region real.

**Limitações / scope-outs**: Multi-region flow real scope-out per ADR-0078.

**Testes canónicos**:
```
colbreak() -> Content::Colbreak { weak: false }
colbreak(weak: true) -> Colbreak { weak: true }
colbreak("x") -> Err "não aceita posicionais"
```

---

### `measure(body)` — **P712: reescrito, medição real via layout isolado**

**Assinatura**: `measure(body: Content | Str) -> Dict { width: Length, height: Length }`

**Argumentos** (validados por `extract_measure_body`, `stdlib/layout.rs`,
partilhado entre `native_measure` e a intercepção real):
- 1º posicional `body`: `Content` ou `Str`.
- Não aceita named args (width/height override — scope-out, ADR-0054).

**Semântica (P712)** — duas peças, não uma:

1. **Intercepção real** (`eval_func_call`, `eval/closures.rs` §P712):
   reconhece `measure(...)` E `std.measure(...)` (qualificado — caminho
   real do `cetz`, `util.typ:197`) comparando a identidade de fn-ptr de
   `native_measure` (`native_fn_addr`, não o nome — não intercepta um
   `measure` sombreado pelo utilizador). Só esta intercepção tem acesso
   a `engine.styles` (necessário — o tamanho medido depende do `#set
   text(size:)` activo, paridade com o vanilla `context.styles()`) e ao
   gate `ctx.in_context` (mesma convenção de `counter.get()`/`state.get()`,
   `stdlib/counter.rs:144`/`stdlib/state.rs:63`): fora de `context`, erra
   `"measure() can only be used inside context"`. Dentro, chama
   `measure_content_real(&body, engine.styles)` (`layout/mod.rs` §P712).
2. **`measure_content_real`** (`layout/mod.rs`): constrói um `Layouter`
   isolado (`FixedMetrics` + `NullImageSizer` — L1 não tem métricas de
   fonte reais, `FallbackFontMetrics` é L3) com `chain` = a `StyleChain`
   do chamador, corre `layout_sub_frame` (Passo 629 — o mesmo mecanismo
   já reutilizado por `Content::Place`/`Content::Transform`/`grid.rs`
   para medir células) numa região efectivamente sem limites (`width:
   f64::INFINITY`, `height: None` — paridade com o vanilla
   `Region::new(.., Abs::inf())` para `measure()` sem `width`/`height`
   explícitos) e devolve `(width, height)` a partir dos itens realmente
   emitidos — não uma aproximação manual por tipo de `Content`.
- `native_measure` (o `NativeFn` registado em `scope`) passou a ser só
  o **fallback de invocação indirecta** (`measure` passado como valor
  de primeira classe, ex.: `arr.map(measure)` — sem consumidor medido).
  Sem acesso a `engine.styles`, **falha sempre** (em vez de devolver
  `(0, 0)` silenciosamente — ADR-0108); o caminho real e directo nunca
  o alcança (interceptado antes).

**Paridade vanilla**: Equivalente a `#measure([Hello world]).width`,
incluindo o gate de `context` (`foundations/measure.rs` `#[func(contextual)]`
no vanilla). **Divergência mecânica documentada, não de língua
(ADR-0107)**: `FixedMetrics` é monoespaçado (0.6×size/codepoint) — a
largura é real e proporcional ao conteúdo dado o motor de layout usado,
mas não byte-exacta ao vanilla (que faz shaping real via `rustybuzz`,
só disponível em L3).

**Limitações / scope-outs (inalterados)**:
- Runtime queries (counter values, labels) diferidas.
- `measure(body, width: 5cm)` scope-out.
- Invocação indirecta de `measure` (não directa/qualificada) não
  suportada — erro claro (P712), não `(0, 0)` silencioso.

**Testes canónicos**:
```
context measure([abc]) -> Dict { width: >0, height: >0 }   (dentro de context)
std.measure("abc") -> Dict { width, height }                (forma qualificada)
measure([abc])                    -> Err "can only be used inside context" (fora)
measure([abc], width: 5cm)        -> Err "named arg não suportado"
#let measure = (x) => x + 1; measure(4) -> 5 (sombreado, não intercepta)
```

---

### `native_stroke(paint?, thickness?, overhang?)`

**Assinatura**: `stroke(paint: Color?, thickness: Length?, overhang: bool = true) -> Stroke`

**Argumentos**:
- `paint`: `Color`, default `BLACK`.
- `thickness`: `Length` (ou `Float`/`Int`), default `1pt`. Deve ser > 0.
- `overhang`: bool default `true`.
- Sem posicionais.

**Semântica**: Cria `Value::Stroke(Stroke { paint: Paint::Solid(paint), thickness, overhang })`.

**Paridade vanilla**: Equivalente a `#stroke(thickness: 2pt)`.

**Limitações / scope-outs**: Outras propriedades de stroke (dash, cap, join, miter-limit) scope-out per ADR-0079.

**Testes canónicos**:
```
stroke(thickness: 2pt) -> Stroke { paint: Black, thickness: 2, overhang: true }
stroke(paint: red, overhang: false) -> Stroke red overhang false
stroke(thickness: 0pt) -> Err "> 0"
stroke(1pt) -> Err "não aceita posicionais"
```

---

### `native_pagebreak(weak?, to?)`

**Assinatura**: `pagebreak(weak: bool = false, to: "even" | "odd"?) -> Content`

**Argumentos**:
- `weak`: bool default `false`.
- `to`: `"even"` ou `"odd"`.
- Sem posicionais.

**Semântica**: Cria `Content::Pagebreak { weak, to }` via `Content::pagebreak(weak, to)`.

**P1140.9:** a nativa preserva `weak_explicit` pela presença da chave named.
`to: Option<Parity>` já preserva naturalmente omissão versus valor. A ordem
observável de `repr` é `weak`, depois `to`.

**Paridade vanilla**: Equivalente a `#pagebreak(to: "odd")`.

**Limitações / scope-outs**: Comportamento de `weak` collapse adiado.

**Testes canónicos**:
```
pagebreak() -> Content::Pagebreak { weak: false, to: None }
pagebreak(weak: true, to: "even") -> Pagebreak { weak: true, to: Even }
pagebreak(to: "odd") -> Pagebreak { to: Odd }
pagebreak("x") -> Err "não aceita posicionais"
```

---

### `page(...)` — auditoria P1140.18; binding ainda não materializado

P335 removeu `native_page` sob a premissa de que a forma-função era legacy e
que `#set page(...)` seria o único caminho canónico. A medição P1140.18 refuta
essa premissa: o vanilla ratificado `a51e02804` expõe `page` como `function` e
`PageElem` conserva `Construct`. O constructor é superfície vigente e distinta
da set rule: isola o body entre duas fronteiras de página, introduz um marker
`flush` que preserva body vazio e aplica a configuração somente ao conteúdo
isolado, restaurando as propriedades anteriores depois dele.

O binding continua deliberadamente ausente nesta fase, mas agora como lacuna
conhecida — não como remoção correcta de uma função legacy. Não restaurar o
antigo P335 `native_page`: ele aceitava somente `width`/`height`/`margin` e
devolvia um `Content::SetPage` isolado, forma que não satisfaz o constructor.

Classificação medida dos 19 parâmetros:

- **A, representados e consumidos no caminho set-rule:** `width`, `height`,
  `columns`;
- **B, parciais:** `margin` (sem `inside`/`outside` e binding), `numbering`
  (somente string/none) e `body` (há `Content`, mas não há isolamento/restauro);
- **C, ausentes:** `paper`, `flipped`, `bleed`, `binding`, `fill`, `supplement`,
  `number-align`, `header`, `header-ascent`, `footer`, `footer-descent`,
  `background`, `foreground`;
- **D:** nenhum.

Decisão γ de P1140.18: corrigir pré-requisitos antes de expor a função. A
nucleação futura fica explicitamente dividida:

1. **P1140.19:** especificar e materializar fronteira de page-run, marker
   invisível não vazio e restauração da `PageConfig` anterior;
2. **P1140.20:** completar a entidade/configuração e consumers dos parâmetros
   C e das partes B, em subconjuntos explicitamente rejeitáveis — nenhum
   argumento pode ser aceito e ignorado;
3. **P1140.21:** materializar o constructor atomizado, binding global/`std`,
   diagnósticos e rebaseline da superfície.

Esses números nomeiam dependências, não reservam autorização de código. Cada
mudança de contrato público ou comportamento por defeito mantém seu próprio
gate ADR-0127. Até P1140.21, `page` permanece ausente e `#set page(...)`
continua sendo o único caminho cristalino implementado.

#### P1140.26 — materialização pública condicionada ao gate ADR-0127

Medição em `page.rs:54-523` do vanilla ratificado `a51e02804`: `page` é um
constructor vigente com body obrigatório e 18 propriedades named; seu
`Construct` aplica configuração somente ao body entre fronteiras de página e
não produz elemento selecionável por show rule. No cristalino,
`entities/elements/page_run.rs:20-42` já transporta exatamente esses deltas.
Sondas no binário ratificado confirmam que `body` é exclusivamente posicional,
`paper` pode ser o primeiro posicional (`page("a5", [body])`), um posicional
além desses é inesperado e `std.page(..)` compila. Proveniência:
HEAD `45b547073d7686cdd5d3e3030c82de3e22ec395f`, working tree não commitado,
`2026-08-24T17:39:40-03:00`, stat
`126 files changed, 3251 insertions(+), 657 deletions(-)`.

Após confirmação do gate, definir `native_page` com assinatura de linguagem:

```text
page(
  paper?, width?, height?, flipped?, margin?, bleed?, binding?, columns?,
  fill?, numbering?, supplement?, number-align?, header?, header-ascent?,
  footer?, footer-descent?, background?, foreground?, body
) -> content
```

O constructor recebe body posicional obrigatório e aceita `paper` como
primeiro posicional opcional ou named, além das mesmas
formas tipadas já legitimadas para `#set page`, preserva omissão como `None` e
constrói `Content::PageRun(Arc<PageRunElem>)`. Named desconhecido, tipo inválido,
body ausente e posicional excedente são erros; nenhum argumento reconhecido é
ignorado. Não produzir `Content::SetPage` e não reaproveitar a mecânica
rejeitada do antigo P335.

Registrar o mesmo `Func` no scope base sob `page`; como esse scope é clonado
para o módulo `std`, as superfícies global e `std.page` devem ter o mesmo
contrato observável. `#set page` e o warning vigente de `#show page` não mudam.

---

### `native_layout(func)`

**Assinatura**: `layout(func: Func) -> Content`

**Semântica**: `layout(size => content)` materializa `Content::Layout { func }` — a closure recebe o tamanho da região e produz o conteúdo a compor nesse espaço.

**Arquitectura**: o scope global expõe `Value::Func(native_layout)` como stub de despacho. A lógica real vive na intercepção de chamadas em eval (`eval/closures.rs::eval_func_call`, pelo `native_fn_addr` — mesmo padrão de `native_measure`). O stub nunca é invocado directamente; serve para existir no scope global e expor um endereço identificável. Invocação indirecta (fora do caminho interceptado) é erro ruidoso, não valor silencioso (ADR-0108).

**Paridade vanilla**: função nativa global `layout` (`typst-library/layout/layout.rs`, `#[func]`). As intercepções relacionadas `text.<campo>` (field access sobre `Func("text")`) e `here().<método>()` (`.page()`, `.position()`, `.page-numbering()` sobre `Value::Location`) vivem em eval e são governadas pelos prompts de eval, não por este módulo.

---

## P726 — `fill: none` / `stroke: none` em `block`/`box`/`grid`

Medido no vanilla (binário release, sonda `/tmp/p726-none.typ`):
`block`, `box`, `rect`, `grid` (e `table`, `table.cell`, `grid.cell` —
ver `structural.md` P726) aceitam `fill: none` e `stroke: none` **sem
erro** — `none` significa "sem preenchimento"/"sem traço", equivalente a
omitir o argumento. O cristalino rejeitava `Value::None` em todos estes
pontos (`"espera Color, recebeu none"`), bloqueando `canvas.typ:111,129`
de cetz (`block.with(breakable: false)` invocado com `fill: background,
stroke: stroke`, defaults `none` em `canvas.typ:25`).

### Semântica de implementação

Em cada ponto de extracção, `Some(Value::None)` junta-se ao braço
`None => None` (mesmo idiom já usado para `caption` em
`structural.rs:694`):

```rust
// fill — 3 pontos neste ficheiro (grid, block, box)
let fill = match args.named.get("fill") {
    Some(Value::Color(c)) => Some(*c),
    Some(Value::None) | None => None,
    Some(other) => return Err(...),   // tipos inválidos continuam erro
};

// stroke — 3 pontos neste ficheiro (grid, block, box)
let stroke = match args.named.get("stroke") {
    Some(Value::None) | None => None,
    Some(val) => Some(extract_stroke(val, "block", "stroke")?),
};
```

`extract_stroke` **não muda** — continua a rejeitar `none` com erro; os
call sites tratam `none` antes de o invocar. Blast radius zero nos
outros consumidores do helper (hlines/vlines — scope-out abaixo).

### Paridade mantida — `stroke(paint: none)` continua erro

Vanilla **rejeita** `stroke(paint: none)` (medido: "expected color,
gradient, tiling, or auto, found none", exit 1). O braço `stroke(paint)`
(`layout.rs:1416`) mantém o seu erro — paridade ao nível "é erro"
(ADR-0107; o texto diverge, mecânica aceite de propósito).

### Scope-out medido — hline/vline `stroke: none`

Vanilla aceita `table.hline(stroke: none)` etc. (linha não desenhada;
medido exit 0 com namespace correcto `table.hline`). O cristalino guarda
`Stroke` **não-opcional** em `GridHLineElem`/`TableHLineElem` (e vlines)
e o render desenha sempre a linha (`compiler/layout/grid.rs:668-689`) —
aceitar `none` aqui exige mudança de entidade para `Option<Stroke>` +
salto no render, de outra espécie que os braços `Option` deste passo, e
sem consumidor em cetz. Registado em `achados-adiados-cetz.md` para
passo futuro (não assumir zero-thickness como atalho — width 0 em PDF é
hairline, não "invisível").


## P1140.1-B — medição anterior à decisão (2026-08-23)

No vanilla ratificado `a51e02804`, `repr(type(PATH))` devolve `"type"`; no
cristalino anterior a esta mudança devolve `"function"`. O catálogo P1140 e
os probes públicos em `00_nucleo/diagnosticos/superficie-linguagem-p1140*`
medem a divergência para `decimal`, `duration`, `regex`, `selector`, `stroke`,
`tiling` e `version`. Os construtores atuais foram novamente executados após
a atomização P1140.1-A: catálogo byte-idêntico e 22 probes byte-idênticos ao
baseline estrutural. Esta é divergência de semântica pública da linguagem,
não de mecânica Rust (ADR-0107).

## P1140.1-B — `stroke` como tipo chamável

O binding global `stroke` passa a `Value::Type(Type::Stroke)`. A chamada
delega a `native_stroke` sem mudar defaults, validação ou `Value::Stroke`
produzido. Este lote não atomiza `layout.rs` nem altera render.
