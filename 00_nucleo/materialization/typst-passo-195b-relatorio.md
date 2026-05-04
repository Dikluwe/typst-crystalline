# Relatório P195b — `ElementPayload::Labelled` + ADR-0069 PROPOSTO

**Data**: 2026-05-04
**Magnitude**: S puro
**Pré-condição**: P195A concluído ✅; tests workspace 1.825
verdes; zero violations.

---

## Resumo

Variant nova adicionada:
- `ElementPayload::Labelled { label: Label, resolved_text:
  Option<String>, figure_number: Option<usize> }` em
  `01_core/src/entities/element_payload.rs:138-167`.

Stub no-op em `from_tags`:
- `ElementPayload::Labelled { .. } => {}` em
  `from_tags.rs` (cláusula gate trivial — match exaustivo
  força arm explícito; replica P186B/P186D pattern).

**ADR-0069 PROPOSTO criada**:
- `00_nucleo/adr/typst-adr-0069-post-recursion-tag-emission.md`.
- 8 secções padrão (Estado, Contexto, Decisão,
  Justificação, Alternativas, Consequências, Critério
  validação, Histórico).
- Documenta pattern arquitectural novo: walk arm emite
  Tag manualmente após recursão para state-dependent
  payload.
- Status PROPOSTO; transita ACEITE em P195E após tests
  E2E confirmarem paridade.

L0s actualizados (2):
- `entities/element_payload.md` — entrada Labelled +
  Histórico.
- `rules/introspect/from_tags.md` — entrada Histórico stub
  no-op.

5 tests unit cobrem variant: construção, equality,
distinção entre variants, distinção por label, hash
distinto, distinção por figure_number.

**Walk arm Labelled NÃO modificado** — preserva mutação
legacy directa de `state.resolved_labels` +
`state.figure_label_numbers`. P195D modifica.

`is_locatable(Content::Labelled) = false` mantido.
`extract_payload(Content::Labelled) = None` mantido (sem
arm — pattern arquitectural novo bypass este mecanismo).

Sem janela invariante quebrada — P186C/D aprendizado não
se aplica porque `is_locatable` não muda.

---

## Confirmação `.H` (14/14)

| # | Verificação | Estado |
|---|-------------|--------|
| 1 | `cargo check --workspace` passa | ✅ |
| 2 | `cargo test --workspace` passa (Δ +5 vs 1825) | ✅ 1830 verdes |
| 3 | `crystalline-lint .` zero violations | ✅ |
| 4 | Variant `ElementPayload::Labelled` construível | ✅ test `labelled_construivel_e_compara` |
| 5 | Stub no-op presente em `from_tags` | ✅ `from_tags.rs` arm Labelled |
| 6 | ADR-0069 PROPOSTO criada (8 secções) | ✅ `typst-adr-0069-post-recursion-tag-emission.md` |
| 7 | `is_locatable(Content::Labelled)` ainda `false` | ✅ `locatable.rs` arm intocado |
| 8 | `extract_payload(Content::Labelled)` retorna `None` | ✅ catch-all `_ => None` intocado |
| 9 | Walk arm Labelled NÃO modificado | ✅ `introspect.rs:432-486` confirmado por `git diff` |
| 10 | Trait `Introspector` NÃO modificado | ✅ |
| 11 | `TagIntrospector` NÃO modificado | ✅ |
| 12 | Consumer C4 NÃO modificado | ✅ |
| 13 | Snapshot tests verdes | ✅ incluídos em workspace |
| 14 | Linter passa final | ✅ |

---

## Δ tests vs baseline

- Baseline P195A: **1.825** verdes.
- Após P195B: **1.830** verdes.
- Δ: **+5** (limite superior do range +3 a +4 — cobertura
  mais completa do que mínimo).

5 tests novos:
- `labelled_construivel_e_compara`
- `labelled_distincao_de_outras_variants`
- `labelled_distingue_por_label`
- `labelled_hash_diferente_para_label_distinto`
- `labelled_figure_number_distingue_payloads`

---

## Hashes finais

L0s modificados (2):

| Ficheiro | Hash código | Hash prompt |
|----------|-------------|-------------|
| `entities/element_payload.md` | `2b440e36` | `86032faf` (em `.rs`) |
| `rules/introspect/from_tags.md` | `3a8f291a` (não-mudado) | `a52d9d63` (em `.rs`) |

ADR criada: `typst-adr-0069-post-recursion-tag-emission.md`
(sem hash — convenção ADR não usa).

`crystalline-lint --fix-hashes .` aplicado uma vez.
Análise final ✅ 0 drift warnings remaining.

---

## Decisões de execução notáveis

### ADR-0069 PROPOSTO criada

**Primeira ADR** desde P185E (ADR-0068 ACEITE).
Formaliza pattern arquitectural novo "post-recursion tag
emission for state-dependent payload" identificado em
P195A §11.1-§11.2.

Conteúdo principal (8 secções):
- **§2 Contexto**: padrão existente (`extract_payload` puro
  pre-recursion) cobre 8 casos. P195A identificou
  `Content::Labelled` como primeiro caso de
  state-dependent payload.
- **§3 Decisão**: walk arm emite Tag manualmente após
  recursão; sem `is_locatable=true`; sem `extract_payload`
  arm; `from_tags` arm processa normalmente.
- **§4-5 Justificação + Alternativas**: 4 alternativas
  rejeitadas com razões empíricas.
- **§7 Aplicabilidade futura**: pattern reutilizável para
  P196 Heading auto-toc, P197 Figure, P198.
- **§8 Critério validação**: ACEITE quando P195E
  confirmar paridade observable.

### Cláusula gate trivial em `from_tags` (esperada)

Match exaustivo sobre `ElementPayload` em `from_tags`
forçou arm explícito após adição de variant. Stub no-op
`ElementPayload::Labelled { .. } => {}` adicionado per
P186B/P186D pattern. P195C estende com populate
completo dos sub-stores.

### Convenção de inserção: ordem cronológica

Variant inserido após `Equation` (P186B), seguindo ordem
cronológica de adição. Convenção observada em P181C
(Bibliography), P186B (Equation), e agora P195B (Labelled).

### Sem cláusula gate substancial disparada

- `Label` import adicionado (cláusula gate trivial).
- Derives existentes cobrem `Labelled` automaticamente.
- Hash manual via `format!("{:?}", self)` cobre Labelled.
- Match exhaustivo em `from_tags` resolve via stub no-op.

---

## Estado actual

- **P195 série**: A ✅ B ✅ | C-E pendentes.
- **`ElementPayload`**: 10 → **11 variants**.
- **`ElementKind`**: 9 (inalterado — sem locatable).
- **Tests workspace**: 1.825 → **1.830** (+5).
- **ADRs**: 68 → **69** (ADR-0069 PROPOSTO).
- **Trait `Introspector`**: 19 métodos (inalterado).
- **`TagIntrospector` sub-stores**: 8 (inalterado).
- **66 passos executados** (P195A = 65 + P195B = 66).
- **DEBT M5-residual**: 2 pré-requisitos pendentes
  (inalterado — P195 trabalha em E4, não em
  pré-requisitos).
- **Padrão diagnóstico-primeiro**: 17ª aplicação consecutiva.

---

## Pendências cumulativas

Inalteradas em P195B:
- 5 excepções activas (E1, E2, E3, E5, E6).
- E4 fecha estruturalmente em P195D (walk arm migration).
- 2 pré-requisitos M5-residual restantes
  (`headings_for_toc`, `SetEquationNumbering`).

---

## Próximo passo

**P195C** — estender stub no-op `ElementPayload::Labelled
{ .. } => {}` em `from_tags` com populate completo:

- Match destructure `ElementPayload::Labelled { label,
  resolved_text, figure_number }`.
- `if let Some(text) = resolved_text { intr.resolved_labels
  .insert(label.clone(), text.clone()); }`.
- `if let Some(n) = figure_number { intr.figure_label_numbers
  .insert(label.clone(), *n); }`.
- Tests unit cobrindo populate.
- L0 `from_tags.md` actualizado.

Magnitude: S puro. Sem cláusulas condicionais. Walk arm
ainda não modificado em P195C — Tag não é emitida em
produção; tests populam manualmente para validar
populate.
