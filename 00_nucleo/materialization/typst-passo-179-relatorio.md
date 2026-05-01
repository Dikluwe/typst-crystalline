## Relatório P179 — `query` upgrade (M9 sub-passo 9 — refino P175)

Executado em 2026-04-29. Feature seguinte M9. Decisão tomada em `.A` entre 4 candidatas — escolhida **`query` upgrade** por menor pré-requisito arquitectural.

## Resumo

- **`Value::Location(Location)`** — variant novo no enum `Value`. Cascade trivial: nenhuma exhaustive match externa exigiu update (matches usam `_ => ...` fall-through ou deriva genericamente).
- **`Value::type_name()`** — arm `Location => "location"` adicionada.
- **Stdlib `query(kind_str)` modificada**: retorna `Value::Array(Vec<Value::Location>)` em vez de `Value::Int(count)`. Cliente que precisa apenas de count usa `len(query("heading"))`.
- **Tests P175 adaptados**: 3 stdlib tests passaram de `Value::Int` para `Value::Array(...)`. Comportamento e contagem preservados.
- **Walk inalterado**, API pública preservada.

## Verificações `.C`

| # | Critério | Estado |
|---|----------|--------|
| 1 | `cargo check --workspace` passa | ✅ |
| 2 | `cargo test --workspace` — todos passam | ✅ **1 440** lib (Δ +2 vs P178 = 1 438). Total workspace: 1 440 + 215 + 24 + 21 = **1 700** (Δ +2) |
| 3 | `crystalline-lint`: zero violations | ✅ |
| 4 | `Value::Location` variant existe | ✅ |
| 5 | Stdlib `query` retorna `Value::Array(Vec<Value::Location>)` | ✅ |
| 6 | Walk em `introspect.rs::walk` NÃO modificado | ✅ |
| 7 | Snapshot tests ADR-0033 verdes | ✅ |
| 8 | Linter passa final | ✅ |

Δ tests: +2 (líquido) — distribuídos por:
- `stdlib::tests`: +1 (`stdlib_query_p179_upgrade_value_location_type_name`); 3 tests existentes adaptados (Int → Array).
- `fixpoint::tests`: +1 (`p179_stdlib_query_retorna_locations_via_fixpoint`).

(3 testes adaptados não contam como Δ — comportamento preservado.)

## Hashes finais

L0s afectados:

| L0 | Hash actual | `@prompt-hash` em L1 | Mudança |
|----|-------------|----------------------|---------|
| (sem L0 modificado em P179) | — | — | Apenas L1 mudou |
| `entities/value.rs` | n/a | `02423035` | Variant nova adicionada |
| `rules/stdlib/foundations.rs` | n/a | `f6cc2443` | Stdlib upgrade |

P179 não tocou em L0s — apenas L1 (Value enum + stdlib func + tests). L0 `entities/value.md` (se existir) não foi modificado, mas a L1 reflecte a nova variant. `crystalline-lint --fix-hashes` confirmou: "Nothing to fix".

## Tests novos

**`stdlib::tests`** — 1 novo + 3 adaptados:
- (Adaptado) `stdlib_query_em_introspector_vazio_retorna_array_vazio` — antes Int(0), agora Array(vec![]).
- (Adaptado) `stdlib_query_em_introspector_populado_retorna_array_de_locations` — antes Int(3), agora Array com 3 Value::Location.
- (Adaptado) `stdlib_query_outline_funciona_pos_p178_p179` — antes Int(0/1), agora Array.
- (Novo) `stdlib_query_p179_upgrade_value_location_type_name` — `Value::Location.type_name() == "location"`.

**`fixpoint::tests`** — 1 novo:
- `p179_stdlib_query_retorna_locations_via_fixpoint` — E2E via `introspect_to_fixpoint`. Closure observa stdlib `native_query` em cada iter. Iter 0: `Value::Array(vec![])`; Iter 1: `Value::Array` com 2 `Value::Location` entries (verifica tipo).

## Decisões registadas em `.A`

### Feature escolhida: **`query` upgrade**

Avaliação factual das 4 candidatas:

| Candidata | Pré-req | Magnitude | Replica padrão? | Valor |
|-----------|---------|-----------|-----------------|-------|
| `here()` | infra "eval-time location" ausente | M-L | não | nenhuma |
| `locate()` | "eval em walk" ou Introspector circular | M-L | não | nenhuma |
| Bib state | inventário ausente | desconhecida | não | lacuna #6 |
| **`query` upgrade** | `Value::Location` localizado | **S-M** | sim (P175) | refino útil |

`query` upgrade venceu por:
- Pré-req mais localizado (`Value::Location` variant + cascade Value).
- Magnitude **confirmada S** após inventário (cascade real foi mínimo — Value matches usam `_` fall-through; só `type_name` foi exhaustive).
- Replica padrão estabelecido em P175 — apenas modifica payload de retorno, não cria infraestrutura nova.
- Refino útil: stdlib agora retorna informação semântica (Locations) em vez de só count. `len(query(...))` continua disponível para casos count-only.

### Pré-requisitos verificados em `.A`

- `Value::Array(Vec<Value>)` existe (P179 verifica em `value.rs:33`).
- `Location` deriva `Hash, Eq, Copy, PartialEq` (P179 verifica em `location.rs:18`).
- `Value` usa `_ => ...` fall-through em maioria dos matches → cascade trivial.

### Alternativas rejeitadas com justificação

- **`here()`** rejeitada: pré-req "current location durante eval" exige infraestrutura nova (mecanismo de Location atribuída durante eval, não walk). Magnitude L provável. Adiar para passo dedicado quando `Locator` durante eval for necessário também por outras features.
- **`locate()`** rejeitada: pré-req "eval em walk OR Introspector circular" — quebra invariante P163 (walk puro) ou exige resolução fixpoint elaborada. Magnitude L-XL.
- **Bib state** rejeitada: lacuna #6 sem inventário próprio. Sugerir como P_inventário antes de P_implementação. Magnitude desconhecida.

### Cascade `Value`: trivial

Esperava-se cascade médio em arms exhaustive. Realidade: apenas `Value::type_name()` (1 site) precisou arm novo. Outros sites usam `_ => ...` fall-through ou `match v { Value::X(_) => ... }` específicos.

## Estado de M9

**9/11 features materializadas**:
1. P169: `metadata(value)` — completa.
2. P170: `CounterKey` hierarquia — completa.
3. P171: `state(key, init)` + `state_update(key, value)` — completa.
4. P172+P173: `state_update_with(key, fn)` — completa.
5. P175 + **P179 upgrade**: `query(selector)` retorna Locations — completa.
6. P176: `counter.final(key)` — completa.
7. P177: `counter.at(label)` — completa.
8. P178: Outline cascade — completa.
9. **P179: `query` upgrade — completa (refino de P175)**.

Próximas candidatas:
- `here()` — pré-req "current location durante eval". Magnitude M-L.
- `locate(callback)` — pré-req "eval em walk OR Introspector circular". Magnitude L-XL.
- Bib state — pré-req inventário próprio. Magnitude desconhecida.

## Estado de M7

Sem mudança estrutural. Mecanismo + 3 clientes (P175/P176/P177); P179 é refino do cliente P175.

## Pendências cumulativas

Lacunas em `m1-lacunas-captura.md`:

| # | Lacuna | Estado |
|---|--------|--------|
| 1 | `figure.kind` None vs "image" | Parcial (P168) |
| 2 | Auto-labels só em state | Adiar |
| 3 | Body frozen em state vs hash em tags | Manter (intencional) |
| 4 | `is_numbering_active` / `numbering_active` | Infraestrutura pronta P171; consumer aguarda M5 |
| 5 | `format_hierarchical` hierárquico | ✅ Resolvida em P170 |
| 6 | `bib_entries` / `bib_numbers` | Adiar — magnitude desconhecida; precisa inventário próprio |
| 7 | `has_outline` | ✅ Resolvida em P178 |

Outras pendências (sem alteração face a P178):
- Reshape opcional `CounterUpdate::Step` → `Step(usize)`.
- Refino opcional `hash_content`.
- `Position` ainda não materializado.
- Erros de Func eval em from_tags propagam silenciosamente.
- `run_fixpoint` sem optimização early-exit.
- ✅ **Fechada em P179**: `Value::Location` ausente.
- ✅ **Fechada em P179**: stdlib `query` retornava count em vez de array.
- Selector apenas `Kind` variant (P175 limitação).
- CounterRegistry tem dual API.
- `ElementPayload::Outline` é unit (refino futuro).
- **Nova em P179**: `here()` e `locate()` continuam adiados; pré-requisitos arquitecturais não-triviais.

## Estado pós-passo

- **P179 concluído**.
- **M9 9/11 features**.
- **`query` upgrade fecha refino** referenciado em P175/P176.
- **P180 desbloqueado** — feature seguinte M9 ou início de outra fase. Candidatas:
  - `here()` (precisa infraestrutura "eval-time location") — passo dedicado M-L.
  - `locate(callback)` — passo dedicado L-XL.
  - Bib state inventário — P_inventário próprio antes de P_implementação.
  - M5 retomar com lacunas resolvidas (#5 P170, #7 P178).

API pública preservada na superfície externa (stdlib `query` mudou retorno mas é parte da extensão M9, não da API legacy preservada). Output observable inalterado em snapshot tests. Sem ADR nova. Walk puro. Sem reservas.
