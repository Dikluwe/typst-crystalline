# Prompt L0 — `entities/state_update`
Hash do Código: 3fe54b8d

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/state_update.rs`
**Criado em**: 2026-04-30 (P171 sub-passo .B — feature `state(key, init)` M9)
**ADRs relevantes**: ADR-0033 (paridade vanilla), ADR-0066 (Introspection runtime)

---

## Contexto

`StateUpdate` é o enum que representa uma operação a aplicar ao runtime mutable state. Vanilla `StateUpdate { Set(Value), Func(Func) }`.

- **P171 (M9 sub-passo 3)**: `Set(Box<Value>)` apenas.
- **P172 (M9 sub-passo 4)**: `Func(Func)` adicionada. **Eval real é stub** — `Func::call` requer `Engine + EvalContext` que não estão disponíveis em `from_tags` (Engine só existe durante eval; from_tags corre depois). `from_tags` reconhece a variant mas **silenciosamente ignora** em `apply_update`. Eval real requer pipeline restructuring (M7+ fixpoint ou refactor dedicado).

Padrão `Box<Value>` consistente com `Content::Metadata` (P169) — evita ciclo Content→Value→Content em compilação.

---

## Restrições Estruturais

- Camada **L1**: enum puro.
- `Clone` derivado.
- `PartialEq` derivado; `Eq` via marker impl manual (Value não impl Eq por f64 NaN); `Hash` manual via `format!("{:?}", self).hash()` — padrão estabelecido em `entities/element_payload.rs` (P169).

---

## Interface pública

```rust
use crate::entities::func::Func;
use crate::entities::value::Value;

#[derive(Debug, Clone)]
pub enum StateUpdate {
    Set(Box<Value>),
    /// **P172** — callback. **Stub** — `from_tags` ignora silenciosamente.
    Func(Func),
}

impl PartialEq for StateUpdate { /* manual: Set por valor; Func por Arc::ptr_eq */ }
impl Eq for StateUpdate {}
impl std::hash::Hash for StateUpdate { /* via format!("{:?}", self) */ }
```

**Nota sobre derives** (P172): `PartialEq` deixou de ser derive porque `Func` interno é `Arc<FuncRepr>` sem `PartialEq`. Comparação `Func` é por ponteiro Arc (`Arc::ptr_eq`) — duas Funcs distintas com mesmo comportamento são `!=` (paridade vanilla).

---

## Semântica

- `Set(value)`: define o novo valor do state. Substitui valor anterior (se houver).
- `Func(fn)`: callback que receberia valor actual e retornaria novo. **Stub em P172** — `from_tags::apply_update` arm `Func(_)` é no-op. Pendência: eval real requer pipeline restructuring para passar `Engine + EvalContext` a `from_tags`.

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
