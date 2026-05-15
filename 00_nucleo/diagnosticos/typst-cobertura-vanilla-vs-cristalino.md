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
| `> blockquote` | model/quote.rs | `implementado` | Passo 155 | `Content::Quote` materializado P155 (Fase 1 sub-passo 2 fechada); 4 attrs (body, attribution, block, quotes) |
| `/ term: definition` | model/terms.rs | `implementado` | Passo 154B | `Content::Terms` + `Content::TermItem` materializados P154B (Fase 1 sub-passo 1) |
| `$inline math$` | math/equation.rs | `implementado⁺` | Passos 34–46 | superscripts/subscripts/fracções; matrix/cases capturados; sem shaping completo (ADR-0054 perfil graded) |
| `$ display math $` | math/equation.rs | `implementado⁺` | idem | `block: true` em `Content::Equation` |
| `` `inline raw` ``, ```` ```block``` ```` | text/raw.rs | `implementado` | Passo 23 | `Content::Raw` com `lang` opcional |
| `<label>`, `@ref` | foundations/label.rs, model/reference.rs | `implementado⁺` | Passo 63 | `Content::Labelled`, `Content::Ref`; forward-refs limitadas (DEBT-10 fechada) |
| Smart quotes (`"foo"` → "foo") | text/smartquote.rs | `implementado` | Passo 155 | smart-quotes lang-aware (6 idiomas + default ASCII) via `rules/lang/quotes.rs`; markup `"..."` produz aspas localizadas via alternância open/close em `eval_markup` |
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
| `pad(...)` | layout/pad.rs | `implementado⁺` ⁶ ²¹ | Passos 156C + 156L (refino sides individualizadas) | `Content::Pad { body, sides: Sides<Option<Length>> }` + stdlib `#pad(body, left:?, right:?, top:?, bottom:?, x:?, y:?, rest:?)`; **P156L refino**: cada side passa a `Option<Length>` per ADR-0064 Caso C (None ↔ default zero resolvido em uso); helper `extract_sides_lengths` privado; `right` continua scope-out em layout (perfil ADR-0054 graded); padding negativo rejeitado |
| `align(alignment, body)` | layout/align.rs | `implementado` | Passos 84.5–84.6 (DEBT-36, 37) | `Align2D`; `Place` com scope |
| `place(alignment, ..., body)` | layout/place.rs | `implementado⁺` ⁵ ⁴⁴ ⁴⁶ | Passos 84.5 + 84.6 + 223 (encerrado série α P225) | refino aditivo P223 — `float: bool` + `clearance: Option<Length>` armazenados (semantic real adiada per ADR-0054 graded; pattern N=5 cumulativo weak/breakable/float/repeat); restrição vanilla `scope: Parent + float: true` restaurada (DEBT-37 §"Divergência" fechada — Decisão 3 Opção α); refino multi-pass flow contorna Fase 5 candidata NÃO-reservada per política P158 |
| `box(...)` | layout/container.rs | `implementado⁺` ¹⁵ ⁶⁵ ⁶⁶ ⁶⁹ | Passos 156H + 231 + 242 + 247 + 248 + **252** | `Content::Boxed { body, width, height, inset, baseline, outset, radius, clip, fill, stroke }` + stdlib `#box(body, width: ?, height: ?, inset: ?, baseline: ?, outset: ?, radius: ?, clip: ?, fill: ?, stroke: ?)`; **P252 Boxed A.4 COMPLETO 6/6** via refactor cross-cutting `Stroke` primitivo (+1 field `overhang: bool`); bounds Shape expandidos por thickness/2 quando overhang=true (paridade vanilla); construtor Rust default `false` (divergência consciente; paridade restaurada via stdlib `extract_stroke`); **segundo variant Content com 100% scope-outs originais P156H fechados cumulativamente** (após Block P250 10/10) |
| `block(...)` | layout/container.rs | `implementado⁺` ¹³ ⁶⁵ ⁶⁶ ⁶⁷ | Passos 156G + 231 + 242 + 243 + 247 + 248 + **250** | `Content::Block { body, width, height, inset, breakable, outset, radius, clip, fill, stroke, spacing, above, below, sticky }` + stdlib `#block(..., spacing: ?, above: ?, below: ?, sticky: ?)`; **P250 Block A.4 COMPLETO 10/10**: spacing/above/below collapse semantic (max prev.below, curr.above) + sticky lookahead 1-block via peekable Sequence consumer; refactor Sequence cross-arm (pattern emergente N=1 inaugurado); **primeira aplicação citante ADR-0082 PROPOSTO N=1**; 10 dos 9 scope-outs originais P156G fechados cumulativamente (incluindo breakable contado como elemento original) |
| `columns(n)` | layout/columns.rs | `parcial` ⁴⁰ ⁴² | Passos 217 + 218 + 219 (encerrado P221) | variant + stdlib + arm consumer real graded; **multi-region flow real ausente** (Opção A diferida a P-Layout-Fase4 candidata NÃO-reservada) |
| `grid(columns, ...)` | layout/grid | `implementado⁺` ⁵ ⁴⁵ ⁴⁶ ⁴⁷ ⁴⁸ ⁴⁹ | Passos 82 + 83 + 84.6 + 224 + 227 + 228 + 230 | refino substantivo P224 + refinos aditivos P227+P228+P230 — Grid agora +7 fields cumulativos (gutter/align/inset/header/footer/stroke/fill); **GridCell + TableCell +2 fields cosméticos per-cell P230** (stroke + fill com precedência override Grid-level via `.or()`); 3 variants Content novos P224 + módulo `grid_placement.rs`. **DEBT-34e ENCERRADO P224**; **DEBT-34d** preservado aberto per `P224.div-1`. Render Opção β Z-order correcto: fill → conteúdo → stroke; precedência per-cell P230 |
| `stack(spacing, ...)` | layout/stack.rs | `implementado` ¹⁷ | Passo 156I (ADR-0061 Fase 2 sub-passo 3; **último Fase 2; atinge target 72%**) | `Content::Stack { children: Arc<[Content]>, dir: Dir, spacing: Option<Length> }` + stdlib `#stack(dir: ?, spacing: ?, ..children)`; tipo `Dir` novo (LTR/RTL/TTB/BTT); 4 direcções implementadas; spacing real entre children |
| `pagebreak()` (manual) | layout/page.rs | `implementado` ¹⁰ | Passo 156E (ADR-0061 Fase 1 sub-passo 3) | `Content::Pagebreak { weak, to: Option<Parity> }` + stdlib `#pagebreak(weak: false, to: ?)`; `to:"even"`/`"odd"` insere página vazia se necessário; `weak` collapse defere; tipo `Parity` novo em `entities/parity.rs` |
| `colbreak()` | layout/columns.rs | `parcial` ⁴¹ ⁴² | Passo 220 (encerrado P221) | variant `Content::Colbreak { weak: bool }` + stdlib `#colbreak(weak: ?)` + arm Layouter Opção β graded (downgrade a pagebreak literal); paridade vanilla quando fora de columns context; **multi-region salto entre colunas reais ausente** (P-Layout-Fase4 candidata NÃO-reservada) |
| `rotate(angle, body)` | layout/transform.rs | `implementado` | Passo 78 | `Content::Transform` |
| `scale(amount, body)` | idem | `implementado` | Passo 78 | |
| `move(dx, dy, body)` | idem | `implementado` | stdlib `native_move` | |
| `hide(body)` | layout/hide.rs | `implementado` ⁶ | Passo 156C (ADR-0061 Fase 1) | `Content::Hide { body }` + stdlib `#hide(body)`; calcula dimensões mas emite zero items (per ADR-0054 graded) |
| `repeat(body)` | layout/repeat.rs | `implementado` ¹⁹ | Passo 156J (ADR-0061 Fase 3 sub-passo 1; **primeira Fase 3**) | `Content::Repeat { body, gap: Option<Length>, justify: bool }` + stdlib `#repeat(body, gap: ?, justify: true)`; default `justify == true` (paridade vanilla); algoritmo dinâmico de quantidade-para-encher diferido per ADR-0054 graded (Layouter executa single-render — paridade estrutural suficiente para counters/labels descenderem) |
| `pad`, `corners`, `sides` (inset modeling) | layout/{pad,corners,sides}.rs | `ausente` | — | duplica `pad()` linha; refino PageConfig é Fase 3 ADR-0061 |
| `measure(body)` | layout/measure.rs | `implementado⁺` ⁴³ ⁴⁶ | Passo 222 (encerrado série α P225) | stdlib `#measure(body) -> dict(width: length, height: length)` exposta — helper `measure_content` em `01_core/src/rules/layout/helpers.rs` promovido a `pub(crate)`; **Opção β graded** — width override scope-out (refino futuro candidato NÃO-reservado); runtime queries genuínas (counter values, labels) continuam diferidas per ADR-0066 PROPOSTO §"Plano promoção" Bloco C cross-módulo primeira materialização parcial |
| `h(amount)` / `v(amount)` ⁵ | layout/spacing.rs | `implementado` ⁸ | Passo 156D (ADR-0061 Fase 1 sub-passo 2) | `Content::HSpace` + `Content::VSpace` com `amount: Length, weak: bool`; stdlib `#h(amount, weak: false)` + `#v(...)`; `weak` armazenado mas collapse defere; amount `Fraction` scope-out (refino futuro per ADR-0061 §6.3) |
| `skew(ax, ay, body)` ⁵ | layout/transform.rs | `implementado` ¹² | Passo 156F (ADR-0061 Fase 1 sub-passo 4) | `TransformMatrix::skew(ax_rad, ay_rad)` novo + `native_skew` reusa `Content::Transform { matrix }` existente desde P78; **sem refactor** (matriz cm já unificava); ângulos próximos de ±π/2 rejeitados; `origin` scope-out |

⁵ — Reclassificação ou adição P156B. Ver
[`diagnostico-layout-passo-156b.md`](diagnostico-layout-passo-156b.md)
§2.7 para detalhe.

### A.6 — Model (structural)

| Feature | Vanilla | Cristalino | Referência | Nota |
|---------|---------|------------|------------|------|
| `heading(level, body)` | model/heading.rs | `implementado` | Passos 22, 99, 103 | construtor + show rules |
| `figure(body, caption, ...)` | model/figure.rs | `implementado⁺` ²⁸ ³¹ ³⁴ | Passos 75 + ADR-0041 + P158A (auto-detecção) + P158B (supplement por lang) + P158C (refactor kind→Option) | numbering por kind; counters; **P158A**: auto-detecção de `kind` baseada no body; **P158B**: supplement automático localizado por lang (6 langs × 3 kinds; fallback PT); **P158C**: refactor `kind: String → Option<String>` per ADR-0064 Caso A estrito (None ↔ Auto; default `"image"` resolvido em uso por callers — patamar Caso A N=6→7) |
| `caption(...)` | model/figure.rs | `parcial` | dentro de figure | sem element dedicado |
| `outline()` | model/outline.rs | `implementado` | Passos 65–66 | TOC via 2-pass introspection |
| `table(columns, ...)` | model/table.rs | `implementado` ²² | Passo 157A (ADR-0060 Fase 2 sub-passo 1; **primeiro Model Fase 2**) | `Content::Table { columns, rows, children: Vec<Content> }` + stdlib `#table(columns: ?, rows: ?, ..children)`; subset minimal per ADR-0054 graded; layouter delega a `layout_grid` (clone simples; sem modificação de `grid.rs`); 9+ atributos vanilla scope-out (gutter/inset/align/fill/stroke/summary; cells estruturadas P157B; header/footer P157C; HLine/VLine cosmetic) |
| `table.cell(body, ...)` | model/table.rs | `parcial⁺` ²⁴ ⁶⁸ | Passos 157B + 248 + **251** | `Content::TableCell { body, x: Option<usize>, y: Option<usize>, colspan: Option<usize>, rowspan: Option<usize> }` + stdlib `#table_cell(body, x: ?, y: ?, colspan: ?, rowspan: ?)`; **naming `table_cell` flat** (não vanilla `table.cell` — FieldAccess actual não suporta namespacing de funcs; divergência intencional per ADR-0033); ADR-0064 Caso A para x/y; Caso C para colspan/rowspan; **placement algorítmico diferido em DEBT-34e** — fields armazenados mas ignorados em layout per ADR-0054 graded; 6 atributos vanilla scope-out (align/stroke/fill/inset/breakable + internals); **P248 overflow clip implícito** (rows Fixed; preservado paridade vanilla); **P251 row break vertical real cell-level γ-Items** (rows Auto/Fraction; slice items + buffer pending + flush em new_page chain; **activa Categoria C.2 parcial**) |
| `table.header(body, ...)` | model/table.rs | `parcial` ²⁶ | Passo 157C (ADR-0060 Fase 2 sub-passo 3 — **fecha table foundations**) | `Content::TableHeader { body, repeat: bool }` + stdlib `#table_header(body, repeat: true)`; **naming `table_header` flat** (paridade decisão P157B); ADR-0064 Caso D para `repeat` (default vanilla `true` — **primeira aplicação Caso D em Model**); algoritmo de repetição em page breaks **diferido em DEBT-56**; `level`/`repeat-rows` scope-out per ADR-0054 graded; divergência aceite per ADR-0033 (`body: Box<Content>` em vez de vanilla `Vec<TableItem>`) |
| `table.footer(body, ...)` | model/table.rs | `parcial` ²⁶ | Passo 157C (par simétrico de header) | `Content::TableFooter { body, repeat: bool }` + stdlib `#table_footer(body, repeat: true)`; paridade absoluta com TableHeader (mesmos fields; mesma decisão Caso D + DEBT-56) |
| `list(items)` (function form) | model/list.rs | `parcial` | sintaxe parcial | sem function form completa |
| `enum(items)` | model/enum.rs | `parcial` | idem | |
| `terms(...)` | model/terms.rs | `implementado` | Passo 154B | `Content::Terms` + `Content::TermItem`; named args via `#terms(key: [desc])`; sem atributos vanilla (tight/separator/indent/hanging-indent) — extensíveis sem breaking change |
| `quote(...)` | model/quote.rs | `implementado` | Passo 155 | `Content::Quote` com 4 attrs; smart-quotes lang-aware (6 idiomas + default ASCII) via `rules/lang/quotes.rs`; markup `"..."` produz aspas localizadas via alternância open/close (não pareadas como bloco) |
| `cite(key)` | model/cite.rs | `parcial` ²⁹ ³² ³⁵ | Passo 159A (par acoplado) + P159C (form variants) + P159F (numbering numérico) | `Content::Cite { key: String, supplement: Option<Box<Content>>, form: Option<CitationForm> }` + stdlib `#cite(key, supplement: ?, form: ?)`; **naming `cite` flat**; ADR-0064 Caso A para supplement (P159A) + form (P159C); **render por form com lookup Bibliography** same-document; **P159F**: form Normal/None ganha numeração numérica `[N]` via `state.bib_numbers` (style numeric simplificado; subpadrão #15 N=3); fallback `[key]` se Bibliography vazia ou key não encontrada; forms Prose/Author/Year inalteradas; sem validação cross-reference (ADR-0017 adiada); 1+ atributo vanilla scope-out (style CSL override) |
| `bibliography(path)` | model/bibliography.rs | `parcial` ²⁹ ³³ ³⁵ ³⁶ ³⁷ | Passo 159A (par acoplado) + P159D (4 fields) + P159F (numbering) + P159E (url+doi) + P159G (6 fields restantes) | `Content::Bibliography { entries: Vec<BibEntry>, title: Option<Box<Content>> }` + stdlib `#bibliography(entries, title: ?)`; **input cristalino literal Vec<BibEntry>** (sem hayagriva; ADR-0062 mantém-se reserva sem ficheiro); tipo entity extendido `BibEntry { key, author, title, year, volume, pages, journal, publisher, url, doi, editor, series, note, isbn, location, organization }` em `entities/bib_entry.rs` (**16 fields**: 4 obrigatórios P159A + 4 comuns P159D + 2 identificadores P159E + 6 restantes P159G via builder pattern; cobertura ~70-75% hayagriva universais); ADR-0064 Caso A para title; **render extendido APA-like** condicional por field presente; **P159F**: walk popula `state.bib_numbers`; **P159E**: url plaintext + DOI prefixo `doi:`; **P159G**: editor `(Ed. ...)`, series `(...)`, location:publisher, isbn:`isbn:...`, note `[...]`, organization substitutivo a publisher; restantes fields vanilla (booktitle/address/chapter/type/institution/etc.) + styles CSL — Bloco B hayagriva) |
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
| `counter(key)` | introspection/counter.rs | `implementado⁺` | Passos 60–62 + P176 + P177 + P210B | step/update; `counter_step` (P210B); `counter_at` (P177 location-aware); `counter_final` (P176 hierarchical). `counter.display(numbering)` here-aware deferred per P210A C3 (gatilho: walk advance materializado) |
| `state(key, ...)` | introspection/state.rs | `implementado` | P171 (M9) | `state(key, init)` + `state_update(key, value)` + `state_update_with(key, fn)` registadas no scope global. `state.get()` here-aware deferred per P210A C3 (mesmo gatilho) |
| `here()` / `locate()` | introspection/{here,locate}.rs | `implementado` | P208B + P208C (M9c) | `here()` lê `EvalContext.current_location` (P208B); `locate(kind)` reusa `parse_selector_arg` + `Introspector::query.first()` (P208C). Walk advance automático deferred (gatilho: `Content::Context` block ou consumer real) |
| `query(...)` | introspection/query.rs | `implementado⁺` | P175 (minimal) + P209A-D (M9c — 5 variants Selector) | `query(arg)` aceita `Value::Str(kind)`, `Value::Str("<label>")`, `Value::Location(loc)` via `parse_selector_arg`. `Selector::And/Or` Rust API only; `Selector::Regex` query stub `vec![]` (gatilho: query-by-text accessível) |
| `metadata(value)` | introspection/metadata.rs | `implementado` | P169 (M9 sub-passo 1) | `metadata(value)` registada; `MetadataStore` sub-store + `Introspector::query_metadata()` |
| `position(target)` | introspection/position.rs | `parcial` | P204D + P205B/C (F3) | `Introspector::position_of(loc) -> Option<Position>` materializado via `SealedPositions` (P205C inject_positions); **sem stdlib expose `position()` standalone**. Bloco B candidato pós-M9c |

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
| `Grid {columns, rows, cells, gutter, align, inset, header, footer}` | GridElem | `implementado⁺` | Passos 82 + 83 + 84.6 + 224 | refino P224 +5 fields (gutter/align/inset/header/footer) — semantic real adiada graded; `stroke`/`fill` cosméticos scope-out; DEBT-34e ENCERRADO P224; DEBT-34d preservado aberto |
| `GridHeader {body, repeat}` | GridHeaderElem | `implementado⁺` | Passo 224.B | paridade P157C TableHeader literal; `repeat` semantic adiada (pattern N=5 weak/breakable/float/repeat) |
| `GridFooter {body, repeat}` | GridFooterElem | `implementado⁺` | Passo 224.B | par simétrico GridHeader |
| `GridCell {body, x, y, colspan, rowspan}` | GridCellElem | `implementado⁺` | Passo 224.C | paridade P157B TableCell literal; placement algorítmico real via `grid_placement::place_cells` (fecha DEBT-34e); per-cell `align`/`fill`/`stroke`/`inset`/`breakable` scope-out |
| `SetPage {...}` | (set rule) | `implementado` | Passo 81 | |
| `Align {...}` | AlignElem | `implementado` | Passo 84.5 | |
| `Place {alignment, dx, dy, scope, float, clearance, body}` | PlaceElem | `implementado⁺` | Passos 84.5 + 84.6 + 223 | refino P223 +2 fields (`float: bool` + `clearance: Option<Length>`) armazenados (semantic adiada per ADR-0054 graded); DEBT-37 §"Divergência" fechada (scope `Parent` exige `float: true` paridade vanilla restaurada) |
| `Styled(Box<Content>, Styles)` | (vtable + show rules) | `implementado` | Passos 99–101 (ADR-0038/0039) | divergência ADR-0026 |
| `Divider` | DividerElem | `implementado` | Passo 154B | singleton; layouter emite linha 0.5pt |
| `Terms {items}` | TermsElem | `implementado` | Passo 154B | sem atributos vanilla (tight/sep/indent) |
| `TermItem {term, description}` | TermItemElem | `implementado` | Passo 154B | par item; standalone permitido |
| `Quote {body, attribution, block, quotes}` | QuoteElem | `implementado` | Passo 155 | 4 atributos materializados; smart-quotes lang-aware via `rules/lang/quotes.rs` |
| `Columns {count, gutter, body}` | ColumnsElem | `parcial` | Passo 221 (cumulativo P217-P219) | variant + stdlib `#columns(count, body, gutter:?)` + arm Layouter consumer real graded (Opção B); width temporariamente reduzida `(full_width − (count−1)·gutter)/count`; body single-render; multi-region flow real scope-out (P-Layout-Fase4 candidato); default gutter ~4% via `COLUMNS_DEFAULT_GUTTER_RATIO` |
| `Colbreak {weak}` | ColbreakElem | `parcial` | Passo 220 | variant + stdlib `#colbreak(weak:?)` + arm Layouter Opção β graded (downgrade literal a pagebreak via reuso `Layouter::new_page`); paridade vanilla quando fora de columns context; sem `to:` (vanilla `ColbreakElem` não tem) |
| **Vanilla-only (ausentes)**: BibliographyElem, CiteElem, FootnoteElem, TableElem, BoxElem, BlockElem, StackElem, HideElem, RepeatElem, PadElem, MoveElem (function só), GradientElem, TilingElem, StrokeElem (object form), … | — | `ausente` (cada) | — | escopo crescente; **`ColumnsElem` e `ColbreakElem` removidos da lista em P221** (transitam para `parcial`) |

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
| Markup syntactic ³⁹ | 11 | 3 | 3 | 1 | 0 | 18 |
| `#let`/`#set`/`#show`/import | 7 | 1 | 4 | 1 | 0 | 13 |
| Text features | 7 | 5 | 1 | 8 | 2 | 23 |
| Math | 6 | 6 | 1 | 0 | 0 | 13 |
| Layout ⁵ ⁶ ⁸ ¹⁰ ¹² ¹³ ¹⁵ ¹⁷ ¹⁹ ²¹ ⁴⁰ ⁴¹ ⁴² ⁴³ ⁴⁴ ⁴⁵ ⁴⁶ | 12 | 4 | 2 | 0 | 0 | 18 |
| Model (structural) ¹ ² ³ ²² ²⁴ ²⁹ | 7 | 4 | 7 | 4 | 0 | 22 |
| Visualize | 6 | 1 | 1 | 5 | 0 | 13 |
| Foundations stdlib | 9 | 1 | 4 | 1 | 0 | 15 |
| Introspection ³⁸ | 3 | 2 | 1 | 0 | 0 | 6 |
| **Total user-facing** ⁵ ⁶ ⁸ ¹⁰ ¹² ¹³ ¹⁵ ¹⁷ ¹⁹ ²¹ ²² ²⁹ ³⁸ ³⁹ ⁴⁰ ⁴¹ ⁴² ⁴³ ⁴⁴ ⁴⁵ ⁴⁶ | **68** | **27** | **24** | **20** | **2** | **141** |

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

**Cobertura user-facing total** (impl + impl⁺) pós-P159A:
(64 + 22) / 141 = **61%** (≈61.0%; **inalterada agregada
impl+impl⁺** vs P157A/B/C/P158/A; entradas `parcial` cresceram
22 → 24 com P159A — `cite` e `bibliography` movem `ausente →
parcial`; ganho qualitativo via 2 entradas saírem de ausente;
cobertura ampla impl+impl⁺+parcial: (64+22+24)/141 = **77%**
crescente)
(antes de P154A: 54%; após P154B: 55%; após P155: ~55-56%;
após P156B: ~53%; após P156C: ~55%; após P156D: ~56%; após
P156E: ~57%; após P156F: ~57%; após P156G: ~58%; após P156H:
~59%; após P156I: ~60% — Layout 67% → 72%, target Fase 1+2
atingido; após P156J: ~60.3% — Layout 72% → 78%, primeira
Fase 3; após P156L: ~60.3% (inalterada) — refino qualitativo
de `pad` per ADR-0064 Caso C; após P157A: ~61.0% — Model
45% → 50%, primeiro sub-passo Model Fase 2; após P157B:
~61.0% (inalterada agregada) — expansão estrutural de
`table` via `TableCell` sub-entrada; ganho qualitativo;
ADR-0064 Caso A primeira aplicação Model; após P157C:
~61.0% (inalterada agregada) — par simétrico
TableHeader/TableFooter completa expansão estrutural de
"table foundations"; ADR-0064 Caso D primeira aplicação Model;
**saturação cross-domínio cross-caso** A/B/C/D em Layout + Model;
após P158A: ~61.0% (inalterada) — refino qualitativo
auto-detecção figure-kinds; **após P159A: ~61.0% impl+impl⁺
inalterada / parcial cresce 22 → 24** — par acoplado
Bibliography+Cite minimal sem hayagriva; ADR-0064 Caso A
patamar N=4 → 5; **após P158B: ~61.0% (inalterada)** — segundo
refino qualitativo consecutivo de `figure` (supplement por lang;
6 langs × 3 kinds); reuso pattern P155 cross-feature N=1;
**após P159C: ~61.0% (inalterada)** — refino estrutural de
`cite` adicionando form variants; ADR-0064 Caso A patamar
N=5 → 6 atingindo equilíbrio cross-domínio 50/50 Layout/Model;
**após P159D: ~61.0% (inalterada)** — refino de tipo entity
`BibEntry` adicionando 4 fields universais opcionais; ADR-0065
critério #2 patamar N=2→3; subpadrão "refino tipo entity sem
alteração Content variant" N=1; **após P158C: ~61.0% (inalterada)**
— refactor cosmético `Figure.kind: String → Option<String>` per
ADR-0064 Caso A estrito; patamar Caso A N=6→7; subpadrão
"refactor de field para Option" N=1 NOVO; **após P159F: ~61.0%
(inalterada)** — counter local + numbering numérico em Cite
Normal/None; subpadrão #15 N=2→3 "infraestrutura state lookup";
**Bloco A esgotado** após este passo; **após P159E: ~61.0%
(inalterada)** — par natural url+doi em BibEntry; primeiro
sub-passo família 159 fora Bloco A; subpadrão #16 N=1→2
"refino tipo entity sem alteração Content"; helper `optional_str`
N=4 cumulativos atinge limiar promoção; **após P159G: ~61.0%
(inalterada)** — 6 fields restantes comuns hayagriva em
BibEntry; segundo sub-passo família 159 fora Bloco A;
**subpadrão #16 N=2→3** atinge limiar formalização; **helper
`optional_str` cumulativo N=12** largamente promovível;
BibEntry com 16 fields cobertura ~70-75% hayagriva universais).
**Itens scope-out**: 2 (font dict via ADR-0054bis; lang shaping via DEBT-53).

### Tabela B — Arquitectural (contagens)

| Tipo | `implementado` | `implementado⁺` | `parcial` | `ausente` | `scope-out` | Total |
|------|----------------|-----------------|-----------|-----------|-------------|-------|
| `Value` variants | 18 | 2 | 2 | 9 | 0 | 31 |
| `Content` variants (cristalino) ³ ⁴ ⁷ ⁹ ¹¹ ¹⁴ ¹⁶ ¹⁸ ²⁰ ²³ ²⁵ ²⁷ ³⁰ | 46 | 9 | 3 | 0 | 0 | 58 |
| `Content` variants (vanilla extra ausentes) | — | — | — | 0 | — | 0 |
| `Style` variants | 5 | 0 | 0 | 0 | 0 | 5 |
| `StyleDelta` fields | 7 | 2 | 0 | 0 | 1 | 10 |
| `FrameItem` variants | 6 | 0 | 0 | 0 | 0 | 6 |
| **Total arquitectural** | **74** | **13** | **5** | **13** | **1** | **106** |

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

²² — Ajuste P157A (materialização Model Fase 2 sub-passo 1;
**primeiro Model Fase 2**): `table` transita `ausente →
implementado` (décima aplicação consecutiva de materialização
desde início da série granular P156C-L; **primeiro Model Fase 2**
após Fase 1 fechada P155). `Content::Table { columns: Vec<TrackSizing>,
rows: Vec<TrackSizing>, children: Vec<Content> }` adicionado
(52 → 53 variants); stdlib `#table(columns: ?, rows: ?,
..children)` em `stdlib/structural.rs` (decisão de módulo Model
existente, não novo `stdlib/model.rs`). Subset minimal per
ADR-0054 graded; layouter delega a `layout_grid` clone simples
per ADR-0060 §"Decisão 4" (sem modificação de `grid.rs`).
Helper `extract_tracks` promovido a `pub(super)` para reuso
cross-módulo (N=2; subpadrão emergente análogo a
`extract_length` N=7). 9+ atributos vanilla scope-out
(gutter/inset/align/fill/stroke/summary; cells estruturadas
P157B; header/footer P157C; HLine/VLine cosmetic). Contagem
Model: 6/4/5/7/0=22 → **7/4/5/6/0=22**. Cobertura Model
(impl + impl⁺): 10/22=45% → **11/22=50%**. Total user-facing:
63/22/22/32/2=141 → **64/22/22/31/2=141** (+1 implementado,
−1 ausente). Tabela B Content: 52 → 53 (footnote ²³).
ADR-0060 mantém-se `IMPLEMENTADO` (Fase 1 fechada P155;
Fase 2 prossegue per roadmap). **Padrões consolidados**:
granularidade N=10 (P156C-L + P157A; primeira N=10 sem
reformulação cross-domínio Layout→Model); inventariar primeiro
N=8 (séptima sob critério estendido ADR-0065); ADR-0064 não
aplicável directamente (subset sem Smart<T>); reuso
`extract_tracks` N=2.

²³ — Ajuste P157A (Tabela B): 52 → **53** (+`Table`).
Vanilla extra ausentes desce de ~2 para ~1-2 (table sai do
conjunto não-capturado). Décima aplicação consecutiva de
materialização; **primeiro Model Fase 2**. ADR-0060 Decisão 4
respeitada (variant dedicado, não Styled).

²⁴ — Ajuste P157B (Tabela A.6): nova entrada `table.cell`
adicionada como **sub-entrada de `table`** (não conta na
agregação Model — vanilla também trata como `#[scope]`
subordinado de `TableElem`). Estado `parcial`: subset minimal
(5 fields) materializado per ADR-0054 graded; placement
algorítmico diferido em DEBT-34e (fields x/y/colspan/rowspan
armazenados mas ignorados em layout). **Naming `table_cell`
flat** (não vanilla `table.cell` — FieldAccess actual não
suporta namespacing de funcs; divergência intencional per
ADR-0033 documentada em diagnóstico P157B §8). **Primeira
aplicação concreta de ADR-0064 Caso A em domínio Model**
(P156G/H/I aplicaram-no em Layout); **Caso C terceira aplicação
global** com **primeira variação `usize`** (anteriores eram
`Length`). Contagem agregada Model **inalterada** (7/4/5/6/0=22)
porque sub-entrada não conta separadamente. Cobertura Model
permanece **50%** (ganho qualitativo via expansão estrutural
de `table`).

²⁵ — Ajuste P157B (Tabela B): 53 → **54** (+`TableCell`).
Vanilla extra ausentes desce de ~1-2 para ~1 (TableCell sai
do conjunto não-capturado). Décima primeira aplicação consecutiva
de materialização; segundo sub-passo Model Fase 2. ADR-0060
Decisão 4 respeitada (variant dedicado para sub-elemento
estrutural). **Patamar empírico ADR-0064**: Caso A N=4
(primeira aplicação Model); Caso C N=3 (primeira variação
`usize`). Helper novo `extract_usize_or_none_min` privado em
`stdlib/structural.rs` (combina parse `x`/`y` com min=0 e
`colspan`/`rowspan` com min=1).

²⁶ — Ajuste P157C (Tabela A.6): 2 novas entradas `table.header`
e `table.footer` adicionadas como **sub-entradas de `table`**
(não contam na agregação Model — paridade decisão P157B).
Estado `parcial`: subset minimal (2 fields cada) materializado
per ADR-0054 graded; algoritmo de repetição em page breaks
diferido em **DEBT-56** (`repeat: bool` armazenado mas ignorado).
**Naming `table_header`/`table_footer` flat** (paridade P157B —
FieldAccess actual não suporta namespacing de funcs; divergência
intencional per ADR-0033). **Primeira aplicação concreta de
ADR-0064 Caso D em domínio Model** (P156D weak / P156G breakable
/ P156J justify aplicaram-no em Layout). **Saturação cross-domínio
cross-caso**: após P157C, **todos os 4 casos canónicos** ADR-0064
(A/B/C/D) têm pelo menos 1 aplicação concreta em Layout E em Model.
Patamar empírico atinge maturidade. **Divergência aceite per
ADR-0033**: `body: Box<Content>` em vez de vanilla `Vec<TableItem>`
para uniformidade com containers cristalinos existentes;
`level`/`repeat-rows` scope-out per ADR-0054 graded. **Helper
novo `extract_bool_with_default(args, fn, field, default)`**
privado em `stdlib/structural.rs` parametrizado (genérico no
key e no default vs `extract_weak` específico para
key="weak"/default=false). Contagem agregada Model **inalterada**
(7/4/5/6/0=22) — sub-entradas qualitativas. Cobertura Model
permanece **50%**. **"Table foundations" declarado em ADR-0060
§"Decisão 1" sub-passo 3 fica integralmente fechado** com
P157A + P157B + P157C (3 sub-passos M cada; granularidade
preservada N=10/11/12).

²⁷ — Ajuste P157C (Tabela B): 54 → **56** (+`TableHeader` +
`TableFooter` par simétrico). **Vanilla extra ausentes desce
de ~1 para 0** — após P157C, todos os variants Content vanilla
relevantes a "table foundations" estão capturados. Décima
segunda aplicação consecutiva de materialização. Patamar
empírico cross-domínio cross-caso ADR-0064 atinge **saturação**:
todos os 4 casos canónicos validados em Layout E em Model.

²⁹ — Ajuste P159A (Tabela A.6 Model): `cite` e `bibliography`
transitam ambos `ausente → parcial` per estrutura A adaptada
do diagnóstico P159 §3.5 (par acoplado num único passo M+ sem
hayagriva). Subset minimal per ADR-0054 graded:
- `Content::Bibliography { entries: Vec<BibEntry>, title:
  Option<Box<Content>> }` + stdlib `#bibliography(entries,
  title: ?)`. Tipo entity novo `BibEntry { key, author, title,
  year }` em `entities/bib_entry.rs`. **Input cristalino literal**
  — sem hayagriva (ADR-0062 mantém-se reserva sem ficheiro);
  refinos futuros (CSL parsing, form variants, numbering) NÃO
  reservados per política P158.
- `Content::Cite { key: String, supplement: Option<Box<Content>> }`
  + stdlib `#cite(key, supplement: ?)`. **Sem validação
  cross-reference** `key ∈ entries` per ADR-0017 Introspection
  runtime adiada — `cite("inexistente")` produz placeholder
  sem erro.
**Naming `bibliography` e `cite` flat** (paridade decisão P157B
naming flat — FieldAccess actual não suporta `Value::Func.subname`).
**ADR-0064 Caso A** aplicado em title (Bibliography) e
supplement (Cite) — patamar Caso A cresce **N=4 → 5** com
P159A (P156G/H/I + P157B + P159A; diversidade cross-domínio
60% Layout + 40% Model). **ADR-0065 critério #2 (escolha de
tipo) primeira aplicação isolada concreta** — decisão de
`BibEntry` 4 fields minimais documentada. Layouter renderiza
placeholder per ADR-0033 + ADR-0054 graded — Bibliography como
lista; Cite como `[key]` + supplement. **Granularidade quebrada
honestamente** N=13 → M+ com precedente P156C par lógico
pad+hide. Contagem Model: 7/4/5/6/0=22 → **7/4/7/4/0=22**
(2 entradas movidas `ausente → parcial`). Cobertura Model
(impl + impl⁺): (7+4)/22 = **50% inalterada**; ganho qualitativo
via 2 entradas saírem de `ausente`. Total user-facing: 64/22/22/
31/2=141 → **64/22/24/29/2=141**. **DEBT-55 contribuído mas
NÃO fechado** — pré-condição hayagriva contornada para subset
minimal; refinos futuros (hayagriva integration, CSL, form
variants, cross-document forward refs) **NÃO reservados**.
Tests +27 (1147 → 1174). **Política "sem novas reservas"
preservada**.

³⁰ — Ajuste P159A (Tabela B): 56 → **58** (+`Bibliography` +
`Cite` par acoplado). **Vanilla extra ausentes mantém 0** —
após P159A, todos os variants Content vanilla relevantes a
"bibliography + cite" subset minimal estão capturados (refinos
futuros são extensões de fields, não variants novos). Décima
quarta aplicação consecutiva de materialização. **Primeiro M+
par acoplado pós-P156C** (P156C era passo aditivo simétrico;
P159A é par funcional inseparável).

²⁸ — Ajuste P158A (Tabela A.6 Model): refino qualitativo de
`figure` — auto-detecção de `kind` baseada no body adicionada
em `native_figure` per diagnóstico P158A §3.2. Helper privado
novo `infer_kind_from_body(body) -> Option<String>` em
`stdlib/figure_image.rs` cobrindo Image/Table/Raw + recursão
limitada a `Content::Sequence` (paridade vanilla parcial per
ADR-0033). Fallback chain 3 níveis: `kind:` explícito > inferência
> default `"image"` (precedência absoluta para `kind:` explícito
preserva tests pré-existentes). **Sem alteração ao variant
`Content::Figure`** (estrutura inalterada; `kind: String`
continua directo). **Sem alteração a `introspect.rs` ou layout**
(counters por kind continuam funcionar inalterados — refino vive
só na origem do valor `kind`). **Hash `entities/content.rs`
preservado** `ec58d849` — sétimo passo consecutivo (P156L →
P158A) sem alteração ao variant Content. ADR-0064 NÃO directamente
aplicável (kind continua String). Cobertura Model agregada
**inalterada** (~50%) — refino qualitativo. Tests +6 (1141 →
1147). **Política "sem novas reservas"** preservada (P158
estabeleceu; P158A respeita) — supplement automático, show
selectors `figure.where(kind:)`, refactor `kind: String →
Option<String>` permanecem candidatos NÃO-reservados.

⁴⁰ — Ajuste P217+P218+P219 (Tabela A.5 Layout — Fase 3
sub-fase b 3/4 sub-passos materializados pós-DEBT-56
decomposto em P215.div-1):
- P217: variant `Content::Columns { count: usize, gutter:
  Option<Length>, body: Box<Content> }` adicionado (Content
  variants 56 → 57); 10 arms exhaustivos em 4 ficheiros
  L1; arm Layouter stub transparente; 6 tests novos.
- P218: stdlib `native_columns(count, body, gutter: ?)`
  registada (~53 → 54 funcs); helper `extract_count` privado
  (N=1); 12 tests novos.
- P219: arm `Content::Columns` em `layout_content` substituído
  por **consumer real graded (Opção B paridade ADR-0054)** —
  width temporariamente reduzida `(full_width - (count-1)*gutter)
  / count`; body single-render; width restaurada. Default
  gutter ~4% via constante `COLUMNS_DEFAULT_GUTTER_RATIO`
  (anti-inflação 14ª aplicação cumulativa). Decisão P216B
  preservada — `Regions { current }` minimal mantido;
  `backlog`/`last` continuam diferidos. **Multi-region
  flow real ausente** (Opção A diferida a P-Layout-Fase4
  candidato). 8 tests E2E novos. Pre-existing tests
  preservados (zero regressão).
Reclassificação §A.5 `columns(n)`: **`ausente` →
`parcial`** (variant + stdlib + arm real existem; multi-region
flow real scope-out per ADR-0054 graded; reclassificação a
`implementado` exige Opção A futura).
Distribuição Layout pós-recálculo: `13/1/3/1/0 = 18` →
**`13/1/4/0/0 = 18`** (1 ausente → parcial; 0 ausente
restantes em Layout). Cobertura `(impl + impl⁺)/total =
14/18 = **78% preservada**` (parcial fora numerador per
metodologia §A.9 P213; ganho qualitativo via 1 ausente
eliminado).
**Total user-facing**: `69/24/25/21/2 = 141 → 69/24/26/20/2
= 141` (1 entrada movida ausente → parcial; total
preservado). Cobertura user-facing total: `(69+24)/141 =
65.96% ≈ **66% preservada**` (idem metodologia).
**Sub-fase (b) DEBT-56**: 3/4 sub-passos materializados;
P220 colbreak pendente; P221 fecho pendente. Tests
workspace: 1952 → 1972 verdes (+20 cumulativo P217+P218+P219).
ADR-0078 mantém PROPOSTO.

⁴¹ — Ajuste P220 (Tabela A.5 Layout — Fase 3 sub-fase b
4/4 sub-passos materializados pós-DEBT-56 decomposto em
P215.div-1; **fecha sub-fase (b) estructuralmente**):
- P220: variant `Content::Colbreak { weak: bool }` adicionado
  (Content variants 55 → 56); 8 arms exhaustivos em 4
  ficheiros L1 (paridade Pagebreak P156E mas leaf sem `to`
  porque vanilla `ColbreakElem` não tem); `native_colbreak(weak:
  ?)` stdlib registada (~54 → 55 funcs); arm Layouter
  `layout_content` Opção β graded — **downgrade a pagebreak
  literal** (paridade vanilla quando fora de columns context);
  reusa `Layouter::new_page` (paridade P156E literal — zero
  refactor estrutural); `weak` armazenado mas semantic adiada
  (paridade P156D/E). 15 tests novos (5 unit content + 6 unit
  stdlib + 4 E2E layout).
- **Sub-passo agregado único** (variant + arm + stdlib em
  P220 vs P217+P218+P219 atomizados) — paridade literal
  P156E pagebreak. **Anti-inflação 15ª aplicação cumulativa**
  (atomização ADR-0036 não é dogma absoluto; quando arm é
  trivial, agregação economiza overhead).
- L0 `entities/content.md` extensão Opção γ (sem secção
  dedicada; convenção emergente "L0 minimal para refactors"
  consolidada N=4 com P217+P218+P219+P220).

Reclassificação §A.5 `colbreak()`: **`ausente` → `parcial`**
(variant + stdlib + arm real existem; multi-region salto
entre colunas reais ausente — P-Layout-Fase4 candidato per
Opção A; reclassificação a `implementado` exige flow real).

**Recontagem Layout pós-auditoria empírica P220** (footnote ⁴⁰
contou erradamente Layout como `13/1/4/0/0=18` quando o
real era `12/1/4/1/0=18` — colbreak permaneceu em `ausente`
até P220; off-by-one corrigido aqui transparentemente):
`12/1/4/1/0 = 18 → **12/1/5/0/0 = 18**` (1 ausente → parcial;
**zero ausentes em Layout pós-P220**). Cobertura
`(impl + impl⁺)/total = 13/18 = **72% preservada**` (parcial
fora numerador per metodologia §A.9 P213; Layout inflama-se
qualitativamente via 2 reclassificações cumulativas
P219+P220 sem alteração quantitativa; ganho estructural
real é "zero ausentes").

**Total user-facing**: recálculo cumulativo (corrige offset
+1/-1 footnote ⁴⁰ + reclassifica colbreak): `69/24/26/20/2
= 141 → **68/24/27/20/2 = 141**` (1 entrada Layout movida
implementado para parcial pela correcção de auditoria;
mais 1 entrada movida ausente para parcial pela
reclassificação P220; net result `-1 implementado, +1
parcial` empírico). Cobertura user-facing total:
`(68+24)/141 = 65.25% ≈ **65% preservada**` (idem
metodologia; declínio de 1 ponto percentual reflecte
correcção de auditoria, não regressão semântica).

**Sub-fase (b) DEBT-56 FECHADA estructuralmente**:
**4/4 sub-passos materializados** (P217 ✓, P218 ✓, P219 ✓,
P220 ✓). P221 é encerramento documental (ADR-0078 PROPOSTO
→ IMPLEMENTADO + DEBT-56 fecha). Tests workspace: 1972 →
**1987 verdes** (+15 P220). ADR-0078 mantém PROPOSTO
(transição em P221).

⁴² — Ajuste P221 (encerramento Fase 3 Layout — consolida
P217+P218+P219+P220+correcção retroactiva auditada P220):
P221 é **passo documental puro** — zero código tocado;
transita 2 ADRs e fecha 1 DEBT estructuralmente.

**Reclassificações cumulativas §A.5** (reapresentação
consolidada ⁴⁰+⁴¹):
- `columns(n)`: ausente → **parcial** (P217 variant + P218
  stdlib + P219 consumer real graded Opção B; multi-region
  flow real Opção A scope-out documentada).
- `colbreak()`: ausente → **parcial** (P220 agregado:
  variant + stdlib + arm Layouter Opção β graded — downgrade
  a pagebreak; multi-region salto entre colunas reais
  scope-out).

**Distribuição §A.5 final pós-P221**: `12/1/5/0/0 = 18`
(zero ausentes em Layout — paridade descoberta empírica
P220 preservada). Cobertura Layout: `13/18 ≈ **72% per
metodologia §A.9 P213**` (parcial fora numerador estricto;
**78% per paridade visual histórica** preservada como Opção
γ blueprint §2.1 com nota explícita "12 impl + 5 parcial").

**Tabela B Content variants pós-P221**: 54 → **56** (+Columns
P217 + Colbreak P220). Linha "Vanilla-only (ausentes)"
actualizada — `ColumnsElem` e `ColbreakElem` removidos
(transitam para `parcial`).

**Stdlib funcs pós-P221**: 53 → **55** (+native_columns
P218 + native_colbreak P220).

**Tests workspace cumulativo P216A-P220**: 1939 → **1987**
(+48 cumulativo: +7 P216A + +3 P216B + +6 P217 + +12 P218 +
+8 P219 + +15 P220; +0 P221 documental). 0 violations
preservadas. "Nothing to fix" lint hashes (L0 não tocado
em P221).

**Multi-region flow real Opção A scope-out** per ADR-0078
§"Decisão" sub-fase (b) — Fase 4 Layout candidata
NÃO-reservada per política P158. Refinos `measure(body)`
stdlib expose + `place` float/clearance ficam como Fase 4
candidatos isolados (sub-passos S+ cada).

**Transições ADR**:
- ADR-0078 column flow algorithm: PROPOSTO (P215) →
  **IMPLEMENTADO** (P221). 6 condições §"Plano materialização"
  satisfeitas explicitamente.
- ADR-0061 Layout Fase X roadmap: PROPOSTO (P156B) →
  **IMPLEMENTADO** (P221). Caminho 1 100% cumprido (Fase 1
  4/4 + Fase 2 3/3 + Fase 3 sub-passo 1 + sub-fase a 2/2 +
  sub-fase b 4/4).

**DEBT-56 ENCERRADO** (P221) — CLOSED via materialização;
critério §"Critério de fecho" 5/5 cumprido. Saldo DEBTs:
14 → **13 abertos**.

**Distribuição ADRs cumulativa**: PROPOSTO 13 → **11**;
IMPLEMENTADO 19 → **21**.

Footnotes ⁴⁰ + ⁴¹ preservadas (paridade pattern P204H+
"histórico textual preservado") para rastreabilidade
P217-P220 incremental.

⁴³ — Ajuste P222 (Tabela A.5 Layout — Fase 4 Layout
candidata sub-passo 1; ADR-0066 §"Plano promoção" Bloco C
cross-módulo primeira materialização parcial):
- P222: stdlib `#measure(body) -> dict(width: length,
  height: length)` exposta — `native_measure` em
  `rules/stdlib/layout.rs` (~70 LOC + 11 unit tests).
  Helper privado `measure_content` em
  `rules/layout/helpers.rs` promovido `pub(super)` →
  `pub(crate)`; módulo `helpers` promovido a `pub(crate)`
  (visibility expansion mínima cross-module crate).
- Stdlib funcs: 55 → **56** (+native_measure). Scope
  register `eval/mod.rs` após `colbreak` (paridade ordem
  P220).
- **Opção β graded fixada** (vs Opção α width override
  paridade vanilla completa; vs Opção γ silently ignore):
  width override `measure(body, width: 5cm)` **rejeitado
  explicitamente** com mensagem clara documentando
  scope-out per ADR-0054 graded; refino futuro candidato
  NÃO-reservado per política P158.
- **Retorno Opção α fixada** — `Value::Dict { "width":
  Length, "height": Length }` (vs Value::Size novo).
  Paridade vanilla observable `measure(body).width`
  literal funcional via Dict indexing.
- **Limitação do helper documentada**: `measure_content`
  retorna `(0, 0)` aproximação conservadora para texto
  multi-linha + equações + heading + etc.; suporte real
  só para `Shape::Rect/Ellipse/Path/Line` + `Sequence`
  composição. Refino sub-feature de Fase 4 candidata.
- 11 tests adicionados em `stdlib/mod.rs` (9 unit + 2
  integração unit-as-E2E — spec C5 propôs 2 E2E layout
  mas parse+eval harness não existe em `layout/tests.rs`;
  decisão pragmática: 2 tests em stdlib/mod.rs testando
  Sequence composição + round-trip Dict indexing). Tests
  workspace: 1987 → **1998 verdes** (+11). 0 regressões.

Reclassificação §A.5 `measure(body)`: **`parcial` →
`implementado⁺`** (asterisco reflecte limitação width
override + helper aproximação conservadora para conteúdo
complexo; runtime queries genuínas continuam diferidas
per ADR-0066).

Distribuição Layout pós-P222: `12/1/5/0/0 = 18 →
**12/2/4/0/0 = 18**` (1 parcial → impl⁺; zero ausentes
preservado). Cobertura Layout per metodologia §A.9 P213:
`(12+2)/18 = **78%**` real ✓ — paridade com Opção γ §2.1
blueprint histórica (coincidência aritmética agradável).

**Total user-facing**: `68/24/27/20/2 = 141 →
**68/25/26/20/2 = 141**` (1 parcial → impl⁺). Cobertura
user-facing: `(68+25)/141 ≈ **66%**` (+1pp real).

**ADR-0066 anotada sem promoção** (3 condições §"Plano
promoção" não satisfeitas: state(), 2-pass pipeline,
E2E feature observable). Bloco C cross-módulo primeira
materialização parcial — runtime queries genuínas
continuam diferidas. **ADR-0061 anotada Fase 4 candidata
sub-passo 1/3** (P222 ✓; P223 place pendente; P224 grid
refino pendente — Opção α P221 §8).

**Pattern emergente "L0 minimal para refactors" N=4 →
**5** (P217+P218+P219+P220+**P222** todos Opção γ; sem
extensão L0 stdlib.md). Promoção formal ADR meta
documental fica como decisão diferida — Caminho 4 P221
§8 candidato; política consistente N=3-4 mínima
ultrapassada.

**Pattern emergente "Fase 4 candidata reclassifica
parcial → impl⁺" N=1 inaugurado** (paridade Fase 3 que
reclassificou ausente → parcial em P219+P220; pattern
"reclassificação qualitativa pós-fecho Fase").

**Coincidência aritmética agradável**: 78% per metodologia
rígida (P213) agora coincide com 78% per paridade visual
histórica (Opção γ §2.1 blueprint) — Layout fechou esta
"divergência metodológica" qualitativa via materialização
real P222. **Opção γ blueprint §2.1 mantém-se literal
"12 impl + 5 parcial" como nota histórica** — actualização
para "12 impl + 2 impl⁺ + 4 parcial" fica diferida (S
documental se humano priorizar).

⁴⁴ — Ajuste P223 (Tabela A.5 Layout — Fase 4 Layout
candidata sub-passo 2; refino aditivo a variant existente):
- P223: `Content::Place` refino aditivo +2 fields graded
  — `float: bool` (default `false`; semantic real adiada
  per ADR-0054 graded; pattern N=3 → 4 cumulativo
  weak/breakable/float) + `clearance: Option<Length>`
  (default `None`; depende `float: true` real; paridade
  Smart→Option N=6 → 7 cumulativo). Arms cascata em ~6
  sítios L1 (5 em content.rs PartialEq/map_content/
  map_text/Eq pattern; 1 em introspect.rs
  materialize_time; 1 em layout/mod.rs layout_content com
  `float: _, clearance: _` ignorados). `native_place`
  stdlib +2 named args extraction + validation; reuso
  `extract_length` helper N=8 → 9.
- **DEBT-37 §"Divergência face ao vanilla" fechada** —
  Decisão 3 Opção α restaurada: `place(scope: "parent")`
  sem `float: true` agora rejeitado com erro hard
  (paridade vanilla literal restaurada; mensagem cristalino
  explicitamente referencia DEBT-37). 1 test pre-existente
  P84.6 adaptado (`place_dentro_de_grid_com_scope_parent_ancora_a_pagina`
  em `03_infra/integration_tests.rs`) adicionando `float:
  true` — paridade visual preservada literal (semantic
  real adiada).
- **3 decisões fixadas**: Opção β float armazenado +
  semantic adiada; Opção β clearance idem; Opção α DEBT-37
  restrição vanilla restaurada.
- 14 tests novos (4 unit content + 8 unit stdlib + 2 E2E
  layout). Tests workspace: 1998 → **2012 verdes** (+14).
  1 adaptação DEBT-37 (regressão intencional documentada).
  0 regressões reais.

Reclassificação §A.5 `place(...)`: **`parcial ⁵` →
`implementado⁺ ⁵ ⁴⁴`** (asterisco reflecte limitação flow
real adiada + DEBT-37 fecho documental).

Distribuição Layout pós-P223: `12/2/4/0/0 = 18 →
**12/3/3/0/0 = 18**` (1 parcial → impl⁺; zero ausentes
preservado). **Cobertura Layout per metodologia §A.9
P213**: `(12+3)/18 = **83%**` real (+5pp vs P222 78%;
+11pp cumulativo Fase 4 P222+P223; **divergência com
paridade visual histórica reaberta** — §2.1 blueprint
ainda lista 78% baseline Opção γ; possível actualização
diferida).

**Total user-facing**: `68/25/26/20/2 = 141 →
**68/26/25/20/2 = 141**` (1 parcial → impl⁺). Cobertura
user-facing: `(68+26)/141 ≈ **67%**` (+1pp real).

**Pattern emergente "refino aditivo a variant existente"
N=1 inaugurado pós-M9c** — distinto de variant novo
(P217 Columns + P220 Colbreak) e de stdlib expose existente
(P222 measure). Pattern reusável para `Block.fill`,
`Boxed.stroke`, etc. (refinos atributos vanilla scope-out
P156G+H+I).

**Pattern emergente "Field armazenado semantic adiada"
N=3 → 4 cumulativo** — P156D `weak`/P156E `weak`/P156G
`breakable`/**P223 `float`**. N=4 atinge limiar
formalização N=3-4 ultrapassado; Caminho 4 candidato
sólido (P221 §8 ADR meta).

**Pattern emergente "L0 minimal para refactors" N=5 →
**6**** (P217+P218+P219+P220+P222+**P223** todos Opção γ).
N≥6 patamar empírico extremamente sólido; promoção formal
fortemente justificada se humano priorizar Caminho 4.

**Pattern emergente "fecho de divergência documentada via
refino" N=1 inaugurado** — DEBT-37 §"Divergência" fechada
exactamente quando float adicionado (paridade comentário
DEBT-37 explícito "quando float for adicionado, repor a
restrição"). Pattern reusável para DEBTs com divergências
condicionais documentadas.

**Helper `extract_length` reuso N=8 → 9** — patamar
crescente; possível candidatura helper público crate-wide
se N ≥ 10 (não-promovido em P223 per política sem novas
reservas P158).

**Cobertura cumulativa pós-Fase 3 fechada P221** (Fase 4
candidata em curso 2/3 sub-passos): Layout 78% → 83% real
em 2 sub-passos (P222 + P223 = +11pp cumulativos). P224
grid refino pendente para completar Fase 4 candidata.

⁴⁵ — Ajuste P224 (Tabela A.5 Layout — **Fase 4 Layout
candidata sub-passo 3/3 — fecha série α "terminar Layout"
estructuralmente**; refino substantivo composto Opção δ):

- P224 (magnitude L cumulativa atomizada A/B/C):
  - **P224.A** — Grid variant +3 fields aditivos
    (`gutter: Option<Length>`, `align: Option<Align2D>`,
    `inset: Sides<Length>`). Semantic real adiada per
    ADR-0054 graded (gutter/align/inset ainda não aplicados
    em geometric layout — refino futuro candidato Fase 5
    NÃO-reservada).
  - **P224.B** — 2 variants Content novos:
    `Content::GridHeader { body, repeat }` + `Content::GridFooter
    { body, repeat }` (paridade P157C TableHeader/Footer literal).
    `repeat: bool` armazenado mas semantic adiada (pattern N=5
    cumulativo weak/breakable/float/repeat).
  - **P224.C** — 1 variant Content novo: `Content::GridCell
    { body, x, y, colspan, rowspan }` (paridade P157B
    TableCell literal). **Módulo L1 novo `01_core/src/rules/
    layout/grid_placement.rs`** (264 LOC) com
    `place_cells(cells, num_cols)` que implementa algoritmo
    placement vanilla paridade (auto linear + explicit x/y +
    colspan/rowspan + conflito detection). **Fecha DEBT-34e**.

- **Variants Content cumulativos**: 56 → **59** (+3:
  GridHeader, GridFooter, GridCell).

- **Stdlib funcs cumulativas**: 56 → **59** (+3:
  `native_grid_header`, `native_grid_footer`,
  `native_grid_cell`; `native_grid` refinada +5 named args).

- **DEBT-34e ENCERRADO P224** (CLOSED via materialização;
  critério 5/5 cumprido). **DEBT-34d preservado aberto**
  (Auto track sizing greediness — refino algorítmico
  distinto não endereçado per `P224.div-1` documentado;
  spec esperava fecho dual mas auditoria empírica revela
  que DEBT-34d é problema independente de track sizing,
  não placement).

- **Saldo DEBTs**: 13 → **12 abertos** (DEBT-34e fecha;
  DEBT-34d preservado).

- **stroke/fill cosméticos scope-out** explicit per ADR-0054
  graded — atributos Grid + per-cell rejeitados como named
  args desconhecidos. Refinos futuros candidatos Fase 5
  NÃO-reservados.

- Reclassificação §A.5 `grid(...)`: **`parcial ⁵` →
  `implementado⁺ ⁵ ⁴⁵`**.

- Distribuição §A.5: `12/3/3/0/0 = 18 → **12/4/2/0/0 = 18**`
  (1 parcial → impl⁺; zero ausentes preservado).

- **Cobertura Layout per metodologia §A.9 P213**:
  `(12+4)/18 = **89%**` real (+6pp vs P223 83%; **+17pp
  cumulativo Fase 4** P222+P223+P224).

- **Total user-facing**: `68/26/25/20/2 = 141 →
  **68/27/24/20/2 = 141**` (1 parcial → impl⁺). Cobertura
  user-facing: `(68+27)/141 ≈ **67%**` preservada
  (Layout não é maioria do user-facing).

- **27 tests P224 cumulativos adicionados** (~37
  hipotetizados spec; magnitude real reduzida):
  - 8 unit content (`p224_grid_variant_aceita_5_fields_aditivos`
    + `_partial_eq` + `_gridheader_variant` + `_gridfooter` +
    `_gridheader_is_empty_proxy` + `_gridcell_variant_5_fields` +
    `_gridcell_partial_eq` + `_gridcell_map_content`).
  - 10 unit stdlib (`p224_native_grid_aceita_gutter` +
    `_gutter_negativo_rejeita` + `_inset_uniforme` +
    `_header_footer_content` + `_named_arg_desconhecido_rejeita` +
    `_native_grid_cell_body_obrigatorio` +
    `_x_y_colspan_rowspan_aceita` + `_colspan_zero_rejeita` +
    `_native_grid_header_aceita_body` +
    `_native_grid_footer_aceita_body_repeat_false`).
  - 7 unit placement (em `grid_placement::tests`: auto
    linear, explicit x/y, colspan adjacente, rowspan
    adjacente, conflito rejeitado, colspan excede num_cols,
    mistura auto+explicit).
  - 2 E2E layout (`p224_grid_com_header_footer_renderiza_body` +
    `_gridcell_isolado_renderiza_body`).

- Tests workspace: 2012 → **2039 verdes** (+27 P224).
  0 violations preservadas. 0 regressões reais (sem
  adaptações pre-existentes necessárias — Table delegate
  preservado simples passa defaults).

- **Pattern emergente "L0 minimal para refactors" suspenso
  N=6 → 7** (P217+P218+P219+P220+P222+P223+**P224** todos
  Opção γ — divergência consciente vs spec C6 Opção α; L0
  `entities/content.md` preservado intocado per pattern
  consolidado N≥6). Promoção formal a ADR meta documental
  Caminho 4 candidato sólido se humano priorizar pós-P225.

- **Pattern emergente "Field armazenado semantic adiada"
  N=4 → 5** (P156D + P156E + P156G + P223 + **P224
  repeat Header/Footer**). N=5 patamar empírico forte.

- **Pattern emergente "fecho cumulativo de DEBTs via
  refino composto" N=1 inaugurado** — DEBT-34e fechada
  via P224.C placement algorítmico. Distinto de DEBT-37
  (refino aditivo single P223) + DEBT-56 (série composta
  P217-P220 encerrada P221).

- **Pattern emergente "subset Fase agregado L cumulativo
  pós-M9c" N=2** — P218+P220 agregados triviais; **P224
  primeiro agregado substantivo (L) com atomização
  interna A/B/C explícita**.

- **`P224.div-1`** registado: spec hipótese "fecha
  DEBT-34d/e simultaneamente" divergente da realidade
  empírica — DEBT-34d (Auto track sizing greediness) é
  problema algorítmico distinto de placement, não
  endereçável pelo módulo `grid_placement.rs`. Apenas
  DEBT-34e fecha; DEBT-34d preservado em aberto como
  refino futuro candidato Fase 5.

**Cobertura cumulativa Fase 4 fechada estructuralmente**:
Layout 78% (Fase 3 fechada P221) → 83% (P223) → **89%
pós-P224** (+17pp cumulativos Fase 4). **Série α "terminar
Layout" fechada 3/3 sub-passos**. P225 será encerramento
documental (paridade P221 para Fase 3).

⁴⁶ — Ajuste P225 (encerramento Fase 4 Layout candidata —
consolida P222+P223+P224+correcção retroactiva auditada
P224.div-1): **P225 é passo documental puro** — zero código
tocado; sem novas transições ADR; sem novos DEBTs fechados.
Anotação cumulativa final série α "terminar Layout".

**Trajectória completa pós-M9c Fase 4 Layout candidata**:
- **P222** `native_measure` stdlib expose graded (Bloco C
  ADR-0066 primeira materialização parcial; helper
  visibility promotion `pub(super)` → `pub(crate)`).
- **P223** `Content::Place` refino +2 fields (`float`/
  `clearance` semantic adiada; pattern N=4 cumulativo
  weak/breakable/float; DEBT-37 §"Divergência" fechada
  via Decisão 3 Opção α restauração paridade vanilla).
- **P224** `Content::Grid` refino substantivo composto +5
  fields + 3 variants Content novos (GridHeader/GridFooter/
  GridCell paridade P157C/B literal) + módulo L1 novo
  `01_core/src/rules/layout/grid_placement.rs` (264 LOC com
  `place_cells` algoritmo placement vanilla paridade).
  DEBT-34e ENCERRADO via P224.C placement algorítmico
  (critério 5/5 cumprido). **DEBT-34d preservado aberto
  per `P224.div-1`** — refino algorítmico track sizing
  greediness distinto, não endereçável por placement work
  (auditoria empírica C1 detectou e ajustou plano
  transparentemente).

**Cumulativo Fase 4** (3 sub-passos):
- 3 variants Content novos cumulativos (GridHeader +
  GridFooter + GridCell em P224); Content variants count
  56 → **59**.
- +7 fields refino a 2 variants existentes (Place +2 P223;
  Grid +5 P224).
- 4 stdlib funcs novas (`native_measure` P222 +
  `native_grid_cell` + `native_grid_header` +
  `native_grid_footer` P224); Stdlib funcs count 55 →
  **59**.
- 2 stdlib refinadas (`native_place` +2 named args P223;
  `native_grid` +5 named args P224).
- 1 helper visibility promotion (`measure_content` P222).
- 1 módulo L1 novo (`grid_placement.rs` P224.C 264 LOC).
- **2 DEBTs fechados** (DEBT-37 §"Divergência" via P223
  anotação histórica; DEBT-34e via P224 materialização
  CLOSED).
- **1 DEBT preservado aberto per `P224.div-1`** (DEBT-34d
  refino algorítmico track sizing distinto não endereçável
  por placement work; Fase 5 candidata NÃO-reservada).
- **0 ADR transitions na série α** (ADR-0061 já
  IMPLEMENTADO desde P221; ADR-0066 mantém PROPOSTO per
  pattern emergente N=1 "ADR PROPOSTO com materialização
  parcial graded" inaugurado P222).
- **52 tests cumulativos Fase 4** (P222 11 + P223 14 +
  P224 27); 1998 → **2039 verdes**.
- **3 reclassificações** §A.5: `measure` + `place` + `grid`
  parcial → impl⁺.

**Distribuição §A.5 Layout final pós-P225**:
**`12/4/2/0/0 = 18`** (zero ausentes preservado desde P220;
**3 entradas reclassificadas cumulativamente Fase 4**).
**Cobertura Layout per metodologia §A.9**: `(12+4)/18 =
**89%**` real (paridade visual histórica §2.1 Opção γ
**refrescada** pós-P225 para "89% (12 impl + 4 impl⁺ + 2
parcial)" — divergência metodológica visual vs real
**fechada via materialização cumulativa**).

**Total user-facing**: `**68/27/24/20/2 = 141**` (preservado
pós-P224). Cobertura: `(68+27)/141 ≈ **67%**` preservada
(Layout não é maioria do user-facing).

**Patterns emergentes cumulativos consolidados Fase 4**:
- **"L0 minimal para refactors"** N=5 → 6 → **7**
  (P222+P223+P224 todos Opção γ; P224 divergência
  consciente vs spec C6 Opção α reforçou em vez de
  suspender). N=7 patamar empírico **muito sólido**.
- **"Field armazenado semantic adiada"** N=3 → 4 → **5**
  (`weak`/`breakable`/`float`/`repeat`).
- **"ADR PROPOSTO com materialização parcial graded"** N=1
  inaugurado P222 (ADR-0066 mantém PROPOSTO).
- **"Refino aditivo a variant existente"** N=1 → **2**
  (P223 Place; P224.A Grid).
- **"Fecho de divergência documentada via refino"** N=1
  inaugurado P223 (DEBT-37 §"Divergência").
- **"Fecho cumulativo de DEBTs via refino composto"** N=1
  parcialmente inaugurado P224 (apenas DEBT-34e fecha;
  DEBT-34d preservado per `P224.div-1`).
- **"Subset Fase agregado L cumulativo pós-M9c"** N=1 →
  **2** (P218+P220 trivial; **P224 substantivo com
  atomização interna A/B/C explícita**).
- **"Divergência factual material registada como
  `Pxxx.div-N`"** N=1 → **2 cumulativo** (P215.div-1
  reabriu Fase 3 sub-fase b; **P224.div-1 preservou
  DEBT-34d**). Pattern de honestidade arquitectural
  consolidado P225.
- **"Consumer geometric integration deferido
  pós-algorítmico"** N=1 inaugurado P224.
- **"Encerramento Fase Layout pós-M9c"** N=1 → **2
  formalizado em P225** (P221 Fase 3; **P225 Fase 4**).

**Estado pós-P225**:
- Sub-fase (a) DEBT-56: 2/2 ✓ (P216A + P216B).
- Sub-fase (b) DEBT-56: 4/4 ✓ (P217-P220).
- DEBT-56 ENCERRADO (P221).
- **Fase 4 candidata 3/3** ✓ (P222-P224); **série α
  fechada estructuralmente E formalmente** em P225.
- Distribuição ADRs preservada P221: PROPOSTO 11
  (ADR-0066 inclusiva); IMPLEMENTADO 21.
- Saldo DEBTs: 14 (pre-M9c) → **12 abertos** (-2
  cumulativo: DEBT-56 P221; DEBT-34e P224; DEBT-34d
  preservado per `P224.div-1`).
- **Layout em estado terminal estructural** — refinos
  remanescentes são cosméticos (stroke/fill) ou exigem
  reabertura arquitectural maior (Opção A multi-region;
  Auto track sizing DEBT-34d; runtime queries genuínas;
  flow real Place float; per-cell GridCell atributos;
  consumer geometric integration P224.C). Fase 5 candidata
  NÃO-reservada per política P158.

**Política "sem novas reservas" preservada per P158** —
Fase 5 Layout candidata identificada mas NÃO reservada;
Opção A multi-region preservada como scope-out per
ADR-0078 IMPLEMENTADO; reservas conceptuais identificadas
mas não formalizadas como DEBTs ou ADRs novos.

Footnotes ⁴³ + ⁴⁴ + ⁴⁵ preservadas (paridade pattern
P204H+ "histórico textual preservado") para rastreabilidade
P222-P224 incremental. ⁴⁶ adiciona vista consolidada
cumulativa.

⁴⁷ — Ajuste P227 (Fase 5 Layout candidata Categoria A
sub-passo 1 — primeiro sub-passo Fase 5 materializado;
**valida ADR-0080 PROPOSTO N=7 → 8**):

- P227 materializa A.1 `stroke` Grid + Table:
  - **`Value::Stroke(Stroke)` variant novo** — primeira
    adição ao enum Value pós-M9c. Value variants: 54 →
    **55** (+Stroke).
  - **Grid +1 field** `stroke: Option<Stroke>` (8 → 9
    fields cumulativos pós-P224).
  - **Table +1 field** `stroke: Option<Stroke>` (3 → 4
    fields).
  - **Helper `extract_stroke(val, fn, field)`** novo em
    `stdlib/layout.rs` aceitando shorthands Length/Color/
    Stroke (Opção β paridade vanilla UX).
  - **`native_stroke(paint:?, thickness:?)` constructor**
    em `stdlib/layout.rs` (~70 LOC) — constructor stdlib
    nova; stdlib funcs 59 → **60**.
  - **`native_grid` + `native_table` accept `stroke:`**
    via `extract_stroke` shorthand.
  - **Renderização Opção β simplificada** em `layout_grid`:
    4 `FrameItem::Shape::Line` per cell border (top +
    bottom + left + right; sem deduplicação adjacentes;
    refino A.3 candidato).

- **6 decisões fixadas**:
  - Decisão 1 — Opção α `Option<Stroke>` uniforme (vs
    Sides per-side A.3; vs novo tipo GridStroke).
  - Decisão 2 — Opção β parsing Length/Color/Stroke
    shorthands (vs literal Value::Stroke apenas).
  - Decisão 3 — `Value::Stroke` variant novo (audit C1
    confirmou ausência; criado paridade `Value::Color`).
  - Decisão 4 — `native_stroke` constructor paridade
    `native_rgb` (não `native_pad` etc — constructor
    primário para Value type).
  - Decisão 5 — Opção β render simplificada (vs Opção α
    deduplicação A.3; vs Opção γ semantic adiada N=5).
  - Decisão 6 — Table refino paralelo Grid (variant-rico
    paridade).
  - Decisão 7 — ADR-0080 NÃO promover EM VIGOR em P227
    (P228 candidato administrativo XS dedicado).

- **L0 NÃO tocado** — Opção γ literal per ADR-0080
  PROPOSTO. **Pattern N=7 → 8 validado** — primeira
  aplicação real pós-formalização do pattern. **Promoção
  ADR-0080 PROPOSTO → EM VIGOR** candidato sólido P228
  administrativo XS.

- **Pattern emergente "refino aditivo paralelo entre
  variants irmãos" N=1 inaugurado P227** (Grid + Table
  recebem mesmo field paralelo; precedente futuro para
  Fase 5 sub-passos cosméticos A.2 fill + A.3 per-cell).

- 18 tests adicionados P227 (4 unit content Grid/Table +
  7 unit Value::Stroke/native_stroke/grid_stroke/table_stroke
  + 4 unit grid/table stroke + 3 E2E layout); workspace
  2039 → **2057 verdes** (+18). 4 adaptações intencionais
  (tests P224 pre-existentes adicionaram `stroke: None`
  ao construtor Grid/Table direct; 1 pattern adapt em
  stdlib/mod.rs P157A). 0 regressões reais.

- Sem reclassificação categórica §A.5 — `grid` já
  `implementado⁺` pós-P224 + P225. Footnote ⁴⁷ adiciona
  refino qualitativo: stroke scope-out cosmético fechado.

- **Patterns emergentes consolidados em P227**:
  - "L0 minimal para refactors" N=7 → **8 (validado real
    pós-ADR-0080 PROPOSTO)** — primeira validação
    empírica do ADR formalizado.
  - Pattern Smart→Option N=7 → **8** (`stroke:
    Option<Stroke>` paridade Smart→Option default None).
  - **Pattern "refino aditivo paralelo entre variants
    irmãos" N=1 inaugurado P227** — pattern reusável
    para Fase 5 sub-passos cosméticos seguintes A.2/A.3.
  - `extract_length` reuso N=9 → **10** (helper público
    candidato fortemente justificado — patamar N=10
    atingido; refino futuro candidato XS administrativo
    separado se humano priorizar).
  - **Anti-inflação 19ª aplicação cumulativa** pós-P205D
    (Opção α field uniforme + Opção β parsing graded +
    Opção γ L0 + Opção β render simplificada + sem
    helper construtor Rust novo + ADR-0080 não promover
    em P227).

- **Distribuição ADRs preservada P226**: PROPOSTO 13
  (ADR-0066, ADR-0079, ADR-0080); IMPLEMENTADO 21; total
  67.
- **Saldo DEBTs preservado**: 12 abertos (DEBT-34d
  preservado per `P224.div-1`; DEBT-34e ENCERRADO P224;
  DEBT-56 ENCERRADO P221).
- **Cobertura Layout per metodologia**: **89% preservado**
  (refino qualitativo P227; nota Grid+Table stroke
  materializado em footnote ⁴⁷).

**Categoria A Fase 5 Layout**: 1/5 sub-passos
materializados (A.1 stroke ✓; A.2 fill + A.3 per-cell +
A.4 Block/Boxed + A.5 Place per-cell pendentes).

⁴⁸ — Ajuste P228 (Fase 5 Layout candidata Categoria A
sub-passo 2 — **paralelo estructural P227**; **valida
ADR-0080 PROPOSTO N=8 → 9** segunda aplicação real
pós-formalização):

- P228 materializa A.2 `fill` Grid + Table:
  - **Grid +1 field** `fill: Option<Color>` (9 → 10
    fields cumulativos pós-P227).
  - **Table +1 field** `fill: Option<Color>` (4 → 5
    fields).
  - **Sem `Value::Fill` variant novo** — Color baseline
    P25 reusado (Decisão 1 Opção α + anti-inflação
    Decisão 3 Opção γ "NÃO criar constructor").
  - **`extract_color` helper ausente** — inline match
    trivial em `native_grid`/`native_table` (Decisão 2
    Opção α: apenas `Value::Color`).
  - **Renderização Opção β Z-order correcto** em
    `layout_grid`: 1 `FrameItem::Shape::Rect` per cell
    emitido **antes do conteúdo** + stroke per cell
    emitido **depois do conteúdo** (Z-order: fill →
    conteúdo → stroke).
  - **Audit C1 confirmou P227 stroke Z-order correcto**
    (emitido após `for item in cell_items`; Z-order
    correcto preservado). Sem `P228.div-N`.

- **7 decisões fixadas**:
  - Decisão 1 — Opção α `Option<Color>` uniforme (vs
    Sides per-side A.3; vs Paint enum criar — Paint
    scope-out ADR-0029).
  - Decisão 2 — Opção α parsing trivial Color directo.
  - Decisão 3 — Opção γ NÃO criar constructor stdlib
    (anti-inflação; Color tem `native_rgb`/`native_luma`).
  - Decisão 4 — Opção β Z-order correcto (fill antes;
    stroke depois).
  - Decisão 5 — Tests E2E Z-order para validar interacção
    P227+P228 (5 layout E2E tests).
  - Decisão 6 — Opção γ L0 NÃO tocado validação ADR-0080
    N=8 → 9.
  - Decisão 7 — ADR-0079 anotação Categoria A 2/5 (sem
    promoção).

- **L0 NÃO tocado** — Opção γ literal validação ADR-0080
  PROPOSTO. **Pattern N=8 → 9 atingido** — segunda
  aplicação real pós-formalização do pattern. Promoção
  EM VIGOR ADR-0080 candidato P229 administrativo XS
  fortemente justificado (N=9 ultrapassa critério N=8+).

- **Pattern emergente "refino aditivo paralelo entre
  variants irmãos" N=1 → 2 consolidado** (P227 stroke +
  P228 fill; Grid + Table recebem mesmo field paralelo
  ambos sub-passos).

- **Pattern emergente "anti-inflação por aproveitamento
  de tipos existentes" N=1 inaugurado P228** — distinto
  de P227 onde Stroke composto justificou
  `native_stroke`. Color primitivo dispensa constructor
  novo.

- 14 tests adicionados P228 (4 unit content + 5 unit
  stdlib + 5 E2E layout Z-order); workspace 2057 →
  **2071 verdes** (+14). 6 adaptações intencionais
  (tests P224+P227 pre-existentes adicionaram `fill:
  None` ao construtor Grid/Table direct). 0 regressões
  reais.

- Sem reclassificação categórica §A.5 — `grid` já
  `implementado⁺` pós-P224+P225+P227. Footnote ⁴⁸
  adiciona refino qualitativo: fill cosmético fechado.

- **Patterns emergentes consolidados em P228**:
  - "L0 minimal para refactors" **N=8 → 9 validado
    real** — segunda aplicação pós-ADR-0080 PROPOSTO;
    promoção EM VIGOR P229 candidato muito sólido.
  - Pattern Smart→Option **N=8 → 9** (`fill:
    Option<Color>` paridade Smart→Option default None).
  - **Pattern "refino aditivo paralelo entre variants
    irmãos" N=1 → 2 consolidado** (P227 stroke +
    P228 fill).
  - **Pattern "anti-inflação por aproveitamento de tipos
    existentes" N=1 inaugurado P228** (Color reusado;
    sem `native_fill` constructor).
  - **Anti-inflação 20ª aplicação cumulativa** pós-P205D
    (Opção α field + Opção α parsing trivial + Opção γ
    sem constructor + Opção β Z-order + Opção γ L0 +
    ADR-0080 não promover em P228).

- **Distribuição ADRs preservada P226**: PROPOSTO 13;
  IMPLEMENTADO 21; total 67.
- **Saldo DEBTs preservado**: 12 abertos.
- **Cobertura Layout per metodologia**: **89% preservado**
  (refino qualitativo P228; nota Grid+Table fill
  materializado).

**Categoria A Fase 5 Layout**: 2/5 sub-passos materializados
(A.1 stroke ✓; **A.2 fill ✓**; A.3 per-cell + A.4
Block/Boxed + A.5 Place per-cell pendentes).

⁴⁹ — Ajuste P230 (Fase 5 Layout candidata Categoria A
sub-passo 3 — stroke/fill per-cell GridCell + TableCell;
**primeira aplicação automática ADR-0080 EM VIGOR**
pós-promoção P229):

- P230 materializa A.3:
  - **GridCell +2 fields** `stroke: Option<Stroke>` +
    `fill: Option<Color>` (5 → 7 fields).
  - **TableCell +2 fields** stroke + fill paralelo
    GridCell (5 → 7 fields; refino paralelo).
  - **`native_grid_cell` + `native_table_cell` accept
    `stroke:` + `fill:` named args** via reuso helper
    `extract_stroke` P227 (N=1 → 2 cumulativo) + parsing
    inline Color paridade P228.
  - **Renderização precedência override** em
    `layout_grid`: `effective_stroke = cell.stroke.or(grid.stroke)`;
    `effective_fill = cell.fill.or(grid.fill)`. Per-cell
    `Some(...)` override Grid-level; per-cell `None`
    inherit Grid-level (paridade ADR-0033 observable
    literal).
  - **Z-order P227+P228 preservado integralmente**: fill
    efectivo atrás do conteúdo → conteúdo cell → stroke
    efectivo à frente.
  - **Refactor pragmático sem `PlacedCell` expandido**:
    `layout_grid` itera `cells: &[Content]` direct; match
    no loop extrai `cell_stroke`/`cell_fill` per-cell.
    Consumer geometric integration P224.C `place_cells`
    continua B.2 candidato Fase 5 distinto.

- **8 decisões fixadas**:
  - Decisão 1 — Opção α fields restritos (stroke + fill;
    align/inset/breakable per-cell são B.3 separado).
  - Decisão 2 — Opção α precedência override completo
    (paridade vanilla literal via `.or()`).
  - Decisão 3 — Opção α Z-order limpo cada cell uma vez.
  - Decisão 4 — Reuso helper `extract_stroke` N=1 → 2
    (sem promoção pública; patamar N=10 paridade
    `extract_length`).
  - Decisão 5 — Tests E2E precedência 5 explícitos.
  - Decisão 6 — Opção γ aplicação automática ADR-0080
    EM VIGOR sem decisão explícita Opção γ por sub-passo.
  - Decisão 7 — Opção α refino paralelo TableCell
    (pattern N=2 → 3 cumulativo).
  - Decisão 8 — `extract_stroke` reuso N=1 → 2.

- **L0 NÃO tocado** — **primeira aplicação automática
  ADR-0080 EM VIGOR** pós-promoção P229. Sem decisão
  explícita Opção γ por sub-passo; regra metodológica
  formal aplicada por defeito.

- **Patterns emergentes consolidados em P230**:
  - **"L0 minimal para refactors" — primeira aplicação
    automática pós-EM VIGOR** N=1 inaugurado P230.
    Precedente para refinos aditivos seguintes (regra
    formal sem decisão explícita).
  - **Pattern "refino aditivo paralelo entre variants
    irmãos" N=2 → 3 cumulativo** (Grid+Table P227/P228;
    **GridCell+TableCell P230**).
  - **Pattern emergente "precedência per-cell vs
    container-level via `.or()` resolution" N=1
    inaugurado P230** — reusável A.4 Block/Boxed +
    B.3 align/inset/breakable per-cell.
  - **Helper `extract_stroke` reuso N=1 → 2 cumulativo**
    (patamar trivial; promoção pública diferida).
  - **22 aplicações cumulativas anti-inflação** pós-P205D
    (Opção α fields restritos + Opção α precedência
    override + Opção γ L0 automático + helper reuso +
    refino paralelo variants irmãos + ADR-0079 sem
    promoção + sem refactor PlacedCell + sem promoção
    helper público).

- 15 tests adicionados P230 (4 unit content + 6 unit
  stdlib + 5 E2E precedência); workspace 2071 →
  **2086 verdes** (+15). Adaptações intencionais Grid
  constructor tests pre-existentes (N=5+: P224 unit
  tests P224.C GridCell + P224.C grid_placement.rs unit
  tests + P157B TableCell pattern; total N=~10
  adaptações). 0 regressões reais.

- Sem reclassificação categórica §A.5 — `grid` já
  `implementado⁺` cumulativo. Footnote ⁴⁹ adiciona
  refino qualitativo: per-cell cosmético com precedência
  fechado.

- **Distribuição ADRs preservada P229**: PROPOSTO 12;
  EM VIGOR 29; IMPLEMENTADO 21; total 67.
- **Saldo DEBTs preservado**: 12 abertos.
- **Cobertura Layout per metodologia**: **89% preservado**
  (refino qualitativo P230).

**Categoria A Fase 5 Layout**: 3/5 sub-passos materializados
(A.1 stroke ✓; A.2 fill ✓; **A.3 per-cell ✓**; A.4
Block/Boxed + A.5 Place per-cell pendentes).

⁵⁰ — Ajuste P231 (Fase 5 Layout candidata Categoria A
sub-passo 4 — outset/radius/clip Block + Boxed;
**segunda aplicação automática ADR-0080 EM VIGOR**
pós-promoção P229; reabertura formal P156G+H scope-outs):

- P231 materializa A.4:
  - **Block +3 fields** `outset: Sides<Length>` + `radius:
    Option<Length>` + `clip: bool` (5 → 8 fields).
  - **Boxed +3 fields** paralelo Block (5 → 8 fields;
    pattern "refino aditivo paralelo entre variants
    irmãos" N=3 → 4 cumulativo).
  - **`native_block` + `native_box` accept 3 named args**
    cada via parsing inline (`outset` Length uniforme;
    `radius` Length opcional; `clip` Bool); validações
    negativos rejeitados.
  - **Renderização Opção β parcial graded** (audit C1
    determinou primitivos baseline ausentes):
    - `outset`: armazenado em variant — semantic real
      (bounds visual expandidos) **adiada** porque
      requer refactor layouter cumulativo.
    - `radius`: **semantic real adiada** —
      `ShapeKind::RoundedRect` primitivo NÃO existe
      geometry.rs P76 (apenas Rect/Ellipse/Line/Path).
      Pattern N=5 → 6 cumulativo.
    - `clip`: **semantic real adiada** — wrap body em
      `FrameItem::Group { clip_mask: Some(ShapeKind::Rect) }`
      requer refactor estructural. Pattern N=6 → 7
      cumulativo.

- **7 decisões fixadas**:
  - Decisão 1 — Opção α escopo restrito (outset + radius
    + clip apenas; fill/stroke separados; spacing/above/
    below/sticky Categoria B candidato).
  - Decisão 2 — Opção α `Sides<Length>` outset (paridade
    `inset` baseline).
  - Decisão 3 — Opção β `Option<Length>` radius uniforme
    (vs `Corners<T>` cristalino NÃO existe; per-corner
    refino futuro candidato; pattern Smart→Option N=9 →
    10).
  - Decisão 4 — Opção α `bool` clip (paridade vanilla
    literal; pattern "Field bool simples" N=2 → 3
    cumulativo).
  - Decisão 5 — Opção β graded parcial (outset/radius/clip
    armazenados; semantic real adiada per audit C1).
  - Decisão 6 — Opção α refino paralelo Block + Boxed
    (pattern N=3 → 4 consolidado).
  - Decisão 7 — Opção γ L0 NÃO tocado automaticamente
    (**segunda aplicação automática ADR-0080 EM VIGOR**
    pós-promoção P229).

- **L0 NÃO tocado** — segunda aplicação automática
  ADR-0080 EM VIGOR pós-promoção P229. Sem decisão
  explícita Opção γ por sub-passo; regra metodológica
  formal aplicada por defeito.

- **Patterns emergentes consolidados em P231**:
  - **"L0 minimal para refactors" — aplicação automática
    pós-EM VIGOR** N=1 → **2 cumulativo** (P230 + P231).
  - **Pattern "refino aditivo paralelo entre variants
    irmãos" N=3 → 4 cumulativo** (Grid+Table P227/P228;
    GridCell+TableCell P230; **Block+Boxed P231**).
    N=4 patamar empírico **muito sólido**; promoção
    formal ADR meta candidato.
  - **Pattern "Field bool simples paridade vanilla" N=2
    → 3 cumulativo** (`breakable` P156G; `repeat` P224.B;
    **`clip` P231**). N=3 atinge limiar formalização
    N=3-4.
  - **Pattern Smart→Option N=9 → 10 cumulativo** (`radius:
    Option<Length>`). N=10 patamar empírico **muito
    sólido**; promoção formal candidato sólido (paridade
    `extract_length` N=10 candidato).
  - **Pattern "Field armazenado semantic adiada" N=5 →
    7 cumulativo** (+outset + radius + clip todos
    adiadas; +3 em P231: outset+radius+clip). N=7
    patamar empírico **muito sólido**; promoção formal
    candidato.
  - **Reabertura formal P156G+H scope-outs** documentados
    há 18 dias (criados 2026-04-25; reabertos
    2026-05-13). Pattern de continuidade arquitectural
    cumulativa.
  - **23 aplicações cumulativas anti-inflação** pós-P205D
    (Opção α escopo + Opção β graded + Opção α paralelo
    + Opção γ L0 automático + sem `Corners<T>` + sem
    `RoundedRect` + sem refactor Group clip_mask + ADR-0079
    sem promoção).

- 15 tests adicionados P231 (4 unit content + 9 unit
  stdlib + 2 E2E layout); workspace 2086 → **2101 verdes**
  (+15). Adaptações intencionais Block/Boxed constructors
  pre-existentes (N=4 em entities/content.rs +
  stdlib/mod.rs). 0 regressões reais.

- Sem reclassificação categórica §A.5 — `block`/`box`
  pre-existentes `implementado` (P156G/H baseline).
  Footnote ⁵⁰ adiciona refino qualitativo cosméticos
  cumulativo a A.1+A.2+A.3.

- **Distribuição ADRs preservada P229**: PROPOSTO 12;
  EM VIGOR 29; IMPLEMENTADO 21; total 67.
- **Saldo DEBTs preservado**: 12 abertos.
- **Cobertura Layout per metodologia**: **89% preservado**
  (refino qualitativo P231).

**Categoria A Fase 5 Layout**: 4/5 sub-passos materializados
(A.1 stroke ✓; A.2 fill ✓; A.3 per-cell ✓; **A.4
Block/Boxed cosméticos ✓**; A.5 Place per-cell pendente).
Após A.5 → Categoria A completa 5/5.

⁵¹ — Ajuste P232 (Fase 5 Layout candidata Categoria A
sub-passo 5 — Place per-cell alignment override;
**FECHA Categoria A 5/5 estructuralmente**; **terceira
aplicação automática ADR-0080 EM VIGOR** pós-P229; **dois
patterns emergentes novos N=1 inaugurados**):

- P232 materializa A.5:
  - **Zero fields novos** em Place/Grid/Table/Cell — pattern
    "sub-passo sem novos fields; só lógica precedence" N=1
    inaugurado P232. Refactor algorítmico puro.
  - **+1 field `cell_align: Option<Align2D>` no Layouter
    struct** (paridade `cell_origin_*` baseline P84.6;
    save/restore ao entrar/sair Grid context em layout_grid).
  - **Lógica precedência per-eixo via `.or()`** no arm
    `Content::Place` em `layout/mod.rs`:
    - `effective_h = alignment.h.or(grid_align.h)`.
    - `effective_v = alignment.v.or(grid_align.v)`.
    - Place explícito override Grid; Place vazio herda Grid.
    - Place fora Grid (cell_align None) preserva baseline
      P84.5 directo.
  - **Stdlib `native_place` NÃO modificado** — sintaxe
    utilizador preservada literal (paridade vanilla
    baseline).

- **Audit C1 findings**:
  - `Content::Table.align` field **NÃO existe** baseline
    (P224.A adicionou apenas Grid). Decisão pragmática
    (sem `P232.div-N` formal): P232 escopo limitado a Grid
    context; Table align paralelo é **refino XS futuro
    candidato** (não-reservado per política P158).
  - `cell_origin_*` save/restore pattern P84.6 confirmado
    em `grid.rs:232+`; P232 adicionou `cell_align`
    paralelo no scope Grid-level (não per-cell — align
    uniforme aplica-se a todas cells do Grid).

- **8 decisões fixadas**:
  - Decisão 1 — Opção α (lógica precedência; zero fields
    novos; paridade pattern P230 GridCell).
  - Decisão 2 — Opção α precedência por eixo independente
    via `.or()`.
  - Decisão 3 — Opção α inline no arm Place existente.
  - Decisão 4 — Opção α stdlib `native_place` preservado.
  - Decisão 5 — 5 tests E2E precedência explícitos.
  - Decisão 6 — Table.align audit C1: ausente baseline →
    escopo Grid only; Table refino XS futuro candidato.
  - Decisão 7 — Anotação ADR-0079 Categoria A 5/5 ✓
    fechada sem transição status (pattern "fecho
    categoria completa dentro de ADR PROPOSTO sem
    transição" N=1 inaugurado P232).
  - Decisão 8 — Opção γ L0 NÃO tocado automaticamente
    (**terceira aplicação automática ADR-0080 EM VIGOR**
    pós-promoção P229).

- **L0 NÃO tocado** — terceira aplicação automática
  ADR-0080 EM VIGOR pós-promoção P229. Pattern "aplicação
  automática ADR EM VIGOR sem decisão explícita por
  sub-passo" N=2 → **3 cumulativo** (P230 + P231 + P232).

- **Patterns emergentes consolidados em P232**:
  - **"L0 minimal para refactors" aplicação automática
    pós-EM VIGOR** N=2 → **3 cumulativo**.
  - **Pattern "precedência per-X via `.or()` resolution"
    N=1 → 2 cumulativo** (P230 GridCell over Grid;
    **P232 Place per-axis over Grid**).
  - **"Fecho categoria completa dentro de ADR PROPOSTO
    sem transição" N=1 inaugurado P232** — distinto de
    §3.0duodecies P221 + §3.0terdecies P225 que
    envolveram transições ADR PROPOSTO → IMPLEMENTADO.
  - **"Sub-passo sem novos fields; só lógica precedence"
    N=1 inaugurado P232** — distinto cumulativo de
    A.1-A.4 que adicionaram fields cumulativos (stroke/
    fill/outset/radius/clip/per-cell).
  - **24 aplicações cumulativas anti-inflação** pós-P205D
    (Opção α lógica não-field + Opção α inline arm +
    Opção α stdlib preservada + Opção γ L0 automático +
    sem helper + sem marco blueprint + sem promoção
    ADR-0079 + sem promoção patterns emergentes).

- 5 tests adicionados P232 (5 E2E layout precedência;
  spec planeou ~9 mas baseline preservation absorve
  maioria; subset pragmático suficiente para validar
  comportamento). Workspace: 2101 → **2106 verdes**
  (+5). Sem adaptações intencionais necessárias.
  0 regressões reais.

- Sem reclassificação categórica §A.5 — `place` já
  `implementado⁺` cumulativo P223+. Footnote ⁵¹ adiciona
  refino qualitativo precedência per-axis dentro Grid.

- **Distribuição ADRs preservada P229**: PROPOSTO 12;
  EM VIGOR 29; IMPLEMENTADO 21; total 67.
- **Saldo DEBTs preservado**: 12 abertos.
- **Cobertura Layout per metodologia**: **89% preservado**.

**Categoria A Fase 5 Layout: 5/5 ✓ FECHADA ESTRUCTURALMENTE**
(A.1 stroke ✓; A.2 fill ✓; A.3 per-cell ✓; A.4 Block/Boxed
✓; **A.5 Place per-cell alignment override ✓**). Pós-P232,
Categoria A não tem mais sub-passos identificados em
ADR-0079 §"Próximos passos". Próximo sub-passo Fase 5
candidata pode pivot Categoria B (algorítmicos) / C
(estruturais reabrindo) / D (runtime queries).

⁵² — Ajuste P233 (Fase 5 Layout candidata Categoria B
sub-passo 1 — DEBT-34d Auto track sizing fix; **DEBT-34d
FECHADO**; saldo DEBTs 12 → 11; quarta aplicação automática
ADR-0080 EM VIGOR; **P224.div-1 RESOLVIDA P233**):

- P233 materializa B.1 fix DEBT-34d (preservado P224.div-1
  há 18 sub-passos):
  - **Algoritmo two-pass measure→place** inaugurado P233:
    - Pass 1 (measure pre-pass): `measure_content_constrained`
      per cell em tracks Auto; max → resolved_widths.
    - Pass 2 (placement final): existing P224.C `place_cells`
      com tamanhos pre-calculados.
  - **Fix subset minimal** (atomização ADR-0036 aplicada):
    `safe = if has_fr { safe_total / (num_auto + num_fr) }
    else { safe_total }`. Auto cap-se quando há fr presente.
  - **Zero fields novos** em Content variants; **zero novas
    stdlib funcs**; refactor algorítmico puro inline em
    `layout_grid` (Decisão 3 Opção β consolidar; sem novo
    módulo per anti-inflação).
  - 5 tests E2E P233 cobrindo Auto sem fr (baseline),
    Auto+Fr mix (DEBT-34d fix), 2-Auto+1-Fr split,
    Fixed+Auto+Fr combinação, Fixed baseline (regressão).

- **8 decisões fixadas**:
  - Decisão 1 — Opção α (audit C1: DEBT-34d unitário; sem
    atomização DEBT-34d-rest necessária).
  - Decisão 2 — Opção α algoritmo two-pass measure→place
    standard vanilla.
  - Decisão 3 — Opção β consolidar em `layout_grid` sem
    novo módulo `track_sizing.rs`.
  - Decisão 4 — Opção α distribuição remaining para fr
    proporcional (split simples `safe_total / (num_auto +
    num_fr)`).
  - Decisão 5 — 5 tests E2E cobrindo cenários canónicos.
  - Decisão 6 — Fecho DEBT-34d formal + referência cruzada
    bidirecional DEBT.md ↔ P233 relatório.
  - Decisão 7 — P224.div-1 RESOLVIDA P233 (anotação
    retrospectiva).
  - Decisão 8 — Opção γ L0 NÃO tocado automaticamente
    (**quarta aplicação automática ADR-0080 EM VIGOR**).

- **L0 NÃO tocado** — quarta aplicação automática ADR-0080
  EM VIGOR pós-promoção P229. Pattern "aplicação automática
  ADR EM VIGOR sem decisão explícita por sub-passo" N=3 →
  **4 cumulativo** (P230+P231+P232+**P233**).

- **DEBT-34d FECHADO P233** — saldo DEBTs 12 → **11** (-1).
  Resolução completa do problema literal documentado
  ("Auto guloso consome todo safe_available deixando 0pt
  para fr") via subset minimal P233. Refino min-content/
  max-content per cell é refino futuro candidato
  independente (não-reservado).

- **P224.div-1 RESOLVIDA P233** — divergência factual
  material preservada conscientemente em P224 (DEBT-34d
  preservado aberto) é agora **resolved**. Pattern
  emergente "fecho retrospectivo de divergência factual
  em sub-passo posterior" N=1 inaugurado P233.

- **Patterns emergentes consolidados/inaugurados em P233**:
  - **"L0 minimal para refactors" aplicação automática
    pós-EM VIGOR** N=3 → **4 cumulativo**.
  - **"Algoritmo two-pass measure→place" N=1 inaugurado
    P233** — primeiro two-pass cristalino pós-M9c.
  - **"Fecho de DEBT preservado conscientemente em
    sub-passo posterior" N=1 inaugurado P233** —
    DEBT-34d preservado 18 sub-passos pós-P224.div-1.
  - **"Fecho retrospectivo de divergência factual em
    sub-passo posterior" N=1 inaugurado P233** —
    P224.div-1 RESOLVIDA.
  - **25 aplicações cumulativas anti-inflação** pós-P205D.

- 5 tests adicionados P233 (5 E2E layout); workspace
  2106 → **2111 verdes** (+5). 0 adaptações intencionais
  (algoritmo correcto preserva tests baseline). 0
  regressões reais.

- Sem reclassificação categórica §A.5 — `grid` já
  `implementado⁺` cumulativo. Footnote ⁵² adiciona refino
  qualitativo algoritmo Auto + fecho DEBT-34d.

- **Distribuição ADRs preservada P229**: PROPOSTO 12;
  EM VIGOR 29; IMPLEMENTADO 21; total 67.
- **Saldo DEBTs**: 12 → **11 abertos** (DEBT-34d fechou).
- **Cobertura Layout per metodologia**: **89% preservado**
  (refino qualitativo P233; algoritmo correcto pós-fecho
  DEBT-34d).

**Categoria B Fase 5 Layout: 1/3 sub-passos materializados**
(**B.1 DEBT-34d Auto track sizing ✓**; B.2 consumer
geometric integration + B.3 per-cell algorítmico pendentes).

⁵³ — Ajuste P234 (Fase 5 Layout candidata Categoria B
sub-passo 2 — Consumer geometric `place_cells` → Layouter
integration; quinta aplicação automática ADR-0080 EM VIGOR;
**colspan/rowspan funcionais em renderização pela primeira
vez pós-M9c**):

- P234 materializa B.2 integração consumer geometric:
  - **`layout_grid` passa a chamar `place_cells`** baseline
    P224.C; obtém `Vec<PlacedCell>` em vez de iterar
    `cells.chunks(num_cols)` direct.
  - **Bounds reais per cell** via helper privado
    `cell_bounds(placed, col_starts, resolved_widths,
    row_heights, current_row_start_y) -> (x0, y0, w, h)`:
    `cell_w = sum(resolved_widths[col..col+colspan])`;
    `cell_h = sum(row_heights[row..row+rowspan])`.
  - **Algoritmo three-pass measure→place→emit** inaugurado
    P234 (extensão pattern two-pass P233): Pass 1 (P233
    measure pre-pass Auto sizing); Pass 2 (P224.C
    `place_cells` placement); Pass 3 (P234 emit final com
    bounds reais).
  - **PlacedCell.body refactorado P234** — preserva outer
    cell (`Content::GridCell {...}` wrapper inteiro) em vez
    de strip inner body. Consumer extrai per-cell stroke/
    fill via match em `placed.body` preservando precedência
    P230. **5 fields baseline preservados** (body/row/col/
    colspan/rowspan); apenas semantic de `body` muda
    (outer em vez de inner — paridade Decisão 2 spec).
  - **cell_cache removido** — emissão pós-P234 itera placed
    cells (não rows_of_items chunks); cells re-medidas
    durante emissão (custo perf ~2× aceitável MVP;
    re-integração cache indexada por input_idx é refino
    futuro candidato).
  - **Row_heights padding** quando num_rows_from_placed >
    num_rows_produced (cells explicit com y maior ou
    rowspan estendido): Fixed track resolved literal;
    Auto/Fraction = 0pt (refino futuro candidato).

- **8 decisões fixadas**:
  - Decisão 1 — Opção α (integração completa; não Opção β
    parcial nem γ refactor PlacedCell rejeitado P230).
  - Decisão 2 — Opção α PlacedCell baseline 5 fields
    literal; **semantic ajustada P234**: body preserva
    outer cell wrapper.
  - Decisão 3 — Opção α bounds calculation com helper
    privado `cell_bounds` (cálculo inline reusável 3
    lugares: cell_origin set + Z1 fill + Z3 stroke).
  - Decisão 4 — Opção α match em placed.body preserva
    semantic P230 literal (post-refactor body wrapper).
  - Decisão 5 — 11 tests P234 (4 colspan/rowspan funcionais
    + 4 regressões baseline P227+P228+P230+P233 + 3 cenários
    adicionais).
  - Decisão 6 — Opção γ L0 NÃO tocado automaticamente
    (**quinta aplicação automática ADR-0080 EM VIGOR**).
  - Decisão 7 — Sem promoção formal patterns N=1
    "three-pass" e "integração consumer pós-isolamento".
  - Decisão 8 — Cache descartado MVP; re-integração
    refino futuro.

- **L0 NÃO tocado** — quinta aplicação automática ADR-0080
  EM VIGOR pós-promoção P229. Pattern "aplicação automática
  ADR EM VIGOR sem decisão explícita por sub-passo" N=4 →
  **5 cumulativo** (P230+P231+P232+P233+**P234**). Pattern
  empíricamente extremamente sólido.

- **Patterns emergentes consolidados/inaugurados em P234**:
  - **"L0 minimal para refactors" aplicação automática
    pós-EM VIGOR** N=4 → **5 cumulativo**.
  - **"Three-pass measure→place→emit" N=1 inaugurado P234**
    — extensão pattern two-pass P233; padrão mais geral
    P233 measure pre-pass + P224.C place pre-pass integrado
    + P234 emit final.
  - **"Integração consumer pós-isolamento algorítmico em
    sub-passo posterior" N=1 inaugurado P234** — P224.C
    `place_cells` criado mas não-integrado conscientemente
    (atomização ADR-0036); P234 integra. Paridade conceitual
    "fecho de DEBT preservado" P233.
  - **"PlacedCell baseline P224.C suficiente sem refactor"
    confirmado N=2** (P230 audit rejeitou refactor; P234
    integração validate empíricamente com mudança semantic
    body apenas).
  - **Reuso `place_cells` N=0 → 1 cumulativo** — primeiro
    consumer geometric real (criado P224.C; isolado até
    P234).
  - **26 aplicações cumulativas anti-inflação** pós-P205D
    (Opção α integração completa + Opção α PlacedCell
    baseline literal + Opção α helper privado + sem novo
    módulo + Opção γ L0 automático + sem refactor PlacedCell
    + sem promoções patterns emergentes + ADR-0079 sem
    promoção + cache descartado).

- 11 tests adicionados P234 (E2E layout); workspace
  2111 → **2122 verdes** (+11). 0 adaptações intencionais
  (placed cells reorder preserva render via match em
  `placed.body`). 0 regressões reais.

- Sem reclassificação categórica §A.5 — `grid` já
  `implementado⁺` cumulativo. Footnote ⁵³ adiciona refino
  qualitativo integração consumer geometric + colspan/
  rowspan funcionais em renderização pela primeira vez.

- **Distribuição ADRs preservada P229**: PROPOSTO 12;
  EM VIGOR 29; IMPLEMENTADO 21; total 67.
- **Saldo DEBTs**: 11 preservado.
- **Cobertura Layout per metodologia**: **89% preservado**
  (refino qualitativo P234 — colspan/rowspan funcionais
  algoritmo existente).

**Categoria B Fase 5 Layout: 2/3 sub-passos materializados**
(**B.1 DEBT-34d Auto track sizing ✓**; **B.2 consumer
geometric ✓**; B.3 per-cell algorítmico pendente — valida
pattern `.or()` N=2 → 3 atinge limiar formalização N=3-4).

⁵⁴ — Ajuste P235 (Fase 5 Layout candidata Categoria B
sub-passo 3 — GridCell + TableCell align/inset/breakable
per-cell; **Categoria B 3/3 ✓ FECHADA estructuralmente**;
sexta aplicação automática ADR-0080 EM VIGOR; **pattern
`.or()` N=2 → 3 atinge limiar formalização N=3-4**):

- P235 materializa B.3 algorítmicos per-cell:
  - **GridCell +3 fields** (`align: Option<Align2D>`,
    `inset: Option<Sides<Length>>`, `breakable: Option<bool>`)
    — **7 → 10 fields**.
  - **TableCell +3 fields paralelo** (pattern "refino
    aditivo paralelo entre variants irmãos" N=4 → **5
    cumulativo**) — 7 → 10 fields.
  - **`native_grid_cell` + `native_table_cell` accept 3
    named args**: helper `extract_align_value` (single
    Value) + `extract_inset_value` (Length uniforme) +
    Bool parsing direct.
  - **Renderização diferenciada por atributo**:
    - **align**: render real via Layouter `cell_align`
      P232 estendido per-cell save/restore (pattern N=1
      inaugurado P235 "Layouter cell_align save/restore
      granularidade per-cell").
    - **inset**: render real via bounds reduction
      `body_x = cell_x + inset.left; body_w = (cell_w -
      inset.left - inset.right).max(0.0)`; layout body
      em bounds reduzidos (pattern N=1 inaugurado P235
      "render real algorítmico per-cell").
    - **breakable**: armazenado semantic adiada graded
      (paridade Block.breakable P156G + repeat P224.B;
      pattern "Field armazenado semantic adiada" N=7 →
      **8 cumulativo**).
  - **Precedência `.or()` uniforme P230 + P232 + P235**:
    `effective_align = cell_align.or(self.cell_align)`;
    `effective_inset = cell_inset.cloned().unwrap_or(inset)`;
    `effective_breakable = cell_breakable` (armazenado).

- **8 decisões fixadas**:
  - Decisão 1 — Opção α escopo restrito (align + inset +
    breakable apenas).
  - Decisão 2 — Opção β tipos Option uniformes (todos
    `Option<T>` para precedência `.or()` consistente).
  - Decisão 3 — Opção α `.or()` precedência uniforme nos
    3 atributos.
  - Decisão 4 — Opção α refino paralelo TableCell.
  - Decisão 5 — Opção β reuso Layouter `cell_align`
    estendido per-cell (não Opção α directo nem γ helper
    separado).
  - Decisão 6 — Opção α render real inset (bounds
    reduction trivial pós-P234 cell_bounds).
  - Decisão 7 — Opção β breakable armazenado adiada graded.
  - Decisão 8 — Opção γ L0 NÃO tocado (**sexta aplicação
    automática ADR-0080 EM VIGOR**).

- **Adaptações intencionais P235**:
  - `native_table_cell_named_arg_desconhecido_rejeitado`
    (stdlib/mod.rs:4150): test usava `inset` como exemplo
    "unknown"; agora `inset` é conhecido P235. Adaptado
    para `outset` que continua scope-out.

- **L0 NÃO tocado** — sexta aplicação automática ADR-0080
  EM VIGOR pós-promoção P229. Pattern "aplicação automática
  ADR EM VIGOR sem decisão explícita por sub-passo" N=5
  → **6 cumulativo** (P230+P231+P232+P233+P234+**P235**).
  Pattern **extremamente sólido empíricamente** — seis
  aplicações automáticas consecutivas sem excepção.

- **Patterns emergentes consolidados/inaugurados em P235**:
  - **"Precedência per-X via `.or()` resolution"** N=2 →
    **3 cumulativo atingindo limiar formalização N=3-4**
    (P230 GridCell stroke/fill; P232 Place per-axis; P235
    GridCell 3 algorítmicos) — **promoção formal ADR meta
    candidato XS futuro paridade P229**.
  - **"Refino aditivo paralelo entre variants irmãos"**
    N=4 → **5 cumulativo** (Grid+Table P227/P228;
    GridCell+TableCell P230; Block+Boxed P231; **GridCell+
    TableCell algorítmico P235**).
  - **"Smart→Option"** N=10 → **12 cumulativo** (+inset
    Option +breakable Option).
  - **"Field armazenado semantic adiada"** N=7 → **8
    cumulativo** (+breakable per-cell).
  - **"L0 minimal para refactors" aplicação automática
    pós-EM VIGOR**: N=5 → **6 cumulativo**.
  - **"Fecho categoria completa dentro de ADR PROPOSTO
    sem transição"** N=1 → **2 cumulativo** (P232 Categoria
    A; **P235 Categoria B**).
  - **"Layouter cell_align save/restore granularidade
    per-cell" N=1 inaugurado P235** — extensão P232 que
    só fez per-Grid save/restore.
  - **"Render real algorítmico per-cell" N=1 inaugurado
    P235** — distinto cumulativo de cosméticos P230.
  - **"Renderização diferenciada por atributo dentro do
    mesmo sub-passo" N=1 inaugurado P235** — align real
    + inset real + breakable graded.

- **15 tests adicionados P235** (4 unit content + 6 unit
  stdlib + 5 layout E2E); workspace 2122 → **2137 verdes**
  (+15). 1 adaptação intencional (`native_table_cell_named_arg_desconhecido_rejeitado`).
  0 regressões reais.

- Sem reclassificação categórica §A.5 — `grid` já
  `implementado⁺` cumulativo. Footnote ⁵⁴ adiciona refino
  qualitativo algorítmicos per-cell + fecho Categoria B
  3/3 estructuralmente.

- **Distribuição ADRs preservada P229**: PROPOSTO 12;
  EM VIGOR 29; IMPLEMENTADO 21; total 67.
- **Saldo DEBTs**: 11 preservado.
- **Cobertura Layout per metodologia**: **89% preservado**
  (refino qualitativo P235 algorítmicos per-cell).

**Categoria B Fase 5 Layout: 3/3 ✓ FECHADA estructuralmente**
(**B.1 DEBT-34d Auto track sizing ✓**; **B.2 consumer
geometric ✓**; **B.3 GridCell + TableCell algorítmicos ✓**).

⁵⁵ — Ajuste P236 (Fase 5 Layout candidata Categoria D 1/?
— `state_final(key)` stdlib func **refino aditivo
verdadeiro** pós-`P236.div-1` divergência factual material;
sétima aplicação automática ADR-0080 EM VIGOR; **state
runtime já materializado pre-P236** P171+M9+M9c):

**`P236.div-1` — divergência factual material registada**:
Spec P236 assumia (a) ADR-0066 status PROPOSTO, (b) state
runtime ausente requerendo materialização nova, (c)
infraestrutura (`Value::State`, `entities/state.rs`,
`rules/stdlib/state.rs`, Layouter HashMap) requerida.
Audit C1 confirmou estado real:

- **ADR-0066 status real: SUPERSEDED-BY 0073** (P204H
  2026-05-07). Cadeia chronológica: ADR-0066 (PROPOSTO
  2026-04-27) → ACEITE (P192B 2026-05-05) → SUPERSEDED-BY
  0073 (P204H 2026-05-07; M8 adoptou comemo) → F3 fechou
  §C6a (ADR-0074 ACEITE P205B+C+E). Promoção PROPOSTO →
  IMPLEMENTADO impossível (status SUPERSEDED).
- **State runtime já materializado P171+M9+M9c**:
  - `Content::State { key, init }` (P171, M9 sub-passo 3).
  - `Content::StateUpdate { key, update: StateUpdate }`
    (P171/P172).
  - `entities/state_registry.rs` (P171) — full
    `HashMap<String, Vec<(Location, Value)>>`.
  - `entities/state_update.rs` (P171/P172) — `enum
    StateUpdate { Set(Value), Func(Func) }`.
  - `entities/layouter_runtime_state.rs` (P190C/D).
  - 3 stdlib funcs em `foundations.rs`: `native_state`,
    `native_state_update`, `native_state_update_with`.
  - Pipeline activo: `Introspector::state_final_value` +
    `state_value(key, location)` lookup; from_tags walk
    aplica updates.

**Decisão humana pós-divergência (Opção 2 do questionário)**:
Refino aditivo subset — adicionar UMA stdlib func que
materializa parte específica de D.1 não coberta pelo M9
baseline. Escolhido: `state_final(key)` paralelo a
`counter_final(key)` P176.

- P236 materializa refino aditivo `state_final`:
  - **`native_state_final(key)` em `foundations.rs`** —
    1 stdlib func nova; argumento posicional Str `key`;
    retorna `Value` (init se state nunca actualizado;
    último valor caso contrário; `Value::None` se key
    inexistente).
  - **Reuso `Introspector::state_final_value`** (P171
    baseline) — wrapper trivial.
  - **Registo scope** em `eval/mod.rs`:
    `scope.define("state_final", ...)`.
  - **Paralelo absoluto a `counter_final` (P176)** —
    pattern "stdlib func runtime para final value lookup"
    N=1 → 2 cumulativo (counter_final P176; **state_final
    P236**).

**Não-objectivos preservados P236**:
- **ADR-0066 NÃO promovido** (permanece SUPERSEDED).
- **`Value::State` NÃO criado** (Content::State é
  suficiente; Value variant adicional inflacionário).
- **`state_at(key, location)` NÃO criado** — refino
  futuro candidato (paralelo a `counter_at` P177; valor
  para uso real esperando promoção formal).
- **L0 NÃO tocado** — refino aditivo verdadeiro qualifica
  Opção γ ADR-0080 §"Escopo" literal ("stdlib func nova
  aditiva" categoria explícita line 66).

- **8 decisões fixadas P236 (revisitadas pós-divergência)**:
  - Decisão 1 — Opção 2 (refino aditivo subset; rejeitar
    Opções 1/3/4 do questionário humano).
  - Decisão 2 — `state_final` escolhido sobre `state_at`
    (mais imediato; paralelo `counter_final` P176 directo).
  - Decisão 3 — Paralelo absoluto pattern `counter_final`
    (Introspector wrapper trivial).
  - Decisão 4 — `Value::None` retornado se key inexistente
    (semantic distinto Value::Str("") `counter_final` —
    state pode ter qualquer Value type).
  - Decisão 5 — Iter 0 fixpoint retorna None (introspector
    vazio).
  - Decisão 6 — 6 unit tests P236 (subset minimal cenários
    canónicos; sem layout E2E pois state não-renderiza
    output).
  - Decisão 7 — ADR-0066 NÃO tocado (status SUPERSEDED
    preservado; promoção impossível).
  - Decisão 8 — **Opção γ L0 NÃO tocado** (refino aditivo
    verdadeiro qualifica per ADR-0080 §"Escopo" line 66
    "stdlib func nova aditiva"; **N=6 → 7 cumulativo
    aplicação automática ADR-0080 EM VIGOR**, NÃO excepção
    como spec original sugeria).

- **L0 NÃO tocado** — sétima aplicação automática ADR-0080
  EM VIGOR pós-promoção P229. Pattern "aplicação automática
  ADR EM VIGOR sem decisão explícita por sub-passo" N=6
  → **7 cumulativo** (P230+P231+P232+P233+P234+P235+
  **P236**). Pattern **extremamente sólido empíricamente**
  — sete aplicações automáticas consecutivas sem excepção.

- **Patterns emergentes consolidados/inaugurados em P236**:
  - **"L0 minimal para refactors" aplicação automática
    pós-EM VIGOR**: N=6 → **7 cumulativo**.
  - **"stdlib func runtime para final value lookup"** N=1
    → **2 cumulativo** (counter_final P176; **state_final
    P236**) — pattern emergente paralelo.
  - **"Divergência factual material registada via P236.div-1
    + decisão humana pós-divergência"** N=1 inaugurado P236
    — primeira divergência factual MATERIAL pós-M9c
    requerendo decisão humana imediata via questionário.
  - **"Spec materializada como refino aditivo subset
    pós-divergência factual"** N=1 inaugurado P236 —
    pattern complementar a "fecho retrospectivo de
    divergência" P233.
  - **"State runtime materializado pre-P236 reconhecido
    retrospectivamente como cumprimento ADR-0066"** —
    documentação corretiva (ADR-0066 SUPERSEDED-BY 0073
    chain materializou state via M9 P171+P172 +
    M8 ADR-0073 + M9c ADR-0074).

- 6 unit tests adicionados P236 (1 stdlib func × 6 cenários
  canónicos: vazio retorna None; init retorna init; updates
  retorna último; key inexistente retorna None; arg
  não-string retorna Err; zero args retorna Err); workspace
  2137 → **2143 verdes** (+6). 0 adaptações intencionais.
  0 regressões reais.

- Sem reclassificação categórica §A.5 — stdlib `state_final`
  é refino aditivo. Footnote ⁵⁵ adiciona refino qualitativo
  + documenta P236.div-1 + state runtime já-materializado
  pre-P236.

- **Distribuição ADRs preservada P229**: PROPOSTO 12;
  EM VIGOR 29; IMPLEMENTADO 21; total 67. **ADR-0066
  permanece SUPERSEDED-BY 0073** — não conta nos 67
  porque SUPERSEDED é status terminal (cadeia fechada).
- **Saldo DEBTs**: 11 preservado.
- **Cobertura Layout per metodologia**: **89% preservado**
  (D.1 é Introspection refino, não Layout).
- **Cobertura Introspection**: state_final user-facing
  exposto pela primeira vez pós-M9c — refino qualitativo
  marginal (+1 stdlib func × paridade vanilla
  `state.final()`).

**Categoria D Fase 5 Layout: 1/? sub-passos materializados**
(**D.1 state runtime ✓ refino aditivo P236 pós state já
materializado pre-P236**; D.2 state.at + state.display
candidatos refino futuro; D.3 query/D.4 counter candidatos
sub-passos separados).

**Stdlib funcs**: 60 → **61** (+`state_final`).

⁵⁶ — Ajuste P237 (Fase 5 Layout candidata Categoria D 1/?
refino estendido — `state_at(key, label)` paralelo absoluto
`counter_at` P177; oitava aplicação automática ADR-0080
EM VIGOR; **primeira aplicação da lição metodológica
P236.div-1 via spec C1 audit obrigatório bloqueante**):

- P237 materializa refino aditivo `state_at`:
  - **`native_state_at(key, label)` em `foundations.rs`** —
    stdlib func nova; reuso `Introspector::query_by_label`
    P139+P140 + `Introspector::state_value` P171; chain
    via `.and_then().unwrap_or(Value::None)` paralelo
    pattern `counter_at` literal.
  - **Registo scope** `state_at` em `eval/mod.rs:606`
    paralelo `counter_at` P177.
  - **6-7 unit tests subset minimal** cenários canónicos
    (paridade P236 — sem layout E2E pois state não-renderiza).

- **Audit C1 obrigatório bloqueante (lição P236.div-1)**:
  - `Introspector::query_by_label(label: &Label) -> Option<Location>`
    confirmado P139+P140 (não `lookup_label(&str) -> SourceResult`
    como spec hipotetizou — ajuste signature trivial sem
    `P237.div-N` formal).
  - `Introspector::state_value(key, location)` confirmado
    P171 (paridade P236 audit §2).
  - `native_counter_at` P177 pattern: `query_by_label`
    chain `state_value` `.and_then().unwrap_or_default()`
    → state_at retorna `Value::None` (não erro hard como
    spec Decisão 4 cenário 2 sugeriu — paridade literal
    counter_at que retorna empty).

- **8 decisões fixadas P237** (spec Decisão 0 = lição
  P236.div-1 aplicada):
  - Decisão 0 — C1 audit obrigatório bloqueante; sem
    `P237.div-N` (audit converge com hipóteses revistas).
  - Decisão 1 — Opção α escopo minimal aditivo (apenas
    `state_at`).
  - Decisão 2 — Signature `(key: Str, label: Str) → Value`
    paridade `counter_at` P177 literal (ajuste trivial
    pós-audit: `query_by_label` em vez de `lookup_label`).
  - Decisão 3 — Reuso wrapper trivial `state_value`
    paralelo P236 `state_final`.
  - Decisão 4 — Semantic edge case: label inexistente
    retorna `Value::None` (paridade counter_at empty
    default; **revisão de spec** que sugeria erro hard).
  - Decisão 5 — 7 unit tests subset minimal cenários
    canónicos (paridade P236; sem layout E2E).
  - Decisão 6 — Opção γ L0 NÃO tocado (**oitava aplicação
    automática ADR-0080 EM VIGOR**).
  - Decisão 7 — ADR-0066 NÃO tocado (SUPERSEDED-BY 0073
    terminal preservado).
  - Decisão 8 — Sem promoção ADR-0079; sem marco
    cirúrgico blueprint (refino estendido não-fecha
    Categoria nem sub-categoria).

- **L0 NÃO tocado** — oitava aplicação automática ADR-0080
  EM VIGOR pós-promoção P229. Pattern "aplicação automática
  ADR EM VIGOR sem decisão explícita por sub-passo" N=7
  → **8 cumulativo** (P230+P231+P232+P233+P234+P235+P236+
  **P237**). Pattern **extremamente sólido empíricamente**
  — oito aplicações automáticas consecutivas sem excepção.

- **Patterns emergentes consolidados/inaugurados em P237**:
  - **"L0 minimal para refactors" aplicação automática
    pós-EM VIGOR**: N=7 → **8 cumulativo**.
  - **"stdlib func runtime para label-based lookup"** N=1
    inaugurado P237 — distinto do "final value lookup"
    (state_at requer Location resolução via label;
    state_final/counter_final não requerem). counter_at
    P177 baseline anterior à série Categoria D refino,
    portanto não conta no N novo.
  - **"spec C1 audit obrigatório bloqueante pós-P236.div-1"
    N=1 inaugurado P237** — metodológico crítico
    aplicável a sub-passos futuros D.2+/C.1+/runtime.
  - **"paralelismo state↔counter completo"** N=1 inaugurado
    P237 — state agora 5 ops (state/state_update/
    state_update_with/state_final/state_at) paridade
    counter 4 ops (counter/counter_update/counter_final/
    counter_at; counter sem paralelo state_update_with
    porque counter mutation é apenas Set, não Func).

- **Adaptações intencionais P237** (N=0 hipotetizadas; N=1
  factual): test pre-existente passou (sed `LabelRegistry::
  insert` → `add` ajustado durante implementação — método
  pre-existente é `add` não `insert`; ajuste trivial sem
  `P237.div-N`).

- 7 unit tests adicionados P237 (label inexistente retorna
  None; key inexistente retorna None; resolve label retorna
  init; updates antes location visível último; updates
  depois location não visíveis; arg não-string rejeita;
  arity errada rejeita); workspace 2143 → **2150 verdes**
  (+7). 0 regressões reais.

- Sem reclassificação categórica §A.5. Footnote ⁵⁶ adiciona
  refino aditivo qualitativo + lição metodológica P236.div-1
  aplicada.

- **Distribuição ADRs preservada P229**: PROPOSTO 12;
  EM VIGOR 29; IMPLEMENTADO 21; total 67. ADR-0066
  SUPERSEDED-BY 0073 preservado.
- **Saldo DEBTs**: 11 preservado.
- **Cobertura Layout per metodologia**: **89% preservado**
  (D.1 é Introspection refino, não Layout).
- **Cobertura Introspection**: state_at user-facing exposto
  pela primeira vez pós-M9c — refino qualitativo marginal
  (+1 stdlib func × paridade vanilla `state.at(location)`).

**Categoria D Fase 5 Layout: 1/? refino estendido completo**
(state_final P236 + state_at P237; **paralelismo state↔counter
completo**).

**Stdlib funcs**: 61 → **62** (+`state_at`).

⁵⁷ — Ajuste P238 reescrito (passo administrativo documental
— auditoria metodológica falhanços `P236.div-1` + `P238.div-1`
+ plano realista cobertura Layout pós-P237; **zero código
tocado**; paridade pattern P225/P229 administrativo; refino
lição `P236.div-1` N=1 → **2 cumulativo**; **`P238.div-1`
registado como segundo falhanço spec arquitectural maior
consecutivo pós-M9c**):

**`P238.div-1` — divergência factual material registada**:
Spec P238 original hipotetizou `state.display` walk-time
render-mediated callback materializável via refino aditivo
(Categoria D 2/?). Audit C1 obrigatório bloqueante (lição
`P236.div-1`) revelou contradição factual material em três
pontos:

- **`Content::State` é zero-size em layout** — arm
  `Content::State { .. } => {}` em `layout/mod.rs:352`
  literalmente vazio. P171 baseline é init marker; valor
  obtido **fora do walk** via queries (`state_value` /
  `state_final_value`).
- **`Func::call` não existe** — só `Func::native` constructor.
  Eval real requer `EvalContext + Engine + World + FileId +
  figure_numbering` indisponíveis durante walk.
- **`StateUpdate::Func` (P172) é stub documentado** por
  blocker arquitectural idêntico: "from_tags reconhece a
  variant mas não avalia a closure — `Func::call` requer
  `EvalContext + Engine` que não estão disponíveis em walk
  nem em from_tags".

Spec P238 original previa walk-time render-mediated callback
**arquiteturalmente impossível sem pipeline restructuring M7+**.

**Decisão humana pós-`P238.div-1`**: P238 reescrito como
auditoria metodológica formal + plano realista cobertura
Layout (decisão literal pós-divergência; sem materializar
código quando bloqueio arquitectural identificado). Paridade
pattern P225 (encerramento Fase 4 documental) + P229 (promoção
ADR-0080 administrativa). **Distinto** dos sub-passos
materialização P227-P237.

- P238 reescrito materializa
  (`typst-passo-238-auditoria.md` 7 §s):
  - **Auditoria metodológica formal** dos dois falhanços
    consecutivos (`P236.div-1` + `P238.div-1`) — causas raiz;
    padrões emergentes; lição refinada (§2).
  - **Estado factual cobertura Layout pós-P237** — sub-passos
    materializados vs pendentes; bloqueadores arquiteturais
    (§3).
  - **Plano realista cobertura Layout** identificando viável
    pós-P237 sem refactor M7+ vs requer pipeline restructuring
    (§4).
  - **Recomendações metodológicas futuras** — aplicação da
    lição refinada; sinais de risco alto/crítico; pattern
    emergente "spec audit prévio para sub-passos
    walk-time/runtime" (§5).
  - **Saída cumulativa** preservando 2150 verdes + 0 violations
    + 11 DEBTs + ADRs distribuição (§6).
  - **Critério aceitação** (§7).

- **Causa raiz comum aos dois falhanços** (§2.3 auditoria):
  - `P236.div-1`: sumário contexto incompleto; spec assumiu
    baseline pré-M9c sem audit prévio.
  - `P238.div-1`: spec incluiu C1 audit obrigatório bloqueante
    (lição `P236.div-1`) mas fixou decisões C2-C8 prováveis
    baseadas em hipóteses análogas eval-time aplicadas
    incorretamente a walk-time/runtime integration. Pattern
    "decisões sujeitas a C1" criam viés cognitivo que resiste
    revisão pós-audit.

- **Refino lição metodológica `P236.div-1`** (§2.5 auditoria):

  > Para sub-passos com risco alto/crítico (walk-time;
  > runtime callback dispatch; pipeline integration), spec
  > deve fazer audit prévio **ANTES** de redigir decisões
  > C2-C8. Para refinos de risco baixo/médio (eval-time
  > wrappers; cosméticos; algorítmicos isolados), C1 audit
  > bloqueante como primeira cláusula é suficiente.

- **Atomização preventiva** (§2.6 auditoria): para sub-passos
  risco alto/crítico, atomizar em (1) prep-passo audit-only
  XS-S sem decisões fixadas + (2) materialização-passo
  conforme audit. Paridade pattern P226 (diagnóstico amplo
  + ADR PROPOSTO + roadmap) que precedeu materialização
  Fase 5 P227+.

- **Bloqueadores arquiteturais identificados pós-P237**
  (§4.1 auditoria):
  - **Bloqueador A** — Walk-time eval Func dispatch
    (`Func::call` inexistente); afecta D.2 `state.display`,
    `counter.display`, possíveis D.3+. Resolução M7+ pipeline
    restructuring.
  - **Bloqueador B** — Multi-region completion (DEBT-56b
    candidato); afecta C.2 + breakable per-cell render real
    (A.4 graded P235). Resolução refactor multi-region
    cell-level.
  - **Bloqueador C** — Place float real (reabertura Opção B
    P219); afecta C.1. Resolução refactor magnitude L+.
  - **Bloqueador D** — Pipeline runtime two-pass walk
    (`state.final()` semantic vanilla); afecta refino
    `state.final()` real two-pass. Resolução M7+
    infrastructure.

- **Sub-passos viáveis sem refactor M7+** identificados
  (§4.2 auditoria):
  - **D.X1 counter.display stub** (paridade P172
    `StateUpdate::Func` stub) — VIÁVEL via stub paralelo;
    não-recomendado se D.2 também stub.
  - **D.X2 query refinos** — eval-time wrappers paridade
    `state_at` / `state_final`. **Audit prévio obrigatório**.
  - **D.X3 numbering refinos** — audit prévio obrigatório.
  - **A.4 refino outset render real** — Block/Boxed; audit
    prévio + materialização conforme.
  - **A.X fill/stroke Block/Boxed** — paridade P227+P228
    estructural; render real viável.

- **Estimativa fecho realista Fase 5 Layout** (§4.5 auditoria):
  - **Sem refactor M7+**: Fase 5 candidata fecha em
    **10-12/13-15 sub-passos materializados** (~67-85%);
    sub-passos bloqueados arquiteturalmente preservados como
    graded/scope-out documentados.
  - **Com refactor M7+**: Fase 5 candidata materializa
    13-15/13-15 (100% interno) mas magnitude cumulativa
    L+ a XL+.
  - **Decisão arquitectural pendente**: humano decide se
    Fase 5 fecha graded a ~80% OU reabre M-fase para refactor
    pipeline.

- **L0 NÃO tocado** — passo administrativo documental
  não-toca código nem prompts; **não conta na contagem
  "aplicação automática ADR-0080 EM VIGOR"** porque não
  envolve materialização sub-passo Fase 5 (paralelo P225/P229
  administrativos). Pattern "aplicação automática ADR EM VIGOR
  sem decisão explícita por sub-passo" preserva **N=8
  cumulativo** P230-P237.

- **Patterns emergentes inaugurados/consolidados em P238
  reescrito** (6):
  - **"spec audit prévio obrigatório para sub-passos
    walk-time/runtime" N=1 inaugurado P238 reescrito** —
    refino lição `P236.div-1`.
  - **"atomização prep-passo audit-only + materialização-passo
    para sub-passos risco alto/crítico" N=1 inaugurado P238
    reescrito** — paridade P226 diagnóstico amplo.
  - **"`Pxxx.div-1` cumulativo para falhanços spec
    arquitectural maior"** N=1 → **2 cumulativo**
    (`P236.div-1` + **`P238.div-1`**).
  - **"passo administrativo documental para auditoria
    metodológica pós-divergência" N=1 inaugurado P238
    reescrito** — distinto de P225 (encerramento Fase) +
    P229 (promoção ADR-0080).
  - **"L0 minimal para refactors" aplicação automática N=8
    preservado** (P230-P237; P238 reescrito documental
    não-incrementa porque não toca código).
  - **"Fase candidata fecha graded a bloqueadores
    arquiteturais identificados" N=1 inaugurado P238
    reescrito** — Fase 5 Layout candidata pode fechar ~80%
    preservando bloqueadores como scope-out documentado.

- **Zero código tocado P238 reescrito**: workspace 2150
  verdes preservado; 0 violations preservadas; 0 adaptações;
  0 regressões; 0 novos tests; 0 fields adicionados; 0
  variants adicionados; 0 stdlib funcs novas; 0 módulos
  novos.

- Sem reclassificação categórica §A.5. Footnote ⁵⁷ adiciona
  documentação metodológica pós-`P238.div-1` + plano realista
  cobertura Layout pós-P237 + refino lição `P236.div-1`.

- **Distribuição ADRs preservada P229**: PROPOSTO 12;
  EM VIGOR 29; IMPLEMENTADO 21; total 67. ADR-0066
  SUPERSEDED-BY 0073 preservado.
- **Saldo DEBTs**: 11 preservado.
- **Cobertura Layout per metodologia**: **89% preservado**.
- **Cobertura user-facing total**: 67% preservada.
- **Anti-inflação 30ª aplicação cumulativa pós-P205D** —
  Opção "auditoria documental" (não materializar código
  quando bloqueio arquitectural identificado) + Opção γ L0
  NÃO tocado + Opção α sem promoção ADR + Opção α sem marco
  cirúrgico blueprint + paridade pattern P225/P229
  administrativo + Decisão refino lição `P236.div-1`.

**Estado pós-P238 reescrito** — Categoria A 5/5 ✓ FECHADA;
Categoria B 3/3 ✓ FECHADA; Categoria D 1/? refino estendido
completo (state_final P236 + state_at P237); Categoria C 0/?;
**D.2 `state.display` walk-time BLOQUEADO arquiteturalmente
identificado formalmente pós-`P238.div-1`**. Fase 5 Layout
candidata 10/13-15 sub-passos materializados preservado
(P238 reescrito administrativo não-incrementa).

**Stdlib funcs**: 62 preservado.

⁵⁸ — Ajuste P239 (prep-passo audit-only reabertura M-fase
pós-M9c para M7+ refactor; **zero código tocado**; **primeira
aplicação real pattern "atomização prep-passo audit-only +
materialização-passo" inaugurado P238 reescrito**; ADR meta
novo ADR-0081 PROPOSTO criado; paridade pattern P225/P229/P238
reescrito administrativo cumulativo N=3 → 4):

**P239 prep-passo audit-only materializa**
(`typst-passo-239-audit-m7-reabertura.md` 9 §s ~24 KB):
- **Audit M-fase histórico** (M5/M6/M7/M8/M9/M9c cumulativos;
  M7 estruturalmente fechado P192B ADR-0072; reabertura M-fase
  para walk-time eval é **nova M-fase**, não reabertura M7).
- **Audit blocker arquitectural walk-time eval Func dispatch**
  — achado material refina hipótese P238 reescrito:
  - ✗ Hipótese P238 reescrito: "`Func::call` não existe".
  - ✓ Realidade: `Func::call` método não existe **como
    método em Func**, mas mecanismo de chamada é
    `closures::apply_func(func, args, ctx, engine)` que
    existe (`01_core/src/rules/eval/closures.rs:59`) e
    funciona.
  - ✓ Realidade: `apply_state_funcs` JÁ EXISTE em
    `01_core/src/rules/introspect/from_tags.rs:48` e avalia
    `StateUpdate::Func` via fixpoint loop pós-walk com
    Engine+ctx disponíveis (caller único `run_fixpoint` em
    `fixpoint.rs:101`).
  - **Blocker real**: layout-time Engine+ctx indisponíveis
    (Layouter puro sem acesso eval) — não walk-time Func
    dispatch.

- **4 opções resolução estructural identificadas P239 §3.2**:
  - Opção α — Pass `Engine + ctx` para Layouter signature
    massivo. Magnitude L+; risco quebrar comemo invariants
    ADR-0073/0074.
  - Opção β — Two-pass walk completo. Magnitude XL+; signature
    refactor cumulativo.
  - **Opção γ recomendada** — `apply_state_displays` pré-eval
    em fixpoint paralelo `apply_state_funcs`; layout consome
    Content pré-renderizado. Magnitude **L (~5-8h)**; paridade
    pattern existente; baixo risco.
  - Opção δ — Show rule synthetic mecanismo recursivo. Magnitude
    L+; incoerência arquitectural Cristalino pós-M9c.

- **Audit blockers relacionados P239 §3.3**:
  - Multi-region completion cell-level (DEBT-56b candidato; L+
    ~8-12h; independente parcial).
  - Place float real (reabertura Opção B P219; L ~5-8h;
    independente).
  - state.final two-pass walk (**sobreposição grande com
    walk-time via Opção γ**; mesmo refactor desbloqueia ambos).
  - **Bloqueador adicional E identificado P239** — A.4
    radius/clip infrastructure: `ShapeKind::RoundedRect`
    AUSENTE (`01_core/src/entities/geometry.rs:32` tem
    `Rect|Ellipse|Line|Path`); `Group::clip_mask: Option<ShapeKind>`
    JÁ EXISTE baseline (`layout_types.rs:235`); refactor M-L
    ~3-5h.

- **Sobreposições P239 §3.4**: bloqueadores A + D partilham
  refactor Opção γ; resto independente.

- **Roadmap atomização 5 sub-passos materialização M7+ P239 §4**:
  - **M7+1** — Pipeline walk-time eval (Opção γ
    `apply_state_displays` + `Content::StateDisplay` variant
    + walk arm pre-render); L ~5-8h; nenhuma dependência;
    desbloqueia D.2 state.display real + state.final two-pass
    real.
  - **M7+2** — counter.display paralelo
    (`Content::CounterDisplay` + `apply_counter_displays`);
    M ~2-4h; depende M7+1 (reuso pattern).
  - **M7+3** — Multi-region completion cell-level (`Regions
    { current, backlog, last }`); L+ ~8-12h; independente.
  - **M7+4** — Place float real (reabertura Opção B P219); L
    ~5-8h; independente.
  - **M7+5** — A.4 radius/clip infrastructure
    (`ShapeKind::RoundedRect` + `Corners<T>` type paridade
    `Sides<T>`); M-L ~3-5h; independente.
  - **Total cumulativo ~23-37h materialização** — refinado
    pós-audit empírico face P238 reescrito XL+ ~20-40h
    hipotetizado.

- **3 pré-condições obrigatórias formalizadas P239 §5**:
  - Testes baseline preservados (2150 verdes pré-M7+;
    adaptações N>0 documentadas).
  - Comemo memoization invariants ADR-0073/0074 preservados.
  - Backward compat eval-time (P236 state_final + P237
    state_at wrappers continuam funcionar).

- **ADR meta novo PROPOSTO criado P239 §6** —
  `00_nucleo/adr/typst-adr-0081-m7-plus-pipeline-restructuring-scope.md`:
  - Status PROPOSTO.
  - Escopo: 5 bloqueadores (A walk-time Func; B multi-region;
    C Place float; D state.final two-pass; **E radius/clip
    novo P239**).
  - Atomização: 5 sub-passos M7+1 a M7+5.
  - Pré-condições obrigatórias formalizadas.
  - Dependencies/ordem propostos.
  - Magnitude cumulativa estimada L+ a XL (~23-37h).
  - 6 alternativas consideradas (A monolítico + B chain
    extensão + C estender 0079 + E Layouter signature + F
    two-pass + G Show rules recursivo preteridas; D pivot
    válida).
  - 3 sub-decisões pendentes (D1 nomenclatura M-fase
    preliminar **M9d**; D2 ordem primeira materialização;
    D3 promoção pós-M7+).

- **8 decisões fixadas P239** (Decisão 0 = lição `P238.div-1`):
  - Decisão 0 — Prep-passo audit-only obrigatório.
  - Decisão 1 — Opção α escopo audit cumulativo.
  - Decisão 2 — Opção α ADR meta novo PROPOSTO criado.
  - Decisão 3 — Opção α atomização ADR-0036 aplicável.
  - Decisão 4 — 3 pré-condições obrigatórias formais.
  - Decisão 5 — Magnitude L+ a XL refinada empíricamente.
  - Decisão 6 — Opção γ L0 NÃO tocado.
  - Decisão 7 — Opção β saldo DEBTs preservado/decresce.
  - Decisão 8 — Sem promoção ADR-0079; Fase 5 candidata
    mantém 10/13-15 sub-passos.

- **L0 NÃO tocado** — passo administrativo audit-only não-toca
  código nem prompts; **não conta na contagem "aplicação
  automática ADR-0080 EM VIGOR"** (paralelo P225/P229/P238
  reescrito). Pattern "L0 minimal para refactors aplicação
  automática" preserva **N=8 cumulativo** P230-P237.

- **Patterns emergentes inaugurados/consolidados em P239** (7):
  - **"spec audit prévio obrigatório para sub-passos
    walk-time/runtime"** N=1 → **2 cumulativo** (P238 reescrito
    + P239).
  - **"prep-passo audit-only preventivo para reabertura M-fase"
    N=1 inaugurado P239** — extensão pattern P238 reescrito.
  - **"passo administrativo documental"** N=3 → **4 cumulativo**
    (P225; P229; P238 reescrito; **P239**) — pattern
    empíricamente sólido.
  - **"ADR meta novo PROPOSTO para reabertura M-fase" N=1
    inaugurado P239** — primeiro ADR meta novo pós-P229.
  - **"atomização prep-passo audit-only + materialização-passo"**
    N=1 → **2 cumulativo** (P238 reescrito + P239).
  - **"L0 minimal para refactors" aplicação automática N=8
    preservado** (P239 administrativo não-incrementa).
  - **"audit empírico refina hipótese spec"** N=2 → **3
    cumulativo** (`P236.div-1`; P237 audit C1; **P239 audit
    C2.1+C2.2 `apply_state_funcs` já existe**).

- **Zero código tocado P239**: workspace 2150 verdes preservado;
  0 violations preservadas; 0 adaptações; 0 regressões; 0
  novos tests; 0 fields adicionados; 0 variants adicionados;
  0 stdlib funcs novas; 0 módulos novos.

- Sem reclassificação categórica §A.5. Footnote ⁵⁸ adiciona
  documentação metodológica audit empírico M-fase + 4+1
  bloqueadores arquiteturais + roadmap atomização M7+ + ADR
  meta novo ADR-0081 PROPOSTO.

- **Distribuição ADRs P239**: PROPOSTO **12 → 13** (+ADR-0081
  M7+ scope PROPOSTO); EM VIGOR 29; IMPLEMENTADO 21; total
  **67 → 68**. ADR-0066 SUPERSEDED-BY 0073 preservado.
- **Saldo DEBTs**: 11 preservado.
- **Cobertura Layout per metodologia**: **89% preservado**.
- **Cobertura user-facing total**: 67% preservada.
- **Anti-inflação 31ª aplicação cumulativa pós-P205D** —
  Opção α audit-only + Opção α ADR meta novo + Opção α
  atomização ADR-0036 + Opção β saldo DEBTs preservado/decresce
  + Opção α pré-condições formalizadas + Opção γ L0 NÃO
  tocado + Opção α sem promoção ADR-0079 + Opção α sem
  materialização imediata.

**Estado pós-P239** — Categoria A 5/5 ✓ FECHADA; Categoria
B 3/3 ✓ FECHADA; Categoria D 1/? refino estendido completo;
Categoria C 0/?; **D.2 + counter.display + state.final two-pass
+ C.1 + C.2 + A.4 radius/clip identificados formalmente como
bloqueadores arquiteturais M7+ refactor desbloqueia via
ADR-0081 PROPOSTO**. Fase 5 Layout candidata 10/13-15
sub-passos materializados preservado (P239 administrativo
não-incrementa); **M7+ refactor posterior desbloqueia 3-5
sub-passos adicionais** (D.2 + counter.display + state.final
real two-pass + C.1 + C.2 + A.4 graded refinos).

**Decisão humana pendente pós-P239** (per audit §7): primeira
materialização sub-passo M7+ (recomendação subjectiva M7+1
pipeline walk-time eval; alternativa válida M7+5 radius/clip
ou pivot outro módulo).

**Stdlib funcs**: 62 preservado.

⁵⁹ — Ajuste P240 (M9d / M7+1 primeira sub-passo materialização
pós-P239 audit-only — **Pipeline walk-time eval via Opção γ
`apply_state_displays`**; **primeira aplicação real do pattern
"atomização prep-passo audit-only + materialização-passo"
inaugurado P238 reescrito N=1 → 2 cumulativo**; **terceira
aplicação cumulativa pattern "spec C1 audit obrigatório
bloqueante pós-P236.div-1" N=2 → 3 cumulativo**; **primeira
excepção justificada à aplicação automática ADR-0080 EM VIGOR
pós-P229**; ADR-0081 PROPOSTO → IMPLEMENTADO parcial;
desbloqueia D.2 state.display walk-time real + state.final
two-pass real via sobreposição bloqueador A+D):

**P240 materializa M7+1 Opção γ** (per ADR-0081 PROPOSTO P239
audit §3.2; refactor pipeline walk-time real cristalino sem
quebrar Layouter pureza arquitectural):

- **`Content::StateDisplay { key: String, callback: Option<Func> }`**
  variant novo em `entities/content.rs`. **Content variants:
  60 → 61** (+StateDisplay).
- **`ElementPayload::StateDisplay { key, callback }`** variant
  novo em `entities/element_payload.rs` (audit C1 P240 refinou
  hipótese spec: Tag enum é `Tag::Start(Location, ElementInfo)`
  com payload via ElementInfo, não `Tag::StateDisplay` directo;
  ajuste signature trivial sem `P240.div-N`).
- **`ElementKind::StateDisplay`** variant novo em
  `entities/element_kind.rs`.
- **`apply_state_displays(tags, intr, engine, ctx)`** fixpoint
  function nova em `rules/introspect/from_tags.rs:80+` —
  **paralelo absoluto `apply_state_funcs` P191B**. Chama
  `apply_func(callback, [state.value_at(key, loc)], ctx,
  engine)` pós-walk com Engine+ctx disponíveis; resultado
  convertido para Content (`Value::Content(c)` passa-through;
  `Value::Str(s)` via `Content::text`; outros tipos
  `Content::Empty`; Err defensive ignore paridade P191B).
- **`Introspector::state_display_value(key, location) ->
  Option<Content>`** trait method novo em
  `entities/introspector.rs` + impl em TagIntrospector +
  adapter em `03_infra/src/measurements.rs::CountingIntrospector`.
  Owned `Content` (clone) porque `comemo::Tracked` não permite
  retornar `&Content`.
- **`TagIntrospector.state_displays: HashMap<(String,
  Location), Content>`** storage novo.
- **`native_state_display(key, [callback])`** stdlib func nova
  em `rules/stdlib/foundations.rs` + scope register em
  `rules/eval/mod.rs:618` + re-export em `rules/stdlib/mod.rs`.
  **Stdlib funcs: 62 → 63** (+state_display).
- **Walk integration layout-time arm `Content::StateDisplay`**
  em `rules/layout/mod.rs:355+`:
  ```rust
  Content::StateDisplay { key, callback: _ } => {
      use crate::entities::introspector::Introspector;
      if let Some(loc) = self.current_location {
          if let Some(pre) = self.introspector
              .state_display_value(key.clone(), loc)
          { self.layout_content(&pre); }
      }
  }
  ```
  **Layouter permanece puro** — sem Engine+ctx em signature;
  paridade arquitectural estrita Opção γ vs α/β/δ P239 audit.
- **`extract_payload` arm StateDisplay** em
  `rules/introspect/extract_payload.rs` emite Tag pós-walk.
- **`populate_intr_from_tag_start` arm StateDisplay** em
  `rules/introspect.rs` regista loc em kind_index.
- **Caller** `apply_state_displays(&tags, &mut introspector,
  engine, ctx)` em `fixpoint::run_fixpoint` após
  `apply_state_funcs`.

**Achado material audit C1 P240** (cenário α confirmed para
state.final two-pass): hipótese P238 reescrito "`Func::call`
não existe" refinada — `apply_state_funcs` JÁ EXISTE e avalia
StateUpdate::Func via fixpoint pós-walk com Engine+ctx;
`state_final_value` retorna `history.last()` que reflete valor
two-pass real cumulativo pós-`apply_state_funcs`. **`state_final`
semantic já é two-pass real pós-P240** — paridade vanilla
`state.final()` sem refactor adicional (refino docs apenas).

**Sobreposição bloqueador A + D desbloqueada via M7+1
sozinho**: walk-time eval Func dispatch + state.final two-pass
real ambos resolvidos via Opção γ paralelo `apply_state_funcs`
existente — refinamento empírico face P238 reescrito que
hipotetizou refactor maior.

**8 decisões fixadas P240** (Decisão 0 = lição N=3 cumulativo):
- Decisão 0 — C1 audit obrigatório bloqueante (lição refinada
  P236.div-1 → P238.div-1 → P239 audit aplicada literal).
- Decisão 1 — Opção γ apply_state_displays.
- Decisão 2 — Opção β Content::StateDisplay variant novo.
- Decisão 3 — Opção α refinada empíricamente:
  ElementPayload::StateDisplay (não Tag::StateDisplay).
- Decisão 4 — Opção β paralelismo absoluto.
- Decisão 5 — Walk integration via Introspector trait.
- Decisão 6 — native_state_display 1-2 arg.
- Decisão 7 — Cenário α state.final two-pass trivial (docs).
- Decisão 8 — **L0 partial tocado** (primeira excepção
  ADR-0080 EM VIGOR pós-P229 justificada).

**Pré-condições obrigatórias verificadas P240** (per ADR-0081
§"Pré-condições obrigatórias" P239):
1. Tests baseline preservados: **2150 → 2162 verdes** (+12 novos
   P240; 0 regressões reais; 0 adaptações intencionais).
2. Comemo memoization invariants ADR-0073/0074 preservados
   (trait `Introspector` `#[comemo::track]` continua válido
   com novo method `state_display_value(String, Location)
   -> Option<Content>` compatível com macro).
3. Backward compat eval-time P236 state_final + P237 state_at
   wrappers continuam funcionar inalterados.

- **L0 partial tocado** (3 ficheiros — **primeira excepção
  justificada à aplicação automática ADR-0080 EM VIGOR
  pós-P229**):
  - `00_nucleo/prompts/entities/content.md` — bloco
    `Content::StateDisplay` documentado.
  - `00_nucleo/prompts/rules/stdlib.md` — bloco
    `state_display(key, [callback])` documentado.
  - `00_nucleo/prompts/rules/introspect.md` — bloco
    `apply_state_displays` + `Introspector::state_display_value`
    documentado.

  **ADR-0080 §"Excepção P240"** anotada formalmente. Pattern
  emergente "L0 tocado para features runtime novas + walk
  integration" N=1 inaugurado P240 (primeira aplicação real;
  P236 spec original hipotetizou; rejeitada empíricamente
  pós-divergência). **Pattern "aplicação automática ADR-0080
  EM VIGOR" N=8 preservado** mas **não-incrementa P240**
  (excepção justificada).

- **Patterns emergentes inaugurados/consolidados em P240** (4):
  - **"L0 tocado para features runtime novas + walk
    integration" N=1 inaugurado P240** — primeira aplicação
    real.
  - **"refino aditivo paralelo entre callers fixpoint" N=1
    inaugurado P240** — extensão pattern P191B
    `apply_state_funcs` baseline para `apply_state_displays`.
  - **"spec C1 audit obrigatório bloqueante pós-P236.div-1"**
    N=2 → **3 cumulativo** (P237 + P238 reescrito + P240).
  - **"atomização prep-passo audit-only +
    materialização-passo"** N=1 → **2 cumulativo** (P238
    reescrito → P239 → P240 validação empírica).

- 12 tests adicionados P240 (3 unit content StateDisplay
  PartialEq + plain_text; 4 unit stdlib native_state_display;
  5 unit introspect/fixpoint apply_state_displays cenários
  canónicos); workspace 2150 → **2162 verdes** (+12; 0
  regressões; 0 adaptações).

- **Distribuição ADRs P240**: ADR-0081 PROPOSTO →
  **IMPLEMENTADO parcial** (M7+1 ✓; M7+2 a M7+5 pendentes).
  PROPOSTO 13 → **12** (-1 transita); IMPLEMENTADO 21 →
  **22** (+1 parcial); EM VIGOR 29; total **68 preservado**.
  ADR-0079 Categoria D 1/? → **2/?** anotado. ADR-0080
  §"Excepção P240" anotada. ADR-0066 SUPERSEDED-BY 0073
  preservado.
- **Saldo DEBTs**: 11 preservado.
- **Cobertura Layout per metodologia**: **89% preservado**
  (M7+1 é Introspection refino + walk integration; não Layout
  estrutural).
- **Cobertura user-facing total**: 67% → **~68-70%** (D.2
  state.display walk-time real + state.final two-pass real
  bonus cumulativo marginal).
- **Anti-inflação 32ª aplicação cumulativa pós-P205D** —
  Opção γ pattern reusado (não α/β/δ inflacionárias) + Opção β
  Content::StateDisplay variant novo (não α refino
  Content::State coerência) + Opção α refinada ElementPayload
  (não Tag::StateDisplay) + Opção β paralelismo absoluto +
  Opção γ L0 partial (não α extenso) + Cenário α state.final
  trivial (não γ atomização) + ADR-0081 IMPLEMENTADO parcial
  (não completo prematuro).

**Categoria D Fase 5 Layout: 1/? → 2/? sub-passos materializados**
(D.1 state_final P236 + state_at P237 eval-time wrappers; **D.2
state.display walk-time real P240 + state.final two-pass real
via sobreposição bloqueador A+D**).

**Fase 5 Layout candidata: 10/13-15 → 11/13-15 sub-passos
materializados** (~73-85% cumulativo; **Categoria A 5/5 ✓ +
Categoria B 3/3 ✓ + Categoria D 2/? + Categoria C 0/?**).

**M9d / M7+ progresso**: **1/5 sub-passos materializados**
(M7+1 ✓; M7+2 + M7+3 + M7+4 + M7+5 pendentes — magnitude
cumulativa restante ~18-29h).

**Marco interno P240**: primeira sub-passo materialização
M-fase pós-M9c reabertura iniciada metodologicamente
correctamente — lição refinada P236.div-1 → P238.div-1 →
P239 audit validada empíricamente N=3 cumulativo via
materialização real funcional. Audit C1 P240 refinou hipótese
Tag enum signature (ajuste trivial sem div-N — paridade lição
N=3 P237/P238 reescrito/P240).

**Decisão humana pendente pós-P240**: M7+2 counter.display
paralelo M7+1 (recomendação subjectiva; M ~2-4h; completa
D.2 → D 3/? totalmente real); OU M7+5 A.4 radius/clip
infrastructure (menor magnitude M-L ~3-5h); OU M7+3/M7+4
independentes; OU pivot outro módulo; OU pausa M-fase
(Fase 5 graded ~80-85%).

**Stdlib funcs**: 62 → **63** (+state_display).

⁶⁰ — Ajuste P241 (M9d / M7+2 segunda sub-passo materialização
pós-P240 — **Pipeline walk-time eval via Opção γ
`apply_counter_displays` paralelo absoluto P240 M7+1**;
**segunda aplicação cumulativa pattern "L0 tocado para
features runtime novas + walk integration" pós-ADR-0080 EM
VIGOR P229** N=1 → 2 cumulativo; **quarta aplicação cumulativa
pattern "spec C1 audit obrigatório bloqueante pós-P236.div-1"**
N=3 → 4 cumulativo; ADR-0081 IMPLEMENTADO parcial 1/5 → 2/5;
desbloqueia D.3 counter.display walk-time real):

**P241 materializa M7+2 Opção γ** (per ADR-0081 IMPLEMENTADO
parcial M7+1 P240 + spec P241 §4-§6; paralelo absoluto P240
substituindo `state_display` por `counter_display`):

- **`Content::CounterDisplayCallback { key: String, callback:
  Option<Func> }`** variant novo em `entities/content.rs`
  (distinto de `Content::CounterDisplay { kind }` legacy
  single-pass que coexiste preservada — Decisão 1 P241 Opção
  α naming explícito `CounterDisplayCallback`). **Content
  variants: 61 → 62**.
- **`ElementPayload::CounterDisplay { key, callback }`** variant
  novo em `entities/element_payload.rs` paralelo
  `ElementPayload::StateDisplay` P240.
- **`ElementKind::CounterDisplay`** variant novo em
  `entities/element_kind.rs` + "counter_display" as_str/from_name.
- **`apply_counter_displays(tags, intr, engine, ctx)`** fixpoint
  function nova em `rules/introspect/from_tags.rs` —
  **paralelo absoluto `apply_state_displays` P240**:
  - Converte `intr.counters.value_at(key, loc)` (Option<&[usize]>)
    para `Value::Array(Vec<Value::Int>)` (paridade vanilla
    `CounterState = SmallVec<[u64; 3]>`).
  - Chama `apply_func(callback, [array], ctx, engine)` pós-walk
    com Engine+ctx disponíveis; resultado convertido para Content
    (Value::Content passa-through; Value::Str via Content::text;
    outros tipos / Err → Content::Empty).
  - Sem callback: formato default "1.2.3" via join "." (paridade
    `formatted_counter_at` P177); counter inexistente:
    Content::Empty.
- **`Introspector::counter_display_value(key, location) ->
  Option<Content>`** trait method novo + impl em TagIntrospector
  + adapter em `03_infra/src/measurements.rs::CountingIntrospector`.
- **`TagIntrospector.counter_displays: HashMap<(String, Location),
  Content>`** storage novo.
- **`native_counter_display(key, [callback])`** stdlib func nova
  em `rules/stdlib/foundations.rs` + scope register em
  `rules/eval/mod.rs:624` + re-export em `rules/stdlib/mod.rs`.
  **Stdlib funcs: 63 → 64** (+counter_display).
- **Walk integration layout-time arm
  `Content::CounterDisplayCallback`** em `rules/layout/mod.rs`
  consome via `Introspector::counter_display_value(key, loc)`.
  **Layouter permanece puro** — paridade arquitectural estrita
  P240 preservada (Opção γ vs α/β/δ P239 audit).
- **`extract_payload` arm CounterDisplayCallback** em
  `rules/introspect/extract_payload.rs` emite Tag pós-walk.
- **`populate_intr_from_tag_start` arm CounterDisplay** em
  `rules/introspect.rs` regista loc em kind_index.
- **Caller** `apply_counter_displays(&tags, &mut introspector,
  engine, ctx)` em `fixpoint::run_fixpoint` após
  `apply_state_displays` (sequência cumulativa
  apply_state_funcs → apply_state_displays → apply_counter_displays).

**Forma do Value passado ao callback** (Decisão 4 P241):
`Value::Array(Vec<Value::Int>)` representando counter state
actual (paridade vanilla literal). Counter inexistente:
`Value::Array(vec![])` (vector vazio passado ao callback;
permite distinguir "counter zerado [0]" de "counter inexistente
[]"). Sem callback + counter existente: formato default "1.2.3";
sem callback + inexistente: `Content::Empty`.

**Audit C1 P241 refinou naming** Content variant final
(`CounterDisplayCallback` em vez de `CounterDisplay2`); ajuste
trivial sem `P241.div-N` (lição N=4 cumulativo: ajustes triviais
naming/signature pós-audit não merecem div-N formal).

**Pré-condições obrigatórias verificadas P241** (per ADR-0081
§"Pré-condições obrigatórias" P239):
1. Tests baseline preservados: **2162 → 2175 verdes** (+13 novos
   P241; 0 regressões reais; 0 adaptações intencionais; spec
   previa +10-14 — real dentro do range).
2. Comemo memoization invariants ADR-0073/0074 preservados
   (trait `Introspector` `#[comemo::track]` continua válido com
   novo method `counter_display_value(String, Location) ->
   Option<Content>` compatível com macro; paridade P240).
3. Backward compat: `Content::CounterDisplay { kind }` legacy
   preservada inalterada — todos os tests pré-P241 que usam
   variant legacy continuam intactos; P240 wrappers
   `state_display` + tests preservados; eval-time wrappers
   P236 state_final + P237 state_at intactos.

- **L0 partial tocado** (3 ficheiros — **segunda excepção
  justificada à aplicação automática ADR-0080 EM VIGOR
  pós-P229**, N=1 → 2 cumulativo P240+P241):
  - `00_nucleo/prompts/entities/content.md` — bloco
    `Content::CounterDisplayCallback` documentado.
  - `00_nucleo/prompts/rules/stdlib.md` — bloco
    `counter_display(key, [callback])` documentado.
  - `00_nucleo/prompts/rules/introspect.md` — bloco
    `apply_counter_displays` + `Introspector::counter_display_value`
    documentado.

  **ADR-0080 §"Excepção P241"** anotada formalmente
  cristalizando N=2 cumulativo. Pattern "L0 tocado para features
  runtime novas + walk integration" promove-se a N=2; atinge
  limiar formalização N=3-4 marginal (promoção a sub-categoria
  ADR-0080 candidata se N=3 atinge em sub-passo M7+ futuro).
  **Pattern "aplicação automática ADR-0080 EM VIGOR" N=8
  preservado** mas **não-incrementa P241** (excepção
  justificada).

- **8 decisões fixadas P241** (Decisão 0 = lição N=4 cumulativo):
  - Decisão 0 — C1 audit obrigatório bloqueante (lição refinada
    aplicada N=4 cumulativo P237+P238 reescrito+P240+P241).
  - Decisão 1 — Opção α variant nova paralela
    `CounterDisplayCallback` (não β refino legacy).
  - Decisão 2 — ElementPayload::CounterDisplay paralelo.
  - Decisão 3 — ElementKind::CounterDisplay paralelo.
  - Decisão 4 — Value::Array para counter state.
  - Decisão 5 — Counter inexistente → Array vazio fallback.
  - Decisão 6 — native_counter_display 1-2 arg.
  - Decisão 7 — L0 partial tocado (segunda excepção ADR-0080).
  - Decisão 8 — Tests materializados no mesmo passo.

- **Patterns emergentes inaugurados/consolidados em P241** (3):
  - **"L0 tocado para features runtime novas + walk integration"**
    N=1 → **2 cumulativo** (P240 + P241).
  - **"Refino aditivo paralelo entre callers fixpoint"** N=1 →
    **2 cumulativo** (P240 `apply_state_displays` + P241
    `apply_counter_displays`).
  - **"Spec C1 audit obrigatório bloqueante pós-P236.div-1"**
    N=3 → **4 cumulativo** (P237 + P238 reescrito + P240 + P241).

- 13 tests adicionados P241 (4 unit content
  `CounterDisplayCallback` PartialEq + plain_text + distinto
  legacy; 4 unit stdlib `native_counter_display` cenários
  canónicos; 5 unit introspect/fixpoint `apply_counter_displays`
  history-aware + callback + Err defensive + counter inexistente
  array vazio); workspace 2162 → **2175 verdes** (+13; 0
  regressões; 0 adaptações).

- **Distribuição ADRs P241**: preservada literal — ADR-0081
  transita 1/5 → **2/5** internamente (M7+2 ✓); sem novos ADRs
  criados; sem PROPOSTO ↔ IMPLEMENTADO. PROPOSTO 12 preservado;
  EM VIGOR 29 preservado; IMPLEMENTADO 22 preservado; total
  **68 preservado**. ADR-0079 Categoria D 2/? → **3/?** anotado.
  ADR-0080 §"Excepção P241" anotada. ADR-0066 SUPERSEDED-BY
  0073 preservado.
- **Saldo DEBTs**: 11 preservado.
- **Cobertura Layout per metodologia**: **89% preservado**
  (M7+2 é Introspection refino + walk integration).
- **Cobertura user-facing total**: ~70% → **~71-72%** (D.3
  counter.display walk-time real bonus cumulativo marginal).
- **Anti-inflação 33ª aplicação cumulativa pós-P205D** — Opção
  γ pattern reusado (paridade absoluta P240) + Opção α variant
  nova paralela (não β refino legacy) + Opção α naming explícito
  `CounterDisplayCallback` + Opção α Value::Array paridade
  vanilla + Opção γ L0 partial (segunda excepção justificada
  N=2 cumulativo) + ADR-0081 IMPLEMENTADO parcial 2/5 (não
  prematuro).

**Categoria D Fase 5 Layout: 2/? → 3/? sub-passos materializados**
(D.1 state_final P236 + state_at P237 eval-time wrappers; D.2
state.display walk-time real P240; **D.3 counter.display
walk-time real P241**).

**Fase 5 Layout candidata: 11/13-15 → 12/13-15 sub-passos
materializados** (~80-92% cumulativo; **Categoria A 5/5 ✓ +
Categoria B 3/3 ✓ + Categoria D 3/? + Categoria C 0/?**).

**M9d / M7+ progresso**: **2/5 sub-passos materializados**
(M7+1 ✓; **M7+2 ✓**; M7+3 + M7+4 + M7+5 pendentes — magnitude
cumulativa restante ~16-25h).

**Marco interno P241**: segunda sub-passo materialização M9d
validada — pattern "refino aditivo paralelo entre callers
fixpoint" N=2 cumulativo confirmado empíricamente sem
divergências factuais materiais (paridade absoluta P240
preservada). Lição N=4 cumulativo C1 audit bloqueante refinou
naming variant pós-audit sem div-N (ajuste trivial precedente
P237/P240).

**Decisão humana pendente pós-P241**: M7+5 A.4 radius/clip
infrastructure (recomendação subjectiva; menor magnitude M-L
~3-5h; geometry isolada); OU M7+3 multi-region (L+); OU M7+4
Place float (L); OU ADR meta admin XS (promoção patterns N=2
P240+P241); OU pivot outro módulo; OU pausa M-fase (Fase 5
graded ~80-85%).

**Stdlib funcs**: 63 → **64** (+counter_display).

⁶¹ — Ajuste P242 (M9d / M7+5 terceira sub-passo materialização
pós-P241 — **A.4 radius/clip infrastructure**; **primeira
sub-passo M7+ não-pipeline**; **primeira promoção real graded
ADR-0054 de scope-out P156G/H → semantic concreta**; sub-padrão
#14 "Tipo entity em ficheiro próprio" N=5 → 6 cumulativo;
sub-padrão "Reuso template helpers extract_*" N=3 → 4 cumulativo;
**terceira excepção justificada ADR-0080 EM VIGOR pós-P229**
sub-categoria nova "geometry/exporter"; quinta aplicação
cumulativa pattern "spec C1 audit obrigatório bloqueante
pós-P236.div-1" N=4 → 5 cumulativo; ADR-0081 IMPLEMENTADO
parcial 2/5 → 3/5; ADR-0079 Categoria A.4 scope-out P231 →
materializado parcial P242):

**P242 materializa M7+5** (per ADR-0081 IMPLEMENTADO parcial
+ spec P242):

- **`Corners<T>`** tipo entity novo em
  `01_core/src/entities/corners.rs` (paralelo absoluto `Sides<T>`
  P156C; derives Debug/Clone/Copy/PartialEq/Eq + `new` + `uniform`
  + `Default` para `T: Default`). **Sub-padrão #14 "Tipo entity
  em ficheiro próprio" N=5 → 6 cumulativo**: Sides (P156C) →
  Parity (P156E) → Dir (P156I) → BibEntry (P159A) → CitationForm
  (P159C) → **Corners (P242)**.
- **`ShapeKind::RoundedRect { radii: Corners<Length> }`** variant
  novo em `entities/geometry.rs`. **ShapeKind variants: 4 → 5**
  (+RoundedRect). Coexiste com `Rect`/`Ellipse`/`Line`/`Path`.
  Degeneração estrutural preservada (radii zero ≠ Rect; PartialEq
  distinto).
- **Refino tipo `Content::Block.radius`** `Option<Length>` →
  `Corners<Length>` per-corner. **Audit C1 P242 refinou hipótese
  spec**: assumira "5 fields → 7" mas Block/Boxed já tinham 8
  fields P231 (`outset` + `radius` + `clip` semantic adiada
  graded); ajuste real = **refine field type** (não add). Paridade
  lição N=5 cumulativo P237/P240/P241 — **sem `P242.div-N`
  formal**.
- **Refino tipo `Content::Boxed.radius`** idem paralelo.
- **`extract_corners_length_value(value, fn_name)`** helper novo
  em `rules/stdlib/layout.rs` (paralelo `extract_sides_lengths`
  P156L). **Sub-padrão "Reuso template helpers extract_*" N=3 →
  4 cumulativo**.
- **stdlib `block(radius:)` + `box(radius:)`** aceitam:
  - **Length uniforme** (paridade pre-P242): `Corners::uniform(L)`.
  - **Dict por canto**: chaves `top-left`/`top-right`/
    `bottom-right`/`bottom-left`/`top`/`bottom`/`left`/`right`/
    `rest`; **precedência específico > eixo > rest** (paridade
    `extract_sides_lengths` per ADR-0064 Caso C).
  - Validação: negativos rejeitados; chaves canto inválidas
    rejeitadas.
- **Layouter Block arm** (`rules/layout/mod.rs`):
  - `clip == true` + radius non-zero: emite `FrameItem::Group`
    com `clip_mask: Some(ShapeKind::RoundedRect { radii: radius })`.
  - `clip == true` + radius zero: `clip_mask: Some(ShapeKind::Rect)`
    (paridade DEBT-30 P79).
  - `clip == false`: comportamento inline original preservado
    (radius armazenado sem clip-mask emit; semantic radius
    isolada continua graded per Decisão 6 spec P242).
  - Algoritmo: snapshot-and-extract — `regions.current.current_items.len()`
    pre/pós layout_content body, drain itens emitidos, re-emit
    como Group com clip_mask.
- **PDF exporter `emit_rounded_rect_ops`** helper novo em
  `03_infra/src/export.rs`:
  - Desenha Bezier 4 corners path via operadores PDF `m`/`l`/`c`/`h`.
  - **Kappa** `0.552_284_749_831` (paridade `ShapeKind::Ellipse`
    mesmo ficheiro; minimiza erro quarto de círculo).
  - **Clamp radii** a `min(w, h) / 2.0` (evita overflow geométrico
    paridade vanilla).
  - Sequência horária após canto top-left; skip cubic arms quando
    raio canto = 0.
  - Reusado em **5 sítios cross-arm** (Shape global + Shape
    local 2× + Group clip_mask path 2×).

**Promoção real graded ADR-0054 P156G/H → semantic concreta**:

- P156G/P156H declararam `radius` + `clip` scope-out com rejeição
  hard em stdlib.
- P231 promoveu para fields graded ("semantic adiada"): `radius:
  Option<Length>` + `clip: bool` aceites em stdlib mas sem render
  real.
- **P242 materializa semantic real**: `radius` refinada per-corner;
  `clip` emite clip_mask via Layouter; PDF exporter desenha Bezier
  path. **Sub-padrão emergente "promoção real scope-out ADR-0054
  graded" N=1 inaugurado P242** — distinto de:
  - Refino qualitativo (P156L Pad sides Length → Option<Length>).
  - Refactor cosmético (P158C Figure.kind String → Option<String>).
  - **Sub-categoria nova**: scope-out P156G/H P231 "semantic
    adiada" → semantic concreta + render PDF real.

**Categoria A.4 ADR-0079** transita scope-out P231 →
**materializado parcial P242** (radius + clip ✓; outset + fill +
stroke restantes N=3 permanecem scope-out P156G/H — refino futuro
candidato).

**Pré-condições obrigatórias verificadas P242** (per ADR-0081
§"Pré-condições obrigatórias"):
1. **Tests baseline preservados**: 2175 verdes pré-P242 → **2190
   verdes pós-P242** (+15 novos; 0 regressões reais; **7
   adaptações triviais** tests pré-existentes P231 que usavam
   `radius: Some(len)` → `radius: Corners::uniform(len)` ou
   `Corners::ZERO`).
2. Comemo memoization invariants ADR-0073/0074 preservados — P242
   NÃO toca trait Introspector nem methods (refino geometry
   isolada cross-camada L1/L3).
3. Backward compat: stdlib `block(radius: 5pt)` continua a
   funcionar via `Corners::uniform`; tests P231 adaptados; eval-time
   wrappers P236/P237 + walk-time runtime P240/P241 intactos.

- **L0 partial tocado** (4 ficheiros — **terceira excepção
  justificada à aplicação automática ADR-0080 EM VIGOR pós-P229**,
  sub-categoria nova "geometry/exporter infrastructure" distinta
  de P240/P241 "walk-time runtime"):
  - `00_nucleo/prompts/entities/corners.md` — **ficheiro novo**.
  - `00_nucleo/prompts/entities/geometry.md` — secção
    `ShapeKind::RoundedRect`.
  - `00_nucleo/prompts/entities/content.md` — refino
    `Block.radius` + `Boxed.radius` + materialização clip semantic.
  - `00_nucleo/prompts/infra/export.md` — secção rounded-rect
    clip path Bezier 4 corners.

  **ADR-0080 §"Excepção P242"** anotada formalmente. Pattern
  emergente total "L0 tocado pós-P229 (sub-categorias)" **N=3
  cumulativo com 2 sub-categorias formalizadas** (walk-time
  N=2 P240+P241 + geometry/exporter N=1 P242).

- **9 decisões fixadas P242** (Decisão 0 = lição N=5 cumulativo):
  - Decisão 0 — C1 audit obrigatório bloqueante.
  - Decisão 1 — `Corners<T>` paralelo absoluto `Sides<T>`.
  - Decisão 2 — `ShapeKind::RoundedRect` novo.
  - Decisão 3 — Refino tipo radius (não add).
  - Decisão 4 — Opção α radius Length OR Dict.
  - Decisão 5 — clip semantic materializada.
  - Decisão 6 — radius sem clip preserva graded.
  - Decisão 7 — L0 partial tocado (terceira excepção ADR-0080).
  - Decisão 8 — Promoção real graded scope-out (sub-padrão N=1).
  - Decisão 9 — Sem fechamento Fase 5 graded.

- **Patterns emergentes inaugurados/consolidados em P242** (4):
  - **"Promoção real scope-out ADR-0054 graded" N=1 inaugurado
    P242** — sub-padrão novo.
  - **"Tipo entity em ficheiro próprio" (sub-padrão #14)** N=5 →
    **6 cumulativo** (Corners adiciona-se).
  - **"Reuso template helpers extract_*"** N=3 → **4 cumulativo**
    (extract_corners_length_value via template
    extract_sides_lengths).
  - **"Spec C1 audit obrigatório bloqueante pós-P236.div-1"** N=4
    → **5 cumulativo** (P237 + P238 reescrito + P240 + P241 +
    P242).

- 15 tests adicionados P242 (4 unit corners + 2 unit geometry
  RoundedRect + 6 unit stdlib radius dict/precedência/validação +
  3 unit/E2E layout clip_mask emit); workspace 2175 → **2190
  verdes** (+15; 0 regressões; 7 adaptações triviais).

- **Distribuição ADRs P242**: preservada literal — ADR-0081
  transita 2/5 → **3/5** internamente (M7+5 ✓); sem novos ADRs
  criados; sem PROPOSTO ↔ IMPLEMENTADO. PROPOSTO 12 preservado;
  EM VIGOR 29 preservado; IMPLEMENTADO 22 preservado; total **68
  preservado**. ADR-0079 Categoria A.4 scope-out P231 →
  **materializado parcial P242** anotado. ADR-0080 §"Excepção
  P242" sub-categoria nova "geometry/exporter" anotada. ADR-0066
  SUPERSEDED-BY 0073 preservado.
- **Saldo DEBTs**: 11 preservado.
- **Cobertura Layout per metodologia**: 89% → **~91-92%** (refino
  qualitativo+quantitativo — **primeira aplicação Layout
  pós-P156L** pós série Model P157-P159 + série M7+ P240-P241).
- **Cobertura user-facing total**: ~72% → **~73-74%** (A.4
  radius/clip real bonus cumulativo).
- **Anti-inflação 34ª aplicação cumulativa pós-P205D** — Opção α
  Corners paralelo + Opção α RoundedRect novo + Opção α refino
  tipo (não add) + Opção α Length OR Dict + Opção γ L0 partial
  terceira excepção sub-categoria nova + Opção β snapshot-extract
  (não refactor pipeline) + Opção α sub-padrão promoção real
  scope-out + ADR-0081 IMPLEMENTADO parcial 3/5 (não completo).

**Categoria A.4 Fase 5 Layout**: scope-out P231 → **materializado
parcial P242** (radius + clip ✓; outset + fill + stroke restantes
N=3 permanecem scope-out P156G/H — refino futuro candidato S-M
cada).

**Categoria D Fase 5 Layout: 3/? sub-passos materializados**
preservado (D.1 eval-time wrappers; D.2 walk-time real P240;
D.3 counter.display real P241; P242 é Categoria A não D).

**Fase 5 Layout candidata: 12/13-15 → 13/13-15 sub-passos
materializados** (~85-92% cumulativo; **Categoria A 5/5 ✓ (A.4
parcial materializado P242) + B 3/3 ✓ + D 3/? + C 0/?**).

**M9d / M7+ progresso**: **3/5 sub-passos materializados** (M7+1
✓ P240; M7+2 ✓ P241; **M7+5 ✓ P242**; M7+3 + M7+4 pendentes —
magnitude cumulativa restante ~13-20h).

**Marco interno P242**: terceira sub-passo M9d validada;
**primeira sub-passo M7+ não-pipeline** (P240/P241 walk-time
refactor vs P242 geometry isolada); primeira aplicação real do
sub-padrão "promoção real scope-out ADR-0054 graded" pós-2 anos
de aplicações apenas de "field armazenado semantic adiada"
graded. Audit C1 P242 refinou hipótese spec fields sem div-N —
paridade lição N=5 cumulativo precedente. **Distinção qualitativa
P242 vs P240/P241**: refino qualitativo+quantitativo (Layout +2
pontos percentuais per metodologia) vs refino apenas qualitativo
(Introspection P240/P241).

**Decisão humana pendente pós-P242**: M7+3 multi-region completion
(recomendação subjectiva; L+ ~8-12h; maior desbloqueio cumulativo
restante — C.2 + A.4 breakable per-cell); OU M7+4 Place float
real (L); OU refino A.4 outset/fill/stroke (S-M); OU ADR meta
admin XS (promoção patterns N=2-4 acumulados P240/P241/P242);
OU pivot outro módulo; OU pausa M-fase (Fase 5 graded ~85-92%
com 13/13-15 sub-passos materializados).

**Stdlib funcs**: 64 preservado. **ShapeKind variants**: 4 →
**5**. **Tipos entity**: +1 Corners<T>.

⁶² — Ajuste P243 (M9d / M7+3 fase (a) quarta sub-passo
materialização pós-P242 — **infrastructure-only do plano duas-fases
DEBT-56**; **primeira sub-passo M7+ não-pipeline #2** (P242 já foi
não-pipeline); **sub-padrão "promoção real scope-out ADR-0054
graded" N=1 → 2 cumulativo**; **quarta excepção justificada
ADR-0080 EM VIGOR pós-P229 sub-categoria nova "Layouter internal
refactor"**; sexta aplicação cumulativa pattern "spec C1 audit
obrigatório bloqueante pós-P236.div-1" N=5 → 6 cumulativo;
ADR-0081 IMPLEMENTADO parcial 3/5 → 4/5):

**Achado material audit C1 P243**: spec hipotetizou **refactor
profundo cross-module L+** (5-7 fields migrar `cursor_x`/`cursor_y`/
`line_start_x`/`current_items`/`current_line` + ~30-50 sítios
adaptação). Reality empírica: refactor field-agregation **já feito
em P216A + P216B** (Region struct em `01_core/src/entities/region.rs`
+ Regions wrapper + Layouter `regions: Regions` field +
`flush_line`/`new_page` actualizados). **P243 reduz para extensão
`Regions`** com `backlog` + `last` + `advance` method + promoção
scope-outs via `regions.current.width` save/restore. **Magnitude
real M (~2-3h)** face L+ (~8-12h) hipotetizado. **Sem `P243.div-N`**
— paridade lição N=6 cumulativo precedente P237/P240/P241/P242
(audit refinou hipótese spec sem div-N formal).

**P243 materializa M7+3 fase (a)** (per ADR-0081 IMPLEMENTADO
parcial 3/5 + spec P243):

- **Extensão `Regions` struct** em `01_core/src/entities/region.rs`:
  - `pub backlog: Vec<Region>` field novo — fase (b) populated
    quando `Content::Columns` materializar.
  - `pub last: Option<Region>` field novo — fase (b) overflow/
    fallback.
  - **`pub fn advance(&mut self) -> Option<Region>`** method novo:
    - `backlog` não-vazio (fase (b)): move current → last;
      consome próximo backlog como novo current; retorna prev.
    - `backlog` vazio (fase (a)): retorna None; caller cria
      nova region externa. Preserva semantic P216A/B literal.
  - Paridade vanilla simplificada per ADR-0078 PROPOSTO
    §"Decisão" — subset essencial (omite `expand`/`full`/`root`).
- **Promoção real ≥3 scope-outs multi-region** via
  `regions.current.width` save/restore em `01_core/src/rules/layout/mod.rs`:
  - **`Pad.right` scope-out P156C** → semantic real P243:
    `self.regions.current.width = (saved_width - right).max(0.0)`
    durante body layout; restaurado pós-body. Width-aware wrap
    em `layout_word` consome largura útil reduzida.
  - **`Block.width` semantic adiada P156G** → semantic real P243:
    `self.regions.current.width = (line_start + w_pt).max(0.0)`
    quando `Some(w)`.
  - **`Boxed.width` semantic adiada P156H** → semantic real P243:
    paralelo Block via `cursor_x + w_pt`.

**Pré-condições obrigatórias verificadas P243** (per ADR-0081
§"Pré-condições obrigatórias"):
1. **Tests baseline preservados**: 2190 verdes pré-P243 → **2198
   verdes pós-P243** (+8 novos; 0 regressões reais; **0 adaptações
   intencionais** — extensão aditiva não-disruptive).
2. Comemo memoization invariants ADR-0073/0074 preservados — P243
   NÃO toca trait Introspector nem methods (refino L1 interno
   isolado).
3. Backward compat: stdlib `block(width: 100pt)` continua a
   funcionar (semantic agora real); tests pré-P243 que usavam
   Block.width/Boxed.width/Pad.right como scope-outs preservados
   inalterados (`let _ = width;` foi removido; comportamento
   anterior — body layouted with default region.width — ainda
   funciona).

- **L0 partial tocado** (2 ficheiros — **quarta excepção
  justificada à aplicação automática ADR-0080 EM VIGOR pós-P229**,
  sub-categoria nova "Layouter internal refactor" distinta de
  P240/P241 walk-time + P242 geometry/exporter):
  - `00_nucleo/prompts/entities/region.md` — secção extensão
    `Regions` `backlog`/`last`/`advance` + sub-padrão promoção
    real scope-out N=2.
  - `00_nucleo/prompts/entities/content.md` — secção promoção
    scope-outs Pad.right + Block.width + Boxed.width.

  **ADR-0080 §"Excepção P243"** anotada formalmente. Pattern total
  "L0 tocado pós-P229 (sub-categorias)" N=3 → **4 cumulativo**
  com **3 sub-categorias formalizadas**: walk-time (N=2 P240+P241);
  geometry/exporter (N=1 P242); **Layouter internal refactor
  (N=1 P243)** ← inaugurada.

- **10 decisões fixadas P243** (Decisão 0 = lição N=6 cumulativo):
  - Decisão 0 — C1 audit obrigatório bloqueante (audit refinou
    hipótese fields já-aggregados P216A/B).
  - Decisão 1 — Regions extensão (paralelo conceptual
    LayouterRuntimeState P190C).
  - Decisão 2 — Migração field-by-field já feita P216A/B (audit
    finding material).
  - Decisão 3 — Fase (a) preserva single-region observable literal.
  - Decisão 4 — Promoção real ≥3 scope-outs.
  - Decisão 5 — Sem Content::Columns/Colbreak em P243.
  - Decisão 6 — Sem ADR column flow algorithm em P243.
  - Decisão 7 — cell_available_h integration diferida (passo
    futuro NÃO reservado).
  - Decisão 8 — Nova sub-categoria ADR-0080 "Layouter internal
    refactor".
  - Decisão 9 — Tests focam preservação observable.
  - Decisão 10 — Sem fechamento Fase 5 / ADR-0061 / DEBT-56.

- **Patterns emergentes inaugurados/consolidados em P243** (4):
  - **"Refactor profundo Layouter internal" N=1 inaugurado P243**
    — sub-padrão novo (magnitude reduzida vs spec por P216A/B
    precedente).
  - **"Sub-categoria ADR-0080 nova"** N=2 → **3 cumulativo**
    (walk-time P240+P241; geometry/exporter P242; **Layouter
    internal refactor P243** inaugurada).
  - **"Promoção real scope-out ADR-0054 graded"** N=1 → **2
    cumulativo** (P242 radius/clip + **P243 multi-region attrs
    Pad.right + Block.width + Boxed.width**). Atinge limiar
    formalização N=2 — candidato a ADR meta passo administrativo
    XS futuro.
  - **"Spec C1 audit obrigatório bloqueante pós-P236.div-1"** N=5
    → **6 cumulativo** (P237 + P238 reescrito + P240 + P241 +
    P242 + P243).

- 8 tests adicionados P243 (4 unit regions backlog/last/advance/
  clone preserva + 4 unit/E2E layout scope-outs promoção); workspace
  2190 → **2198 verdes** (+8; 0 regressões; 0 adaptações).

- **Distribuição ADRs P243**: preservada literal — ADR-0081
  transita 3/5 → **4/5** internamente (M7+3 fase (a) ✓); sem
  novos ADRs criados; sem PROPOSTO ↔ IMPLEMENTADO. PROPOSTO 12
  preservado; EM VIGOR 29 preservado; IMPLEMENTADO 22 preservado;
  total **68 preservado**. ADR-0079 Categoria A.4 preservada P242
  parcial. ADR-0080 §"Excepção P243" sub-categoria nova "Layouter
  internal refactor" anotada. ADR-0066 SUPERSEDED-BY 0073
  preservado.
- **Saldo DEBTs**: 11 preservado (DEBT-56 §"Plano" checklist ✓
  item 1 anotado P243 fase (a); fase (b) pendente preserva
  DEBT-56 aberta).
- **Cobertura Layout per metodologia**: ~91-92% → **~93-94%**
  (refino qualitativo + parcial quantitativo via 3 scope-outs
  promovidos a real).
- **Cobertura user-facing total**: ~73-74% → **~74-75%**
  (scope-outs promovidos bonus marginal — Block.width agora
  efectivo desbloqueia uso real).
- **Anti-inflação 35ª aplicação cumulativa pós-P205D** — Opção α
  extensão Regions paralelo + Opção α `backlog/last` fields novos
  + Opção α `advance` method novo + Opção α promoção real
  scope-outs save/restore + Opção γ L0 partial quarta excepção
  sub-categoria nova + ADR-0081 IMPLEMENTADO parcial 4/5 (não
  completo prematuro) + Opção β `cell_available_h` integration
  diferida (Decisão 7).

**Categoria A.4 Fase 5 Layout**: preservada P242 parcial (radius+
clip ✓; outset+fill+stroke N=3 restantes scope-out preservado).

**Categoria D Fase 5 Layout: 3/? sub-passos materializados**
preservado (D.1+D.2+D.3 pós-P241; P242+P243 são Categoria A/
refactor).

**Fase 5 Layout candidata: 13/13-15 → 14/13-15 sub-passos
materializados** (~93-100% cumulativo).

**M9d / M7+ progresso**: **4/5 sub-passos materializados** (M7+1
✓ P240; M7+2 ✓ P241; **M7+3 fase (a) ✓ P243**; M7+5 ✓ P242;
M7+3 fase (b) + M7+4 pendentes — cumulativa restante ~10-16h).

**DEBT-56 §"Plano" checklist** anotado pós-P243:
- ✓ "Refactor minimal `Layouter` para multi-region" — P243 fase
  (a) (extensão `Regions` backlog+last+advance) + P216A/B
  precedente (Region struct + agregação fields).
- ✗ ADR dedicada column flow — fase (b) pendente.
- ✗ `Content::Columns` + `Content::Colbreak` — fase (b) pendente.
- ✗ `native_columns` + `native_colbreak` — fase (b) pendente.
- ✗ Layouter consumer multi-column — fase (b) pendente.
- ✗ Tests + inventário 148 + DEBT fecho — fase (b) pendente.

**Marco interno P243**: quarta sub-passo M9d validada; **primeira
fase (a) duas-fases DEBT-56** materializada (infrastructure-only);
audit C1 refinou hipótese spec material sem div-N (paridade lição
N=6 cumulativo); sub-padrão "promoção real scope-out ADR-0054
graded" atinge limiar formalização N=2; pattern "sub-categoria
ADR-0080 nova" N=3 cumulativo confirma robustez sistema excepções
EM VIGOR pós-P229.

**Decisão humana pendente pós-P243**: M7+3 fase (b) (recomendação
subjectiva; sequência natural pós-fase (a); L ~5-8h; fecha DEBT-56
+ completa M7+3 + promove potencialmente ADR-0061 → IMPLEMENTADO);
OU M7+4 Place float real (L; isolada); OU cell layout migration
(M; activa A.4 breakable per-cell — Decisão 7 P243 diferida);
OU refino A.4 outset/fill/stroke (S-M); OU ADR meta admin XS;
OU pivot outro módulo; OU pausa M-fase.

**Stdlib funcs**: 64 preservado. **ShapeKind variants**: 5
preservado. **Regions fields**: 1 → **3** (+backlog +last).
**Regions methods**: +1 (`advance`). **Scope-outs promovidos**:
3 (Pad.right + Block.width + Boxed.width).

⁶³ — Ajuste P245 (M9d / M7+4 quinta e última sub-passo
materialização pós-P244 — **Place float real**; **fecha
ADR-0081 IMPLEMENTADO total 5/5**; **promoção real graded P223
→ semantic activa consumer**; sub-padrão emergente **"Promoção
graded → real semantic activação consumer" N=1 inaugurado**;
**Categoria C.1 Fase 5 Layout transita pendente → CUMPRIDO**;
oitava aplicação cumulativa pattern "spec C1 audit obrigatório
bloqueante pós-P236.div-1" N=7 → 8 cumulativo):

**P245 materializa M7+4 Place float real**:

- **Novo struct `DeferredFloat`** local em
  `01_core/src/rules/layout/mod.rs` (`pub(super)`; **não L1
  entity** — buffer entry específico ao módulo `layout/`):
  - `alignment: Align2D`, `body_items: Vec<FrameItem>`,
    `body_height: f64`, `body_width: f64`, `clearance: f64`.
- **3 fields novos no Layouter**:
  - `floats_pending: Vec<DeferredFloat>` — buffer floats
    pendentes na página actual.
  - `cursor_y_top_reserve: f64` — espaço reservado top
    (futuro reserve mechanism per anti-collision).
  - `cursor_y_bottom_reserve: f64` — espaço reservado bottom.
- **Arm `Content::Place { float: true, .. }` activa em
  `mod.rs:916`**:
  - `layout_sub_frame_with_width` captura body items + altura.
  - `measure_content` extrai content_w para alignment.x final.
  - Clearance resolved via `resolve_pt(font_size)`.
  - Push ao `floats_pending` (buffer linear).
  - **Cursor.y NÃO avança** (paridade vanilla — float não
    consome flow space direct).
- **`float: false` preservado P84.5+P84.6 literal** (Decisão 3
  spec): comportamento in-place via cursor — sem path mudado.
- **`flush_pending_floats` method novo** em `cursor.rs`:
  - Separa `floats_pending` em top vs bottom via
    `alignment.v == Top`.
  - Top stack do topo área útil (`area_top = margin`); bottom
    stack do fundo área útil (`area_bot = page_h - margin`).
  - Clearance afasta float da margem (e stack-up neighbor).
  - `floats_pending.clear()` após emit.
- **`emit_deferred_float` helper novo**:
  - Translate items locais (origem 0,0 + ascender) para
    coordenadas finais.
  - **Correcção ascender**: `target_y -= ascender.0` (paridade
    pattern `layout_place` em `placement.rs`); items shape
    posicionam-se exactamente em `target_y` final, não baseline.
  - `alignment.x` per `Align2D::h`:
    `Left|None` → `target_x = margin`; `Center` → centrado em
    `(avail_w - body_width)/2`; `Right` → `avail_w - body_width`.
- **`new_page()` e `finish()` chamam `flush_pending_floats`**
  antes da transição de página / commit final.
- **Reset `cursor_y_top_reserve` + `cursor_y_bottom_reserve`**
  em `new_page` (nova página inicia limpa).

**DEBT-37 sentinela** `scope: Parent + float: true` exigido
(P223) **preservada literal** — `native_place` continua a
rejeitar `scope: parent + float: false`.

**Categoria C.1 Fase 5 Layout** transita **pendente → CUMPRIDO
P245** ✓. ADR-0079 desbloqueada para promoção potencial →
IMPLEMENTADO graded (depende de Categoria C.2 multi-region
completion OR scope-out humano — decisão diferida pós-P245).

**Pré-condições obrigatórias verificadas P245** (per ADR-0081
§"Pré-condições obrigatórias" P239):
1. **Tests baseline preservados**: 2198 verdes pré-P245 →
   **2203 verdes pós-P245** (+5 novos P245; 0 regressões reais;
   **0 adaptações intencionais** — extensão non-disruptive).
2. **Comemo memoization invariants ADR-0073/0074 preservados**:
   P245 toca Layouter consumer apenas; trait `Introspector`
   intocada; sub-stores trackable F3 intocados.
3. **Backward compat**: `Content::Place { float: false }`
   preserva P84.5+P84.6 literal; tests P223 storage
   preservados; eval-time wrappers P236/P237 + walk-time
   runtime P240/P241 + geometry P242 + Regions P243 intactos.

- **L0 partial NÃO tocado** — paridade P243 Layouter internal
  refactor sub-categoria. **Sub-categoria nova ADR-0080
  "Layouter internal refactor (semantic activation)"** N=1 →
  **2 cumulativo** (P243 extensão Regions + scope-outs
  promovidos + **P245 Place float semantic activa**). Pattern
  total "L0 tocado pós-P229 (sub-categorias)" N=4 cumulativo
  preservado (não-incrementa P245 — sub-categoria L0
  preserva).

- **9 decisões fixadas P245** (Decisão 0 = lição N=7 → 8
  cumulativo):
  - Decisão 0 — C1 audit obrigatório bloqueante.
  - Decisão 1 — Buffer `floats_pending: Vec<DeferredFloat>`.
  - Decisão 2 — Arm Place float: true → buffer.
  - Decisão 3 — Place sem float preservado P84.5+P84.6 literal.
  - Decisão 4 — Flush em `new_page` + `finish`.
  - Decisão 5 — Reserva espaço top/bottom para floats.
  - Decisão 6 — `scope: Parent + float: true` real (DEBT-37
    sentinela preservada P223).
  - Decisão 7 — Sem tipo entity novo; sem ADR nova; sub-categoria
    "Layouter internal refactor (semantic activation)" preserva
    P243 pattern.
  - Decisão 8 — Anti-inflação 37ª aplicação cumulativa
    pós-P205D.
  - Decisão 9 — Padrão emergente "Promoção graded → real
    semantic" inaugurado N=1.

- **Patterns emergentes inaugurados/consolidados em P245** (3):
  - **"Promoção graded → real semantic activação consumer" N=1
    inaugurado P245** — sub-padrão novo (storage P223 graded
    → semantic activa P245 cross-passo). Candidato a
    formalização N=3-4 futuro.
  - **"Spec C1 audit obrigatório bloqueante pós-P236.div-1"**
    N=7 → **8 cumulativo** (P237 + P238 reescrito + P240 +
    P241 + P242 + P243 + P244 + **P245**). Lição refinada
    N=8: "grep fields/arms já implementados antes de assumir
    trabalho original" (extensão da lição P244 "grep variants
    candidatas").
  - **"Layouter internal refactor (semantic activation)"** N=1
    → **2 cumulativo** (P243 + P245). Sub-categoria 4ª
    formalizada ADR-0080 §"Lição refinada P245".

- 5 tests adicionados P245 (1 unit float bottom + 1 unit float
  top + 1 unit float false baseline + 1 unit clearance + 1
  unit buffer flush); workspace 2198 → **2203 verdes** (+5;
  0 regressões; 0 adaptações).

- **Distribuição ADRs P245**: ADR-0081 IMPLEMENTADO parcial
  4.5/5 → **IMPLEMENTADO total 5/5** ✓. **PROPOSTO 12
  preservado; EM VIGOR 29 preservado; IMPLEMENTADO 22 → 23**
  (+1 ADR-0081 transita parcial → total); total **68
  preservado**. ADR-0079 Categoria C.1 transita **pendente
  → CUMPRIDO P245**. ADR-0080 sub-categoria nova "Layouter
  internal refactor (semantic activation)" N=2 cumulativo
  anotada. ADR-0066 SUPERSEDED-BY 0073 preservado.
- **Saldo DEBTs**: 11 preservado (DEBT-37 sentinela preservada
  P223; nada novo).
- **Cobertura Layout per metodologia**: **~93-94% preservado**
  (P245 promove `place(...)` graded → real — refino qualitativo
  para entrada já implementado⁺).
- **Cobertura user-facing total**: ~74-75% → **~75-76%** (Place
  float real bonus cumulativo).
- **Anti-inflação 37ª aplicação cumulativa pós-P205D** —
  Opção α extensão Layouter (não L1 entity) + Opção α arm
  consumer activado (não nova variant) + Opção α flush em
  pontos canónicos (`new_page` + `finish`) + Opção α
  preservação `float: false` literal + Opção β L0 intocados
  + Opção α promoção ADR-0081 IMPLEMENTADO total + Opção α
  Categoria C.1 cumprida + Opção α sub-padrão N=1 inaugurado
  (sem ADR meta prematura).

**Categoria C.1 Fase 5 Layout**: **pendente → CUMPRIDO P245** ✓.
**Categoria C.2 Fase 5 Layout**: pendente (cell-level
multi-region; scope-out humano candidato pós-P245).

**Fase 5 Layout candidata: 14/13-15 → 15/13-15 sub-passos
materializados** (~100% cumulativo se C.2 scope-out humano).

**M9d / M7+ progresso**: **5/5 sub-passos materializados** ✓✓✓
COMPLETO (M7+1 ✓ P240; M7+2 ✓ P241; M7+3 ✓ via cumulativo
P243 + Linha A; M7+4 ✓ **P245**; M7+5 ✓ P242).

**Marco interno P245**: M9d / M7+ COMPLETO total via Linha B;
ADR-0081 fecha 5/5 IMPLEMENTADO total; Categoria C.1 Fase 5
Layout cumprida; **primeiro sub-padrão "Promoção graded → real
semantic activação consumer" inaugurado** (P223 graded → P245
real cross-passo); audit C1 lição N=8 cumulativo refinada
("grep fields/arms já implementados antes de assumir trabalho
original" — extensão da lição P244).

**Decisão humana pendente pós-P245**:
- Promoção ADR-0079 → IMPLEMENTADO graded (scope-out humano
  Categoria C.2). XS-S magnitude.
- Cell layout migration → `regions.current.height` (M ~2-4h;
  Decisão 7 P243 diferida; activa A.4 breakable per-cell).
- Refino A.4 outset/fill/stroke (S-M por attr).
- ADR meta admin XS (formalizar pattern "passo administrativo
  XS" N=6 ou novos N=2 P244/P245).
- Pivot outro módulo OR pausa M-fase.

**Stdlib funcs**: 64 preservado. **ShapeKind variants**: 5
preservado. **Layouter fields**: +3 (`floats_pending`,
`cursor_y_top_reserve`, `cursor_y_bottom_reserve`). **Layouter
struct local**: +1 `DeferredFloat` (não L1 entity). **Layouter
methods novos**: 2 (`flush_pending_floats` +
`emit_deferred_float`).

³⁹ — Ajuste P214 (Tabela A.1 Markup syntactic — recálculo
ampliado pós-M9c 2026-05-12): **3 reclassificações
empíricas materiais** detectadas em §A.1 face ao estado
factual pós-P212 (M9c ACEITE):
- `> blockquote`: ausente → **implementado** (P155 Fase 1
  sub-passo 2 — `Content::Quote` materializado com 4 attrs;
  par `quote(...)` na §A.6 Model L174 já registrado).
- `/ term: definition`: ausente → **implementado** (P154B
  Fase 1 sub-passo 1 — `Content::Terms` + `Content::TermItem`
  materializados; par `terms(...)` na §A.6 Model L173).
- Smart quotes: ausente → **implementado** (P155 — alternância
  open/close lang-aware via `rules/lang/quotes.rs`; 6 idiomas
  suportados).
Distribuição Markup pós-recálculo: **8/3/3/4/0 → 11/3/3/1/0
= 18** (3 ausente → implementado). Cobertura: 11/18=61% →
**14/18=78%** (Δ +17pp).
**Total user-facing**: `66/24/25/24/2 = 141 → 69/24/25/21/2
= 141` (3 entradas movidas de ausente para implementado;
total preservado). Cobertura user-facing total:
(66+24)/141=63.8% → (69+24)/141=**~66%** (Δ +2pp).
**Sincronizações §2.1 ↔ Tabela A** documentadas
adicionalmente (sem material change na Tabela A; §2.1
desactualizado desde 2026-04-25):
- Layout §2.1 38% → **78%** (Tabela A já 14/18=78% via
  footnotes ⁵⁶⁸¹⁰¹²¹³¹⁵¹⁷¹⁹²¹).
- Model §2.1 45% → **50%** (Tabela A já 11/22=50% via
  footnotes ¹²³²²²⁴²⁹).
**Política "sem novas reservas" preservada per P158**:
candidatos identificados em P213 §A.9 (Bloco A `measure`,
Bloco B `position()` standalone, `query_count_before`)
mantêm-se NÃO reservados; P214 não adiciona novos.
Tests +0 (1939 inalterados — recálculo documental sem
código tocado). 0 violations preservadas.
**Pattern emergente "diagnóstico-recálculo pós-marcos"
cresce N=1→9** (P213 + 8 categorias auditadas P214; 1
categoria com material reclass + 2 sincronizações §2.1
+ 5 inalteradas). **Subpadrão "passo administrativo XS"
cresce N=3→4** (`ADR-0062-create` + P160A + P213 + P214);
ultrapassa limiar formalização N=3-4; promoção a ADR meta
diferida per política P158.

³⁸ — Ajuste P213 (Tabela A.9 Introspection — recálculo
pós-fecho marco M9c 2026-05-12): **5 reclassificações
empíricas** face ao estado factual pós-P212 (M9c ACEITE):
- `counter(key)`: `implementado` → **`implementado⁺`**
  (P210B `counter_step` + P176 `counter_final` + P177
  `counter_at` materializados ao longo M9; deferreds
  `counter.display(numbering)` here-aware + `counter.update`
  documentados com gatilho walk advance per P210A C3).
- `state(key, ...)`: `ausente` → **`implementado`** (P171
  `state` + `state_update` + `state_update_with` registadas;
  `StateRegistry` sub-store + `Introspector::state_value`/
  `state_final_value` métodos trait; `state.get()` here-aware
  deferred per P210A C3 com mesmo gatilho).
- `here()` / `locate()`: `ausente` → **`implementado`**
  (P208B `native_here` lê `EvalContext.current_location`;
  P208C `native_locate` reusa `parse_selector_arg` +
  `Introspector::query.first()`; walk advance automático
  deferred per P208B C2 — gatilho `Content::Context` block
  ou consumer real).
- `query(...)`: `ausente` → **`implementado⁺`** (P175
  minimal `Selector::Kind` + P209A-D estendido para 6
  variants `Kind/Label/Location/And/Or/Regex`; helper
  `parse_selector_arg` em foundations.rs com type
  dispatch; `Selector::And/Or` Rust API only per P209A C3
  Opção c; `Selector::Regex` query stub `vec![]` per
  P209D C5).
- `metadata(value)`: `ausente` → **`implementado`** (P169
  M9 sub-passo 1 — `MetadataStore` sub-store +
  `Introspector::query_metadata()` + stdlib
  `metadata(value)` registada).
- `position(target)`: `ausente` → **`parcial`**
  (`Introspector::position_of(loc) -> Option<Position>`
  materializado P204D + `SealedPositions` sub-store P205B
  + `inject_positions` injection P205C; **stdlib expose
  `position()` standalone ainda ausente** — gatilho Bloco
  B candidato pós-M9c).
Distribuição pós-recálculo: **3/2/1/0/0 = 6 (vs prévio
1/0/0/5/0 antes M9c)**. Cobertura `(impl + impl⁺)/total =
(3+2)/6 = **83%** (vs 17% prévio); +66pp categoria.
**Cobertura user-facing total**: peso categoria
Introspection ~4.3% das ~141 entradas; Δ +66pp × peso
4.3% ≈ +2.8pp; total user-facing ~61% → **~63-64%**.
**Política "sem novas reservas" preservada per P158**:
candidatos Bloco A (`measure` stdlib expose) + Bloco B
(`position()` stdlib + `query_count_before`) identificados
mas NÃO reservados em P213 — decisão humana posterior em
sessão futura. Tests +0 (1939 inalterados — recálculo
documental sem código tocado). 0 violations preservadas.
**Pattern emergente "diagnóstico-recálculo pós-marcos"
N=1** (P213 primeira aplicação pós-fecho marco M9c;
distinto de P154A/P156B/P157/P158/P159 que precederam
materialização). **Subpadrão "passo administrativo XS"
N=3** (`ADR-0062-create` + P160A + P213) atinge limiar
formalização — promoção ADR meta diferida para passo
dedicado se N=4+ ocorrer.

³⁷ — Ajuste P159G (Tabela A.6 Model): **segundo sub-passo
família 159 fora do Bloco A** (Bloco A esgotado pós-P159F).
Refino estrutural de tipo entity `BibEntry` adicionando **6
fields restantes mais comuns hayagriva** (`editor`/`series`/
`note`/`isbn`/`location`/`organization`) — listados em P159D
§9.3 como diferidos por menor universalidade. **Pattern P159D
replicado pela terceira vez** — **subpadrão #16 cresce N=2 →
3** "refino tipo entity sem alteração ao variant Content"
(P159D BibEntry 4 fields + P159E BibEntry 2 fields + **P159G
BibEntry 6 fields**); **patamar atinge limiar formalização
N=3-4**. Builder pattern fluente extendido (6 novos `with_*`
métodos paridade P159D/E). Helper inline `optional_str`
(P159D/E) reusado N=6 P159G — **cumulativo N=4 P159D + N=2
P159E + N=6 P159G = N=12** (largamente acima do limiar
promoção N=3-4; promoção a `pub(super)` ou helper público
diferida em passo administrativo XS NÃO reservado). Layout
`format_bib_entry` extendido com concatenação condicional
APA-like extendida (decisão diagnóstico §8.2 ordem + §9
formatos individuais): editor `(Ed. ...)` após title; series
`(...)` após title; location: antes de publisher; organization
substitutivo a publisher quando publisher ausente; isbn antes
de url/doi com prefixo lowercase `isbn:` (paridade P159E doi);
note ao final entre brackets `[...]`. **Sem alteração ao
variant `Content::Bibliography` ou `Content::Cite`**. **Hash
`entities/content.rs` preservado** `ec58d849` — **décimo
sétimo passo consecutivo** (P156L → P159G via L0-baseline
interpretation). **Hash `entities/bib_entry.rs` preservado**
`5a2c0ebd` (paridade P159D+P159E resultado — extensão via
doc-comment). Tests +11 (1230 → 1241; 4 unit bib_entry P159G +
4 stdlib parse + 3 layout E2E formato extendido/regression/
organization substitutivo; range esperado +8-12). Cobertura
Model agregada **inalterada** (~50%) — refino tipo entity.
Cobertura ampla 77% inalterada. Cobertura arquitectural
**inalterada** 82%. **BibEntry pós-P159G: 16 fields total**
(4 obrigatórios + 12 opcionais; cobertura ~70-75% hayagriva
universais). **Política "sem novas reservas" preservada** —
restantes fields vanilla (`booktitle`/`address`/`chapter`/
`type`/`institution`/etc.), tipos estruturados (`Vec<Person>`
editor, location codes, ISBN validation), CSL real (depende
hayagriva ADR-0062), promoção `optional_str` a helper público
permanecem candidatos NÃO-reservados.

³⁶ — Ajuste P159E (Tabela A.6 Model): **primeiro sub-passo
família 159 fora do Bloco A** (esgotado pós-P159F). Refino
estrutural de tipo entity `BibEntry` adicionando 2 fields
opcionais identificadores digitais (`url`, `doi`) — par natural
identificado em P159D §9 como candidato a sub-passo M futuro.
**Pattern P159D replicado fielmente** — **subpadrão #16 cresce
N=1 → 2** "refino de tipo entity sem alteração ao variant
Content" (P159D BibEntry 4 fields + **P159E BibEntry 2 fields**).
Builder pattern fluente extendido (`with_url`/`with_doi`).
Helper inline `optional_str` (P159D) reusado N=2 P159D + N=2
P159E = **N=4 cumulativos** (atinge limiar promoção N=3-4;
reavaliação em passo administrativo XS futuro NÃO reservado).
Layout `format_bib_entry` extendido com concatenação condicional
APA-like (Opção C diagnóstico §8.2): url/doi após `(year).` —
backwards compat preserva output P159D quando ambos `None`.
Formato: URL plaintext literal `https://...`; DOI prefixo `doi:`
(decisão diagnóstico §9). **Sem alteração ao variant
`Content::Bibliography` ou `Content::Cite`**. **Hash
`entities/content.rs` preservado** `ec58d849` — **décimo sexto
passo consecutivo** (P156L → P159E via L0-baseline interpretation).
**Hash `entities/bib_entry.rs` preservado** `5a2c0ebd` (paridade
P159D resultado — extensão via doc-comment não modifica prompt
L0). Tests +8 (1222 → 1230; 3 unit bib_entry url/doi + 3 stdlib
parse + 2 layout E2E formato extendido/regression; range
esperado +5-8). Cobertura Model agregada **inalterada** (~50%)
— refino tipo entity. Cobertura ampla 77% inalterada. Cobertura
arquitectural **inalterada** 82%. **Política "sem novas
reservas" preservada** — restantes fields vanilla
(`editor`/`series`/`note`/`isbn`/`location`/`organization`),
tipos estruturados (`QualifiedUrl`/`Doi`), URL/DOI validation,
hyperlinks (Bloco C) permanecem candidatos NÃO-reservados.

³⁵ — Ajuste P159F (Tabela A.6 Model): quarto sub-passo
substantivo Bibliography + Cite — **último candidato Bloco A
do diagnóstico P159B**. Refino comportamental: counter local
de bib entries + render numerado em Cite Normal/None. Field
novo `pub bib_numbers: HashMap<String, u32>` em `CounterState`
(paridade aditiva infraestrutura state lookup — **subpadrão #15
cresce N=2 → 3** via `state.lang` P158B + `state.bib_entries`
P159C + **`state.bib_numbers` P159F**). Walk arm Bibliography
popula contínuamente (multi-Bibliography preserva primeiro
número via `or_insert`). Layout arm Cite Normal/None faz
lookup `state.bib_numbers.get(key)` → `[N]` ou fallback `[key]`
(regression P159A). Forms diferenciadas (Prose/Author/Year)
inalteradas — numeração só em Normal/None (decisão diagnóstico
§10). **Decisão arquitectural-chave Opção C** (Cite.form
interaction; sem field user-facing) escolhida vs Opção A
(substituir sempre; rejeitada por quebrar tests P159A/C) e
Opção B (Bibliography.style field novo; rejeitada por alteração
estrutural sem ganho proporcional). **Multi-Bibliography
contínua** (paridade vanilla numeric style; decisão diagnóstico
§9). **Sem alteração ao variant `Content::Cite` ou
`Content::Bibliography`**. **Hash `entities/content.rs`
preservado** `ec58d849` — **décimo quinto passo consecutivo**
(P156L → P159F via L0-baseline interpretation). Tests +8
(1214 → 1222; 2 unit counter_state + 6 layout E2E numbering;
range esperado +10-15 ligeiramente abaixo por helper inline
trivial). Cobertura Model agregada **inalterada** (~50%) —
refino comportamental. Cobertura ampla 77% inalterada.
Cobertura arquitectural **inalterada** 82%. **Marca conceptual**:
P159F **esgota Bloco A** do diagnóstico P159B (último candidato).
Pós-P159F, Model puro está saturado per recomendação do
diagnóstico (~55-60% estimado com 24 entradas parciais).
**Política "sem novas reservas" preservada** — outras styles
(alphanumeric, author-date, CSL), `Bibliography.style` field
user-facing (Opção B refino futuro), numeração independente
multi-Bibliography permanecem candidatos NÃO-reservados.

³⁴ — Ajuste P158C (Tabela A.6 Model): quarto sub-passo Model
figure-kinds — refactor cosmético `kind: String → Option<String>`
em `Content::Figure` per **ADR-0064 Caso A estrito** (vanilla
`Smart<Str>` → cristalino `Option<String>`; None ↔ Auto; default
`"image"` resolvido em uso por callers, não em construção).
**Patamar Caso A cresce N=6 → 7** com **primeiro Caso A "estrito"
em refactor** (não em variant aditivo). Distribuição cross-domínio
desloca-se: 3 Layout (P156G/H/I) + 4 Model (P157B/P159A/C +
**P158C**) — passa de 50/50 para 43/57 favorecendo Model.
**Subpadrão emergente N=1 NOVO** "refactor de field para Option"
(precedente novo — distinto de variant aditivo com Option<T>
field; aplicação em refactor de tipo existente). ~10 sítios
callers adaptados (stdlib `native_figure` retorna `Option<String>`
directamente; introspect/layout fazem `kind.as_deref().unwrap_or("image")`
em uso). **Sem alteração observable** (output preservado;
backwards compat trivial). **Hash `entities/content.rs` preservado**
`ec58d849` — **décimo quarto passo consecutivo** (P156L → P158C
via L0-baseline interpretation; lição P159A/C/D internalizada —
preservação é regra default para refactors internos cosméticos).
Tests +2 (1212 → 1214; 1 novo `figure_kind_auto_explicito_devolve_none` +
1 novo `introspect_figure_kind_none_resolve_para_image_no_counter`;
range esperado +2-4). ~5 tests existentes adaptados para
`kind.as_deref() == Some(...)` em vez de `kind == "..."`. Cobertura
Model agregada **inalterada** (~50%) — refactor cosmético sem
mover counts. Cobertura ampla 77% inalterada. Cobertura
arquitectural **inalterada** 82%. **Política "sem novas reservas"
preservada** — refactor análogo de outros String fields, helper
público `kind_or_default`, documentação completa de variants no
L0 prompt content.md permanecem candidatos NÃO-reservados.

³³ — Ajuste P159D (Tabela A.6 Model): terceiro sub-passo
substantivo Bibliography + Cite — refino estrutural de tipo
entity `BibEntry` adicionando 4 fields opcionais universais
(`volume`/`pages`/`journal`/`publisher`) per ADR-0065 critério
#2 (terceira aplicação isolada concreta — selecção de fields
universais). **Builder pattern** (`with_volume`/`with_pages`/
etc.) escolhido per Opção C diagnóstico §8 (legibilidade +
backwards compat trivial). Helper `extract_bib_entries` extendido
para parsing dos 4 fields opcionais com validação tipo
`Value::Str`. Helper privado novo `format_bib_entry` em
`layout/mod.rs` para concatenação condicional APA-like
(`[key] author. title journal vol. volume, pp. pages. publisher
(year).`). **Sem alteração ao variant `Content::Bibliography`**
(estrutura inalterada; expansão de tipo entity ortogonal ao
enum Content). **Sem alteração ao variant `Content::Cite`**
(P159C inalterado). **Hash `entities/content.rs` preservado**
`ec58d849` — **décimo terceiro passo consecutivo** (P156L →
P159D via L0-baseline interpretation). **Hash `entities/bib_entry.rs`
também preservado** `5a2c0ebd` (L0-baseline interpretation —
prompt `bib_entry.md` não modificado; spec previa quebra mas
extensão via doc-comment). Tests +8 (1204 → 1212; 3 unit
bib_entry + 3 stdlib parse + 2 layout E2E; range esperado +5-8).
Cobertura Model agregada **inalterada** (~50%) — refino
qualitativo. Cobertura ampla 77% inalterada. Cobertura
arquitectural **inalterada** 82%. **Subpadrão emergente N=1**
"refino de tipo entity sem alteração ao variant Content"
(precedente novo — P156L/P159C refinaram variants Content;
P159D primeiro a refinar tipo entity puro). **Política "sem
novas reservas" preservada** — fields restantes vanilla
(url/doi/editor/series/note/isbn/location/etc.), tipos
estruturados (PageRange), CSL real, estilo configurável
permanecem candidatos NÃO-reservados.

³² — Ajuste P159C (Tabela A.6 Model): segundo sub-passo
substantivo Bibliography + Cite — refino estrutural-comportamental
de `cite` adicionando enum `CitationForm { Normal, Prose, Author,
Year }` em `entities/citation_form.rs` (5ª aplicação consecutiva
do padrão "tipo entity em ficheiro próprio") + field
`form: Option<CitationForm>` em `Content::Cite` per ADR-0064
Caso A (patamar Caso A cresce **N=5 → 6**; equilíbrio
cross-domínio 50% Layout + 50% Model atingido). 13 sítios
pattern-match Content actualizados. Helper privado novo
`extract_citation_form` em stdlib/structural.rs (strict matching
case-sensitive; 4 forms válidos). Layout placeholder melhorado
por form com lookup Bibliography via novo field
`pub bib_entries: Vec<BibEntry>` em `CounterState` (paridade
infraestrutural P158B `state.lang`); populado por introspect
walk; multi-Bibliography concatena. Fallback `[key]` quando key
não encontrada (paridade Normal sem entry). **Hash
`entities/content.rs` preservado** `ec58d849` — **décimo
segundo passo consecutivo** (P156L → P159C; L0-baseline
interpretation: prompt `content.md` não modificado; refino
arquitectural via doc-comments e referência cruzada citation_form.md).
Tests +15 (1189 → 1204; 8 unit citation_form + 2 cite com form +
6 stdlib parse + 4 layout E2E forms incluindo lookup
Bibliography; range esperado +12-17). Cobertura Model agregada
**inalterada** (~50%) — refino qualitativo. Cobertura ampla 77%
inalterada. Cobertura arquitectural **inalterada** 82%.
**Política "sem novas reservas" preservada** (P158/A/B; P159/A/B
respeitam) — forms vanilla adicionais (Full, CSL-specific),
CSL render real, `style: Str` per-Cite, cross-document refs
permanecem candidatos NÃO-reservados.

³¹ — Ajuste P158B (Tabela A.6 Model): segundo refino qualitativo
consecutivo de `figure` — supplement automático localizado por
lang adicionado em `introspect.rs` linha 334. Helper novo
`figure_supplement_for_lang(kind, lang) -> String` em
`rules/lang/figure_supplement.rs` cobrindo 6 langs (pt/en/de/
fr/es/it) × 3 kinds (image/table/raw) = 18 entradas + fallback
PT por kind + capitalização para kind desconhecido. Field novo
`pub lang: Option<Lang>` em `CounterState` para lang resolution
(default `None` → fallback PT, paridade backwards compat).
**Reuso explícito do padrão P155** `localize_quotes(lang)` —
primeiro reuso cross-feature (quotes → figure supplement);
**subpadrão emergente N=1** "padrão P155 i18n reusado
cross-feature". **Sem alteração ao variant `Content::Figure`**
(estrutura inalterada). **Hash `entities/content.rs` preservado**
`ec58d849` — **décimo primeiro passo consecutivo** (P156L → P158B)
sem alteração ao variant Content. ADR-0064 NÃO directamente
aplicável (kind continua String; lang é Option mas em
CounterState, não em variant). Cobertura Model agregada
**inalterada** (~50%) — refino qualitativo. Tests +15 (1174
→ 1189; 8 unit em figure_supplement.rs + 7 integration em
introspect.rs). **Política "sem novas reservas"** preservada
(P158 estabeleceu; P158A/B respeitam) — `supplement: Option<Content>`
field user-facing, mais langs, CSL-aware format permanecem
candidatos NÃO-reservados.

²¹ — Ajuste P156L (refino sides individualizadas; **primeira
aplicação concreta de ADR-0065 critério #3** — expansão de
variant existente; **segunda aplicação concreta de ADR-0064
Caso C**). `pad` transita `implementado → implementado⁺`
(refino de variant existente; cobertura quantitativa
inalterada). Variant `Pad { body, padding: Sides<Length> }`
refactorado para `Pad { body, sides: Sides<Option<Length>> }`
— cada side `None ↔ default zero` resolvido em momento de uso
no Layouter. **Divergência da spec do passo P156L §"Verificação"
#5** detectada e documentada em diagnóstico §6.1: spec assumia
`pad` como `parcial` (factualmente `implementado` desde P156C);
cobertura **não passa para 84%** como spec previa. Em vez disso,
pad ganha sufixo ⁺ indicando refino além do mínimo (consistente
com `figure` em ADR-0041). Helper `extract_sides_lengths`
privado adicionado a `stdlib/layout.rs`. `extract_length`
reusado **N=7** vezes consecutivas (P156C/D/G/H/I/J/L). Tipo
`Sides<T>` reusado segunda vez concreta (P156C origin; P156L
nova materialização). Contagem Layout: 14/0/3/1/0=18 →
**13/1/3/1/0=18** (uma entrada move-se de implementado puro
para implementado⁺). Cobertura `(impl + impl⁺)`: 14/18 =
**78%** (inalterada quantitativamente). Total user-facing:
64/21/22/32/2=141 → **63/22/22/32/2=141** (uma entrada move-se
de implementado para implementado⁺). Tabela B Content: 52
(inalterada — refino, não adição). ADR-0061 mantém-se
`PROPOSTO`. **Padrões consolidados**: ADR-0064 N=7 implícito;
ADR-0065 N=6 implícito; reuso `Sides<T>` N=2; reuso
`extract_length` N=7.

**Cobertura arquitectural total**: (74 + 13) / 106 = **82%**
(era 80% pós-P157C/P158/P158A; era 78% pós-P157B; era 77-78%
pós-P157A; era 76-77% pós-P156L; era 75-76% pós-P156I; era
75% pré-P155; era 72% pré-P154B; era 70% pré-P149; **inalterada
pós-P158B + P159C + P159D + P158C + P159F + P159E + P159G** —
refinos qualitativos/refactors cosméticos/numbering numérico/par
identificadores digitais/6 fields restantes hayagriva de variants
Content existentes ou tipos entity ortogonais; **Bloco A
diagnóstico P159B esgotado**; **P159E e P159G sub-passos família
159 fora Bloco A**; BibEntry pós-P159G com 16 fields). **Patamar 82% atingido em P159A** — par acoplado
Bibliography+Cite minimal adiciona 2 variants Content (56 → 58);
vanilla extra ausentes mantém 0 (subset minimal cobre todos os
variants core para Bibliography+Cite).
**Nota**: variants extra cristalino (`Value::Align`, `Content::Styled`) são **divergências intencionais**
favoráveis — cristalino tem features que vanilla não (em forma de Value); contadas como `implementado` porque
encerradas por ADRs (0026, 0028→0029, 0036, etc.).

⁶⁵ — Ajuste P247 (M9d / M7+5; ADR-0079 Categoria A.4 cumulativa)
— **3 scope-outs cosméticos visuais Block + Boxed promovidos
cumulativamente** (outset semantic real + fill + stroke);
**fecha 5 dos 9 scope-outs Block originais P156G** (outset
P231→P247 + radius P242 + clip P242 + fill P247 + stroke P247);
**fecha 5 dos 6 scope-outs Boxed P156H** (resta stroke-overhang);
inaugura sub-padrão **"agregar promoções scope-outs cosméticos
visuais"** N=1; lição refinada N=9 → 10 cumulativo P247
("mapear scope-outs declarados historicamente vs estado real
materializado antes de assumir ausência").

**P247 materializa A.4 cumulativa**:

- **+2 fields em Block + Boxed**: `fill: Option<Color>` + `stroke:
  Option<Stroke>`; paridade simétrica; 8 → 10 fields ambos
  variants.
- **outset semantic real activado** (cenário A audit C1 §2.4-§2.5
  — outset zero-uso pré-P247): cursor.y avança outset.top antes
  do inset.top; outset.bottom após height min; bounds Shape
  expandem em todos os lados.
- **Layouter activa emissão `FrameItem::Shape`** ANTES do body
  via `current_items.insert(items_before, ...)` (Z-order); reuso
  `ShapeKind::RoundedRect { radii: radius }` P242 quando radius
  non-zero.
- **stdlib `block(fill:, stroke:)` + `box(fill:, stroke:)`**: fill
  aceita `Value::Color` directo (pattern inline 1-linha paridade
  Grid/Table P228); stroke reusa helper `extract_stroke` P227.

**Reclassificações §A.5 P247** (2 reclassificações):

- `block(...)`: `implementado` ¹³ → **`implementado⁺` ¹³ ⁶⁵** (P247
  fill+stroke+outset semantic real activados cumulativamente).
- `box(...)`: `implementado` ¹⁵ → **`implementado⁺` ¹⁵ ⁶⁵** (P247
  análogo; paridade simétrica).

**Recontagem Layout per metodologia pós-P247**: `~93-94% →
~94-95%` (+1pp refino qualitativo; reclassificações `implementado`
→ `implementado⁺` em 2 funcs Layout cumulativas P247).
**Cobertura user-facing total preservada**: ~75-76%.

**Distribuição §A.5 Layout preservada literal**: contagens
preservadas (reclassificações `implementado` → `implementado⁺`
são qualitativas; não migram entre categorias `parcial`/`ausente`).

**Stdlib funcs**: 64 preservado. **ShapeKind variants**: 5
preservado. **Regions fields**: 4 preservado. **Layouter
fields**: preservado.

**Block fields**: 8 → **10** (+fill, +stroke). **Boxed fields**:
8 → **10** (+fill, +stroke). **Content variants**: 62 preservado
(refino aditivo a variants existentes, sem novos).

**Scope-outs promovidos cumulativos**: 3 (Pad.right P243 +
Block.width P243 + Boxed.width P243) + 2 (Block/Boxed radius
P242 + clip P242) + **3 (outset semantic + fill + stroke P247)**
= **8 promoções reais cumulativas**. Sub-padrão "agregar
promoções scope-outs cosméticos visuais" N=1 inaugurado P247.

**Tests P247** (20 unit/E2E):
- 6 unit content (`entities/content.rs`): variant aceita
  fill+stroke; partialEq inclui; map_content preserva;
  construtores defaults None.
- 8 unit stdlib (`stdlib/mod.rs`): native_block + native_box
  aceitam fill Color + stroke shorthand; defaults None; tipos
  errados rejeitados; combina fill+stroke+radius+clip+outset.
- 6 E2E layout (`layout/tests.rs`): Block fill emite Shape;
  Block stroke emite Shape; fill + radius emite RoundedRect;
  outset expande bounds; backward compat
  (fill=stroke=None+outset=ZERO sem Shape); Boxed fill emite
  Shape.

**N=12 adaptações** em tests pré-existentes (2 acima range
N=0-10 §1.4 estimado; documentadas):
- 4 sítios `entities/content.rs` (P231 testes).
- 7 sítios `layout/tests.rs` (P231/P242/P243).
- 1 sítio `introspect.rs` (materialize_time arms).

**Workspace pós-P247**: **2209 → 2229 verdes** (+20 P247; 0
regressões; N=12 adaptações).

⁶⁶ — Ajuste P248 (M9d / M7+5; ADR-0079 Categoria A.4 cumulativa)
— **3 promoções graded → real semantic activação consumer**
agregadas via mecanismo comum medição antecipada: Block.breakable
+ Boxed.height overflow + TableCell overflow clip implícito;
inaugura sub-padrão **"Activação semantic real multi-consumer
via mecanismo comum"** N=1; promoção graded → real N=1 → N=2
cumulativo (P245 N=1; P248 N=2 agregado); lição refinada N=10
→ 11 cumulativo P248 ("mapear pontos de check overflow
existentes antes de adicionar novos checks duplicados"); 11ª
aplicação cumulativa pattern "spec C1 audit obrigatório
bloqueante pós-P236.div-1".

**P248 materializa A.4 cumulativa via 3 activações graded → real**:

- **Activação A — Block.breakable** (Layouter `mod.rs` Block
  arm): medição antecipada via `measure_content_constrained`
  puro (audit C1 §2.4); `new_page()` antecipado se bloco não-
  breakable não cabe na actual mas cabe noutra; overlong emit
  normal (paridade vanilla "overlong atómico não quebra").
- **Activação B — Boxed.height** overflow: `clip: true` wrap
  body em FrameItem::Group com clip_mask Rect altura h (reuso
  mecanismo P242); `clip: false` overflow visível paridade
  vanilla default. `height: None` preservado P156H literal.
- **Activação C — TableCell.body** overflow clip implícito:
  detecção via `regions.cell.height` (P246) + `cell_h_measured`
  retornado por `layout_sub_frame_with_width`; clip via Group
  + clip_mask Rect quando overflow. Row break real diferido
  (refino futuro; DEBT-34e preservado aberto, distinto: DEBT-34e
  cobre colspan/rowspan placement).

**Reclassificações §A.5 P248** (2 reclassificações):

- `block(...)`: `implementado⁺` ¹³ ⁶⁵ → **`implementado⁺` ¹³ ⁶⁵ ⁶⁶**
  (P248 breakable real activado cumulativo).
- `box(...)`: `implementado⁺` ¹⁵ ⁶⁵ → **`implementado⁺` ¹⁵ ⁶⁵ ⁶⁶**
  (P248 height overflow semantic real activado).

**Recontagem Layout per metodologia pós-P248**: `~94-95% →
~95-96%` (+1pp refino qualitativo).
**Cobertura user-facing total preservada**: ~75-76%.

**Distribuição §A.5 Layout preservada literal**: contagens
preservadas (reclassificações qualitativas não migram entre
categorias).

**Stdlib funcs**: 64 preservado. **ShapeKind variants**: 5
preservado. **Regions fields**: 4 preservado. **Layouter
fields**: preservado. **Content variants**: 62 preservado
(refino consumer puro; sem novos).

**Block fields**: 10 preservado (P247 final). **Boxed fields**:
10 preservado. **TableCell fields**: 5 preservado (P157B final).

**Scope-outs promovidos cumulativos**: 3 (P243 Pad.right +
Block.width + Boxed.width) + 2 (P242 radius + clip) + 3 (P247
outset + fill + stroke) + **3 (P248 breakable + height + cell
overflow)** = **11 promoções reais cumulativas**.
**Sub-padrão "Activação semantic real multi-consumer via
mecanismo comum" N=1 inaugurado P248**.

**Promoção graded → real semantic** N=1 → **N=2 cumulativo P248**
(P245 Place float = N=1; P248 agregado = N=2; granular = N=4
contando 3 sub-activações P248 + 1 P245).

**Tests P248** (26 unit/E2E):
- 16 unit Block.breakable + Boxed.height + cell overflow + E2E
  cross-activação em `layout/tests.rs`.
- 4 unit stdlib `native_block`/`native_box` propagação
  breakable/height/clip.
- 6 unit cross-attribute (breakable + outset; height + radius;
  cell overflow + radius+clip; etc).

**N=0 adaptações** em tests pré-existentes — defaults preservados
literais (`breakable: true` + `height: None` + cell sem overflow
renderizam idênticos P247; backward compat estrita).

**Workspace pós-P248**: **2229 → 2255 verdes** (+26 P248; 0
regressões; **0 adaptações**).

⁶⁷ — Ajuste P250 (M9d / M7+5; ADR-0079 Categoria A.4 Block
**COMPLETO 10/10**; primeira aplicação citante ADR-0082
PROPOSTO N=1) — **4 promoções graded → real semantic** agregadas
em Block: spacing + above + below + sticky; **fecha Block A.4
COMPLETO 10/10 scope-outs originais P156G** (incluindo breakable
contado como elemento original); **Boxed preservado 5/6** (resta
stroke-overhang; P250 não toca Boxed por assimetria intencional
— estes scope-outs são exclusivos Block); inaugura sub-padrão
**"Refactor Sequence consumer cross-arm via peekable + neighbour
context"** N=1; lição refinada N=12 → 13 cumulativo P250
("refactor cross-arm Sequence consumer exige audit de todos os
patterns de iteração existentes antes de migrar a peekable").

**P250 materializa A.4 Block COMPLETO**:

- **+4 fields em Block**: `spacing: Option<Length>` + `above:
  Option<Length>` + `below: Option<Length>` + `sticky: bool`;
  defaults None×3 + false. **10 → 14 fields**.
- **Boxed preservado 10 fields** (assimetria intencional;
  scope-outs exclusivos Block; pattern "refino aditivo paralelo
  entre variants irmãos" N=5 P247 **não aplica P250**).
- **Spacing collapse semantic** (paridade vanilla CSS margin
  collapse `max(prev.below, curr.above)`): `above`/`below`
  fallback `spacing`; above suprimido no primeiro Block dum
  Sequence (`block_chain_active == false`); non-Block intermediário
  quebra chain.
- **Sticky lookahead 1-block** via peekable Sequence consumer:
  `new_page()` antecipado se `combined_h > remaining + cabe em
  página inteira`.
- **Refactor Sequence consumer cross-arm**: layout consumer
  (mod.rs:478) migrado para `iter.peek()` + neighbour context;
  measure consumer (mod.rs:1850+) preservado simples (spacing
  colapsa para zero em medição estática per ADR-0054 graded).
- **+2 Layouter fields**: `prev_block_below_pending: f64` +
  `block_chain_active: bool` (state save/restore entre Sequences).

**Reclassificação §A.5 P250** (1 reclassificação):

- `block(...)`: `implementado⁺` ¹³ ⁶⁵ ⁶⁶ → **`implementado⁺` ¹³
  ⁶⁵ ⁶⁶ ⁶⁷** (P250 spacing+above+below+sticky semantic real
  activados; Block A.4 COMPLETO 10/10).

**Reclassificação §A.5 `box(...)` preservada** (P250 não toca
Boxed; asymetria intencional).

**Recontagem Layout per metodologia pós-P250**: `~95-96% →
~96-97%` (+1pp refino qualitativo).
**Cobertura user-facing total preservada**: ~75-76%.

**Distribuição §A.5 Layout preservada literal**: contagens
preservadas (reclassificação qualitativa).

**Stdlib funcs**: 64 preservado. **ShapeKind variants**: 5
preservado. **Regions fields**: 4 preservado. **Layouter
fields**: **+2** (`prev_block_below_pending` + `block_chain_
active`). **Content variants**: 62 preservado.

**Block fields**: 10 → **14** (+spacing, +above, +below, +sticky).
**Boxed fields**: 10 preservado. **TableCell fields**: 5 preservado.

**Scope-outs promovidos cumulativos**: 3 (P243 multi-region) + 2
(P242 radius+clip) + 3 (P247 outset+fill+stroke) + 3 (P248
breakable+height+cell_overflow) + **4 (P250 spacing+above+below+
sticky)** = **15 promoções reais cumulativas**.
**Sub-padrão "Refactor Sequence consumer cross-arm" N=1 inaugurado
P250**. **Sub-padrão "Aplicação citante ADR-0082 PROPOSTO" N=0 →
N=1 P250** (primeira aplicação concreta a citar ADR-0082
explicitamente; promoção EM VIGOR pendente N=3 citantes).

**Tests P250** (21 unit/E2E):
- 14 unit Block spacing/above/below + sticky + Sequence refactor
  + A.4 completude em `layout/tests.rs`.
- 5 unit stdlib `native_block` 4 args novos + defaults + tipos
  errados rejeitados.
- 2 unit Sequence aninhado + chain quebrada por non-Block.

**N=21 adaptações** em tests pré-existentes (dentro do range
N=5-15 estimado §1.4 + 6 adicionais — 31 sítios `stroke: None,`
em entities/content.rs+layout/tests.rs cascade replace_all + 3
sítios deeper indent; introspect.rs materialize_time arm
adaptado).

**Workspace pós-P250**: **2255 → 2276 verdes** (+21 P250; 0
regressões).

**Marco P250**: **Block A.4 COMPLETO 10/10**. Primeiro variant
Content com **100% dos scope-outs originais fechados**
cumulativamente. Pattern empírico "Promoção real scope-out
ADR-0054 graded" granular N=8 → N=12 cumulativo P250
(P250 ×4: spacing + above + below + sticky).

⁶⁸ — Ajuste P251 (M9d / M7+5; ADR-0079 Categoria C.2 parcial
activada cell-level; **segunda aplicação citante ADR-0082
PROPOSTO N=2**) — promoção scope-out TableCell.body overflow de
"clip implícito P248" para "row break vertical real cell-level
γ-Items"; inaugura sub-padrão **"Slice frame items at height
via filter + rebase pos.y"** N=1; consolida sub-padrão
**"DeferredX buffer + flush em new_page"** N=1 → N=2 cumulativo
(P245 floats + P251 cell tails); lição refinada N=13 → 14
cumulativo P251 ("audit C1 deve confirmar localidade pos.y
antes de fixar abordagem γ-Items vs γ-Content para slicing").

**P251 materializa Categoria C.2 parcial cell-level**:

- **Novo módulo** `01_core/src/rules/layout/slicing.rs` (~270 LoC)
  com função pura `slice_frame_items_at_height(items, threshold)
  -> (head, tail)` + helper `rebase_item_y(item, delta)`
  exhaustive sobre 6 variants `FrameItem`
  (Text/Line/Glyph/Image/Shape/Group).
- **Layouter +1 field** `pending_cell_tails: Vec<DeferredCellTail>`
  (paridade arquitectural P245 `floats_pending`).
- **+1 struct local** `DeferredCellTail` (items + origin_x +
  width + fill + stroke + forwarded_count).
- **+1 método** `flush_pending_cell_tails()` chamado no fim de
  `new_page()` (Z-order paridade P248: fill atrás → items
  rebased → stroke à frente).
- **Refactor `grid.rs:393-433`** cell overflow: rows
  `TrackSizing::Fixed` preservam P248 clip implícito (paridade
  vanilla "Fixed rows clip"); rows Auto/Fraction usam P251
  slice + tail push.
- **Limit 3 iterações** de tail forwarding (mitigação loop
  infinito; paridade vanilla heurística).

**Limitações conscientes γ-Items (per ADR-0054 graded)**:

- Items atómicos (Group/Shape) não dividem mid-item (paridade
  vanilla).
- Fill/stroke re-emit per fragment (visualmente "dois
  rectângulos separados").
- Outras cells da row original **não continuam** na nova página
  (row-level imperfeito; só cell que overflow continua).

**Reclassificação §A.5 P251** (1 reclassificação):

- `table_cell(...)`: `parcial` ²⁴ → **`parcial⁺` ²⁴ ⁶⁸** (P251
  row break real cell-level activado cumulativo; activa
  Categoria C.2 parcial).

**Recontagem Layout per metodologia pós-P251**: `~96-97% →
~97-98%` (+1pp refino qualitativo).
**Cobertura user-facing total preservada**: ~75-76%.

**Distribuição §A.5 Layout preservada literal**: contagens
preservadas (reclassificação qualitativa).

**Stdlib funcs**: 64 preservado. **ShapeKind variants**: 5
preservado. **Regions fields**: 4 preservado. **Layouter
fields**: **+1** (`pending_cell_tails`). **Layouter methods**:
**+1** (`flush_pending_cell_tails`). **Layouter struct local**:
**+1** (`DeferredCellTail`). **Layouter modules**: **+1**
(`layout/slicing.rs`). **Content variants**: 62 preservado.

**Block / Boxed / TableCell fields**: preservados (P251 é
refino consumer Layouter sem alterar entities).

**Scope-outs promovidos cumulativos**: 15 (pós-P250) + **1
(P251 TableCell row break)** = **16 promoções reais cumulativas**.
**Sub-padrão "Slice frame items at height" N=1 inaugurado P251**.
**Sub-padrão "DeferredX buffer + flush em new_page" N=1 → N=2
cumulativo P251**. **Sub-padrão "Aplicação citante ADR-0082
PROPOSTO" N=1 → N=2 cumulativo P251** (promoção EM VIGOR pendente
N=3 citantes).

**Tests P251** (18 unit/E2E):
- 10 unit slice + rebase em `layout/slicing.rs` (slice vazio,
  slice todos head, slice todos tail rebased, slice mistos,
  atomic Shape grande, threshold zero, rebase 4 variants
  Text/Line/Shape/Group).
- 8 unit/E2E em `layout/tests.rs` (row Fixed preserva P248;
  row Auto P251 slice; cell sem overflow preserva P248;
  pending_cell_tails inicial vazio; tail flushed em new_page
  via pagebreak; cell overflow com fill re-emit; flush vazio
  no-op; 2 rows independentes).

**N=0 adaptações** em tests pré-existentes — sentinelas P248
usam `TrackSizing::Fixed` rows que preservam clip implícito
paridade vanilla; backward compat literal estrita.

**Workspace pós-P251**: **2276 → 2294 verdes** (+18 P251; 0
regressões; **0 adaptações**).

**Marco P251**: Categoria C.2 Fase 5 Layout **activada parcial
cell-level** (multi-region completo via column flow DEBT-56
continua diferido). Padrão "Slice frame items at height" N=1
inaugurado primeiro uso γ-Items no Layouter.

⁶⁹ — Ajuste P252 (M9d / M7+5; ADR-0079 Categoria A.4 Boxed
COMPLETO 6/6; **terceira aplicação citante ADR-0082 PROPOSTO
N=3 — limiar interno atingido**) — refactor cross-cutting
entity primitivo `Stroke` (+1 field `overhang: bool`); cascade
~42 construtores literais; activação Layouter Block + Boxed
Shape emit (bounds expandidos por thickness/2 quando
overhang=true); inaugura sub-padrão **"Refactor cross-cutting
entity primitivo com cascade replace_all guiado"** N=1;
consolida sub-padrão **"Backward compat literal estrita"** N=1
→ N=2 cumulativo (P251 cell tails + P252 stroke overhang);
lição refinada N=14 → 15 cumulativo P252 ("refactor cross-
cutting de entity primitivo exige mapa empírico exhaustive de
todos os construtores literais antes de modificar struct").

**P252 materializa Boxed A.4 COMPLETO**:

- **`Stroke` struct +1 field** `overhang: bool` (paridade vanilla
  literal).
- **Cascade ~42 construtores literais** em entities/geometry +
  entities/content + rules/layout + rules/stdlib via sed pattern
  `Stroke { paint, thickness: <num> } → Stroke { paint,
  thickness: <num>, overhang: false }`.
- **Helper `extract_stroke` expandido**: defaults vanilla
  `overhang: true` para Length/Color atalhos; opcional
  explícito via Dict (não suportado actualmente — fallback true
  via stdlib parse).
- **stdlib `native_stroke`** aceita `overhang` named arg (Bool;
  default `true` vanilla).
- **Layouter Block + Boxed Shape emit**: bounds expandidos por
  `thickness/2` em cada lado quando `stroke.overhang == true`
  (default Rust `false` preserva bounds literais; backward
  compat estrita).
- **Grid/Table cell borders preservados** (`FrameItem::Shape::Line`;
  overhang n/a conceptual — line cap distinct). Divergência
  consciente per ADR-0054 graded.

**Limitações conscientes P252**:

- Construtor Rust low-level `overhang: false` divergente vanilla
  `true` — paridade restaurada via stdlib parse.
- PDF exporter intocado (bounds finais single source of truth).
- Round corners (RoundedRect P242) + overhang: bounds expandidos
  com radius preservado (paridade vanilla graded).
- Grid/Table cell borders preservados literal (Line strokes;
  overhang n/a).

**Reclassificação §A.5 P252** (1 reclassificação):

- `box(...)`: `implementado⁺` ¹⁵ ⁶⁵ ⁶⁶ → **`implementado⁺` ¹⁵
  ⁶⁵ ⁶⁶ ⁶⁹** (P252 stroke-overhang activado cumulativo;
  **Boxed A.4 COMPLETO 6/6**).

**Recontagem Layout per metodologia pós-P252**: `~97-98% →
~98-99%` (+1pp refino qualitativo).
**Cobertura user-facing total preservada**: ~75-76%.

**Distribuição §A.5 Layout preservada literal**: contagens
preservadas.

**Stdlib funcs**: 64 preservado. **ShapeKind variants**: 5
preservado. **Regions fields**: 4 preservado. **Layouter
fields**: preservado. **Content variants**: 62 preservado.
**`Stroke` fields: 2 → 3** (+overhang).

**Block / Boxed / TableCell fields**: preservados (P252 é
refactor entity primitivo + activação consumer; sem alterar
variants Content).

**Scope-outs promovidos cumulativos**: 16 (pós-P251) + **1
(P252 Boxed.stroke-overhang)** = **17 promoções reais
cumulativas**.

**Sub-padrão "Refactor cross-cutting entity primitivo com
cascade replace_all guiado" N=1 inaugurado P252**. **Sub-padrão
"Aplicação citante ADR-0082 PROPOSTO" N=2 → N=3 cumulativo
P252** — **limiar interno N=3 atingido** (promoção EM VIGOR
humana possível). **Sub-padrão "Backward compat literal estrita"
N=1 → N=2 cumulativo P252** (P251 cell tails + P252 stroke
overhang).

**Tests P252** (10 unit/E2E):
- 4 unit Stroke struct + Layouter bounds em `layout/tests.rs`
  (PartialEq inclui overhang; Clone preserva; overhang=false
  preserva bounds; overhang=true expande por thickness;
  Boxed paralelo Block).
- 5 unit stdlib em `stdlib/mod.rs` (Length atalho default
  vanilla true; Color atalho default true; native_stroke
  overhang=false explícito; native_stroke default true; sticky
  não-Bool rejeitado).
- 1 sentinela Boxed E2E.

**N=33 adaptações** em construtores literais via cascade
replace_all guiado (sed pattern + ~4 fixes manuais formatting/
shorthand). Dentro do range N=30-40 estimado §1.5.

**Workspace pós-P252**: **2294 → 2304 verdes** (+10 P252; 0
regressões; **N=33 adaptações** documentadas cascade).

**Marco P252**: **Boxed A.4 COMPLETO 6/6** — segundo variant
Content com 100% scope-outs originais fechados cumulativamente
(após Block P250 10/10). Pattern "Refactor cross-cutting entity
primitivo" N=1 inaugurado primeiro uso entity primitivo
cross-cutting. **ADR-0082 N=3 citantes limiar atingido**
(promoção EM VIGOR humana possível).

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
