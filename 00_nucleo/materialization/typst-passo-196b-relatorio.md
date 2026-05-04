# Relatório P196B — Walk arm Heading auto-toc via Tag pattern (ADR-0069)

**Data**: 2026-05-03
**Estado**: ✅ Completo (4/4 sub-passos D-G executados após .A-.C da sessão prévia)
**Pattern arquitetural**: ADR-0069 post-recursion tag emission (segunda aplicação)

## §1 Sumário executivo

P196B materializa a **segunda aplicação** do pattern ADR-0069
(post-recursion tag emission) — aplicada ao walk arm
`Content::Heading` para resolver E2 (excepção M5 de 4 mutações).

**Resultado estrutural**:
- 3 das 4 mutações originais de E2 fecham via Tag::Labelled
  auto-toc emitida pós-recursão.
- 1 mutação residual (`headings_for_toc.push`) persiste como
  **E2-residuo**, bloqueada por lacuna #3 (sub-store
  `intr.headings_for_toc` ausente).
- Mutação legacy preservada como write paralelo durante janela
  compat M5 — fecha orgânicamente em M6.

**Resultado funcional**: output observable em produção
inalterado. Caminho Introspector (consumer C4 P194B) recebe
`Some(text)` directamente, sem necessidade de fallback legacy.

## §2 Mutações migradas (3 de 4)

| # | Mutação legacy | Origem da paridade pós-P196B |
|---|----------------|------------------------------|
| 1 | `state.step_hierarchical("heading", level)` | Walk continua a mutar (write paralelo); paridade futura via Tag::StateUpdate quando E5 fechar |
| 2 | `state.auto_label_counter += 1` | Walk continua a mutar (write paralelo); auto_label sintetizada também na arm via helper |
| 3 | `state.resolved_labels.insert(auto_label, resolved_text)` | **Migrado**: Tag::Labelled auto-toc populates `intr.resolved_labels` via from_tags |

**Mutação 4 (E2-residuo)**: `state.headings_for_toc.push((auto_label, frozen_body, level))` — sem destino estrutural ainda. Mantida activa.

## §3 Helper privado novo

```rust
fn compute_heading_auto_toc(
    state:        &CounterStateLegacy,
    auto_label_n: usize,
) -> (Label, String)
```

- Análogo a `compute_labelled` (P195D).
- Pura sobre `&CounterStateLegacy`.
- Sempre retorna concrete `(Label, String)` em vez de
  `Option<…>` — paridade legacy preserva insert de
  `auto_label → ""` quando numbering inactivo.

## §4 Walk arm Heading (after P196B)

Sequência:
1. Mutação legacy `step_hierarchical` + `auto_label_counter++`
   (preservada).
2. `compute_heading_auto_toc(state, n)` → `(label, text)`.
3. `state.resolved_labels.insert(label, text)` (write paralelo M5).
4. `materialize_time(body, state)` + `headings_for_toc.push`
   (E2-residuo mantida).
5. `walk(body, …)` recursivo.
6. **`if let Some(loc) = emitted_loc`** → emit
   `Tag::Start(loc, Labelled{label, resolved_text:Some(text), figure_number:None})`
   + `Tag::End(loc, 0)`.

**Diferença vs P195D**: Heading é locatable, então `emitted_loc`
do walk top já está disponível directamente — sem snapshot+find_map.

## §5 Sequência de tags emitida

Para `heading(1, text("título"))` (text não-locatable):

```
Tag::Start(loc,    Heading)               // walk top
Tag::Start(loc,    Labelled auto-toc-1)   // arm pós-recursão
Tag::End(loc, 0)                          // arm pós-recursão (hash=0)
Tag::End(loc, hash_content(heading))      // walk bottom
```

4 tags com mesma Location, 2 pares Start/End, bracketing válido.

Para `heading(1, figure)`:

```
Tag::Start(loc_h, Heading)
Tag::Start(loc_f, Figure)
Tag::End(loc_f, hash_figure)
Tag::Start(loc_h, Labelled auto-toc-1)
Tag::End(loc_h, 0)
Tag::End(loc_h, hash_heading)
```

6 tags. Bracketing preserva (Heading bracket envolve Figure
bracket; auto-toc inserido entre fim de recursão e End externo).

## §6 Tests actualizados (5)

Tests existentes que assumiam 2 tags por Heading foram
actualizados para reflectir 4 tags pós-P196B:

| Test | Antes | Depois |
|------|-------|--------|
| `walk_emite_start_e_end_para_heading` | 2 tags | 4 tags (Start, Start, End, End mesma Location) |
| `walk_aninha_start_end_para_heading_contendo_figure` | 4 tags | 6 tags |
| `walk_emite_tags_em_paralelo_com_state` | 6 tags | 10 tags (1 Set × 2 + 2 Heading × 4) |
| `bracketing_valido_em_sequencia_plana` | 6 tags | 12 tags (3 Heading × 4) |
| `end_hash_distingue_conteudo` | first End | filter `hash != 0` (hash=0 do auto-toc Tag::End) |

## §7 Tests E2E novos (5)

Adicionados ao final de `mod tests` em
`01_core/src/rules/introspect.rs`:

1. `heading_auto_toc_walk_emite_tag_e_popula_introspector` —
   valida que Introspector.resolved_label_for(`auto-toc-1`) retorna
   `Some("Secção 1")` via Tag::Labelled.
2. `heading_auto_toc_paridade_legacy_vs_introspector` — paridade
   compat M5: legacy state e Introspector têm mesmo valor para
   auto-toc-1, auto-toc-2.
3. `heading_auto_toc_numbering_inactivo_emite_string_vazia` —
   numbering inactivo → resolved_text = "" em ambos os paths
   (paridade legacy preservada).
4. `walk_e2_residuo_headings_for_toc_via_legacy` — confirma que
   E2-residuo (`headings_for_toc.push`) continua activo via mutação
   legacy (3 entries para 3 headings).
5. `consumer_c4_recebe_some_para_auto_toc_label` — primeira branch
   substitution-with-fallback (`intr.resolved_label_for`) retorna
   `Some` sem fallback legacy necessário.

## §8 L0 actualizado

`00_nucleo/prompts/rules/introspect.md` (hash novo `3bc33823`):

- Tabela Excepções M5: linha E2 → **E2-residuo** com 1 mutação;
  linha E4 marcada como "Fechou estruturalmente em P195D".
- Lista "Ordem inversa à mutação": passos 1-4 marcados ✅
  (P193B, P194B, P195D, P196B); passo 5 novo (sub-store
  `intr.headings_for_toc` para fechar E2-residuo).
- Nova secção **"Walk arm Heading migrado (P196B, ADR-0069)"** —
  análoga a "Walk arm Labelled migrado (P195D, ADR-0069)".

## §9 Verificações finais

| # | Verificação | Resultado |
|---|-------------|-----------|
| 1 | `cargo build --workspace` | ✅ ok (warnings pré-existentes não relacionados) |
| 2 | `cargo test --workspace` | ✅ 1843 verdes (1838 + 5 novos P196B) |
| 3 | `cargo test rules::introspect::tests` | ✅ 56 verdes (51 + 5 novos) |
| 4 | `crystalline-lint .` | ✅ 0 violations |
| 5 | Hash L0 atualizado | ✅ via `crystalline-lint --fix-hashes`: `73489ae5` em introspect.rs ↔ `3bc33823` em L0 |
| 6 | Walk arm Heading agora chama helper privado | ✅ `compute_heading_auto_toc` invocado linha 441 |
| 7 | Walk arm Heading emite Tag::Labelled pós-recursão | ✅ linhas 461-471 |
| 8 | E2 fechou estruturalmente (3 de 4 mutações) | ✅ documentado em L0 §"Excepções M5" |
| 9 | E2-residuo declarado e justificado | ✅ tabela Excepções + comentário inline na arm |
| 10 | Mutação legacy preservada (write paralelo M5) | ✅ linhas 438-452 |
| 11 | Tag::End com hash=0 para auto-toc | ✅ linha 470 (`Tag::End(loc, 0)`) |
| 12 | Reuso da `emitted_loc` (Heading locatable) | ✅ `if let Some(loc) = emitted_loc` |
| 13 | `compute_heading_auto_toc` puro (sem mutação) | ✅ assinatura `&CounterStateLegacy` |
| 14 | Helper sempre retorna concrete `(Label, String)` | ✅ paridade legacy preservada |
| 15 | Numbering inactivo → resolved_text vazio | ✅ test `heading_auto_toc_numbering_inactivo_emite_string_vazia` |
| 16 | E2-residuo preserva push em headings_for_toc | ✅ test `walk_e2_residuo_headings_for_toc_via_legacy` |
| 17 | Consumer C4 recebe Some via Introspector path | ✅ test `consumer_c4_recebe_some_para_auto_toc_label` |
| 18 | Bracketing válido com 4 tags por Heading | ✅ test `bracketing_valido_em_sequencia_plana` (12 tags em 3 headings) |

**18/18 verde.**

## §10 Estado pós-P196B

### M5 walk-puro: progressão

| Excepção | Estado pré-P196B | Estado pós-P196B |
|----------|------------------|-------------------|
| E1 (Equation) | activa | activa (independente; aguarda `Content::SetEquationNumbering`) |
| E2 (Heading) | activa, 4 mutações | **E2-residuo**, 1 mutação (push em `headings_for_toc`) |
| E3 (Figure) | activa | activa (cadeia com E2-residuo) |
| E4 (Labelled) | fechou em P195D | fechou (estruturalmente; legacy mutation M5 → M6) |
| E5 (SetHeadingNumbering) | activa | activa (cadeia com E2/E3) |
| E6 (CounterUpdate) | activa | activa (cadeia com E2/E3) |

### Próximos passos

- **P196C** (próximo): relatório consolidado série P196 (.A diagnostic + .B materialization).
- **Passo dedicado** (futuro, fora série P196): abrir sub-store `intr.headings_for_toc` para fechar E2-residuo. Lacuna #3 precisa de design (decisão sobre estrutura do sub-store: `Vec<(Label, Content, usize)>` ou variant em Tag).
- **M6** (futuro distante): remover write paralelo legacy (`state.step_hierarchical` + `auto_label_counter` + `resolved_labels.insert`) quando todos consumers migrarem para Introspector path.

## §11 Linhagem

- Pattern ADR-0069 (post-recursion tag emission) — segunda aplicação.
- Diagnóstico P196A (`00_nucleo/diagnosticos/diagnostico-walk-heading-passo-196a.md`).
- Helper `compute_heading_auto_toc` análogo a `compute_labelled` (P195D).
- Sub-store `intr.resolved_labels` (P193B) consumido via Tag::Labelled.
- Consumer C4 substitution-with-fallback (P194B) recebe Some pós-P196B.
- L0 `00_nucleo/prompts/rules/introspect.md` hash `3bc33823`.
- Código `01_core/src/rules/introspect.rs` hash `73489ae5`.
