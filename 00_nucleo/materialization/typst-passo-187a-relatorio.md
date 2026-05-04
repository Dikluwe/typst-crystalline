# Relatório P187A — Diagnóstico C1 heading prefix migration

**Data**: 2026-05-03
**Magnitude**: S (diagnóstico puro)
**Pré-condição**: P186 série fechada; ADR-0068 ACEITE; tests
workspace 1.801 verdes; zero violations.

---

## §1 Escopo

P187A é o passo de diagnóstico-primeiro que precede a
migração C1 (heading prefix em `mod.rs:345`). Replica
registo de P181A/P182A/P183A/P184A/P185A/P186A.

P187A produziu 2 artefactos documentais e zero código:

- `00_nucleo/diagnosticos/diagnostico-c1-heading-prefix-passo-187a.md` (8 secções).
- `00_nucleo/materialization/typst-passo-187a-relatorio.md` (este ficheiro, 14 secções).

Sem ADR nova. Sem DEBT novo. Sem código tocado.

---

## §2 Inputs verificados empiricamente (8 grep/read)

| # | Input | Resultado |
|---|-------|-----------|
| 1 | Site C1 actual | `mod.rs:345` `self.counter.format_hierarchical("heading")` (linha 310 da spec desactualizada — P185C/P186 introduziram código antes; site real é 345) |
| 2 | `self.introspector` acesso | OK; já consultado em `mod.rs:341-343` (P182D `is_numbering_active`) |
| 3 | `self.current_location` acesso | OK; `pub(super) Option<Location>` em `mod.rs:131-141` (P185C) |
| 4 | `formatted_counter_at` API | OK; trait method P177 `(&str, Location) -> Option<String>` |
| 5 | `current_location` populated antes do site | OK; gating `advance_locator_if_locatable` precede match arm; Heading é locatable |
| 6 | `format_hierarchical` legacy | `counter_state_legacy.rs:126` retorna `Option<String>` (mesma shape) |
| 7 | P185D `.E` blueprint | `pipeline_e2e_is_numbering_active_at_via_current_location`; adapt directamente |
| 8 | P183B aprendizado | `formatted_counter` snapshot-final → preempt fallback. `_at` location-aware corrige. |

Crítico descoberto: **site C1 está em mod.rs:345** (não 310
como spec indica). Diferença irrelevante para
implementação; apenas atualizar referência na materialização
P187B.

---

## §3 Decisões cláusulas 1–6 (resumo)

| # | Cláusula | Decisão |
|---|----------|---------|
| 1 | Forma da expressão | **Combinação Opção B + Opção A**: `current_location.and_then(...).or_else(legacy)` |
| 2 | `None` do Introspector | **Opção A**: `or_else` para legacy `format_hierarchical` (replica P184D) |
| 3 | `None` do `current_location` | **Opção B**: `and_then` defensivo (sem panic) |
| 4 | P183B aprendizado | **Não-aplicável** após P185 — primitiva location-aware corrige |
| 5 | Forma migração | substitution-with-fallback per P184D padrão |
| 6 | Critério fecho | **Opção 3**: consumer migrado + tests E2E + actualização DEBT |

Forma final da expressão:

```rust
self.current_location
    .and_then(|loc| self.introspector.formatted_counter_at("heading", loc))
    .or_else(|| self.counter.format_hierarchical("heading"))
```

---

## §4 Plano de sub-passos B (sem condicionais)

**Sub-passo único agregado**:

| Sub-passo | Escopo | Magnitude |
|-----------|--------|-----------|
| `.B` | Migrar `mod.rs:345` + L0 + tests E2E paridade (incl. re-update H1-H2-H1) + actualização nota DEBT M4-residual + relatório consolidado P187 | S |

Granularidade inferior a P186 (5 sub-passos) porque migração
é linha única + tests directos. Coeso.

---

## §5 Magnitude agregada

**P187 série = S puro** (1×S agregado em sub-passo único).

Diferente de P186 (S agregado em 6 sub-passos para
infraestrutura nova). P187 é migração de consumer único —
infra completa (P185 + P177).

---

## §6 Dependências de outros passos

### §6.1 — Pré-requisitos (cumpridos)

- `formatted_counter_at(key, location)` no trait (P177).
- `current_location: Option<Location>` no Layouter (P185C).
- Sincronização Locator validada (P185D).
- ADR-0068 ACEITE (P185E).
- Heading locatable confirmado por construção (P164/P186).

### §6.2 — Dependentes

- **DEBT M4-residual fecha** após P187 + P188 (cobre apenas
  C2 entre P187 e P188).

### §6.3 — Independente

- **P186 (Equation locatable)** — independente; serve C2
  apenas, não C1.
- **P188 (C2 migration)** — pode prosseguir em paralelo;
  blueprint P186F similar.

---

## §7 ADR avaliação

**Sem ADR criada.** Substitution-with-fallback é padrão
estabelecido (P184D Figure, P181G Cite, P182D
heading-arm-numbering-active). Decisão atómica registada no
diagnóstico §2.

---

## §8 DEBT avaliação

### Cenário B identificado

`grep` em `00_nucleo/` confirmou que **DEBT M4-residual não
foi formalmente aberto** (P183F não executado). Apenas notas
preventivas em relatórios consolidados (P184F, P185, P186)
mencionam C1+C2 informalmente.

**Acção P187B**: relatório P187 actualiza a nota
preventiva indicando:
- Após P187: cobre apenas **C2**.
- Quando P188 fechar C2 + P183F formalizar (se ainda
  necessário), DEBT M4-residual completo fecha.

Sem DEBT formal aberto/editado em P187 — apenas trabalho
documental.

---

## §9 Restrições honradas

- **Zero código tocado** em qualquer ficheiro fora de
  `00_nucleo/`.
- **Zero testes** modificados.
- **Sem reservas de identificadores**.
- **Não modifica trait `Introspector`** (P185B fechou).
- **Não modifica Layouter struct** (P185C fechou).
- **Não migra consumer C1** — P187B.
- **Sem inflação retórica**.
- **Sem cláusulas condicionais nos sub-passos**.

---

## §10 Verificações

- ✅ `cargo check --workspace` passa (sem código tocado).
- ✅ `cargo test --workspace` passa: **1.801** inalterado vs
  P186F.
- ✅ `crystalline-lint .` zero violations.
- ✅ Diagnóstico produzido (8 secções).
- ✅ Relatório produzido (este ficheiro).
- ✅ Sem ADR nova.
- ✅ Sem DEBT novo.

---

## §11 Achados não-triviais

### §11.1 — Site C1 actualmente em mod.rs:345 (não 310)

Spec referencia "mod.rs:310" — referência herdada de P183B
e P184F. Após P185C e P186 introduzirem código antes do
arm Heading, site real é mod.rs:345. Diferença irrelevante
para implementação; corrigida em P187B materialização.

### §11.2 — `format_hierarchical` legacy e `formatted_counter_at` partilham shape `Option<String>`

Ambos retornam `Option<String>` formatado como `"1.2.3"`.
Substituição directa de uma por outra preserva tipo na
expressão. Migração é trivial em forma.

### §11.3 — P183B aprendizado completamente preservado

P183B falhou por escolha errada de primitiva
(`formatted_counter` em vez de `formatted_counter_at`).
Resto da estrutura (substitution-with-fallback) era
correcta. P187 valida P183B retroactivamente: a estratégia
era certa; a primitiva precisava de adaptação.

P185 (4 sub-passos) foi necessário para construir
`formatted_counter_at` location-aware E `current_location`
no Layouter. Sem P185, P183B aprendizado seria
inaplicável.

### §11.4 — Heading-arm já consulta Introspector via P182D

Linha `mod.rs:341-343` chama `self.introspector
.is_numbering_active("numbering_active:heading")` (snapshot
final) com fallback `|| self.counter.is_numbering_active("heading")`.

Esta consulta usa `is_numbering_active` (não `_at`) porque:
- Numbering activo é state binário; `Bool(true)` final é
  o que importa para gating.
- Para counter (C1), o valor depende da position no
  documento — daí precisar de `_at`.

P187 estende o pattern P182D do site mas com primitiva
location-aware. Coerência com decisão arquitectural.

### §11.5 — DEBT M4-residual cenário B confirmado

Sem ficheiro `DEBT-M4-residual.md` em `00_nucleo/`. Notas
preventivas em relatórios mencionam C1+C2 informalmente.
P187B não abre DEBT formal — apenas actualiza nota.

---

## §12 Snapshot pós-P187A

- **Tests workspace**: 1.801 (inalterado).
- **Trait `Introspector`**: 18 métodos (inalterado).
- **Layouter**: `current_location` + `locator` (inalterado).
- **ADR-0068**: ACEITE (P185E).
- **DEBT M4-residual**: cobre C1 + C2 (inalterado;
  reduzirá para C2 após P187B).
- **53 passos executados** (após P186F + P187A = 53).

Wait — **54 passos executados** após P187A. Recontagem:
- P185-consolidado contou 48.
- P186A = 49, B = 50, C = 51, D = 52, E = 53, F = 54.
- P187A = 55.

Actualizo: **55 passos executados**.

- **Padrão diagnóstico-primeiro**: 12ª aplicação consecutiva
  (adicionada P187A à lista P131A/132A/140A/148/154A/181A/182A/183A/184A/185A/186A).

---

## §13 Próximo passo

**P187B** — migração C1 + tests E2E + actualização nota
DEBT M4-residual:

- Editar `01_core/src/rules/layout/mod.rs:345`:
  - Substituir `self.counter.format_hierarchical("heading")`
    pela expressão substitution-with-fallback location-aware.
- Editar L0 `00_nucleo/prompts/rules/layout.md`:
  - Secção sobre heading-arm migração (P184D padrão).
- Tests E2E:
  - `c1_heading_prefix_via_introspector_path`.
  - `c1_heading_prefix_via_fallback_legacy`.
  - `c1_heading_prefix_paridade_legacy_vs_migrated`.
  - `c1_heading_prefix_re_update_correctness` (H1-H2-H1
    sequence — empiricamente valida que P183B aprendizado é
    abordado por P185).
- Actualizar nota DEBT M4-residual no relatório P187
  consolidado.

Magnitude: S puro. Sem cláusulas condicionais.

---

## §14 Conclusão

P187A fechou 6 cláusulas com decisão literal e plano em
sub-passo único. Magnitude S agregada confirmada para P187.
ADR avaliada e dispensada (replicação de padrão P184D). DEBT
avaliado: cenário B (sem DEBT formal; nota preventiva).

P183B aprendizado validado: estratégia substitution-with-fallback
correcta; primitiva precisava de adaptação (P185
forneceu).

Após P187B, **C1 fechado** com Introspector como **caminho
funcional real** (diferente de C2 em P186, onde Introspector
fica dormente). DEBT M4-residual reduz a apenas C2.

P187 → P188 → fim de M4-residual → segue M5 (P189). Três
passos para fechar fase.

Padrão diagnóstico-primeiro mantido — 12/12 acertaram a
magnitude planeada ±1 nível.
