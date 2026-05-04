# Relatório P195c — `from_tags` arm `Labelled` funcional

**Data**: 2026-05-04
**Magnitude**: S puro
**Pré-condição**: P195B concluído ✅; tests workspace 1.830
verdes; zero violations.

---

## Resumo

Stub no-op P195B substituído por arm funcional em
`from_tags.rs`:

```rust
ElementPayload::Labelled {
    label,
    resolved_text,
    figure_number,
} => {
    if let Some(text) = resolved_text {
        intr.resolved_labels
            .insert(label.clone(), text.clone());
    }
    if let Some(n) = figure_number {
        intr.figure_label_numbers
            .insert(label.clone(), *n);
    }
}
```

Comportamento:
- `resolved_text` Some → popula `intr.resolved_labels`.
- `figure_number` Some → popula `intr.figure_label_numbers`.
- Ambos None → arm não panica; sub-stores não tocados
  para esta key (caso edge "target não-resolvível" per
  walk arm legacy `_ => None`).

Walk arm Labelled **NÃO modificado** — preserva mutação
legacy directa. Tags Labelled não chegam a `from_tags` em
produção até P195D activar walk arm emit.

L0 `rules/introspect/from_tags.md` actualizado com
entrada Histórico P195C.

4 tests unit cobrem casos:
- Populate básico (resolved_text Some).
- Populate com figure_number Some.
- Caso edge ambos None (sem panic).
- Múltiplos labels isolados.

---

## Confirmação `.E` (13/13)

| # | Verificação | Estado |
|---|-------------|--------|
| 1 | `cargo check --workspace` passa | ✅ |
| 2 | `cargo test --workspace` passa (Δ +4 vs 1830) | ✅ 1834 verdes |
| 3 | `crystalline-lint .` zero violations | ✅ |
| 4 | Arm `ElementPayload::Labelled` funcional (não-stub) | ✅ `from_tags.rs` |
| 5 | `intr.resolved_labels.insert` chamado quando `Some(text)` | ✅ tests |
| 6 | `intr.figure_label_numbers.insert` chamado quando `Some(n)` | ✅ test `figure_number_some` |
| 7 | Walk arm Labelled NÃO modificado | ✅ `introspect.rs:432-486` confirmado |
| 8 | Tags Labelled não chegam a `from_tags` em produção | ✅ walk arm não emite até P195D |
| 9 | Trait `Introspector` NÃO modificado | ✅ |
| 10 | `TagIntrospector` NÃO modificado | ✅ |
| 11 | Consumer C4 NÃO modificado | ✅ |
| 12 | Snapshot tests verdes | ✅ |
| 13 | Linter passa final | ✅ |

---

## Δ tests vs baseline

- Baseline P195B: **1.830** verdes.
- Após P195C: **1.834** verdes.
- Δ: **+4** (range planeado +3 a +4).

4 tests novos:
- `labelled_arm_popula_resolved_labels`
- `labelled_arm_popula_figure_label_numbers_quando_some`
- `labelled_arm_resolved_text_none_nao_popula`
- `labelled_arm_multiplos_labels_isolados`

---

## Hashes finais

L0 modificado: `rules/introspect/from_tags.md`

- Hash código (no L0): `b982323d`
- Hash prompt (`@prompt-hash` no `.rs`): `e7647593`

`crystalline-lint --fix-hashes .` aplicado uma vez.
Análise final ✅ 0 drift warnings remaining.

---

## Decisões de execução notáveis

### Helper `labelled_payload` em tests

Para reduzir verbosidade dos tests (variant tem 3 campos
+ Option<String>), introduzi helper privado em `mod tests`:

```rust
fn labelled_payload(
    label: &str,
    resolved_text: Option<&str>,
    figure_number: Option<usize>,
) -> ElementPayload {
    ElementPayload::Labelled {
        label:         lbl(label),
        resolved_text: resolved_text.map(String::from),
        figure_number,
    }
}
```

Replica padrão `equation_payload`, `heading_payload`
existentes. Reduz verbose noise nos tests.

### Sem cláusula gate substancial disparada

- `ResolvedLabelStore::insert` confirmou `pub(crate)`
  (P193B `.B`).
- `figure_label_numbers` em `TagIntrospector` é `pub`
  (per `introspector.rs`); `.insert(label.clone(), n)`
  directo.
- `Label` tem `Clone` derivado.
- Caso edge `None` ambos não panica (if let chains).

### Pattern arquitectural P195 em construção

P195C completa o lado **consumer** do pattern post-recursion
(ADR-0069). P195D completa o lado **producer** (walk arm
emite Tag).

Estado intermédio actual:
- Variant declarado ✅ (P195B).
- ADR PROPOSTO ✅ (P195B).
- `from_tags` arm funcional ✅ (P195C).
- Walk arm emit ainda **NÃO** (P195D).
- Em produção: Tags Labelled não emitidas → arm
  funcional nunca chamado em runtime real → output
  observable inalterado.

---

## Estado actual

- **P195 série**: A ✅ B ✅ C ✅ | D-E pendentes.
- **`ElementPayload`**: 11 variants (inalterado vs P195B).
- **`from_tags` arm Labelled**: stub no-op (P195B) →
  funcional (P195C).
- **Tests workspace**: 1.830 → **1.834** (+4).
- **ADRs**: 69 (ADR-0069 PROPOSTO ainda).
- **67 passos executados** (P195B = 66 + P195C = 67).
- **DEBT M5-residual**: 2 pré-requisitos (inalterado).
- **Excepção E4**: ainda activa em produção (walk arm
  legacy mutação preservada; sub-store `intr.resolved_labels`
  vazio em runtime real).

---

## Pendências cumulativas

Inalteradas em P195C:
- E1, E2, E3, E5, E6 activas.
- E4 fecha estruturalmente em P195D (não em P195C).
- 2 pré-requisitos M5-residual restantes.

---

## Próximo passo

**P195D** — walk arm Labelled emite Tag pós-recursão.
Magnitude **M** — primeiro pattern post-recursion
materializado em walk; pattern arquitectural ADR-0069
aplicado pela primeira vez.

Passos:
1. Editar `01_core/src/rules/introspect.rs:432-486`:
   - Após walk recursivo do target, computar
     `(resolved_text, figure_number)` (replica lógica
     actual).
   - **Manter** mutação legacy (write paralelo durante
     janela compat M5).
   - Emitir `Tag::Start(loc, ElementInfo::new(
     ElementPayload::Labelled { label, resolved_text,
     figure_number }))` + `Tag::End(loc, 0)` manualmente
     em `tags`.

2. Helper privado `compute_labelled(target, state, label)
   -> (Option<String>, Option<usize>)` em
   `introspect.rs` (per P195A §11.6).

3. Tests E2E em `tests.rs` (ou similar) confirmam:
   - Sub-store `intr.resolved_labels` populated em
     produção real (após walk).
   - Consumer C4 (P194B) começa a receber `Some(text)`
     para explicit labels.
   - Paridade observable preservada (mutação legacy
     paralela continua activa).

P195D fecha **E4 estruturalmente**. P195E é tests E2E +
relatório consolidado P195.
