# Prompt L0 — `rules/introspect/convergence`
Hash do Código: b0ab02cc

**Camada**: L1
**Ficheiro alvo**: `01_core/src/rules/introspect/convergence.rs`
**Criado em**: 2026-04-29 (P174 sub-passo .C — mecanismo HASH_TAGS para fixpoint)
**ADRs relevantes**: ADR-0066 (Introspection runtime)

---

## Contexto

`compute_tags_hash(&[Tag]) -> u64` é o helper de detecção de convergência usado por `run_fixpoint` (P174 sub-passo .D). Compara duas iterações de fixpoint via hash da sequência de tags emitidas pelo walk.

Vanilla equivalente: convergência via `comemo::analyze::analyze` + comparison estrutural sob memoization. Cristalino simplifica: hash directo sobre `Vec<Tag>` que é determinístico (P163 invariante).

---

## Restrições Estruturais

- Camada **L1**: função pura, sem I/O.
- Tipo de retorno `u64` — `DefaultHasher` produz `u64`.
- Determinístico: para o mesmo input produz mesmo hash.

---

## Interface pública

```rust
use crate::entities::tag::Tag;

pub fn compute_tags_hash(tags: &[Tag]) -> u64;
```

---

## Semântica

- `compute_tags_hash(&[]) == compute_tags_hash(&[])` (consistência).
- `compute_tags_hash(&t1) == compute_tags_hash(&t2)` quando `t1 == t2` (estrutural).
- `compute_tags_hash(&t1) != compute_tags_hash(&t2)` quando `t1 != t2` (com probabilidade muito alta — colisão SipHash teórica).

`Tag` deriva `Hash` (P162) — implementação delega para `Hash::hash_slice` sobre `&[Tag]`.

---

## Algoritmo

```rust
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn compute_tags_hash(tags: &[Tag]) -> u64 {
    let mut hasher = DefaultHasher::new();
    tags.hash(&mut hasher);
    hasher.finish()
}
```

`Vec<Tag>` (e `&[Tag]`) implementa `Hash` automaticamente quando `Tag: Hash`. `DefaultHasher` é SipHash-1-3 — qualidade suficiente para detecção de mudança estrutural (não criptográfico).

---

## Tests obrigatórios

- `compute_tags_hash(&[])` consistente entre chamadas.
- Tags idênticas produzem mesmo hash.
- Tags com payload diferente produzem hash diferente.
- Tags com locations diferentes produzem hash diferente.
- Ordem das tags afecta hash (verificação de bracketing-sensitivity).

---

## Consumers

- `rules/introspect/fixpoint.rs::run_fixpoint` (P174 sub-passo .D).

## Sobre paridade

Vanilla detecta convergência via comemo memoization automática. Cristalino simplifica: hash explícito sobre `Vec<Tag>`. Custo: 1 pass O(n) por iteração. Não memoizado (M7+ pode adicionar).

---

## Resultado Esperado

- `01_core/src/rules/introspect/convergence.rs` — função + 5 tests.
- Re-export em `01_core/src/rules/introspect.rs` (`pub mod convergence`).

---

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-04-29 | P174 sub-passo .C: helper HASH_TAGS para detecção de convergência | `convergence.rs`, `convergence.md` |
