# Inventário de cobertura: vanilla Typst vs cristalino

**Status**: `PROPOSTO` (inventário factual; actualização ad-hoc nos passos que materializam features)
**Data**: 2026-04-24
**Vanilla snapshot**: `lab/typst-original/` em commit `ba61529986e0a5a916cbf937c3c65117cd450683` (2026-04-24).
**Cristalino snapshot**: Passo 146; 57 ADRs; DEBT-1 + DEBT-52 fechados.

---

## Reformulação da pergunta

A pergunta original "em que paridade estamos?" está **mal-formulada** sem inventário prévio: medir paridade contra
a totalidade do vanilla produz percentagens irrelevantes (≈0%) quando a estratégia é "subset progressivo".

Reformulação adoptada na série paridade (Passo 148):

1. **(148, este passo)** "que features cristalino afirma cobrir?" — **inventário declarado**.
2. **(149+)** "do que cobre, X% bate observacionalmente com vanilla?" — **medição** com denominador honesto.

---

## Critério de classificação

Cada entrada tem uma das **5 classes**:

| Classe | Critério |
|--------|----------|
| `implementado` | Consumer activo + tests + sem ressalvas materiais. |
| `implementado⁺` | Consumer activo mas com **aproximação documentada por ADR-0054** (perfil observacional graded) ou ADR específica. Ex: `text.weight` faux-bold (P139); fonts sem subsetting (ADR-0027). |
| `parcial` | Captura/estrutura existe; consumer ausente, divergente, ou opcional. Ex: `Content::Heading` capturado mas alguns atributos não consumidos. |
| `ausente` | Não capturado. Verificação dupla: grep + sem ADR/Passo que materialize. |
| `scope-out` | ADR explícita declara fora do escopo. Ex: shaping rustybuzz (DEBT-53); font dict (gap 8 / ADR-0054bis condicional). |

**Referência canónica** por entrada: passo de materialização ou ADR; "—" se ausente.

---

## Tabela A — Vista user-facing

Features visíveis ao utilizador no Typst (markup, funções stdlib, `#set`/`#show`/`#let`, math, layout, model, visualize, introspection).

### A.1 — Markup syntactic

| Feature | Vanilla path | Cristalino estado | Referência | Nota |
|---------|--------------|--------------------|------------|------|
| Texto plano | sintaxe | `implementado` | Passo 30 | `Content::Text` |
| Espaço entre palavras | sintaxe | `implementado` | Passo 30 | `Content::Space` |
| Quebra de linha (`\`, `linebreak`) | text/linebreak.rs | `parcial` | math passos | `Content::Linebreak` em math; markup linebreak depende de quebra-linha greedy (Passo 144 hyphenation activa o caminho) |
| `*bold*` (sintaxe) | model/strong.rs | `implementado` | Passos 30, 99, 101 | desugars para `Content::Styled([Style::Bold(true)])` |
| `_italic_` | model/emph.rs | `implementado` | Passos 30, 99, 101 | análogo, `Style::Italic(true)` |
| `= heading`, `== heading`, ... | model/heading.rs | `implementado` | Passos 22, 99, 103 | níveis 1–6; show rules |
| `- list item` | model/list.rs | `parcial` | Passo 23 | captura via `Content::ListItem`; rendering básico; sem marcadores configuráveis |
| `+ enum`, `1. enum` | model/enum.rs | `parcial` | Passo 23 | captura via `Content::EnumItem`; numeração simples |
| `> blockquote` | model/quote.rs | `ausente` | — | sem `Content::Quote` |
| `/ term: definition` | model/terms.rs | `ausente` | — | sem `Content::Terms` |
| `$inline math$` | math/equation.rs | `implementado⁺` | Passos 34–46 | superscripts/subscripts/fracções; matrix/cases capturados; sem shaping completo (ADR-0054 perfil graded) |
| `$ display math $` | math/equation.rs | `implementado⁺` | idem | `block: true` em `Content::Equation` |
| `` `inline raw` ``, ```` ```block``` ```` | text/raw.rs | `implementado` | Passo 23 | `Content::Raw` com `lang` opcional |
| `<label>`, `@ref` | foundations/label.rs, model/reference.rs | `implementado⁺` | Passo 63 | `Content::Labelled`, `Content::Ref`; forward-refs limitadas (DEBT-10 fechada) |
| Smart quotes (`"foo"` → "foo") | text/smartquote.rs | `ausente` | — | sem `Content::SmartQuote` |
| Soft hyphen Unicode (`\u{00AD}`) | text/linebreak.rs | `ausente` | — | hyphenation usa apenas literal `-` (Passo 144) |

### A.2 — `#let`, `#set`, `#show`

| Feature | Vanilla | Cristalino estado | Referência | Nota |
|---------|---------|--------------------|------------|------|
| `#let var = ...` | foundations | `implementado` | Passo 30 | bindings + scoping |
| `#let f(x) = ...` (closure) | foundations/func.rs | `implementado` | Passo 31 (DEBT-2 parcial) | captura eager; closures nested |
| `#set text(...)` | foundations/styles.rs | `implementado` | Passo 102 (ADR-0040) | bold/italic/size/fill activos; weight/tracking/leading/lang/font activos pós-DEBT-52 |
| `#set par(...)` | model/par.rs | `parcial` | Passo 138 | `leading` capturado em text por conveniência; sem `par` propriamente |
| `#set heading(...)` | model/heading.rs | `parcial` | Passo 99 | `level` activo via `HeadingLevel`; `numbering` parcial |
| `#set page(...)` | layout/page.rs | `implementado` | Passo 81 | width/height/margin |
| `#set table/grid(...)` | layout/grid | `parcial` | Passos 82–83 | `grid` materializado; `table` ausente |
| `#set figure(...)` | model/figure.rs | `parcial` | Passo 75 | numbering pattern; kind hardcoded |
| `#show heading: ...` | foundations/styles.rs | `implementado` | Passo 103 (ADR-0041) | NodeKind selector |
| `#show strong/emph: ...` | idem | `implementado` | Passo 70, 101 | DEBT-19/20 encerradas |
| `#show <selector>: ...` (regex/where) | idem | `ausente` | — | requer `regex` em L1 (ADR-0054bis condicional) |
| `#import`, `#include` | foundations/module.rs | `implementado⁺` | Passos 71, 75 | filesystem real (L3); subset de features |

### A.3 — Text features

| Feature | Vanilla | Cristalino | Referência | Nota |
|---------|---------|------------|------------|------|
| `text.bold` | text/raw.rs | `implementado⁺` | Passos 30, 139 (ADR-0054) | faux-bold via stroke quando font Bold dedicada não disponível (ADR-0055bis candidata) |
| `text.italic` | idem | `implementado` | Passo 30 | Helvetica-Oblique |
| `text.size` | idem | `implementado` | Passo 30 | métricas + glyph emit |
| `text.fill` | idem | `implementado` | Passo 102 | `rg`/`RG` operators no PDF |
| `text.font` (string) | idem | `implementado` | Passo 140B | single-font dispatch via `FontBook::select` |
| `text.font` (array) | idem | `implementado` | Passo 141 | array fallback chain |
| `text.font` (multi-doc) | idem | `implementado` | Passo 146 | multi-font per document |
| `text.font` (dict) | idem | `scope-out` | ADR-0054bis condicional | gap 8 DEBT-52 (requer `regex` em L1) |
| `text.weight` (numérico/simbólico) | idem | `implementado⁺` | Passo 139 | faux-bold; sem font-file Bold dedicado (ADR-0055bis candidata) |
| `text.tracking` | idem | `implementado` | Passo 137 | `Tc` operator em PDF |
| `text.lang` (hyphenation) | text/lang.rs | `implementado⁺` | Passo 144 (ADR-0057) | crate `hypher`; shaping ausente |
| `text.lang` (shaping features) | idem | `scope-out` | DEBT-53 candidato | rustybuzz integration XL |
| `text.region` | idem | `ausente` | — | regional variants |
| `text.dir` (LTR/RTL) | idem | `ausente` | — | bidi via rustybuzz scope-out |
| `text.script` | text/shift.rs | `ausente` | — | super/sub script standalone |
| `smallcaps` | text/smallcaps.rs | `ausente` | — | |
| `upper` / `lower` (funções) | text/case.rs | `implementado` | stdlib | `native_upper`/`native_lower` |
| `replace` (string) | text/case.rs | `implementado` | stdlib | `native_replace` |
| `lorem` | text/lorem.rs | `ausente` | — | |
| `underline` / `strike` / `overline` | text/deco.rs | `ausente` | — | |
| `linebreak` (function) | text/linebreak.rs | `parcial` | math context | só em math |

### A.4 — Math

| Feature | Vanilla | Cristalino | Referência | Nota |
|---------|---------|------------|------------|------|
| Inline `$x$` | math/equation.rs | `implementado` | Passos 34–36 | `Content::Equation` |
| Display `$ x $` | idem | `implementado` | idem | block: true |
| `frac(a, b)` ou `a/b` | math/frac.rs | `implementado` | Passos 37–38 | linha + posicionamento vertical |
| `x^n`, `x_n`, attach | math/attach.rs | `implementado` | Passo 35 | super/subscript |
| `sqrt(x)` / `root(n, x)` | math/root.rs | `implementado⁺` | math passos | layout aproximado |
| `mat(...)` matriz | math/matrix.rs | `implementado⁺` | Passo 79 | `Content::MathMatrix`; tolerância visual |
| `cases` | math/matrix.rs | `implementado⁺` | Passo 79 | `Content::MathCases` |
| `lr(...)` | math/lr.rs | `implementado⁺` | Passo 76 | delimitadores extensíveis |
| `accent(c, mark)` | math/accent.rs | `parcial` | math passos | heuristic; sem todas as variantes Unicode |
| `cancel`, `underover`, `op` | math/{cancel,underover,op}.rs | `parcial` | math passos | parcial |
| Símbolos gregos (alpha, beta, ...) | foundations/symbol.rs | `implementado⁺` | Passo 50 | mapping Unicode literal; sem `Symbol` Value |
| `align` em equação (`& =`) | math/equation.rs | `implementado` | math passos | `MathAlignPoint` |
| `equation.numbering` | model/numbering.rs | `parcial` | Passo 36 | só block equations |

### A.5 — Layout

**Reclassificação P156B (2026-04-25)**: diagnóstico Layout
revelou divergências empíricas vs declaração; actualização
de 4 entradas (`pad`, `pagebreak`, `grid`, `place`) e adição
de 2 entradas que não estavam na lista (`h`/`v` spacing
primitives e `skew`). Detalhe em
[`diagnostico-layout-passo-156b.md`](diagnostico-layout-passo-156b.md).

| Feature | Vanilla | Cristalino | Referência | Nota |
|---------|---------|------------|------------|------|
| `pad(...)` | layout/pad.rs | `implementado` ⁶ | Passo 156C (ADR-0061 Fase 1) | `Content::Pad { body, padding: Sides<Length> }` + stdlib `#pad(body, left:?, right:?, top:?, bottom:?, x:?, y:?, rest:?)`; `right` scope-out em layout (perfil ADR-0054 graded); padding negativo rejeitado |
| `align(alignment, body)` | layout/align.rs | `implementado` | Passos 84.5–84.6 (DEBT-36, 37) | `Align2D`; `Place` com scope |
| `place(alignment, ..., body)` | layout/place.rs | `parcial` ⁵ | Passo 84.6 | reclassificado em P156B (era `implementado`); sem `float`, `clearance`; divergência `PlaceScope::Parent` |
| `box(...)` | layout/container.rs | `implementado` ¹⁵ | Passo 156H (ADR-0061 Fase 2 sub-passo 2) | `Content::Boxed { body, width, height, inset, baseline }` + stdlib `#box(body, width: ?, height: ?, inset: ?, baseline: ?)`; container inline (não força flush_line); 6 atributos vanilla scope-out (outset/fill/stroke/radius/clip/stroke-overhang); width/height/baseline armazenados mas semantic real adiada |
| `block(...)` | layout/container.rs | `implementado` ¹³ | Passo 156G (ADR-0061 Fase 2 sub-passo 1) | `Content::Block { body, width, height, inset, breakable }` + stdlib `#block(body, width: ?, height: ?, inset: ?, breakable: true)`; subset Fase 1 per ADR-0054 graded; 9 atributos vanilla scope-out (outset/fill/stroke/radius/clip/spacing/above/below/sticky) |
| `columns(n)` | layout/columns.rs | `ausente` | — | Fase 3 ADR-0061; **DEBT-56** (column flow L+) |
| `grid(columns, ...)` | layout/grid | `parcial` ⁵ | Passos 82–84.6 | reclassificado em P156B (era `implementado⁺`); sem `gutter`, `align`, `stroke`, `fill`, `inset`, `header`, `footer`, `colspan`/`rowspan`. DEBT-34d/e abertos |
| `stack(spacing, ...)` | layout/stack.rs | `implementado` ¹⁷ | Passo 156I (ADR-0061 Fase 2 sub-passo 3; **último Fase 2; atinge target 72%**) | `Content::Stack { children: Arc<[Content]>, dir: Dir, spacing: Option<Length> }` + stdlib `#stack(dir: ?, spacing: ?, ..children)`; tipo `Dir` novo (LTR/RTL/TTB/BTT); 4 direcções implementadas; spacing real entre children |
| `pagebreak()` (manual) | layout/page.rs | `implementado` ¹⁰ | Passo 156E (ADR-0061 Fase 1 sub-passo 3) | `Content::Pagebreak { weak, to: Option<Parity> }` + stdlib `#pagebreak(weak: false, to: ?)`; `to:"even"`/`"odd"` insere página vazia se necessário; `weak` collapse defere; tipo `Parity` novo em `entities/parity.rs` |
| `colbreak()` | layout/columns.rs | `ausente` | — | depende de columns; Fase 3 ADR-0061; DEBT-56 |
| `rotate(angle, body)` | layout/transform.rs | `implementado` | Passo 78 | `Content::Transform` |
| `scale(amount, body)` | idem | `implementado` | Passo 78 | |
| `move(dx, dy, body)` | idem | `implementado` | stdlib `native_move` | |
| `hide(body)` | layout/hide.rs | `implementado` ⁶ | Passo 156C (ADR-0061 Fase 1) | `Content::Hide { body }` + stdlib `#hide(body)`; calcula dimensões mas emite zero items (per ADR-0054 graded) |
| `repeat(body)` | layout/repeat.rs | `implementado` ¹⁹ | Passo 156J (ADR-0061 Fase 3 sub-passo 1; **primeira Fase 3**) | `Content::Repeat { body, gap: Option<Length>, justify: bool }` + stdlib `#repeat(body, gap: ?, justify: true)`; default `justify == true` (paridade vanilla); algoritmo dinâmico de quantidade-para-encher diferido per ADR-0054 graded (Layouter executa single-render — paridade estrutural suficiente para counters/labels descenderem) |
| `pad`, `corners`, `sides` (inset modeling) | layout/{pad,corners,sides}.rs | `ausente` | — | duplica `pad()` linha; refino PageConfig é Fase 3 ADR-0061 |
| `measure(body)` | layout/measure.rs | `parcial` | helper privado | helper `measure_content` em `01_core/src/rules/layout/helpers.rs`; sem stdlib exposta; depende de Introspection (ADR-0017 adiada) |
| `h(amount)` / `v(amount)` ⁵ | layout/spacing.rs | `implementado` ⁸ | Passo 156D (ADR-0061 Fase 1 sub-passo 2) | `Content::HSpace` + `Content::VSpace` com `amount: Length, weak: bool`; stdlib `#h(amount, weak: false)` + `#v(...)`; `weak` armazenado mas collapse defere; amount `Fraction` scope-out (refino futuro per ADR-0061 §6.3) |
| `skew(ax, ay, body)` ⁵ | layout/transform.rs | `implementado` ¹² | Passo 156F (ADR-0061 Fase 1 sub-passo 4) | `TransformMatrix::skew(ax_rad, ay_rad)` novo + `native_skew` reusa `Content::Transform { matrix }` existente desde P78; **sem refactor** (matriz cm já unificava); ângulos próximos de ±π/2 rejeitados; `origin` scope-out |

⁵ — Reclassificação ou adição P156B. Ver
[`diagnostico-layout-passo-156b.md`](diagnostico-layout-passo-156b.md)
§2.7 para detalhe.

### A.6 — Model (structural)

| Feature | Vanilla | Cristalino | Referência | Nota |
|---------|---------|------------|------------|------|
| `heading(level, body)` | model/heading.rs | `implementado` | Passos 22, 99, 103 | construtor + show rules |
| `figure(body, caption, ...)` | model/figure.rs | `implementado⁺` | Passos 75, ADR-0041 | numbering por kind; counters |
| `caption(...)` | model/figure.rs | `parcial` | dentro de figure | sem element dedicado |
| `outline()` | model/outline.rs | `implementado` | Passos 65–66 | TOC via 2-pass introspection |
| `table(columns, ...)` | model/table.rs | `ausente` | — | escopo grande; DEBT-34 family parcial em grid |
| `list(items)` (function form) | model/list.rs | `parcial` | sintaxe parcial | sem function form completa |
| `enum(items)` | model/enum.rs | `parcial` | idem | |
| `terms(...)` | model/terms.rs | `implementado` | Passo 154B | `Content::Terms` + `Content::TermItem`; named args via `#terms(key: [desc])`; sem atributos vanilla (tight/separator/indent/hanging-indent) — extensíveis sem breaking change |
| `quote(...)` | model/quote.rs | `implementado` | Passo 155 | `Content::Quote` com 4 attrs; smart-quotes lang-aware (6 idiomas + default ASCII) via `rules/lang/quotes.rs`; markup `"..."` produz aspas localizadas via alternância open/close (não pareadas como bloco) |
| `cite(key)` | model/cite.rs | `ausente` | — | requer bibliography |
| `bibliography(path)` | model/bibliography.rs | `ausente` | — | escopo XL: CSL parsing |
| `link(dest, body)` | model/link.rs | `parcial` | Passo 23 | `Content::Link` capturado; sem render visual |
| `footnote(body)` | model/footnote.rs | `ausente` | — | |
| `ref(target)` | model/reference.rs | `implementado⁺` | Passos 63–66 | `Content::Ref` com forward-resolve |
| `numbering(pattern, ...)` | model/numbering.rs | `implementado⁺` | Passos 75, 99 | numéricas/letras/romanas |
| `document(...)` | model/document.rs | `ausente` | — | document metadata wrapper |
| `divider` | model/divider.rs | `implementado` | Passo 154B | `Content::Divider` singleton; layouter emite `FrameItem::Shape::Line` 0.5pt |
| `asset`, `title` | model/{asset,title}.rs | `ausente` | — | Fase 3 ADR-0060 |
| `par` (paragraph element) | model/par.rs | `parcial` | Passo 138 | `leading` activo; sem `Content::Par` |

### A.7 — Visualize

| Feature | Vanilla | Cristalino | Referência | Nota |
|---------|---------|------------|------------|------|
| `rect(width, height, fill, ...)` | visualize/shape.rs | `implementado` | Passos 78–79 | `Content::Shape{ Rect }` |
| `ellipse(...)` / `circle(...)` | idem | `implementado` | idem | `Content::Shape{ Ellipse }` |
| `line(start, end, ...)` | visualize/line.rs | `implementado` | idem | `Content::Shape{ Line }` |
| `polygon(points)` | visualize/polygon.rs | `implementado` | stdlib | |
| `path(...)` / `curve(...)` | visualize/curve.rs | `implementado⁺` | Passo 78 | `ShapeKind::Path`; sem cubic optimisation completa (DEBT-33) |
| `image(path, ...)` | visualize/image | `implementado` | Passos 72–74 | PNG (alpha + opaque) + JPEG; DEBT-26/27/28/29 fechados |
| `square(...)` | visualize/shape.rs | `ausente` | — | |
| `rgb(...)`, `luma(...)` | visualize/paint.rs | `implementado` | stdlib | `native_rgb`, `native_luma` |
| `cmyk(...)`, `oklab(...)`, etc. | visualize/color.rs | `ausente` | — | space-specific constructors |
| `gradient(...)` | visualize/gradient.rs | `ausente` | — | |
| `tiling(...)` | visualize/tiling.rs | `ausente` | — | |
| `stroke(...)` (object) | visualize/stroke.rs | `parcial` | shape passos | `Stroke` em `FrameItem::Shape`; sem todas as variantes (paint, dash, …) |

### A.8 — Foundations (stdlib functions)

| Feature | Vanilla | Cristalino | Referência | Nota |
|---------|---------|------------|------------|------|
| `type(value)` | foundations/ty.rs | `implementado` | stdlib `native_type` | |
| `len(value)` | foundations/array.rs etc. | `implementado` | stdlib `native_len` | |
| `range(...)` | foundations/array.rs | `implementado` | stdlib `native_range` | |
| `int(...)`, `float(...)`, `str(...)` | foundations/{int,float,str}.rs | `implementado` | stdlib | |
| `assert(cond, msg)` | foundations/calc.rs | `implementado` | stdlib `native_assert` | |
| `calc.*` (math functions) | foundations/calc.rs | `implementado⁺` | stdlib `make_calc_module` | subset (sin/cos/sqrt/min/max/abs/...) |
| `array.{push, pop, ...}` | foundations/array.rs | `parcial` | passos | algumas methods |
| `dict.{at, keys, values, ...}` | foundations/dict.rs | `parcial` | passos | algumas methods |
| `str.{contains, replace, ...}` | foundations/str.rs | `parcial` | passos | algumas methods |
| `eval(string)` | foundations | `ausente` | — | |
| `repr(value)` | foundations/repr.rs | `parcial` | sub-set | |
| `panic(msg)` | foundations | `ausente` | — | |
| `if/else`, `while`, `for`, `break`, `continue` | foundations/ops.rs | `implementado` | Passo 30 + flow | control flow |
| `import math: ...`, `from math: ...` | foundations | `parcial` | Passos 71, 75 | std imports limitados |

### A.9 — Introspection

| Feature | Vanilla | Cristalino | Referência | Nota |
|---------|---------|------------|------------|------|
| `counter(key)` | introspection/counter.rs | `implementado` | Passos 60–62 | step/update; counter.display() |
| `state(key, ...)` | introspection/state.rs | `ausente` | — | |
| `here()` / `locate()` | introspection/{here,locate}.rs | `ausente` | — | Passo 17 ADR-0017 adiou; runtime não está pronto |
| `query(...)` | introspection/query.rs | `ausente` | — | |
| `metadata(value)` | introspection/metadata.rs | `ausente` | — | |
| `position(target)` | introspection/position.rs | `ausente` | — | |

---

## Tabela B — Vista arquitectural

Tipos do Rust em `01_core/src/entities/` versus `lab/typst-original/crates/typst-library/src/foundations/value.rs` etc.

### B.1 — `Value` enum (vanilla 30 variants; cristalino 18 variants)

| Variant | Vanilla path | Cristalino estado | Referência | Nota |
|---------|--------------|--------------------|------------|------|
| `None` | foundations/none.rs | `implementado` | Passo 13 | |
| `Auto` | foundations/auto.rs | `implementado` | Passo 25 (ADR-0028→0029) | |
| `Bool` | foundations/bool.rs | `implementado` | Passo 13 | |
| `Int` | foundations/int.rs | `implementado` | Passo 13 (ADR-0025 Int==Float) | |
| `Float` | foundations/float.rs | `implementado` | Passo 13 (ADR-0025) | |
| `Length` | layout/length.rs | `implementado` | Passos 25, 127 | tracking |
| `Angle` | layout/angle.rs | `implementado` | Passo 25 | rotação |
| `Ratio` | layout/ratio.rs | `implementado` | Passo 25 | |
| `Relative` (Rel<Length>) | layout/rel.rs | `ausente` | — | composto abs+ratio |
| `Fraction` (Fr) | layout/fr.rs | `implementado` | Passo 80 | f64 directo (não tipo dedicado) |
| `Color` | visualize/color.rs | `implementado` | Passo 102 | `entities/layout_types::Color` |
| `Gradient` | visualize/gradient.rs | `ausente` | — | |
| `Tiling` | visualize/tiling.rs | `ausente` | — | |
| `Symbol` | foundations/symbol.rs | `ausente` | — | mapping literal Unicode em vez de tipo dedicado |
| `Version` | foundations/version.rs | `ausente` | — | |
| `Str` | foundations/str.rs | `implementado` | Passo 13, 24 (EcoString ADR-0024) | |
| `Bytes` | foundations/bytes.rs | `ausente` | — | |
| `Label` | foundations/label.rs | `parcial` | Passos 63–66 | `Label` tipo separado de Value |
| `Datetime` | foundations/datetime.rs | `implementado` | Passo 21 (ADR-0021) | |
| `Decimal` | foundations/decimal.rs | `ausente` | — | |
| `Duration` | foundations/duration.rs | `ausente` | — | |
| `Content` | foundations/content/ | `implementado` | Passos 18, ADR-0026 + 0026-R1 | enum vs vtable |
| `Styles` | foundations/styles.rs | `parcial` | Passos 99 | `Styles` tipo separado; não Value variant |
| `Array` | foundations/array.rs | `implementado` | Passo 15 | |
| `Dict` | foundations/dict.rs | `implementado` | Passo 23 (ADR-0023 indexmap) | |
| `Func` | foundations/func.rs | `implementado` | Passo 16 | Arc<FuncRepr> |
| `Args` | foundations/args.rs | `implementado⁺` | Passo 16; **ADR-0059** (P149) | divergência intencional: `Args` é struct separada em `entities/args.rs`, passada como `&Args` às nativas. Não-variant de `Value`. |
| `Type` | foundations/ty.rs | `implementado⁺` | Passo 13-14; **ADR-0058** (P149) | divergência intencional: `type(x)` devolve `Value::Str(type_name)` em vez de `Value::Type(Type)` rico. `type(x) == "int"` funciona; `type(x) == int` não. |
| `Module` | foundations/module.rs | `implementado` | Passo 17 | Arc<ModuleInner> |
| `Dyn` (Dynamic) | foundations/value.rs | `ausente` | — | escape hatch raramente usado |
| `Align` (extra cristalino) | — | `implementado` | Passo 84.5 (DEBT-36) | divergência intencional: `Value::Align` simplifica resolução |

### B.2 — `Content` (vanilla = vtable + elem!; cristalino = enum)

Nota: ADR-0026 + 0026-R1 declaram **divergência intencional**. Cristalino usa enum fechado com Arc<[T]> em Sequence.

| Cristalino variant | Equivalente vanilla | Estado | Referência | Nota |
|--------------------|---------------------|--------|------------|------|
| `Empty` | (n/a — vanilla usa Sequence vazia) | `implementado` | Passo 22 | |
| `Text(EcoString, TextStyle)` | TextElem | `implementado` | Passo 30 | |
| `Space` | SpaceElem | `implementado` | Passo 30 | |
| `Sequence(Arc<[Content]>)` | SequenceElem | `implementado` | Passos 22, ADR-0026-R1 | |
| `Heading {level, body}` | HeadingElem | `implementado⁺` | Passos 22, 99, 103 | nem todos os attrs |
| `Raw {text, lang, block}` | RawElem | `implementado` | Passo 23 | |
| `ListItem(Box<Content>)` | ListElem/ItemElem | `parcial` | Passo 23 | rendering básico |
| `EnumItem {number, body}` | EnumElem/ItemElem | `parcial` | Passo 23 | |
| `Link {url, body}` | LinkElem | `parcial` | Passo 23 | sem render visual |
| `Equation {body, block}` | EquationElem | `implementado⁺` | Passo 34 | |
| `MathSequence(Arc<[Content]>)` | (interno math) | `implementado` | Passo 34 | |
| `MathIdent(EcoString)` | MathIdent | `implementado` | Passo 34 | |
| `MathText(EcoString)` | MathText | `implementado` | Passo 34 | |
| `MathFrac {num, den}` | FracElem | `implementado` | Passo 37 | |
| `MathAttach {...}` | AttachElem | `implementado` | Passo 35 | super/subscript |
| `MathRoot {...}` | RootElem | `implementado⁺` | math passos | |
| `MathDelimited {...}` | LrElem | `implementado⁺` | Passo 76 | |
| `MathAlignPoint` | AlignPointElem | `implementado` | math passos | `&` em equação |
| `Linebreak` | LinebreakElem | `parcial` | math context | só em math |
| `MathMatrix {...}` | MatElem | `implementado⁺` | Passo 79 | |
| `MathCases {...}` | CasesElem | `implementado⁺` | Passo 79 | |
| `Labelled {...}` | (label pos-process) | `implementado` | Passo 63 | |
| `Ref {...}` | RefElem | `implementado⁺` | Passos 63–66 | |
| `SetHeadingNumbering` | (set rule) | `implementado` | Passo 99 | |
| `CounterDisplay {...}` | CounterDisplayElem | `implementado` | Passo 60 | |
| `CounterUpdate {...}` | CounterUpdateElem | `implementado` | Passo 60 | |
| `Outline` | OutlineElem | `implementado` | Passos 65–66 | |
| `Figure {...}` | FigureElem | `implementado⁺` | Passos 75, DEBT-14/15 | |
| `SetFigureNumbering {...}` | (set rule) | `implementado` | Passo 75 | |
| `Image {...}` | ImageElem | `implementado` | Passos 72–74 | |
| `Shape {...}` | RectElem/CircleElem/LineElem/PathElem | `implementado` | Passos 78–79 | unificado em `Shape` |
| `Transform {...}` | RotateElem/ScaleElem | `implementado` | Passo 78 | |
| `Grid {...}` | GridElem | `implementado⁺` | Passos 82–84.6 | DEBT-34d/e abertos |
| `SetPage {...}` | (set rule) | `implementado` | Passo 81 | |
| `Align {...}` | AlignElem | `implementado` | Passo 84.5 | |
| `Place {...}` | PlaceElem | `implementado` | Passo 84.6 | |
| `Styled(Box<Content>, Styles)` | (vtable + show rules) | `implementado` | Passos 99–101 (ADR-0038/0039) | divergência ADR-0026 |
| `Divider` | DividerElem | `implementado` | Passo 154B | singleton; layouter emite linha 0.5pt |
| `Terms {items}` | TermsElem | `implementado` | Passo 154B | sem atributos vanilla (tight/sep/indent) |
| `TermItem {term, description}` | TermItemElem | `implementado` | Passo 154B | par item; standalone permitido |
| `Quote {body, attribution, block, quotes}` | QuoteElem | `implementado` | Passo 155 | 4 atributos materializados; smart-quotes lang-aware via `rules/lang/quotes.rs` |
| **Vanilla-only (ausentes)**: BibliographyElem, CiteElem, FootnoteElem, TableElem, ColumnsElem, BoxElem, BlockElem, StackElem, HideElem, RepeatElem, PadElem, MoveElem (function só), GradientElem, TilingElem, StrokeElem (object form), … | — | `ausente` (cada) | — | escopo crescente |

### B.3 — `Style` enum

| Variant | Vanilla | Cristalino | Referência |
|---------|---------|------------|------------|
| `Bold(bool)` | (vtable per style) | `implementado` | Passo 99 (ADR-0038) |
| `Italic(bool)` | idem | `implementado` | idem |
| `Size(Pt)` | idem | `implementado` | idem |
| `Fill(Color)` | idem | `implementado` | Passos 99, 102 |
| `HeadingLevel(u8)` | idem | `implementado` | Passos 99, 103 |

Nota: `Style` é divergência intencional (ADR-0038); vanilla usa vtable polimórfica.

### B.4 — `StyleDelta` fields (10 fields per relatório 142 §3)

| Field | Cristalino | Referência |
|-------|------------|------------|
| `bold` | `implementado` | Passo 30 |
| `italic` | `implementado` | Passo 30 |
| `size` | `implementado` | Passo 30 |
| `fill` | `implementado` | Passo 102 |
| `heading_level` | `implementado` | Passo 99 |
| `weight` | `implementado⁺` | Passo 139 (faux-bold; ADR-0054) |
| `tracking` | `implementado` | Passo 137 |
| `leading` | `implementado` | Passo 138 |
| `lang` | `implementado⁺` | Passo 144 (hyphenation; shaping ausente) |
| `font` | `implementado` | Passos 140B, 141, 146 |

### B.5 — `FrameItem` enum (cristalino 6 variants)

| Variant | Vanilla equivalente | Cristalino | Referência |
|---------|---------------------|------------|------------|
| `Text` | (vtable: TextItem) | `implementado` | Passo 19 |
| `Line` | LineItem | `implementado` | math passos |
| `Glyph` | (interno) | `implementado` | Passo 50 (math variants) |
| `Image` | ImageItem | `implementado` | Passos 72–74 |
| `Shape` | ShapeItem | `implementado` | Passos 78–79 |
| `Group` | GroupItem | `implementado` | Passo 84.6 (transform) |

---

## Tabela C — Vista cruzada (parciais e ausentes)

Para cada feature `parcial` ou `ausente` da Tabela A, lista de tipos arquitecturais bloqueantes e ADR/DEBT canónica.

| Feature user-facing | Bloqueantes arquitecturais | ADR/DEBT/Próximo passo |
|---------------------|----------------------------|------------------------|
| `text.font` dict | `Value::Regex` ausente; `Covers` em `FontList` deferido | gap 8 DEBT-52; ADR-0054bis condicional |
| `text.weight` Bold dedicada | `FontVariant` selection variant-aware ausente | ADR-0055bis candidata |
| `text.lang` shaping (bidi/kern/lig) | rustybuzz integration ausente | DEBT-53 candidato XL |
| `text.region` / `text.script` | `Region`, `Script` types ausentes | escopo XL com rustybuzz |
| `text.dir` (LTR/RTL) | bidi shaping ausente | DEBT-53 |
| `smartquote` | `Content::SmartQuote` ausente | escopo S |
| `smallcaps` | `Content::SmallCaps`; OpenType features | DEBT-53 (shaping) |
| `lorem` | sem stdlib helper | escopo S |
| `underline` / `strike` / `overline` | `Content::Underline` etc. ausentes | escopo S |
| Soft hyphen (`\u{00AD}`) | hyphenation espera literal `-` (Passo 144) | passo dedicado futuro |
| `quote(...)` | `Content::Quote` ausente | escopo S |
| `terms(...)` | `Content::Terms` ausente | escopo S |
| `footnote(body)` | `Content::Footnote` + locate runtime ausente | escopo M (locate ADR-0017 adiada) |
| `cite(key)` | `Content::Cite` + bibliography parser ausente | escopo XL |
| `bibliography(path)` | CSL parser; `Content::Bibliography`; `loading` module | escopo XL |
| `link` render visual | `Content::Link` capturado mas sem render | escopo S |
| `state(key, ...)` | introspection runtime ausente | depende de ADR-0017 (adiada) |
| `here()` / `locate()` / `query()` | introspection runtime ausente | depende de ADR-0017 (adiada) |
| `metadata(value)` | introspection runtime ausente | idem |
| `eval(string)` | runtime de re-eval ausente | escopo M |
| `panic(msg)` | sem helper stdlib | escopo XS |
| `repr(value)` (completo) | `repr` de cada Value variant parcial | passos passados materializaram subset |
| `box(...)` / `block(...)` | `Content::Box`/`Block` ausentes | escopo M |
| `columns(n)` | `Content::Columns` + multi-col layout ausentes | escopo M |
| `stack(...)` | `Content::Stack` ausente | escopo M |
| `colbreak` | sem layout column-aware | depende de columns |
| `pagebreak` (explicit) | implícito via layout overflow; sem `Content::PageBreak` | escopo XS |
| ~~`repeat(body)`~~ | resolvido P156J (`Content::Repeat`) | — |
| `hide(body)` | `Content::Hide` ausente | escopo XS |
| `measure(body)` | introspection runtime ausente | depende de ADR-0017 |
| `square(...)` | `ShapeKind::Square` ausente; trivially derivable de Rect com width=height | escopo XS |
| `gradient(...)` | `Value::Gradient` ausente; render gradient em PDF | escopo M |
| `tiling(...)` | `Value::Tiling` ausente | escopo M |
| `cmyk` / `oklab` cores | Color space não-RGB ausente | escopo S |
| `table(columns, ...)` | `Content::Table`; cell layout; rowspan/colspan (DEBT-34d/e) | escopo M; DEBT-34d/e abertos |
| `Value::Bytes` | foundations/bytes.rs ausente | escopo XS |
| `Value::Decimal` | precision arithmetic ausente | escopo S |
| `Value::Duration` | `Duration` type ausente | escopo XS |
| `Value::Symbol` | mapping literal usado em vez de tipo | divergência aceitável; escopo S se reverso |
| `Value::Version` | sem semver type | escopo XS |
| `Value::Args` | tipo `Args` separado, não em Value | divergência cristalino; refactor escopo S |
| `Value::Type` | string-based em cristalino vs `Type` em vanilla | divergência ADR-0025; refactor escopo S |
| `accent` math (variantes Unicode completas) | mapping char→glyph parcial | passo dedicado math |
| `cancel` / `underover` / `op` | layout aproximado | passos dedicados math |
| Show selectors (regex/where) | `regex` em L1 ausente | ADR-0054bis condicional |
| `#import "@preview/..."` | package resolver completo ausente | escopo M |

---

## Resumo agregado

### Tabela A — User-facing (contagens)

Categorias e contagens são aproximadas (~1 por linha listada acima):

| Categoria | `implementado` | `implementado⁺` | `parcial` | `ausente` | `scope-out` | Total |
|-----------|----------------|-----------------|-----------|-----------|-------------|-------|
| Markup syntactic | 8 | 3 | 3 | 4 | 0 | 18 |
| `#let`/`#set`/`#show`/import | 7 | 1 | 4 | 1 | 0 | 13 |
| Text features | 7 | 5 | 1 | 8 | 2 | 23 |
| Math | 6 | 6 | 1 | 0 | 0 | 13 |
| Layout ⁵ ⁶ ⁸ ¹⁰ ¹² ¹³ ¹⁵ ¹⁷ ¹⁹ | 14 | 0 | 3 | 1 | 0 | 18 |
| Model (structural) ¹ ² ³ | 6 | 4 | 5 | 7 | 0 | 22 |
| Visualize | 6 | 1 | 1 | 5 | 0 | 13 |
| Foundations stdlib | 9 | 1 | 4 | 1 | 0 | 15 |
| Introspection | 1 | 0 | 0 | 5 | 0 | 6 |
| **Total user-facing** ⁵ ⁶ ⁸ ¹⁰ ¹² ¹³ ¹⁵ ¹⁷ ¹⁹ | **64** | **21** | **22** | **32** | **2** | **141** |

¹ — Ajuste P154A (diagnóstico Model): cobertura empírica
revisada (era 4/4/5/8/0=21; passa a 3/4/5/10/0=22 após
contagem por sub-elementos: caption isolado de figure,
divider/asset/title individualizados). `Heading` reclassificado
de `implementado` para `implementado⁺` (tem ressalvas em
`numbering` e `outlined`). Ver
[`diagnostico-model-passo-154a.md`](diagnostico-model-passo-154a.md).

² — Ajuste P154B (materialização Fase 1 sub-passo 1):
`terms` e `divider` transitam `ausente → implementado`.
Contagem Model: 3/4/5/10/0=22 → 5/4/5/8/0=22.
Cobertura Model: (3+4)/22=32% → (5+4)/22=**41%**.

³ — Ajuste P155 (materialização Fase 1 sub-passo 2; **fecha
Fase 1**): `quote` transita `ausente → implementado`. Contagem
Model: 5/4/5/8/0=22 → **6/4/5/7/0=22**. Cobertura Model:
(5+4)/22=41% → (6+4)/22=**45%**. ADR-0060 transita
`PROPOSTO → IMPLEMENTADO`.

⁵ — Ajuste P156B (diagnóstico Layout Fase X; **oitava
aplicação do padrão diagnóstico-primeiro**): cobertura Layout
recalculada empiricamente. Reclassificações de 4 entradas
(`pad` parcial→ausente; `pagebreak` parcial→ausente para
manual; `grid` impl⁺→parcial; `place` implementado→parcial)
e adição de 2 entradas (`h`/`v` spacing primitives e `skew`,
não estavam em A.5). Contagem Layout: 6/0/2/8/0=16 →
**4/0/3/11/0=18**. Cobertura Layout (impl + impl⁺):
6/16=38% → **4/18=22%** (recálculo para baixo, padrão análogo
a Model 154A). Contagem user-facing total ajustada:
56/21/21/39/2=139 → **54/21/22/42/2=141** (aproximação:
−2 implementado, +1 parcial, +3 ausente, +2 entradas
agregadas).

⁶ — Ajuste P156C (materialização Layout Fase 1 sub-passo 1):
`pad` e `hide` transitam `ausente → implementado` (primeira
aplicação concreta de ADR-0061). Contagem Layout: 4/0/3/11/0=18
→ **6/0/3/9/0=18**. Cobertura Layout (impl + impl⁺):
4/18=22% → **6/18=33%**. Contagem user-facing total ajustada:
54/21/22/42/2=141 → **56/21/22/40/2=141** (+2 implementado,
−2 ausente). ADR-0061 mantém-se `PROPOSTO` (anotação
cumulativa após Fase 1 completa, per decisão humana).

⁸ — Ajuste P156D (materialização Layout Fase 1 sub-passo 2):
`h` e `v` spacing primitives transitam `ausente → implementado`
(segunda aplicação consecutiva de ADR-0061). Contagem Layout:
6/0/3/9/0=18 → **8/0/3/7/0=18**. Cobertura Layout (impl +
impl⁺): 6/18=33% → **8/18=44%**. Contagem user-facing total
ajustada: 56/21/22/40/2=141 → **58/21/22/38/2=141** (+2
implementado, −2 ausente). ADR-0061 continua `PROPOSTO`
(anotação cumulativa após Fase 1 completa).

¹⁰ — Ajuste P156E (materialização Layout Fase 1 sub-passo 3):
`pagebreak` manual transita `ausente → implementado` (terceira
aplicação consecutiva de ADR-0061; **50% Layout atingido**).
Contagem Layout: 8/0/3/7/0=18 → **9/0/3/6/0=18**. Cobertura
Layout (impl + impl⁺): 8/18=44% → **9/18=50%**. Contagem
user-facing total ajustada: 58/21/22/38/2=141 →
**59/21/22/37/2=141** (+1 implementado, −1 ausente). Tipo
`Parity` novo em `entities/parity.rs` (infraestrutura).
ADR-0061 continua `PROPOSTO`.

¹² — Ajuste P156F (materialização Layout Fase 1 sub-passo 4):
`skew` transita `ausente → implementado` (quarta aplicação
consecutiva de ADR-0061). **Decisão de divergência da spec**:
inventário em 156F.1 revelou que `Content::Transform { body,
matrix: TransformMatrix }` já era unificado via matriz cm
desde P78 (move/rotate/scale partilham mesma struct). Spec
propunha refactor com `TransformKind` enum, mas seria
redundante. P156F adiciona apenas método estático
`TransformMatrix::skew(ax, ay)` + `native_skew` em
`stdlib/transforms.rs`; **zero refactor**, **zero risco de
regressão** (puramente aditivo). Contagem Layout:
9/0/3/6/0=18 → **10/0/3/5/0=18**. Cobertura Layout (impl +
impl⁺): 9/18=50% → **10/18=56%**. Contagem user-facing total
ajustada: 59/21/22/37/2=141 → **60/21/22/36/2=141** (+1
implementado, −1 ausente). Tabela B Content variants
inalterado (48; sem nova variant). ADR-0061 continua
`PROPOSTO`.

¹⁵ — Ajuste P156H (materialização Layout Fase 2 sub-passo 2):
`box` transita `ausente → implementado` (sexta aplicação
consecutiva de ADR-0061; segunda Fase 2). **Decisão
arquitectural reusada de P156G** (variant rico) sem nova
decisão. `Content::Boxed { body, width, height, inset,
baseline }` adicionado; stdlib `#box(...)`. Distinção
material face a Block: posicionamento **inline** (não força
flush_line); atributo único `baseline` (vs `breakable`).
Naming `Boxed` em Rust evita conflito com `std::boxed::Box`.
6 atributos vanilla scope-out (outset, fill, stroke, radius,
clip, stroke-overhang). Contagem Layout: 11/0/3/4/0=18 →
**12/0/3/3/0=18**. Cobertura Layout: 11/18=61% → **12/18=67%**.
Total user-facing: 61/21/22/35/2=141 → **62/21/22/34/2=141**.
Tabela B Content: **49 → 50**. ADR-0061 mantém-se `PROPOSTO`.

¹⁷ — Ajuste P156I (materialização Layout Fase 2 sub-passo 3;
**último Fase 2; atinge target 72%**): `stack` transita
`ausente → implementado` (sétima aplicação consecutiva de
ADR-0061; **última Fase 2 — fechamento de série P156C-I**).
Decisão arquitectural reusada de P156G/H (variant rico) com
adaptação para `Arc<[Content]>` (clone O(1) per ADR-0026
revisão). `Content::Stack { children: Arc<[Content]>, dir:
Dir, spacing: Option<Length> }` adicionado. **Tipo `Dir`
novo** em `entities/dir.rs` (4 direcções LTR/RTL/TTB/BTT).
Spacing implementado real (trivial via cursor advance per
inventário 156I.1). Sem atributos vanilla scope-out (vanilla
stack tem só estes 3). Contagem Layout: 12/0/3/3/0=18 →
**13/0/3/2/0=18**. Cobertura Layout: 12/18=67% → **13/18=72%
(target atingido)**. Total user-facing: 62/21/22/34/2=141 →
**63/21/22/33/2=141**. Tabela B Content: **50 → 51**.
ADR-0061 mantém-se `PROPOSTO` (Fase 3 pendente — repeat,
columns/colbreak); **anotação cumulativa Fase 1+2 adicionada
em ADR-0061 §Aplicações cumulativas** (sem promoção formal).

¹³ — Ajuste P156G (materialização Layout Fase 2 sub-passo 1;
**primeira aplicação Fase 2** — containers ricos): `block`
transita `ausente → implementado` (quinta aplicação consecutiva
de ADR-0061). **Decisão arquitectural 156G.2**: variant rico
`Content::Block { body, width, height, inset, breakable }`
escolhido sobre Style cascade — `Style` enum cobre só
propriedades de texto (Bold/Italic/Size), vocabulário
não-encaixa para width/height/inset; coerente com `Content::Pad`
(P156C). Subset Fase 1 implementado per ADR-0054 graded; **9
atributos vanilla scope-out** (outset, fill, stroke, radius,
clip, spacing, above/below, sticky). Contagem Layout:
10/0/3/5/0=18 → **11/0/3/4/0=18**. Cobertura Layout (impl +
impl⁺): 10/18=56% → **11/18=61%**. Contagem user-facing total
ajustada: 60/21/22/36/2=141 → **61/21/22/35/2=141** (+1
implementado, −1 ausente). Tabela B Content variants:
**48 → 49** (+`Block`). ADR-0061 continua `PROPOSTO`.

**Cobertura user-facing total** (impl + impl⁺) pós-P156J:
(64 + 21) / 141 = **60%** (≈60.3%)
(antes de P154A: 54%; após P154B: 55%; após P155: ~55-56%;
após P156B: ~53%; após P156C: ~55%; após P156D: ~56%; após
P156E: ~57%; após P156F: ~57%; após P156G: ~58%; após P156H:
~59%; após P156I: ~60% — Layout 67% → 72%, target Fase 1+2
atingido; **após P156J: ~60.3%** — Layout 72% → 78%, primeira
Fase 3).
**Itens scope-out**: 2 (font dict via ADR-0054bis; lang shaping via DEBT-53).

### Tabela B — Arquitectural (contagens)

| Tipo | `implementado` | `implementado⁺` | `parcial` | `ausente` | `scope-out` | Total |
|------|----------------|-----------------|-----------|-----------|-------------|-------|
| `Value` variants | 18 | 2 | 2 | 9 | 0 | 31 |
| `Content` variants (cristalino) ³ ⁴ ⁷ ⁹ ¹¹ ¹⁴ ¹⁶ ¹⁸ ²⁰ | 40 | 9 | 3 | 0 | 0 | 52 |
| `Content` variants (vanilla extra ausentes) | — | — | — | ~2 | — | ~2 |
| `Style` variants | 5 | 0 | 0 | 0 | 0 | 5 |
| `StyleDelta` fields | 7 | 2 | 0 | 0 | 1 | 10 |
| `FrameItem` variants | 6 | 0 | 0 | 0 | 0 | 6 |
| **Total arquitectural** | **68** | **13** | **5** | **19** | **1** | **106** |

³ — Ajuste P154B: 39 → 42 (+`Divider`, +`Terms`, +`TermItem`).
Vanilla extra ausentes desce de ~14 para ~12 (terms + divider
saíram do conjunto não-capturado).

⁴ — Ajuste P155: 42 → 43 (+`Quote`). Vanilla extra ausentes
desce de ~12 para ~11 (quote saiu do conjunto não-capturado).
ADR-0060 fechada (Fase 1 completa).

⁷ — Ajuste P156C: 43 → 45 (+`Pad`, +`Hide`). Vanilla extra
ausentes desce de ~11 para ~9 (pad e hide saíram do conjunto
não-capturado). Primeira aplicação concreta de ADR-0061
(Layout Fase X roadmap; mantém-se `PROPOSTO` até Fase 1
completa).

⁹ — Ajuste P156D: 45 → 47 (+`HSpace`, +`VSpace`). Vanilla
extra ausentes desce de ~9 para ~7 (h e v saem do conjunto
não-capturado). Segunda aplicação consecutiva de ADR-0061;
ADR-0061 mantém-se `PROPOSTO`.

¹¹ — Ajuste P156E: 47 → 48 (+`Pagebreak`). Vanilla extra
ausentes desce de ~7 para ~6 (pagebreak sai do conjunto
não-capturado). Terceira aplicação consecutiva de ADR-0061;
**halfway point Fase 1 atingido** (50% cobertura Layout).
ADR-0061 mantém-se `PROPOSTO`.

¹⁴ — Ajuste P156G: 48 → 49 (+`Block`). Vanilla extra ausentes
desce de ~6 para ~5 (block sai do conjunto não-capturado).
Quinta aplicação consecutiva de ADR-0061; **primeira aplicação
Fase 2** (containers ricos). Decisão arquitectural variant
rico (Opção A modificada) sobre Style cascade per inventário
156G.1. ADR-0061 mantém-se `PROPOSTO`.

¹⁶ — Ajuste P156H: 49 → 50 (+`Boxed`). Vanilla extra ausentes
desce de ~5 para ~4 (box sai do conjunto não-capturado). Sexta
aplicação consecutiva de ADR-0061; segunda Fase 2. Naming
`Boxed` em Rust evita conflito com `std::boxed::Box`; stdlib
`#box(...)` (paridade vanilla). Decisão arquitectural reusada
de P156G. ADR-0061 mantém-se `PROPOSTO`.

¹⁸ — Ajuste P156I: 50 → 51 (+`Stack`). Vanilla extra ausentes
desce de ~4 para ~3 (stack sai). Sétima aplicação consecutiva
de ADR-0061; **última Fase 2 — fechamento de série P156C-I**.
Decisão arquitectural reusada de P156G/H (variant rico) com
adaptação para `Arc<[Content]>` (clone O(1) per ADR-0026
revisão). Tipo `Dir` novo em `entities/dir.rs`. Cobertura
Layout 67% → **72% (target atingido)**. ADR-0061 mantém-se
`PROPOSTO` (Fase 3 pendente); anotação cumulativa Fase 1+2
adicionada à ADR sem promoção formal.

¹⁹ — Ajuste P156J (materialização Layout Fase 3 sub-passo 1;
**primeira aplicação Fase 3**): `repeat` transita `ausente →
implementado` (oitava aplicação consecutiva de ADR-0061;
**activa caminho 1** dos 3 documentados em §"Aplicações
cumulativas"). Decisão arquitectural reusada de P156G/H/I
(variant rico). `Content::Repeat { body, gap: Option<Length>,
justify: bool }` adicionado. Default `justify == true`
(paridade vanilla — divergência intencional do default Rust
`bool::default() == false`). **Limitação aceite per ADR-0054
graded**: algoritmo dinâmico de "quantidade-para-encher"
(vanilla calcula `floor(available / (body_width + gap))`)
diferido — Layouter executa single-render. Suficiente para
paridade estrutural (variant disponível em todo o pipeline,
counters/labels descem via walk). Helper `extract_length`
reusado **N=6** vezes consecutivas (P156C/D/G/H/I/J) — emergiu
como vocabulário canónico para coerção Length em named args
(subpadrão dentro de "reuso de template containers" N=3).
Padrão Smart→Option/default atinge **N=6** aplicações
consecutivas (P156D weak; P156E to; P156G/H width; P156I
spacing; **P156J gap**). Contagem Layout: 13/0/3/2/0=18 →
**14/0/3/1/0=18**. Cobertura Layout: 13/18=72% → **14/18=78%**.
Total user-facing: 63/21/22/33/2=141 → **64/21/22/32/2=141**.
Tabela B Content: **51 → 52**. ADR-0061 mantém-se `PROPOSTO`
(restante Fase 3: columns/colbreak — DEBT-56 column flow);
**anotação cumulativa P156J adicionada em §Aplicações
cumulativas** (sem promoção formal).

²⁰ — Ajuste P156J (Tabela B): 51 → **52** (+`Repeat`).
Vanilla extra ausentes desce de ~3 para ~2 (repeat sai do
conjunto não-capturado). Oitava aplicação consecutiva de
ADR-0061; **primeira Fase 3**. Restantes ~2 ausentes
(Bibliography/Cite, Footnote, Table, Columns, Stroke-object —
variando por classificação). ADR-0061 mantém-se `PROPOSTO`.

**Cobertura arquitectural total**: (68 + 13) / 106 = **76-77%**
(era 75-76% pós-P156I; era 75% pré-P155; era 72% pré-P154B;
era 70% pré-P149).
**Nota**: variants extra cristalino (`Value::Align`, `Content::Styled`) são **divergências intencionais**
favoráveis — cristalino tem features que vanilla não (em forma de Value); contadas como `implementado` porque
encerradas por ADRs (0026, 0028→0029, 0036, etc.).

---

## Top divergências surpreendentes

1. **`Value::Type` é `implementado⁺` em cristalino** (`type()` devolve `Value::Str(type_name)`, não
   `Value::Type(Type)` rico do vanilla). **Formalizado por ADR-0058** (Passo 149). `type(x) == "int"`
   funciona; `type(x) == int` (vanilla idiom) não funciona. Aceite dentro do perfil observacional
   graded de ADR-0054.

2. **`Value::Args` não é variant** em cristalino — `Args` é struct separada em `entities/args.rs`
   passada como `&Args` às funções nativas. **Formalizado por ADR-0059** (Passo 149). Alinhado com
   ADR-0036 (atomização progressiva).

3. **`Content::Heading` com show rules**: ADR-0041 declara "implementado" mas alguns atributos do vanilla
   `HeadingElem` (numbering style, supplement, outline-position) são parciais. Spec actual de
   `Content::Heading {level, body}` não cobre todos.

4. **`text.lang` parcial** — relatório 142 §3 listava como **scope-out total**; pós-Passo 144 é
   **implementado⁺** (hyphenation activo). Documentos pré-144 podem ter listas desactualizadas. Esta tabela
   corrige.

5. **`paint.rs` em vanilla expõe construtores** (`rgb`, `luma`, `cmyk`, `oklab`) mas cristalino só tem
   `rgb`/`luma`. CMYK e oklab são ausentes. Visual; afecta apenas casos de uso editoriais.

6. **`figure` é `implementado⁺`** — DEBT-14 e DEBT-15 fechadas (Passo 75), mas algumas variantes do
   `kind` parameter (table figures, equation figures) requerem `Content::Table` ou parametrização
   adicional. Aceite dentro de ADR-0033 perfil graded.

7. **Lista de `Content::*` ausentes em cristalino mas presentes em vanilla** é maior do que esperado:
   ~11 elementos pós-P155 (Bibliography, Cite, Footnote, Table, Columns, Box, Block, Stack,
   Hide, Repeat, Pad, Stroke-object — `Terms`, `Divider` e `Quote` saíram).
   **Refinamento P156B**: para a sub-categoria Layout especificamente,
   diagnóstico em [`diagnostico-layout-passo-156b.md`](diagnostico-layout-passo-156b.md)
   confirmou 11 entradas ausentes (`pad`, `box`, `block`, `stack`,
   `hide`, `repeat`, `columns`, `colbreak`, `pagebreak` manual,
   `h`/`v` combinada, `skew`) tratadas em **ADR-0061 PROPOSTO**
   (roadmap Fase 1 / 2 / 3). **DEBT-56 aberto** para `columns`
   (column flow L+). Cobertura Layout pós-recálculo: **22%
   implementado puro** (vs 38% declarado). Adição de `Content::Pad`,
   `Content::Hide`, `Content::Pagebreak`, `Content::HSpace`,
   `Content::VSpace` planeada para P156C (Fase 1 Layout) +
   extensão `Page::footnote_area` que **desbloqueia footnote**
   em Model Fase 2.
   **Refinamento P156C** (materialização Fase 1 sub-passo 1):
   `Content::Pad` e `Content::Hide` adicionados ao enum (43 → 45
   variants); stdlib `#pad(...)` e `#hide(body)` registadas.
   Cobertura Layout (impl + impl⁺): 22% → **33%** (4/18 → 6/18).
   Restantes 9 entradas Layout ausentes (`box`, `block`, `stack`,
   `repeat`, `columns`, `colbreak`, `pagebreak` manual, `h`/`v`
   combinada, `skew`) prosseguem em sub-passos seguintes da
   ADR-0061 (Fase 1 sub-passos 2-N: pagebreak, h+v; Fase 2:
   block+box+stack; Fase 3: columns+colbreak+repeat+skew).
   **Refinamento P156D** (materialização Fase 1 sub-passo 2):
   `Content::HSpace` e `Content::VSpace` adicionados (45 → 47
   variants); stdlib `#h(amount, weak: false)` e `#v(amount,
   weak: false)` registadas. Cobertura Layout (impl + impl⁺):
   33% → **44%** (6/18 → 8/18). Restantes 7 entradas ausentes
   (`pagebreak` manual, `box`, `block`, `stack`, `repeat`,
   `columns`/`colbreak`, `skew`) prosseguem nos sub-passos
   seguintes (P156E pagebreak; Fase 2 block+box+stack; Fase 3
   columns+repeat+skew).
   **Refinamento P156E** (materialização Fase 1 sub-passo 3;
   **halfway point Fase 1**): `Content::Pagebreak { weak, to:
   Option<Parity> }` adicionado (47 → 48 variants); stdlib
   `#pagebreak(weak: false, to: ?)`. Tipo `Parity` (Even/Odd)
   novo em `entities/parity.rs`. Cobertura Layout (impl +
   impl⁺): 44% → **50%** (8/18 → 9/18) — meio caminho para
   72% target. Restantes 6 entradas ausentes (`box`, `block`,
   `stack`, `repeat`, `columns`/`colbreak`, `skew`) prosseguem
   em Fase 2 (block+box+stack) e Fase 3 (columns+repeat+skew).
   **Refinamento P156F** (materialização Fase 1 sub-passo 4):
   `skew` transita `ausente → implementado` via método novo
   `TransformMatrix::skew(ax, ay)` + `native_skew` em
   `stdlib/transforms.rs`; **divergência da spec**: TransformKind
   enum não criado porque arquitectura matriz cm já unificava
   (descoberta empírica em 156F.1). Tabela B Content variants
   inalterado (48; refactor zero). Cobertura Layout (impl +
   impl⁺): 50% → **56%** (9/18 → 10/18). Restantes 5 entradas
   ausentes (`box`, `block`, `stack`, `repeat`,
   `columns`/`colbreak`) prosseguem em Fase 2 e Fase 3.
   **Refinamento P156G** (materialização Fase 2 sub-passo 1;
   **primeira Fase 2 — containers ricos**): `Content::Block
   { body, width, height, inset, breakable }` adicionado
   (48 → 49 variants); stdlib `#block(...)` registada.
   Decisão arquitectural variant rico (Opção A modificada
   sobre Style cascade) per inventário 156G.1; 9 atributos
   vanilla scope-out per ADR-0054 graded. Cobertura Layout
   (impl + impl⁺): 56% → **61%** (10/18 → 11/18). Restantes 4
   entradas ausentes (`box`, `stack`, `repeat`,
   `columns`/`colbreak`) prosseguem em Fase 2 sub-passos
   restantes (P156H box; P156I stack) e Fase 3 (repeat,
   columns).
   **Refinamento P156H** (materialização Fase 2 sub-passo 2):
   `Content::Boxed { body, width, height, inset, baseline }`
   adicionado (49 → 50 variants); stdlib `#box(...)`. Decisão
   arquitectural reusada de P156G (variant rico) sem nova
   decisão. Distinção material face a Block: **inline** (não
   força flush_line); atributo único `baseline`. 6 atributos
   vanilla scope-out (outset, fill, stroke, radius, clip,
   stroke-overhang). Cobertura Layout (impl + impl⁺): 61% →
   **67%** (11/18 → 12/18). Restantes 3 entradas ausentes
   (`stack`, `repeat`, `columns`/`colbreak`) prosseguem em
   P156I (stack — Fase 2 último sub-passo) e Fase 3 (repeat,
   columns).
   **Refinamento P156I** (materialização Fase 2 sub-passo 3;
   **último Fase 2 — fechamento de série P156C-I; atinge
   target 72%**): `Content::Stack { children: Arc<[Content]>,
   dir, spacing }` adicionado (50 → 51 variants); stdlib
   `#stack(dir: ?, spacing: ?, ..children)`. Tipo `Dir` novo
   (LTR/RTL/TTB/BTT). Decisão arquitectural reusada de P156G/H
   com adaptação para Vec/Arc. Cobertura Layout (impl +
   impl⁺): 67% → **72%** (12/18 → 13/18). Restantes 2 entradas
   ausentes (`repeat`, `columns`/`colbreak`) prosseguem em
   Fase 3 condicional (DEBT-56 column flow).
   **Refinamento P156J** (materialização Fase 3 sub-passo 1;
   **primeira aplicação Fase 3**): `repeat` transita `ausente
   → implementado`. `Content::Repeat { body, gap: Option<Length>,
   justify: bool }` adicionado (51 → 52 variants); stdlib
   `#repeat(body, gap: ?, justify: true)`. Default `justify ==
   true` (paridade vanilla). Decisão arquitectural reusada de
   P156G/H/I (variant rico). Algoritmo dinâmico de quantidade-
   para-encher diferido per ADR-0054 graded — Layouter executa
   single-render. Helper `extract_length` reusado N=6 vezes
   consecutivas; padrão Smart→Option/default atinge **N=6**.
   Cobertura Layout (impl + impl⁺): 72% → **78%** (13/18 →
   14/18). Restante 1 entrada ausente (`columns`/`colbreak`)
   bloqueada por DEBT-56 (column flow L+).
   Isto é **escopo XL agregado** se priorizado.
   **Refinamento P154A** (diagnóstico Model): para a sub-categoria Model especificamente, breakdown
   detalhado em [`diagnostico-model-passo-154a.md`](diagnostico-model-passo-154a.md). 6 entradas Model
   alto-valor (`bibliography`, `cite`, `footnote`, `quote`, `terms`, `table`) são tratadas em **ADR-0060
   PROPOSTO** (roadmap Fase 1+2+3); `bibliography`+`cite` ficam em **DEBT-55** (XL; bloqueado por
   ADR-0061 a criar). Recálculo Model: cobertura empírica **32-36%** (vs 38% declarado aqui), 22
   entradas (vs 21 declarados); ajuste integrado neste documento na Tabela A linha "Model".
   **Refinamento P154B** (materialização): `terms` + `divider` transitam para `implementado`;
   contagem Model é 5/4/5/8/0=22; cobertura Model **32-36% → 41%** (10/22 entradas implementadas
   ou parciais).
   **Refinamento P155** (materialização Fase 1 sub-passo 2; **fecha Fase 1**): `quote` transita
   para `implementado`; contagem Model é 6/4/5/7/0=22; cobertura Model **41% → 45%** (11/22).
   ADR-0060 transita `PROPOSTO → IMPLEMENTADO`.

8. **Math layout aproximado (`implementado⁺`)** vs vanilla strict — cristalino tem `MathMatrix`,
   `MathFrac`, `MathRoot`, `MathDelimited`, etc., mas posicionamento exacto requer métricas de font math
   (OpenType MATH table) que não são consumidas. ADR-0033 perfil graded cobre.

9. **`Value::Align` é divergência cristalino** — vanilla resolve via `HAlign` + `VAlign` separadas;
   cristalino tem `Value::Align(Align2D)`. Resolução em DEBT-36 (Passo 84.5).

10. **`Content::Styled(Box, Styles)` é divergência ADR-0026** — vanilla usa vtable; cristalino enum
    fechado. Esta é **a** divergência arquitectural fundamental. Mantida como escolha estrutural.

---

## Notas operacionais

- **Inventário factual, não medição**. As classificações reflectem estado em 2026-04-24; medição
  observacional contra vanilla virá no Passo 149+.
- **`scope-out` é decisão consciente, não dívida**. Itens com ADR explícita (gap 8 font dict;
  shaping rustybuzz; lang shaping) são excluídos do denominador de "paridade cobrível" se assim for
  declarado em medição futura.
- **Actualização ad-hoc**: cada passo que materializa uma feature deve actualizar a entrada
  correspondente neste documento. Sem ADR formal de governança — burocracia mínima.
- **Vanilla freezing**: documento âncora no commit `ba61529986e0a5a916cbf937c3c65117cd450683`. Quando
  vanilla actualizar (versão maior), abrir passo dedicado para re-inventário (não automático).
- **Granularidade**: features com mais de uma forma (ex: `text.font` string/array/dict) listadas como
  entradas separadas. Aliases (ex: `*bold*` sintaxe vs `strong()` função) listadas separadamente com
  cross-reference.
- **Categorias seguem `lab/typst-original/crates/typst-library/src/`**: foundations, introspection,
  layout, loading, math, model, pdf, text, visualize.

---

## Cross-references

- ADR-0001, 0016, 0017, 0021, 0023, 0025, 0026 + 0026-R1, 0027, 0028→0029, 0033, 0034, 0038, 0039,
  0040, 0041, 0052, 0053, 0054, 0055, 0057, **0060** (Model roadmap), **0061** (Layout roadmap, P156B).
- Passos 9, 13–25, 30–46, 50, 60–66, 70–84.6, 96–146, **156A** (historiograma), **156B** (diagnóstico Layout).
- DEBTs encerrados: 1, 2 (parcial), 6, 7, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
  24a/b/c, 25, 26, 27, 28, 29, 30, 31, 32, 36, 37, 38, 39, 40, 41, 44, 45, 46, 47, 48, 49, 51, 52.
- DEBTs abertos: 2 (residual), 8, 9, 33, 34d, 34e, 35b, 42, 43, 50, **53**, **54**, **55**, **56** (P156B; column flow Fase 3 Layout).
- Candidatos futuros: ADR-0054bis (regex em L1; gap 8), ADR-0055bis (variant-aware fonts),
  ADR-0056 (subsetting), **ADR-0062 reservada** (hayagriva; era ADR-0061 antes de P156B);
  ADR dedicada column flow algorithm (quando DEBT-56 for materializado);
  DEBT-53 (rustybuzz integration).
- Documentos relacionados:
  - [`typst-paridade-definicoes.md`](typst-paridade-definicoes.md) — definições operacionais de "passa".
  - [`typst-paridade-plano-medicao.md`](typst-paridade-plano-medicao.md) — plano de medição P1–P4.
  - [`relatorios/fecho-debt-1-passo-142.md`](../relatorios/fecho-debt-1-passo-142.md) — mapeamento campo-a-campo de `StyleDelta`.
