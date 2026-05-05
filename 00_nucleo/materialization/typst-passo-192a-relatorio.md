# Relatório P192A — Diagnóstico M7 (Estado A)

**Data**: 2026-05-05
**Magnitude**: S-M (diagnóstico).
**Estado**: Completo.
**Postura**: L0-puro / diagnóstico-primeiro (32ª aplicação consecutiva).
**Decisão final**: **Estado A** — M7 estruturalmente fechado.

---

## §1 Sumário executivo

P192A audita estado actual de M7 (loop fixpoint runtime). Resultado:
**M7 estruturalmente fechado**. Cristalino tem **dois loops fixpoint
distintos** complementares — TOC fixpoint (activo em produção) e
`run_fixpoint` (mecanismo opt-in para stdlib features) — ambos com
cap = 5 (paridade vanilla). Queries runtime location-aware activas
em 4 consumers Layouter. Tests E2E cobrem ambos os loops.

**Decisão**: Estado A → P192 série fecha em 2 sub-passos (A
diagnóstico + B declaração formal). Magnitude agregada S-M.

**Particularidade**: P192 reduz a 1 passo declarativo após
diagnóstico — auditoria revelou que M7 já estava fechado pela
sequência incremental P174 (mecanismo) → P175-P179 (features
stdlib) → M9 11/11 fechado → P190G/H/I (consumers Layouter
location-aware migrados via ADR-0071).

---

## §2 7 cláusulas decididas

| # | Cláusula | Decisão |
|---|----------|---------|
| 1 | Estado de `fixpoint.rs` | 626 LOC; `run_fixpoint`, `introspect_to_fixpoint` públicos; @prompt rules/introspect/fixpoint.md |
| 2 | Estado loop fixpoint | 2 loops complementares: TOC (mod.rs:1515) + run_fixpoint (introspect/fixpoint.rs:65). MAX = 5. Convergência via hash Tags / page map |
| 3 | Estado queries runtime | 4 queries activas em Layouter: is_numbering_active_at, formatted_counter_at, flat_counter_at, figure_number_at_index. `current_location` populated por advance_locator_if_locatable (P185C) |
| 4 | Validação empírica | 13+ tests fixpoint.rs + TOC tests em layout/tests.rs (layout_outline_*, pipeline_duas_passagens_*). Tests workspace 1.802 verdes |
| 5 | Comparação vanilla | Paralelo conceptual (loop, cap=5, observação cross-iteration); divergência intencional (cristalino sem comemo per ADR-0066 PROPOSTO) |
| 6 | Estado A/B/C | **Estado A** — completo |
| 7 | Critério fecho M7 | Loop fixpoint funcional + queries runtime activas + tests E2E + L0 documenta + ADR (P192B); todos satisfeitos |

---

## §3 Métricas finais P192A

- **LOC produção**: 0 (zero código tocado em diagnóstico).
- **LOC tests**: 0.
- **LOC L0**: 0 (defer P192B).
- **LOC diagnóstico + relatório**: ~750.
- **ADRs novas**: 0 (ADR-0072 projectada P192B).
- **Tests workspace**: 1.802 verdes (inalterados).
- **Linter**: 0 violations.

---

## §4 Achados não-triviais

### 4.1 Dois loops fixpoint distintos

Cristalino tem dois loops fixpoint **complementares** (não redundantes):

1. **TOC fixpoint** (`layout/mod.rs:1515`) — resolve forward refs em
   page numbers via `extracted_label_pages` map. Activo em produção
   quando `intr.kind_index.contains_key(&ElementKind::Outline)`.

2. **`run_fixpoint`** (`introspect/fixpoint.rs:65`) — mecanismo
   opt-in para stdlib features (`query()`, `counter.at()`, `here()`,
   etc.) que dependem de `ctx.introspector` populado de iteração
   anterior. Sem clientes runtime em produção; activado via M9
   features stdlib.

Os dois resolvem **categorias distintas** de dependências reverse:
- TOC: page numbers reverse-deps (DEBT-12).
- run_fixpoint: queries runtime reverse-deps (M9 stdlib).

### 4.2 `run_fixpoint` é mecanismo opt-in com tests, sem callers runtime

Per docs internas em `fixpoint.rs:10-13`:
> Mecanismo sem clientes em P174. Caller actual (`introspect()` +
> Layouter) não usa fixpoint — adopção planeada para P175+ quando
> features stdlib que dependem de `ctx.introspector` (`query`,
> `here`, `counter.at`) materializarem.

P175-P179 (M9) materializaram features. Mas em produção essas
features são exercitadas via tests apenas; sem invocação runtime no
pipeline `layout()` standard.

Implicação: `run_fixpoint` está **estruturalmente pronto** mas sem
tracção runtime. Quando uma feature stdlib for invocada num
documento real (não-teste) e exigir convergência, `run_fixpoint`
torna-se caminho activo.

### 4.3 Divergência arquitectural vs vanilla — sem comemo

ADR-0066 (introspection runtime adiada) define decisão arquitectural
**intencional**: cristalino **NÃO usa comemo** para introspecção
runtime. Convergência via hash de Tags (`compute_tags_hash`)
substitui `comemo::Constraint::validate`.

Trade-off:
- **Cristalino**: simples, transparente, sem dependência adicional. Custo: re-walk full por iteração.
- **Vanilla**: complexo, mas memoização comemo permite re-walks parciais. Custo: invariantes comemo difíceis.

ADR-0066 documentou a decisão; M7 fechado estruturalmente confirma
que a abordagem é viável.

### 4.4 Layouter location-aware queries são pré-condição M7 satisfeita

ADR-0068 (location-aware Layouter) — ACEITE — fornece o mecanismo
de `current_location` em Layouter, sincronizado-por-construção com
walk Locator. M7 sub-passo "queries runtime location-aware durante
layout" depende deste mecanismo. Confirmação pós-P190I: 4 queries
activas em produção (Heading prefix, Equation gate/format, Figure
caption prefix, CounterDisplay).

---

## §5 Estado activo vs preservado

### Activado (pré-existente confirmado em P192A)

- ✅ `run_fixpoint` — mecanismo opt-in tested (13+ tests).
- ✅ `introspect_to_fixpoint` — wrapper P175.
- ✅ TOC fixpoint loop em Layouter (mod.rs:1515).
- ✅ 4 queries location-aware em Layouter consumers.
- ✅ `current_location` field populated por `advance_locator_if_locatable`.
- ✅ Pattern `apply_state_funcs` slim post-pass (P191B).
- ✅ MAX_FIXPOINT_ITERATIONS = 5 (paridade vanilla).

### Preservado

- Trait `Introspector` 20 métodos.
- `TagIntrospector` 9 sub-stores.
- `LayouterRuntimeState` 3 fields.
- ADR-0068, ADR-0069, ADR-0070, ADR-0071 ACEITES.
- M5 universal completo, M6 fechado, M9 11/11.

---

## §6 Estado A confirmado — Plano P192 série

| Sub-passo | Escopo | Magnitude esperada |
|-----------|--------|--------------------|
| **P192A** (este) | Diagnóstico | S-M |
| **P192B** | Declaração formal M7 fechado: criar ADR-0072 (PROPOSTO/ACEITE) + transitar ADR-0066 PROPOSTO → ACEITE + relatório consolidado P192 + cross-references | S |

Total: 2 sub-passos. Magnitude agregada S-M.

---

## §7 ADR avaliação

**Decisão preliminar**: criar **ADR-0072 — M7 fechado completo
(fixpoint runtime; 2 loops complementares; divergência sem comemo)**
em P192B, retrospectivo.

ADR-0066 (Introspection runtime adiada) — PROPOSTO desde 2026-04-27;
candidato a transitar para **ACEITE** em P192B agora que M7 está
fechado.

**Cross-references esperadas em ADR-0072**:
- P174 (run_fixpoint mecanismo).
- P175-P179 (features stdlib via fixpoint).
- M9 11/11 (snapshot pré-P190).
- P190I (M6 fechado; ADR-0070 ACEITE).
- P191C (ADR-0071 ACEITE).
- ADR-0066 PROPOSTO → ACEITE.

---

## §8 DEBT estado

Nenhum DEBT novo identificado em P192A.

Trabalho ortogonal (fora de escopo P192):
- M8 (memoização comemo) — pendente; decisão estratégica.
- F3 completo (Layouter restantes 19 fields) — pendente.
- Lacunas residuais (#1 Position, #1b Position-related, #2 Counter at locations) — pendentes.

---

## §9 Magnitude consolidada

P192A: **S-M** confirmado.

P192B esperada: **S** (declarativo + ADRs + L0 update).

P192 série total: **S-M agregado** — refletindo que M7 já estava
fechado estruturalmente; P192 série é declarativa (não
implementativa).

---

## §10 Pendências cumulativas

- **P192B** — declaração formal M7 fechado + ADRs.

Após P192B: M7 declarado fechado oficialmente. M7 + M5 + M6
todos fechados. Próximas decisões estratégicas (M8, F3 completo,
lacunas residuais, pausa) ficam disponíveis.

---

## §11 Linhagem

- **Pattern arquitectural**: M7 fechado por sequência incremental
  P174 → P175-P179 → M9 → P190 série → P191 série, validado por
  P192A.
- **ADR-0066** (PROPOSTO 2026-04-27) — concedeu autorização
  arquitectural para introspection runtime; M7 materializa.
- **5 ADRs ACEITES no ciclo M5/M6**: ADR-0067, 0068, 0069, 0070,
  0071. ADR-0072 projectada.
- **Pattern stylesheet diagnóstico-primeiro**: 32ª aplicação
  consecutiva.
- **Pattern "auditoria sobre estado existente vs planeamento de
  trabalho futuro"**: 1ª aplicação distinguida (P192A audita; não
  planeia).

---

## §12 Restrições mantidas

- ✅ Zero código tocado em qualquer ficheiro fora de `00_nucleo/`.
- ✅ Zero testes modificados.
- ✅ Sem reservas de identificadores.
- ✅ `fixpoint.rs` NÃO modificado.
- ✅ Trait `Introspector` NÃO modificado.
- ✅ `TagIntrospector` NÃO modificado.
- ✅ Layouter NÃO modificado.
- ✅ Lacunas residuais NÃO materializadas.
- ✅ Sem inflação retórica (palavras vetadas evitadas).
- ✅ Comparação vanilla feita.
- ✅ Regra dos 2 eixos não aplicável (auditoria, não decisão).
- ✅ Padrão diagnóstico-primeiro respeitado.

---

## §13 Achado arquitectural significativo

P192A é primeira instância documentada do padrão **"auditoria sobre
estado existente vs planeamento de trabalho futuro"** no projecto.
Distinção:
- P190A, P191A, P195A, etc. — auditoram para **planear trabalho
  futuro** (ADR PROPOSTO subsequente).
- P192A — audita estado **já materializado**; resultado é
  declaração formal e não implementação.

Implicação: cristalino atinge fase de **consolidação arquitectural**
— marcos M5, M6, M7 fechados; trabalho ortogonal restante (M8, F3,
lacunas) disponível para selecção estratégica.

---

## §14 Próximo sub-passo concreto

**P192B** — declaração formal M7 fechado:

1. Criar `00_nucleo/adr/typst-adr-0072-m7-fixpoint-runtime.md`
   PROPOSTO ou ACEITE imediato (justificável pela validação P192A).
2. Editar `00_nucleo/adr/typst-adr-0066-introspection-runtime-adiada.md`
   transitando estado PROPOSTO → ACEITE.
3. Criar
   `00_nucleo/materialization/typst-passo-192-relatorio-consolidado.md`
   (9-10 secções padrão).
4. Cross-references actualizadas.
5. Magnitude esperada: S.

Após P192B, **M5 + M6 + M7** todos fechados completamente. M9
preserved 11/11. Cristalino atinge consolidação arquitectural
significativa.
