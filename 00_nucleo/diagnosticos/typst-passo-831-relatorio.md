# Relatório — typst-passo-831: triagem sistemática, lote 5 (15 dos 22 módulos restantes)

**Data:** 2026-07-22
**Executor:** Kimi Code (agente principal; triagem executada por 6 subagentes em paralelo, cada um com fixtures próprios e saídas literais — prompt lido de `00_nucleo/materialization/typst-passo-831.md`).
**Proveniência das medições:** commit HEAD `3bcb695ea` (commit de P830, feito nesta sessão como pré-requisito). Working tree limpa no arranque da triagem (`git status --short` vazio). Todos os fixtures e assets em `temp/p831/` (pasta em `.gitignore`). Binários: cristalino `./target/release/typst` (rebuildado em release em 2026-07-22, pós-P830), vanilla `lab/typst-original/target/release/typst` (0.15.0, `969087ec`).
**Pré-requisito verificado:** P828 commitado (`56bc7cdc3`); suíte `cargo test --workspace` verde no arranque: **5257 passed; 0 failed** (typst-core 4517/0/2ign; typst-infra 672/0/5ign; demais crates 68/0). Nenhum teste novo neste passo (triagem não altera código — correcções são passos dedicados), logo a contagem mantém-se 4517 em typst-core.

---

## Passo 1 — Selecção do lote

- Inventário: `00_nucleo/diagnosticos/lente-lista-B-2026-07-15.txt`, secção `lacuna-inventario` → **82 módulos** (contagem verificada por `awk`/`sort -u` sobre os paths de módulo; ficheiro inalterado desde P810).
- Já triados: 45 (P785+P786+P798) + 15 (P810) = **60**. Restantes confirmados: **22** (82 − 60), sem discrepância face a P810.
- **Critério de selecção:** o mesmo de P810 — **maior superfície de língua** (estimada por nº de itens do módulo no inventário + inspecção da fonte vanilla). Seleccionados os 15 de maior superfície (Tiers A+B+C); ficam de fora exactamente os 7 de mecânica pura (Tier D).

**Os 15 seleccionados:**

| # | Módulo | Superfície |
|---|--------|-----------|
| 1 | `typst_library::visualize::image::raster` | 13 itens — `image()` raster PNG/JPEG/GIF, EXIF, DPI |
| 2 | `typst_library::visualize::image::svg` | 7 itens — `image()` SVG, fontes/imagens embutidas |
| 3 | `typst_library::text::font::variations` | 5 itens — `#text(variations:)` |
| 4 | `typst_library::text::font::metrics` | 5 itens — `top-edge:`/`bottom-edge:` |
| 5 | `typst_library::text::font::info` | 5 itens — name table, nomes de família |
| 6 | `typst_library::visualize::image::pdf` | 4 itens — `image()` com PDF fonte |
| 7 | `typst_library::text::font::book` | 3 itens — matching de famílias, fallback |
| 8 | `typst_library::text::font::exceptions` | 2 itens — excepções de nomes/pesos |
| 9 | `typst_library::layout::em` | 1 item — unidade `em` |
| 10 | `typst_library::layout` (define) | 1 item — registo de tipos/elementos layout + `measure`/`layout` |
| 11 | `typst_library::foundations` (define) | 1 item — registo de tipos + `repr`/`panic`/`assert`/`eval` |
| 12 | `typst_library::introspection` (define) | 1 item — `counter`/`state`/`query`/`here`/`locate`/metadata |
| 13 | `typst_library::loading` (define) | 1 item — registo dos loaders (já triados individualmente) |
| 14 | `typst_syntax::span` | 10 itens — posições de diagnóstico |
| 15 | `typst_library::layout::frame` | 4 itens — árvore de render (grupos/transforms) |

## Passo 2 — Triagem módulo a módulo

Convenção de comandos (raiz do repo): cristalino `./target/release/typst temp/p831/<f>.typ -o <out>.pdf`; vanilla `lab/typst-original/target/release/typst compile temp/p831/<f>.typ <out>.pdf`; texto com `pdftotext`, geometria com `pdftotext -bbox`/`pdfinfo`, raster com `pdftoppm`. **Nota transversal (registada uma vez, não contada como achado por caso):** o CLI cristalino emite erros em linha única `path:linha:col: error: msg` e o vanilla em formato codespan multi-linha — diferença sistémica de shell (L2), não de língua.

### 1. `visualize::image::raster` — **ACHADO** (com paridade extensa medida)

Paridade com evidência literal: PNG sem/com DPI pHYs (bbox idênticos, diferença só de arredondamento de impressão na 3.ª baseline: `293.803866` vs `293.804000`), JPEG JFIF 220 DPI (`87.310920` vs `87.311000`), rotação EXIF Orientation=6 (**pixel-idêntica** — `ImageChops.difference` → bbox `None`), formato desconhecido (`error: unknown image format` nos dois).

- **#17 (R1)** — GIF e WebP não suportados. `#image("raster3_gif.typ"...)`: cristalino `error: unknown image format` (exit 1); vanilla compila (GIF fica estático no frame 1). Vanilla: `raster.rs:80-81,248-251` (`GifDecoder`/`WebPDecoder`). Cristalino: `01_core/src/entities/image_format.rs:14-30` (enum só `Jpeg|Png|Unknown`).
- **#18 (R2)** — PNG corrupto: vanilla `error: failed to decode image (Format error decoding Png: ...)` (exit 1); cristalino **exit 0 com PDF válido e imagem omitida** (`eprintln!` "PNG inválido — imagem omitida"). Cristalino: validação só de assinatura em `figure_image.rs:189`; falha tardia em `03_infra/src/export/images.rs:348`. Vanilla: `raster.rs:448-453`. **Perda silenciosa de conteúdo.**

### 2. `visualize::image::svg` — **ACHADO**

- **#19 (S1)** — `#image("svg1.svg")` válido: cristalino `error: SVG images are not supported yet` (exit 1, rejeição em `figure_image.rs:167-173`); vanilla compila e renderiza. Scope-out explícito em código. Sub-casos de erro (SVG malformado, linked image em falta) divergem em mensagem por arrastamento. Vanilla: `svg.rs:54-98,160-169,343-358`.

### 3. `visualize::image::pdf` — **ACHADO**

- **#20 (P1)** — `#image("pdf1_fonte.pdf")`: cristalino `error: PDF images are not supported yet` (exit 1, `figure_image.rs:174-188` — scope-out consciente documentado em P781); vanilla compila. Inclui: parâmetro `page:` inexistente no cristalino (`error: argumento nomeado inesperado em image(): 'page'`). Vanilla: `pdf.rs:23-26,62-65,88-90`.

### 4. `text::font::variations` — **ACHADO**

- **#21** — `#text(variations: (wght: 250))`: cristalino `error: text() argumento nomeado desconhecido: 'variations'` (exit 1 em 5/5 fixtures); vanilla compila os válidos e dá erros específicos com hints nos inválidos (ex.: `tag must be one to four characters in length` + `found 5 characters`). Vanilla: `text/mod.rs:850`, `variations.rs:217-236`. Cristalino: `01_core/src/engine/eval/stdlib/text.rs:92-100` (qualquer named arg ≠ `fill` é erro hard).

### 5. `text::font::metrics` — **ACHADO** (paridade nas métricas nomeadas)

Paridade: bbox idênticos ao centésimo para `ascender/descender` (`yMin=10.000000 yMax=32.800000` em ambos), `cap-height/baseline` (`5.280000/28.080000`), `x-height/bounds` (`0.700000/23.500000`).

- **#22 (met3)** — `#set text(top-edge: "middle")`: vanilla `error: expected "ascender", "cap-height", "x-height", "baseline", "bounds", or length` (exit 1); cristalino exit 0 silencioso (cai no default). Cristalino: `engine/eval/rules.rs:1624-1629`, `03_infra/src/font_metrics.rs:175-178`. Vanilla: `text/mod.rs:1169-1177`.
- **#23 (met4)** — `#set text(top-edge: 18pt, bottom-edge: -4pt)`: vanilla `yMin=10.120000 yMax=32.920000`; cristalino `5.280000/28.080000` (= default — lengths descartados). Cristalino: `rules.rs:1624-1635` (`if let Value::Str` — `Value::Length` ignorado).

### 6. `text::font::book` — **ACHADO** (paridade no warning e no variant matching)

Paridade: warning `unknown font family: familia que nao existe` verbatim nos dois (span `1:16`; o vanilla 0.15.0 **não** emite hint "did you mean" — medido); matching de variantes `wght` com fonte variável Ubuntu (thin/black/450) com larguras idênticas palavra a palavra.

- **#24 (book2)** — fallback CJK: vanilla embute `NotoSansCJKjp-Regular` (16pt/glifo, scoring por similaridade — `book.rs:94-115,139-185`); cristalino escolhe a primeira fonte por ordem de índice que cobre o char (DroidSansFallbackFull ou similar, 9.6pt/glifo, quebra de linha diferente). Cristalino: `03_infra/src/shaper.rs:604-626`, `font_metrics.rs:708-733`, `fallback_fonts.rs:28-33` (sem CJK nas listas).

### 7. `text::font::info` — **ACHADO** (4 achados; fixtures com fontes sintéticas fontTools em `temp/p831/fonts/`)

- **#25 (I1)** — sem aparo de sufixos do ID1 (`TriagX Bold`): `#set text(font: "TriagX")` → cristalino `warning: unknown font family: triagx`, vanilla compila; e o inverso com `"TriagX Bold"`. Vanilla: `info.rs:73-77,206-267`. Cristalino: `03_infra/src/fonts.rs:189-198` (ID cru).
- **#26 (I2)** — sem `decode_mac_roman`: fonte só com nomes Macintosh (`TriagRésumé`) → vanilla encontra e embute; cristalino `warning: unknown font family`. Vanilla: `info.rs:168-203`. Cristalino: `fonts.rs:196`.
- **#27 (I3)** — sem inferência de estilo pelo full name (`TriagSlant Oblique` sem bits/ângulo): vanilla selecciona com `style: "oblique"`; cristalino marca `Normal`, selecção dependente da ordem do FontBook. Vanilla: `info.rs:80-103`. Cristalino: `fonts.rs:200-206`.
- **#28 (I4)** — FontBook↔font_slots desalinhados quando uma fonte falha a extracção de info: `discover_fonts` cria slot incondicional (`fonts.rs:159-164`), `build_font_book` faz push condicional (`fonts.rs:223-234`) → fallback renderiza com a face errada (11pt/22pt vs 16.5pt vanilla). Vanilla emparelha sempre (`typst-kit/src/fonts.rs:39-43`).

### 8. `text::font::exceptions` — **ACHADO** (a tabela não existe no cristalino)

- **#29 (E1)** — excepções de família ausentes. Com fonte **embutida nos dois binários**: `#set text(font: "New Computer Modern")` → cristalino `warning: unknown font family: new computer modern` (ID1 cru `NewComputerModern10`); vanilla compila com `NewCM10-Regular`. O nome documentado na referência Typst falha no cristalino. Vanilla: `exceptions.rs:5-7,46-342`.
- **#30 (E2)** — excepções de peso ausentes: face com `usWeightClass` errado (400 numa Bold) → vanilla selecciona-a com `weight: "bold"` (via tabela); cristalino fica na regular. Vanilla: `info.rs:60-61,105-112`.

### 9. `layout::em` — **ACHADO** (resto em paridade)

Paridade: 11 expressões eval verbatim idênticas (`1em`, `3.75em`, `-1.5em`, `6pt + 1em`, …); geometria `#h(1em)` a 10pt = gap 10.00pt nos dois, `#h(2em)` a 20pt = 40.00pt nos dois.

- **#31 (EM1)** — `#repr(2em - 5em)`: cristalino `error: cannot apply Sub to length and length` (exit 1); vanilla `-3em`. Falta o braço `Sub` para Length em `01_core/src/engine/eval/operators.rs` (só `Add` em `:382`). Vanilla: `foundations/ops.rs:196`.

### 10. `layout` (define) — **ACHADO** (8 achados)

Paridade: `layout(size => ...)` idêntico (`W: 180pt, H: 80pt`); 11/13 tipos registados idênticos; `(30% + 1em).length` → `1em`, `.ratio` → `30%`; `left + top`; `10pt + 2em`; `#h(50% + 1em)`; `box(width: 50%)`; erros verbatim idênticos em `m.keys()`/`m.width` sobre `context measure`.

- **#32 (L1)** — `type(50%)` → cristalino `length`, vanilla `ratio`; `type(30% + 1em)` → cristalino `length`, vanilla `relative`. Cristalino: `entities/value.rs:341` (`Relative → Type::Length`, com comentário que afirma paridade — refutado pela medição). Vanilla: `ratio.rs:62`, `rel.rs:76`.
- **#33 (L2)** — `measure` rejeita `width:`/`height:` (cristalino exit 1, mensagem declara scope-out graded **ADR-0054 — já documentado**); vanilla exit 0. Cristalino: `stdlib/layout.rs:1724`. Vanilla: `measure.rs:47,67`.
- **#34 (L3)** — `measure` devolve métricas diferentes para o mesmo conteúdo: cristalino `(width: 33pt, height: 14.85pt)` vs vanilla `(22.19pt, 7.24pt)`. Provável colateral do engine de texto; a investigar no passo dedicado.
- **#35 (L4)** — `#context (10pt)` → cristalino página vazia; vanilla `10pt`. `value_to_content` cai em `Content::Empty` para `Value::Length` (`stdlib/state.rs:137-161`, via `03_infra/src/pipeline.rs:156`).
- **#36 (L5)** — subtração de ângulos (`90deg - 45deg`): cristalino `error: cannot apply Sub to angle and angle` (exit 1); vanilla exit 0, `45deg`. Falta braço em `operators.rs:532`. Vanilla: `foundations/ops.rs:194`.
- **#37 (L6)** — `1fr + 2fr`: cristalino `error: cannot apply Add to fraction and fraction`; vanilla `3fr`. `operators.rs:532`. Vanilla: `foundations/ops.rs:127`.
- **#38 (L7)** — `#h(1fr)`: cristalino `error: h() espera amount como length, recebeu fraction`; vanilla compila (fractional spacing). Cristalino: `stdlib/layout.rs:879`. Vanilla: `spacing.rs:37,130-135`.
- **#39 (L8, menor — só mensagem)** — `2 * ltr`: ambos rejeitam (exit 1); cristalino `cannot apply Mul to int and direction` vs vanilla `cannot multiply integer with direction`.

### 11. `foundations` (define) — **ACHADO** (7 achados)

Paridade: `eval` idêntico; `repr` de 16 valores idênticos (decimal, version, symbol, regex, lengths, dirs, arrays, dicts, label, escapes, none, auto); `type(...)` de 16 tipos idênticos; `assert.eq` verbatim (`error: equality assertion failed: value 10 was not equal to 11`).

- **#40 (F1)** — `repr(duration)`: cristalino `duration(3s)`; vanilla `duration(seconds: 3)`. `eval/repr.rs:91` vs `foundations/duration.rs:137-160`.
- **#41 (F2)** — `repr` de content: cristalino `["hi"space*"bold"*]`; vanilla `sequence([hi], [ ], strong(body: [bold]))`. `repr.rs:329,339` vs `foundations/content/mod.rs:611`.
- **#42 (F3)** — `repr(type(none))`/`repr(type(auto))`: cristalino `none`/`auto`; vanilla `type(none)`/`type(auto)`. `repr.rs:124` vs `foundations/ty.rs:159-163`.
- **#43 (F4)** — tipo `bytes` sem constructor: cristalino `error: type bytes does not have a constructor`; vanilla `bytes(3)`. `eval/closures.rs:866-868` vs `foundations/bytes.rs:244-245`.
- **#44 (F5)** — tipo `datetime` sem constructor: idem (`closures.rs:866-868`) vs vanilla `datetime(year: 2024, ...)` (`foundations/datetime.rs:265`).
- **#45 (F6)** — `panic`: cristalino `error: this is wrong` (1 arg string, mensagem nua); vanilla `error: panicked with: this is wrong` (variádico, `panicked with: `, não-strings via repr). `stdlib/panic.rs:28-37` vs `foundations/mod.rs:140-152`.
- **#46 (F7)** — `assert`: cristalino `error: Asserção falhou` / mensagem nua com `message:`; vanilla `error: assertion failed` / `assertion failed: math broke`. `stdlib/assert.rs:68` vs `foundations/mod.rs:179-181`.

### 12. `introspection` (define) — **ACHADO** (8 achados)

Paridade: `here()` (`1 1`); `state` get/update/display básico; `locate(<label>)` (`Título|1`); `metadata` + `query(<label>).len()` (`1`); heading stepping sem context blocks; `locate(callback)` rejeitado nos dois (paridade de rejeição).

- **#47 (A1)** — `query()` devolve `location` em vez de `content`: `type(query(<meta>).first())` → cristalino `location`, vanilla `content`; colateral: `query(<meta>).first().value` → cristalino `error: cannot access fields on type location`, vanilla emite `segredo`. Cristalino: `stdlib/foundations.rs:1202-1213`. Vanilla: `introspection/query.rs:160`.
- **#48 (A2)** — `query()`/`locate()` não aceitam seletores de função de elemento (`locate(heading)`): cristalino exit 1 (`parse_selector_arg` sem braço `Value::Func`, `foundations.rs:1225-1278`); vanilla funciona. Vanilla: `locate.rs:28`, `query.rs:160`.
- **#49 (A3)** — `state.at`/`state.final`/`counter.final` inexistentes no dispatch de métodos (as nativas **existem**: `native_state_at` `foundations.rs:1154`, `native_state_final` `:1108`, `native_counter_final` `:1051` — falta ligar em `eval/bindings.rs:962-996,1187-1241`). Vanilla: `state.rs:274,288`, `counter.rs:467`.
- **#50 (A4)** — `counter.at()` não aceita `location` (`counter(heading).at(here())`): cristalino `error: counter.at() requer label ou string`; vanilla `3`. `bindings.rs:1229-1235` vs `counter.rs:452`.
- **#51 (A5)** — `#context ((3,))` → cristalino `3` (join `"."`); vanilla `(3,)` (repr). `stdlib/state.rs:148-159`. Fora de `context` o repr está correcto nos dois.
- **#52 (A6)** — `counter.display()` sem argumento ignora a numbering do `#set heading(numbering:)`: cristalino `1`, vanilla `1.`. `stdlib/counter.rs:182-186` vs `counter.rs:379`.
- **#53 (A7)** — `counter.display(pattern)` é stub: counter=2, patterns `"I"`/`"A"`/`"i"`/`"①"`/`"1.1"` → cristalino `I|A|i|①|2.1`, vanilla `II|B|ii|②|2`. `counter.rs:234-246` ("Pattern minimal" no próprio comentário) vs vanilla `counter.rs:634`.
- **#54 (A8)** — qualquer `#context` entre headings desalinha a numeração seguinte (off-by-one): `= Um / #context 1 / = Dois / = Três` → cristalino `1.|1.|2.`, vanilla `1.|2.|3.`. Sonda aponta `layout/heading.rs:58-84`, `entities/counter_registry.rs:127-134`, `pipeline.rs:107-160`.

### 13. `loading` (define) — **ACHADO** (1 colateral de render; loaders em paridade por efeito colateral — item 5 do passo)

Spot-checks de valor idênticos nos 7 loaders (csv/json/toml/yaml/cbor/xml/read) — paridade confirmada por efeito colateral dos lotes anteriores (csv/xml P786, yaml/toml P798, cbor/read P810), sem achados duplicados.

- **#55 (L1)** — texto com `\n` truncado na primeira newline no shaping (afecta a **exibição** de `read()`, não o valor): `#"a\nb\nc"` → cristalino só `a`; vanilla `a|b|c`. Causa: `bidi_runs` usa só `bidi.paragraphs[0]` (`03_infra/src/shaper.rs:830`). Vanilla: `typst-layout/src/inline/linebreak.rs:69`, `shaping.rs:675-690`.

### 14. `typst_syntax::span` — **ACHADO** (2 achados; paridade parcial)

Paridade (posição + mensagem verbatim): erro de sintaxe (`5:12 expected expression` em ambos), variável desconhecida em markup (`5:14`), erro em ficheiro importado (ambos apontam `span3_lib.typ:5:10`).

- **#56 (S1)** — span de erros dentro de `#eval` diverge: cristalino ancora ao span da lista de argumentos (`args.span`, `stdlib/eval.rs:135`), vanilla ao literal string (`SpanMode::Uniform`, `foundations/mod.rs:267,318`) — ex.: `span7.typ` cristalino `3:5` vs vanilla `4:2`. **Parcialmente documentado** no L0 `stdlib/eval.md` §3 (span âncora + «nuance de uma coluna» de P814); a medição mostra divergência maior que a nuance registada — a reavaliar no passo dedicado. Causa estrutural: `Args` cristalino não guarda span por item (débito P772s).
- **#57 (S2)** — call trace ausente: vanilla emite `while calling `boom` at ...`; cristalino omite (`Tracepoint` existe em `entities/source_result.rs:22` mas `trace` nunca é populado). Vanilla: `typst-eval/src/call.rs:168`, `diag.rs:446`.

### 15. `layout::frame` — **ACHADO** (2 achados; paridade parcial)

Paridade: `#place` com texto simples (texto idêntico); rotate de shape (rect vermelho rodado ~30° no raster cristalino).

- **#58 (F1) — GRAVE, silencioso** — texto dentro de `move`/`rotate`/`scale` é **omitido do PDF sem erro** (exit 0): `#rotate(30deg)[Rodado sozinho]` → página completamente em branco no cristalino; vanilla renderiza. Causa: `layout/helpers.rs:249-288` (`collect_items_at` só trata `Content::Shape` e `Content::Sequence`; resto cai no `_ => {}`), chamado de `layout/transform.rs:61`. Vanilla: `frame.rs:184,376`, `transform.rs:56`.
- **#59 (F2)** — `-15deg` rejeitado: cristalino `error: cannot apply Neg to angle` (exit 1); vanilla compila. Faltam braços `Neg` para `Angle`/`Ratio`/`Fraction`/`Duration` em `operators.rs:748-766`. Vanilla: `foundations/ops.rs:80`.

### Achado incidental (fora dos 15 módulos, emergiu na triagem)

- **#60 (X1)** — `array.join` inexistente: `#("a", "b").join("-")` → cristalino `error: type array has no method 'join'`; vanilla `a-b`. `stdlib/collections.rs:32-111` (19 métodos, falta `join`) vs vanilla `foundations/array.rs:755`.

## Taxa de sinal real

Cálculo: módulos com pelo menos um achado real (divergência medida com saída literal dos dois binários) / módulos triados = **15 / 15 = 100%**. Nenhum módulo saiu limpo — mas todos tiveram também paridade medida e documentada em partes da superfície (ver secções acima). Comparando com os lotes anteriores (P798 corrigido: 60% real), o salto é esperado: este lote concentrou os módulos de **maior superfície de língua** restantes — exactamente onde a migração é menos profunda (imagem, fontes, introspecção, operadores).

Dos 44 achados: **4 já estavam documentados** como scope-out/débito consciente (#19 SVG e #20 PDF-imagem em código/P781; #33 `measure` named args ADR-0054; #56 span de `#eval` parcialmente no L0 `stdlib/eval.md` §3 — embora a medição mostre divergência maior que a registada). Os restantes **40 são achados novos**. Nenhum achado exigiu decisão de escopo neste passo — todos seguem para a fila de passos dedicados; decisões de escopo (ex.: suportar GIF/WebP/SVG/PDF-imagem, call traces) ficam **pendentes de decisão do dono** nos passos respectivos, não fechadas aqui (lição P829/P830).

## Tabela de achados pendentes (fila — continuação da numeração de P810, que parou em #16)

| # | Módulo | Achado |
|---|--------|--------|
| 17 | image::raster | GIF/WebP não suportados (`unknown image format`) |
| 18 | image::raster | PNG corrupto omitido silenciosamente com exit 0 (perda de conteúdo) |
| 19 | image::svg | SVG não suportado — scope-out documentado; decisão de escopo pendente |
| 20 | image::pdf | PDF como fonte de `image()` não suportado (P781) + parâmetro `page:` ausente |
| 21 | font::variations | parâmetro `variations:` de `#text` inexistente |
| 22 | font::metrics | `top-edge:`/`bottom-edge:` inválido aceite silenciosamente |
| 23 | font::metrics | edges em `Length` descartados |
| 24 | font::book | fallback CJK por ordem de índice, sem similarity scoring |
| 25 | font::info | sem aparo de sufixos do name ID1/ID16 |
| 26 | font::info | sem `decode_mac_roman` para nomes Macintosh |
| 27 | font::info | sem inferência de estilo pelo full name |
| 28 | font::info | FontBook↔font_slots desalinhados quando extracção de info falha |
| 29 | font::exceptions | tabela de excepções de família ausente (afecta New Computer Modern embutida) |
| 30 | font::exceptions | tabela de excepções de peso ausente |
| 31 | layout::em | `Sub` length−length inexistente |
| 32 | layout (define) | `type(50%)`/`type(rel)` devolvem `length` em vez de `ratio`/`relative` |
| 33 | layout (define) | `measure` rejeita `width:`/`height:` (scope-out ADR-0054 documentado) |
| 34 | layout (define) | `measure` devolve métricas diferentes (33pt/14.85pt vs 22.19pt/7.24pt) |
| 35 | layout (define) | `Value::Length` devolvido de `#context` não renderiza |
| 36 | layout (define) | `Sub` angle−angle inexistente |
| 37 | layout (define) | `Add` fr+fr inexistente |
| 38 | layout (define) | `#h(1fr)` rejeitado |
| 39 | layout (define) | mensagem de `2 * ltr` diverge (menor) |
| 40 | foundations (define) | `repr` de duration diverge |
| 41 | foundations (define) | `repr` de content diverge |
| 42 | foundations (define) | `repr(type(none/auto))` diverge |
| 43 | foundations (define) | tipo `bytes` sem constructor |
| 44 | foundations (define) | tipo `datetime` sem constructor |
| 45 | foundations (define) | `panic`: assinatura e mensagem divergentes |
| 46 | foundations (define) | `assert`: mensagens divergentes |
| 47 | introspection | `query()` devolve `location` em vez de `content` |
| 48 | introspection | `query()`/`locate()` sem seletores de função de elemento |
| 49 | introspection | `state.at`/`state.final`/`counter.final` não ligados no dispatch (nativas existem) |
| 50 | introspection | `counter.at()` não aceita `location` |
| 51 | introspection | `#context` de array exibe join em vez de repr |
| 52 | introspection | `counter.display()` ignora numbering do `set` |
| 53 | introspection | `counter.display(pattern)` stub (sem estilos não-arábicos) |
| 54 | introspection | `#context` entre headings desalinha a numeração (off-by-one) |
| 55 | loading | `\n` em texto trunca o shaping (só `bidi.paragraphs[0]`) |
| 56 | syntax::span | span de erros dentro de `#eval` (parcialmente documentado; divergência > nuance registada) |
| 57 | syntax::span | call trace (`while calling ...`) ausente |
| 58 | layout::frame | **GRAVE**: texto em `move`/`rotate`/`scale` omitido do PDF silenciosamente |
| 59 | layout::frame | `Neg` de angle (e ratio/fraction/duration) inexistente |
| 60 | (incidental) foundations::array | `array.join` inexistente |

**Notas laterais registadas pelos agentes (fora dos módulos triados; para triagens/passos futuros, não numeradas):** espaçamento entre `#block`s consecutivos diverge (26.86pt vs 37.86pt — módulo block/par); `#text(size:)` como chamada rejeitado e `box(width:)` ignorado em pt absoluto (stdlib/text e box); redacção genérica dos erros de operadores (`cannot apply Add to length and int` vs `cannot add length and integer`); o CLI cristalino ignora a extensão do `-o` (escreve sempre PDF); o cristalino não aceita `#set page(height: auto)` (`expected length, float, or int, found auto`).

## Contagem actualizada

22 restantes − 15 triados = **7 módulos não triados**: `typst_library::foundations::styles::rule`, `typst_library::layout::abs`, `typst_library::layout::axes`, `typst_library::layout::corners`, `typst_library::layout::fragment`, `typst_utils::pico::bitcode`, `typst_utils::pico::exceptions` — todos de mecânica pura (Tier D; os 2 de `pico` já têm uma verificação escrita da reverificação de P785 classificando-os como mecânica). A varredura sistemática (lista de P772t) **ainda não está completa** — resta o lote 6 com estes 7.
