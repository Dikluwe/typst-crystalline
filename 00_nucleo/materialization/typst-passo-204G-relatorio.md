# Relatório do passo P204G

**Data de execução**: 2026-05-07.
**Executor**: Claude Code (Opus 4.7).
**Spec**: `00_nucleo/materialization/typst-passo-204G.md`.
**Natureza**: implementação de instrumentação (measurements
internos).
**Sub-passo `G` da série M8** — sexto de 7 (B-H) per
ADR-0073.
**Magnitude planeada**: S (com ressalva M).
**Magnitude real**: **S+** (~50 min; 2 ficheiros novos +
3 modificados; 8 tests novos; 1 refactor mid-execution
para resolver V12).

---

## §1 O que foi feito

P204G materializou measurements internos per ADR-0073 C10:
cache stats (`crystalline_evict` calls + last `max_age`)
e counts de invocação dos 20 métodos do trait
`Introspector` via wrapper newtype `CountingIntrospector`.

Forma dual implementada (per P204A C10):

1. **Logging opt-in** — env var
   `CRYSTALLINE_MEASUREMENTS=1` aciona dump no fim do
   `main()` via `eprintln!`. Default silencioso.
2. **Tests dedicados** — 8 tests em
   `03_infra/src/measurements.rs::tests` (2 sentinelas + 5
   cláusula-C6 + 1 auxiliar).

Caminhos fixados:

- **C2 = B** — counter próprio via `AtomicUsize` global.
  `comemo::testing::last_was_hit` rejeitado por ser
  per-call (não cumulativo) e exigir activação de feature
  `testing` + wrapping em todas as call sites memoized.
- **C3 = a** — wrapper newtype `CountingIntrospector<I>`
  que delega cada método ao `inner` e incrementa counter
  global por método. Aplicado em test fixtures (não invade
  L1 production).

### Output 1 — Inventário interno

Localização:
`00_nucleo/diagnosticos/typst-passo-204G-inventario.md`.

Conteúdo:
- §1-§5 C1 (5 sub-secções: 3 AJUSTES, 2 CONFIRMADOS).
- §6 C2 caminho fixado (B).
- §7 C3 caminho fixado (a).
- §8 C4-C5 alterações literais.
- §9 5 decisões durante a leitura (D1–D5).
- §10 hashes finais.

Tamanho: ~7 KB.

### Output 2 — Relatório (este ficheiro)

### Output 3 — Alterações em código

#### Ficheiros novos

- **`00_nucleo/prompts/infra/measurements.md`** (L0; hash
  `c89617ca`) — spec arquitectural completa: contexto,
  decisão, localização, restrições, não-objectivos, plano
  de validação, cross-references.
- **`03_infra/src/measurements.rs`** (L3; ~410 LOC com
  tests; hash `84928cb2`) — módulo principal:
  - `CacheStats { evict_calls, last_max_age }`.
  - `CallCounts { total, per_method: Vec<(&str, usize)> }`
    com helper `count_for`.
  - `pub const INTROSPECTOR_METHODS: [&str; 20]` —
    ordem canónica dos 20 métodos.
  - `cache_stats()`, `introspector_call_counts()`,
    `reset()`, `record_evict(max_age)`.
  - `pub struct CountingIntrospector<I>` + impl
    `Introspector` que delega + incrementa counter.

#### Ficheiros modificados

- **`03_infra/src/lib.rs`** — `pub mod measurements;`
  adicionado entre `layout` e `pipeline`.
- **`04_wiring/src/eviction.rs`** — `crystalline_evict`
  chama `typst_infra::measurements::record_evict(max_age)`
  antes de `comemo::evict(max_age)`.
- **`04_wiring/src/main.rs`** — bloco opt-in
  `CRYSTALLINE_MEASUREMENTS=1` no fim de `main()` antes
  de retornar `exit_code` (~17 linhas). Comentário
  explicando que measurements vivem em L3 (V12 OK).
- **`00_nucleo/adr/typst-adr-0073-comemo-introspector.md`**
  — §P204G anotado com `✅ MATERIALIZADO 2026-05-07` +
  sumário literal das decisões fixadas.

---

## §2 Tempo de execução

~50 minutos efectivos:

- ~10 min: C1 inventário empírico (5 sub-secções —
  comemo source, trait Introspector, locais L3/L4, tests
  existentes, logging mechanism).
- ~5 min: C2 + C3 fixação de caminhos.
- ~10 min: C4 implementação inicial em L4 (incluindo L0
  prompt em `wiring/measurements.md`).
- ~5 min: C7 build verde + C8 test workspace verde.
- ~10 min: C9 detecção de V12 → refactor para L3
  (recriação L0 em `infra/measurements.md`, código em
  `03_infra/src/measurements.rs`, eliminação dos
  ficheiros L4 anteriores, ajuste de imports em
  `eviction.rs` e `main.rs`).
- ~5 min: re-build, re-test, re-lint.
- ~5 min: C10 anotação ADR + C13 outputs (inventário e
  este relatório).

---

## §3 Métricas

| Métrica | Valor |
|---------|-------|
| Tests workspace antes | 1844 |
| Tests workspace depois | 1852 (+8) |
| Tests measurements | 8 (2 sentinelas + 6) |
| Linter violations | 0 |
| Linter warnings | 0 |
| Ficheiros novos | 2 (1 L0, 1 L3) |
| Ficheiros modificados | 4 (lib.rs, eviction.rs, main.rs, ADR-0073) |
| LOC novas | ~410 (L3) + ~190 (L0) = ~600 |
| Cargo deps adicionados | 0 |

Distribuição dos tests:

- `typst_core` unit: 1576 (sem alteração).
- `typst_infra` unit: 229 (+8) + 6 ignored (sem alteração).
- `typst_shell` unit: 24 (sem alteração).
- `typst-wiring` binary unit: 2 (sentinelas eviction).
- `typst-wiring` integration `tests/cli.rs`: 21 (sem
  alteração).

Doc-tests: 1 ignored (sem alteração).

Total: 1852 verdes.

---

## §4 Decisões

### D1 — Localização L3 (refactor mid-execution)

Implementação inicial em `04_wiring/src/measurements.rs`
disparou 2 warnings V12 ("Lógica no fio: impl declarado em
L4. L4 não cria tipos") sobre `impl CallCounts` e
`impl<I> CountingIntrospector<I>`. V12 é warning level
(`crystalline.toml` linha 60), exit=0, mas a nota literal
em `04_wiring/src/main.rs:101` ("L4 faz I/O trivial sem
criar tipos (V12 OK) — composição pura") indica que a
disciplina é tomada a sério. Refactor para L3 elimina os
2 warnings sem custo arquitectural — L3 já é a localização
canónica para infraestrutura I/O-adjacente
(`export`, `pipeline`, `world`).

### D2 — `INTROSPECTOR_METHODS` como `pub const [&str; 20]`

Ordem canónica fixada num array constante; index na
constante = índice em `CALL_COUNTERS`. Vantagem: snapshot
estável em logging, tests podem indexar por nome via
`count_for`, e adição/remoção futura de métodos é mecânica
(actualizar array + counter ao mesmo tempo).

### D3 — `Ordering::Relaxed` em todos os atomics

Counters são *observação*, não controle de fluxo entre
threads. Relaxed é suficiente e evita custo de fences.

### D4 — `TEST_LOCK: Mutex<()>` em test module

Cargo corre tests em paralelo por defeito; state global
partilhado pode contaminar tests entre si. Mutex
serializa testes que tocam o state. Não-tests não são
afectados.

### D5 — Reset separado de evict

`reset()` zera measurement counters; `crystalline_evict(0)`
zera comemo cache. Vivem em paralelo. Test 5 de C6
codifica esta decisão como sentinel: chamar evict não zera
counters de instrumentação.

---

## §5 Hipóteses de obstrução listadas no spec

Confrontadas com a realidade:

| Hipótese | Resultado |
|----------|-----------|
| `comemo::testing` não exposto sem feature | **CONFIRMADO** — está atrás de `feature = "testing"`, default vazio. Caminho B adoptado. |
| Wrapping `Introspector` não-trivial devido a `Tracked` | **NÃO MATERIALIZOU** — wrapping pré-track é trivial (newtype + delegate). 20 métodos delegados manualmente em ~95 LOC. |
| Feature flag `comemo` precisa adicionar em `Cargo.toml` (C2 = A) | N/A — caminho A não escolhido. |
| `AtomicUsize` precisa import (C2 = B) | **CONFIRMADO** — import trivial `std::sync::atomic`. |
| `Tracked` opacity bloqueia wrapping (C3 = a) | **NÃO MATERIALIZOU** — wrapping não interage com `Tracked` (acontece pré-track no caller). |
| Regra L0 sobre módulo novo | **CONFIRMADO** — L0 prompt em `infra/measurements.md` criado per Protocolo de Nucleação (CLAUDE.md §). Hash sincronizado via `--fix-hashes`. |
| V12/V13 violations | **CONFIRMADO** (V12 mid-execution); resolvido via D1. |

Adicional **não previsto**: V12 forçou refactor mid-execution
de L4 → L3. Custo ~10 min; sem impacto arquitectural
negativo (L3 é localização mais natural).

---

## §6 Sugestão para próximo sub-passo (P204H)

P204G fechado per C13 com todos os critérios cumpridos:

- ✓ C1 inventário completo.
- ✓ C2 + C3 caminhos fixados (B + a).
- ✓ C4 módulo de measurements criado em L3.
- ✓ C5 logging opt-in aplicado em main.rs.
- ✓ C6 tests dedicados (8 tests; 5 cláusula-C6 +
  2 sentinelas + 1 auxiliar).
- ✓ C7 compilação verde.
- ✓ C8 tests workspace verdes (1852).
- ✓ C9 linter 0 violations.
- ✓ C10 ADR-0073 anotada.
- ✓ C11 L0 prompt criado em `infra/measurements.md`.
- ✓ C12 sentinelas (`p204g_cache_stats_existe`,
  `p204g_introspector_call_counts_existe`).
- ✓ Inventário registado em `diagnosticos/`.
- ✓ Relatório escrito (este ficheiro).

P204H pode prosseguir com:

- Relatório consolidado da série P204 (B-G).
- Transitar ADR-0073 PROPOSTO → ACEITE.
- Anotar ADR-0066 secção "validação empírica" com registo
  de que M8 fechou.
- Actualizar `00_nucleo/projecto/blueprint-projecto.md`
  com M8 fechado.
- Considerar superseded para ADR-0066 (Introspection
  runtime adiada) — ACEITE com nota "intermediário até
  M8" cuja condição agora é satisfeita.

---

## §7 Cross-references

- ADR-0073 PROPOSTO §P204G plano + §P204G ✅ MATERIALIZADO.
- P204A C10 (estimativa measurements internos).
- P204A C13.1 (P204G no plano global da série).
- P204F (predecessor — corpus paridade fechado).
- L0 prompt: `00_nucleo/prompts/infra/measurements.md`
  (hash `c89617ca`).
- Inventário: `00_nucleo/diagnosticos/typst-passo-204G-inventario.md`.
- Spec original: `00_nucleo/materialization/typst-passo-204G.md`.
- Vanilla: `lab/typst-original/crates/typst-cli/src/watch.rs:81`
  (`comemo::evict(10)` único call site — paralelo a
  `crystalline_evict`).
- comemo 0.4.0:
  `~/.cargo/registry/src/index.crates.io-*/comemo-0.4.0/src/cache.rs:96-99`
  (testing API per-call atrás de feature flag).
- V12 disciplina L4: `04_wiring/src/main.rs:101` (nota
  "L4 faz I/O trivial sem criar tipos — composição pura").
