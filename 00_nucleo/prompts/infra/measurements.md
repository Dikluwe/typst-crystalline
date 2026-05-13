# Prompt L0 — `infra/measurements`
Hash do Código: a354a7b1

**Camada**: L3.
**Fase**: M8 / P204G.
**ADRs vinculantes**: ADR-0073 (comemo Introspector — paridade
vanilla literal); complementa ADR-0066 (Introspection runtime).
**Cross-references**: P204A C10 (measurements internos); P204G
C2/C3 (caminhos fixados); `wiring/eviction.md` (wrapper
paralelo sobre `comemo::evict`).

---

## Contexto

P204A C10 fixou que measurements internos (cache hits/misses
do `comemo` + counts de invocação dos métodos do trait
`Introspector`) entram em sub-passo dedicado pós-paridade
corpus. P204F fechou paridade corpus (6 ficheiros
introspection); P204G materializa a infraestrutura.

Investigação `comemo` 0.4.0 (registry source):

- `comemo::testing::last_was_hit` **não existe**. Símbolo
  está em `comemo::internal::last_was_hit`, atrás de
  `feature = "testing"` (não default), e é per-call (saber
  se a última chamada foi hit), não cumulativo.
- Não existem hooks `comemo::stats` nem feature
  `track-counts`.
- Para stats cumulativos sem patchear comemo, é necessário
  contador próprio em call sites controlados pelo
  cristalino.

Investigação topologia: `static AtomicUsize` em L1 dispara
V13 `MutableStateInCore`. Tipos novos em L4 disparam V12
("L4 não cria tipos — composição pura"; cf. nota literal
em `04_wiring/src/main.rs:101`). Counters globais e tipos
ficam em L3 — L3 já é localização canónica para
infraestrutura I/O-adjacente (cf. módulos `export`,
`pipeline`, `world`).

---

## Decisão

### Forma dual (per P204A C10)

1. **Logging opt-in** — env var `CRYSTALLINE_MEASUREMENTS=1`
   aciona dump de `cache_stats()` + `introspector_call_counts()`
   no fim do `main()` (L4) via `eprintln!`. Default:
   silencioso.
2. **Tests dedicados** — asserts sobre counts; detecção
   automática de regressão.

### Caminho fixado para hits/misses (C2 = B)

Counter próprio via `AtomicUsize` global (L3). Hook
`record_evict(max_age: usize)` chamado por
`crystalline_evict` (L4 wiring) antes de `comemo::evict`.
API exposta: `cache_stats() -> CacheStats` com campos
`evict_calls` e `last_max_age`.

P204G **não** activa feature `comemo::testing` (per-call,
não cumulativo, exige wrapping em todas as call sites
memoized — magnitude desproporcional).

### Caminho fixado para counts de invocação (C3 = a)

Wrapper newtype `CountingIntrospector<I: Introspector>` que
delega cada método ao `inner: I` e incrementa `AtomicUsize`
global por método (26 métodos do trait `Introspector` — 20
originais + `query_labelled` (P207B) + `label_count` (P207C)
+ 4 page-aware `pages`/`page`/`page_numbering`/`page_supplement`
(P207D), todos M9c). Aplicado em test fixtures (não invade L1
production).

API exposta:

- `pub struct CountingIntrospector<I> { inner: I }` com
  `new(inner) -> Self`, `into_inner(self) -> I`,
  `inner(&self) -> &I`.
- `impl<I> Introspector for CountingIntrospector<I>` quando
  `I: Introspector + Send + Sync`.
- `introspector_call_counts() -> CallCounts` com `total` +
  `per_method: Vec<(&'static str, usize)>` ordenado por
  índice fixo. Helper `CallCounts::count_for(&self,
  method: &str) -> usize`.
- `reset()` zera cache stats + call counts.

---

## Localização

`03_infra/src/measurements.rs` (módulo dedicado).

Exposto como `pub mod measurements` em `03_infra/src/lib.rs`.

`crystalline_evict` em `04_wiring/src/eviction.rs`
actualizado para chamar
`typst_infra::measurements::record_evict(max_age)` antes de
`comemo::evict(max_age)`.

`04_wiring/src/main.rs` lê env var e dispara dump opt-in
chamando `typst_infra::measurements::{cache_stats,
introspector_call_counts}`.

---

## Restrições absolutas

- L3 (infraestrutura I/O-adjacente; sem lógica de domínio).
- 26 `AtomicUsize` globais correspondendo aos 26 métodos do
  trait `Introspector` — 20 originais + `query_labelled`
  (P207B) + `label_count` (P207C) + 4 page-aware (`pages`,
  `page`, `page_numbering`, `page_supplement`) (P207D) —
  em ordem fixada na constante `INTROSPECTOR_METHODS`.
- 2 `AtomicUsize` globais para `EVICT_CALLS` e
  `LAST_MAX_AGE`.
- `Ordering::Relaxed` em todos os accesses (não há
  sincronização explícita; observação aproximada é
  aceitável).
- `record_call(idx: usize)` privado ao módulo; invocado
  apenas pelo wrapper.
- Sentinelas: `p204g_cache_stats_existe` e
  `p204g_introspector_call_counts_existe` (compile-time
  smoke).
- Sem dependências adicionais em `Cargo.toml`.
- Trait `Introspector` importado de
  `typst_core::entities::introspector` (L3 → L1 é permitido
  por topologia).

---

## Não-objectivos

- Não activa feature `comemo::testing` (caminho rejeitado
  por desproporção).
- Não wrappa `Introspector` em production code (apenas
  wrapper newtype usado em test fixtures).
- Não modifica trait `Introspector` ou impl
  `TagIntrospector`.
- Não modifica Layouter ou consumers.
- Não toca em loops fixpoint.
- Não adiciona `crystalline_evict` em CLI (pós-M8).
- Não implementa hits/misses por sub-store individual
  (granularidade per-method via comemo é suficiente; P204G
  é leve).
- Não adiciona benchmarks comparativos com vanilla (per
  C10 do diagnóstico P204A; sub-passo dedicado pós-M8 se
  for relevante).

---

## Plano de validação

`measurements` é considerado materializado quando:

1. Módulo existe em `03_infra/src/measurements.rs` com
   API: `cache_stats`, `introspector_call_counts`, `reset`,
   `record_evict`, `CountingIntrospector`, `CacheStats`,
   `CallCounts`.
2. Exportado em `03_infra/src/lib.rs` via `pub mod
   measurements`.
3. `crystalline_evict` em `04_wiring/src/eviction.rs` chama
   `typst_infra::measurements::record_evict(max_age)` antes
   de `comemo::evict(max_age)`.
4. `main.rs` lê `CRYSTALLINE_MEASUREMENTS` e dump opt-in
   no fim do pipeline.
5. Workspace compila verde.
6. Tests workspace 1844+ verdes (com 3+ novos de C6).
7. Sentinelas confirmam `cache_stats` e
   `introspector_call_counts` existem.
8. Crystalline-lint 0 violations.

---

## Cross-references

- P204A C10 (measurements internos sem comparação vanilla
  absoluta).
- P204G C2 + C3 (caminhos fixados B + a).
- ADR-0073 PROPOSTO (transita ACEITE em P204H).
- `wiring/eviction.md` (wrapper paralelo sobre `comemo::evict`).
- comemo 0.4.0:
  `~/.cargo/registry/src/index.crates.io-*/comemo-0.4.0/src/cache.rs`
  (linha 96–99: `last_was_hit` per-call atrás de
  `feature = "testing"`).
- Trait `Introspector`:
  `01_core/src/entities/introspector.rs:40` (`#[comemo::track]`
  com 26 métodos — 20 originais + `query_labelled` (P207B) +
  `label_count` (P207C) + 4 page-aware (P207D), todos M9c).
- V12 disciplina L4: `04_wiring/src/main.rs:101` (nota
  "L4 faz I/O trivial sem criar tipos — composição pura").
