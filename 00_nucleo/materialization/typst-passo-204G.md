# Passo 204G — Measurements internos (comemo + counts de invocação Introspector)

**Série**: 204 (sub-passo `G` = measurements após P204F
corpus paridade).
**Tipo**: implementação de instrumentação.
**Magnitude planeada**: S (com ressalva: pode subir a M
se counts de invocação exigirem wrapping não trivial).
**Pré-condição**: P204F concluído; corpus paridade tem
6 ficheiros introspection; tests 1844 verdes; 0 violations;
15 sentinelas activas; ADR-0073 PROPOSTO em vigor (6/7
sub-passos materializados).
**Output**: 3 ficheiros (inventário + relatório +
alterações de código).

---

## §1 Propósito

Adicionar instrumentação leve que mede:

- Cache hits/misses do `comemo` durante compilação típica.
- Counts de invocação de métodos do trait `Introspector`.

Forma dual (decisão da clarificação inicial):

- **Logging** — opt-in via env var ou debug build; uso
  manual para inspecção.
- **Tests dedicados** — asserts sobre ratios; detecção
  automática de regressão.

P204G **não inclui benchmarks comparativos com vanilla**
(per C10 do diagnóstico P204A — measurements internos
sem comparação vanilla absoluta).

P204G respeita a convenção: começa com inventário
empírico antes de qualquer alteração.

---

## §2 Material de partida verificado em P204F

Antes de qualquer alteração, confirmar empíricamente:

- 6 ficheiros corpus introspection em
  `lab/parity/corpus/visual/` (per P204F).
- Helper `compile_to_pdf` em
  `03_infra/src/integration_tests.rs`.
- Trait `Introspector` com 20 métodos read-only em
  `01_core/src/entities/introspector.rs`.
- `comemo` 0.4.0 com `#[comemo::track]` aplicado ao
  trait.
- `crystalline_evict` wrapper em `04_wiring`.

Sem isto, recuar para P204F.

---

## §3 Cláusulas de execução (sem condicionais)

### C1 — Inventário empírico inicial

Antes de tocar em código, listar literalmente:

1. **API de measurements do `comemo`** — confirmar:
   - Existe `comemo::testing::last_was_hit` (per
     P204A C10)?
   - Existem outros hooks (`comemo::stats`, feature
     flag `track-counts`, etc.)?
   - Que feature flags do crate estão activas em
     `Cargo.toml` workspace?
2. **Pattern para counts de invocação Introspector** —
   confirmar:
   - Wrapping `dyn Introspector` é viável (manual impl
     que delega + incrementa counter)?
   - `comemo::track` cria o `Tracked<dyn Introspector>`
     opaco — wrapping pode ter de ser feito **antes** do
     `.track()` call.
   - Existe pattern noutro crate cristalino para
     instrumentação semelhante?
3. **Localização canónica para measurement infra** —
   decidir:
   - `01_core/src/diagnostics/` (módulo dedicado).
   - `04_wiring/src/measurements.rs` (junto ao evict
     wrapper).
   - `03_infra/src/measurements.rs` (lab/test infra).
   - Outro caminho se inventário sugerir.
4. **Tests existentes que tocam comemo** — confirmar:
   - Padrão de helper para reset de cache antes de
     test.
   - Se `evict(0)` é suficiente para isolamento.
5. **Logging mechanism** — confirmar:
   - `tracing` ou `log` crate em uso?
   - Convenção env var (ex: `RUST_LOG`,
     `CRYSTALLINE_DEBUG`)?
   - Se cristalino tem mecanismo proprietário ou
     herdou de typst.

Output: 5 sub-secções com etiqueta CONFIRMADO ou
**AJUSTE NECESSÁRIO**.

Se C1.1 revelar que `comemo::testing` não está exposto
(feature flag desactivada ou crate sem expor) ou C1.2
revelar que wrapping é não-trivial, registar `P204G.div-N`
e re-fixar C2.

### C2 — Decisão sobre forma de measurements

Com base em C1.1, fixar mecanismo para hits/misses:

- **Caminho A — `comemo::testing` directo** — se feature
  flag estiver disponível e API for estável.
- **Caminho B — wrapper próprio** — counter manual via
  `AtomicUsize` em singleton; útil mesmo sem
  `comemo::testing`.
- **Caminho C — degradado** — só counts de invocação
  (via wrapping `Introspector`); hits/misses do comemo
  ficam para sub-passo dedicado pós-M8 se `comemo::testing`
  não disponível.

Critério: se A viável, escolher A. Caso contrário, B se
infraestrutura simples, C se B exigir trabalho
desproporcional.

C2 fixa **uma** alternativa.

### C3 — Decisão sobre forma de counts de invocação

Com base em C1.2, fixar mecanismo:

- **Caminho a — wrapper newtype** — `struct
  CountingIntrospector<I: Introspector>` que delega +
  incrementa counter. Aplicado em test fixtures.
- **Caminho b — `thread_local` counter no impl
  `TagIntrospector`** — incrementa counter em cada
  método. Invasivo no production code.
- **Caminho c — proc-macro derive** — gera wrapper
  automático. Magnitude L. Rejeitado por inflação.

Critério: caminho a é preferido (não invade L1
production). Caminho b é fallback se wrapping não for
viável devido a `Tracked`.

Hipótese de obstrução: `Tracked<dyn Introspector>`
pode não permitir wrapping pré-track sem trabalho não
trivial. A confirmar em C1.2.

C3 fixa **uma** alternativa.

### C4 — Estrutura do módulo de measurements

Com base em C1.3, criar:

- 1 ficheiro novo no caminho fixado (ex:
  `01_core/src/diagnostics/measurements.rs` ou
  similar).
- API pública:
  - `pub fn cache_stats() -> CacheStats { hits, misses }`.
  - `pub fn introspector_call_counts() -> CallCounts
    { ... }` — campos por método trackado.
  - `pub fn reset()` — limpa contadores (chamado por
    tests).
- `CacheStats` e `CallCounts` são tipos públicos com
  Debug/Clone.

Edição literal segue forma fixada em C2 + C3.

### C5 — Logging opt-in

Mecanismo: env var (per C1.5 — fixar nome em P204G).
Convenção provável: `CRYSTALLINE_MEASUREMENTS=1`.

Quando activado:

- No fim do `pub fn layout` (ou ponto de saída do
  pipeline), imprimir `cache_stats()` e
  `introspector_call_counts()` para `stderr` ou
  `tracing::info!`.

Edição: 5–10 linhas em `04_wiring/src/main.rs` ou
caminho real do entry point.

Cuidado: logging **não muda** valores em testes (env
var não setada por defeito).

### C6 — Tests dedicados

Adicionar 3–5 tests em `03_infra/tests/measurements.rs`
(ou caminho consistente com existing tests):

- **Test 1** — Smoke: compilar `outline-toc.typ` (P204F
  corpus); confirmar que `cache_stats()` reporta `hits >
  0 || misses > 0` (algum tracking activo).
- **Test 2** — Counts: compilar `figure-ref.typ`;
  confirmar que método `figure_number_for_label` foi
  invocado pelo menos 3 vezes (3 figures).
- **Test 3** — Reset: chamar `reset()`; confirmar que
  contadores ficam a zero.
- **Test 4 (opcional)** — Regressão: compilar mesmo
  documento duas vezes consecutivas; confirmar que
  segunda vez tem mais hits que a primeira (cache em
  acção).
- **Test 5 (opcional)** — `crystalline_evict(0)` repõe
  contadores ou não? (Decisão de design.)

Critério: 3–5 tests verdes; 1844 → 1847+.

### C7 — Compilação

```
cargo build --workspace 2>&1 | tail -10
```

Critério: verde. Hipóteses prováveis de erro:

- Feature flag `comemo` precisa de adição em Cargo.toml
  (se C2 = A).
- `AtomicUsize` precisa import (se C2 = B).
- `Tracked` opacity bloqueia wrapping (se C3 = a).

### C8 — Tests workspace

```
cargo test --workspace 2>&1 | tail -10
```

Critério: 1844+ tests verdes (com 3–5 novos de C6).

### C9 — Linter

```
crystalline-lint .
```

Critério: 0 violations.

Hipóteses prováveis:

- Regra L0 sobre módulo novo (precisa prompt L0?).
- Regra de visibilidade.
- `--fix-hashes` aplicado se necessário.

### C10 — Documentação ADR-0073

ADR-0073 mantém PROPOSTO. Anotação cirúrgica em §P204G
com `✅ MATERIALIZADO` + sumário (1–2 linhas).

### C11 — L0 prompt (se C1.3 escolher caminho que exija)

Se módulo novo for criado em L1 (ex:
`01_core/src/diagnostics/`), L0 prompt é pré-requisito
(per CLAUDE.md Protocolo de Nucleação, lição de P204D
e P204E).

Se módulo for criado em L4 ou L3, verificar convenção
do crate.

C11 cria L0 prompt se necessário, com `--fix-hashes`
para sincronização.

### C12 — Sentinelas

Adicionar 1–2 sentinelas:

- `p204g_cache_stats_existe` — falha de compilação se
  função for removida.
- `p204g_introspector_call_counts_existe` — idem.

Decisão dentro de P204G.

### C13 — Critério de fecho de P204G

P204G concluído quando:

- C1 inventário completo.
- C2 + C3 caminhos fixados.
- C4 módulo de measurements criado.
- C5 logging opt-in aplicado.
- C6 tests dedicados (3 mínimo, 5 recomendado).
- C7 compilação verde.
- C8 tests workspace verdes.
- C9 linter 0 violations.
- C10 ADR-0073 anotada.
- C11 L0 prompt criado se necessário.
- C12 sentinelas (mínimo 1).
- Inventário registado.
- Relatório escrito.

### C14 — Sem cláusulas condicionais

C1 produz dados. C2 e C3 fixam **uma** alternativa cada.
C4–C12 executam.

Hipóteses de obstrução são listadas, não pré-fixadas
como ramos.

---

## §4 Outputs concretos

### Ficheiro 1 — Inventário interno

Localização:
`00_nucleo/diagnosticos/typst-passo-204G-inventario.md`.

Conteúdo:
- §1 C1 — inventário (5 sub-secções).
- §2 C2 — caminho hits/misses fixado.
- §3 C3 — caminho counts fixado.
- §4 C4–C5 — alterações literais.
- §5 Decisões durante a leitura.

### Ficheiro 2 — Relatório

Localização:
`00_nucleo/materialization/typst-passo-204G-relatorio.md`.

Conteúdo:
- O que foi feito.
- Tempo de execução.
- Métricas.
- Decisões.
- Sugestão para próximo sub-passo (P204H).

### Ficheiro 3 — Alterações em código

Não é ficheiro discreto. Conjunto de:

- 1 ficheiro novo (módulo de measurements).
- Possível L0 prompt.
- Edição em `Cargo.toml` (feature flag se C2 = A).
- 5–10 linhas em entry point para logging opt-in.
- 3–5 tests em `03_infra/tests/measurements.rs`.
- 1–2 sentinelas.
- Anotação cirúrgica em ADR-0073.

---

## §5 Critério de progressão para P204H

P204G fechado quando C13 cumprido.

Em caso de divergência empírica relevante (ex:
`comemo::testing` indisponível, wrapping `Introspector`
bloqueado por `Tracked`), registar em `P204G.div-N` e:

- Resolver dentro de P204G (preferido — caminhos
  alternativos B/C).
- Recuar para P204A re-fixar C10 se obstrução for
  estrutural.

P204H só começa quando P204G fechado.

---

## §6 Convenções mantidas

- Sem condicionais estruturais.
- 3 outputs.
- Inventário empírico antes de implementação.
- Localização canónica:
  `00_nucleo/diagnosticos/` para inventário;
  `00_nucleo/materialization/` para relatório.
- Sem inflação retórica.

---

## §7 Não-objectivos

P204G não:

- Adiciona benchmarks comparativos com vanilla (per
  C10 do diagnóstico P204A; sub-passo dedicado pós-M8
  se for relevante).
- Transita ADR-0073 para ACEITE (P204H).
- Transita ADR-0066 para superseded (P204H).
- Cria ADR nova.
- Modifica trait `Introspector` ou impl
  `TagIntrospector` (a não ser por delegação via wrapper
  newtype em test fixtures, se C3 = a).
- Modifica Layouter ou consumers.
- Toca em loops fixpoint.
- Expande corpus paridade (P204F já fechou).
- Adiciona `crystalline_evict` em CLI (pós-M8).
- Implementa hits/misses por sub-store individual
  (granularidade per-method via comemo é suficiente).

---

## §8 Erro a não repetir

P204A C10 estimou measurements internos S (logging +
measurements de regressão). P204G materializa essa
estimativa.

Risco específico identificado: `comemo::testing` pode
não estar exposto na versão 0.4.0 sem feature flag
específica (cliente normalmente desactiva para reduzir
binary size). Caso confirmado em C1.1, P204G adopta
Caminho B (counter próprio via AtomicUsize).

Hipótese específica: counts de invocação via wrapper
newtype pode bloquear porque `comemo::track` exige
trait bounds que tornam o wrapping não-trivial.

P204D detectou tensão estrutural similar em C6 (3
alternativas listadas). P204G aplica padrão idêntico
em C2 e C3.

---

## §9 Particularidade — execução

P204G é trabalho de instrumentação leve:

- 1 ficheiro novo (~50–100 LOC).
- Possível L0 prompt (~50 LOC).
- Edição em entry point (~10 LOC).
- 3–5 tests (~50–80 LOC).

Volume baixo a médio. Magnitude S–M.

Recomendado Claude Code dado:

- Investigação de `comemo::testing` API exige leitura
  do crate fonte.
- Iteração rápida com cargo test para confirmar
  measurement values.
- Hipótese de obstrução em C3 que pode exigir caminhos
  alternativos.

Sessão actual viável se C1 não revelar obstrução
estrutural.
