# Relatório P198A — Diagnóstico walks SetHeadingNumbering + CounterUpdate

**Data**: 2026-05-04
**Magnitude**: S puro (diagnóstico-primeiro)
**Estado**: Completo
**Pattern arquitectural relevante**: ADR-0069 — 4ª e 5ª aplicações.
**Variantes operacionais aplicadas**: cenário α (E5) + cenário β-promote (E6).

---

## §1 Sumário executivo

P198A audita os 2 últimos walks M5 não-residuais — E5
(`Content::SetHeadingNumbering`) e E6 (`Content::CounterUpdate`)
— para fechar série §9 P189 antes dos pré-requisitos paralelos
finais (`SetEquationNumbering` / sub-store `headings_for_toc`).

Auditoria empírica revela **estados divergentes** entre os 2 arms:
- **E5**: cenário α aplicável directamente (caminho Introspector
  activo desde P182C; refactor estilístico só).
- **E6**: cenário β-promote necessário (não-locatable;
  `extract_payload`/`from_tags` arms ausentes; CounterRegistry
  não populated via CounterUpdate).

P198 implementação granular em **3 sub-passos**:
- **P198B** (cenário α — S): declaração formal E5 + L0 + 5 tests.
- **P198C** (cenário β-promote — M): promote CounterUpdate +
  nova ElementPayload variant + extract_payload arm + from_tags
  arm + L0 + 5-6 tests.
- **P198D** (S): consolidado + DEBT.

Após P198: **1 excepção activa + 1 residuo** (E1 + E2-residuo);
2 pré-requisitos restantes inalterados.

---

## §2 Contexto

P198 é **passo 6 da sequência §9 P189**. Último passo da
sequência antes dos pré-requisitos paralelos:
- E1 fecha quando `Content::SetEquationNumbering` materializar.
- E2-residuo fecha quando sub-store `intr.headings_for_toc` abrir.

**Estado pré-P198**:
- 3 excepções activas + 1 residuo (E1, E2-residuo, E5, E6).
- 2 pré-requisitos restantes.
- Pattern ADR-0069 com 3 variantes operacionais (P195D, P196B,
  P197B).

**Esperado pós-P198**:
- 1 excepção activa + 1 residuo (E1, E2-residuo).
- 2 pré-requisitos restantes (inalterado — P198 fecha excepções,
  não pré-requisitos).
- Pattern ADR-0069 com 4 helpers no stylesheet (mas P198 sem
  novos helpers — distinção do padrão).

---

## §3 Mutações actuais por arm

### E5 — SetHeadingNumbering (introspect.rs:611-623)

```rust
state.numbering_active.insert("heading".to_string(), *active);
```

**1 mutação**.

### E6 — CounterUpdate (introspect.rs:625-642)

```rust
match action {
    CounterAction::Step =>
        if key == "heading" { state.step_hierarchical(...) }
        else                { state.step_flat(key) },
    CounterAction::Update(val) => state.update_flat(key, *val),
}
```

**3 caminhos** sob match.

---

## §4 Decisões cláusula 1–9

| # | Cláusula | Decisão | Magnitude |
|---|----------|---------|-----------|
| 1 | Variante E5 | Cenário α (caminho activo desde P182C) | S |
| 2 | Variante E6 | Cenário β-promote (locatable + new variant) | M |
| 3 | Helpers | 0 novos (arm trivial em E5; from_tags arm em E6) | 0 LOC |
| 4 | Ordem | Opção β — sub-passos separados (variantes diferentes) | — |
| 5 | Cadeia E5/E6 | Preservar mutação legacy (consumers `compute_*`) | 0 LOC |
| 6 | E1 interaction | Independente; sem trabalho em P198 | 0 LOC |
| 7 | Mutação legacy | Write paralelo M5 → cleanup M6 | 0 LOC |
| 8 | Critério fecho | E5+E6 estruturalmente; M5 universal NÃO fecha (E1, E2-residuo restam) | declaração L0 |
| 9 | Plano | P198B (S) + P198C (M) + P198D (S) | M- agregado |

---

## §5 Estados confirmados empiricamente

### E5 — Cenário α confirmado

| Componente | Estado |
|------------|--------|
| Variant `ElementPayload::StateUpdate` | ✅ existe (P171) |
| `is_locatable(SetHeadingNumbering)` | ✅ true (P182C) |
| `extract_payload` arm | ✅ retorna `Some(StateUpdate)` |
| `from_tags` arm StateUpdate | ✅ existe (popula StateRegistry) |
| Sub-store equivalente | ✅ `intr.state` (StateRegistry) |
| Consumer downstream | ✅ `compute_heading_auto_toc`, walk arm Equation |
| Cadeia E5 | ✅ preservada via mutação legacy |

### E6 — Cenário β-promote necessário

| Componente | Estado pré-P198 | Estado pós-P198C |
|------------|-----------------|-------------------|
| Variant `ElementPayload::CounterUpdate` | ❌ não existe | ✅ adicionar |
| `is_locatable(CounterUpdate)` | ❌ false | ✅ true |
| `extract_payload` arm | ❌ não existe | ✅ adicionar |
| `from_tags` arm | ❌ não existe | ✅ adicionar |
| Sub-store equivalente | ❌ CounterRegistry não populated | ✅ via apply_at + apply_hierarchical_at |
| Consumer downstream | ✅ `compute_*` helpers (lêm legacy) | ✅ inalterado (legacy preservada) |
| Cadeia E6 | ✅ preservada via mutação legacy | ✅ preservada |

---

## §6 Cláusula gate substancial — cadeia E5/E6 ↔ helpers compute_*

`compute_heading_auto_toc` (P196B) lê `state.is_numbering_active("heading")`.
`compute_labelled` Equation arm (P195D) lê `state.get_flat("equation")`.
`compute_heading_auto_toc` lê `state.format_hierarchical("heading")`.
`compute_figure` (P197B) lê `state.local_figure_counters`.

Mutações legacy `numbering_active.insert` (E5) e `step_*`/`update_flat`
(E6) **DEVEM ser preservadas** durante M5 — cláusula gate
substancial análoga a cadeia E2-E3 (P197A).

**Mitigação**: write paralelo M5 preservado em ambos arms.
Cleanup orgânico em M6 quando `compute_*` helpers migrarem
para sub-stores Introspector (ou eliminarem-se via remoção
de CounterStateLegacy).

---

## §7 Plano de sub-passos

| Sub | Escopo | Magnitude |
|-----|--------|-----------|
| **P198B** | Walk arm SetHeadingNumbering — declaração formal cenário α + L0 + 5 tests | **S** |
| **P198C** | Promote `Content::CounterUpdate`: locatable.rs + extract_payload.rs + element_payload.rs + from_tags.rs + walk arm preservation + L0 + 5-6 tests E2E | **M** |
| **P198D** | Auditoria + relatório consolidado P198 + DEBT M5-residual | **S** |

**Total agregado**: M- (~85 LOC produção + ~200 LOC tests + ~130 LOC L0 + relatórios).

---

## §8 Magnitude consolidada

- **P198A**: S puro. ~300 LOC diagnóstico + relatório.
- **P198B**: S. ~5 LOC produção (comentário) + ~80 LOC tests + ~50 LOC L0.
- **P198C**: M. ~80 LOC produção (variant + 2 arms) + ~120 LOC tests + ~80 LOC L0.
- **P198D**: S puro. ~250 LOC consolidado.

Total agregado: ~930 LOC documentação/relatórios + ~285 LOC
código/tests cristalinos.

---

## §9 ADR avaliação

- 3 variantes ADR-0069 cobrem ambos arms.
- Cenário α (E5) replica P197B directamente.
- Cenário β-promote (E6) é variação operacional do P196B variante (locatable + Tag) aplicada a leaf content sem body recursivo. Pattern ADR-0069 stylesheet ainda aplica (helper opcional dispensado porque from_tags arm directamente popula sub-store).
- Sem decisão arquitectural nova.

**Conclusão**: **não cria ADR**.

---

## §10 DEBT M5-residual avaliação

- **Antes P198**: 3 excepções activas + 1 residuo.
- **Após P198**: **1 excepção activa + 1 residuo** (E1, E2-residuo).
- **2 pré-requisitos restantes** (inalterado — P198 fecha excepções E5+E6 que não tinham pré-requisitos M5-residual; eram bloqueadas só por análise insuficiente).

**Cenário B continua** (sem DEBT formal aberto).

---

## §11 Estado dormente vs activo (esperado pós-P198)

### Activo

- **Caminho Introspector E5**: StateRegistry populated via Tag::StateUpdate (já activo desde P182C).
- **Caminho Introspector E6** (após P198C): CounterRegistry populated via CounterUpdate Tag → `apply_at`/`apply_hierarchical_at`.
- Helpers `compute_*` continuam a ler legacy (write paralelo).

### Dormente / continua legacy

- Mutações legacy preservadas em ambos walk arms.
- `compute_heading_auto_toc`, `compute_labelled`, `compute_figure` continuam a ler `state.numbering_active`, `state.flat`, `state.hierarchical`, `state.figure_numbers` directamente.
- Cleanup orgânico em M6.

---

## §12 Próximo sub-passo concreto

**P198B — Walk arm SetHeadingNumbering cenário α**:

1. Confirmar walk arm `introspect.rs:611-623`.
2. Adicionar comentário inline P198B declarando E5 fechada estruturalmente via cenário α.
3. Actualizar L0 introspect.md (tabela Excepções M5; secção nova).
4. Adicionar 5 tests sentinela:
   - `set_heading_numbering_extract_payload_emite_state_update`.
   - `set_heading_numbering_from_tags_popula_state_registry`.
   - `set_heading_numbering_paridade_legacy_vs_introspector`.
   - `compute_heading_auto_toc_le_numbering_active_legacy`.
   - `walk_arm_preserva_write_paralelo_legacy_para_compute_helpers`.
5. `crystalline-lint --fix-hashes` para actualizar hash L0.

**Critério de fecho P198B**: tests workspace 1.848 + 5 = 1.853 verdes; tests sentinela P189B preservados; lint zero violations; L0 hash actualizado.

---

## §13 Restrições mantidas

- ✅ Zero código tocado em P198A (passo diagnóstico-primeiro).
- ✅ Zero testes modificados.
- ✅ Sem reservas de identificadores criadas.
- ✅ Walk não modificado.
- ✅ `from_tags` não tocado.
- ✅ Trait `Introspector` não modificado.
- ✅ `TagIntrospector` não modificado.
- ✅ Consumer C3/C4 não modificados.
- ✅ Linguagem operacional sem inflação retórica.
- ✅ Regra dos 2 eixos aplicada por arm (§1.13 do diagnóstico).
- ✅ Pattern ADR-0069 + 3 variantes reutilizados.
- ✅ Plano P198B/C/D sem cláusulas condicionais.
- ✅ ADR não criada.
- ✅ DEBT formal não aberto.

---

## §14 Linhagem

- Pattern ADR-0069 (PROPOSTO P195B; ACEITE P195E).
- Variante cenário α (P197B) aplicada em P198B (E5).
- Variante β-promote (P196B variante operacional adaptada) aplicada em P198C (E6).
- Reuso `ElementPayload::StateUpdate` (P171/P173) em E5.
- Nova variant `ElementPayload::CounterUpdate` em E6 (P198C).
- Sub-stores consumidos:
  - E5: `intr.state` (StateRegistry P171).
  - E6: `intr.counters` (CounterRegistry P184B).
- Consumer downstream `compute_*` helpers (P195D Equation; P196B Heading; P197B Figure) — preservados; lêm legacy durante M5.
- L0 alvo: `00_nucleo/prompts/rules/introspect.md` (a actualizar em P198B/C).
- Padrão diagnóstico-primeiro: 20ª aplicação consecutiva.
