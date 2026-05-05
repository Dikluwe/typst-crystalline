# Relatório Consolidado — Série P200

**Data**: 2026-05-04
**Sub-passos**: A ✅ B ✅ C ✅
**Magnitude agregada**: S diagnóstico (P200A) + M+ trabalho híbrido (P200B) + S documental (P200C)
**Estado**: Série fechada.
**Pattern arquitectural**: ADR-0069 stylesheet — 7ª aplicação concreta; trabalho híbrido sem nova variante operacional.
**MARCO ARQUITECTURAL**: **M5 universal completo pela primeira vez desde declaração em P189B**.

---

## §1 Resumo executivo

Série P200 fecha **E2-residuo** + **lacuna #3**
simultaneamente via **trabalho híbrido** combinando 3 padrões
testados — sub-store novo (P193B-style) + variant Tag
pós-recursão (variante P196B) + consumer migration
substitution-with-fallback (P184D / P194B-style). **Sem nova
variante operacional ADR-0069** — combinação directa de
variantes existentes.

**MARCO ARQUITECTURAL — M5 universal completo**:
- **0 excepções activas**.
- **0 residuos**.
- **0 pré-requisitos restantes**.
- Todos walk arms cristalinos fechados estruturalmente.

**Desbloqueia M6** (P190A reescrita do zero — eliminação
`CounterStateLegacy`; magnitude L cross-modular).

**Output observable em produção**: inalterado —
substitution-with-fallback no consumer outline garante
paridade; sub-store fornece dados; legacy fica funcional como
backup.

---

## §2 Sub-passos materializados

| Sub | Magnitude planeada | Magnitude real | Δ tests | L0s tocados |
|-----|-------------------|----------------|---------|-------------|
| **P200A** (diagnóstico) | S puro | S | 0 | 0 |
| **P200B** (trabalho híbrido) | M+ | M+ | +5 | 1 (`introspect.md`) |
| **P200C** (encerramento) | S puro | S | 0 | 0 |
| **Total série** | **M+** agregado | **M+** | **+5** | **1 L0 distinto** |

**Totais**:
- 3 sub-passos.
- +5 testes E2E novos + 4 testes P196B adaptados.
- 1 sub-store TagIntrospector novo (`headings_for_toc`).
- 1 trait method novo (`headings_for_toc()`).
- 1 ElementPayload variant nova (`HeadingForToc`).
- 0 ElementKind variants novas (HeadingForToc é Tag derivada).
- 1 helper privado novo (4º na família ADR-0069 stylesheet).
- 0 ADRs novas.
- 0 sub-stores adicionais.

---

## §3 Decisões arquitecturais

9 cláusulas P200A fechadas:

1. **Cláusula 1 — Forma sub-store**: Opção α — `Vec<(Label, Content, usize)>` literal (corrigido de `u8` em P200A §1.4).
2. **Cláusula 2 — Variant Tag**: Opção α — `ElementPayload::HeadingForToc` nova.
3. **Cláusula 3 — `is_locatable`**: sem arm — Tag pós-recursão usa `emitted_loc` Heading.
4. **Cláusula 4 — Walk arm**: 3ª Tag pós-recursão após Tag Labelled auto-toc P196B.
5. **Cláusula 5 — Helper**: `compute_heading_for_toc` (4º na família ADR-0069).
6. **Cláusula 6 — `from_tags` arm**: push directo em sub-store.
7. **Cláusula 7 — Trait method**: `headings_for_toc()` (19→20).
8. **Cláusula 8 — Consumer outline**: substitution-with-fallback.
9. **Cláusula 9 — Critério fecho**: E2-residuo + lacuna #3 fecham; **M5 universal completo**.

**Decisões de execução notáveis em P200B**:

- **`ElementKind::HeadingForToc` NÃO criada** (P200B §7): HeadingForToc é Tag derivada de Heading, análoga a Tag::Labelled auto-toc P196B (também sem ElementKind). Justificação inline em L0.
- **Helper sempre retorna `Some`** (P200B §7): re-verificação empírica revelou que mutação 4 legacy é incondicional (introspect.rs:486 pre-P200B faz push sempre). Helper segue paridade.
- **`frozen_body` clonado para reuso** (P200B §7): walk arm pre-P200B fazia move; agora clona porque Tag::HeadingForToc emit precisa do body. Custo aceitável (Content é Arc-internamente em variants compostos).
- **Apenas 4 tests P196B regridem** (não 5 projectado em P200A): `end_hash_distingue_conteudo` já filtra `hash != 0` e mantém-se válido.

---

## §4 Achados não-triviais durante execução

### Achado A1 — Type signature `usize` (não `u8`)

P200A §1.4 corrigiu instrução do prompt original que mencionava `u8`. Empíricamente `state.headings_for_toc: Vec<(Label, Content, usize)>` (`counter_state_legacy.rs:42`). Sub-store novo segue paridade.

### Achado A2 — Layouter assignments duais

P200A §1.6 descobriu que Layouter faz `l.counter.headings_for_toc = initial_state.headings_for_toc` em **2 lugares** (`mod.rs:1490, 1521`). Write paralelo M5 obrigatório porque ambos assignments dependem de state legacy.

### Achado A3 — Helper sempre retorna `Some`

P200A diagnostic §2 cláusula 5 inicialmente sugeriu gate por `is_numbering_active("heading")`. P200B §7 corrigiu — mutação 4 legacy é incondicional (push sempre); helper segue paridade. Test 3 do diagnostic §8 (`headings_for_toc_numbering_inactivo_nao_emite_tag`) substituído por `bracketing_valido_6_tags_por_heading_p200b`.

### Achado A4 — `ElementKind::HeadingForToc` NÃO criada

Decisão deferida a empírica em P200A §1.7 e §2 cláusula 6. Resolução em P200B: HeadingForToc é Tag **derivada** de Heading (não Content standalone parsável); precedente P196B (Labelled auto-toc também sem ElementKind correspondente). Sub-store dedicado é caminho de query directo via trait method.

### Achado A5 — `frozen_body` clonado

Walk arm pre-P200B fazia `state.headings_for_toc.push((auto_label, frozen_body, level))` movendo `frozen_body`. Para reusar em Tag::HeadingForToc emit, P200B clona: `state.headings_for_toc.push((auto_label.clone(), frozen_body.clone(), level))`. Content tem variants compostos com Arc — clone é O(1) para vias importantes.

### Achado A6 — Apenas 4 tests P196B regridem

P200A diagnostic §7 projectou 5 tests adaptados. Empíricamente apenas 4: `end_hash_distingue_conteudo` já filtra `hash != 0` e mantém-se válido com qualquer número de Tag::End. Padrão pragmático auditor #1 aplicado.

### Achado A7 — Trabalho híbrido sem nova variante operacional ADR-0069

P200B combina **3 padrões testados** (sub-store novo P193B-style + variant Tag pós-recursão variante P196B + consumer migration substitution-with-fallback P184D/P194B-style). **Sem nova variante operacional ADR-0069** — pattern stylesheet 5 variantes operacionais permanecem inalteradas.

---

## §5 Estado activo vs preservado

### Activado em P200B

- **Caminho Introspector para `headings_for_toc`**: sub-store populated via Tag::HeadingForToc emitida pelo walk arm Heading pós-recursão.
- **Consumer outline.rs:24 first branch activa** em produção real: `intr.headings_for_toc()` retorna `Some(entries)`; substitution-with-fallback fornece backup raramente disparado.
- **Trait method novo** `headings_for_toc()` exposto a consumers M5+.

### Mutação legacy preservada (write paralelo M5)

- **Walk arm Heading mutação 4** (`state.headings_for_toc.push((auto_label, frozen_body, level))`): incondicional. Clone do `frozen_body` para reuso em Tag::HeadingForToc emit.
- **Layouter assignments** (`mod.rs:1490, 1521`): `l.counter.headings_for_toc = initial_state.headings_for_toc.clone()` — write paralelo dual obrigatório.

### Cleanup orgânico em M6 (P190A reescrita do zero)

Quando Layouter migrar para Introspector path completo
(`l.counter` deixa de receber assignment do legacy), mutação 4
walk arm pode ser removida; consumer outline removerá fallback
legacy.

---

## §6 Estado final M9 e M5

### Marco M9 (Introspector capabilities)

| Métrica | P199B | P200B | Δ |
|---------|-------|-------|---|
| Variants `ElementPayload` | 12 | **13** | +1 (HeadingForToc) |
| Variants `ElementKind` | 10 | 10 | 0 |
| Métodos trait `Introspector` | 19 | **20** | +1 (headings_for_toc) |
| Sub-stores `TagIntrospector` | 8 | **9** | +1 (headings_for_toc) |
| Variants `Content` | +1 P199B | +1 P199B | 0 (em P200) |
| Tests workspace | 1.864 | **1.869** | +5 |

### Marco M5 (walk-puro progressão) — **COMPLETO**

| Arm | Estado pré-P200 | Estado pós-P200 |
|-----|-----------------|-----------------|
| Outline | migrado (P189B) | migrado |
| Bibliography | migrado (P181H) | migrado |
| Labelled | fechada estruturalmente (P195D) | fechada estruturalmente |
| Heading | parcial (E2-residuo P196B) | **fechada estruturalmente** (P196B 3/4 + **P200B 4/4**) |
| Figure | fechada estruturalmente (P197B) | fechada estruturalmente |
| SetHeadingNumbering | fechada estruturalmente (P198B) | fechada estruturalmente |
| CounterUpdate | fechada estruturalmente (P198C) | fechada estruturalmente |
| SetEquationNumbering | fechada estruturalmente (P199B) | fechada estruturalmente |

**Excepções M5 activas após P200**: **0 + 0 residuos + 0 pré-requisitos**.

---

## §7 Estado final lacunas

| # | Lacuna | Pré-P200 | Pós-P200 |
|---|--------|----------|----------|
| #1 | Figure kind=None ↔ Introspector | activa | activa (ortogonal a M5) |
| #1b | from_tags arm Figure sem gate `is_counted` | activa | activa (ortogonal) |
| #2 | reservada | — | — |
| #3 | `headings_for_toc` sub-store ausente | activa, bloqueia E2-residuo | **fechada (P200B)** |
| #4 | reservada | — | — |
| #5 | `formatted_counter` Introspector | resolvida (P170) | resolvida |

Lacunas #1 + #1b ortogonais a M5; permanecem activas. **Não bloqueiam M6**.

---

## §8 Pendências cumulativas + DEBT M5-residual + DEBT M6 documentação

### Pendências série P200

- ✅ A — diagnóstico empírico sub-store + 9 cláusulas fechadas.
- ✅ B — trabalho híbrido (sub-store + Tag + consumer + helper + L0 + 5 tests + 4 adaptações).
- ✅ C — auditoria + relatório consolidado + DEBT.

### DEBT M5-residual — actualizada

> **Antes P200**: 0 excepções activas + 1 residuo (E2-residuo); 1 pré-requisito paralelo restante (lacuna #3).
>
> **Após P200B**: **0 excepções activas + 0 residuos + 0 pré-requisitos**.
>
> **MARCO ARQUITECTURAL — M5 universal completo pela primeira vez desde declaração em P189B**:
> - Todos walk arms fechados estruturalmente.
> - 6 excepções fechadas + 1 residuo fechado + 2 pré-requisitos paralelos materializados.
> - Pattern ADR-0069 com 5 variantes operacionais consolidadas; 7 aplicações concretas; 4 helpers privados.
> - Trait `Introspector`: 19 → 20 métodos.
> - `ElementPayload`: 12 → 13 variants.
> - `TagIntrospector` sub-stores: 8 → 9.
>
> **Cenário B continua para M5** (sem DEBT formal aberto; fechado naturalmente após P200B).

### DEBT M6 documentação (informacional — não-formal)

> **Write paralelo M5 ainda activo** em todos walk arms fechados estruturalmente:
> - Mutações legacy preservadas em walk arms Heading (4 mutações), Figure (2 mutações), SetHeadingNumbering (1 mutação), CounterUpdate (3 caminhos), SetEquationNumbering (1 mutação), Labelled (2 mutações), etc.
> - 4 helpers privados `compute_*` (P195D `compute_labelled`, P196B `compute_heading_auto_toc`, P197B `compute_figure`, P200B `compute_heading_for_toc`) leem `state` legacy durante walk.
> - Layouter assignments (`mod.rs:1490, 1521`) dependem de `state.headings_for_toc`.
> - Layouter consumers fazem read directo de `state.numbering_active`, `state.flat`, `state.hierarchical`, `state.figure_numbers`, etc.
> - `CounterStateLegacy` struct ainda existe em `01_core/src/entities/counter_state_legacy.rs`.
>
> **Cleanup orgânico em M6 (P190A reescrita do zero)** — magnitude L cross-modular esperada:
> - Eliminar mutações legacy em walk arms.
> - Migrar/eliminar 4 helpers `compute_*` (substituir por queries Introspector location-aware).
> - Migrar Layouter assignments para Introspector path completo.
> - Eliminar struct `CounterStateLegacy`.
> - Adaptar consumers que ainda dependem de legacy.
>
> P190A original (`typst-passo-185a-relatorio.md` renomeado em série P185) declarado obsoleto — escrever do zero baseado no estado consolidado pós-P200B.

---

## §9 Próximos passos sugeridos

**M5 universal fechado**. 3 opções estratégicas:

### Opção 1 — Pausa estratégica

Boa altura para reflectir antes de M6 (refactor maior cross-modular). Permite consolidar lições e planear M6 com escopo claro.

### Opção 2 — Iniciar P190A reescrita do zero (M6)

**Eliminação `CounterStateLegacy`**. Magnitude **L** cross-modular esperada. Trabalho previsto:
- Eliminar mutações legacy em walk arms (Heading 4, Figure 2, Set/Counter arms, etc.).
- Migrar/eliminar 4 helpers `compute_*` (substituir por queries Introspector location-aware via `flat_counter_at`, `formatted_counter_at`, `is_numbering_active_at`).
- Migrar Layouter assignments (`mod.rs:1490, 1521` + outros consumers Layouter directos).
- Eliminar struct `CounterStateLegacy`.
- Adaptar consumers que ainda dependem de legacy (Layouter Equation, Heading rendering, Figure rendering, outline, etc.).

P190A original (`typst-passo-185a-relatorio.md`) declarado obsoleto. P190A será **reescrita do zero** baseada no estado consolidado pós-P200B.

### Opção 3 — Lacunas residuais (#1, #1b)

Passo dedicado paralelo se desejado. **Ortogonais a M6** — não bloqueiam P190A. Lacuna #1 (Figure kind=None ↔ Introspector) + lacuna #1b (from_tags arm Figure sem gate `is_counted`) podem ser endereçadas independentemente em qualquer altura.

---

## §10 Marco arquitectural — M5 universal completo

**Primeira vez desde declaração em P189B**.

### Histórico

| Série | Data | Estado | Excepção fechada |
|-------|------|--------|------------------|
| **P189B** (declaração) | ~2026-05-02 | 6 excepções + bibliografia em desenvolvimento | (declaração) |
| P193 (sub-store ResolvedLabelStore) | 2026-05-03 | infraestrutura | (pré-requisito) |
| P194 (consumer C4 migrado) | 2026-05-03 | infraestrutura | (pré-requisito) |
| P195 (E4 fechada — Labelled) | 2026-05-03 | E4 fechada | E4 |
| P196 (E2 → E2-residuo — Heading auto-toc) | 2026-05-03 | E2 parcial (3/4) | E2 (3/4) |
| P197 (E3 fechada — Figure cenário α) | 2026-05-04 | E3 fechada | E3 |
| P198 (E5+E6 fechadas — Set+Counter) | 2026-05-04 | sequência §9 P189 cumprida | E5 + E6 |
| P199 (E1 fechada — SetEquationNumbering) | 2026-05-04 | **0 excepções activas pela primeira vez** | E1 |
| **P200** (E2-residuo + lacuna #3 fechadas) | 2026-05-04 | **M5 universal COMPLETO** | E2-residuo |

**9 séries materializadas** entre declaração P189B e fecho P200B.

### 5 variantes operacionais ADR-0069 consolidadas

Catálogo arquitectural completo para futuras aplicações:

| Variante | Aplicação | Pré-passo | Trabalho |
|----------|-----------|-----------|----------|
| **P195D variante** | Não-locatable | Caminho inactivo | Tag pós-recursão + snapshot+find_map |
| **P196B variante** | Locatable + body | Caminho inactivo | Tag pós-recursão + emitted_loc directo |
| **Cenário α** (P197B Figure, P198B SetHeadingNumbering) | Caminho activo | Refactor estilístico ou declaração formal |
| **Cenário α por construção** (P199B SetEquationNumbering) | Caminho activável | Materializar variant — caminho activa imediatamente |
| **Cenário β-promote** (P198C CounterUpdate) | Caminho inactivo | Promote completo (variant + locatable + 2 arms) |

### 7 aplicações concretas ADR-0069 stylesheet

| # | Aplicação | Variante operacional |
|---|-----------|---------------------|
| 1 | P195D Labelled | P195D variante (não-locatable) |
| 2 | P196B Heading auto-toc | P196B variante (locatable + body) |
| 3 | P197B Figure | Cenário α |
| 4 | P198B SetHeadingNumbering | Cenário α |
| 5 | P198C CounterUpdate | Cenário β-promote (1ª aplicação) |
| 6 | P199B SetEquationNumbering | Cenário α por construção (1ª aplicação) |
| 7 | **P200B Heading mutação 4** | **Trabalho híbrido** (combinação 3 padrões) |

### 4 helpers privados família ADR-0069

| Helper | Passo | Assinatura |
|--------|-------|------------|
| `compute_labelled` | P195D | `(target, state) → (Option<String>, Option<usize>)` |
| `compute_heading_auto_toc` | P196B | `(state, n) → (Label, String)` |
| `compute_figure` | P197B | `(state, kind, is_counted) → Option<usize>` |
| `compute_heading_for_toc` | **P200B** | `(state, frozen_body, level) → Option<(Label, Content, usize)>` |

### Ferramentas arquitecturais activadas durante M5

- Sub-stores `TagIntrospector`: 8 → 9 (`headings_for_toc`).
- Variants `ElementPayload`: 12 → 13 (`HeadingForToc`).
- Métodos trait `Introspector`: 19 → 20 (`headings_for_toc()`).
- Variants `Content`: + 1 (`SetEquationNumbering`).

### Padrão diagnóstico-primeiro confirmado em escala

- **22 aplicações consecutivas** sem falhar magnitude planeada ±1 nível.
- **0 cláusulas substanciais disparadas** em séries P195-P200.
- **Reservas mantidas explícitas** (Reserva 1 = SetEquationNumbering) endereçadas no momento apropriado (P199B após >12 séries).
- **Cláusulas gate substanciais resolvidas sem disparar** em todas as séries (cadeia E2-E3, cadeia E5/E6 ↔ helpers compute_*, cadeia E1).

### Após M5 universal completo

**Desbloqueia M6 (P190A reescrita do zero — eliminação `CounterStateLegacy`; magnitude L cross-modular)**.

P190A original (em `typst-passo-185a-relatorio.md`) declarado **obsoleto** — escrever do zero baseado no estado consolidado pós-P200B. Trabalho previsto inclui eliminação de mutações legacy em walk arms, migração/eliminação de 4 helpers `compute_*`, migração de Layouter assignments, eliminação do struct `CounterStateLegacy`, e adaptação de consumers que ainda dependem de legacy.

---

## §11 Linhagem

- **Pattern arquitectural stylesheet**: ADR-0069 (PROPOSTO P195B; ACEITE P195E).
- **5 variantes operacionais ADR-0069 consolidadas**.
- **7 aplicações concretas ADR-0069 stylesheet**: P195D + P196B + P197B + P198B + P198C + P199B + **P200B**.
- **Trabalho híbrido P200B**: combinação directa de 3 padrões testados (sub-store P193B + Tag pós-recursão variante P196B + consumer migration P184D/P194B). **Sem nova variante operacional**.
- **4 helpers privados família ADR-0069**.
- **Sub-store novo**: `intr.headings_for_toc` (9º).
- **Trait method novo**: `headings_for_toc()` (20º).
- **Variant nova**: `ElementPayload::HeadingForToc` (13ª).
- **Consumer migrado**: `outline.rs:24` (3ª migration substitution-with-fallback).
- **Cadeia E2-residuo**: walk arm Heading mutação 4 → mut 4 preservada; Tag::HeadingForToc emite payload com body materializado.
- **L0 tocado**: `00_nucleo/prompts/rules/introspect.md` hash `7a3ba2b7`.
- **Código tocado**: 5 ficheiros `01_core/src/`:
  - `entities/introspector.rs` (sub-store + trait method + impl).
  - `entities/element_payload.rs` (variant nova).
  - `rules/introspect.rs` (helper + walk arm; hash `8e0128e4`).
  - `rules/introspect/from_tags.rs` (arm novo).
  - `rules/layout/outline.rs` (consumer migration).

---

## §12 Métricas finais

- **Sub-passos**: 3 (A diagnóstico + B trabalho híbrido + C encerramento + marco).
- **LOC produção**: ~120 (sub-store + trait + variant + helper + walk arm + from_tags + consumer).
- **LOC teste**: ~180 (5 tests novos + 4 adaptações P196B).
- **LOC L0**: ~110 (secção nova "Walk arm Heading mutação 4 fechada"; secção "Marco M5 universal completo"; tabela Excepções + ordem inversa).
- **LOC relatórios**: ~1.300 (P200A diagnóstico + P200A relatório + P200B relatório + P200 consolidado com §10 marco).
- **Variants Content novas em P200**: 0 (P199B teve +1).
- **Variants ElementPayload novas**: +1 (`HeadingForToc`).
- **Variants ElementKind novas**: 0 (HeadingForToc é Tag derivada).
- **Sub-stores novos**: +1 (`headings_for_toc`).
- **Trait methods novos**: +1 (`headings_for_toc()`).
- **Helpers privados novos**: +1 (4º na família ADR-0069).
- **ADRs novas**: 0.
- **Excepções M5 fechadas em P200**: 1 (E2-residuo).
- **Lacunas fechadas em P200**: 1 (#3).
- **Tests netos adicionados**: +5.
- **Hashes desactualizados**: 0 (corrigidos por `--fix-hashes` em P200B).

**Marco arquitectural final**: M5 universal completo.
