# Relatório Consolidado — Série P198

**Data**: 2026-05-04
**Sub-passos**: A ✅ B ✅ C ✅ D ✅
**Magnitude agregada**: S diagnóstico (P198A) + S declaração (P198B) + M promote (P198C) + S documental (P198D)
**Estado**: Série fechada
**Pattern arquitectural**: ADR-0069 stylesheet — 4ª variante operacional consolidada (cenário β-promote).

---

## §1 Resumo executivo

Série P198 fecha **E5** (`Content::SetHeadingNumbering`) e
**E6** (`Content::CounterUpdate`) — as últimas 2 excepções
M5 não-residuais. **Marco arquitectural**: sequência §9 P189
está cumprida na totalidade após 6 séries materializadas
(P193 → P198).

**M5 universal a 2 pré-requisitos paralelos do fecho**:
- **E1** (Equation) ↔ `Content::SetEquationNumbering` materialização (passo independente, fora série §9).
- **E2-residuo** (Heading `headings_for_toc`) ↔ sub-store `intr.headings_for_toc` (lacuna #3).

Característica distintiva: **2 variantes operacionais
diferentes** consolidadas dentro da mesma série:

| Sub-passo | Variante | Caminho Introspector pré-passo |
|-----------|----------|-------------------------------|
| **P198B** (E5) | Cenário α | Activo desde P182C |
| **P198C** (E6) | **Cenário β-promote** (1ª aplicação) | **Inactivo** — promote completo |

P198C consolida a 4ª variante operacional ADR-0069: nova
variant em `ElementPayload`, novo `ElementKind`, promote a
locatable, 2 arms novos (`extract_payload` + `from_tags`).

**Output observable em produção**: inalterado (write paralelo
legacy preservado em ambos arms).

---

## §2 Sub-passos materializados

| Sub | Magnitude planeada | Magnitude real | Δ tests | L0s tocados |
|-----|-------------------|----------------|---------|-------------|
| **P198A** (diagnóstico) | S puro | S | 0 | 0 |
| **P198B** (cenário α — E5) | S | S | +5 | 1 (`introspect.md`) |
| **P198C** (β-promote — E6) | M | M | +6 | 3 (`introspect.md`, `element_payload.md`, `element_kind.md`) |
| **P198D** (encerramento) | S puro | S | 0 | 0 |
| **Total série** | **M-** agregado | **M-** | **+11** | **3 L0s distintos** |

**Totais**:
- 4 sub-passos.
- +11 testes E2E novos.
- 0 testes existentes adaptados.
- 1 ElementPayload variant nova (`CounterUpdate`).
- 1 ElementKind variant nova (`CounterUpdate`).
- 3 helpers privados na família ADR-0069 stylesheet (inalterado — sem helpers novos em P198).
- 0 ADRs novas (ADR-0069 já ACEITE).
- 0 sub-stores novos (CounterRegistry P184B reusado para CounterUpdate).

---

## §3 Decisões arquitecturais

9 cláusulas P198A fechadas:

1. **Cláusula 1 — Variante E5**: cenário α (paralelo a Figure
   P197B). Caminho Introspector activo desde P182C.
2. **Cláusula 2 — Variante E6**: cenário β-promote (1ª
   aplicação). Promove + variant nova + 2 arms.
3. **Cláusula 3 — Helpers**: 0 novos. P198B mutação trivial
   1 linha; P198C lógica em `from_tags` arm próprio.
4. **Cláusula 4 — Ordem**: Opção β — sub-passos separados
   (variantes diferentes; magnitudes divergentes).
5. **Cláusula 5 — Cadeia E5/E6 ↔ helpers compute_***:
   preservar mutações legacy. Cláusula gate substancial
   resolvida sem disparar gate.
6. **Cláusula 6 — E1 interacção**: independente; sem trabalho
   em P198.
7. **Cláusula 7 — Mutação legacy**: write paralelo M5 →
   cleanup orgânico em M6.
8. **Cláusula 8 — Critério fecho**: E5+E6 estruturalmente; M5
   universal NÃO fecha (E1 + E2-residuo restam).
9. **Cláusula 9 — Plano**: P198B (S) + P198C (M) + P198D (S) =
   M- agregado.

**Decisões adicionais durante execução**:

- **`ElementKind::CounterUpdate` adicionada** (P198C §7): convenção cristalino — todo `ElementPayload` locatable tem `ElementKind` correspondente. 10ª variant.
- **Sem helpers estilísticos** (P198B §7 + P198C §7): mutação trivial em E5; lógica de mapeamento em `from_tags` arm próprio em E6.
- **Reuso enum `CounterUpdate`** (P198C §3): `action: CounterUpdate` reusa enum de `counter_update.rs` (P161 rename de CounterAction). Namespacing Rust resolve colisão nominal (`ElementPayload::CounterUpdate` vs `crate::entities::counter_update::CounterUpdate`).

---

## §4 Achados não-triviais durante execução

### Achado A1 — estados divergentes confirmados (P198A §5)

E5 cenário α: caminho Introspector activo desde P182C
(StateRegistry populated via Tag::StateUpdate).
E6 cenário β-promote: caminho Introspector NÃO existia
pré-P198C (CounterRegistry não populated via CounterUpdate).
**2 variantes diferentes na mesma série** — primeira ocorrência.

### Achado A2 — sem helper estilístico em P198B (§7)

Mutação E5 é 1 linha (`state.numbering_active.insert("heading", *active)`).
Helper estilístico não acrescenta valor. Cenário α aceita
ambas formas (com helper P197B / sem helper P198B).

### Achado A3 — reuso enum CounterUpdate (P198C §3)

`ElementPayload::CounterUpdate { key: String, action: CounterUpdate }`
reusa enum existente (P161 rename de CounterAction). Namespacing
Rust mantém `ElementPayload::CounterUpdate` (variant) e
`CounterUpdate` (enum) distintos.

### Achado A4 — paridade exacta walk legacy ↔ from_tags (P198C §4)

3 caminhos da match em walk arm legacy mapeiam exactamente
para 3 caminhos no `from_tags` arm:
- `state.step_hierarchical("heading", 1)` ↔ `apply_hierarchical_at("heading", 1, loc)`.
- `state.step_flat(key)` ↔ `apply_at(key, Step, loc)`.
- `state.update_flat(key, val)` ↔ `apply_at(key, Update(val), loc)`.

### Achado A5 — `ElementKind::CounterUpdate` per convenção (P198C §7)

Convenção cristalino: todo `ElementPayload` locatable tem
`ElementKind` correspondente. 10ª variant adicionada;
`as_str()` retorna `"counter_update"`; `from_name(...)` activo.

### Achado A6 — import necessário em from_tags (P198C §7)

`from_tags.rs` não importava `CounterUpdate` enum no scope do
módulo (só em `mod tests`). Necessário importar para arm novo.
Cláusula gate trivial.

### Achado A7 — distinção cenário α vs β-promote consolidada

P198 introduz **distinção formal** entre cenário α (caminho
activo; refactor/declaração) e cenário β-promote (caminho
inactivo; promote completo). 4 variantes operacionais
ADR-0069 agora documentadas.

---

## §5 Estado activo vs preservado

### Activo desde P182C (E5)

- **Caminho Introspector para SetHeadingNumbering**:
  StateRegistry populated via Tag::StateUpdate (chave canónica
  `numbering_active:heading`).
- Consumer C5 via `is_numbering_active` — caminho Introspector
  funcional desde antes de P198B.

### Activado em P198C (E6)

- **Caminho Introspector para CounterUpdate**: CounterRegistry
  populated via Tag::CounterUpdate (3 caminhos `apply_at` /
  `apply_hierarchical_at`).
- `kind_index[ElementKind::CounterUpdate]` populated.
- Variante β-promote 1ª aplicação concreta.

### Mutação legacy preservada (write paralelo M5)

- **SetHeadingNumbering**: 1 mutação preservada
  (`state.numbering_active.insert`). Necessária porque
  `compute_heading_auto_toc` (P196B) + walk arm Equation lêem
  `state.is_numbering_active` durante walk.
- **CounterUpdate**: 3 mutações preservadas
  (`state.step_hierarchical`, `state.step_flat`,
  `state.update_flat`). Necessárias porque `compute_*` helpers
  (P195D Equation, P196B Heading auto-toc, P197B Figure) lêem
  `state.flat`/`hierarchical` durante walk.

### Cleanup orgânico em M6 (P190/P200)

Quando `compute_*` helpers migrarem para sub-stores Introspector
location-aware (`flat_counter_at`, `formatted_counter_at`,
`is_numbering_active_at`) ou eliminarem-se com remoção de
`CounterStateLegacy`, mutações legacy podem ser removidas.

---

## §6 Estado final M9 e M5

### Marco M9 (Introspector capabilities)

| Métrica | P197B | P198C | Δ |
|---------|-------|-------|---|
| Variants `ElementPayload` | 11 | **12** | +1 (CounterUpdate) |
| Variants `ElementKind` | 9 | **10** | +1 (CounterUpdate) |
| Métodos trait `Introspector` | 19 | 19 | 0 |
| Sub-stores `TagIntrospector` | 8 | 8 | 0 |
| Tests workspace | 1.848 | **1.859** | +11 |

### Marco M5 (walk-puro progressão)

| Arm | Estado pré-P198 | Estado pós-P198 |
|-----|-----------------|-----------------|
| Outline | migrado (P189B) | migrado |
| Bibliography | migrado (P181H) | migrado |
| Labelled | migrado estruturalmente (P195D) | migrado estruturalmente |
| Heading | migrado parcialmente (E2 → E2-residuo P196B) | inalterado (E2-residuo persiste) |
| Figure | fechada estruturalmente (P197B) | fechada estruturalmente |
| **SetHeadingNumbering** | activa (E5) | **fechada estruturalmente (P198B)** |
| **CounterUpdate** | activa (E6) | **fechada estruturalmente (P198C)** |
| Equation | activa (E1) | activa (independente — pré-requisito SetEquationNumbering) |

**Excepções M5 activas após P198**: **1 + 1 residuo**
(E1, E2-residuo). Ambos pré-requisitos paralelos fora série
§9 P189.

---

## §7 Estado final lacunas

| # | Lacuna | Pré-P198 | Pós-P198 |
|---|--------|----------|----------|
| #1 | Figure kind=None ↔ Introspector | activa | activa |
| #1b | from_tags arm Figure sem gate `is_counted` | activa | activa |
| #2 | reservada | — | — |
| #3 | `headings_for_toc` sub-store ausente | activa, bloqueia E2-residuo | activa |
| #4 | reservada | — | — |
| #5 | `formatted_counter` Introspector | resolvida (P170) | resolvida |

Lacunas inalteradas em P198 — fecho de E5+E6 ortogonal a
lacunas existentes.

---

## §8 Pendências cumulativas + DEBT M5-residual

### Pendências série P198

- ✅ A — diagnóstico empírico walks SetHeadingNumbering + CounterUpdate.
- ✅ B — declaração formal cenário α para E5 + 5 tests + L0.
- ✅ C — promote cenário β-promote para E6 + 6 tests + 3 L0s.
- ✅ D — auditoria + relatório consolidado + nota DEBT.

### DEBT M5-residual — estado actualizado

> **Antes P198**: 3 excepções activas + 1 residuo (E1, E2-residuo, E5, E6); 2 pré-requisitos M5-residual restantes.
>
> **Após P198C**: **1 excepção activa + 1 residuo**:
> - E1 — Reserva 1 (`Content::SetEquationNumbering` ausente).
> - **E2-residuo** — `headings_for_toc.push` (lacuna #3 bloqueia fechamento total).
>
> **2 pré-requisitos restantes** (inalterado vs P195/P196/P197):
> - Sub-store `intr.headings_for_toc` (lacuna #3). **Fecha E2-residuo**.
> - `Content::SetEquationNumbering`. **Fecha E1**.
>
> **E5 fechada estruturalmente** (P198B — cenário α; caminho Introspector activo desde P182C).
> **E6 fechada estruturalmente** (P198C — cenário β-promote 1ª aplicação; promote completo).
>
> **Sequência §9 P189 cumprida na totalidade**: P193 → P194 → P195 → P196 → P197 → P198. M5 universal a 2 pré-requisitos paralelos do fecho — ambos fora série §9.
>
> Mutações legacy preservadas como write paralelo M5 em todas as excepções fechadas; cleanup orgânico em M6 (P190/P200) quando `compute_*` helpers migrarem ou `CounterStateLegacy` for eliminado.

**Cenário B continua** (sem DEBT formal aberto).

---

## §9 Próximos passos sugeridos

### Pré-requisitos paralelos (M5 universal fecha após ambos)

- **`Content::SetEquationNumbering` materialização** (passo
  independente, fora série §9): fecha **E1**. Magnitude
  esperada: M (variant + arms análoga a SetHeadingNumbering
  P182C). Independente — pode executar primeiro ou último.

- **Sub-store `intr.headings_for_toc`** (passo independente,
  lacuna #3): fecha **E2-residuo**. Magnitude esperada: M
  (sub-store + arm + consumer outline). Independente — pode
  executar primeiro ou último.

### Após M5 universal fechar

- **P190 / P200 — M6 eliminação `CounterStateLegacy`** (passo
  agregado): cleanup do write paralelo M5; remoção do struct +
  dependências; consumer migrações finais (`compute_*` helpers
  eliminados ou migrados para sub-stores Introspector
  location-aware). Magnitude esperada: **L** (refactor maior
  cross-modular).

### Decisão estratégica para o utilizador

- Ordem dos pré-requisitos paralelos (E1 vs E2-residuo): E1
  pode ser executada independentemente; E2-residuo idem.
- Ambos fecharem em qualquer ordem antes de P190/P200.

---

## §10 Linhagem

- **Pattern arquitectural stylesheet**: ADR-0069 (PROPOSTO P195B; ACEITE P195E).
- **4 variantes operacionais consolidadas**:
  - **P195D variante** (não-locatable): snapshot+find_map.
  - **P196B variante** (locatable + body): `emitted_loc` directo.
  - **Cenário α** (P197B Figure, **P198B SetHeadingNumbering**): refactor estilístico ou declaração formal sem Tag pós-recursão.
  - **Cenário β-promote** (**P198C CounterUpdate** — 1ª aplicação): promote completo (variant nova + locatable + 2 arms).
- **5 aplicações ADR-0069 stylesheet**: P195D + P196B + P197B + P198B + **P198C**.
- **Helpers privados**: 3 (P195D `compute_labelled`, P196B `compute_heading_auto_toc`, P197B `compute_figure`); P198B + P198C sem helper.
- **Sub-stores consumidos**:
  - E5: `intr.state` (StateRegistry P171/P182C).
  - E6: `intr.counters` (CounterRegistry P184B).
- **Consumer downstream**: `compute_*` helpers (P195D Equation; P196B Heading; P197B Figure) — preservados; lêem legacy durante walk em todos os casos.
- **L0 tocado**: `00_nucleo/prompts/rules/introspect.md` hash `d25dfc47`.
- **Código tocado**: 6 ficheiros `01_core/src/`:
  - `entities/element_payload.rs` (variant nova).
  - `entities/element_kind.rs` (variant nova).
  - `rules/introspect/locatable.rs` (CounterUpdate locatable).
  - `rules/introspect/extract_payload.rs` (arm novo).
  - `rules/introspect/from_tags.rs` (arm novo + import).
  - `rules/introspect.rs` (comentários inline P198B + P198C; hash `f49ec9df`).
- **Padrão diagnóstico-primeiro**: 20ª aplicação consecutiva (P198A diagnóstico).

---

## §11 Métricas finais

- **Sub-passos**: 4 (A diagnóstico + B cenário α + C β-promote + D encerramento).
- **LOC produção**: ~85 (variant + arms + comentários).
- **LOC teste**: ~230 (5 P198B + 6 P198C).
- **LOC L0**: ~130 (secções novas + tabelas + ordem inversa).
- **LOC relatórios**: ~1.000 (4 relatórios + consolidado).
- **Variants ElementPayload novas**: +1 (CounterUpdate).
- **Variants ElementKind novas**: +1 (CounterUpdate).
- **Sub-stores novos**: 0.
- **ADRs novas**: 0.
- **Excepções M5 fechadas**: 2 (E5 cenário α; E6 cenário β-promote).
- **Tests netos adicionados**: +11.
- **Hashes desactualizados**: 0 (corrigidos por `--fix-hashes` em P198B + P198C).
- **79 passos executados** (contagem corrigida — P197C=76, P198A=77, P198B=78, P198C=79, **P198D=80**).

**Nota sobre contagem**: P198B §8 reportou inicialmente "77 passos" mas re-verificação consolidada em P198D ajusta para coerência: P198A=77, P198B=78, P198C=79, P198D=80 (cada passo soma 1 ao executado).

---

## §12 Marco arquitectural

**Sequência §9 P189 cumprida na totalidade**:

| Série | Estado | Excepção fechada |
|-------|--------|------------------|
| P193 (sub-store ResolvedLabelStore) | ✅ | (infraestrutura) |
| P194 (consumer C4 Ref-arm) | ✅ | (infraestrutura) |
| P195 (walk arm Labelled) | ✅ | E4 |
| P196 (walk arm Heading auto-toc) | ✅ | E2 → E2-residuo |
| P197 (walk arm Figure) | ✅ | E3 |
| **P198** (walks Set + Counter) | **✅** | **E5 + E6** |

**6 séries materializadas, 5 excepções fechadas, 1 residuo declarado**.

**M5 universal a 2 pré-requisitos paralelos do fecho**:
- E1 ↔ `Content::SetEquationNumbering` materialização.
- E2-residuo ↔ sub-store `intr.headings_for_toc`.

Ambos fora série §9. Ordem livre. Após ambos fecharem,
**M5 universal completo desbloqueia M6** (P190/P200 eliminação
`CounterStateLegacy`).

---

## §13 Notas operacionais

- **Tamanho série**: ~315 LOC produção/tests + ~1.000 LOC documentação.
- **Sem dependências externas novas**.
- **Sem ADR; sem DEBT formal aberto**.
- **Padrão replicado**: encerramento série P186/P187/P188/P189/P193/P194/P195/P196/P197 (relatório consolidado 9 secções padrão).
- **Cláusulas gate disparadas**: 0 substanciais (cadeia E5/E6 ↔ helpers compute_* resolvida sem disparar via mutação legacy preservada).
- **Cláusulas gate triviais resolvidas**: 1 (import `CounterUpdate` em `from_tags.rs`).
- **2 variantes operacionais novas em série única** — primeira ocorrência (P198B cenário α paralelo + P198C cenário β-promote).

**Próximo passo (ordem livre)**:
- `Content::SetEquationNumbering` materialização (fecha E1).
- Sub-store `intr.headings_for_toc` (fecha E2-residuo).
- (Após ambos) P190/P200 — M6 eliminação `CounterStateLegacy`.
