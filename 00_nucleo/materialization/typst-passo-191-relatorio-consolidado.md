# Relatório Consolidado P191 — Walk pipeline redesign (ADR-0071)

**Data**: 2026-05-05
**Magnitude consolidada**: M+ a L (real M+).
**Estado**: P191 série fechada — ADR-0071 ACEITE.
**Sub-passos**: A (diagnóstico) ✅ + B (prova de conceito) ✅ + C (helper final + ADR ACEITE) ✅.

---

## §1 Resumo executivo

P191 série materializa **ADR-0071 — walk pipeline com Introspector
acessível durante execução** (Opção A). Resolve barreira
arquitectural identificada em P190F §3: walk fn não tinha acesso a
`Introspector` (construído POST-walk via `from_tags::from_tags`),
bloqueando migração de helpers walk-readers e walk-arm-gates para
Introspector path location-aware (P185B).

P190 série em pausa após P190F (3 sub-passos restantes G/H/I + 4
defers acumulados). P191 abre **ramo paralelo** para resolver a
barreira arquitectural. ADR-0071 PROPOSTA em P191A; validada
empíricamente em P191B (1 helper migrado); confirmada e ACEITE em
P191C (2º helper migrado + cleanup).

**Ganhos**:
- Walk fn ganha `intr: &mut TagIntrospector` parameter.
- Pipeline simplificado: `walk → return` (sem etapa from_tags
  intermédia; `apply_state_funcs` slim residual chamado apenas
  por fixpoint).
- 2 helpers walk-readers migrados para Introspector path
  location-aware: `compute_heading_auto_toc`, `compute_labelled`.
- Walk arm Equation gate migrado.
- `from_tags::from_tags` eliminado (969 LOC removidos).
- LOC líquido P191B: -699.
- 5+1 cláusulas gate substanciais resolvidas empíricamente.

**Pré-condição arquitectural cumprida** para retomar P190G.

---

## §2 Sub-passos materializados

| Passo | Magnitude planeada | Magnitude real | Δ tests | L0s | Trabalho |
|-------|--------------------|--------------------|---------|-----|----------|
| P191A | S-M | S-M | 0 | 0 | Diagnóstico — 4 opções; Opção A escolhida; ADR-0071 PROPOSTO; lembrete P190 pause-resume tracker |
| P191B | M+ | M+ | -2 | 0 | Walk signature + 25 call sites + populate_intr_from_tag_start centralizado + helper compute_heading_auto_toc migrado + walk arm Equation gate migrado + from_tags eliminado + apply_state_funcs preservada para Funcs |
| P191C | S-M | S-M | +1 | 0 | Helper compute_labelled migrado (4 arms) + caller adapt (snapshot+find_map preservado) + Figure populate_intr gated by is_counted (cláusula gate empírica) + ADR-0071 ACEITE + tracker P190 retomar |
| **Total** | M+ a L | **M+ real** | **-1 marginal** | **0** | Mecanismo Opção A completo |

---

## §3 Decisões arquitecturais

### 9 cláusulas P191A fechadas

| # | Cláusula | Decisão final |
|---|----------|---------------|
| 1 | Mecanismo | Opção A — walk recebe `&mut TagIntrospector` |
| 2 | Helpers | 2 migrar (compute_labelled, compute_heading_auto_toc); 2 manter (compute_figure, compute_heading_for_toc walk-internal) |
| 3 | Walk arm Equation gate | Migrar para `is_numbering_active_at` location-aware |
| 4 | ADR-0069 compatibilidade | Preservada — 5 variantes operacionais; signatures alteradas mas pattern identity preservado |
| 5 | `from_tags` | Eliminado (Opção α) em P191B; `apply_state_funcs` slim residual para Funcs |
| 6 | Pre-condições populate timing | Sequencial natural — Locator monotónico garante ordering |
| 7 | Estratégia migração | P191B mecanismo + 1 helper validation; P191C 2º helper + ADR ACEITE |
| 8 | Tests | Padrão pragmático auditor #1 + 3 sentinelas mecanismo (P191B 2 + P191C 1) |
| 9 | Critério fecho | Mecanismo + 2 helpers + walk gate + tests + ADR ACEITE + pre-condição P190G |

### 4 decisões empíricas P191B

| # | Decisão | Razão |
|---|---------|-------|
| B1 | from_tags::from_tags eliminado totalmente (Opção α) | Caller único (fixpoint) migrado para `apply_state_funcs` slim |
| B2 | `apply_state_funcs` preservada (15 LOC) | Func eval requires Engine+ctx — não disponível em walk; post-pass apenas em fixpoint |
| B3 | Helper signature genérica `<I: Introspector>` | Permite mocks futuros; cost-free para production |
| B4 | Locations monotónicas confirmadas | Set tags emitidas ANTES de Equation tag → state populated antes do gate query |

### 1 decisão empírica P191C

| # | Decisão | Razão |
|---|---------|-------|
| C1 | populate_intr Figure arm gated by `is_counted` | Restaura paridade com legacy `state.figure_numbers` que só registava is_counted; divergência latente exposta após `compute_labelled` migrar para Introspector |

### 3 decisões empíricas P191C (cláusulas gate)

| Cláusula | Resolução |
|----------|-----------|
| `lang` no helper Figure arm | Opção β — passar como parameter `Option<&Lang>` ao helper; caller (walk arm Labelled) tem `state.lang` no scope |
| Snapshot+find_map P195D | Preservado conceptualmente; obtenção de target_loc movida para ANTES da chamada a compute_labelled |
| API location-aware suficiente | Sim — `formatted_counter_at` (Heading), `flat_counter_at` (Equation, Figure), `is_numbering_active_at` (gate Equation) cobrem 4 arms |

---

## §4 Achados não-triviais

### 4.1 `apply_state_funcs` preservada (Func eval em fixpoint)

Eliminação `from_tags::from_tags` (Opção α) **não é total**: Func
updates em StateRegistry requerem `apply_func` + `Engine + EvalContext`
(eval real). Walk não tem acesso a Engine. Solução: post-pass slim
`apply_state_funcs` chamada apenas em `fixpoint::run_fixpoint` que
tem Engine+ctx. Path legacy (`introspect_with_introspector`)
silenciosamente ignora Funcs — coerente com semântica P171/P173
pré-P191B (sem Engine = defensive ignore).

### 4.2 Locations monotónicas garantem ordering Sets vs queries

Locator gera Locations strictly monotonic via counter incremento.
Walk processa Content em tree order. Por isso:
- `SetEquationNumbering` Tag emit @ loc_set < `Equation` Tag emit @ loc_eq.
- populate_intr aplica state.update("numbering_active:equation", loc_set) ANTES de query state.value_at("numbering_active:equation", loc_eq).
- Gate query no Equation populate retorna correctamente.

Cláusula gate substancial 6 P191A resolvida por construção.

### 4.3 Caso edge Set após Func intermediário não exercitado

Tag stream `[State("c", 0)@10, Func("c", x=>x+1)@20, Set("c", 100)@30]`:
- Walk popula intr.state.init/update apenas para Set tags em ordem.
- post-pass `apply_state_funcs` aplica Func @20 sobre intr (já com Set@30).
- StateRegistry adiciona (20, val_func) AO FIM da history.
- value_at(40) retorna last entry = Func resultado, não Set@30.
- DIVERGE de from_tags pré-P191B (que processava em ordem).

Limitação aceite — não exercitada por tests existentes; documentada
em P191B relatório §5.3. Resolução futura via re-ordenamento history
post-pass ou outro mecanismo.

### 4.4 LOC líquido P191B -699

Refactor simplifica globalmente:
- Insertions P191B: 801 LOC.
- Deletions P191B: 1500 LOC.
- Net: -699 LOC.

`from_tags::from_tags` (969 LOC + tests) substituído por
`populate_intr_from_tag_start` (167 LOC) + `apply_state_funcs`
(15 LOC) + tests preservados (3 Func tests).

### 4.5 P191C — populate_intr Figure gate restaurado

Pre-existing divergence: from_tags arm Figure aplicava counter
unconditionally; legacy state.figure_numbers só registava is_counted.
compute_labelled lia state legacy → ocultada. P191C migra
compute_labelled para Introspector → expõe. Solução: gate populate
em populate_intr Figure arm por `is_counted`. Restaura paridade
sem modificar Layouter (Layouter consumer C3 path falha-tolerant
via fallback substitution-with-fallback P184D + unwrap_or).

---

## §5 Estado activo vs preservado

### Activado em P191

- ✅ Walk fn aceita `&mut TagIntrospector` parameter.
- ✅ 12 ElementPayload variants populated directamente durante walk
  via `populate_intr_from_tag_start` (167 LOC, centralizado).
- ✅ 2 helpers walk-readers usam Introspector path location-aware:
  `compute_heading_auto_toc`, `compute_labelled`.
- ✅ Walk arm Equation gate via `intr.is_numbering_active_at(...)`.
- ✅ `from_tags::from_tags` eliminado (Opção α).
- ✅ `introspect_with_introspector` simplificada — drops engine/ctx
  params (Funcs continuam ignorados neste path).

### Preservado

- 2 helpers walk-internal (`compute_figure`, `compute_heading_for_toc`)
  — não migrados; chamados antes de Tag emission para feeds locais.
- Pattern ADR-0069 stylesheet (5 variantes operacionais inalteradas
  conceptualmente).
- `apply_state_funcs` slim post-pass para Func eval em fixpoint
  (15 LOC).
- `CounterStateLegacy` 10 fields (4 com walk readers; defer
  P190G/H/I — agora resolvíveis via Introspector path).
- Trait `Introspector` 20 métodos (inalterado).
- `TagIntrospector` 9 sub-stores (inalterado).
- `LayouterRuntimeState` 3 fields (inalterado).
- Tags emit + ADR-0069 5 variantes operacionais funcionais.

---

## §6 Estado final M9, M5, M6

| Marco | Pré-P191 | Pós-P191 | Δ |
|-------|----------|----------|---|
| M9 | 11/11 | 11/11 | inalterado |
| **M5 universal completo** | ✅ | ✅ | inalterado |
| **M6 (eliminação CounterStateLegacy)** | ⚠️ pause após P190F | 🟢 **barreira resolvida** | desbloqueado |
| `Content` enum | 13 variants | 13 | inalterado |
| `ElementPayload` | 12 variants | 12 | inalterado (P200B já tinha HeadingForToc) |
| `ElementKind` | 10 | 10 | inalterado |
| Trait `Introspector` | 20 métodos | 20 | inalterado |
| `TagIntrospector` | 9 sub-stores | 9 | inalterado |
| `CounterStateLegacy` | 10 fields | 10 | inalterado em P191; defer P190G/H/I |
| `LayouterRuntimeState` | 3 fields | 3 | inalterado |
| Walk fn signature | 5 params | **6 params** (+intr) | mudança P191B |
| Helpers walk-readers migrados | 0 | **2** | P191B + P191C |
| `from_tags::from_tags` | 969 LOC | **eliminado** | -969 LOC |
| `apply_state_funcs` (residual) | — | 15 LOC | nova |
| `populate_intr_from_tag_start` (helper) | — | 167 LOC | novo |

---

## §7 Estado final lacunas

| Lacuna | Estado pré-P191 | Estado pós-P191 |
|--------|-----------------|-----------------|
| #1 (Position) | residual | residual (inalterado) |
| #1b (Position-related) | residual | residual (inalterado) |
| #2 (Counter at locations) | residual | residual (inalterado) |
| #3 (headings_for_toc) | fechada P200B | fechada (inalterado) |

P191 não impactou lacunas residuais.

---

## §8 Pendências cumulativas

### **P190 série** — pronta para retomar

**Próximo passo concreto**: **P190G** — Categoria 6 (Labels & TOC).
Magnitude esperada: M.

### 4 defers acumulados — agora resolvíveis

| Defer | Origem | Status pós-P191 | Resolução |
|-------|--------|-----------------|-----------|
| `lang` | P190D | resolvível via parameter passing (Opção β) ou eliminação | P190G ou P190I |
| `numbering_active` | P190E | **resolvível** via `intr.is_numbering_active_at` | P190G após |
| `flat` | P190F | **resolvível** via `intr.flat_counter_at` | P190G/H após |
| `hierarchical` | P190F | **resolvível** via `intr.formatted_counter_at` | P190G após |

3 dos 4 defers agora têm path Introspector activo (consumers
walk-side podem migrar). Defer `lang` continua via parameter
passing até consumer walk-side ser eliminado.

### Restante M6

- **P190G** Labels & TOC (M).
- **P190H** Figures (M).
- **P190I** Walk arms purification + Layouter final + struct elim
  + ADR-0070 ACEITE (M+).

### F1, F3

- F1 (`CounterStateLegacy` 16 fields heterogéneos): fecha após P190I.
- F3 (Layouter 19 fields): parcialmente fecha após P190I (campo
  `counter` eliminado).

### DEBT M6 documentação fecha por execução em P190G/H/I.

---

## §9 Próximos passos

### Imediato — retomar P190 série

1. **P190G** — Categoria 6 (Labels & TOC). Magnitude M.
   Trabalho:
   - Migrar consumers walk-side de `auto_label_counter`,
     `resolved_labels`, `headings_for_toc` (3 fields) para
     Introspector path location-aware OU eliminar fields se
     consumers já estavam migrados via P191.
   - 1ª aplicação directa do mecanismo P191/ADR-0071.
   - Resolve parcialmente defer `lang`.

2. **P190H** — Categoria 7 (Figures). Magnitude M.
   Trabalho:
   - Migrar consumers de `figure_numbers`, `figure_label_numbers`,
     `local_figure_counters` (3 fields).

3. **P190I** — Walk arms purification + Layouter final + struct
   elim + ADR-0070 ACEITE. Magnitude M+.

### Após M6 fechar

- M7 (loop fixpoint).
- M8 (memoização comemo).
- Lacunas residuais (#1, #1b, #2) — passos dedicados.

---

## §10 Marco arquitectural

P191 é **primeiro ramo paralelo** na série P190 cumprida com sucesso.
Padrão consolidado:

```
diagnóstico-primeiro → ADR-PROPOSTO → validação parcial (1 helper)
→ ADR-ACEITE (2º helper + cleanup) → retomar série principal
```

Análoga a ADR-0068 P185A em estrutura mas executada em **escala
menor** (3 sub-passos vs ~5 para ADR-0068).

29ª aplicação consecutiva do padrão diagnóstico-primeiro
(P181-P200 + P190A-F + P191A-C).

5 ADRs completos no ciclo M5/M6:
1. ADR-0068 (location-aware Layouter; ACEITE).
2. ADR-0069 (write paralelo M5; ACEITE em P195E).
3. ADR-0070 (eliminação CounterStateLegacy; PROPOSTO P190A; ACEITE projectada P190I).
4. **ADR-0071 (walk pipeline redesign; ACEITE P191C)**.

---

## §11 Métricas finais P191

- **LOC produção líquido**: -699 (P191B) + ~50 (P191C) = -649 ~líquido.
- **LOC tests**: ~26 from_tags arm tests removidos; 3 Func tests
  preservados (apply_state_funcs); 2 sentinelas P191B + 1 P191C
  novos. Net Δ tests workspace: marginal.
- **LOC L0**: 0 (defer documentação para P190G+).
- **Variants Content novas**: 0.
- **Sub-stores novos**: 0.
- **ADRs novas**: 1 (ADR-0071).
- **Helpers privados novos**: 1 (`populate_intr_from_tag_start`).
- **Helpers públicos**: -1 (`from_tags`); +1 (`apply_state_funcs`).
- **Cláusulas gate substanciais resolvidas**: 5 (P191B) + 1 (P191C empírica).
- **F1 progresso**: defer P190G/H/I (CounterStateLegacy still 10 fields).
- **F3 progresso**: defer P190G/H/I.

---

## §12 Lembrete formal CRÍTICO

**P191 série fechada — ADR-0071 ACEITE — pré-condição arquitectural cumprida.**

**Próximo passo**: **P190G** (Categoria 6 — Labels & TOC).

Tracker em `00_nucleo/p190-pause-resume-tracker.md` actualizado
com snapshot pós-P191 e plano de retomar.

---

## §13 Linhagem

- **Pattern arquitectural**: ADR-0071 PROPOSTO P191A → **ACEITE P191C**.
- **Pre-condição barrier**: P190F §3.
- **ADR análogo**: ADR-0068 (location-aware Layouter mecanismo).
- **5 variantes operacionais ADR-0069**: preservadas.
- **7 aplicações ADR-0069 stylesheet**: preservadas (P195D + P196B
  + P197B + P198B + P198C + P199B + P200B).
- **Pattern stylesheet "diagnóstico-primeiro"**: 29ª aplicação
  consecutiva.
- **F1**: não fecha em P191; fecha após P190G/H/I.
- **F3**: não fecha em P191; parcialmente fecha após P190I.
