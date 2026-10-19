## Relatório P175 — `query(selector)` com Selector minimal (M9 sub-passo 5)

Executado em 2026-04-29. Quinta feature M9. **Primeira feature que usa fixpoint (P174)**.

## Resumo

- **`Selector` enum minimal** materializado em `01_core/src/entities/selector.rs`. Único variant: `Kind(ElementKind)`.
- **`Introspector::query(&Selector) -> Vec<Location>`** — método novo no trait, impl em `TagIntrospector` delega a `query_by_kind`.
- **Stdlib `query(kind_str)`** — registada em `make_stdlib`. Aceita string (`"heading"`, `"figure"`, etc.); retorna `Value::Int(count)` (forma minimal sem `Value::Location`).
- **`introspect_to_fixpoint`** entry point opt-in — wrapper directo sobre `run_fixpoint` com nome semanticamente claro.
- **`ElementKind::from_name(&str)`** helper para parse inverso.
- **Lacuna #7 (`has_outline`)**: NÃO fechada em P175. `ElementKind::Outline` não existe; `Content::Outline` não é payload-yielder. Documentado.
- **Walk inalterado**, `introspect_with_introspector` signature inalterada (P173 forma preservada).

## Verificações `.G`

| # | Critério | Estado |
|---|----------|--------|
| 1 | `cargo check --workspace` passa | ✅ |
| 2 | `cargo test --workspace` — todos passam | ✅ **1 408** lib (Δ +13 vs P174 = 1 395). Total workspace: 1 408 + 215 + 24 + 21 = **1 668** (Δ +13) |
| 3 | `crystalline-lint`: zero violations | ✅ |
| 4 | `Selector` tipo existe | ✅ enum minimal `Kind(ElementKind)` |
| 5 | `Introspector::query(selector)` no trait | ✅ |
| 6 | `TagIntrospector` impl de query | ✅ delega a query_by_kind |
| 7 | Stdlib `query(...)` registada | ✅ `query(kind_str) -> Int(count)` |
| 8 | `introspect_to_fixpoint` entry point novo | ✅ wrapper sobre run_fixpoint |
| 9 | Walk NÃO modificado | ✅ |
| 10 | `introspect()` legacy preservada | ✅ |
| 11 | `introspect_with_introspector` preservada | ✅ |
| 12 | Lacuna #7 resolvida | ❌ — Outline não está em ElementKind. Documentado em P175 como pendência. |
| 13 | Snapshot tests ADR-0033 verdes | ✅ |
| 14 | Linter passa final | ✅ |

Δ tests: +13 — distribuídos por:
- `selector::tests`: +3 (igualdade, kinds_distintos, hash_determinismo).
- `introspector::tests`: +3 (query_vazio, query_kind_devolve_locations_em_ordem, query_kind_isola_por_kind).
- `stdlib::tests`: +4 (query_em_introspector_vazio, query_populado, query_kind_invalido, query_arg_nao_string).
- `fixpoint::tests`: +3 (p175_query_em_doc_estavel_converge, p175_query_evolui_entre_iters_e_converge, p175_lacuna_7_outline_kind_ausente).

## Hashes finais

L0s novos (P175):

| L0 | Hash actual | `@prompt-hash` em L1 |
|----|-------------|----------------------|
| `entities/selector.md` | `3490d19c` | `92ddd3cd` |

L0s modificados (P175):

| L0 | Hash actual | `@prompt-hash` em L1 |
|----|-------------|----------------------|
| `entities/introspector.md` | `322924e5` | `932588ff` |
| `rules/introspect/fixpoint.md` | `03cec7ed` | `455bbe61` |

L0s não modificados mas afectados (alterações em L1 sem mudar L0):

| L0 | Hash actual | `@prompt-hash` em L1 |
|----|-------------|----------------------|
| `entities/element_kind.md` | `4dd8a2b5` | `90bffae0` (from_name adicionado em L1) |
| `rules/stdlib.md` | `6e6c49e4` | `f6cc2443` (native_query em L1) |

Hashes consolidados via `crystalline-lint --fix-hashes`.

## Tests novos

**`selector::tests`** — 3:
- `igualdade_estrutural`, `kinds_distintos_sao_diferentes`, `hash_determinismo`.

**`introspector::tests`** — 3:
- `query_vazio_devolve_vec_vazio`, `query_kind_devolve_locations_em_ordem`, `query_kind_isola_por_kind`.

**`stdlib::tests`** — 4:
- `stdlib_query_em_introspector_vazio_retorna_zero` — `Value::Int(0)`.
- `stdlib_query_em_introspector_populado_retorna_count` — `Value::Int(3)` para 3 headings.
- `stdlib_query_kind_invalido_retorna_err` — kind desconhecido erro.
- `stdlib_query_arg_nao_string_retorna_err` — type-check.

**`fixpoint::tests`** — 3 (P175 E2E):
- `p175_query_em_doc_estavel_converge` — introspect_to_fixpoint sobre 2 headings.
- `p175_query_evolui_entre_iters_e_converge` — observação iter 0 vê 0; iter 1 vê 1.
- `p175_lacuna_7_outline_kind_ausente` — documenta estado da lacuna #7.

## Decisões registadas em `.A`

### Tipo de retorno de query: LOCATIONS (no trait) + COUNT (na stdlib)

Trait `Introspector::query -> Vec<Location>` — alinhado com `query_by_kind` existente.

Stdlib `query(kind_str) -> Value::Int(count)` — forma minimal **Opção β**. `Value::Location` não existe em cristalino; cascade para adicionar é alta. Suficiente para casos como `has_outline := query("outline") > 0` (quando Outline existir).

Refino futuro: adicionar `Value::Location(Location)` + retornar `Value::Array(Vec<Location>)`. Adiar.

### Forma de Selector: minimal Kind variant

Apenas `Kind(ElementKind)`. Variants vanilla (`Label`, `And`, `Or`, `Where`, `Before`, `After`, `Regex`, ...) adiados para passos dedicados quando consumers reais necessitarem.

### Adopção de fixpoint: Entry point novo opt-in

`introspect_to_fixpoint` wrapper sobre `run_fixpoint`. Adopção pontual; callers existentes (`introspect()` legacy, Layouter pipeline) **não migram**.

### `ElementKind::Outline`: ausente

`ElementKind` tem 6 variants (Heading, Figure, Citation, Metadata, State, StateUpdate). Outline não existe. `Content::Outline` existe mas não é payload-yielder em walk.

Lacuna #7 (`has_outline`) **NÃO fecha** em P175. Resolvendo requereria:
1. Adicionar `ElementKind::Outline`.
2. Adicionar arm em `extract_payload` para emitir tag.
3. Adicionar arm em walk para tag emission.
4. Adicionar `ElementPayload::Outline` (mesmo que vazio).

Trabalho cascade médio. Adiar para passo dedicado quando consumer real necessite.

### Stdlib forma: string como kind

`query(kind_str: Str)` em vez de `query(selector: Value::Selector)`. Razão: `Value::Selector` exigiria novo Value variant (cascade alto). String é minimal viable.

## Estado de M9

**5/11 features materializadas**:
1. P169: `metadata(value)` — completa.
2. P170: `CounterKey` hierarquia — completa.
3. P171: `state(key, init)` + `state_update(key, value)` — completa.
4. P172+P173: `state_update_with(key, fn)` — completa.
5. **P175: `query(selector)` minimal** — completa (count-based).

Próximas candidatas pós-P175:
- `here()` — depende de `Locator::current()` + EvalContext.
- `counter.at(label)` / `counter.final()` — capitaliza P170 + LabelRegistry.
- `locate(callback)` — depende de Position type.
- `query` upgrade para retornar Locations/Content (refino de P175).

## Estado de M7

**2/N sub-passos**:
1. P174: mecanismo de fixpoint (loop + convergência + EvalContext.introspector).
2. **P175: primeiro cliente do fixpoint** (`introspect_to_fixpoint` + stdlib `query`).

Cascade Engine + fixpoint validados em conjunto. Padrão estabelecido para features M9+ que dependem de introspector.

## Pendências cumulativas

Lacunas em `m1-lacunas-captura.md`:

| # | Lacuna | Estado |
|---|--------|--------|
| 1 | `figure.kind` None vs "image" | Parcial (P168) |
| 2 | Auto-labels só em state | Adiar |
| 3 | Body frozen em state vs hash em tags | Manter (intencional) |
| 4 | `is_numbering_active` / `numbering_active` | Infraestrutura pronta P171; consumer aguarda M5 |
| 5 | `format_hierarchical` hierárquico | ✅ Resolvida em P170 |
| 6 | `bib_entries` / `bib_numbers` | Adiar — M9 feature dedicada |
| 7 | `has_outline` | **Parcial em P175** — `query` mecanismo pronto, mas `ElementKind::Outline` ausente. |

Outras pendências:
- Reshape opcional `CounterUpdate::Step` → `Step(usize)`.
- Refino opcional `hash_content`.
- `Position` ainda não materializado.
- Erros de Func eval em from_tags propagam silenciosamente.
- `run_fixpoint` sem optimização early-exit.
- **Nova em P175**: `Value::Location` ausente — stdlib `query` retorna count em vez de array de locations.
- **Nova em P175**: Selector apenas `Kind` variant — futures variants (`Label`, `And`, `Or`, `Where`) adiados.

## Estado pós-passo

- **P175 concluído**.
- **M9 5/11 features**.
- **M7 2/N sub-passos**.
- **Lacuna #7 parcial** — `query` pronta; `ElementKind::Outline` aguarda passo dedicado.
- **P176 desbloqueado** — feature seguinte M9 (candidata: `here()` ou `counter.at`).

API pública preservada. Output observable inalterado em snapshot tests. Sem ADR nova. Walk puro. Sem reservas.
