# Relatório P197B — Refactor walk arm Figure (cenário α)

**Data**: 2026-05-04
**Estado**: ✅ Completo (7 sub-passos A-G)
**Magnitude**: S/M (refactor estilístico — não migração arquitectural)
**Pattern arquitectural**: ADR-0069 stylesheet (helper privado); Tag pós-recursão **dispensada** per cenário α.

---

## §1 Sumário executivo

P197B materializa o **3º helper privado** da família ADR-0069
(`compute_figure`, análogo a `compute_labelled` P195D e
`compute_heading_auto_toc` P196B), refactorando o walk arm
`Content::Figure` para invocá-lo. **Caminho Introspector para
figure numbering já estava activo desde P184** — refactor é
estilístico, não arquitectural.

**E3 fecha estruturalmente** via cenário α (declaração formal
em L0). Diferente de E2 que ficou com residuo (lacuna #3) e de
E4 que fechou via Tag pós-recursão P195D.

**Output observable em produção**: inalterado.

## §2 Mutações migradas e preservadas

| # | Mutação | Estado pós-P197B |
|---|---------|------------------|
| 1 | `state.local_figure_counters.entry(kind).or_insert(0); *counter += 1;` | **Preservada** (write paralelo M5) — walk-internal trivial; sem consumer downstream. |
| 2 | `state.figure_numbers.entry(kind).or_default().push(n);` | **Preservada** (write paralelo M5) — `compute_labelled` P195D Figure arm depende. |

**Cleanup orgânico em M6** quando `compute_labelled` Figure
arm migrar para CounterRegistry (`flat_counter_at` ou
similar).

## §3 Helper privado novo

```rust
fn compute_figure(
    state:      &CounterStateLegacy,
    kind:       &Option<String>,
    is_counted: bool,
) -> Option<usize>
```

- 3º helper na família ADR-0069.
- Pura sobre `(state, kind, is_counted)`.
- `None` quando `is_counted = false` (figura sem caption ou
  numbering — não consome número, paridade com gate legacy).
- `Some(n+1)` 1-based quando `is_counted = true`, baseado em
  `state.local_figure_counters[kind]`.

## §4 Walk arm Figure (after P197B)

```rust
Content::Figure { body, caption, kind, numbering } => {
    let is_counted = numbering.is_some() && caption.is_some();
    if let Some(figure_number) = compute_figure(state, kind, is_counted) {
        let kind_key = kind.as_deref().unwrap_or("image").to_string();
        *state.local_figure_counters
            .entry(kind_key.clone())
            .or_insert(0) += 1;
        state.figure_numbers
            .entry(kind_key)
            .or_default()
            .push(figure_number);
    }
    walk(body, state, locator, tags, None);
    if let Some(cap) = caption {
        walk(cap, state, locator, tags, None);
    }
}
```

**Diferença vs P195D/P196B**: nenhuma Tag pós-recursão. Walk
top já emite `Tag::Start(loc, ElementInfo { payload: Figure
{...}, label: ... })` antes de entrar na arm via locatable +
`extract_payload`. Pattern ADR-0069 dispensado.

## §5 Tests sentinela cenário α (5 testes novos)

| # | Test | Cobre |
|---|------|-------|
| 1 | `figure_walk_caminho_introspector_ja_activo` | Caminho Introspector P184 → C3 P184D `figure_number_at_index` retorna `Some(1)`. Independente de P197B. |
| 2 | `figure_walk_helper_compute_figure_invocado` | Black-box: 2 figures image numeradas → `state.figure_numbers["image"] = [1, 2]`. Helper preserva semântica legacy. |
| 3 | `figure_paridade_legacy_vs_introspector_inalterada` | `state.figure_numbers.last() == intr.figure_number_at_index(...)` paridade pós-refactor. |
| 4 | `figure_numbering_inactivo_helper_retorna_none` | `is_counted = false` → helper retorna None → sem push em state. (Divergência conhecida from_tags arm Figure sem gate `is_counted` documentada como ortogonal.) |
| 5 | `figure_compute_labelled_p195d_continua_funcional` | Cadeia E2-E3 preservada: `compute_labelled` Figure arm produz `figure_label_numbers` populado em legacy + Introspector. |

## §6 L0 actualizado

`00_nucleo/prompts/rules/introspect.md` (hash novo `b9f78ff9`):

- Tabela "Excepções M5": linha **E3** → "**Fechou
  estruturalmente em P197B (cenário α — caminho Introspector
  activo desde P184)**".
- Lista "Ordem inversa à mutação": passo 6 marcado ✅
  (P197B); estado P197B 2026-05-04 substituiu P196B 2026-05-03.
- Nova secção **"Walk arm Figure migrado (P197B, cenário α)"**
  — análoga a "Walk arm Heading migrado (P196B, ADR-0069)"
  e "Walk arm Labelled migrado (P195D, ADR-0069)".
- Cross-references explícitas: P184B, P184C, P184D, P168,
  P195D, ADR-0069.

## §7 Verificações finais (.F — 18 checks)

| # | Verificação | Resultado |
|---|-------------|-----------|
| 1 | `cargo check --workspace` | ✅ ok |
| 2 | `cargo test --workspace` | ✅ 1848 verdes (Δ vs baseline 1843: **+5**) |
| 3 | `crystalline-lint .` | ✅ 0 violations |
| 4 | Tests P197B passam isoladamente | ✅ 5/5 verde |
| 5 | Tests existentes não regridem | ✅ (1843 inalterado; sentinela E3 P189B layout/tests.rs:4895 ok) |
| 6 | Walk arm Figure usa helper `compute_figure` | ✅ linha 542 |
| 7 | Mutação legacy preservada | ✅ linhas 543-552 |
| 8 | Comentário inline P197B presente | ✅ linhas 524-540 |
| 9 | L0 secção "Walk arm Figure migrado (P197B, cenário α)" | ✅ presente |
| 10 | Tabela Excepções M5 com E3 fechada estruturalmente | ✅ |
| 11 | `compute_labelled` P195D Figure arm NÃO modificado | ✅ |
| 12 | Variant `ElementPayload::Figure` NÃO modificado | ✅ |
| 13 | `from_tags` arm Figure NÃO modificado | ✅ |
| 14 | Trait `Introspector` NÃO modificado | ✅ |
| 15 | Consumer C3 P184D NÃO modificado | ✅ |
| 16 | Consumer C4 P194B NÃO modificado | ✅ |
| 17 | Snapshot tests verdes | ✅ |
| 18 | Linter passa final | ✅ |

**18/18 verde.**

## §8 Decisões de execução notáveis

### Test 4 ajustado pós-execução

Test inicial assertava `intr.figure_number_at_index("image", 0) == None` para figura sem caption. Mas `from_tags` arm Figure (P184B) incrementa CounterRegistry **independente de `is_counted`** — comportamento pre-existente, não introduzido por P197B. Ajustei o test para validar apenas os efeitos directos do helper sobre state legacy (`state.figure_numbers` e `state.local_figure_counters`), com nota explicando a divergência conhecida (m1-lacunas-captura.md #1) como ortogonal ao refactor.

### EcoString vs String

Test inicial usou `Some(EcoString::from("1"))` para `Content::Figure.numbering`. Variant é `Option<String>` (não `Option<EcoString>`) — corrigi via sed para `Some("1".to_string())`.

## §9 Estado actual

- **P197 série**: A ✅ B ✅ | C pendente.
- **E3 fechada estruturalmente** via cenário α — diferente de E2 que ficou residuo.
- **Hashes**: L0 `b9f78ff9` ↔ código `c938c001`.
- **73 passos executados** (pré-P196A 70 + P196A 71 + P196B 72 + P196C 73 + P197A 74 + P197B 75 — corrigir contagem em P197C consolidado).

## §10 Pendências cumulativas

**Excepções activas pós-P197B**: 3 + 1 residuo:
- E1 (Equation) — independente; pré-requisito `Content::SetEquationNumbering`.
- E2-residuo (Heading `headings_for_toc`) — pré-requisito sub-store `intr.headings_for_toc`.
- E5 (SetHeadingNumbering).
- E6 (CounterUpdate).

**2 pré-requisitos restantes** (inalterado vs P196).

## §11 Próximo passo

**P197C** — relatório consolidado P197 (9 secções padrão) +
actualização nota DEBT M5-residual (3 excepções activas + 1
residuo). Magnitude S puro.

## §12 Linhagem

- **Pattern arquitectural stylesheet**: ADR-0069 (helper privado).
- **Tag pós-recursão dispensada**: cenário α (P197A diagnóstico §5).
- **Helper análogo**: `compute_labelled` (P195D) ↔ `compute_heading_auto_toc` (P196B) ↔ **`compute_figure` (P197B)** — 3º helper na família.
- **Sub-store consumido**: `intr.counters` (CounterRegistry P184B) via `figure_number_at_index` (P184C); `intr.figure_label_numbers` (P168 + P195D combinados); `intr.kind_index[Figure]`.
- **Consumer C3**: `references.rs::layout_ref` figure ref-arm (P184D substitution-with-fallback) — inalterado em P197B.
- **L0 tocado**: `00_nucleo/prompts/rules/introspect.md` hash `b9f78ff9`.
- **Código tocado**: `01_core/src/rules/introspect.rs` hash `c938c001`.
- **Padrão diagnóstico-primeiro**: 19ª aplicação consecutiva (P197A diagnóstico).
- **3ª aplicação família ADR-0069**: P195D (Labelled, snapshot+find_map) + P196B (Heading, emitted_loc directo) + **P197B (Figure, dispensa Tag)** — variantes operacionais consolidadas para P198 decidir empiricamente.
