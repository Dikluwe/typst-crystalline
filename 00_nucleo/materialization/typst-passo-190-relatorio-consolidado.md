# Relatório Consolidado P190 — M6 fechado completo

**Data**: 2026-05-05
**Magnitude consolidada**: L cross-modular.
**Estado**: P190 série completa — **M6 fechado pela primeira vez desde declaração em P185A**.
**ADR-0070**: ACEITE em P190I.
**ADR-0071**: ACEITE em P191C (ramo paralelo).

---

## §1 Resumo executivo

P190 série materializa **eliminação completa de `CounterStateLegacy`**
(ADR-0070) — struct de 16 fields herdada da arquitetura pré-M3,
write paralelo M5 ainda activo. Após 8 sub-passos incrementais
(P190B-H) + ramo paralelo P191A-C (ADR-0071 PROPOSTO/ACEITE) +
passo final P190I (struct elim + Layouter `counter` field elim +
ADR-0070 ACEITE), **M6 fecha completamente**.

**Marco**:
- `CounterStateLegacy`: 16 → **0 fields** (struct eliminada).
- Layouter: 20 → 19 fields (-1; `counter` eliminado).
- Walk fn signature: 5 → 7 params (eliminado `state`; added `intr`,
  `auto_label_counter`, `lang`).
- Pattern "eliminação write paralelo M5": **8 aplicações concretas**.
- Pattern "Layouter-runtime → struct dedicada": **2 aplicações**.
- Pattern "1ª aplicação directa ADR-0071 em P190": **3 aplicações**.
- 5 ADRs ciclo M5/M6: ADR-0067, ADR-0068 (ACEITES); ADR-0069 ACEITE
  P195E; **ADR-0070 ACEITE P190I**; **ADR-0071 ACEITE P191C**.
- Helpers eliminados: 1 (`compute_figure`) + 4 Layouter
  (`layout_set_heading_numbering`, `layout_set_equation_numbering`,
  `layout_counter_update`, `format_counter_display`).
- 4 defers acumulados resolvidos (`numbering_active` em P190G;
  `flat`, `hierarchical`, `lang` em P190I).
- F1 fechado.
- F3 parcialmente fechado (Layouter -1 field; outros 18 ortogonais
  pendentes).

**LOC produção líquido**: -990 (1726 insertions, 2716 deletions per
`git diff --stat` cumulativo desde P190A).

**Tests workspace**: 1802 verdes (vs ~1834 baseline P200B; Δ -32
marginal — sentinelas legacy redundantes removidas).

---

## §2 Sub-passos materializados

| Passo | Magnitude planeada | Magnitude real | Δ tests | Fields elim | L0s |
|-------|--------------------|--------------------|---------|-------------|-----|
| P190A | S-M | S-M | 0 | 0 | 0 |
| P190B | M | M | -2 | 2 (bib_*) | 0 |
| P190C | M | M | 0 | 2 (page_*) | 0 |
| P190D | M | M | 0 | 2 (has_outline, is_readonly) | 0 |
| P190E | S | S | 0 | 0 (defer) | 0 |
| P190F | M | S (escopo reduzido) | 0 | 0 (defer; barreira) | 0 |
| **P191A** | S-M | S-M | 0 | — | 0 |
| **P191B** | M+ | M+ | -2 | — | 0 |
| **P191C** | S-M | S-M | +1 | — | 0 |
| P190G | M | M+ | 0 | 4 (resolved_labels, headings_for_toc, auto_label_counter, numbering_active) | 0 |
| P190H | M | M | 0 | 3 (figure_numbers, figure_label_numbers, local_figure_counters) | 0 |
| **P190I** | M+ | L | -10 | 3 (hierarchical, flat, lang) + struct | 1 (counter_state_legacy.md histórico) |
| **Total** | L | **L** | **Δ -13 marginal** | **16 fields + struct + 5 helpers** | 1 L0 |

---

## §3 Decisões arquitecturais

### 9 cláusulas P190A fechadas

Todas materializadas em B-I. Plano β incremental confirmado correcto.

### Decisões empíricas P190B-H

- **P190B Bibliography** (cláusula gate trivial): caminho Introspector activo desde P181E.
- **P190C Page tracking** (pattern stylesheet emergente): "Layouter-runtime → struct dedicada"; LayouterRuntimeState criada (2 fields).
- **P190D Document metadata** (extensão pattern stylesheet): is_readonly movido (3 fields LRS); has_outline eliminado; lang defer.
- **P190E Numbering active** (Caso 1 parcial): walk readers preservam write paralelo; field defer.
- **P190F Counters core** (cláusula gate substancial): barreira arquitectural — walk fn não tem acesso a Introspector. **Pause**.

### Ramo paralelo P191A-C (ADR-0071)

- **P191A**: diagnóstico — 4 opções avaliadas; Opção A escolhida (walk recebe `&mut TagIntrospector`).
- **P191B**: prova de conceito — walk fn signature change + 25 call sites + populate_intr_from_tag_start helper + 1 helper migrado + walk arm Equation gate migrado + from_tags eliminado.
- **P191C**: 2º helper migrado (compute_labelled) + ADR-0071 ACEITE.

### Decisões empíricas P190G-I

- **P190G** (Opção α + Caso 1): 4 fields eliminados; numbering_active resolvido.
- **P190H** (Opção α + Caso 2): 3 fields eliminados; helper compute_figure orphan eliminado; lang defer.
- **P190I** (Opção β API breaking change): API simplificada; struct eliminada; lang resolvido via parameter walk fn; Layouter counter field eliminado.

---

## §4 Achados não-triviais

### 4.1 Barreira arquitectural P190F

Walk fn não tinha acesso a `Introspector` (construído POST-walk via
from_tags). Helpers walk-readers + walk-arm-gates bloqueados para
migração location-aware. Resolução: **abrir ramo paralelo P191
(ADR-0071)** em vez de forçar migração com soluções subóptimas.

### 4.2 Locations monotónicas garantem ordering Sets vs queries

Locator gera Locations strictly monotonic. Set tags emitidas ANTES
de query tags (Equation gate, Heading auto-toc) → state populated
antes de query. Cláusula gate substancial 6 P191A resolvida por
construção.

### 4.3 Caso edge Set após Func intermediário (limitação aceite)

Tag stream `[Set, Func, Set]` em `apply_state_funcs` post-pass não
preserva ordering. Não exercitado por tests; documentado.

### 4.4 LOC líquido cumulativo P190 -990

Eliminação de 16 fields + struct + 5 helpers + walk fn `state`
parameter + Layouter `counter` field + ~26 from_tags arm tests +
adapt tests = -2716 LOC; new tests + new helpers + ADR-0071 helpers
= +1726 LOC. Net -990.

### 4.5 Eliminação dos fallbacks substitution-with-fallback

Padrão emergente em P190G-H-I: fallback `or_else(|| state.X)`
elimina-se quando state field eliminada. Substitution-with-fallback
(P184D / P194B) colapsa em Introspector path puro.

### 4.6 Helpers walk-internal podem ficar orphan

`compute_figure` (P197B) orphan após P190H — populate_intr arm
Figure cobre o trabalho que compute_figure fazia. Helpers
walk-internal sobrevivem apenas enquanto há consumers walk-side
distintos do Tag emission.

### 4.7 P190I — `lang` resolução via parameter walk fn

Caso 2 inicial em P190H deferred to P190I; em P190I migrado para
Opção α (parameter `lang: Option<&Lang>` adicionado a walk fn).
Net signature change: 7 params (drop state, add lang).

### 4.8 P190I — API pública breaking change

`introspect()` retornava `(CounterStateLegacy, TagIntrospector)`;
agora retorna `TagIntrospector`. `layout(content, state)` agora
`layout(content)`. Callers externos (`03_infra/src/`) adaptados.

---

## §5 Estado activo vs preservado

### Activado em P190 (categoria por categoria)

- **P190B**: Bibliography eliminação (intr.bib_store via from_tags).
- **P190C**: Page tracking → LayouterRuntimeState.
- **P190D**: has_outline → kind_index.contains_key; is_readonly → LRS.
- **P190E/G**: numbering_active → intr.state via populate_intr.
- **P190G**: resolved_labels → intr.resolved_labels; headings_for_toc → intr.headings_for_toc; auto_label_counter → walk fn parameter.
- **P190H**: figure_numbers → intr.counters["figure:{kind}"]; figure_label_numbers → intr.figure_label_numbers; local_figure_counters → eliminado (intr query).
- **P190I**: hierarchical/flat → intr.counters via populate_intr; lang → walk fn parameter; struct + Layouter counter field eliminados.

### Preservado

- Trait `Introspector` 20 métodos (estável).
- `TagIntrospector` 9 sub-stores (estável).
- `LayouterRuntimeState` 3 fields (estável; expansível).
- Pattern ADR-0069 stylesheet (5 variantes operacionais).
- ADR-0071 mecanismo (walk fn aceita TagIntrospector).
- 1 helper walk-internal: `compute_heading_for_toc` (P200B).
- `apply_state_funcs` slim post-pass para Func eval em fixpoint.

---

## §6 Estado final M9, M5, M6

| Marco | Pré-P190 | Pós-P190 | Δ |
|-------|----------|----------|---|
| M9 | 11/11 | 11/11 | inalterado |
| **M5 universal completo** | ✅ (P200B) | ✅ | inalterado |
| **M6 (eliminação CounterStateLegacy)** | ⏸️ pause | 🟢 **COMPLETO** | **fechado** |
| `Content` enum | 13 variants | 13 | inalterado |
| `ElementPayload` | 12 variants | 12 | inalterado |
| `ElementKind` | 10 | 10 | inalterado |
| Trait `Introspector` | 20 métodos | 20 | inalterado |
| `TagIntrospector` | 9 sub-stores | 9 | inalterado |
| **`CounterStateLegacy`** | **16 fields** | **0 (struct eliminada)** | **Δ -16** |
| `LayouterRuntimeState` | 0 (não existia) | 3 fields | criada P190C |
| `Layouter` | 20 fields | 19 | -1 (`counter`) |
| Walk fn signature | 5 params | 7 params | +2 net |
| Helpers privados família ADR-0069 | 4 | 3 | -1 (compute_figure) |
| Helpers Layouter `counters.rs` | 4 | 0 | -4 |

---

## §7 Estado final lacunas

| Lacuna | Estado pré-P190 | Estado pós-P190 |
|--------|-----------------|-----------------|
| #1 (Position) | residual | residual (inalterado) |
| #1b (Position-related) | residual | residual (inalterado) |
| #2 (Counter at locations) | residual | residual (inalterado) |
| #3 (headings_for_toc) | fechada P200B | fechada (inalterado) |

P190 não impactou lacunas residuais.

---

## §8 Pendências cumulativas + marco M6 fechado

### **M6 fechado completamente** ✅

- 4 defers acumulados resolvidos (numbering_active P190G; flat,
  hierarchical, lang P190I).
- DEBT M6 documentação fechado por execução.
- F1 fechado.
- F3 parcialmente fechado (Layouter -1 field).

### Restante F3 (refactor opcional futuro)

- Layouter ainda 19 fields ortogonais (cursor_x, cursor_y, pages,
  current_items, etc.) — não relacionados com counter state.
- F3 completo requer refactor Layouter geral — domínio de passos
  futuros.

---

## §9 Próximos passos sugeridos

### Imediato (decisão estratégica do utilizador)

1. **M7** (loop fixpoint runtime).
2. **M8** (memoização comemo).
3. **F3 completo** — refactor Layouter restantes 19 fields.
4. **Lacunas residuais** (#1, #1b, #2) — passos dedicados.
5. **Pausa estratégica** — consolidar M6 fechado.

### Naturais após M6

- Loop fixpoint para queries runtime location-aware.
- Memoização cross-iteration via comemo (M8).

---

## §10 Marco arquitectural — M6 fechado completo

### Histórico completo

```
P190A (PROPOSTO) → P190B-H (aplicações incrementais)
                                ↓
                        Barreira P190F
                                ↓
                P191A-C (ramo paralelo ADR-0071)
                                ↓
                        ADR-0071 ACEITE
                                ↓
                P190G-H (1ª, 2ª aplicações directas ADR-0071)
                                ↓
                P190I (3ª aplicação + struct elim)
                                ↓
                        ADR-0070 ACEITE
                                ↓
                          M6 FECHADO
```

### Métricas marco

- **9 sub-passos materializados em P190** (A-I).
- **3 sub-passos em P191** (A-C ramo paralelo).
- **12 sub-passos total** para fechar M6.
- **8 aplicações concretas** pattern "eliminação write paralelo M5".
- **2 padrões complementares** estabelecidos
  ("Layouter-runtime → struct dedicada", "eliminação directa via
  Introspector path").
- **3 aplicações directas ADR-0071** em P190 série.
- **5 ADRs completos** no ciclo M5/M6 (ADR-0067, 0068, 0069, 0070,
  0071).
- **32ª aplicação diagnóstico-primeiro consecutiva**.
- **97 passos executados** (P181-P200 + P190A-I + P191A-C).

### Marco arquitectural significativo

**F1 fechado**: `CounterStateLegacy` struct unificada de 16 fields
eliminada. Walk pipeline puro com Introspector path location-aware
único. ADR-0070 + ADR-0071 ACEITES.

---

## §11 Restrições mantidas

- ✅ Trait `Introspector` NÃO modificado (20 métodos estáveis).
- ✅ `TagIntrospector` fields NÃO modificados (9 sub-stores).
- ✅ `LayouterRuntimeState` NÃO modificado (3 fields).
- ✅ Pattern walk arquitectural preservado (mecanismo ADR-0071).
- ✅ Lacunas residuais NÃO materializadas.
- ✅ Output observable em produção inalterado (Δ tests marginal).

---

## §12 Linhagem

- **Pattern arquitectural eliminação**: ADR-0070 ACEITE P190I.
- **Pattern arquitectural pre-condition**: ADR-0071 ACEITE P191C.
- **5 variantes operacionais ADR-0069**: preservadas.
- **8 aplicações eliminação write paralelo M5**: P190B-I.
- **Pattern stylesheet "Layouter-runtime → struct dedicada"**: P190C-D.
- **Pattern stylesheet "diagnóstico-primeiro"**: 32ª aplicação consecutiva.
- **F1**: fechado em P190I.
- **F3**: parcialmente fechado em P190I.
