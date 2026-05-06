# ⚖️ ADR-0073: Adopção de `#[comemo::track]` no trait `Introspector` (M8)

**Status**: `PROPOSTO`
**Validado**: pendente — vinculativo após materialização
P204B–G; transita ACEITE em P204H.
**Data**: 2026-05-06
**Sub-passo**: P204A (PROPOSTO).
**Diagnóstico prévio**:
- `00_nucleo/diagnosticos/typst-passo-204A-auditoria-comemo.md` (P204A).
- `00_nucleo/diagnosticos/typst-passo-204A-diagnostico.md` (P204A).

---

## Contexto

M5+M6+M7+M9 estruturalmente fechados em P200B (M5
universal), P190I (M6), P192B (M7), P182F (M9 11/11).

Pré-condição arquitectónica para M8 cumprida em P203
consolidado §13:
- Zero lacunas residuais formalmente catalogadas.
- Baseline empírico reconciliado (snapshot 2026-05-05).
- 1824 tests workspace; 0 violations.
- 70 ADRs (-1 slot 0063).

ADR-0066 (ACEITE com nota "intermediário até M8") declarou
que hash-based convergence é decisão **intermédia**;
paridade arquitectural com vanilla typst exige adopção de
`comemo::Track` para:
- Memoização cross-iteration de queries de introspection.
- Tracking granular de dependências.
- Performance comparável a vanilla.

P204A auditou empíricamente (16 cláusulas A1–A16) o estado
pré-M8 e confirmou:
- comemo 0.4.0 declarado em workspace; já usado em
  World/Engine/eval (3 `#[comemo::track]` em
  `world_types.rs`).
- comemo 0.4.0 suporta `#[track]` em traits não-genéricos
  (per `comemo-macros-0.4.0/src/track.rs:30-43`).
- Trait `Introspector` cristalino: 20 métodos, todos
  `&self` read-only; restrições compatíveis.
- Vanilla `Introspector` usa exactamente
  `#[comemo::track] pub trait Introspector: Send + Sync`
  (per `lab/typst-original/.../introspector.rs:28`) —
  paridade literal disponível.
- Loops fixpoint cristalinos ortogonais a comemo
  (mecanismos de convergência paralelos, não conflituosos).

---

## Decisão

Cristalino adopta **`#[comemo::track]` directamente no
trait `Introspector`**, com paridade literal ao padrão
vanilla.

### Mecanismo

```text
+ #[comemo::track]
  pub trait Introspector: Send + Sync {
      // 20 métodos existentes (assinaturas inalteradas).
  }
```

### Consumers (Layouter)

```text
- pub struct Layouter<M: FontMetrics, S: ImageSizer = NullImageSizer>
+ pub struct Layouter<'a, M: FontMetrics, S: ImageSizer = NullImageSizer>
```

```text
- pub(super) introspector: TagIntrospector,
+ pub(super) introspector: comemo::Tracked<'a, dyn Introspector + 'a>,
```

API pública mantém retrocompatibilidade via wrapper (constrói
`Tracked` internamente).

### Position concrete

Sub-passo M8 (P204D) materializa Position:

```text
+ pub struct Position {
+     pub page: NonZeroUsize,
+     pub point: Point,
+ }
```

Substitui stub `position_of() -> Option<()>` por
`position_of() -> Option<Position>`. Layouter feedback
single-pass popula `runtime.positions: HashMap<Location,
Position>` durante layout (per A14 categoria A — `runtime`
field é eligível).

### Política de invalidação

Tracking-based intra-compilation (gere automaticamente).
`crystalline_evict(n)` wrapper sobre `comemo::evict(n)`
exposed em L4 wiring para watch mode futuro (paridade
vanilla `comemo::evict(10)` em CLI).

### Loops fixpoint

**Mantidos** sem alteração. Hash-based convergence cristalina
(MAX=5 nos 2 loops) e vanilla MAX_ITERS=5 são paralelos
arquitecturais; comemo adiciona granularidade dentro de
cada iteração.

---

## Alternativas consideradas

### Alternativa B — Padrão B3 (ADR-0005): trait plain + TrackedIntrospector separado

**Rejeitada**. Acrescenta indirecção sem benefício.
Padrão B3 é útil quando trait tem métodos não-trackable;
não é o caso (todos `&self`).

### Alternativa C — Funções livres memoizadas

**Rejeitada**. Quebra OOP-ish API e diverge do vanilla.

### Alternativa D — Sub-trait dedicada para subset trackable

**Rejeitada**. Só faz sentido se houver `&mut self` em
métodos de introspecção; A2 confirmou que não.

### Alternativa E — Adiar M8 indefinidamente

**Rejeitada**. ADR-0066 (ACEITE com nota "intermediário
até M8") explicitamente compromete-se a M8. Pré-condição
arquitectónica cumprida. Adiar é inflacionar dívida
técnica.

---

## Consequências

### Positivas

- **Paridade vanilla literal** — mesmo padrão `#[comemo::track]
  pub trait Introspector: Send + Sync`.
- **Granularidade de invalidação** per-method (per-key
  effectiva via constraint tracking).
- **Memoização cross-iteration** — queries repetidas em
  fixpoint loops re-usam cache.
- **Position concrete** — fecha pendência ADR-0066 +
  paridade observable.
- **Fundação para optimizações futuras** — re-walks
  parciais possíveis (não em M8 base; pós-M8).

### Negativas

- **Layouter ganha lifetime parameter** — propagação
  cross-modular; ~10 call sites.
- **Complexidade de tipos** ligeira — `Tracked<'a, dyn
  Introspector + 'a>` substitui `&dyn Introspector`.
- **Restrições comemo** aplicáveis: trait não-genérico
  (já satisfeito), métodos não-genéricos (já satisfeito),
  args `ToOwned` (todos satisfazem), returns `Hash`
  (todos satisfazem).
- **Tests precisam adaptação mínima** para construir
  `Tracked` em ambiente de teste.

### Neutras

- **Loops fixpoint cristalinos preservados** —
  ortogonais a comemo. Hash-based convergence mantida.
- **Sub-stores `TagIntrospector` não tracked
  separadamente** — granularidade per-method via trait
  track é suficiente; vanilla também não trackeia
  sub-stores.
- **`evict()` exposed mas não automatizado** — paridade
  vanilla (apenas CLI watch usa).

---

## Plano de validação

ADR-0073 transita de `PROPOSTO` para `ACEITE` quando todas
estas condições forem verdadeiras (verificadas em P204H):

1. **P204B materializado**: `#[comemo::track]` aplicado ao
   trait `Introspector`. Compila sem erros. `Send + Sync`
   bounds verificados.
2. **P204C materializado**: Layouter ganha lifetime
   parameter; field `introspector` é
   `Tracked<'a, dyn Introspector + 'a>`. Consumers
   migrados (10 sites Layouter).
3. **P204D materializado**: tipo `Position` em L1; `runtime.positions`
   populated; `position_of` retorna `Option<Position>`.
4. **P204E materializado**: `crystalline_evict(n)` wrapper
   em L4.
5. **P204F materializado**: 5-7 ficheiros corpus paridade
   novos cobrindo introspection features.
6. **P204G materializado**: measurements internos com
   logging hits/misses.
7. **Tests workspace verdes**: estimativa 1824 → 1830-1840
   (∆+6 a +16 tests novos para corpus + Position +
   unit tests de tracking).
8. **Crystalline-lint 0 violations**.
9. **Saída cristalino sanity-check** vs vanilla nos 5-7
   ficheiros corpus paridade — sem regressões observable.

ADR transita para `REJEITADO` se durante materialização
for descoberto:
- comemo 0.4.0 incompatível com `Introspector` por motivo
  não antecipado em A10.
- Tests catastróficos por mudança de assinatura
  Layouter (>10% regressão).
- Performance regressão >2× (improvável; comemo é
  optimização).

Se ADR for rejeitada, ADR-0066 permanece (decisão
intermédia hash-based mantém-se); explorar alternativas
B/C/D em ADR sucessor.

---

## Plano de materialização

7 sub-passos (P204B–H):

### P204B — `#[comemo::track]` em trait Introspector

Magnitude S-M.

- Adicionar `#[comemo::track]` ao trait
  `Introspector`.
- Adicionar `: Send + Sync` bound.
- Verificar `TagIntrospector` impl satisfaz bounds
  (provavelmente já).
- Tests workspace verdes.

### P204C — Layouter consumers via `Tracked`

Magnitude M.

- Layouter ganha `'a` lifetime parameter.
- Field `introspector: Tracked<'a, dyn Introspector +
  'a>`.
- Migração ~10 call sites em Layouter consumers.
- Wrapper API público mantém retrocompatibilidade.
- Tests adaptados (construir Tracked em fixtures).

### P204D — Position concrete

Magnitude S-M.

- Tipo `Position { page: NonZeroUsize, point: Point }` em
  `01_core/src/entities/position.rs`.
- `runtime.positions: HashMap<Location, Position>` em
  LayouterRuntimeState.
- Layouter popula durante layout (single-pass).
- Trait método `position_of -> Option<Position>` (substitui
  stub).
- 2-3 tests E2E.

### P204E — `crystalline_evict()` wrapper — ✅ MATERIALIZADO 2026-05-06

Magnitude S (real: ~30 min).

- ✅ Wrapper L4 em `04_wiring/src/eviction.rs`.
- ✅ Função passthrough: `pub fn crystalline_evict(max_age:
  usize) { comemo::evict(max_age); }`.
- ✅ `comemo` adicionado a `04_wiring/Cargo.toml`.
- ✅ L0 prompt em `00_nucleo/prompts/wiring/eviction.md`
  (hash 7ac7b48b).
- ✅ 2 sentinels (`p204e_crystalline_evict_existe`,
  `p204e_crystalline_evict_aceita_max_age_parametro`).
- Tests workspace: 1836 → 1838 (+2).
- Sem CLI integration (futuro pós-M8).

### P204F — Corpus paridade reduzido — ✅ MATERIALIZADO 2026-05-06

Magnitude M (real: ~70 min).

- ✅ 5 core + 1 opcional ficheiros `.typ` adicionados a
  `lab/parity/corpus/visual/`:
  - `outline-toc.typ`.
  - `counter-heading.typ`.
  - `figure-ref.typ`.
  - `equation-ref.typ`.
  - `cite-bibliography.typ` (+ `refs.yaml` asset).
  - `query-metadata.typ` (opcional).
- ✅ Cada ficheiro com `.toml` de expectativa
  cristalino-only.
- ✅ 6 smoke tests cristalino em
  `03_infra/src/integration_tests.rs`
  (`p204f_corpus_*_compila`).
- ⚠️ **`P204F.div-1`**: vanilla integration deferred per
  pre-existing DEBT-53/54 (lab/parity harness vanilla
  não funcional; spec assumiu observable harness; realidade
  é cristalino-only baseline). `here-locate.typ` (2º
  opcional) skipped — `here()`/`locate()` não estão
  registadas em stdlib cristalino.
- Tests workspace: 1838 → 1844 (+6).

### P204G — Measurements internos

Magnitude S.

- Logging hits/misses do cache comemo (feature
  `testing` se aplicável).
- Measurements de regressão em corpus existente.
- Sem comparação vanilla absoluta.

### P204H — Consolidado série + ADR ACEITE

Magnitude S documental.

- Relatório consolidado P204.
- Transitar ADR-0073 PROPOSTO → ACEITE.
- Anotar ADR-0066 secção "validação empírica" com
  registo de que M8 fechou.

---

## Cross-references

- ADR-0066 (Introspection runtime adiada — ACEITE com
  nota "intermediário até M8"). Após M8 fechar:
  superseded pela materialização (não revogada).
- ADR-0067 (Attribute grammar scoping — PROPOSTO). M8
  ortogonal; ADR-0067 permanece PROPOSTO.
- ADR-0072 (M7 fixpoint runtime estruturalmente fechado).
  Mantido. Loops fixpoint preservados em M8.
- ADR-0005 (PackageSpec/World — padrão B3 de tracking).
  Padrão B3 considerado (Alternativa B); rejeitado por
  desnecessidade.
- P192B consolidado (M7 fechado; M8 reconhecido como
  próximo passo natural).
- P200B (M5 universal completo).
- P190I (M6 fechado; F1 fechado).
- P203 consolidado (lacunas zeradas; baseline reconciliado).

---

## Pattern emergente

ADR-0073 aplica padrão consolidado pela série P204:

1. **Auditoria empírica de profundidade máxima** (16
   cláusulas A1-A16 cobrindo 5 blocos arquitecturais).
2. **Decisões fixadas com base em empírico**, não
   herdadas (per P203 §9.1 "mesmo `*B+` começam com
   inventário empírico").
3. **Paridade vanilla literal** quando viável (padrão
   A em C2; preferida sobre indirecções B/C/D).
4. **Magnitude calibrada por soma de componentes** —
   M8 é L cross-modular = M+M+S-M+S+S+M+S+S documental.

Pattern reaproveitável para futuras adopções de
infraestrutura cross-modular.
