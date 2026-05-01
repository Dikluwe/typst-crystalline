# Prompt L0 — `entities/state_registry`
Hash do Código: f00eb92f

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/state_registry.rs`
**Criado em**: 2026-04-30 (P171 sub-passo .E — sub-store de M9 state feature)
**ADRs relevantes**: ADR-0033 (paridade vanilla), ADR-0066 (Introspection runtime)

---

## Contexto

`StateRegistry` é o sub-store de runtime mutable state para `Introspector`. P171 (M9 sub-passo 3) — feature `state(key, init)` + `state_update(key, value)`.

Vanilla equivalente: `lab/typst-original/.../introspection/state.rs`. Cristalino isola num sub-store dedicado por simetria com `LabelRegistry`/`CounterRegistry`/`MetadataStore`.

---

## Estrutura

`HashMap<String, Vec<(Location, Value)>>` — para cada key, lista ordenada de pares (Location, value). Init é a primeira entrada (cronologicamente). Updates posteriores adicionam ao Vec.

Decisão (P171.A.3): escolhida `HashMap` por:
- Lookup por key é O(1).
- Vec interno permite append em O(1).
- `value_at(key, location)` é O(n) onde n = updates da key (aceitável; n é tipicamente pequeno).

Alternativas consideradas: `BTreeMap<Location, ...>` (ordenação automática mas lookup por key complicado), `Vec<(Location, key, Value)>` (sort needed antes de lookup).

---

## Restrições Estruturais

- Camada **L1**: struct puro, sem I/O.
- Read-only após construção (mutação só via `pub(crate) fn init` / `update` / `apply_update` durante construção em `from_tags`).
- `Clone` derivado.

---

## Interface pública

```rust
use crate::entities::location::Location;
use crate::entities::state_update::StateUpdate;
use crate::entities::value::Value;

#[derive(Debug, Clone, Default)]
pub struct StateRegistry { /* HashMap<String, Vec<(Location, Value)>> */ }

impl StateRegistry {
    pub fn empty() -> Self;
    pub fn value_at(&self, key: &str, location: Location) -> Option<&Value>;
    pub fn final_value(&self, key: &str) -> Option<&Value>;
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;

    pub(crate) fn init(&mut self, key: String, init: Value, location: Location);
    pub(crate) fn update(&mut self, key: String, value: Value, location: Location);
    pub(crate) fn apply_update(&mut self, key: String, update: StateUpdate, location: Location);
}
```

---

## Semântica

- `empty()`: registry vazio.
- `init(key, init, location)`: regista valor inicial. Apenas a **primeira chamada para cada key** é considerada — segundo init é ignorado (paridade vanilla; multi-init no mesmo doc é inválido mas não panic).
- `update(key, value, location)`: regista update. **Se key não foi inicializada, update é ignorado** (defensive — vanilla geraria erro mas P171 minimal não erra).
- `apply_update(key, update, location)`: forma de conveniência para `from_tags` — match sobre `StateUpdate` enum, delega a `update` para `Set` variant.
- `value_at(key, location)`: encontra último (key-value) pair com `loc <= location` na ordem do Vec; retorna value ou None.
- `final_value(key)`: retorna o último value registado (init se nenhum update, ou último update aplicado).

---

## Algoritmo `value_at`

```rust
fn value_at(&self, key: &str, location: Location) -> Option<&Value> {
    let history = self.inner.get(key)?;
    history
        .iter()
        .filter(|(loc, _)| loc.as_u128() <= location.as_u128())
        .last()
        .map(|(_, v)| v)
}
```

Comparação por `as_u128()` directa — Locations são monotonicamente crescentes pelo `Locator` (P161), portanto a ordem de inserção é a ordem cronológica.

---

## Tests obrigatórios (sub-passo .E P171)

- `empty()` retorna `None` em qualquer query.
- `init` only → `value_at` em qualquer location ≥ init_loc retorna init.
- `update` após `init` aplica no ponto correcto (antes do update: init; depois: novo).
- Múltiplos updates em sequência: cada location resolve para o valor correcto.
- Keys distintas isoladas.
- `update` sem `init` é ignorado.
- Segundo `init` para mesma key é ignorado.

---

## Consumers actuais

Nenhum no momento da criação. Consumido em P171 .F por `rules/introspect/from_tags.rs` arms `ElementPayload::State` e `ElementPayload::StateUpdate`.

## Consumers planeados

- `entities/introspector.rs::TagIntrospector.state` field — exposição via `state_value` / `state_final_value`.
- M5 retorno: `Layouter` pode consumir `state` para resolver flags como `numbering_active("heading")`.
- M9+ features que dependem de state runtime.

---

## Sobre paridade

Vanilla `state.rs` armazena state via fixpoint comemo. Cristalino simplifica para single-pass linear (sem fixpoint M7). Suficiente para casos comuns onde `state.update` aparece no mesmo documento que `state.value_at`. Refino futuro (M9+) pode adicionar fixpoint quando consumers reais exigirem.

---

## Resultado Esperado

- `01_core/src/entities/state_registry.rs` — struct + 8 métodos + 7 tests.
- Re-export em `01_core/src/entities/mod.rs`.

---

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-04-30 | P171: sub-store de runtime mutable state para Introspector M9 | `state_registry.rs`, `state_registry.md` |
