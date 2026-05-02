# Auditoria Fresh — Projecto Typst Cristalino

Data: 2026-04-29. Auditor inspecciona o estado actual do código contra critérios objectivos (O1–O5) e qualitativos (Q1–Q6). ADRs/diagnósticos/relatórios lidos como contexto factual, não como justificação.

Snapshot:
- 1 700 tests workspace (1 440 lib `01_core`, 215 `03_infra`, 24 `02_shell`, 21 CLI integration).
- `crystalline-lint .` → "✓ No violations found".
- Refactor Introspection P161–P180 fechado (15 passos, 9/11 features M9, lacunas #5/#7 fechadas, #6 inventariada).

---

## Secção 1 — Inventário

### `01_core/src/entities/` (56 ficheiros, ~15 273 linhas)

**Grandes**:
- `content.rs` — 3 560 linhas. Enum `Content` central, ~59 variants.
- `syntax_node.rs` — 1 102 linhas.
- `layout_types.rs` — 1 022 linhas. `Pt`, `Length`, `Color`, `TextStyle`, `Align2D`, `PlaceScope`, `TrackSizing`, `TransformMatrix`, `PageConfig`, `Page`, `FrameItem`, etc.
- `world_types.rs` — 698 linhas. `Bytes`, `Datetime`, `FileError`, `FileResult`, `Font`, `Library`, `Route`, `Traced`.
- `syntax_kind.rs` — 570 linhas, ~134 variants.
- `style_chain.rs` — 537 linhas.
- `bib_entry.rs` — 413 linhas. 16 fields.
- `counter_registry.rs` — 342 linhas.
- `font_book.rs` — 339 linhas.
- `counter_state_legacy.rs` — 330 linhas. **18 fields públicos** (mais 2 privados).
- `value.rs` — 324 linhas, 19 variants.
- `source.rs` — 313 linhas.
- `introspector.rs` — 313 linhas. Trait `Introspector` (12 métodos) + `TagIntrospector` struct (6 sub-stores).

**Sub-stores Introspection (M9 P165–P178)**:
- `label_registry.rs`, `counter_registry.rs`, `metadata_store.rs`, `state_registry.rs`, `state_update.rs`, `selector.rs`, `element_kind.rs` (7 variants), `element_payload.rs` (7 variants), `element_info.rs`, `tag.rs`, `location.rs`, `locator.rs`, `engine.rs`.

**Pequenos / utilitários**: `args.rs`, `citation_form.rs`, `dir.rs`, `file_id.rs`, `func.rs`, `label.rs`, `lang.rs`, `module.rs`, `parity.rs`, `ptr_eq_arc.rs`, `scope.rs`, `show.rs`, `sides.rs`, `sink.rs`, `source_result.rs`, `span.rs`, `style.rs`, `syntax_*.rs` (5 files), `geometry.rs`, `glyph_variants.rs`, `image_sizer.rs`, `font_list.rs`, `math_class.rs`, `math_constants.rs`, `operators.rs`, `package_spec.rs`.

### `01_core/src/rules/` (~28 622 linhas, incluindo tests)

- `layout/tests.rs` — 3 503 linhas.
- `stdlib/mod.rs` — 3 220 linhas (parte tests).
- `eval/tests.rs` — 2 739 linhas.
- `introspect.rs` — 1 986 linhas. Walk + `materialize_time` + `pub fn introspect()` + `pub fn introspect_with_introspector()` + tests E2E.
- `layout/mod.rs` — 1 445 linhas. `pub struct Layouter` + `pub fn layout()` + `pub fn layout_with_introspector()` + 101 referências `Content::`.
- `stdlib/{layout, structural, foundations, ...}` — ~3 000 linhas total.
- `parse/parser.rs` — 707 linhas.
- `eval/mod.rs` — 662 linhas. `EvalContext` + `pub fn eval()` + `eval_markup()` + `make_stdlib()`.
- `lexer/scanner.rs` — 645 linhas.
- `introspect/from_tags.rs` — 637 linhas. Match exhaustivo sobre `ElementPayload`.
- `introspect/fixpoint.rs` — 614 linhas. `run_fixpoint` + `introspect_to_fixpoint` + tests.
- `eval/rules.rs`, `eval/closures.rs`, `eval/markup.rs`, `eval/control_flow.rs`, `eval/bindings.rs`, `eval/operators.rs`, `eval/math.rs`, `eval/modules.rs`.
- `math/layout/`, `math/symbols.rs`.
- `introspect/{convergence, extract_payload, locatable, fixpoint, from_tags}.rs`.

### `02_shell/src/` (~530 linhas)
- `cli.rs` — 313 linhas.
- `diagnostic.rs` — 204 linhas.
- `lib.rs` — 13 linhas.

### `03_infra/src/` (~6 500 linhas, dos quais 2 684 são `integration_tests.rs`)
- `integration_tests.rs` — 2 684 linhas (suite E2E real).
- `export.rs` — 2 090 linhas. PDF export. ~69 funções.
- `pipeline.rs` — 600 linhas.
- `world.rs` — 457 linhas.
- `font_metrics.rs` — 303 linhas.
- `fonts.rs` — 243 linhas.

### `04_wiring/src/`
- `main.rs` — 106 linhas.

---

## Secção 2 — Tabela quantitativa (O1–O5)

Apenas estruturas significativas. Fan-in via `grep -lr` sobre `01_core/src/`.

| # | Estrutura | Ficheiro | O1 (linhas / fields | variants | métodos) | O2 fan-in | O2 fan-out (`use`) | O3 razões para mudar | O4 funções >100 linhas | O5 testabilidade isolada |
|---|-----------|----------|--------------------------------------------------------|-----------|--------------------|---------------------|-------------------------|--------------------------|
| 1 | `Content` enum | `entities/content.rs` | 3 560 / 59 variants / 165 métodos / 81 imports | 49 ficheiros | 7 | ~40 conceitos (markup, math, layout, transform, table, math composto, citation, bibliografia, outline, state, metadata, etc.) | match em layout/walk com 100+ arms; sem função >100 mas matches gigantes | sim, mas o tamanho arrasta |
| 2 | `CounterStateLegacy` | `entities/counter_state_legacy.rs` | 330 / 18 fields públicos / 25 métodos | 12 | 4 | bib + figures + outline + lang + label_pages + auto_label + numbering_active + readonly + figure local + headings_for_toc + resolved_labels + known_page_numbers — **>=12 conceitos ortogonais** | n/a | sim |
| 3 | `TagIntrospector` | `entities/introspector.rs` | 313 / 6 sub-stores / trait com 12 métodos | 7 | 9 | 6 sub-stores cobrem 6 domínios distintos (labels, counters, metadata, state, kind_index, figure_label_numbers) — coesão razoável | n/a | sim |
| 4 | `Layouter<M, S>` | `rules/layout/mod.rs` | 1 445 / 19 fields / muitos métodos / 12 imports | grande (consumidor central) | 12 | layout per-Content + page state + chain + figure progress + grid cell state — múltiplos eixos | match `layout_content` com 101 arms, "fn layout_content" é volumoso | difícil — instanciar Layouter requer FontMetrics+ImageSizer; tests usam `FixedMetrics`/`NullImageSizer` |
| 5 | `Engine<'a>` | `entities/engine.rs` | 76 / 8 fields | 11 ficheiros | 6 | aglomera world+route+styles+show_rules+active_guards+current_file+figure_numbering+sink — agregador transparente | n/a | exige stub de cada field para test |
| 6 | `EvalContext` | `rules/eval/mod.rs` (campo dentro) | 4 fields + introspector / `eval()` 100+ linhas | 25 | n/a | loop_iter + max + next_rule_id + introspector — coesão fraca (limite de loop é separável de introspector) | `eval()` ~80 linhas; `eval_markup` ~150+ | sim |
| 7 | `Value` enum | `entities/value.rs` | 324 / 19 variants / 9 impls From | 38 | 0 imports especiais | tipos Typst (None/Bool/Int/Float/Str/Array/Dict/Module/Datetime/Func/Content/Auto/Length/Ratio/Angle/Color/Fraction/Align/Location) | `type_name` match exhaustivo | sim |
| 8 | `SyntaxKind` enum | `entities/syntax_kind.rs` | 570 / 134 variants | grande | n/a | tipos lexicais — coesão alta | trivia checks, etc | sim |
| 9 | walk em `rules/introspect.rs` | `rules/introspect.rs` | 1 986 (ficheiro inteiro) / 1 fn `walk` recursiva com 336 referências `Content::` | 1 (consumidor único é `introspect()` API) | 5 | um arm por Content variant que precisa de tratamento — coesão alta, mas tamanho elevado | função `walk` >> 100 linhas; tem 50+ arms | sim, via `introspect_with_introspector` E2E |
| 10 | `Sub-stores Introspection` | `entities/{label,counter,metadata,state}_registry.rs`, `selector.rs`, `state_update.rs`, `tag.rs`, `location.rs`, `locator.rs`, `element_kind.rs`, `element_payload.rs`, `element_info.rs` | 30–342 linhas cada | 7 (introspector + from_tags + walk + extract_payload + alguns tests) | 1–4 | um conceito cada; coesão alta | n/a | sim, isoladamente |
| 11 | `BibEntry` | `entities/bib_entry.rs` | 413 / 16 fields | poucos | 1 | 1 conceito (entrada bibliográfica) com fields opcionais — coesão alta | n/a | sim |
| 12 | `run_fixpoint` + `introspect_to_fixpoint` | `rules/introspect/fixpoint.rs` | 614 (com tests) / 2 fns + `MAX_FIXPOINT_ITERATIONS` + `FixpointError` | poucos (só tests + entry points stdlib via P175+) | 8 | 1 conceito (loop convergência) | `run_fixpoint` ~30 linhas | sim |
| 13 | `pub fn export_pdf*` | `03_infra/src/export.rs` | 2 090 / ~69 funções privadas + 3 públicas | 1 (chamado por pipeline) | n/a | múltiplos conceitos PDF (estrutura, páginas, fontes, imagens, compressão, encriptação) | algumas fns >100 linhas | difícil — toca em filesystem-like data |

---

## Secção 3 — Tabela qualitativa (Q1–Q6)

| # | Estrutura | Q1 clareza propósito | Q2 apropriação domínio | Q3 fluência uso | Q4 robustez crescimento | Q5 honestidade nome | Q6 vs vanilla |
|---|-----------|----------------------|------------------------|------------------|--------------------------|---------------------|---------------|
| 1 | `Content` enum | ambíguo — nome genérico mas é a árvore inteira do documento (markup + math + layout + transform + table + bibliografia) | parcial — modela árvore declarativa do documento mas mistura camadas (markup-level com layout-time como `Frame`-esque) | match exhaustivo guia compilador, mas 59 variants exigem cuidado | invasivo — adicionar variant força arms em ~9 sítios (extract_payload, locatable, layout, eval, introspect walk, materialize_time, etc.) | aspiracional/genérico — "Content" não diz que é declarativo nem que distingue eval-tempo de layout-tempo | mais simples — vanilla usa vtable + proc macros + Arc manual; cristalino usa enum linear (~1070 linhas vs ~2578 linhas vanilla na pasta `content/`). **Melhor para testabilidade; pior para extensibilidade ortogonal** |
| 2 | `CounterStateLegacy` | enganador — nome diz "counter" mas guarda 12 conceitos não-counter (labels, page numbers, lang, bib, outline flags) | ad-hoc — 18 fields agregam o que era passado entre passes, sem domínio coerente | exige conhecimento implícito sobre quem escreve qual field e quando | invasivo — adicionar feature exige novo field, dual-state durante migração | desonesto — sufixo "Legacy" reconhece o problema mas o tipo continua a ser usado; sem deprecation enforced | pior — vanilla não tem equivalente (estado é fixpoint via `comemo`); cristalino acumulou aqui o que vanilla distribui em sub-stores tracked. **Pior arquitecturalmente, melhor para single-pass minimal** |
| 3 | `TagIntrospector` | claro — agregador de sub-stores read-only para queries | real — modela o Introspector vanilla via composição visível | trait + struct concreta guia bem; sub-stores como fields públicos permitem acesso directo | gracioso — sub-store novo é ficheiro novo + field novo + arm em `from_tags`; padrão estabelecido P165→P178 | honesto — "TagIntrospector" diz que constrói via Tags | mais simples — sem `comemo::track`, sem genérico em `P` (paged-only), sub-stores explícitos. **Pior em performance fixpoint (sem memoization); melhor em legibilidade e testabilidade** |
| 4 | `Layouter<M, S>` | ambíguo — nome diz "layout" mas guarda também progress, cell state, counter state, introspector | parcial — modela acumulador de páginas + estado de fluxo + métricas; mistura múltiplos eixos | difícil — 19 fields exigem atenção a invariantes não documentados (cursor_x/cursor_y/line_start_x relação) | invasivo — match `layout_content` cresce linearmente com Content variants; adicionar feature toca em layout, counter, introspector | ambíguo — "Layouter" sugere layout mas inclui state mutável de muitos eixos | mais simples — vanilla tem pipeline multi-stage (realize → layout → frame); cristalino agrega num único struct iterativo. **Mais fácil de seguir; pior para reuse** |
| 5 | `Engine<'a>` | claro — agregador transparente de 8 parâmetros | real — coesão por domínio (handle externo / fluxo / efeitos) | guiado — caller constrói Engine uma vez no eval root | gracioso — campo novo é `Engine` field novo (precedente P109) | honesto | mais pequeno — vanilla `Engine` tem 11+ campos incluindo `introspector`, `routines`, `traced`. Cristalino omitiu 3. **Igual em forma; pior em completude** |
| 6 | `EvalContext` | ambíguo — nome diz "eval context" mas o que ficou aqui é o resíduo dos campos não-Engine (loop counter + introspector) | ad-hoc — `loop_iterations` + `max_loop_iterations` + `next_rule_id` + `introspector` não partilham domínio coerente | superficial mas correcto | pequeno — adição segura | parcialmente honesto — 4 fields, sem claro tema | sem equivalente directo (vanilla agrega tudo em `Engine`). **Cristalino tem 2 estruturas (Engine + EvalContext) onde vanilla tem 1; complica o callsite** |
| 7 | `Value` enum | claro — valor do runtime Typst | real | guiado — `From` impls para tipos básicos + `type_name()` | gracioso — variants têm exemplos (Length/Ratio/Angle/Color/Fraction/Align/Location adicionados ao longo) | honesto | igual em conceito; vanilla tem mais variants (Symbol, Bytes, Decimal, Duration, Styles, Args, Type, Dyn) — **cristalino é subset assumido (~13 diferidos com comentário)** |
| 8 | `SyntaxKind` enum | claro | real | guiado | gracioso | honesto | similar tamanho (cristalino 134, vanilla similar ordem) — **igual** |
| 9 | walk em `introspect.rs` | claro — pré-passagem analítica | real (modela o que vanilla faz via comemo) | match exhaustivo guia | invasivo — todo Content variant novo exige arm aqui (336 referências `Content::`) | honesto | divergente — vanilla usa fixpoint via comemo. Cristalino single-pass + tags em paralelo. **Mais previsível; pior para forward refs sem fixpoint manual** |
| 10 | Sub-stores Introspection | claro — cada um modela um sub-domínio | real | guiado — métodos read-only + `pub(crate)` para mutação durante construção | gracioso | honesto | sem equivalente vanilla (vanilla agrega via `IndexMap` + comemo). **Cristalino mais explícito; pior para combinação cross-sub-store** |
| 11 | `BibEntry` | claro | real | guiado (builder pattern) | gracioso (16 fields refinados em P159A→P159G) | honesto | divergente — vanilla usa `hayagriva::Entry` rich + parser BibLaTeX. Cristalino é struct literal sem parsing. **Pior para compatibilidade BibLaTeX; melhor para isolation L1** |
| 12 | `run_fixpoint` + `introspect_to_fixpoint` | claro | real | closure-based; caller controla eval | gracioso (`introspect_to_fixpoint` é wrapper opt-in) | honesto | divergente — vanilla usa `comemo::analyze`. Cristalino loop linear sem memoization. **Pior em performance; melhor em transparência** |
| 13 | `export_pdf*` em `03_infra` | parcial — 3 fns públicas com nomes parecidos (sem font, com font, multifont) | real | guiado pelo tipo de input | gracioso (formato PDF é fechado) | parcialmente honesto (`export_pdf_multifont` é o real; outros são stub legacy?) | sem comparação directa (vanilla usa `typst-pdf` crate). Cristalino reimplementa. **Custo alto em mantenance; mais isolado** |

---

## Secção 4 — Findings

### F1 — `CounterStateLegacy` é depósito ad-hoc com 12+ conceitos ortogonais (Q2, O3)

**Estrutura**: `entities/counter_state_legacy.rs`.

**Descrição**: 18 fields públicos cobrindo: contadores hierárquicos/flat, numbering active flags, resolved_labels, headings_for_toc, auto_label_counter, label_pages (escrita por `references.rs`), known_page_numbers (lida por `outline.rs`), `has_outline`, `is_readonly`, figure_numbers, figure_label_numbers, local_figure_counters, lang, bib_entries, bib_numbers. Sufixo "Legacy" sinaliza intenção de remoção mas nenhum mecanismo enforce; field count cresceu até P159F (18 fields). Acoplamento com Layouter forte (`pub counter: CounterStateLegacy`).

Decisões prévias (P166–P180) reconhecem o problema (sub-stores Introspection paralelos) mas migração de consumers M5 está em pausa (1/6 consumer migrado; 5 bloqueados — ver `inventario-consumers-counter-state-legacy.md` P167). Lacunas #4 (numbering_active) e #6 (bib_entries/bib_numbers) ainda usam `CounterStateLegacy`.

**Magnitude**: grande. Eliminar `CounterStateLegacy` é objectivo M6, não trabalho local.

### F2 — `Content` enum tem 59 variants em ficheiro de 3 560 linhas (O1, Q4)

**Estrutura**: `entities/content.rs`.

**Descrição**: `Content` é a árvore declarativa. ADR-0026 escolheu enum em vez de vtable (decisão razoável). Mas o crescimento aceitou que **49 ficheiros importam `Content::`** e **layout/walk fazem 100+ arms cada**. Adicionar uma feature como `Outline` (P178) tocou em 5 sítios (`element_kind`, `element_payload`, `is_locatable`, `extract_payload`, `from_tags`). Adicionar uma feature com renderização (e.g. `Equation`) toca em ~9 sítios.

Cabeçalho do ficheiro (linha 7-13) reconhece: "Excepção Regra 6 da ADR-0037: ... ~1070 linhas aceitas como custo de coesão por domínio." Mas o ficheiro está em 3 560 linhas, **3.3× a estimativa anunciada**.

**Magnitude**: média. Coesão por domínio é razoável; tamanho cresceu além do declarado.

### F3 — `Layouter` tem 19 fields e match `layout_content` com 101 arms (O3, O4, Q3)

**Estrutura**: `rules/layout/mod.rs`.

**Descrição**: `Layouter<M, S>` agrega: métricas, sizer, font_size, style, chain, page_config, pages, current_items, cursor_x/y, line_start_x, current_line, counter (CounterStateLegacy embedded), introspector, figure_progress, is_height_unconstrained, cell_available_h, cell_origin_x/y/w. Várias destas têm invariantes implícitos (cursor_x/cursor_y/line_start_x precisam alinhar; cell_origin_x/y/w + cell_available_h são "todos Some em conjunto"). Tests usam `FixedMetrics`+`NullImageSizer` para isolar — mas os 19 fields complicam construção mental do invariante de cada arm.

**Magnitude**: média. Sintoma do mesmo problema F1 — Layouter herdou estado que `CounterStateLegacy` acumulou.

### F4 — Walk em `introspect.rs` tem 336 referências `Content::` em 1 986 linhas (O1, O4, Q4)

**Estrutura**: `rules/introspect.rs::walk`.

**Descrição**: walk recursivo + `materialize_time` recursivo, ambos com match exhaustivo sobre `Content`. Adicionar variant `Content` força arm aqui. Walk é puro (P163 invariante mantido), mas a função `walk` em si é gigante. Nos últimos passos (P162→P178) os arms cresceram conforme features novas.

**Magnitude**: média. Padrão estabelecido (match exhaustivo, sem `_ => fall-through` em walk) é defensivamente correcto, mas o tamanho do ficheiro cresce sem freio.

### F5 — `EvalContext` é depósito de "resíduo" não-Engine (Q1, Q2)

**Estrutura**: `rules/eval/mod.rs::EvalContext` (struct dentro do mod).

**Descrição**: 4 fields: `loop_iterations`, `max_loop_iterations`, `next_rule_id`, `introspector`. Os 3 primeiros são contadores monotónicos. `introspector` foi adicionado em P174 sem inventário. Domínio fragmentado — limite de loop não é da mesma área que snapshot de fixpoint.

ADR-0044 (Engine) e ADR-0036 (campos eval_*) explicam por que estes ficaram fora do `Engine`. Mas `EvalContext` ficou como saco-resto.

**Magnitude**: pequena. Honesta divisão de responsabilidades possível com renomeação ou splits.

### F6 — `CounterRegistry` tem dual API (`apply` vs `apply_at`) (Q3, Q4)

**Estrutura**: `entities/counter_registry.rs` (P177).

**Descrição**: `apply(key, update)` e `apply_hierarchical(key, level)` mantidos para compat com tests; `apply_at` e `apply_hierarchical_at` adicionados em P177 com Location para suportar `value_at`. Resultado: dois pares de métodos paralelos com semânticas distintas (history populada ou não). Risco: caller que use `apply` em vez de `apply_at` produz `value_at` com history vazia — silencioso.

Decisão pragmática (preservar tests sem refactor) tem custo permanente.

**Magnitude**: pequena.

### F7 — `bib_entries` / `bib_numbers` em `CounterStateLegacy` ainda não migrados (Q4, lacuna #6)

**Estrutura**: `entities/counter_state_legacy.rs:84-92` + `rules/introspect.rs:567-573` (walk arm).

**Descrição**: Walk arm `Content::Bibliography` muta state directamente em vez de emitir Tag. Quebra o padrão pós-P162 de walk emitir Tags + sub-store popular via `from_tags`. Documentado em `inventario-bib-state.md` (P180) com magnitude S-M para migração.

**Magnitude**: pequena.

### F8 — `auto_label_counter` em `CounterStateLegacy` mistura domínios (Q2)

**Estrutura**: `entities/counter_state_legacy.rs:45`.

**Descrição**: Field comentado como "Não representa numeração de secções — é apenas um gerador de IDs". Ou seja, está num struct chamado `CounterStateLegacy` mas não é counter. Sintoma de F1.

**Magnitude**: pequena (consequência de F1).

### F9 — Tests gigantes em `rules/layout/tests.rs` (3 503), `rules/eval/tests.rs` (2 739), `rules/stdlib/mod.rs` (3 220) (O5)

**Estruturas**: tests E2E.

**Descrição**: Tests E2E vivem em ficheiros monolíticos. ADR-0037 Regra 6 reconhece a excepção ("testes seguem o domínio") mas os ficheiros excederam o que está declarado (cabeçalho de `eval/tests.rs` diz ~2080 linhas; está em 2 739 — **30% acima** do declarado em comentário).

Tests funcionam, mas adicionar test obriga ler ficheiro grande para descobrir helpers existentes.

**Magnitude**: pequena-média.

### F10 — `format!("{:?}", x)` como hash determinístico em vários sítios (Q4)

**Estruturas**: `entities/content_hash.rs`, `entities/element_payload.rs::Hash`, `rules/introspect/convergence.rs::compute_tags_hash`, `entities/state_update.rs::Hash`.

**Descrição**: Hash via Debug-string + SipHash duplo. Determinismo depende de Debug derive permanecer estável. Documentado como "Fragilidade declarada" em vários L0s. Refino para hash recursivo manual está pendente desde P162.

Risco: alguma struct adiciona `#[derive(Debug)]` custom que muda formato → hashes mudam silenciosamente entre versões. Detectado apenas via tests de paridade.

**Magnitude**: pequena (pendência conhecida).

### F11 — `export.rs` em `03_infra` tem 2 090 linhas e ~69 funções (O1, Q3)

**Estrutura**: `03_infra/src/export.rs`.

**Descrição**: PDF export concentrado num ficheiro. 69 funções (entre privadas e públicas). Lê PNG/JPEG, faz compressão zlib, monta xobjects, encode glyphs, etc. Sem decomposição em módulos. Difícil de navegar.

**Magnitude**: média (não foi tocado pelo refactor Introspection P161-P180; preocupação separada).

### F12 — `Func` derives e `apply_func` cascade (Q4)

**Estrutura**: `entities/func.rs` + `rules/eval/closures.rs::apply_func` + `rules/introspect/from_tags.rs`.

**Descrição**: P173 adicionou cascade `Engine + EvalContext` opcional para `from_tags` poder avaliar `StateUpdate::Func`. Funcs em path legacy (`introspect()` sem Engine) são silenciosamente ignoradas. Comportamento documentado mas não detectado em tests por callers reais — apenas se utilizador escrever `state_update_with(...)` num path sem fixpoint. Erros de Func eval em from_tags propagam silenciosamente (defensive ignore P173).

**Magnitude**: pequena (refino com Sink documentado como pendência futura).

### F13 — `Value::Location` tipo "location" sem operações associadas (Q3)

**Estrutura**: `entities/value.rs:68-71` (P179).

**Descrição**: `Value::Location(Location)` adicionado em P179 para query upgrade. Mas não há stdlib funcs que tomem `Value::Location` como input — caller não pode fazer nada com Location além de contar (`len(query("heading"))`). Refino futuro pendente.

**Magnitude**: pequena (refino conhecido).

### F14 — `MAX_LOOP_ITERATIONS = 1_000_000` é hard-coded em `EvalContext::new()` (Q3)

**Estrutura**: `rules/eval/mod.rs::EvalContext::new`.

**Descrição**: Limite máximo é constante hard-coded; não é configurável via API pública (`EvalContext` é construído internamente em `eval()`). Apenas test helper `eval_for_test_with_limits` permite override.

**Magnitude**: muito pequena.

---

## Secção 5 — Pontos fortes

### P1 — `crystalline-lint .` zero violations

Linter próprio (V3 ForbiddenImport, V4 ImpureCore, V5 PromptDrift, etc.) corre sem reportar nada. Topologia L0–L4 respeitada em todo o `01_core/`.

### P2 — 1 700 tests verdes em workspace

Suite cobre lib (1 440) + integration (215+24+21). Refactor P161–P180 não introduziu regressões.

### P3 — Walk puro preservado em 15 passos consecutivos

P162 estabeleceu invariante "walk não modifica nada além de emitir Tags + popular CounterStateLegacy". Refactors P163–P178 mantiveram esta invariante mesmo perante feature complexa (`StateUpdate::Func` callbacks, P172–P173) — Resolution localizou eval em `from_tags`, não em walk.

### P4 — Sub-stores Introspection bem isolados

`LabelRegistry`, `CounterRegistry`, `MetadataStore`, `StateRegistry`, `BibStore` (planeado): cada um num ficheiro, ~150–340 linhas, single concern, tests co-localizados, mutação só via `pub(crate) fn apply*`. Padrão claramente replicável.

### P5 — `Content` é enum, não vtable

ADR-0026 evitou proc-macro + Arc manual + vtable do vanilla. Resultado: `01_core/src/entities/content.rs` 3 560 linhas vs vanilla `crates/typst-library/src/foundations/content/` 2 578 linhas em 6 ficheiros + proc-macros separados. Cristalino é maior em linhas mas todo Rust-native + match exhaustivo guia compilador.

### P6 — Match exhaustivo onde importa

`is_locatable`, `from_tags`, `Value::type_name` usam match exhaustivo (sem `_ => ...`). Adicionar variant força revisão. P164 documenta a decisão; P178 cascade Outline mostrou que funciona.

### P7 — `introspect_to_fixpoint` opt-in

P174 mecanismo + P175 primeiro cliente + P176/P177 replicações. Caller existente (`introspect()` legacy) não migrou — quem precisa de fixpoint usa entry point novo. Sem disrupção a 38 call-sites identificados em P167.

### P8 — API pública preservada em todos os 15 passos do refactor

`pub fn introspect(content: &Content) -> CounterStateLegacy` continua a compilar com mesma signature. Adições foram em entry points novos (`introspect_with_introspector`, `layout_with_introspector`, `introspect_to_fixpoint`), não modificações.

### P9 — `crystalline-lint --fix-hashes` sincroniza L0/L1

Mecanismo automático de hash-syncing entre prompts e código. Drift detectado por linter (V5 PromptDrift). Disciplina enforced.

### P10 — Cláusula gate trivial vs substancial

P163 estabeleceu vocabulário. Decisões locais (forma de Selector, tipo de retorno de stdlib, ordem de args) são "trivial" — prosseguem. Decisões arquitecturais (eval em walk, Engine cascade) são "substancial" — paralisam o passo. Aplicado consistentemente.

### P11 — `MockWorld` minimal em tests

Pattern reusável: `MockWorld { library, book, main_id }` + `with_engine!` macro. Tests podem construir Engine real com poucas linhas.

---

## Secção 6 — Resumo executivo

1. **Estruturas inspeccionadas (significativas)**: ~13 (incluindo `Content`, `CounterStateLegacy`, `TagIntrospector`, `Layouter`, `Engine`, `EvalContext`, `Value`, `SyntaxKind`, walk, sub-stores Introspection conjunto, `BibEntry`, `run_fixpoint`, `export_pdf*`).

2. **Estruturas com findings significativos (médios ou grandes)**: 4 (`CounterStateLegacy` F1 grande, `Content` F2 médio, `Layouter` F3 médio, walk F4 médio). Mais 8 findings pequenos (F5–F14).

3. **Estruturas claramente bem**: 5 (`TagIntrospector`, `Engine`, `Value`, sub-stores Introspection conjunto, `run_fixpoint`).

4. **Top 3-5 problemas mais pungentes**:
   - **F1 — `CounterStateLegacy`** com 18 fields + 12 conceitos ortogonais. Sufixo "Legacy" reconhece o problema; eliminação é objectivo M6 mas migração M5 está em pausa (1/6 consumers).
   - **F2 — `Content` em 3 560 linhas** com 59 variants e ~336 referências `Content::` no walk. Cresceu além do declarado (cabeçalho do ficheiro diz ~1070).
   - **F3 — `Layouter` com 19 fields** e match `layout_content` 101 arms. Herda o problema de `CounterStateLegacy` (campo `counter` embebido) e adiciona próprios eixos (cursor/cell/page).
   - **F4 — walk `introspect.rs` em 1 986 linhas** com 336 arms. Padrão exhaustivo é defensivo mas o ficheiro cresce sem freio.
   - **F5 — `EvalContext` 4 fields fragmentados**. Pequeno mas claramente arbitrário.

5. **Top 3-5 pontos fortes mais notáveis**:
   - **P1 — Linter zero violations** + **P2 — 1 700 tests verdes**. Disciplina infraestrutural.
   - **P3 — Walk puro preservado em 15 passos**. Invariante arquitectural respeitado mesmo perante features de natureza mista.
   - **P4 — Sub-stores Introspection bem isolados**. Pattern repetido 5+ vezes (P165, P169, P171, P177, P180-P181 planeado) sem deriva.
   - **P5/P6 — `Content` enum + match exhaustivo guiado pelo compilador**. Decisão arquitectural fundacional certa.
   - **P7/P8 — `introspect_to_fixpoint` opt-in + API pública preservada**. Refactor de 15 passos sem disrupção a callers.

6. **Avaliação geral**: O projecto está em **estado misto consistente**. A camada de Introspection nova (sub-stores, fixpoint mechanism, Content payload kinds, M9 features 9/11) é **arquitecturalmente coerente** — sub-stores isolados, walk puro, exhaustive matches, opt-in entry points. A camada legacy (`CounterStateLegacy`, `Layouter` com 19 fields, walk gigante) reconhece os seus próprios problemas (sufixo "Legacy", DEBT-10, lacunas registadas) mas o trabalho de migração M5/M6 está parado. Cristalino é mais simples que vanilla onde escolheu sê-lo (Content enum, `BibEntry` sem hayagriva, walk single-pass) e tem disciplina infraestrutural notável (linter, hash-syncing, ADRs). O risco maior é que `CounterStateLegacy` continue a viver até M5/M6 retomarem — o resto está sob controlo.
