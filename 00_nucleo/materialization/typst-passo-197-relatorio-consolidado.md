# Relatório Consolidado — Série P197

**Data**: 2026-05-04
**Sub-passos**: A ✅ B ✅ C ✅
**Magnitude agregada**: S diagnóstico (P197A) + S/M refactor (P197B) + S documental (P197C)
**Estado**: Série fechada
**Pattern arquitectural**: ADR-0069 stylesheet — 3ª variante operacional.

---

## §1 Resumo executivo

Série P197 fecha **E3** (Figure walk arm) — penúltima excepção
M5 não-residual antes do passo dedicado abrir sub-store
`headings_for_toc` (lacuna #3). Característica distintiva:
**cenário α** — caminho Introspector para figure numbering já
activo em produção desde P184 (variant `ElementPayload::Figure`
+ `from_tags` arm + sub-store via `CounterRegistry` + consumer
C3 P184D). P197B é refactor estilístico, não migração
arquitectural.

**Resultado estrutural**:
- E3 declarada formalmente fechada estruturalmente em L0.
- Helper `compute_figure(state, kind, is_counted) -> Option<usize>`
  extraído, terceiro na família ADR-0069 stylesheet
  (`compute_labelled` P195D + `compute_heading_auto_toc` P196B
  + **`compute_figure` P197B**).
- Mutação legacy preservada como write paralelo M5 — cláusula
  gate substancial cadeia E2-E3 resolvida sem disparar gate
  (`compute_labelled` P195D Figure arm lê
  `state.figure_numbers.last()` durante walk).
- **Tag pós-recursão dispensada** — diferente de P195D/P196B.
  Pattern ADR-0069 aplicado apenas como stylesheet de helper.

**Resultado funcional**: output observable em produção
inalterado (refactor estilístico puro).

**Pattern ADR-0069 consolidado com 3 variantes operacionais**:
- **P195D variante** — target não-locatable: snapshot+find_map.
- **P196B variante** — content locatable: `emitted_loc` directo.
- **P197B variante (cenário α)** — declaração formal sem Tag pós-recursão.

---

## §2 Sub-passos materializados

| Sub | Magnitude planeada | Magnitude real | Δ tests | L0 tocado |
|-----|-------------------|----------------|---------|-----------|
| **P197A** (diagnóstico) | S puro | S | 0 | 0 |
| **P197B** (refactor) | S/M | S/M | +5 | 1 (`introspect.md` hash `b9f78ff9`) |
| **P197C** (encerramento) | S puro | S | 0 | 0 |

**Totais série**:
- 3 sub-passos.
- +5 testes E2E novos (`figure_walk_*`, `figure_paridade_*`,
  `figure_numbering_inactivo_*`, `figure_compute_labelled_*`).
- 0 testes existentes adaptados.
- 1 L0 tocado (introspect.md).
- 1 helper privado novo (`compute_figure`).
- 0 ADRs novas (ADR-0069 já ACEITE; aplicação como stylesheet).
- 0 sub-stores novos.
- 0 variants novas em `ElementPayload`.

---

## §3 Decisões arquitecturais

7 cláusulas P197A fechadas (cenário α em todas — vide
diagnóstico §2):

1. **Cláusula 1 — Forma do payload**: Cenário α — variant
   `ElementPayload::Figure { kind, counter_update, is_counted }`
   já cobre semântica desde P184B/P168.
2. **Cláusula 2 — Helper `compute_figure`**: extrair (consistência
   com pattern ADR-0069 stylesheet).
3. **Cláusula 3 — `local_figure_counters`**: walk-internal
   apenas; sem sub-store novo (comentário explícito em
   `counter_state_legacy.rs:69` documenta ausência de consumer).
4. **Cláusula 4 — Cadeia E2-E3**: preservar mutação legacy
   `state.figure_numbers.push` porque `compute_labelled`
   P195D Figure arm lê `state.figure_numbers.last()` durante
   walk. Cláusula gate substancial resolvida sem disparar gate.
5. **Cláusula 5 — Locator handling**: variante P196B
   (`emitted_loc` directo) disponível mas **não aplicada** —
   cenário α dispensa Tag pós-recursão.
6. **Cláusula 6 — Mutação legacy preservada**: write paralelo
   M5 → cleanup orgânico em M6.
7. **Cláusula 7 — Critério de fecho**: E3 fecha
   estruturalmente em P197B.

**Decisão P197B execução — helper retorna `Option<usize>`**:
`None` quando `is_counted = false` (paridade com gate legacy
`numbering.is_some() && caption.is_some()`). Diferente de
`compute_heading_auto_toc` que retorna concrete `(Label,
String)` com presença de insert mesmo em string vazia. Razão:
para Figure, ausência de número significa "não consome
número" (sem insert em legacy state); para Heading auto-toc,
ausência de numbering significa "label sintetizada sem text".
Semânticas distintas.

---

## §4 Achados não-triviais durante execução

### Achado A1 — cenário α confirmado empiricamente

P197A diagnóstico §5 confirmou empiricamente que caminho
Introspector já estava activo desde P184. 5 elementos:
- Variant `ElementPayload::Figure` materializada (P184B).
- `from_tags` arm Figure popula 4 sub-stores (kind_index,
  counters per-kind, counters global, figure_label_numbers).
- Trait método `figure_number_at_index` (P184C).
- Consumer C3 substitution-with-fallback (P184D).
- `figure_label_numbers` populated quando `is_counted &&
  label` (P168).

**Implicação**: pattern ADR-0069 não precisa de aplicação
concreta. Refactor estilístico é suficiente.

### Achado A2 — cláusula gate substancial resolvida sem disparar

P197A §6 identificou cadeia E2-E3: `compute_labelled` P195D
Figure arm (introspect.rs:344-365) lê
`state.figure_numbers.last()` durante walk. Mutação legacy
**não pode** ser removida em P197 — quebra `compute_labelled`.

**Mitigação**: preservar mutação legacy como write paralelo
M5. Cleanup orgânico em M6 quando `compute_labelled` Figure
arm migrar para CounterRegistry (location-aware lookup).

### Achado A3 — divergência from_tags arm Figure ortogonal

`from_tags` arm Figure (P184B) incrementa CounterRegistry
**independente de `is_counted`**. Para figura uncounted:
- `state.figure_numbers["image"]` fica vazio (legacy gate).
- `intr.figure_number_at_index("image", 0) = Some(1)` (CounterRegistry sem gate).

Divergência conhecida (m1-lacunas-captura.md #1) — pre-existente
desde P184B. **Não introduzida por P197B; não é responsabilidade
de P197 corrigir**. Test 4 ajustado para validar apenas
efeitos directos sobre state legacy.

### Achado A4 — `Content::Figure.numbering` é `Option<String>`

Test inicial usou `Some(EcoString::from("1"))`. Variant é
`Option<String>` (não EcoString). Corrigido para
`Some("1".to_string())`. Cláusula gate trivial.

### Achado A5 — contagem cumulativa de passos corrigida

P197B §9 reportou inicialmente "73 passos executados" (cálculo
incorrecto). Re-verificação: P196A=71, P196B=72, P196C=73,
P197A=74, **P197B=75**, P197C=76. Corrigido em §9 do
consolidado.

### Achado A6 — refactor estilístico vs migração arquitectural

Distinção formal entre:
- **Migração arquitectural** (P195D, P196B): redirecciona
  comportamento — Tag pós-recursão captura state-dependent
  payload; sub-store novo populated via from_tags.
- **Refactor estilístico** (P197B): preserva comportamento
  — helper extraído para consistência de shape; sem mudança
  semântica; sub-store equivalente já existente.

Cenário α aplica-se quando arquitectura subjacente já
satisfaz invariantes M5; refactor é puramente para
legibilidade e consistência com pattern stylesheet.

### Achado A7 — 3 variantes operacionais ADR-0069

| Variante | Aplicação | Exigência |
|----------|-----------|-----------|
| P195D | Target não-locatable | Snapshot+find_map para descobrir Location do recurse |
| P196B | Content locatable | `emitted_loc` directo do walk top |
| P197B | Cenário α | Caminho Introspector já activo; sem Tag pós-recursão |

P198 auditor decide empiricamente qual variante aplica a cada
arm (SetHeadingNumbering, CounterUpdate).

---

## §5 Estado estilístico vs activo

### Activo desde P184 (independente de P197B)

- **Caminho Introspector para figure numbering**: consumer C3
  (`mod.rs:484`, P184D) recebe `Some(n)` via
  `intr.figure_number_at_index(kind_key, idx)`. Fallback
  legacy raramente disparado.
- **`intr.kind_index[Figure]`** populated.
- **`intr.counters`** chaves `figure:{kind}` + `figure` global
  populated via `apply_at`.

### Activo desde P195D (independente de P197B)

- **`intr.resolved_labels`** populated para Figure labels via
  Tag::Labelled pós-recursão (Labelled wrapper sobre Figure).
- **`intr.figure_label_numbers`** populated quando
  `is_counted && label` (P168 + P195D combinados).

### Refactor estilístico em P197B

- Helper `compute_figure` extraído.
- Walk arm Figure invoca helper.
- Sem mudança semântica; sem novo Tag; sem novo sub-store.

### Mutação legacy preservada

- `state.figure_numbers.push` continua activo — `compute_labelled`
  P195D Figure arm lê durante walk.
- `state.local_figure_counters` continua activo — walk-internal
  trivial.
- **Cleanup orgânico em M6** quando `compute_labelled` Figure
  arm migrar para CounterRegistry (location-aware lookup
  via `flat_counter_at` ou similar).

---

## §6 Estado final M9 e M5

### Marco M9 (Introspector capabilities)

| Métrica | P196B | P197B | Δ |
|---------|-------|-------|---|
| Variants `ElementPayload` | 11 | 11 | 0 |
| Variants `ElementKind` | 9 | 9 | 0 |
| Métodos trait `Introspector` | 19 | 19 | 0 |
| Sub-stores `TagIntrospector` | 8 | 8 | 0 |
| Tests workspace | 1.843 | 1.848 | +5 |

M9 estável — refactor estilístico não introduz capabilities
novas.

### Marco M5 (walk-puro progressão)

| Arm | Estado pré-P197 | Estado pós-P197 |
|-----|-----------------|-----------------|
| Outline | migrado (P189B) | migrado |
| Bibliography | migrado (P181H) | migrado |
| Labelled | migrado estruturalmente (P195D) | migrado estruturalmente |
| Heading | migrado parcialmente (E2 → E2-residuo P196B) | inalterado |
| **Figure** | activa (E3) | **fechada estruturalmente (cenário α P197B)** |
| Equation | activa (E1) | activa |
| SetHeadingNumbering | activa (E5) | activa |
| CounterUpdate | activa (E6) | activa |

**Excepções M5 activas após P197**: 3 + 1 residuo
(E1, E2-residuo, E5, E6).

---

## §7 Estado final lacunas

| # | Lacuna | Pré-P197 | Pós-P197 |
|---|--------|----------|----------|
| #1 | Figure kind=None ↔ Introspector | activa | activa (sem trabalho em P197) |
| #1b | from_tags arm Figure sem gate `is_counted` para CounterRegistry | activa | activa (documentada como ortogonal a P197) |
| #2 | reservada | — | — |
| #3 | `headings_for_toc` sub-store ausente | activa, bloqueia E2-residuo | activa |
| #4 | reservada | — | — |
| #5 | `formatted_counter` Introspector | resolvida (P170) | resolvida |

Lacunas inalteradas em P197 — refactor estilístico não toca em
sub-stores nem from_tags.

---

## §8 Pendências cumulativas + DEBT M5-residual

### Pendências série P197

- ✅ A — diagnóstico empírico walk arm Figure.
- ✅ B — refactor walk arm Figure + helper + L0 + 5 tests.
- ✅ C — auditoria + relatório consolidado + nota DEBT.

### DEBT M5-residual — estado actualizado

> **Antes P197**: 4 excepções activas + 1 residuo (E1, E2-residuo, E3, E5, E6); 2 pré-requisitos M5-residual restantes.
>
> **Após P197B**: **3 excepções activas + 1 residuo**:
> - E1 — Reserva 1 (`Content::SetEquationNumbering` ausente).
> - **E2-residuo** — `headings_for_toc.push` (lacuna #3 bloqueia fechamento total).
> - E5 — SetHeadingNumbering walk arm.
> - E6 — CounterUpdate walk arm.
>
> **2 pré-requisitos restantes** (inalterado vs P196):
> - Sub-store `intr.headings_for_toc` (lacuna #3). **Fecha E2-residuo**.
> - `Content::SetEquationNumbering`. **Fecha E1**.
>
> **E3 fechada estruturalmente** (cenário α — caminho
> Introspector activo desde P184; refactor estilístico em
> P197B). Diferente de E2 (que ficou com residuo) e de E4
> (que fechou via Tag pós-recursão P195D).
>
> Mutação legacy preservada como write paralelo M5
> (`compute_labelled` P195D Figure arm depende); cleanup
> orgânico em M6.

**Cenário B continua** (sem DEBT formal aberto). Notas
preventivas só.

---

## §9 Próximos passos sugeridos

### Imediato (próxima série)

- **P198A — diagnóstico walks SetHeadingNumbering +
  CounterUpdate** (E5 + E6 fecham): magnitude S esperada.
  Auditor decide empiricamente que variante operacional
  aplica a cada arm — 3 variantes consolidadas:
  - P195D (não-locatable): snapshot+find_map.
  - P196B (locatable): `emitted_loc` directo.
  - P197B (cenário α): declaração formal sem Tag.

  SetHeadingNumbering é **locatable** (`extract_payload`
  retorna `Some(StateUpdate)` per P182C — já emite Tag);
  `from_tags` arm StateUpdate popula StateRegistry. Forte
  candidato a **cenário α** (paralelo a Figure P197B).

  CounterUpdate **não é locatable** atualmente (per P184A
  inventário) — se for promovido, candidato a P196B
  variante; se cenário α aplicar (Tag StateUpdate
  via SetEquationNumbering quando materializado),
  candidato a cenário α com pré-requisito.

### Encadeamento M5 universal

- **Passo dedicado abrir sub-store `intr.headings_for_toc`**
  (fora série P196/P197/P198): fecha **E2-residuo**.
  Decisão pendente sobre estrutura (Vec vs variant em Tag).
- **Materialização `Content::SetEquationNumbering`** (passo
  dedicado, paralelo a P198): fecha E1.

### Após sequência completa

- Walk torna-se universalmente puro.
- M5 fecha.
- Segue **M6** (P200 ou P190): eliminação de
  `CounterStateLegacy` + remoção de mutações legacy paralelas
  (`state.figure_numbers.push`, `state.headings_for_toc.push`,
  etc.).

---

## §10 Linhagem

- **Pattern arquitectural stylesheet**: ADR-0069 (PROPOSTO em
  P195B, ACEITE em P195E).
- **3 variantes operacionais consolidadas**:
  - P195D variante (não-locatable): snapshot+find_map.
  - P196B variante (locatable): `emitted_loc` directo.
  - P197B variante (cenário α): declaração formal sem Tag pós-recursão.
- **Helpers análogos** — 3 na família ADR-0069 stylesheet:
  `compute_labelled` (P195D) + `compute_heading_auto_toc`
  (P196B) + `compute_figure` (P197B).
- **Sub-stores consumidos** (todos pre-existentes):
  `intr.counters` (CounterRegistry P184B); `intr.kind_index`
  (P162); `intr.figure_label_numbers` (P168); `intr.resolved_labels`
  (P193B + P195D); `intr.labels` (P162).
- **Consumer C3**: `references.rs::layout_ref` figure ref-arm
  (P184D substitution-with-fallback) — inalterado em P197.
- **Consumer C4**: `references.rs::layout_ref` text ref-arm
  (P194B substitution-with-fallback) — inalterado em P197.
- **L0 tocado**: `00_nucleo/prompts/rules/introspect.md`
  hash `b9f78ff9`.
- **Código tocado**: `01_core/src/rules/introspect.rs`
  hash `c938c001`.
- **Padrão diagnóstico-primeiro**: 19ª aplicação consecutiva
  (P197A diagnóstico antes de P197B refactor).

---

## §11 Métricas finais

- **Sub-passos**: 3 (A diagnóstico + B refactor + C encerramento).
- **LOC produção**: ~15 (helper) + ~10 (refactor walk arm) = ~25.
- **LOC teste**: ~150 (5 tests sentinela).
- **LOC L0**: ~70 (secção nova "Walk arm Figure migrado P197B" + actualização tabela Excepções + lista ordem inversa).
- **LOC relatórios**: ~750 (P197A diagnóstico + P197A relatório + P197B relatório + P197 consolidado).
- **Variants ElementPayload novas**: 0.
- **Sub-stores novos**: 0.
- **ADRs novas**: 0.
- **Excepções M5 fechadas**: 1 (E3 — fechada estruturalmente cenário α).
- **Tests netos adicionados**: +5.
- **Hashes desactualizados**: 1 → 0 (corrigido por `--fix-hashes` em P197B).
- **76 passos executados** (contagem corrigida — P196A=71, P196B=72, P196C=73, P197A=74, P197B=75, P197C=76).

---

## §12 Notas operacionais

- **Tamanho série**: ~250 LOC produção/tests + ~750 LOC documentação.
- **Sem dependências externas novas**.
- **Sem ADR; sem DEBT formal aberto**.
- **Padrão replicado**: encerramento série P186/P187/P188/P189/P193/P194/P195/P196 (relatório consolidado 9 secções padrão).
- **Cláusulas gate disparadas**: 0 substanciais (cláusula gate substancial cadeia E2-E3 resolvida sem disparar via cenário α + write paralelo).
- **Cláusulas gate triviais resolvidas**: 2 (test 4 ajuste; EcoString → String).

**Próximo passo**: **P198A** — diagnóstico walks SetHeadingNumbering + CounterUpdate. Magnitude S esperada para diagnóstico; implementação P198B+ depende de qual variante operacional aplica a cada arm (3 disponíveis).
