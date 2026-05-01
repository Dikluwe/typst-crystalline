# Relatório P171 — `state(key, init)` runtime mutable state (M9 sub-passo 3)

Executado em 2026-04-30. Terceira feature M9 do refactor Introspection.

## Resumo

- **Feature `state(key, init)` materializada** com runtime mutable state via `StateRegistry`.
- **Callbacks (`Func`) adiados** — P171 implementa apenas `Set` variant em `StateUpdate`. Vanilla `Func(Func)` permanece reservado para passo M9+ quando mecanismo de eval em walk context existir.
- **2 Content variants novos** (`State`, `StateUpdate`) + **2 ElementPayload variants** + **2 ElementKind variants**.
- **`StateRegistry` sub-store** criado (`HashMap<String, Vec<(Location, Value)>>`).
- **2 stdlib funcs**: `state(key, init)` e `state_update(key, value)` (forma funcional; methods-on-values adiados).
- **2 métodos novos no trait `Introspector`**: `state_value(key, location)` + `state_final_value(key)`.
- **Lacuna #4** (`numbering_active`): infraestrutura pronta — consumer (`Layouter`) não migra aqui (M5 em pausa).

## Verificações .I

| # | Critério | Estado |
|---|----------|--------|
| 1 | `cargo check --workspace` passa | ✅ |
| 2 | `cargo test --workspace` — todos passam, contagem aumenta | ✅ **1 374** lib (Δ +15 vs P170 = 1 359). Total workspace: 1 374 + 215 + 24 + 21 = 1 634 (Δ +15) |
| 3 | `crystalline-lint`: zero violations | ✅ "✓ No violations found" |
| 4 | `Content::State` e `Content::StateUpdate` existem | ✅ |
| 5 | `ElementPayload::State` e `ElementPayload::StateUpdate` existem | ✅ |
| 6 | `StateRegistry` em `entities/` | ✅ |
| 7 | `Introspector::state_value` e `state_final_value` existem | ✅ trait + impl |
| 8 | Stdlib `state(key, init)` e `state_update(key, value)` registadas | ✅ |
| 9 | Callbacks (`Func`) NÃO implementados — pendência documentada | ✅ |
| 10 | Snapshot tests ADR-0033 verdes | ✅ |
| 11 | Linter passa final | ✅ |

Δ tests: +15 — distribuídos por StateRegistry (7), StateUpdate (3), p171_state_feature E2E (5).

## Hashes finais

L0s novos:

| L0 | Hash do código L0 | `@prompt-hash` em L1 |
|----|-------------------|----------------------|
| `entities/state_update.md` | `0a0a535e` | `d244bda8` |
| `entities/state_registry.md` | `f00eb92f` | `b7e8d5ad` |

L0s modificados:

| L0 | Hash anterior (P170) | Hash actual | `@prompt-hash` em L1 |
|----|----------------------|-------------|----------------------|
| `entities/element_kind.md` | `1e8df079` | `4dd8a2b5` | `90bffae0` |
| `entities/element_payload.md` | `c49a4e16` | `6edd1b7e` | `724e8afd` |
| `entities/introspector.md` | `ee0371c4` | `530c5a98` | `f5187643` |
| `rules/introspect/extract_payload.md` | `2a0d9c0d` | `743d6d4e` | `493cdaed` |
| `rules/introspect/from_tags.md` | `d3c24085` | `80018900` | `a55b50db` |

`Content` arms novos (em `content.rs`, `introspect.rs`, `layout/mod.rs`, `locatable.rs`) sincronizados via lint.

## Decisões registadas em `.A`

### Callbacks `Func` — adiadas

Cristalino não tem mecanismo de eval de `Func` em walk context. Vanilla suporta `state.update(key, fn)` onde `fn` recebe valor actual e retorna novo. Implementar exigiria criar mecanismo de eval-em-walk (passo M9+ dedicado). P171 implementa só `Set` variant; enum `StateUpdate { Set(Box<Value>) }` deixa espaço para `Func(Func)` futuro sem mudar API.

### Estrutura `StateRegistry`: `HashMap<String, Vec<(Location, Value)>>`

Justificação:
- Lookup por key é O(1).
- Vec interno permite append em O(1).
- `value_at(key, location)` é O(n) onde n = número de updates da key (tipicamente pequeno).
- Locations são monotonicamente crescentes (Locator P161), portanto Vec na ordem de inserção é cronológico.

Alternativas rejeitadas: `BTreeMap<Location, ...>` (lookup por key complicado), `Vec<(Location, key, Value)>` (sort needed antes de lookup).

### Forma `StateUpdate`: enum

Adoptado enum `StateUpdate { Set(Box<Value>) }` em vez de struct. Razão: forward-compat para `Func(Func)` variant futuro sem partir API.

### Stdlib forma: funcional

`state_update(key, value)` em vez de `state.update(key, value)` método. Razão: cristalino não suporta methods em values; forma funcional cobre o caso. Refino futuro pode adicionar method dispatch via stdlib.

### State init/update sem init: defensive ignore

Decisão local em `StateRegistry`:
- Segundo `init` para mesma key → ignorado (primeiro ganha).
- `update` sem `init` prévio → ignorado.

Vanilla geraria erro nestes casos. Cristalino P171 minimal: comportamento defensivo (sem panic, sem erro). Refino futuro pode adicionar validação stricter se necessário.

## Tests novos (resumo)

- **`state_update::tests`** — 3 (set round-trip, distintos, hash determinismo).
- **`state_registry::tests`** — 7 (empty, init only, update após init, múltiplos updates, keys distintas, update sem init, segundo init ignorado).
- **`p171_state_feature` E2E** — 5 (state init acessível, update aplica no ponto correcto, state invisível em layout, keys distintas isoladas, state inexistente devolve None).

## Estado de M9

**3/11 features materializadas**:
1. P169: `metadata(value)` — Content variant + sub-store + stdlib.
2. P170: `CounterKey` hierarquia — refactor de tipo existente, lacuna #5 fechada.
3. **P171: `state(key, init)` + `state_update(key, value)`** — runtime mutable state, infraestrutura pronta para lacuna #4.

Próximas candidatas:
- `state.update(key, fn)` callback support — adicionar Func variant + mecanismo de eval em walk.
- `query()` user-facing — depende de tipo `Selector`.
- `here()` — depende de `Locator::current()` + EvalContext.
- `counter.at(label)` / `counter.final()` — desbloqueada por hierarquia P170.

## Lacuna #4

`is_numbering_active` / `numbering_active`: **infraestrutura pronta** (`StateRegistry` + `state_value` no trait). Consumer real (Layouter `state.numbering_active.get("heading")` legacy) **não migra em P171** — M5 em pausa. Quando M5 retomar, Layouter pode consumir `introspector.state_final_value("heading_numbering")` ou similar via mecanismo state — mas isto exige que `SetHeadingNumbering` walk arm passe a emitir `Content::StateUpdate` em vez de mutar directly `state.numbering_active`. Passo M5+ dedicado.

Lacuna #4 fica em **"infraestrutura pronta, consumer aguarda M5 retomar"**.

## Pendências cumulativas (M1+M2+M3+M4+M5+M9 P169-P171)

Lacunas em `m1-lacunas-captura.md`:

| # | Lacuna | Estado |
|---|--------|--------|
| 1 | `figure.kind` None vs "image" | Parcial (P168 figure-ref filter) |
| 2 | Auto-labels só em state | Adiar |
| 3 | Body frozen em state vs hash em tags | Manter (intencional) |
| 4 | `is_numbering_active` / `numbering_active` | **Infraestrutura pronta em P171; consumer aguarda M5** |
| 5 | `format_hierarchical` hierárquico | ✅ Resolvida em P170 |
| 6 | `bib_entries` / `bib_numbers` | Adiar — M9 feature dedicada |
| 7 | `has_outline` | Adiar — pode resolver via `query(Outline)` futuramente |

Outras pendências:
- Reshape opcional `CounterUpdate::Step` → `Step(usize)`.
- Refino opcional `hash_content`.
- `Position` ainda não materializado.
- `comemo::track` deferido (M7+).
- **Nova em P171**: `Func` callback em `StateUpdate` (mecanismo de eval em walk context).

## Estado pós-passo

- **P171 concluído**.
- **M9 3/11 features**.
- **Lacuna #4 infraestrutura pronta** (consumer real em M5).
- **P172 desbloqueado** — quarta feature M9.

API pública preservada. Output observable inalterado (verificado por test `state_e_invisivel_em_layout`). Sem ADR nova. Sem reservas.
