# Relatório P197A — Diagnóstico walk arm Figure

**Data**: 2026-05-04
**Magnitude**: S puro (diagnóstico-primeiro)
**Estado**: Completo
**Pattern arquitectural relevante**: ADR-0069 — 3ª aplicação potencial.
**Variante candidata**: P196B (locatable, `emitted_loc` directo).

---

## §1 Sumário executivo

P197A audita o walk arm `Content::Figure` para fechar **E3** —
penúltima excepção M5 não-residual (E1 + E5 + E6 ainda
activas). Auditoria empírica confirma **cenário α**: variant
`ElementPayload::Figure` cobre semântica; sub-store
equivalente para `state.figure_numbers` já existe via
CounterRegistry (P184B); consumer C3 já usa Introspector path
via `figure_number_at_index` (P184C/D); `local_figure_counters`
é walk-internal sem consumer.

P197B é refactor mínimo:
- Extrair helper `compute_figure(state, kind, is_counted)`.
- Preservar mutação legacy como write paralelo M5 (cláusula
  gate substancial **resolvida sem disparar gate** porque
  `compute_labelled` Figure arm P195D ainda lê
  `state.figure_numbers`).
- Declarar E3 fechada estruturalmente em L0.

P197C é encerramento documental.

---

## §2 Contexto

P197 é **passo 5 da sequência §9 P189**: migrar walk arm Figure
para emitir Tag em vez de mutar state directamente. Pattern
ADR-0069 disponível desde P195E.

**Estado pré-P197**:
- 4 excepções activas + 1 residuo (E1, E2-residuo, E3, E5, E6).
- 2 pré-requisitos restantes (`headings_for_toc` para
  E2-residuo; `SetEquationNumbering` para E1).
- Pattern ADR-0069 com 2 aplicações concretas (P195D + P196B).

**Esperado pós-P197B**:
- 3 excepções activas + 1 residuo (E1, E2-residuo, E5, E6).
- 2 pré-requisitos restantes (inalterado).
- E3 fechada estruturalmente — diferente de E2 que ficou com
  residuo.

---

## §3 Mutações actuais walk arm Figure

`introspect.rs:490-519`. Sob gate `numbering.is_some() && caption.is_some()`:

1. `state.local_figure_counters.entry(kind_key).or_insert(0); *counter += 1;`
2. `state.figure_numbers.entry(kind_key).or_default().push(figure_number);`

---

## §4 Decisões cláusula 1–7

| # | Cláusula | Decisão | Magnitude |
|---|----------|---------|-----------|
| 1 | Forma do payload | Cenário α — variant cobre, sem mudança | 0 LOC |
| 2 | Helper `compute_figure` | Opção α — extrair (consistência ADR-0069) | ~15 LOC |
| 3 | `local_figure_counters` | Opção α — walk-internal, sem sub-store novo | 0 LOC |
| 4 | Cadeia E2-E3 | Opção α — preservar mutação legacy (write paralelo M5) | 0 LOC |
| 5 | Locator handling | Variante P196B disponível mas não aplicada (cenário α) | 0 LOC |
| 6 | Mutação legacy | Opção α — write paralelo M5 → cleanup M6 | 0 LOC |
| 7 | Critério fecho | E3 fecha estruturalmente em P197B | declaração L0 |

---

## §5 Cenário α confirmado

| Componente | Estado |
|------------|--------|
| Variant `ElementPayload::Figure` | ✅ existe (P184B + P168) |
| `is_locatable(Figure)` | ✅ true |
| `extract_payload(Figure)` | ✅ retorna Some |
| `from_tags` arm Figure | ✅ existe; popula 4 sub-stores |
| Sub-store equivalente `figure_numbers` | ✅ via CounterRegistry (chave `figure:{kind}`) |
| Consumer C3 (`mod.rs:484`) | ✅ usa Introspector path (P184D) |
| `local_figure_counters` consumer | ❌ nenhum (walk-internal apenas) |
| Cadeia E2-E3 | ✅ confirmada empiricamente |

**Implicação**: P197 não precisa de novo Tag pós-recursão, novo
sub-store, ou nova variant. Caminho Introspector já activo.

---

## §6 Cláusula gate substancial — cadeia E2-E3

`compute_labelled` Figure arm (P195D, introspect.rs:344-365)
lê `state.figure_numbers.last()` durante walk. Se mutação
legacy for removida em P197B, `compute_labelled` quebra para
target Figure dentro de wrapper Labelled.

**Mitigação cenário α**: mutação legacy `state.figure_numbers.push`
preservada como write paralelo M5. Cadeia E2-E3 continua
funcional. `compute_labelled` não é tocado.

**Cleanup M6 (futuro)**: `compute_labelled` Figure arm migra
para ler de CounterRegistry via API location-aware
(`flat_counter_at("figure:{kind}", current_location)` ou
similar). Mutação legacy torna-se redundante, removível.

**Cláusula gate substancial resolvida sem disparar gate**.

---

## §7 Plano de sub-passos

| Sub | Escopo | Magnitude |
|-----|--------|-----------|
| **P197B** | Walk arm Figure refactor + helper + L0 + 5 tests | S/M |
| **P197C** | Auditoria + relatório consolidado P197 + DEBT M5-residual | S |

**Total agregado**: S+ a M-. Significativamente menor que P196B+P196C.

---

## §8 Magnitude consolidada

- **P197A**: S puro. ~250 LOC diagnóstico + relatório.
- **P197B**: S/M. ~30 LOC produção + ~80 LOC testes + ~50 LOC L0.
- **P197C**: S puro. ~200 LOC consolidado.

Total agregado: ~610 LOC documentação/relatórios + ~110 LOC
código/tests cristalinos.

---

## §9 ADR avaliação

- Pattern ADR-0069 reusado conceptualmente (helper privado).
- Sem decisão arquitectural nova.

**Conclusão**: **não cria ADR**.

---

## §10 DEBT M5-residual avaliação

- **Antes P197**: 4 excepções activas + 1 residuo.
- **Após P197B**: 3 excepções activas + 1 residuo (E1, E2-residuo, E5, E6).
- **2 pré-requisitos restantes** (inalterado).

**Cenário B continua** (sem DEBT formal aberto).

---

## §11 Estado dormente vs activo (esperado pós-P197B)

### Activo

- Caminho Introspector para figure numbering: consumer C3
  (`mod.rs:484`) recebe Some via `figure_number_at_index`;
  fallback legacy raramente disparado.
- `intr.figure_label_numbers` populated quando `is_counted &&
  label` (P168 + P195D combinados).
- `intr.kind_index[Figure]` populated.
- `intr.counters` chaves `figure:{kind}` + `figure` populated.

### Dormente / continua legacy (write paralelo M5)

- `state.figure_numbers.push` continua activo — `compute_labelled`
  Figure arm (P195D) lê dela. Cleanup orgânico em M6 quando
  `compute_labelled` migrar para CounterRegistry.
- `state.local_figure_counters` continua activo — walk-internal
  trivial; sem consumer downstream. Cleanup orgânico em M6.

---

## §12 Próximo sub-passo concreto

**P197B — Walk arm Figure refactor (cenário α)**:

1. Adicionar helper `compute_figure(state, kind, is_counted) -> Option<usize>`.
2. Refactor walk arm Figure para chamar helper.
3. Actualizar L0 introspect.md (tabela Excepções M5: E3 → fechada estruturalmente; nova secção "Walk arm Figure migrado P197B").
4. Adicionar 5 testes E2E (cenário α sentinelas).
5. `crystalline-lint --fix-hashes` para actualizar hash L0.

**Critério de fecho P197B**: tests workspace 1.843 + 5 = 1.848
verdes; tests sentinela E3 P189B inalterados; lint zero
violations.

---

## §13 Restrições mantidas

- ✅ Zero código tocado em P197A (passo diagnóstico-primeiro).
- ✅ Zero testes modificados.
- ✅ Sem reservas de identificadores criadas.
- ✅ Walk não modificado.
- ✅ `from_tags` não tocado.
- ✅ Trait `Introspector` não modificado.
- ✅ `TagIntrospector` não modificado.
- ✅ Consumer C4 não modificado.
- ✅ Walks SetHeadingNumbering + CounterUpdate não migrados.
- ✅ Linguagem operacional sem inflação retórica.
- ✅ Regra dos 2 eixos aplicada (§1.11 do diagnóstico).
- ✅ Pattern ADR-0069 + variante P196B avaliados (não aplicados em
  P197B porque cenário α dispensa Tag pós-recursão).
- ✅ Plano P197B sem cláusulas condicionais.

---

## §14 Linhagem

- Pattern ADR-0069 (PROPOSTO P195B; ACEITE P195E).
- Variante P196B (locatable, `emitted_loc` directo) avaliada — não aplicada por cenário α.
- Helper análogo a `compute_labelled` (P195D) e `compute_heading_auto_toc` (P196B).
- Sub-store consumido: `intr.counters` (CounterRegistry P184B);
  `intr.figure_label_numbers` (P168); `intr.kind_index` (existente).
- Consumer C3: `references.rs::layout_ref` figure ref-arm
  (P184D substitution-with-fallback).
- L0 alvo: `00_nucleo/prompts/rules/introspect.md` (a actualizar em P197B).
- Padrão diagnóstico-primeiro: 19ª aplicação consecutiva.
