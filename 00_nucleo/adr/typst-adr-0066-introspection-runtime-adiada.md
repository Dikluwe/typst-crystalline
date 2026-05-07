# ⚖️ ADR-0066: Introspection runtime — promoção da reserva conceptual a ficheiro PROPOSTO

**Status**: **SUPERSEDED-BY 0073** (P204H 2026-05-07).
**Data**: 2026-04-27 (PROPOSTO); 2026-05-05 (ACEITE com
nota "intermediário até M8" — P192B); 2026-05-07
(SUPERSEDED-BY 0073 — P204H); 2026-05-07 (anotação F3
fecho §C6a — P205E).

---

## Pendência §C6a fechada por F3 (P205B+C 2026-05-07) — anotação P205E

ADR-0073 §C6a (pendência herdada por P204D — `position_of`
retornava `None` como solução temporária por
`TagIntrospector` ser construído pre-layout sem acesso
a Layouter runtime) foi **fechada estruturalmente em F3**:

- **P205B** materializou `SealedPositions` em
  `01_core/src/entities/sealed_positions.rs` (newtype
  `#[comemo::track] impl` per ADR-0074 PROPOSTO).
- **P205C** activou impl real via `TagIntrospector::
  inject_positions(sealed)` — `Introspector::position_of`
  agora devolve `Some(Position)` real após injecção
  pós-layout (default empty preserva semântica
  pre-injecção).
- **P205D** deferiu `SealedLabelPages` por ausência de
  benefício empírico (Caminho B); F3 minimal completo
  via P205B+C suficiente para fechar §C6a.
- **P205E** (este registo) transitou ADR-0074 PROPOSTO
  → ACEITE final.

Cross-reference para auditor futuro:

- ADR-0074 (ACEITE 2026-05-07) — F3 minimal.
- `00_nucleo/materialization/typst-passo-205-relatorio-consolidado.md`.
- `00_nucleo/diagnosticos/typst-passo-205E-inventario.md`.

A cadeia chronológica completa é: **introspection
runtime adiada (ADR-0066, 2026-04-27)** → **M8 adoptou
comemo (ADR-0073, 2026-05-07; P204B-G)** → **F3 fechou
§C6a (ADR-0074, 2026-05-07; P205B+C+E)**. ADR-0066
permanece SUPERSEDED-BY 0073 — esta anotação não altera
o status, apenas regista o ponto final do chain-of-
custody que ADR-0066 originou.

---

## Superseded em P204H per ADR-0073 ACEITE 2026-05-07

ADR-0066 foi declarada **intermediária até M8** em P192B.
M8 fechou estruturalmente em P204H (2026-05-07) per
ADR-0073 ACEITE com 8/9 condições CUMPRIDAS (condição 9
PARCIAL por `P204F.div-1` — DEBT-53/54 pre-existing).

A promessa estrutural de ADR-0066 (adopção de
`#[comemo::track]` em M8) foi cumprida pela
materialização de ADR-0073 ao longo dos sub-passos
P204B–G:

- P204B aplicou `#[comemo::track]` ao trait `Introspector`.
- P204C migrou Layouter para `Tracked<'a, dyn Introspector + 'a>`.
- P204D materializou `Position` concrete; substituiu stub
  `position_of() -> Option<()>` por `Option<Position>`.
- P204E expôs `crystalline_evict(n)` wrapper em L4.
- P204F adicionou corpus paridade introspection
  (cristalino-only baseline per P204F.div-1).
- P204G materializou measurements internos
  (`typst_infra::measurements`).

ADR-0066 **conteúdo histórico preservado abaixo** —
serve como registo da decisão intermédia e do raciocínio
que motivou hash-based convergence como ponte até M8.
Não revogada (a decisão foi correcta no seu contexto);
superseded (a sua promessa cumpriu-se).

Ver:
- ADR-0073 (ACEITE 2026-05-07).
- `00_nucleo/materialization/typst-passo-204-relatorio-consolidado.md`.
- `00_nucleo/diagnosticos/typst-passo-204H-inventario.md`.

---

## Validação empírica P192A + estado intermediário

**Data**: 2026-05-05 (P192B).

ADR-0066 transita PROPOSTO → ACEITE com qualificação **intermediário
até M8**.

### Validação empírica

P192A diagnóstico confirmou que decisão de adiar introspection
runtime (e adoptar hash-based convergence como mecanismo
intermédio) é viável:

- **M7 estruturalmente fechado** (P192B; ADR-0072).
- **2 loops fixpoint funcionais**:
  - TOC fixpoint (`layout/mod.rs:1515`) — activo em produção;
    forward refs page numbers.
  - `run_fixpoint` (`introspect/fixpoint.rs:65`) — opt-in para
    stdlib features.
- **Tests E2E verdes**: 1.802 workspace; 13+ tests fixpoint.rs.

### Estado intermediário

Hash-based convergence é decisão **intermédia viável**, **não
solução arquitectural definitiva**.

Diferenças vs vanilla typst:

| Aspecto | Vanilla typst | Cristalino actual |
|---------|---------------|-------------------|
| Convergence | comemo::Constraint::validate | hash-based (compute_tags_hash / page map) |
| Re-walks | parciais (cache granular) | full por iteração |
| Cap | MAX_ITERS = 5 | MAX_ITERATIONS = 5 (paridade nominal) |
| Performance | comparable | aceitável; gargalo se features stdlib expandirem em produção |
| Tracking | `#[comemo::track]` | sem track |

### M8 — adopção comemo planeada

**M8 introduzirá `comemo::Track`** em:

- Trait `Introspector` (`#[comemo::track]` impl block).
- Queries location-aware (`is_numbering_active_at`,
  `flat_counter_at`, `formatted_counter_at`,
  `figure_number_at_index`, etc.).
- Sub-stores `TagIntrospector` se aplicável.

**Objectivos M8**:

- Paridade vanilla typst.
- Saída igual ao vanilla.
- Performance comparável.
- Re-walks parciais via invalidação granular.

ADR-0066 cobre **decisão de adiar** comemo até M5+M6+M7 estarem
estruturalmente fechados. **Cumprido em P192B**. **Próximo passo
natural**: M8 — ADR dedicada à adopção comemo (futura).

### Cross-references

- **P192A** — diagnóstico estado actual M7.
- **P192B** — declaração formal M7 estruturalmente fechado.
- **ADR-0072** — ACEITE; M7 fixpoint runtime fechado.
- **M8** — próximo passo natural; adopção `comemo::Track`.

---

## Status original (preservado para histórico)

**Status original**: `PROPOSTO` (2026-04-27).

---

## Nota sobre numeração

A reserva conceptual referida cumulativamente como "ADR-0017"
em relatórios e diagnósticos (P156B inventário 148 §A.9, P159A
"sem validação cross-reference per ADR-0017", P159B §3 categoria
A, P160 §1) **NÃO corresponde** ao ADR-0017 actual em ficheiro,
que cobre **adiamento de `eval()` e estratégia typst-library**
(IMPLEMENTADO desde 2026-03-26 —
`typst-adr-0017-adiamento-eval-typst-library.md`).

A reserva "Introspection runtime adiada" foi sempre conceptual,
sem ficheiro próprio. P160 §1 confirmou empiricamente o estado
factual. Este ADR usa **número 0066 (próximo disponível)** em
vez de reocupar 0017 — reocupação seria divergência observable
do ADR existente já IMPLEMENTADO.

**Slot 0063** está reservada conceptualmente para "outra crate
específica" (column flow per nota README); preserva-se sem
ficheiro per política "sem novas reservas".

Referências cruzadas em código/relatórios anteriores que
mencionam "ADR-0017 Introspection runtime" devem ler-se como
**ADR-0066** após este passo (P160A). Refactor cumulativo de
relatórios antigos NÃO é necessário per política — comentários
históricos preservam-se.

---

## Contexto

Cristalino actual é **single-pass** per **ADR-0033** minimal:
pipeline `eval` → `introspect` → `layout` → `export_pdf`.
`introspect.rs` (1108 linhas) executa pré-passagem analítica
DFS recursiva sobre Content tree, populando counters e
resolved_labels antes de `layout()` correr.

**Vanilla typst Introspection é fundamentalmente runtime/
multi-pass**:
- `counter()` runtime queries (counter values em diferentes
  posições do documento).
- `state()` mutable runtime state com chaves arbitrárias.
- `here()` / `locate()` position-aware computations.
- `query(target)` runtime introspection (lookup de elementos
  por tipo/label).
- `metadata(value)` arbitrary attaching para query subsequente.
- `position(target)` location-aware (page/column/y-coord).
- `convergence` fixpoint logic para multi-pass stabilization.
- `introspector` engine (695 linhas vanilla) que cache states
  para queries.

**Cobertura cristalina actual** (per P160 §3 + cobertura A.9):
- 1/13 features implementadas: `counter()` (subset minimal P60-62
  single-pass).
- 1/13 parcial: `measure()` (helper privado `measure_content`;
  sem stdlib expose).
- 11/13 ausentes: `state`, `here`, `locate`, `query`, `metadata`,
  `position`, `convergence`, `introspector`, `location`,
  `locator`, `tag`.

**Cobertura observable A.9**: 1/6 = ~17% (saturada por counter()
single-pass).

**Subpadrão cristalino emergente** (P158B/P159C/P159F): "infra-
estrutura state lookup" via fields aditivos em `CounterState`
populados pelo walk introspect e consumidos pelo layouter.
Patamar N=3:
- `state.lang: Option<Lang>` (P158B).
- `state.bib_entries: Vec<BibEntry>` (P159C).
- `state.bib_numbers: HashMap<String, u32>` (P159F).

Este subpadrão demonstra **viabilidade single-pass para subset
de features que NÃO exigem runtime queries genuínas** —
counters por tipo, lookups cross-element same-document, etc.

**Limitação fundamental**: features observable user-facing além
de counter() exigem multi-pass ou runtime state mutável.
P160 §4 confirma empiricamente: tecto Introspection puro
single-pass está **saturado em ~17%**. Refinos qualitativos
adicionais (R1/R2/R3 em P160 §5) movem N=3 → 4+ mas não
cobertura observable.

---

## Decisão

**Promover a reserva conceptual "Introspection runtime adiada"
a ADR concreta com status PROPOSTO** sob número 0066.

**Subset minimal pós-promoção** (per P160 §6 recomendação Bloco B):
1. **`state(key, init)` runtime mutable state** (M; P160B
   primeiro candidato Bloco B).
2. **`metadata(value)` arbitrary attaching** (S+; P160C).
3. **`here()` / `locate()` current location** (M; P160D; depende
   `Location` type novo).
4. **`query(target)` runtime introspection** (M+; P160E; depende
   `Location` + query engine).
5. **`position(target)` location-aware** (S+; P160F; depende P160D).

**Cobertura esperada pós-Bloco B subset minimal**: ~17% → ~50%.

**Status PROPOSTO** — autorização arquitectural concedida em
princípio mas **não em vigor** até passo de materialização real
(P160B subset minimal ou equivalente).

**Promoção a `IMPLEMENTADO`** ocorre quando:
1. Pelo menos uma feature runtime queries genuína materializada
   (e.g. `state(key, init)` com mutable storage end-to-end).
2. Pipeline `introspect` extendido com 2-pass ou state queries
   reais (não apenas state lookup ortogonal subpadrão N=3).
3. Tests E2E cobrem feature observable user-facing (não apenas
   infraestrutura internal).

---

## Análise de pureza (paridade ADR-0029)

| Propriedade | Estado | Notas |
|-------------|--------|-------|
| Zero I/O | ✓ | Runtime queries operam sobre AST + state em memória; sem disk/network |
| Zero estado global mutável | ✓ (com nuance) | `state()` é mutable mas escopo do documento; não global ao processo |
| Determinismo total | ✓ | Same input + same queries = same output |
| Sem dependência runtime externa | ✓ | Pipeline cristalino |
| Compatível L1 | ✓ | Sem requisitos de I/O |

**Nuance**: `state()` runtime mutable é mutable mas localizada
ao document compile run — não viola "zero estado global mutável"
da ADR-0029 (que se refere a `static mut` cross-process).

---

## Consequências

**Positivas**:
- Paridade observable significativamente mais ampla com vanilla
  (Introspection 17% → ~50% pós-Bloco B subset minimal; ~83-100%
  pós-`measure()` cross-módulo).
- Desbloqueia features família 159 cross-document (cite refs
  cross-document) — P159A "sem validação cross-reference"
  removida.
- Desbloqueia counters refinados (lookup por tipo, count
  totals, etc.).
- Desbloqueia `query()` que é fundamento para muita lógica show
  rules user-facing.

**Negativas**:
- Complexidade pipeline aumenta — 2-pass runtime queries vs
  single-pass actual.
- Divergência arquitectural vs cristalino single-pass actual.
  ADR-0033 paridade observable preserva-se mas implementação
  interna diverge.
- Superfície de testes cresce significativamente — runtime
  queries têm semântica complexa (forward refs, fixpoint
  convergence, etc.).
- Refactor `introspect.rs` (1108 linhas) significativo.

**Trade-off aceite**: complexidade vs paridade ampla.
Justifica-se pelo tecto saturado em ~17% sem promoção.

---

## Alternativas consideradas

### Alt A — Manter single-pass com features observable limitadas
- **Pro**: simplicidade arquitectural; pipeline já estável.
- **Con**: tecto Introspection saturado em ~17% per P160 §4;
  Cite cross-document refs permanecem ausentes; `query()` e
  show rules user-facing limitadas.
- **Rejeitada**: tecto trivialmente saturado por counter()
  single-pass; refinos qualitativos (R1/R2/R3) não movem
  cobertura observable.

### Alt B — Implementar Introspection runtime cristalino sem ADR
- **Pro**: trabalho directo sem overhead administrativo.
- **Con**: magnitude da decisão arquitectural exige formalização;
  ausência de ADR torna decisões ad-hoc difíceis de trace.
- **Rejeitada**: paridade política ADR-0062-create — reservas
  pré-existentes são formalizadas antes de materialização real.

### Alt C — Adoptar pipeline vanilla integralmente (multi-pass + comemo)
- **Pro**: paridade arquitectural máxima.
- **Con**: desproporcionalidade vs subset minimal cristalino;
  comemo cache infrastructure significativa; arquitectura L1
  pura ficaria ameaçada por dependência runtime cache.
- **Rejeitada**: subset minimal (P160 §6 5 features) é
  suficiente para destrancar Bloco B; refactor pipeline real
  pode preservar single-pass para casos comuns + 2-pass apenas
  para queries runtime.

---

## Plano de promoção futuro

**P160B (próximo passo natural pós-este)**: materializar
`state(key, init)` runtime mutable state.
- Tamanho estimado: M.
- Δ tests esperado: +10-15.
- Cobertura Δ: +6-8pp Introspection.
- Pipeline: extender `introspect.rs` para suportar state queries
  via fixpoint simples (2 iterações se state convergir).
- Promoção ADR-0066: PROPOSTO → IMPLEMENTADO após P160B
  materializa.

**Ordem subsequente**: P160C (metadata) → P160D (here/locate) →
P160E (query) → P160F (position). Cada passo individual M com
incrementos de cobertura ~3-10pp.

**Bloco C cross-módulo** (após Bloco B saturado): `measure()`
stdlib expose (depende Layout integration); cross-document refs
(depende multi-document pipeline).

---

## Precedentes citáveis

### Reservas pré-existentes formalizadas (subpadrão emergente)

**ADR-0062-create** (passo administrativo XS executado pós-P159B):
formalizou reserva pré-existente "ADR-0062 hayagriva" como
ficheiro PROPOSTO. Subpadrão "passo administrativo XS criar
ADR PROPOSTO a partir de reserva pré-existente" emergiu N=1.
**P160A é segunda aplicação** do subpadrão (N=1 → 2).

### ADRs precedentes de pipeline (cristalino)

- **ADR-0033** Paridade observable — fundamento para divergência
  single-pass cristalino vs vanilla multi-pass.
- **ADR-0054** Perfil graded — fundamento para subset minimal
  Introspection runtime.
- **ADR-0029** Pureza física — confirma compatibilidade L1.
- **ADR-0017 (existente)** Adiamento de eval — paridade estrutural
  histórica de "feature deferida que requer migração faseada".

### Diagnósticos cristalinos relevantes

- **P156B inventário 148** — primeira menção cumulativa
  "ADR-0017" Introspection (conceptual; não ficheiro).
- **P159A** — "sem validação cross-reference per ADR-0017"
  (citação que agora se resolve a ADR-0066).
- **P159B §3 categoria A** — features bloqueadas por reserva
  conceptual.
- **P160 relatório** — diagnóstico Introspection com tecto
  saturado + recomendação primária `ADR-0017-create` (que com
  esta resolução de numeração se torna `ADR-0066-create` =
  este passo).

---

## Referências

- ADR-0017 (existente) — `typst-adr-0017-adiamento-eval-typst-library.md`
  — IMPLEMENTADO 2026-03-26; cobre tópico distinto.
- ADR-0029 — Pureza física L1 (revoga ADR-0028).
- ADR-0033 — Paridade observable.
- ADR-0034 — Estrutura diagnóstico canónica.
- ADR-0054 — Perfil graded.
- ADR-0060 — Model structural roadmap.
- ADR-0061 — Layout Fase X roadmap.
- ADR-0062 — Hayagriva bibliography parsing (PROPOSTO; paridade
  administrativa para este passo).
- ADR-0065 — Inventariar primeiro (P160 §1 + este passo §1
  inventário).
- P156B relatório — inventário 148 com Introspection §A.9.
- P159 relatório — par acoplado Bibliography + Cite com referência
  cruzada conceptual.
- P159B relatório — diagnóstico amplo família 159 com matriz
  dependências.
- P160 relatório — diagnóstico Introspection com tecto saturado.
- P160A (este passo) — formaliza reserva conceptual em ficheiro
  ADR concreto.

---

## Próximos passos

1. **P160B** (próximo natural; passo de materialização):
   `state(key, init)` runtime mutable state. M; +10-15 tests;
   +6-8pp Introspection. Promoção ADR-0066 PROPOSTO → IMPLEMENTADO
   após este passo materializa.

2. **P160C-F** (sequência Bloco B per P160 §5): metadata, here/
   locate, query, position. Cada um M; cumulativo +30-40pp
   Introspection.

3. **Bloco C cross-módulo** (pós-Bloco B): measure stdlib expose;
   cross-document cite refs. Não materializável em Introspection
   puro — depende Layout integration ou multi-document pipeline.
