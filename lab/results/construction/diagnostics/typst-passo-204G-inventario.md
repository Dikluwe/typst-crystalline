# P204G — Inventário empírico

**Data**: 2026-05-07.
**Cláusula**: C1 do passo P204G.
**Pré-condição confirmada**: P204F fechado (corpus paridade
6 ficheiros introspection; tests 1844 verdes; 0 violations;
ADR-0073 PROPOSTO).

---

## §1 C1.1 — API de measurements do `comemo`

**Resultado**: AJUSTE NECESSÁRIO.

Inspecção empírica de
`~/.cargo/registry/src/index.crates.io-*/comemo-0.4.0/`:

- `comemo::testing::last_was_hit` **não existe** como
  módulo. O símbolo está em
  `comemo::internal::last_was_hit` e atrás de `feature =
  "testing"` (`Cargo.toml` `[features] testing = []`,
  default vazio).
- A semântica é per-call (saber se a *última* invocação foi
  hit), via `thread_local! LAST_WAS_HIT: Cell<bool>`. Não
  cumulativo.
- Não existe `comemo::stats` nem feature
  `track-counts` no crate.
- Workspace `Cargo.toml` não activa nenhuma feature
  `comemo` (default vazio); `01_core`, `03_infra`,
  `04_wiring` apenas declaram `comemo = { workspace = true }`.

**Implicação**: para stats cumulativos sem patchear comemo
nem activar feature de uso restrito, é necessário contador
próprio em call sites controlados pelo cristalino.

---

## §2 C1.2 — Pattern para counts de invocação Introspector

**Resultado**: CONFIRMADO (com ressalva).

- Trait `Introspector` em `01_core/src/entities/introspector.rs:40`
  tem `#[comemo::track]` aplicado, com 20 métodos read-only.
- Wrapping `dyn Introspector` é viável via `struct
  CountingIntrospector<I: Introspector>` que delega cada
  método e incrementa um counter. Pré-track: o wrapper é
  instanciado pelo caller e depois `.track()` é aplicado.
- `comemo::Track` é implementado para `dyn Introspector +
  '__comemo_dynamic` (gerado pelo macro). O wrapper
  participa como `&dyn Introspector` ou `Tracked<...>`
  conforme caller.
- Ressalva: o wrapping não conta invocações **dentro** de
  funções memoized; conta invocações feitas pelo caller
  externo. Para test fixtures, isto é suficiente.
- Não existe outro pattern em cristalino para
  instrumentação semelhante. `crystalline_evict` (L4) é
  delegate puro sem counter.

---

## §3 C1.3 — Localização canónica para measurement infra

**Resultado**: AJUSTE NECESSÁRIO (versão 2 após confronto
com V12).

Decisão inicial **rejeitada**: `04_wiring/src/measurements.rs`
(análogo a `eviction.rs`). Falhou validação V12 ("L4 não
cria tipos") porque `CallCounts`, `CountingIntrospector`,
e impl `Introspector for CountingIntrospector` constituem
criação de tipos em L4 — proibido per disciplina explícita
em `04_wiring/src/main.rs:101` ("L4 faz I/O trivial sem
criar tipos — composição pura").

Decisão final **fixada**: `03_infra/src/measurements.rs`
(L3). Localização canónica para infraestrutura
I/O-adjacente (módulos `export`, `pipeline`, `world`). L3
permite `static AtomicUsize` global sem disparar V13
(restrito a L1).

Topologia: L3 → L1 (importa `Introspector` trait do core)
permitida.

---

## §4 C1.4 — Tests existentes que tocam comemo

**Resultado**: CONFIRMADO.

- `04_wiring/src/eviction.rs::tests` chamam
  `crystalline_evict(0)` para reset cache (sentinel
  P204E).
- Não há helper centralizado para reset de cache antes de
  test em workspace.
- `evict(0)` é suficiente para isolamento — clears entire
  cache (per `comemo/src/cache.rs:81` doc).
- P204G adiciona `typst_infra::measurements::reset()` para
  isolamento entre tests do próprio módulo measurements
  (separado de `comemo::evict` — measurement counters
  vivem em paralelo ao cache do comemo).

---

## §5 C1.5 — Logging mechanism

**Resultado**: AJUSTE NECESSÁRIO.

- `tracing` e `log` crates **não** estão em
  `[workspace.dependencies]`. Cristalino usa apenas
  `eprintln!` para diagnostics (cf. `04_wiring/src/main.rs`
  `drain_to_stderr`).
- Não há convenção de env var herdada de typst (vanilla
  usa `TYPST_FONT_PATHS`, `TYPST_ROOT` para CLI; nada
  para measurements).
- Convenção fixada para P204G: `CRYSTALLINE_MEASUREMENTS=1`
  via `std::env::var` no `main.rs`, dump via `eprintln!`
  no fim do pipeline.
- Default silencioso (env var não setada). Não muda
  valores em tests.

---

## §6 C2 — Caminho hits/misses fixado

**Caminho B** (counter próprio via `AtomicUsize`).

Justificação:

1. C1.1 mostrou que `comemo::testing::last_was_hit` é
   per-call (não cumulativo) e exigiria activar feature
   `testing` no workspace + wrapping em todas as call
   sites memoized. Magnitude desproporcional para measure
   leve.
2. Caminho B é simples (2 atomics globais para
   `evict_calls` e `last_max_age`) e suficiente para
   sinalizar regressão via diff entre baseline e nova
   versão.
3. Caminho C (degradado: só counts de invocação) seria
   inferior porque ignoraria a sequência de evictions —
   sinal útil para watch mode futuro pós-M8.

---

## §7 C3 — Caminho counts de invocação fixado

**Caminho a** (wrapper newtype `CountingIntrospector<I>`).

Justificação:

1. C1.2 confirmou que wrapping é viável; trait tem 20
   métodos read-only que delegam trivialmente.
2. Não invade L1 production — `TagIntrospector` impl
   permanece intacto.
3. Caminho b (thread_local counter no impl
   `TagIntrospector`) seria invasivo em L1 e violaria V13
   (estado mutável em L1).
4. Caminho c (proc-macro derive) magnitude L; rejeitado
   por inflação (per próprio passo).

---

## §8 C4–C5 — Alterações literais

### Ficheiros novos

- `00_nucleo/prompts/infra/measurements.md` (L0; hash
  `c89617ca`).
- `03_infra/src/measurements.rs` (L3; ~410 LOC com tests;
  hash `84928cb2`).

### Ficheiros modificados

- `03_infra/src/lib.rs` — `pub mod measurements;`
  adicionado entre `layout` e `pipeline`.
- `04_wiring/src/eviction.rs` — `crystalline_evict` chama
  `typst_infra::measurements::record_evict(max_age)` antes
  de `comemo::evict(max_age)`.
- `04_wiring/src/main.rs` — bloco opt-in
  `CRYSTALLINE_MEASUREMENTS=1` no fim de `main()` antes
  de retornar `exit_code`. Comentário explicando que
  measurements vivem em L3 (V12 OK).
- `00_nucleo/adr/typst-adr-0073-comemo-introspector.md` —
  §P204G anotado com `✅ MATERIALIZADO 2026-05-07` +
  sumário literal das decisões fixadas.

---

## §9 Decisões durante a leitura

### D1 — `INTROSPECTOR_METHODS` constante pública

A ordem dos 20 métodos no array `CALL_COUNTERS` foi fixada
como constante `pub const INTROSPECTOR_METHODS: [&str;
20]`. Permite indexar counters por nome e expõe a ordem
canónica para callers / debug.

### D2 — `CallCounts.per_method: Vec<(&'static str, usize)>`

Snapshot retorna Vec ordenado por índice (não HashMap)
para estabilidade de output em logging e tests. Helper
`count_for(&self, method: &str) -> usize` é O(20) — sem
custo significativo.

### D3 — `Ordering::Relaxed` em todos os atomics

Não há sincronização explícita entre threads de tests; os
counters são *observação*, não controle de fluxo. Relaxed
é suficiente e mais barato.

### D4 — `TEST_LOCK: Mutex<()>` em test module

State global partilhado entre tests. Cargo corre tests em
paralelo por defeito; sem Mutex, o test 1 (smoke) pode
ver counters incrementados pelo test 2 (counts) e
falhar. Solução: cada teste que toca state global lockia
no início.

### D5 — Reset separado de evict

`reset()` é função dedicada do módulo measurements; não
chama `comemo::evict(0)` automaticamente. Decisão de
design: evict cristalino e measurements counters vivem em
paralelo. Caller controla cada um separadamente. Test 5
de C6 codifica esta decisão.

---

## §10 Hash do código

- `00_nucleo/prompts/infra/measurements.md` →
  `Hash do Código: c89617ca`.
- `03_infra/src/measurements.rs` → `@prompt-hash 84928cb2`.

Ambos fixados via `crystalline-lint --fix-hashes .`.
