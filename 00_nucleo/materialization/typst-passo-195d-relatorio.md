# Relatório P195d — Walk arm Labelled emite Tag pós-recursão

**Data**: 2026-05-04
**Magnitude**: M (genuíno) — primeiro pattern post-recursion
materializado em walk; ADR-0069 aplicada pela primeira vez.
**Pré-condição**: P195C concluído ✅; tests workspace 1.834
verdes; zero violations.

---

## Resumo

Walk arm `Content::Labelled` em
`01_core/src/rules/introspect.rs:Content::Labelled`
modificado para emitir Tag pós-recursão (pattern ADR-0069):

1. **Helper privado `compute_labelled(target, state) ->
   (Option<String>, Option<usize>)`** introduzido.
   Replica lógica legacy (Heading/Equation/Figure match)
   sem mutação. Usado pelo walk arm para computar
   `resolved_text` e `figure_number` em paralelo.

2. **Mutação legacy preservada** (write paralelo durante
   janela compat M5):
   - `state.figure_label_numbers.insert(label, n)` se
     `figure_number.is_some()`.
   - `state.resolved_labels.insert(label, text)` se
     `resolved_text.is_some()`.

3. **Tag pós-recursão emitida**:
   - Snapshot `tags.len()` antes da recursão.
   - Após recursão, `find_map` para primeira `Tag::Start`
     no range novo → reuso de Location do target.
   - Push `Tag::Start(loc, ElementInfo::new(
     ElementPayload::Labelled { label, resolved_text,
     figure_number }))` + `Tag::End(loc, 0)`.

4. **Reuso de Location preserva sincronização ADR-0068** —
   walk Locator não avança para Labelled (snapshot recovery
   evita `locator.next()`); Layouter Locator também não
   avança (is_locatable=false). Sequência sincronizada.

L0 `rules/introspect.md` actualizado com secção
"Walk arm Labelled migrado (P195D, ADR-0069)" + helper
documentado.

4 tests E2E em `mod p195d_walk_labelled`:
- Activação Introspector path para explicit labels.
- Paridade observable legacy vs Introspector.
- Figure target popula `figure_label_numbers`.
- Target não-resolvível: Tag não emitida; sub-store vazio.

---

## Confirmação `.F` (16/16)

| # | Verificação | Estado |
|---|-------------|--------|
| 1 | `cargo check --workspace` passa | ✅ |
| 2 | `cargo test --workspace` passa (Δ +4 vs 1834) | ✅ 1838 verdes |
| 3 | `crystalline-lint .` zero violations | ✅ |
| 4 | Tests `p195d_walk_labelled` passam | ✅ 4/4 |
| 5 | Tests sentinela E4 P189B não regridem | ✅ mutação legacy preservada |
| 6 | Walk arm Labelled emite Tag pós-recursão | ✅ `introspect.rs:Content::Labelled` |
| 7 | Helper `compute_labelled` invocado | ✅ `introspect.rs:` |
| 8 | Mutação legacy preservada | ✅ `state.resolved_labels.insert` + `state.figure_label_numbers.insert` continuam |
| 9 | `from_tags` arm Labelled (P195C) processa Tags | ✅ verified by sub-store population |
| 10 | `is_locatable(Content::Labelled)` ainda `false` | ✅ `locatable.rs` arm intocado |
| 11 | `extract_payload(Content::Labelled)` retorna `None` | ✅ catch-all intocado |
| 12 | Trait `Introspector` NÃO modificado | ✅ |
| 13 | `TagIntrospector` NÃO modificado | ✅ |
| 14 | Consumer C4 NÃO modificado | ✅ |
| 15 | Snapshot tests verdes | ✅ |
| 16 | Linter passa final | ✅ |

---

## Δ tests vs baseline

- Baseline P195C: **1.834** verdes.
- Após P195D: **1.838** verdes.
- Δ: **+4** (range planeado +3 a +4).

4 tests novos:
- `labelled_walk_emite_tag_e_popula_introspector`
- `labelled_paridade_observable_legacy_vs_introspector`
- `labelled_figure_target_popula_figure_label_numbers`
- `labelled_target_nao_resolvivel_nao_popula_introspector`

---

## Hashes finais

L0 modificado: `rules/introspect.md`

- Hash código (no L0): `e7f49e39`
- Hash prompt (`@prompt-hash` no `.rs`): `fef4c6d8`

`crystalline-lint --fix-hashes .` aplicado uma vez.
Análise final ✅ 0 drift warnings remaining.

---

## Decisões de execução notáveis

### Decisão `.A.3` — Locator: Opção (a) reuso de Location

Análise revelou bloqueador potencial:
- Se walk arm Labelled chamasse `locator.next()` para
  Labelled tag, walk Locator avançaria 1 a mais que
  Layouter (que mantém `is_locatable=false` para Labelled).
- ADR-0068 sincronização-por-construção quebrada para
  todos os locatables APÓS uma Labelled.
- C1 (heading prefix) e C2 (equation counter) Layouter
  consultam `intr.flat_counter_at(key, current_location)` —
  desync produziria None inesperado.

**Solução implementada**: snapshot `tags.len()` antes da
recursão; após recursão, `find_map` para primeira
`Tag::Start` no range novo extrai a Location do target;
P195D reusa essa Location.

```rust
let tags_len_before = tags.len();
walk(target, ...);
let target_loc = tags[tags_len_before..]
    .iter()
    .find_map(|t| if let Tag::Start(l, _) = t { Some(*l) } else { None });
```

Implicação: walk Locator não avança para Labelled; Layouter
Locator não avança para Labelled; sequências sincronizadas.

Caso edge: target não-locatable (find_map retorna None);
Tag não emitida; sub-store via Tag não populated; mutação
legacy preservada cobre.

### Helper `compute_labelled` materializado

Função privada (sem `pub`) em `introspect.rs:323-368`.
Replica match sobre target type (Heading/Equation/Figure/_)
sem mutação. Retorna `(Option<String>, Option<usize>)`.

Vantagens:
- Reuso entre mutação legacy e populate Tag (sem
  duplicação literal).
- Testável isoladamente.
- Replica literal da lógica legacy (preserva semântica).

### Mutação legacy preservada (write paralelo)

Per pattern P181 Bibliography (P181D-P181H):
- Walk arm muta legacy E também emite Tag.
- Consumers M4-fallback (C4 P194B) lêem Introspector
  primeiro com `or_else` legacy.
- Output observable inalterado em produção.
- Cleanup em M6 quando legacy for removido.

### Sem cláusula gate substancial disparada

- Locator sync preservado via reuso (Opção a).
- Tests sentinela E4 P189B passam (mutação legacy
  preservada).
- Helper `compute_labelled` replica lógica sem desviar
  semântica.
- Tags Labelled emitidas em ordem correcta (após Tags
  do target).

### Sem cláusula gate trivial significativa

`ElementPayload` adicionado a imports (`element_payload::ElementPayload`).
Forma exacta da expressão chegou intacta da spec.

---

## Estado actual

- **P195 série**: A ✅ B ✅ C ✅ D ✅ | E pendente.
- **Walk arm Labelled**: emite Tag pós-recursão; mutação
  legacy preservada paralela.
- **Helper `compute_labelled`**: introduzido em
  `introspect.rs`.
- **`ElementPayload`**: 11 variants (inalterado vs P195B).
- **Tests workspace**: 1.834 → **1.838** (+4).
- **ADR-0069**: PROPOSTO ainda — transita ACEITE em P195E
  após confirmação consolidada.
- **68 passos executados** (P195C = 67 + P195D = 68).
- **DEBT M5-residual**: 2 pré-requisitos pendentes
  (inalterado).
- **E4 fecha estruturalmente** — caminho Introspector
  activa para explicit labels. **Funcionalmente fecha em
  M6** quando mutação legacy for removida.

---

## Pendências cumulativas

- E1, E2, E3, E5, E6 continuam activas.
- E4 estruturalmente fechada (P195D); funcionalmente em M6.
- 2 pré-requisitos M5-residual restantes
  (`headings_for_toc`, `SetEquationNumbering`).
- ADR-0069 PROPOSTO; transita ACEITE em P195E.

---

## Próximo passo

**P195E** — encerramento da série P195:

1. Tests E2E adicionais (se necessário) confirmando
   paridade observable em pipeline real.
2. Transição ADR-0069 PROPOSTO → ACEITE (validação
   empírica de P195D habilita).
3. Relatório consolidado P195 (9 secções padrão) com:
   - Tabela sub-passos A-E.
   - 7 cláusulas P195A fechadas + decisão Locator.
   - Achados não-triviais (P195A §11.1-§11.6 + P195D
     decisão Locator).
   - Estado dormente vs activo: explicit labels via
     Introspector (P195D); auto-toc continua legacy
     (E2 → P196).
   - Próximo passo P196 (Heading walk arm migration —
     auto-toc).

Magnitude: S puro. Sem cláusulas condicionais.
