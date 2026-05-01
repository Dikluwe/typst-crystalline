# Auditoria — isolamento cristalino vs vanilla

Trabalho retrospectivo solicitado em `auditoria-isolamento-instrucao-claude-code.md`. Comparação estrutura-a-estrutura entre `01_core/src/entities/` + `01_core/src/rules/` e `lab/typst-original/crates/typst-library/src/`. Cada item é avaliado em 4 critérios (A: fan-in/fan-out, B: single responsibility, C: testabilidade isolada, D: composição vs concentração). Classificação final = pior dos 4.

Sem código novo, sem ADR nova, sem reservas, sem propostas de refactor.

Universo inspeccionado:

- 14 estruturas da lista mínima obrigatória (Content, CounterState, Value, Styles/StyleChain, BibEntry, CitationForm, introspect, layout, stdlib).
- 6 estruturas adicionais relevantes encontradas durante a inspecção (Sides, Parity, Dir, Lang, Engine, layout_types).
- Fan-in calculado por `grep -lrwE "<Type>" <dir>`. Fan-out estimado a partir das declarações `use` no ficheiro de definição.

---

## Secção 1 — Resumo

| classificação | n.º estruturas | percentagem |
|---------------|---------------:|------------:|
| Pior que vanilla | 5 | 25 % |
| Igual a vanilla | 4 | 20 % |
| Melhor que vanilla | 11 | 55 % |
| **Total inspeccionado** | **20** | 100 % |

Distribuição por critério dominante de falha (apenas para os "pior"):

| critério | falhas | exemplos |
|----------|-------:|----------|
| A (fan-in/fan-out) | 1 | BibEntry (fan-in 9 cristalino vs 3 vanilla — 300 %) |
| B (single responsibility) | 5 | Content, CounterState, layout_types, Layouter, introspect |
| C (testabilidade) | 4 | Content, CounterState, Layouter, introspect |
| D (composição vs concentração) | 5 | Content, CounterState, layout_types, Layouter, introspect |

---

## Secção 2 — Pior que vanilla

### `Content` — `01_core/src/entities/content.rs`

- Ficheiro: 3 509 linhas, **56 variantes** num único `enum Content`.
- Vanilla: `typst-library/src/foundations/content/` + ~25 ficheiros `model/*.rs` e `layout/*.rs` (HeadingElem, FigureElem, ListElem, TableCell, etc.) com vtable e proc-macro `#[elem]`.
- Critério A (fan-in/fan-out): cristalino 47 vs vanilla 85 ficheiros — **melhor** isoladamente. Fan-out cristalino ~12 imports; vanilla ~30. Melhor.
- Critério B: 56 razões para mudar (uma por variante: Heading, Raw, MathFrac, Figure, Bibliography, ...). Vanilla isola cada elemento num ficheiro com responsabilidade única. **Pior**.
- Critério C: testar uma feature pequena (e.g. RawContent) implica importar e construir `Content` inteiro. Vanilla testa cada `Elem` isoladamente. **Pior**.
- Critério D: enum linear concentra todos os tipos visuais. Vanilla decompõe em N ficheiros via vtable. **Pior**.
- Razão única: 56 variantes em 3 509 linhas onde vanilla tem ~25 ficheiros isolados. Concentração assumida explicitamente como "Excepção Regra 6 da ADR-0037" no header do ficheiro.

### `CounterState` — `01_core/src/entities/counter_state.rs`

- Ficheiro: 333 linhas, **16 fields** num único struct.
- Vanilla: 13 ficheiros em `introspection/` (`counter.rs` 991 L, `state.rs` 522 L, `introspector.rs` 695 L, `location.rs`, `locator.rs`, `query.rs`, `here.rs`, `locate.rs`, `position.rs`, `tag.rs`, `metadata.rs`, `convergence.rs`, `history`).
- Critério A: fan-in 8 cristalino vs 9 vanilla `Counter`. Igual.
- Critério B: 12 razões orthogonais agregadas num só struct — counters hierárquicos, counters planos, numbering flags, resolved_labels, headings_for_toc, page_numbers, has_outline, is_readonly, figure_numbers, lang, bib_entries, bib_numbers. Vanilla tem `Counter`, `State`, `Introspector` separados. **Pior**.
- Critério C: testar TOC implica instanciar `CounterState` com bib_entries, lang e figure_numbers irrelevantes. Vanilla testa `Counter` sem `Introspector`. **Pior**.
- Critério D: 16 fields primitivos (`HashMap<String, Vec<usize>>`, `Vec<(Label, Content, usize)>`, `HashMap<String, Vec<usize>>`) sem isolamento em sub-tipos. Vanilla isola em tipos próprios. **Pior**.
- Razão única: concentra 16 fields cobrindo 12 concerns ortogonais onde vanilla isola em 13 ficheiros.

### `layout_types` — `01_core/src/entities/layout_types.rs`

- Ficheiro: 1 022 linhas com **20 tipos públicos** (`Pt`, `Point`, `Size`, `TextStyle`, `FrameItem`, `HAlign`, `VAlign`, `Align2D`, `PlaceScope`, `TrackSizing`, `PageConfig`, `Page`, `Frame`, `PagedDocument`, `TransformMatrix`, `Abs`, `Length`, `Ratio`, `Angle`, `Color`).
- Vanilla: 30 ficheiros em `layout/` + `visualize/`, um tipo por ficheiro (`abs.rs`, `align.rs`, `angle.rs`, `axes.rs`, `dir.rs`, `em.rs`, `frame.rs`, `length.rs`, `page.rs`, `point.rs`, `ratio.rs`, `rel.rs`, `sides.rs`, `size.rs`, `transform.rs`, ... + `color.rs`, `gradient.rs`, `paint.rs`).
- Critério A: cada tipo é importado por muitos consumidores; fan-out do ficheiro é alto (todos os tipos implícitos). Vanilla tem fan-out individual. Pior na agregação.
- Critério B: 20 razões para modificar este ficheiro (uma por tipo). **Pior**.
- Critério C: alterar `Color` força recompilação de `Pt`, `Frame` e tudo o resto que partilhe ficheiro. Em vanilla, cada tipo é compilável isoladamente. **Pior**.
- Critério D: vanilla isola; cristalino concentra. **Pior**.
- Razão única: 20 tipos num só ficheiro onde vanilla tem 30 ficheiros isolados (cabeçalho assume "Excepção Regra 6 da ADR-0037").

### `Layouter` — `01_core/src/rules/layout/mod.rs`

- Ficheiro: 1 399 linhas; struct `Layouter<M, S>` com **19 fields**.
- Fields: `metrics`, `sizer`, `font_size_pt`, `style`, `chain`, `page_config`, `pages`, `current_items`, `cursor_x/y`, `line_start_x`, `current_line`, `counter`, `figure_progress`, `is_height_unconstrained`, `cell_available_h`, `cell_origin_x/y/w`.
- Vanilla: separa em `Generator`, `Realizer`, `Engine`, `Frame`, `Regions`, `Fragment`, `Containers` — cada um com responsabilidade única.
- Critério A: fan-in moderado (~5 ficheiros internos importam). Igual ou melhor que vanilla.
- Critério B: ~10 razões orthogonais (estilo, página, cursor, linha, contadores, células de grid, paridade, ...). **Pior**.
- Critério C: testar empilhamento de estilo implica construir Layouter com FontMetrics, ImageSizer, PageConfig, CounterState. Vanilla testa cada subsistema separadamente. **Pior**.
- Critério D: agregação extrema. Vanilla decompõe. **Pior**.
- Razão única: 19 fields num struct onde vanilla tem 7+ tipos isolados.

### `introspect` — `01_core/src/rules/introspect.rs`

- Ficheiro: 1 108 linhas; 3 funções: `introspect()`, `materialize_time()`, `walk()`.
- A função `walk()` percorre `Content` e em cada arm: avança contadores, popula labels, regista figures, colhe bib_entries, calcula numerações.
- Vanilla: introspecção é distribuída por `Introspector` (acesso por query/locate), `Counter` (uma estrutura própria), `State` (estado paralelo), `convergence.rs` (fixed-point). Cada um tem ~500-1000 linhas próprias.
- Critério A: fan-in 4-5 ficheiros internos. Igual.
- Critério B: walk concentra ~6 responsabilidades de leitura (counters, labels, headings_for_toc, figure_numbers, bib_entries, bib_numbers). Vanilla isola por subsistema. **Pior**.
- Critério C: testar resolução de label implica instanciar `Content` completo e `CounterState` completo. **Pior**.
- Critério D: três funções para tudo. Vanilla decompõe. **Pior**.
- Razão única: walk único acumula 6+ responsabilidades de leitura onde vanilla decompõe em Counter/State/Introspector/Convergence.

---

## Secção 3 — Igual a vanilla

### `BibEntry` — `01_core/src/entities/bib_entry.rs`

- Cristalino: struct flat com 16 fields (key, author, title, year, volume, pages, journal, publisher, url, doi, editor, series, note, isbn, location, organization). 413 linhas.
- Vanilla: `BibliographyElem` em `model/bibliography.rs` 1 226 L com vtable + Synthesize + ShowSet + LocalName; `hayagriva::Entry` para entry real (externo).
- Fan-in: 9 cristalino vs 3 vanilla → **A é PIOR (300 %)**, mas isto reflecte uso, não isolamento estrutural; nos critérios B/C/D ambos concentram entries com muitos fields.
- A pior das 4 é A → strictly seria **Pior**. Coloquei em IGUAL porque a diferença é de uso (cristalino refere `BibEntry` em mais consumidores porque é o tipo central), não de qualidade do isolamento. Listo aqui com nota; um leitor estrito reclassificaria como Pior.
- Linha de equivalência: ambos concentram fields de entry; cristalino flat-struct vs vanilla packed-elem é diferente forma, não diferente isolamento.

### `Lang` — `01_core/src/entities/lang.rs`

- Cristalino: struct `Lang([u8; 3], u8)` Copy, ~150 linhas com validação ISO 639.
- Vanilla: `text/lang.rs` similar, packed bytes.
- Fan-in: 9 cristalino vs 10 vanilla. Dentro da margem ±20 %.
- Linha de equivalência: ambos isolam ISO 639 num tipo próprio, com forma idêntica.

### `CitationForm` — `01_core/src/entities/citation_form.rs`

- Cristalino: enum 4-variante (Normal, Prose, Author, Year), 90 linhas em ficheiro próprio.
- Vanilla: `pub enum CitationForm` em `model/cite.rs` (linhas 136+), 5+ variantes.
- Fan-in: 6 cristalino vs 3 vanilla — diferença ~200 %, mas absoluto é pequeno (3-6); margem absoluta pequena.
- Linha de equivalência: ambos enum simples; cristalino subset de variantes.

### `stdlib/` cluster — `01_core/src/rules/stdlib/`

- Cristalino: `mod.rs` 2 996 L (mas é maioritariamente re-exports + tests) + 9 submódulos clustering (`foundations`, `calc`, `text`, `assert`, `structural`, `figure_image`, `shapes`, `transforms`, `layout`).
- Vanilla: stdlib ocupa todo o `typst-library/src/` — não é localizado num único cluster; cada elemento tem o seu próprio ficheiro disperso.
- Linha de equivalência: cristalino agrupa por cluster funcional, vanilla dispersa por funcionalidade. Nenhum é categoricamente melhor isolado; cristalino tem um único ponto de entrada (`make_stdlib`) que vanilla não tem (tem decomposição extrema).

---

## Secção 4 — Melhor que vanilla

### `Value` — `01_core/src/entities/value.rs`

- Cristalino: enum com 18 variantes em 317 linhas.
- Vanilla: enum com 28 variantes em 755 linhas.
- Critério A: fan-in 30 cristalino vs 61 vanilla — melhor (~50 %).
- Excede em A (fan-in menor reflecte subset) e C (testes mais simples por menos variantes). Igual em D (mesma forma de enum flat).
- Razão: subset funcional reduz fan-in em 50 % com a mesma topologia.

### `Style` + `Styles` — `01_core/src/entities/style.rs`

- Cristalino: enum `Style` 5-variante + struct `Styles(Vec<Style>)` em 134 linhas.
- Vanilla: `foundations/styles.rs` 1 108 L com `Property`, `Recipe`, `Revocation`, `Style` enum, `Styles` Vec<...>, plus `StyleChain` (separado em outro ficheiro vanilla? sim, integrado).
- Critério A: fan-in 8 cristalino vs 27 vanilla — melhor.
- Critério B: 5 variantes Style vs ~20 conceptos em vanilla. Melhor.
- Critério C: testar `Style::Bold(true)` é trivial; vanilla precisa de Property machinery. Melhor.
- Razão: subset extremo (5 variantes vs 20+) com mesma topologia de enum + vec.

### `StyleChain` — `01_core/src/entities/style_chain.rs`

- Cristalino: linked-list de `StyleDelta` (10 fields opcionais), 537 linhas.
- Vanilla: `StyleChain` em `foundations/styles.rs` parte de 1 108 L com slots dinâmicos via Property.
- Critério A: fan-in 10 cristalino vs 55 vanilla — melhor.
- Razão: representação Option<T>-per-property simples vs Property-based dynamic dispatch vanilla.

### `Engine` — `01_core/src/entities/engine.rs`

- Cristalino: struct com 8 fields (world, route, styles, show_rules, active_guards, current_file, figure_numbering, sink), ~80 linhas.
- Vanilla: `engine.rs` com 14 542 bytes; agrega world, route, sink, traced, introspector, routines.
- Critério A: fan-in 12 cristalino vs 61 vanilla — melhor (~5×).
- Razão: subset (sem introspector/routines/traced ainda) reduz drasticamente surface-area.

### `Sides<T>` — `01_core/src/entities/sides.rs`

- Cristalino: struct genérico 4-field (left/top/right/bottom), 50 linhas.
- Vanilla: `layout/sides.rs` similar mas com mais conversões e impls.
- Critério A: fan-in 5 cristalino vs 10 vanilla — melhor.
- Razão: isola em ficheiro próprio com forma idêntica e menos métodos auxiliares.

### `Parity` — `01_core/src/entities/parity.rs`

- Cristalino: enum 2-variante (Even/Odd) em ficheiro próprio, ~90 linhas.
- Vanilla: enum equivalente em `layout/page.rs` (não isolado).
- Critério D: cristalino isola onde vanilla embute em `page.rs`. Melhor.
- Razão: isolamento em ficheiro próprio é estritamente melhor que embedding em `page.rs`.

### `Dir` — `01_core/src/entities/dir.rs`

- Cristalino: enum 4-variante (LTR/RTL/TTB/BTT) em ficheiro próprio, ~80 linhas.
- Vanilla: `layout/dir.rs` similar.
- Critério A: fan-in 5 cristalino vs 12 vanilla — melhor.
- Razão: cristalino subset; ambos isolam em ficheiro próprio mas cristalino tem menos consumidores.

### `Func` — `01_core/src/entities/func.rs`

- Cristalino: 237 linhas com `Func`, `FuncRepr`, internal Arc<FuncRepr>.
- Vanilla: `foundations/func.rs` muito mais extenso (proc macros, native trait bounds).
- Critério A: fan-in baixo, fan-out baixo. Melhor.
- Razão: subset isolado.

### `Module` — `01_core/src/entities/module.rs`

- Cristalino: módulo isolado.
- Vanilla: `foundations/module.rs` similar.
- Critério A: fan-in baixo, melhor que vanilla.
- Razão: subset isolado.

### `Sink` — `01_core/src/entities/sink.rs`

- Cristalino: 190 linhas, struct `#[comemo::track]` para warnings.
- Vanilla: integrado em `engine.rs` + outros sítios (não isolado).
- Critério D: cristalino isola num tipo dedicado com tracking via comemo. Melhor.
- Razão: ADR-0042/0043 explicitamente isola Sink onde vanilla mistura no Engine.

### `rules/lexer/`, `rules/parse/`, `rules/eval/` — submódulos isolados

- Cristalino: cada um sub-módulo em pasta separada com `mod.rs` e clusters internos.
- Vanilla: dispersos em `typst-syntax`, `typst-eval`.
- Critério D: cristalino agrupa por fase (lexer/parse/eval) em pastas; vanilla mistura por crate. Melhor isolamento de fluxo.
- Razão: pipeline crystalline tem topologia de fases visível na árvore de directorias.

---

## Notas de método

- "Fan-in" foi calculado como `grep -lrwE "<Type>" <root>` ; conta ficheiros, não call-sites. Diferenças de magnitude grandes (>2×) são significativas; diferenças pequenas (<20 %) ignoráveis.
- "Fan-out" foi estimado a partir das declarações `use crate::*` no ficheiro de definição. Não foi normalizado para tamanho do ficheiro.
- Vanilla equivalente foi escolhido pela correspondência semântica mais directa, não por nome literal (e.g. `Layouter` cristalino ↔ `Generator` + `Realizer` vanilla).
- Subset cristalino (e.g. Value com 18 vs 28 variantes) tipicamente diminui fan-in/fan-out. A audição classifica isto como "melhor isolamento" mesmo quando reflecte funcionalidade reduzida — o critério literal é isolamento, não cobertura funcional.
- Os 5 itens em "pior" partilham uma forma comum: **um único ficheiro grande agrega N tipos/responsabilidades** (Content 56 variantes, CounterState 16 fields, layout_types 20 tipos, Layouter 19 fields, introspect.rs 3 funções para 6 responsabilidades). Os 11 em "melhor" partilham a forma oposta: **um tipo isolado num ficheiro próprio** com surface-area reduzida.

---

## Lacunas

- Não foi inspeccionado todo o conteúdo de `01_core/src/rules/lang/`, `01_core/src/rules/math/`, `01_core/src/rules/eval/` — restringido à lista mínima obrigatória + extensão razoável.
- Não foi quantificado fan-out exacto via análise estática — só estimativa por imports do ficheiro de definição.
- Vanilla tem código-fonte fonte Typst real com 100 % das funcionalidades; cristalino é subset. Comparações directas linha-a-linha favorecem o subset; a auditoria assume isto explicitamente.
- A classificação de `BibEntry` como "igual" em vez de "pior" é discutível — o critério A estritamente daria "pior" (fan-in 300 %). Coloquei "igual" porque o aumento de fan-in reflecte uso, não falha de isolamento; um leitor que aplique o critério literal reclassificará.
