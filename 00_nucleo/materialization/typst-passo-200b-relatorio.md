# Relatório P200B — Sub-store + Tag + consumer (M5 universal completo)

**Data**: 2026-05-04
**Estado**: ✅ Completo (14 sub-passos A-N)
**Magnitude**: M+ genuína — trabalho híbrido (3 categorias combinadas).
**Pattern arquitectural**: ADR-0069 stylesheet — 7ª aplicação concreta.
**Marco arquitectural**: **M5 universal completo pela primeira vez desde declaração em P189B**.

---

## §1 Sumário executivo

P200B fecha **E2-residuo** + **lacuna #3** simultaneamente
via **trabalho híbrido** combinando 3 padrões testados —
sem nova variante operacional ADR-0069:

| Categoria | Padrão precedente | Trabalho P200B |
|-----------|-------------------|-----------------|
| **A — Sub-store novo** | P193B (`ResolvedLabelStore`) | `headings_for_toc: Vec<(Label, Content, usize)>` (8º → 9º) |
| **B — Variant Tag pós-recursão** | Variante P196B (Heading auto-toc) | `ElementPayload::HeadingForToc` (12ª → 13ª variant) |
| **C — Consumer migration** | P184D / P194B | `outline.rs:24` substitution-with-fallback |

**Marco arquitectural — M5 universal completo**:
- 0 excepções activas.
- 0 residuos.
- 0 pré-requisitos restantes.

Desbloqueia M6 (P190A reescrita do zero — eliminação `CounterStateLegacy`; magnitude L cross-modular).

**Output observable em produção**: inalterado — substitution-with-fallback no consumer outline garante paridade; sub-store fornece dados; legacy fica funcional como backup.

---

## §2 Trabalho concreto

| # | Ficheiro | Mudança |
|---|----------|---------|
| 1 | `entities/introspector.rs` | Field novo `headings_for_toc: Vec<(Label, Content, usize)>` em `TagIntrospector`; trait method `headings_for_toc(&self)` (19→20); impl trait. |
| 2 | `entities/element_payload.rs` | Variant nova `HeadingForToc { label, body, level }` (12→13). |
| 3 | `rules/introspect.rs` | Helper privado `compute_heading_for_toc` (4º na família ADR-0069); walk arm Heading modificado para emitir 3ª Tag pós-recursão; comentário inline P200B substitui notas E2-residuo. |
| 4 | `rules/introspect/from_tags.rs` | Arm `ElementPayload::HeadingForToc` push directo no sub-store. |
| 5 | `rules/layout/outline.rs` | Consumer migrado para substitution-with-fallback. |
| 6 | `prompts/rules/introspect.md` | Tabela Excepções E2-residuo fechada; ordem inversa passo 5 ✅; secção nova "Walk arm Heading mutação 4 fechada (P200B, trabalho híbrido)"; **marco "M5 universal completo"** documentado. |

**ElementKind::HeadingForToc NÃO adicionada** — HeadingForToc é Tag derivada de Heading (não Content standalone); justificação inline em L0.

**Layouter assignments (`mod.rs:1490, 1521`) NÃO modificados** — write paralelo M5; cleanup orgânico em M6.

**`compute_heading_auto_toc` (P196B) NÃO modificado** — sub-stores diferentes (`resolved_labels` vs `headings_for_toc`).

---

## §3 Helper privado novo `compute_heading_for_toc`

```rust
fn compute_heading_for_toc(
    state:       &CounterStateLegacy,
    frozen_body: Content,
    level:       usize,
) -> Option<(Label, Content, usize)> {
    let auto_label = Label(format!("auto-toc-{}", state.auto_label_counter));
    Some((auto_label, frozen_body, level))
}
```

- 4º helper na família ADR-0069 stylesheet (após `compute_labelled` P195D, `compute_heading_auto_toc` P196B, `compute_figure` P197B).
- Sempre retorna `Some` — paridade com mutação 4 legacy (push incondicional pre-P200B).
- Reusa `frozen_body` já computed pelo walk arm — evita chamada redundante a `materialize_time`.

---

## §4 Walk arm Heading (após P200B)

```rust
Content::Heading { level, body } => {
    // P200B (M5 universal completo) — walk arm Heading
    // E2-residuo fechada estruturalmente. Trabalho híbrido...
    state.step_hierarchical("heading", *level as usize);

    state.auto_label_counter += 1;
    let (auto_label, resolved_text) = compute_heading_auto_toc(state, state.auto_label_counter);
    state.resolved_labels.insert(auto_label.clone(), resolved_text.clone());

    // Mutação 4 legacy preservada (write paralelo M5).
    let frozen_body = materialize_time(body, state);
    state.headings_for_toc.push((auto_label.clone(), frozen_body.clone(), *level as usize));

    walk(body, state, locator, tags, None);

    // P196B: Tag::Labelled auto-toc pós-recursão.
    if let Some(loc) = emitted_loc { /* Labelled tag */ }

    // P200B: Tag::HeadingForToc pós-recursão (3ª Tag, mesma loc).
    if let Some(loc) = emitted_loc {
        if let Some((label, body_for_toc, lvl)) = compute_heading_for_toc(
            state, frozen_body, *level as usize,
        ) {
            tags.push(Tag::Start(loc, ElementInfo::new(ElementPayload::HeadingForToc {
                label, body: body_for_toc, level: lvl,
            })));
            tags.push(Tag::End(loc, 0));
        }
    }
}
```

**6 tags por Heading folha** (era 4 pós-P196B). Bracketing válido — todas mesma Location.

---

## §5 Tests novos (5) + tests adaptados (4)

### Tests novos P200B

| # | Test | Cobre |
|---|------|-------|
| 1 | `headings_for_toc_walk_emite_tag_e_popula_sub_store` | Sub-store populated com 1 entry para 1 Heading numerado. |
| 2 | `headings_for_toc_paridade_legacy_vs_introspector` | Paridade exacta (3 entries; labels + levels idênticos) entre legacy state e Introspector. |
| 3 | `bracketing_valido_6_tags_por_heading_p200b` | 6 tags por Heading folha; bracketing por Location válido. |
| 4 | `e2_residuo_fechada_paridade_legacy_introspector` | Sentinela substituindo `walk_e2_residuo_headings_for_toc_via_legacy` P196B; ambos paths populated em paralelo. |
| 5 | `headings_for_toc_helper_compute_produces_correct_entry` | Helper `compute_heading_for_toc` produz tuple correcto (auto-label + body materializado + level cast). |

### Tests P196B adaptados

| Test | Antes (P196B) | Depois (P200B) |
|------|---------------|----------------|
| `walk_emite_start_e_end_para_heading` | 4 tags | 6 tags |
| `walk_aninha_start_end_para_heading_contendo_figure` | 6 tags | 8 tags |
| `walk_emite_tags_em_paralelo_com_state` | 10 tags | 14 tags |
| `bracketing_valido_em_sequencia_plana` | 12 tags | 18 tags |

**Apenas 4 tests P196B regridem** (não 5 como projectado em P200A) — `end_hash_distingue_conteudo` já filtra `hash != 0` e mantém-se válido.

---

## §6 Verificações finais (.M — 24 checks)

| # | Verificação | Resultado |
|---|-------------|-----------|
| 1 | `cargo check --workspace` | ✅ ok |
| 2 | `cargo test --workspace` | ✅ 1869 verdes (Δ vs baseline 1864: **+5**) |
| 3 | `crystalline-lint .` | ✅ 0 violations |
| 4 | Tests P200B passam isoladamente | ✅ 5/5 verde |
| 5 | Tests P196B adaptados passam | ✅ 4/4 verde |
| 6 | Tests existentes não regridem | ✅ |
| 7 | Field `headings_for_toc` presente | ✅ |
| 8 | Trait method `headings_for_toc()` presente | ✅ (20º método) |
| 9 | Variant `ElementPayload::HeadingForToc` presente | ✅ (13ª variant) |
| 10 | Decisão `ElementKind::HeadingForToc` materializada | ✅ NÃO criada (justificação documentada) |
| 11 | Helper `compute_heading_for_toc` presente | ✅ (4º na família ADR-0069) |
| 12 | Walk arm Heading emite 3 Tags pós-recursão | ✅ |
| 13 | Mutação 4 legacy preservada | ✅ |
| 14 | `from_tags` arm `HeadingForToc` funcional | ✅ |
| 15 | Consumer outline.rs:24 migrado | ✅ |
| 16 | Comentários inline P200B presentes | ✅ |
| 17 | L0 `introspect.md` actualizado | ✅ |
| 18 | Tabela Excepções M5 com E2-residuo fechada | ✅ |
| 19 | Marco "M5 universal completo" registado em L0 | ✅ |
| 20 | Layouter assignments (`mod.rs:1490, 1521`) NÃO modificados | ✅ |
| 21 | `compute_heading_auto_toc` (P196B) NÃO modificado | ✅ |
| 22 | Walk arms outros NÃO modificados | ✅ |
| 23 | Snapshot tests verdes | ✅ |
| 24 | Linter passa final | ✅ |

**24/24 verde.**

---

## §7 Decisões de execução notáveis

### Apenas 4 tests P196B regridem (não 5 projectado)

P200A diagnostic §7 projectou 5 tests adaptados. Empiricamente apenas 4 regridem (`walk_emite_start_e_end_para_heading`, `walk_aninha_start_end_para_heading_contendo_figure`, `walk_emite_tags_em_paralelo_com_state`, `bracketing_valido_em_sequencia_plana`). `end_hash_distingue_conteudo` já filtra `hash != 0` e mantém-se válido com qualquer número de Tag::End.

### ElementKind::HeadingForToc NÃO criada

P200A diagnostic §1.7 e §2 cláusula 6 deferiram a decisão a empíricamente. Decisão final: NÃO criar — HeadingForToc é Tag **derivada** de Heading (não Content standalone parsável), análoga a Tag::Labelled auto-toc P196B (também sem ElementKind correspondente). Sub-store dedicado `headings_for_toc` é caminho de query directo via trait method; sem necessidade de `kind_index` paralelo.

### Helper sempre retorna `Some`

P200A diagnostic §2 cláusula 5 inicialmente sugeriu gate `if !state.is_numbering_active("heading") { return None; }`. Re-verificação empírica revelou que mutação 4 legacy é **incondicional** (introspect.rs:486 pre-P200B faz push sempre, mesmo sem numbering activo). Helper segue paridade — sempre retorna `Some(...)`. Test 3 do diagnostic §8 (`headings_for_toc_numbering_inactivo_nao_emite_tag`) substituído por test mais útil (`bracketing_valido_6_tags_por_heading_p200b`).

### `frozen_body` clonado para reuso

Walk arm pre-P200B fazia `state.headings_for_toc.push((auto_label, frozen_body, level))` movendo `frozen_body`. Para reusar em Tag::HeadingForToc emit, agora faz `.clone()` na mutação 4: `state.headings_for_toc.push((auto_label.clone(), frozen_body.clone(), *level as usize))`. Custo de clone aceitável (Content é Arc-internamente em variants compostos; clone é O(1) para vias importantes).

### Layouter assignments preservados

`mod.rs:1490, 1521` fazem `l.counter.headings_for_toc = initial_state.headings_for_toc`. Mover para Introspector path completo exige refactor do Layouter — domínio de M6 (P190A reescrita do zero). Mutação 4 legacy + assignments preservados como write paralelo M5.

---

## §8 Estado actual

- **P200 série**: A ✅ B ✅ | C pendente.
- **E2-residuo + lacuna #3 fechadas**.
- **M5 universal completo pela primeira vez desde P189B**.
- **Hashes**: L0 `7a3ba2b7` ↔ código `8e0128e4`.

---

## §9 Pendências cumulativas

**Excepções activas pós-P200B**: **0**.
**Residuos**: **0**.
**Pré-requisitos restantes**: **0**.

**Marco arquitectural — M5 universal completo**.

**DEBT M6**: write paralelo M5 ainda activo — mutações legacy em todos walk arms preservadas; `compute_*` helpers leem legacy; Layouter assignments dependem do legacy. Cleanup orgânico em **M6 (P190A reescrita do zero)**.

---

## §10 Próximo passo

**P200C** — encerramento série P200:
- Auditoria empírica final (15 verificações).
- Relatório consolidado `typst-passo-200-relatorio-consolidado.md` (9 secções padrão).
- Nota DEBT M5-residual actualizada (0 + 0 + 0).
- **Marco arquitectural — M5 universal completo**.
- Verificação estrutural final.

Magnitude **S puro**.

Após P200C: **M5 universal fechado**. Desbloqueia **M6 (P190A reescrita do zero — eliminação `CounterStateLegacy`; magnitude L cross-modular)**.

---

## §11 Linhagem

- **Pattern arquitectural stylesheet**: ADR-0069 (PROPOSTO P195B; ACEITE P195E).
- **5 variantes operacionais consolidadas** (inalterado em P200):
  - P195D (não-locatable + snapshot+find_map).
  - P196B (locatable + body + emitted_loc directo).
  - Cenário α (P197B Figure, P198B SetHeadingNumbering).
  - Cenário α por construção (P199B SetEquationNumbering).
  - Cenário β-promote (P198C CounterUpdate).
- **Trabalho híbrido P200B**: combinação directa de 3 padrões testados; **sem nova variante operacional**.
- **7 aplicações ADR-0069 stylesheet**: P195D + P196B + P197B + P198B + P198C + P199B + **P200B**.
- **Helpers privados família ADR-0069**: 4 (P195D `compute_labelled`; P196B `compute_heading_auto_toc`; P197B `compute_figure`; **P200B `compute_heading_for_toc`**).
- **TagIntrospector**: 8 → **9 sub-stores** (`headings_for_toc` adicionado).
- **Trait `Introspector`**: 19 → **20 métodos** (`headings_for_toc()` adicionado).
- **`ElementPayload`**: 12 → **13 variants** (`HeadingForToc` adicionada).
- **`ElementKind`**: 10 (inalterado em P200 — HeadingForToc é Tag derivada; sem ElementKind correspondente).
- **Sub-stores consumidos por sub-store novo**: nenhum (push directo em from_tags arm).
- **Consumer migrado**: `outline.rs:24` (3ª migration substitution-with-fallback após C3 P184D + C4 P194B).
- **Cadeia E2-residuo**: walk arm Heading mutação 4 → mut 4 preservada; Tag::HeadingForToc pós-recursão emite payload com body materializado para outline.
- **L0 tocado**: `00_nucleo/prompts/rules/introspect.md` hash `7a3ba2b7`.
- **Código tocado**: 5 ficheiros `01_core/src/`:
  - `entities/introspector.rs` (sub-store + trait method + impl).
  - `entities/element_payload.rs` (variant nova).
  - `rules/introspect.rs` (helper + walk arm; hash `8e0128e4`).
  - `rules/introspect/from_tags.rs` (arm novo).
  - `rules/layout/outline.rs` (consumer migration).
- **Padrão diagnóstico-primeiro**: 22ª aplicação consecutiva (P200A diagnóstico).
- **Marco arquitectural**: M5 universal completo pela primeira vez desde declaração em P189B.

---

## §12 Métricas finais P200B

- LOC produção: ~120 (sub-store + trait + variant + helper + walk arm modifications + from_tags arm + consumer migration).
- LOC teste: ~180 (5 tests novos + 4 adaptações P196B).
- LOC L0: ~110 (secção nova "Walk arm Heading mutação 4 fechada"; secção "Marco M5 universal completo"; tabela Excepções + ordem inversa actualizadas).
- +5 testes workspace (1864 → 1869).
- +1 sub-store TagIntrospector (8 → 9).
- +1 trait method (19 → 20).
- +1 ElementPayload variant (12 → 13).
- 0 ElementKind variants novas.
- 0 ADRs novas.
- 1 helper privado novo (4º na família ADR-0069 stylesheet).
- 7ª aplicação ADR-0069 stylesheet.
- **Excepções M5 fechadas**: 1 (E2-residuo). **Lacunas fechadas**: 1 (#3).
- **Marco arquitectural**: M5 universal completo.
