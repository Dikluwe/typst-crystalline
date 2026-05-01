# Relatório P165 — `Introspector` + sub-stores + `from_tags` (M3)

Executado em 2026-04-30. Passo único de M3 do refactor Introspection.

## Resumo

- Trait `Introspector` + struct concreta **`TagIntrospector`** materializadas em `01_core/src/entities/introspector.rs`.
- 2 sub-stores criados: `LabelRegistry` (Label→Location) em `entities/label_registry.rs`; `CounterRegistry` (counters por kind) em `entities/counter_registry.rs`.
- Construtor `from_tags(&[Tag]) -> TagIntrospector` em `rules/introspect/from_tags.rs`. Match exaustivo sobre `ElementPayload` (force revisão quando variant novo for adicionado).
- Walk em `rules/introspect.rs` continua a popular `CounterStateLegacy` exactamente como antes; `pub fn introspect()` agora chama `from_tags(&tags)` em paralelo. Resultado descartado em M3 — M4-M5 começarão a expô-lo.
- API pública `introspect(content) -> CounterStateLegacy` preservada. Output observable inalterado.
- 5 tests E2E verificam consistência entre `Introspector` e `CounterStateLegacy` para os 3 kinds + queries por label + first/unique.

## Verificações .H

| # | Critério | Estado |
|---|----------|--------|
| 1 | `cargo check --workspace` passa | ✅ |
| 2 | `cargo test --workspace` — todos passam, contagem aumenta vs P164 | ✅ **1 590 tests** (Δ +26 vs P164 = 1 564) |
| 3 | `crystalline-lint`: zero violations | ✅ "✓ No violations found" |
| 4 | 4 ficheiros L1 novos | ✅ `label_registry.rs`, `counter_registry.rs`, `introspector.rs`, `from_tags.rs` |
| 5 | 4 L0 novos correspondentes | ✅ |
| 6 | L0 `introspect.md` actualizado para construção paralela | ✅ Hash `2e13b8b8` → `aecff834` |
| 7 | Walk emite tags como antes (lógica P162 preservada) | ✅ Body do walk inalterado neste passo |
| 8 | `pub fn introspect()` retorna `CounterStateLegacy` (assinatura preservada) | ✅ `_introspector` descartado |
| 9 | Snapshot tests ADR-0033 verdes | ✅ |
| 10 | Linter passa em verificação final | ✅ |

Δ tests: +26 (LabelRegistry 5 + CounterRegistry 6 + TagIntrospector 4 + from_tags 6 + E2E 5).

## Hashes finais

L0 novos (calculados pelo `crystalline-lint --fix-hashes`):

| L0 | Hash do código L0 | `@prompt-hash` em L1 |
|----|-------------------|----------------------|
| `entities/label_registry.md` | `630fada0` | `8bfee760` |
| `entities/counter_registry.md` | `bc222255` | `d1984390` |
| `entities/introspector.md` | `27afd094` | `fb507123` |
| `rules/introspect/from_tags.md` | `ff2f0f2f` | `f54c791a` |

L0 modificado:

| L0 | Hash anterior (P163) | Hash actual (P165) | `@prompt-hash` em L1 |
|----|----------------------|--------------------|-----------------------|
| `rules/introspect.md` | `2e13b8b8` | `aecff834` | `7c3acd7d` |

## Decisões registadas em .A

### Nome da implementação concreta: **`TagIntrospector`**

Convenção cristalina inspeccionada (`World` trait → `SystemWorld` / `MockWorld` / `FontMockWorld`): nomes descritivos do tipo de impl, sem sufixo genérico (`Impl`, `Default`). Adoptei `TagIntrospector` — descritivo da fonte (construído a partir de `Vec<Tag>`).

### Padrão `comemo::Track`: **deferido para M7+**

Cristalino usa `#[comemo::track] impl T { ... }` em `Route` e `Traced` (em `world_types.rs`). Decisão local (gate trivial): para M3 minimal viável, `Introspector` é trait plain (sem macro) e `TagIntrospector` implementa-o sem tracking. Justificação: M3 é só leitura-única após construção; sem necessidade de invalidação cross-iteration ainda. Tracking real fica para M7+ quando fixpoint exigir memoização.

### Sub-stores: nenhum existia

`LabelRegistry` e `CounterRegistry` ainda não existiam no cristalino (verificado por grep). Criados de raiz nos sub-passos .B e .C.

### `MetadataStore`: adiado para M9

Não criado em P165 conforme spec. Razão: `extract_payload` em M1 não tem variant `Metadata`. Pendência registada para M9 quando feature `metadata()` for adicionada.

### Forma simplificada de `CounterRegistry`

Decisão: `HashMap<String, Vec<usize>>` — flat counter por kind (1 nível). `CounterKey` enum vanilla (`Page | Selector(Selector) | Str(Str)`) **não** replicado em M3. Em vez disso, usar `String` directo como chave (`"heading"`, `"figure"`). Hierarquia rica adiada para M9+ paralelamente à introdução de `CounterKey` enum.

### Position vazio em M3

`TagIntrospector.kind_index` populado; `positions: HashMap<Location, Position>` adiado. `Introspector::position_of` retorna sempre `None` em M3. Mecanismo de população só virá quando layout integrar (M5+ ou M9 conforme ordem dos passos).

## Δ tests por componente

| Componente | Tests novos | Cumulativo |
|------------|-------------|------------|
| `LabelRegistry` | 5 | 5 |
| `CounterRegistry` | 6 | 11 |
| `TagIntrospector` | 4 | 15 |
| `from_tags` | 6 | 21 |
| E2E em `rules/introspect.rs` (`.G`) | 5 | 26 |
| **Total** | **26** | |

Baseline P164 = 1 564. Após P165 = 1 590. Δ = +26 tests, todos verdes.

## Estado pós-passo

**M3 concluído** (passo único P165). `Introspector` (trait) + `TagIntrospector` (impl concreta) + 2 sub-stores + `from_tags` materializados. Walk popula `Introspector` em paralelo a `CounterStateLegacy` em cada chamada a `introspect()`; resultado descartado em M3.

**M4 desbloqueado** — pode começar exposição do `Introspector` ao caller (entry point novo ou output adicional). Primeiro consumer real virá em M5 (provavelmente layout ou `materialize_time` migrando de `CounterStateLegacy.resolved_labels` para `query_by_label`).

**Pendências passadas para M4+** (cumulativas com M1+M2):
- 3 divergências em `m1-lacunas-captura.md` (figure.kind, auto-labels, body frozen).
- Reshape opcional `CounterUpdate::Step` → `Step(usize)`.
- Refino opcional de `hash_content`.
- **Novas em M3**:
  - `Position` ainda não materializado (M5/M9).
  - `comemo::track` deferido (M7+).
  - `CounterKey` enum vanilla deferido (M9).
  - `MetadataStore` adiado (M9 quando `metadata()` for adicionada).

API pública preservada. Sem ADR nova. Sem reservas.
