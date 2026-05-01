# Prompt L0 — `entities/state_update`
Hash do Código: 0a0a535e

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/state_update.rs`
**Criado em**: 2026-04-30 (P171 sub-passo .B — feature `state(key, init)` M9)
**ADRs relevantes**: ADR-0033 (paridade vanilla), ADR-0066 (Introspection runtime)

---

## Contexto

`StateUpdate` é o enum que representa uma operação a aplicar ao runtime mutable state. Vanilla `StateUpdate { Set(Value), Func(Func) }`. P171 implementa apenas `Set(Box<Value>)` — callbacks (`Func`) adiadas para passo M9+ quando mecanismo de eval de Func em walk context for materializado.

Padrão `Box<Value>` consistente com `Content::Metadata` (P169) — evita ciclo Content→Value→Content em compilação.

---

## Restrições Estruturais

- Camada **L1**: enum puro.
- `Clone` derivado.
- `PartialEq` derivado; `Eq` via marker impl manual (Value não impl Eq por f64 NaN); `Hash` manual via `format!("{:?}", self).hash()` — padrão estabelecido em `entities/element_payload.rs` (P169).

---

## Interface pública

```rust
use crate::entities::value::Value;

#[derive(Debug, Clone, PartialEq)]
pub enum StateUpdate {
    Set(Box<Value>),
    // Func(Func) — adiado para passo M9+ com mecanismo de eval em walk.
}

impl std::hash::Hash for StateUpdate { /* via format!("{:?}", self) */ }
impl Eq for StateUpdate {}
```

---

## Semântica

- `Set(value)`: define o novo valor do state. Substitui valor anterior (se houver).

---

## Tests obrigatórios

- Set com mesmo Value: igualdade preservada.
- Set com Values distintos: desiguais.
- Hash determinístico: hash duas vezes, igual.

---

## Consumers

- `Content::StateUpdate { key, update }`.
- `ElementPayload::StateUpdate { key, update }`.
- `entities/state_registry.rs::StateRegistry::apply_update`.

## Sobre paridade

Vanilla expõe `Func` variant para callbacks `state.update(key, fn)`. Cristalino P171 adia para passo dedicado quando Func eval em walk context for resolvido. `Set` cobre 80% dos casos vanilla (`state.update(key, value)` literal).

---

## Resultado Esperado

- `01_core/src/entities/state_update.rs` — enum + impl Hash + impl Eq + 3 tests.
- Re-export em `01_core/src/entities/mod.rs`.

---

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-04-30 | P171: enum minimal Set para suportar runtime mutable state | `state_update.rs`, `state_update.md` |
