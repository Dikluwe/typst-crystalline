# Prompt L0 — `wiring/eviction`
Hash do Código: 36fde5f8

**Camada**: L4.
**Fase**: M8 / P204E.
**ADRs vinculantes**: ADR-0073 (comemo Introspector — paridade
vanilla literal); ADR-0066 (Introspection runtime adiada —
ACEITE com nota "intermediário até M8"; superseded em P204H).
**Cross-references**: P204A C6 (política invalidação); P204A
C13.1 (P204E plano de materialização).

---

## Contexto

ADR-0073 PROPOSTO declarou política de invalidação per
P204A C6:

> Per A9: vanilla usa exactamente esta política
> (`comemo::evict(10)` no CLI watch mode). Cristalino implementa
> wrapper `crystalline_evict(n: usize)` em L4 wiring; L4
> opcionalmente integra em watch mode futuro.

P204E materializa este wrapper.

---

## Decisão

Wrapper passthrough em L4 wiring:

```text
pub fn crystalline_evict(max_age: usize) {
    comemo::evict(max_age)
}
```

Sem policy adicional — paridade vanilla literal (que também
chama `comemo::evict(10)` directamente sem wrapper).

`max_age` semantics (per documentação comemo 0.4.0
`cache.rs:21`):
- Remove memoized results cuja age >= `max_age`.
- Age cresce em 1 por eviction; reset a 0 em cache hit.
- `max_age = 0` clears entire cache.

---

## Localização

`04_wiring/src/eviction.rs` (módulo dedicado).

`04_wiring` é binary-only crate (`[[bin]] typst`). Sem
`lib.rs`. Wrapper fica como módulo dentro do binário; mas
declarado `pub fn` para consumo via `eviction::crystalline_evict`
em `main.rs` (caso integração CLI futura).

---

## Restrições absolutas

- L4 (composição pura; sem lógica de negócio).
- Função 1-linha (delegate trivial).
- `comemo` adicionado a `Cargo.toml` `[dependencies]`.
- 1 sentinel test (compilação smoke).

---

## Não-objectivos

- Não integra em CLI / watch mode (futuro pós-M8).
- Não automatiza invalidação (deixa controle a callers).
- Não toca em consumers de Introspector ou Layouter.
- Não wraps outras APIs comemo (apenas `evict`).

---

## Plano de validação

`crystalline_evict` é considerado materializado quando:

1. Função existe em `04_wiring/src/eviction.rs` com
   assinatura `pub fn crystalline_evict(max_age: usize)`.
2. Delega a `comemo::evict(max_age)`.
3. `comemo` está em `04_wiring/Cargo.toml` `[dependencies]`.
4. Workspace compila verde.
5. Tests workspace verdes (1836+).
6. Sentinel test confirma função existe.
7. Crystalline-lint 0 violations.

---

## Cross-references

- P204A C6 (política invalidação tracking-based +
  evict() exposed).
- P204A C13.1 (P204E plano de materialização).
- ADR-0073 PROPOSTO (transita ACEITE em P204H).
- Vanilla: `lab/typst-original/crates/typst-cli/src/watch.rs:81`
  (`comemo::evict(10)` único call site).
- comemo 0.4.0: `~/.cargo/registry/src/index.crates.io-*/comemo-0.4.0/src/cache.rs`
  (pub fn evict).
