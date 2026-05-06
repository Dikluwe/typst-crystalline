# Diagnóstico P204A — Decisões C1-C14 com base em A1-A16

**Data**: 2026-05-06.
**Executor**: Claude Code (Opus 4.7).
**Spec**: `00_nucleo/materialization/typst-passo-204A.md`.
**Auditoria fonte**:
`00_nucleo/diagnosticos/typst-passo-204A-auditoria-comemo.md`.

---

## §1 Sumário das decisões

| Cláusula | Decisão | Magnitude impacto |
|----------|---------|-------------------|
| C1 | Escopo M8: **Médio** (adopção comemo + sub-stores trackable + Position concrete + validação reduzida) | L |
| C2 | Mecanismo: **`#[comemo::track]` directamente no trait `Introspector`** (paridade vanilla literal) | M |
| C3 | Layouter consumers: `Tracked<'a, dyn Introspector + 'a>` (vanilla pattern) | M |
| C4 | Sub-stores trackable: granularidade per-method (não per-sub-store) | inerente a C2 |
| C5 | Re-walks parciais: **NÃO** em M8 base; comemo memoíza queries sobre output do walk | resolvido por C7 |
| C6 | Política invalidação: tracking-based intra-compilation; `evict()` exposed para callers (paridade vanilla) | trivial |
| C7 | Fixpoint loops: **manter** (ortogonais a comemo per A13) | sem alteração |
| C8 | Position concrete: **sub-passo M8** (não M8.5 nem pós-M8) | S-M dentro de M8 |
| C9 | Validação paridade: **escala reduzida** — sanity checks + corpus expandido (5-7 ficheiros novos com introspection) | M |
| C10 | Benchmarks: **escala reduzida** — measurements internos sem comparação vanilla absoluta | S-M |
| C11 | Magnitude agregada: **L** (cross-modular; análoga a M6 série) | — |
| C12 | Plano `*B+`: 7 sub-passos (B-H) sem condicionais; cada um é S-M | — |
| C13 | ADR-0073 PROPOSTO: estrutura standard; transita ACEITE em fim de M8 | — |
| C14 | Sem condicionais: **CUMPRIDO** | — |

---

## §2 C1 — Escopo de M8

### 2.1 Range avaliado

- **Mínimo**: adopção `#[comemo::track]` em trait
  `Introspector`; consumers Layouter migram. ~M.
- **Médio**: Mínimo + sub-stores trackable selectivamente
  + queries location-aware re-emitidas com tracking
  granular + Position concrete + validação reduzida. ~L.
- **Completo**: Médio + benchmarks comparativos vanilla
  + corpus paridade introspection completo. ~XL.

### 2.2 Decisão fixada — **Médio**

**Justificação** (baseada em A1-A16):

- A1+A2: trait Introspector cristalino é compatível
  trivialmente — todos os 20 métodos `&self`. Mínimo é
  baixo custo.
- A3: 10 consumers Layouter activos — migração C3 é parte
  do trabalho.
- A14: 2 fields Layouter categoria A (trackable). Sub-stores
  trackable selectivamente é leve (granularidade per-method
  via comemo).
- A16: Position concrete é sub-passo M8 natural — incluir.
- A15: corpus paridade actual zero introspection.
  Validação completa requer expansão substancial (XL).
  **Reduzir** a 5-7 ficheiros novos cobrindo features
  básicas (TOC + counters + figure refs).
- Benchmarks comparativos vanilla (corpus completo)
  exigem infraestrutura de medição não existente — fora
  de M8 inicial.

**Médio** dá fecho funcional de M8 sem inflação.

---

## §3 C2 — Mecanismo de adopção `comemo` no trait

### 3.1 Alternativas

- **A** — `#[comemo::track]` directamente em `Introspector`
  (paridade vanilla literal).
- **B** — Padrão B3 ADR-0005: `Introspector` plain +
  `TrackedIntrospector` separado + blanket impl.
- **C** — Funções livres memoizadas em vez de métodos.
- **D** — Sub-trait dedicada para subset trackable.

### 3.2 Decisão fixada — **A** (paridade vanilla literal)

**Justificação**:

- A6 confirma: comemo 0.4.0 suporta `#[track]` em traits
  não-genéricos. `Introspector` cristalino é
  não-genérico (trait sem parameters).
- A7: vanilla usa **exactamente** `#[comemo::track] pub
  trait Introspector` — paridade literal disponível.
- A2: todos os 20 métodos cristalinos são `&self` — sem
  necessidade de sub-trait separada.
- A10: restrições satisfeitas (no generics, `&self`,
  `ToOwned` args, `Hash` returns).

Padrão B (B3) acrescenta camada de indirecção sem
benefício — `Introspector` plain só seria útil se houvesse
métodos não-trackable; não há.

Padrão C (funções livres) quebra OOP-ish API; vanilla
não usa.

Padrão D (sub-trait) só faz sentido se houver mut-during-
build — A2 confirmou que não.

### 3.3 Trait modificado

```text
+ #[comemo::track]
  pub trait Introspector: Send + Sync {
      // 20 métodos existentes (inalterados em assinatura).
  }
```

**Trait fica `Send + Sync`** (requisito comemo, paridade
vanilla). Implica que `TagIntrospector` e qualquer impl
ganhe `Send + Sync` (provavelmente já satisfeito; verificar
em sub-passo).

---

## §4 C3 — Layouter consumers — assinatura

### 4.1 Alternativas

- **a** — `&dyn Introspector` (actual).
- **b** — `Tracked<'a, dyn Introspector>` (paridade
  vanilla).
- **c** — `&'a (impl Introspector + 'a)` (concrete generic).

### 4.2 Decisão fixada — **b** (paridade vanilla)

**Justificação**:

- A8: vanilla usa `Tracked<dyn Introspector + '_>` em
  todos os consumers. Padrão canónico.
- A11: `Tracked<'a, T>` armazenável em struct com
  lifetime parameter. `Layouter<M, S>` precisa ganhar
  parameter `'a`.
- A12: lookup O(1) amortizado; cost por chamada
  consideravelmente baixo.

Layouter ganha lifetime parameter:

```text
- pub struct Layouter<M: FontMetrics, S: ImageSizer = NullImageSizer>
+ pub struct Layouter<'a, M: FontMetrics, S: ImageSizer = NullImageSizer>
```

Field actualizado:

```text
- pub(super) introspector: TagIntrospector,
+ pub(super) introspector: Tracked<'a, dyn Introspector + 'a>,
```

API pública (`pub fn layout`) mantém retrocompatibilidade
via wrapper que constrói `Tracked` internamente.

### 4.3 Migration custo

- Layouter ganha lifetime parameter — propagação por
  ~10 call sites de `Layouter<M, S>`.
- Consumers Layouter (mod.rs, equation.rs, references.rs,
  outline.rs) consomem via `self.introspector.<method>(...)`
  — sintaxe inalterada (Tracked deref-coerces).
- Tests podem precisar de adaptação para construir `Tracked`.

Magnitude C3 puro: M.

---

## §5 C4 — Sub-stores trackable selectivamente

### 5.1 Decisão fixada — **granularidade per-method**

Não há sub-store trackable directamente — granularidade
é dada por `#[comemo::track]` no trait via per-method
constraint tracking.

**Justificação**:

- A4: granularidade dominante per-key (HashMap-based).
  Comemo já oferece isso via constraint per-method.
- Tornar sub-stores tracked separadamente (e.g.
  `#[comemo::track] impl LabelRegistry`) seria duplicação:
  Introspector trait métodos delegariam para sub-stores
  tracked + sub-stores também tracked. Granularidade
  efectiva é a mesma.
- Vanilla **não** trackeia sub-stores separadamente —
  só o trait. Paridade.

### 5.2 Sub-stores especiais

- `metadata` (append-only) e `headings_for_toc`
  (append-only) — comemo trata como qualquer outro.
- `state` (mutável durante populate) — populate ocorre
  pré-tracking; tracking ocorre pós-populate (read-only).
  Não há conflito.

### 5.3 LayouterRuntimeState

Categoria A em A14. Decisão: **não trackable em M8 base**.
Razão: `runtime` é write-only durante layout (popula
`label_pages`, `is_readonly`); não consultado por queries
de introspection externas (via trait).

Caso futuro (pós-M8) precise de queries sobre runtime,
adicionar `#[comemo::track]` em LayouterRuntimeState.
**Adiado** explicitamente.

---

## §6 C5 — Re-walks parciais

### 6.1 Decisão fixada — **NÃO em M8 base**

**Justificação** (baseada em A8 + A13):

- Vanilla **não** faz re-walks parciais via comemo. comemo
  memoíza queries sobre output do walk; o walk em si é
  re-executado integralmente cada iteração de fixpoint.
- A13 confirmou: fixpoint loops cristalinos (TOC +
  run_fixpoint) iteram completamente cada vez.
- Mudar para re-walks parciais é trabalho adicional
  significativo (não trivial); não é parte de "paridade
  vanilla".

M8 base **mantém walk integral** + comemo memoíza queries.

Re-walks parciais ficam para optimização posterior se for
medido bottleneck (não é em fase actual).

---

## §7 C6 — Política de invalidação

### 7.1 Decisão fixada — **paridade vanilla**

- **Tracking-based** intra-compilation: comemo gere
  automaticamente.
- **`evict()` exposto** para callers (CLI / wiring) —
  para watch mode futuro.

Per A9: vanilla usa exactamente esta política (`evict(10)`
no CLI watch).

### 7.2 Cristalino implementa

- M8 sub-passo dedicado expõe `crystalline_evict(n: usize)`
  ou similar wrapper sobre `comemo::evict`.
- L4 wiring opcionalmente integra em watch mode futuro.

---

## §8 C7 — Loops fixpoint cristalinos

### 8.1 Decisão fixada — **manter** (ortogonais)

Per A13: ortogonais a comemo. Cada loop tem MAX=5
(paridade vanilla).

M8 **não toca** em `01_core/src/rules/layout/mod.rs:1506`
nem `01_core/src/rules/introspect/fixpoint.rs:33`. Loops
permanecem como fonte da verdade de convergência;
comemo adiciona granularidade dentro de cada iteração.

---

## §9 C8 — Position concrete — escopo dentro de M8

### 9.1 Decisão fixada — **sub-passo M8**

Per A16:
- Vanilla tem `Introspector::position()` no trait.
- Cristalino tem stub `position_of() -> Option<()>`.
- Position é parte natural da paridade vanilla.

### 9.2 Sub-passo M8 dedicado

Sub-passo M8 chamado P204X (X letra concreta a fixar em
C12) materializa Position:

1. Tipo `Position` em `01_core/src/entities/position.rs`
   (struct paralela a `PagedPosition`).
2. Sub-store ou field em LayouterRuntimeState
   (`runtime.positions: HashMap<Location, Position>`).
3. Layouter feedback single-pass popula durante layout.
4. Trait `Introspector::position_of` retorna
   `Option<Position>` (substituindo stub).
5. 2-3 tests E2E.

Magnitude S-M dentro de M8.

### 9.3 Não M8.5 nem pós-M8

M8.5 separado seria sub-marco artificial. Pós-M8 deixaria
paridade vanilla incompleta. Sub-passo dedicado dentro de
M8 é solução natural.

---

## §10 C9 — Validação de paridade

### 10.1 Decisão fixada — **escala reduzida**

- A15 confirmou: corpus actual tem 0 ficheiros que
  exercitam introspection.
- Validação completa exige expansão substancial do corpus
  (cobrindo TOC, counters, figure refs, equation refs,
  bibliography, here(), locate, query) — XL trabalho.

**M8 base** adiciona **5-7 ficheiros novos** ao corpus
cobrindo features básicas:

- `lab/parity/corpus/visual/outline-toc.typ` (TOC).
- `lab/parity/corpus/visual/counter-heading.typ` (counter
  heading).
- `lab/parity/corpus/visual/figure-ref.typ` (figure ref).
- `lab/parity/corpus/visual/equation-ref.typ` (equation
  ref).
- `lab/parity/corpus/visual/cite-bibliography.typ`
  (bibliography + cite).
- (Opcional) `lab/parity/corpus/visual/here-locate.typ`.
- (Opcional) `lab/parity/corpus/visual/query-metadata.typ`.

Sub-passo M8 dedicado (P204Y; letra a fixar em C12) cobre
isto. Magnitude S-M.

Cobertura completa fica para pós-M8.

---

## §11 C10 — Benchmarks

### 11.1 Decisão fixada — **escala reduzida**

- Vanilla benchmarks comparativos requerem infraestrutura
  de medição não existente (pó equivalente para tempo de
  introspection cristalino vs vanilla; corpus comparável;
  hardware controlado).
- M8 base **NÃO inclui** benchmarks comparativos.

**Measurements internos** simples (sem comparação vanilla
absoluta):

- Hits/misses do cache comemo durante compilação típica
  (via `comemo::testing::last_was_hit` se feature
  activada, ou logging dedicado).
- Tempo de query introspection antes/depois do tracking
  para detectar regressões.

Sub-passo M8 dedicado (P204Z; letra a fixar em C12).
Magnitude S.

Benchmarks comparativos vanilla ficam para passo
dedicado pós-M8.

---

## §12 C11 — Magnitude agregada

### 12.1 Soma das decisões C1-C10

| Componente | Magnitude |
|------------|-----------|
| C2 (`#[comemo::track]` em Introspector) | M |
| C3 (Layouter ganha lifetime; consumers Tracked) | M |
| C4 (granularidade per-method; sem trabalho extra) | trivial |
| C5 (sem re-walks parciais) | 0 |
| C6 (`evict()` wrapper) | trivial |
| C7 (manter fixpoint) | 0 |
| C8 (Position concrete) | S-M |
| C9 (validação paridade reduzida; 5-7 ficheiros corpus) | M |
| C10 (benchmarks reduzidos) | S |
| **Total agregado** | **L** |

### 12.2 Comparação com M6

M6 série teve P190 (9 sub-passos) + P191 ramo paralelo
(3 sub-passos) = 12 sub-passos efectivos. **L
cross-modular** confirmada.

M8 **menos sub-passos** mas **cross-modular**: trait L1
+ Layouter L1 + tests L1 + corpus L1.

Magnitude **L** confirmada.

---

## §13 C12 — Sub-passos `*B+`

### 13.1 Plano de 7 sub-passos (B-H)

| Sub-passo | Tipo | Conteúdo | Magnitude |
|-----------|------|----------|-----------|
| **P204B** | implementação base | `#[comemo::track]` em trait `Introspector` (C2) + `Send + Sync` bounds; verificar `TagIntrospector` impl satisfaz | S-M |
| **P204C** | implementação Layouter | Layouter ganha `'a` lifetime; `introspector` field passa a `Tracked<'a, dyn Introspector + 'a>` (C3); migrar 10 consumers | M |
| **P204D** | implementação Position | Tipo `Position` em L1; `runtime.positions` populated pelo Layouter; `position_of` retorna `Option<Position>` (C8) | S-M |
| **P204E** | infra `evict()` | Wrapper `crystalline_evict(n)` em L4 wiring; expose para CLI futuro (C6) | S |
| **P204F** | corpus paridade reduzido | 5-7 ficheiros .typ novos cobrindo features de introspection (C9) | M |
| **P204G** | benchmarks reduzidos | Logging hits/misses; measurements de regressão (C10) | S |
| **P204H** | consolidado série + ADR-0073 ACEITE | Relatório consolidado P204; transitar ADR-0073 PROPOSTO → ACEITE | S documental |

### 13.2 Sem condicionais

C12 lista sub-passos com magnitude e conteúdo fixos.
Cada sub-passo `*B+` aplica a convenção de inventário
empírico antes de implementação (per P203 §9.1).

Sub-passo `*B-G` **não pré-define** as exact LOC alterations
— essas emergem do inventário de cada sub-passo.

---

## §14 C13 — ADR-0073 PROPOSTO

### 14.1 Decisão fixada — **criar PROPOSTO**

Localização:
`00_nucleo/adr/typst-adr-0073-comemo-introspector.md`.

Estrutura:
- Estado: `PROPOSTO` (transita ACEITE em P204H).
- Data: 2026-05-06.
- Contexto: M5+M6+M7+M9 estruturalmente fechados; baseline
  empírico reconciliado; lacunas zeradas (per P203
  consolidado §13).
- Decisão: adopção `#[comemo::track]` em trait `Introspector`
  per padrão A (vanilla literal); Layouter consumers via
  `Tracked`; Position concrete inclusa.
- Consequências: positivas (paridade vanilla; granularidade
  invalidação), negativas (Layouter ganha lifetime; complexidade
  de tipos), neutras (tests precisam adaptação mínima).
- Alternativas avaliadas (B, C, D) com justificação de
  rejeição.
- Cross-references (ADR-0066 superseded; ADR-0067 PROPOSTO
  permanece; ADR-0072 mantido).
- Plano de validação para transição ACEITE.

### 14.2 Transição ACEITE

ADR-0073 transita PROPOSTO → ACEITE quando:

1. P204B-G todos materializados.
2. Tests workspace verdes (estimativa: 1824 → 1830-1840).
3. Crystalline-lint 0 violations.
4. `#[comemo::track]` aplicado a `Introspector` sem
   diagnóstico bloqueante.
5. Layouter consumers usam `Tracked<dyn Introspector>`.
6. Position concrete materializada.
7. 5-7 ficheiros corpus paridade adicionados.

### 14.3 Transição superseding ADR-0066

ADR-0066 (Introspection runtime adiada — ACEITE com nota
"intermediário até M8") **NÃO é revogada** mas **superseded
pela transição** — anotação adicional na ADR-0066 secção
"validação empírica" registando que M8 chegou (em P204H
quando ADR-0073 ACEITE).

---

## §15 C14 — Sem cláusulas condicionais

**CUMPRIDO**. Cada decisão C1-C13 fixada com valor
concreto:

- C1: Médio.
- C2: A (paridade vanilla literal).
- C3: b (Tracked).
- C4: per-method via trait track.
- C5: NÃO em M8 base.
- C6: tracking-based + `evict()` exposed.
- C7: manter fixpoint.
- C8: sub-passo M8 (P204D).
- C9: escala reduzida (5-7 ficheiros corpus).
- C10: escala reduzida (measurements internos).
- C11: L cross-modular.
- C12: 7 sub-passos B-H sem condicionais.
- C13: ADR-0073 PROPOSTO criado em P204A.

Casos que dependem de auditoria foram resolvidos com base
em A1-A16. Sem `if`s remanescentes.

---

## §16 Critério de progressão para `*B`

Per spec §6, P204A está concluído quando:

- [x] A1-A16 todos com etiqueta CONFIRMADO ou DIVERGÊNCIA
  registada (todos CONFIRMADO).
- [x] C1-C14 instanciadas com valores concretos.
- [x] ADR-0073 PROPOSTO escrito.
- [x] Magnitude calibrada (C11: L cross-modular).
- [x] Plano `*B+` sem condicionais (7 sub-passos B-H em
  C12).

Sem `P204A.div-N`. Snapshot 2026-05-05 reflecte realidade
(zero divergências detectadas).

---

## §17 Referências

- `00_nucleo/diagnosticos/typst-passo-204A-auditoria-comemo.md`
  (auditoria empírica fonte).
- `00_nucleo/materialization/typst-passo-204A.md` (spec).
- `00_nucleo/diagnosticos/snapshot-2026-05-05.md` (snapshot
  reconciliado).
- `00_nucleo/adr/typst-adr-0066-introspection-runtime-adiada.md`
  (estratégia adiamento — superseded em P204H).
- `00_nucleo/adr/typst-adr-0072-m7-fixpoint-runtime-fechado.md`
  (M7 mantido; fixpoint loops permanecem per C7).
- `00_nucleo/adr/template-adr.md` (template para ADR-0073).
