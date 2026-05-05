# Relatório P191A — Diagnóstico walk pipeline redesign

**Data**: 2026-05-05
**Magnitude**: S-M (diagnóstico).
**Estado**: Completo.
**Pattern arquitectural**: ADR-0071 PROPOSTO — walk pipeline com Introspector accessible durante execução (Opção A).
**Lembrete crítico**: **P190 série em pausa**. Retomar P190G após P191 fechar.

---

## §1 Sumário executivo

P191A audita barreira arquitectural identificada em P190F §3 (walk fn sem acesso a Introspector). Avalia 4 opções de redesign e recomenda **Opção A — walk recebe `&mut TagIntrospector`**.

**Decisão**:
- **Mecanismo**: Opção A — walk fn signature ganha `intr: &mut TagIntrospector` parameter. Walk arms replicam from_tags logic incrementalmente.
- **2 helpers migráveis**: `compute_labelled` + `compute_heading_auto_toc`.
- **2 helpers walk-internal**: `compute_figure` + `compute_heading_for_toc` (inalterados).
- **from_tags eliminado** ou reduzido a no-op em P191B.
- **Pattern ADR-0069 stylesheet preservado** — 5 variantes operacionais inalteradas.

**Plano**:
- **P191B** (M+): implementar mecanismo + migrar 1 helper validation + walk arm Equation gate.
- **P191C** (S-M): migrar 2º helper + cleanup + ADR-0071 ACEITE + lembrete.

**Após P191 fechar**: pre-condição arquitectural cumprida para retomar P190G.

---

## §2 Estado actual confirmado

| Item | Estado |
|------|--------|
| Tests workspace | 1.855 verdes |
| Linter | 0 violations |
| Walk fn signature | Sem `&Introspector` |
| Pipeline | walk → from_tags → return |
| Helpers walk-readers | 2 (compute_labelled, compute_heading_auto_toc) |
| Helpers walk-internal | 2 (compute_figure, compute_heading_for_toc) |
| Walk arm gates state-dependent | 1 (Equation gate) |
| `from_tags` arms | 12 ElementPayload variants |
| `CounterStateLegacy` | 10 fields |
| Defers acumulados | 4 (lang, numbering_active, flat, hierarchical) |
| P190 série | em pausa após P190F |

---

## §3 4 opções avaliadas

| Opção | Mecanismo | Magnitude | Veredicto |
|-------|-----------|-----------|-----------|
| **A** — Walk recebe `&mut TagIntrospector` | Single pass; replicate from_tags em walk arms | M+ | **Escolhida** — directa, preserve ADR-0069 |
| **B** — Two-pass walk | 1ª pass + build intr + 2ª pass com intr access | L | Descartada — performance + complexidade |
| **C** — Eliminate helpers; embed inline | 4 helpers eliminados | M+L | Descartada — perde 7 séries ADR-0069 |
| **D** — Deferred resolution Layouter-side | Tags com payloads parciais; lazy at render | M+ | Descartada — semantic change cross-cutting |

---

## §4 Decisões cláusula 1–9

| # | Cláusula | Decisão |
|---|----------|---------|
| 1 | Mecanismo | **Opção A** — walk recebe `&mut TagIntrospector` |
| 2 | Helpers | 2 migrar (compute_labelled, compute_heading_auto_toc); 2 manter (walk-internal) |
| 3 | Walk arm Equation gate | Migrar para `intr.is_numbering_active_at` location-aware |
| 4 | ADR-0069 compatibilidade | Preservada — 5 variantes operacionais inalteradas; signatures alteradas mas padrão identity preservado |
| 5 | `from_tags` | Eliminado em P191B (replaced por mutação directa de intr durante walk) |
| 6 | Pre-condições populate timing | Sequencial natural: walk arm SetX populate intr.state ANTES de walk arm Equation query gate |
| 7 | Estratégia migração | P191B: mecanismo + 1 helper como prova de conceito; P191C: 2º helper + cleanup |
| 8 | Tests | Existentes preservados via padrão pragmático auditor #1; 1-2 tests novos sentinela mecanismo |
| 9 | Critério fecho | Mecanismo + 2 helpers migrados + walk gate migrado + tests verdes + ADR-0071 ACEITE + pre-condição P190G cumprida |

---

## §5 Magnitude consolidada

| Sub | Magnitude planeada |
|-----|-------------------|
| P191A | S-M (diagnóstico) |
| P191B | M+ (implementação + 1 helper validation) |
| P191C | S-M (2º helper + cleanup + ADR ACEITE) |
| **Total** | **M+ a L** cross-modular |

---

## §6 ADR-0071 PROPOSTO

**Título**: "Walk pipeline com Introspector acessível durante execução".

**Estado**: PROPOSTO em P191A. ACEITE após P191C empirically validate.

**Contexto**: P190F identificou barreira arquitectural — walk fn não tem acesso a Introspector. Migração de helpers para Introspector path location-aware bloqueada.

**Decisão**: **Opção A** — walk fn signature change para `intr: &mut TagIntrospector`. Walk arms replicam `from_tags` logic incrementalmente. Pattern ADR-0069 stylesheet preserved.

**Consequências**:
- F1 desbloqueado (após P190G/H/I com retomar série).
- 4 defers resolvíveis.
- from_tags eliminado.
- Pattern ADR-0069 preservado.
- M5 universal completo inalterado.

**Detalhes**: ficheiro `00_nucleo/adr/typst-adr-0071-walk-pipeline-redesign.md`.

---

## §7 Compatibilidade ADR-0069

5 variantes operacionais ADR-0069 preserved após Opção A:
- P195D variante (não-locatable + snapshot+find_map): inalterada conceptualmente; helper signature change.
- P196B variante (locatable + body): idem.
- Cenário α (P197B, P198B): inalterado.
- Cenário α por construção (P199B): inalterado.
- Cenário β-promote (P198C): inalterado.

7 aplicações concretas funcionais.

---

## §8 Estado dormente vs activo (esperado pós-P191)

### Activado em P191

- Walk fn com `&mut TagIntrospector` accessible.
- 2 helpers walk-readers queryam Introspector.
- Walk arm Equation gate via location-aware.
- from_tags eliminado.

### Preservado

- 2 helpers walk-internal (compute_figure, compute_heading_for_toc).
- Pattern ADR-0069 stylesheet (5 variantes).
- Tags emit (para outros consumers eventualmente).
- M5 universal completo (0+0+0).

---

## §9 Próximo sub-passo concreto

**P191B**:

1. Walk fn signature change.
2. ~20 recursive walk calls actualizados (mecânico).
3. `introspect_with_introspector` simplificado.
4. 12 walk arms replicam from_tags logic.
5. `compute_heading_auto_toc` migrado (signature `<I: Introspector>(intr: &I, loc: Location, ...)`)
6. Walk arm Equation gate migrado.
7. `from_tags::from_tags` eliminado ou no-op.
8. Tests workspace verdes (Δ ≈ 0).

Magnitude: **M+**.

---

## §10 Restrições mantidas

- ✅ Zero código tocado em P191A.
- ✅ Zero testes modificados.
- ✅ ADR-0071 PROPOSTO criada em P191A.N (não ACEITE até P191C).
- ✅ Lembrete formal P190 retomar (em ficheiro dedicado P191A.N).
- ✅ Walk fn não modificado.
- ✅ Helpers não modificados.
- ✅ Trait Introspector não modificado.
- ✅ TagIntrospector não modificado.
- ✅ from_tags não modificado.
- ✅ Linguagem operacional sem inflação retórica.
- ✅ Plano P191B/C sem cláusulas condicionais.

---

## §11 Achado arquitectural significativo

**P191 introduz mecanismo arquitectural novo sem precedente directo no projecto**. Análoga a ADR-0068 (P185A — location-aware Layouter mecanismo).

P190F descobriu barreira; P191A propõe redesign; P191B+ implementa. Padrão "diagnóstico-primeiro → ADR-PROPOSTO → implementação → ADR-ACEITE" replicado.

---

## §12 Estado projectado pós-P191

- **P191 série**: A ✅ B-C pendentes.
- **Mecanismo Opção A**: implementado e validado.
- **2 helpers walk-readers migrados**.
- **Walk arm Equation gate migrado**.
- **from_tags eliminado**.
- **ADR-0071 ACEITE** (em P191C).
- **Pre-condição arquitectural cumprida** para retomar P190G.

---

## §13 Lembrete formal CRÍTICO — P190 série em pausa

**P190 série em pausa após P190F**. Retomar P190G após P191 fechar.

3 sub-passos restantes:
- **P190G** — Categoria 6 (Labels & TOC).
- **P190H** — Categoria 7 (Figures).
- **P190I** — Walk arms purification + Layouter final + struct elim + ADR-0070 ACEITE.

4 defers acumulados:
- `lang` (P190D).
- `numbering_active` (P190E).
- `flat` (P190F).
- `hierarchical` (P190F).

Após P191 fechar e P190G/H/I executados, M5 + M6 universalmente completos.

Lembrete formalizado em ficheiro dedicado `00_nucleo/p190-pause-resume-tracker.md` (P191A.N).

---

## §14 Linhagem

- **Pattern arquitectural**: ADR-0071 PROPOSTO (P191A).
- **Pre-condição barrier**: P190F §3.
- **ADR análogo**: ADR-0068 (location-aware Layouter mecanismo).
- **5 variantes operacionais ADR-0069**: preservadas.
- **7 aplicações ADR-0069 stylesheet**: preservadas (P195D + P196B + P197B + P198B + P198C + P199B + P200B).
- **Pattern stylesheet "diagnóstico-primeiro"**: 28ª aplicação consecutiva.
- **F1**: não fecha em P191; fecha após P190G/H/I.
- **F3**: não fecha em P191; fecha após P190G/H/I.

---

## §15 Métricas finais P191A

- **LOC produção**: 0 (zero código tocado em diagnóstico).
- **LOC teste**: 0.
- **LOC L0**: 0 (defer P191B).
- **LOC relatório + diagnóstico**: ~750.
- **LOC ADR-0071**: ~150 (a criar em P191A.N).
- **LOC lembrete formal**: ~50 (a criar em P191A.N).
- **Variants Content novas**: 0.
- **Sub-stores Introspector novos**: 0.
- **ADRs novas**: 1 (ADR-0071 PROPOSTO).
- **Helpers privados**: 0 mudanças.
- **F1 progresso**: defer P190G/H/I.
- **F3 progresso**: defer P190G/H/I.

**Achado arquitectural significativo**: P191 abre **ramo paralelo** ao M6 — redesign walk pipeline antes de retomar elimination cleanup. Honestidade arquitectural: barreira identificada → ADR-0071 → implementação → cleanup.
