# Relatório — typst-passo-844: `introspection` — 8 achados (#47–#54)

**Origem**: achados #47 a #54 de P831 (lote 5), prompt `00_nucleo/materialization/typst-passo-844.md`.
**Estado**: concluído — 8/8 com paridade verbatim medida nos dois binários.

---

## Proveniência das medições

- **Commit HEAD**: `2cef88e22e52b88ef69f145f2b7f59434243dc8f` (P843). Todas as alterações deste passo estão **por commitar** (working tree); `git diff HEAD --stat` na secção final.
- **Binários**: vanilla `lab/typst-original/target/release/typst` (typst 0.15.0, 969087ec); cristalino `./target/release/typst` — "antes" medido com o binário de 22/07 19:53 (pré-P844), "depois" com rebuild `cargo build --release` de 22/07 ~23:4x UTC (pós-P844).
- **Fixtures**: `temp/p844/a1_…`–`a8_… .typ` (+ `probe_metadata.typ`, `probe_counterstep.typ` para a sonda #54). Texto extraído com `pdftotext` (saídas vazias filtradas).
- **Baseline de testes** (arranque, HEAD limpo): `cargo test -p typst-core` = **4621 passed, 0 failed**; `typst-infra` = **698 passed, 0 failed**. Nuance: na primeira corrida do baseline, `export::tests::p272_pdf_bytes_conic_rgb_unified_reproduziveis` falhou uma vez (697+1F) e passou na repetição imediata (698) — flake pré-existente, não relacionado com este passo.
- **Contagens finais**: `typst-core` = **4625 passed** (+4), `typst-infra` = **709 passed** (+11), `cargo test --workspace` todo verde (33+2+31+2 nos restantes crates). Delta bate com os testes novos `p844_a1_…`–`p844_a8_…` (4 L1 + 11 L3).

---

## #47 (A1) — `query()` devolvia `location` em vez de `content`

**Medição antes** (`temp/p844/a1_query_content.typ`):
- vanilla: `content ola` (type → `content`; `.value` → `ola`).
- cristalino: `error: cannot access fields on type location` (type seria `location`).

**Causa**: o introspector só indexava `Location`s; não havia resolução `Location → Content`.

**Código**:
- `01_core/src/entities/introspector.rs` — novo sub-store `TagIntrospector.elements: HashMap<Location, Content>` + método de trait `element_at(location) -> Option<&Content>`.
- `01_core/src/engine/introspect.rs` (`walk`, ponto de emissão da `Tag::Start`) — `intr.elements.insert(loc, content.clone())`.
- `01_core/src/engine/stdlib/foundations.rs` (`native_query`) — mapeia cada `Location` para `Value::Content` via `element_at`; **fallback** para `Value::Location` em introspectors sintéticos sem walk (preserva o contrato P179 dos testes que populam `kind_index` directamente).
- `01_core/src/entities/content.rs` — `get_field` ganha braço `(Content::Metadata(e), "value")`.
- `03_infra/src/measurements.rs` — `CountingIntrospector::element_at` delega sem `record_call` (padrão `heading_has_numbering`).

**Contratos ajustados (lidos antes de alterar)**: teste E2E de fixpoint `p179_stdlib_query_retorna_locations_via_fixpoint` renomeado para `p179_stdlib_query_via_fixpoint_retorna_content_p844` e actualizado (agora espera `Value::Content`); testes de `native_query` em `stdlib/mod.rs` inalterados (cobertos pelo fallback).

**Medição depois**: `content ola` — **verbatim vanilla**.

**Testes**: `p844_a1_query_devolve_content_com_campos` (L3), `p844_a1_walk_popula_elements_e_element_at` (L1 introspect), `p844_a1_metadata_get_field_value` (L1 metadata).

---

## #48 (A2) — `query()`/`locate()` sem seletor por função de elemento

**Medição antes** (`a2_selector_func.typ`): vanilla `Titulo / location 1`; cristalino `error: query() requer string ou location, recebeu function…`.

**Código** (`01_core/src/engine/stdlib/foundations.rs`): braço `Value::Func` em `parse_selector_arg` + helper `element_kind_of_native_func` (fn_addr_eq, padrão de `native_selector`). Subset: kinds com `kind_index` populado em L1 — **heading, figure, table, metadata**. Demais funções → erro verbatim medido: `only element functions can be used as selectors` (medido no vanilla com `query((x) => x)`).

**Limitação assumida (decisão minha)**: funções de elemento fora do subset (ex.: `strong`, `raw`) caem na mesma mensagem de erro em vez de seleccionar — os respectivos kinds não têm `kind_index` populado em L1 (P494: contagem só em L3 `query_helpers.rs`, fora do caminho `#context`), pelo que mapeá-las devolveria silenciosamente `[]`.

**Medição depois**: `Titulo / location 1` — **verbatim vanilla**.

**Testes**: `p844_a2_locate_aceita_funcao_de_elemento`, `p844_a2_query_func_nao_elemento_erro_verbatim` (L3).

---

## #49 (A3) — `state.at`/`state.final`/`counter.final` fora do dispatch

**Medição antes** (`a3_state_at_final.typ`): vanilla `0 3 (0,)`; cristalino `error: counter não tem método 'final'`.

**Código**:
- `01_core/src/engine/eval/bindings.rs` — braços `at`/`final` em `eval_state_method` (nova `state_at_dispatch`) e braço `final` em `eval_counter_method_value`.
- `01_core/src/engine/stdlib/state.rs` — `state_at_location`, `state_final` (fallback para init sem updates — medido: `state("s", 7).final()` → `7`).
- `01_core/src/engine/stdlib/counter.rs` — `counter_final` (fallback `[0]` → `(0,)`, medido).
- `01_core/src/entities/introspector.rs` — novo método de trait `counter_final_values` (delega a `CounterRegistry::value`).
- `03_infra/src/measurements.rs` — `INTROSPECTOR_METHODS`/`CALL_COUNTERS` 27→28; sentinel `p204g_introspector_call_counts_existe` actualizado para 28.

**Mensagens verbatim medidas no vanilla** (ordem também medida — validação de argumentos precede o gate de contexto): `missing argument: selector`; `unexpected argument`; `expected label, function, location, or selector, found integer`; `text is not locatable` (string em `at`); ``label `<x>` does not exist in the document``; `can only be used when context is known` (gate). Nota: o gate dos métodos novos usa a forma vanilla; o `state.get()`/`counter.get()` pré-existentes mantêm a mensagem antiga divergente (`… can only be used inside context`) — não tocado (fora de escopo).

**Medição depois**: `0 3 (0,)` — **verbatim vanilla**. Nota: o `(0,)` de `counter(heading).final()` reflecte a regra medida do vanilla 0.15.0: headings **sem** `numbering:` não stepam o counter (probe em `/tmp`: `get()` → `(0,)` sem set, `(2,)` com set).

**Testes**: `p844_a3_state_at_e_final_via_context`, `p844_a3_counter_final_via_context`, `p844_a3_state_at_erros_verbatim_vanilla` (L3).

---

## #50 (A4) — `counter.at()` não aceitava `location`

**Medição antes** (`a4_counter_at_location.typ`, com `#set heading(numbering: "1.")`): vanilla `1. Um 2. Dois 3. Tres (3,)`; cristalino `error: counter.at() requer label ou string como argumento`.

**Código** (`bindings.rs` + `counter.rs::counter_at_location`): o braço `at` foi reescrito — aceita `<label>` (AST), string, `Content::Label`, `Value::Label` (caminhos pré-existentes, comportamento inalterado) e **`Value::Location`** (novo, fallback `[0]` como `counter_get`). A helper P506 `extract_label_from_args` foi absorvida pelo braço e removida (dead code). Mensagens de aridade/tipo alinhadas com o vanilla (`missing argument: selector`, `unexpected argument`, `expected label, …`).

**Divergência conhecida (fora de escopo)**: `counter.at(<inexistente>)` mantém o comportamento pré-P844 (array vazio `()`); o vanilla erra ``label `<x>` does not exist in the document`` (medido). Em `state.at` (código novo) o erro verbatim já foi implementado.

**Medição depois**: `(3,)` — **verbatim vanilla**.

**Teste**: `p844_a4_counter_at_aceita_location` (L3).

---

## #51 (A5) — `#context ((3,))` mostrava join em vez de repr

**Medição antes** (`a5_context_array.typ`): vanilla `(3,) (3,)`; cristalino `3 (3,)` — fora de `#context` já estava correcto (P801); o achado era só o caminho `value_to_content`.

**Código** (`01_core/src/engine/stdlib/state.rs`): braço `Value::Array` de `value_to_content` passa a usar `repr::repr_value` (a rotina corrigida em P801); o join próprio com `.` foi removido.

**Medição depois**: `(3,) (3,)` — **verbatim vanilla**.

**Testes**: `p844_a5_value_to_content_array_usa_repr` (L1), `p844_a5_context_array_usa_repr` (L3). Colateral verificado: testes P506 que asserem `contains("1")`/`contains("2")` sobre displays de arrays continuam verdes (`(1,)` contém `1`).

---

## #52 (A6) — `counter.display()` ignorava numbering do `#set heading(numbering:)`

**Medição antes** (`a6_display_set_numbering.typ`): vanilla `1.`; cristalino `1`.

**Código** (`01_core/src/engine/stdlib/counter.rs`): o braço `[]` de `counter_display` lê o custom `"{key}.numbering.pattern"` da chain (`engine.styles`, mesmo canal `rules.rs` usado pelo próprio heading) e formata via `format_pattern`; sem pattern na chain mantém o join hierárquico.

**Medição depois**: `1.` — **verbatim vanilla** (controlo sem `numbering:` mantém `1`).

**Teste**: `p844_a6_display_sem_pattern_usa_numbering_do_set` (L3).

---

## #53 (A7) — `counter.display(pattern)` era stub

**Medição antes** (`a7_display_pattern.typ`, counter=2): vanilla `II B ii ② 2`; cristalino `I A i ① 2.1`.

**Verificação prévia (pedida no handoff)**: a lógica P793 (`structural.rs::format_pattern`) já cobria romano/alfabético e a semântica de tokens (descarte de tokens extra, repetição do último) — faltava só o token `①`. Não foi portado nada do vanilla; o stub foi **substituído pela partilha** da rotina existente.

**Código**:
- `01_core/src/engine/stdlib/structural.rs` — `format_pattern` promovido a `pub(crate)`; token `①` adicionado (0 → `⓪`; 1..=50 → ①..㊿; >50 → warning verbatim medido ``the number {n} is too large to be represented with the `arabic.o` numeral system`` + fallback decimal). Beneficia também `numbering()` (medido: `numbering("①", 2)` → `②`).
- `01_core/src/engine/stdlib/counter.rs` — `apply_numbering_pattern` (stub "Pattern minimal") removido; `display(pattern)` usa `format_pattern`.

**Medições auxiliares**: vanilla `numbering("①", 1|21|36)` → `① ㉑ ㊱`; `display("Cap")` → `Cbp` (o `a` é token!); `display("")` → `error: invalid numbering pattern`. Nuance: a mensagem de padrão sem token em `format_pattern` continua a variante cristalina pré-existente (`padrão de numeração inválido (…)`), não a verbatim vanilla — divergência pré-existente de `numbering()`, não agravada; registada para passo futuro.

**Medição depois**: `II B ii ② 2` — **verbatim vanilla**.

**Testes**: `p844_a7_display_pattern_aplica_estilos_reais` (L3), `p844_a7_numbering_circled_number` (L1).

---

## #54 (A8) — `#context` entre headings desalinhava a numeração

**Medição antes** (`a8_context_between_headings.typ`): vanilla `1.|2.|3.`; cristalino `1.|1.|2.` (Dois repetia o número de Um).

**Sonda (medida antes de assumir causa)**:
- `#metadata(1)` entre headings → numeração correcta `1.|2.|3.` (probe).
- `#counter(heading).step()` entre headings → `1.|3.|4.` consistente nas duas caminhadas (probe).
- Conclusão: o problema não é o `#context` tocar o contador — é a **remoção de um locatable**. O `ContextBlock` é locatable no walk de introspecção (consome uma `Location`), mas `expand_context_blocks` substitui-o por conteúdo não-locatable; o walk de layout tem `Locator` próprio (`layout/mod.rs::advance_locator_if_locatable`, invariante P185C "sincronizado-por-construção") e atribuía Locations desfasadas do introspector pré-expansão (`03_infra/src/pipeline.rs` passava `intr` pré-expansão + conteúdo expandido a `layout_with_introspector_and_metrics`). `CounterRegistry::value_at` devolvia então o snapshot anterior (Dois←[1], Tres←[2]) — exactamente o observado.

**Código**:
- `03_infra/src/pipeline.rs` — nova função pública `expand_context_blocks_and_reintrospect` (expansão + `introspect_with_introspector` do conteúdo expandido). A pipeline de produção usa-a e **re-injecta os styles CSL** no `BibStore` reconstruído (P429); o introspector reconstruído também alimenta `intr_for_positions` (P535 — as posições auto-toc sofriam da mesma dessincronização).
- O introspector pré-expansão continua a ser o usado **pela expansão** (necessário para resolver os blocos).

**Trade-off (decisão minha, documentada)**: a re-introspecção é feita sobre o conteúdo pós-show-rules expandido (o mesmo que o layout percorre), não sobre `module.introspection_content()` (P498). Em documentos sem show rules são idênticos; com show rules que transformem elementos locatable, o introspector de layout passa a reflectir o conteúdo realmente laid-out (mais coerente com a invariante P185C). Updates de state por closure (`StateUpdate::Func`) são ignorados em `introspect_with_introspector` antes e depois — sem mudança de comportamento nesse eixo.

**Medição depois**: `1. Um / 1 / 2. Dois / 3. Tres` — **verbatim vanilla**.

**Teste**: `p844_a8_context_entre_headings_nao_dessincroniza_numeracao` (L3; exercita a mesma função da pipeline de produção).

---

## L0 e lint

- Prompts actualizados com secção P844: `engine/stdlib/state.md`, `engine/stdlib/counter.md`, `engine/stdlib/foundations.md`, `entities/introspector.md`, `entities/content.md`, `entities/elements/metadata.md`, `engine/introspect.md`, `engine/introspect/fixpoint.md`, `engine/stdlib/structural.md`, `engine/eval.md`, `infra/pipeline.md`, `infra/measurements.md`, `infra.md`.
- `crystalline-lint --fix-hashes .` → "0 drift warnings remaining" (hashes propagados também aos ficheiros que referenciam esses prompts: `bibliography.rs`, `control_flow.rs`, `flow.rs`, `markup.rs`, `math.rs`, `modules.rs`, `entities/counter.rs`, `entities/state.rs`, `03_infra/src/lib.rs` — diffs de 2 linhas).
- `crystalline-lint .` → **exit 0**, 6 warnings — **idênticos ao baseline** (comparados via `git worktree` limpa em HEAD: mesmos 6 warnings, incluindo o V7 de `structural.md` órfão por header multi-`@prompt` pré-existente — nenhum warning novo introduzido).

## Ficheiros tocados (diff HEAD, working tree não commitada)

Código L1: `01_core/src/engine/eval/bindings.rs` (+226/−…), `engine/introspect.rs`, `engine/introspect/fixpoint.rs`, `engine/stdlib/{state,counter,foundations,structural}.rs`, `entities/{introspector,content,elements/metadata}.rs`, `engine/eval/tests.rs`. L3: `03_infra/src/{pipeline,measurements,integration_tests}.rs`. Hash-only: `engine/eval/{bibliography,control_flow,flow,markup,math,modules}.rs`, `entities/{counter,state}.rs`, `03_infra/src/lib.rs`. L0: 13 prompts em `00_nucleo/prompts/`. Fixtures: `temp/p844/`.

## Contagens antes/depois

| Suite | Antes | Depois | Delta |
|-------|-------|--------|-------|
| `cargo test -p typst-core` | 4621 passed / 0 failed | **4625 passed / 0 failed** | +4 (`p844_a1`×2, `p844_a5`, `p844_a7`) |
| `cargo test -p typst-infra` | 698 passed / 0 failed | **709 passed / 0 failed** | +11 (`p844_a1`–`p844_a8`) |
| `cargo test --workspace` | — | todo verde (4625+709+33+2+31+2) | — |
| `crystalline-lint .` | exit 0 (6 warnings) | **exit 0 (6 warnings, idênticos)** | 0 |

## Desvios/limitações a rever antes do commit

1. **#48**: subset de funções de elemento mapeadas (heading/figure/table/metadata); demais funções de elemento erram com `only element functions can be used as selectors` em vez de seleccionar (decisão minha — mapear sem `kind_index` devolveria `[]` silencioso).
2. **#50**: `counter.at(<label-inexistente>)` devolve `()`; vanilla erra ``label `<x>` does not exist in the document`` (medido). Não alterado para não tocar o caminho P506 existente.
3. **#53**: mensagem de padrão sem token é a cristalina pré-existente, não a verbatim `invalid numbering pattern` do vanilla.
4. **#54**: re-introspecção pós-expansão é sobre o conteúdo pós-show-rules (ver trade-off na secção A8).
5. Gate de contexto de `state.get`/`counter.get` (pré-existente) usa mensagem não-verbatim; os métodos novos usam a verbatim `can only be used when context is known`.
6. Flake pré-existente observado uma vez no baseline: `p272_pdf_bytes_conic_rgb_unified_reproduziveis` (passou em todas as corridas subsequentes, incluindo a final).
7. Renomeado o teste `p179_stdlib_query_retorna_locations_via_fixpoint` → `p179_stdlib_query_via_fixpoint_retorna_content_p844` (contrato mudou pelo achado #47).
