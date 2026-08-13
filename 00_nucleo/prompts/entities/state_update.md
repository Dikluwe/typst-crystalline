# Prompt L0 — `entities/state_update`
Hash do Código: 8716ebb4

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

Vanilla aceita função em `state("key").update(fn)`. Cristalino P171 adia para passo dedicado quando Func eval em walk context for resolvido. `Set` cobre o caso literal (`state("key").update(value)`).

> **Fonte de paridade (P1031)** — **citação literal**, vanilla ratificado (`e0e8ca4d`),
> `crates/typst-library/src/introspection/state.rs:336-337`, doc comment do parâmetro de
> `state.update`, publicado em
> `typst.app/docs/reference/introspection/state/#definitions-update`:
>
> *"- If given a function, that function receives the state's previous value and has to
> return the state's new value."*
>
> Confirma que a forma-função existe na linguagem, e a forma de superfície é
> `state("key").update(f => …)` (método sobre o estado), não `state.update(key, fn)` como a
> redacção anterior sugeria — corrigido acima.
>
> **A preferência é do lado da linguagem, não só conveniência** — `state.rs:339-349`:
> *"When updating the state based on its previous value, you should **prefer the function
> form** instead of retrieving the previous value from the context. This allows the compiler
> to resolve the final state efficiently, minimizing the number of layout iterations
> required."* Ou seja, a variante diferida é a **recomendada** pela documentação, o que
> agrava o scope-out face à estimativa "`Set` cobre 80% dos casos".
>
> **Sobre os "80%"**: era número sem medição. Removido — não há contagem de corpus que o
> sustente, e a documentação empurra os utilizadores para a forma que falta. O scope-out
> mantém-se, mas sem a quantificação não fundamentada.

---

## Resultado Esperado

- `01_core/src/entities/state_update.rs` — enum + impl Hash + impl Eq + 3 tests.
- Re-export em `01_core/src/entities/mod.rs`.

---

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-04-30 | P171: enum minimal Set para suportar runtime mutable state | `state_update.rs`, `state_update.md` |
