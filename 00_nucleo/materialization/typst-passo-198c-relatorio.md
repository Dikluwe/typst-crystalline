# Relatório P198C — Promote Content::CounterUpdate (cenário β-promote)

**Data**: 2026-05-04
**Estado**: ✅ Completo (11 sub-passos A-K)
**Magnitude**: M genuína — primeira aplicação cenário β-promote (variante operacional nova)
**Pattern arquitectural**: ADR-0069 stylesheet — 5ª aplicação concreta.

---

## §1 Sumário executivo

P198C materializa a **primeira aplicação do cenário β-promote**
— variante operacional nova consolidada em P198A. Distinção das
2 variantes prévias do pattern ADR-0069:

| Variante | Caminho Introspector pré-passo | Trabalho concreto |
|----------|-------------------------------|-------------------|
| **P195D** (não-locatable) | Inactivo | Tag pós-recursão + snapshot+find_map |
| **P196B** (locatable) | Inactivo | Tag pós-recursão + emitted_loc directo |
| **P197B / P198B** (cenário α) | Activo | Refactor estilístico / declaração formal |
| **P198C** (cenário β-promote) | **Inactivo** | **Promote completo: variant + locatable + 2 arms** |

**E6 fecha estruturalmente**. Resta apenas E1 (Equation, pré-requisito
`SetEquationNumbering`) + E2-residuo (lacuna #3
`headings_for_toc`).

**Output observable em produção**: inalterado — caminho Introspector
activado em paralelo ao legacy; ambos fornecem valores idênticos.

---

## §2 Trabalho concreto

| # | Ficheiro | Mudança |
|---|----------|---------|
| 1 | `entities/element_payload.rs` | Adicionada variant `CounterUpdate { key: String, action: CounterUpdate }` (12ª variant). |
| 2 | `entities/element_kind.rs` | Adicionada variant `ElementKind::CounterUpdate` (10ª); `as_str` + `from_name` actualizados. |
| 3 | `rules/introspect/locatable.rs` | `Content::CounterUpdate { .. }` movida para lista locatable (`true`). |
| 4 | `rules/introspect/extract_payload.rs` | Arm novo retorna `Some(ElementPayload::CounterUpdate { key, action })`. |
| 5 | `rules/introspect/from_tags.rs` | Arm novo: 3 caminhos (Step+heading → `apply_hierarchical_at`; Step+other → `apply_at(Step)`; Update → `apply_at(Update)`); indexa em `kind_index[CounterUpdate]`. |
| 6 | `rules/introspect.rs` (walk arm) | Comentário inline P198C; 3 mutações legacy preservadas. |
| 7 | `prompts/rules/introspect.md` | Tabela Excepções E6 fechada estruturalmente; secção nova "Walk arm CounterUpdate migrado (P198C, β-promote)"; ordem inversa passo 8 ✅. |

---

## §3 Variante nova `ElementPayload::CounterUpdate`

```rust
CounterUpdate {
    key:    String,
    action: CounterUpdate,  // enum from counter_update.rs (P161 rename)
}
```

Field `action` reusa enum existente — 2 variants:
- `CounterUpdate::Step`.
- `CounterUpdate::Update(usize)`.

Convenção: variant `ElementPayload::CounterUpdate` colide
nominalmente com enum `CounterUpdate` mas Rust namespacing
mantém-os distintos (`ElementPayload::CounterUpdate` vs
`crate::entities::counter_update::CounterUpdate`).

---

## §4 from_tags arm — 3 caminhos

```rust
ElementPayload::CounterUpdate { key, action } => {
    intr.kind_index.entry(ElementKind::CounterUpdate).or_default().push(*loc);
    match action {
        CounterUpdate::Step => {
            if key == "heading" {
                intr.counters.apply_hierarchical_at(key.clone(), 1, *loc);
            } else {
                intr.counters.apply_at(key.clone(), CounterUpdate::Step, *loc);
            }
        }
        CounterUpdate::Update(val) => {
            intr.counters.apply_at(key.clone(), CounterUpdate::Update(*val), *loc);
        }
    }
}
```

Paridade exacta com walk arm legacy (3 mutações):
- Walk: `state.step_hierarchical("heading", 1)` ↔ from_tags: `apply_hierarchical_at("heading", 1, loc)`.
- Walk: `state.step_flat(key)` ↔ from_tags: `apply_at(key, Step, loc)`.
- Walk: `state.update_flat(key, val)` ↔ from_tags: `apply_at(key, Update(val), loc)`.

---

## §5 Tests E2E (6 testes novos)

| # | Test | Cobre |
|---|------|-------|
| 1 | `counter_update_extract_payload_emite_payload` | `extract_payload(CounterUpdate)` retorna `Some(ElementPayload::CounterUpdate { key: "equation", action: Step })`. |
| 2 | `counter_update_is_locatable_true` | `is_locatable(CounterUpdate) = true` após promote. |
| 3 | `counter_update_walk_popula_counter_registry` | 2 Steps em "equation" → `flat_counter_at("equation", last_loc) == Some(2)`; `kind_index[CounterUpdate]` tem 2 locations. |
| 4 | `counter_update_paridade_legacy_vs_introspector` | 3 Steps → `state.get_flat("equation") == 3 == intr.flat_counter_at(...)`. |
| 5 | `counter_update_action_update_apply_correctly` | `Update(42)` → ambos paths retornam 42. |
| 6 | `counter_update_compute_helpers_continuam_funcionais` | Cadeia E6 ↔ E4: `compute_labelled` Equation arm produz "Equação (1)" via state.get_flat (mutação legacy preservada). |

---

## §6 Verificações finais (.J — 21 checks)

| # | Verificação | Resultado |
|---|-------------|-----------|
| 1 | `cargo check --workspace` | ✅ ok |
| 2 | `cargo test --workspace` | ✅ 1859 verdes (Δ vs baseline 1853: **+6**) |
| 3 | `crystalline-lint .` | ✅ 0 violations |
| 4 | Tests P198C passam isoladamente | ✅ 6/6 verde |
| 5 | Tests existentes não regridem | ✅ |
| 6 | Tests adaptados via padrão pragmático | N/A (sem regressões) |
| 7 | Variant `ElementPayload::CounterUpdate` presente | ✅ |
| 8 | `ElementKind::CounterUpdate` presente | ✅ |
| 9 | `is_locatable(CounterUpdate) = true` | ✅ |
| 10 | `extract_payload` arm novo | ✅ |
| 11 | `from_tags` arm novo | ✅ |
| 12 | 3 mutações legacy preservadas no walk arm | ✅ |
| 13 | Comentário inline P198C presente | ✅ |
| 14 | L0 secção "Walk arm CounterUpdate migrado" | ✅ |
| 15 | Tabela Excepções M5 com E6 fechada estruturalmente | ✅ |
| 16 | `compute_*` helpers NÃO modificados | ✅ |
| 17 | Trait `Introspector` NÃO modificado | ✅ |
| 18 | `TagIntrospector` NÃO modificado | ✅ |
| 19 | Consumer C3 P184D / C4 P194B NÃO modificados | ✅ |
| 20 | Snapshot tests verdes | ✅ |
| 21 | Linter passa final | ✅ |

**21/21 verde.**

---

## §7 Decisões de execução notáveis

### ElementKind::CounterUpdate adicionada

Convenção cristalino: todo `ElementPayload` locatable tem
`ElementKind` correspondente para suportar
`query_by_kind`/`kind_index`. Adicionada como 10ª variant em
`entities/element_kind.rs`. `as_str()` retorna `"counter_update"`;
`from_name("counter_update")` ativo.

### Sem helper extraído

P195D/P196B/P197B extraíram helpers privados (`compute_labelled`,
`compute_heading_auto_toc`, `compute_figure`). P198B + **P198C**
não extraíram porque:
- P198B: mutação trivial 1 linha.
- P198C: lógica de mapeamento `(key, action) → CounterRegistry::{apply_at,apply_hierarchical_at}` está no `from_tags` arm próprio (não duplicada com walk arm que apenas chama `state.step_*`/`update_flat`).

Cenário α/β-promote aceita ambas formas (com ou sem helper).

### Import CounterUpdate em from_tags.rs

`from_tags.rs` não importava `CounterUpdate` enum no scope do
módulo (só dentro de `mod tests`). Necessário importar para
arm novo. Adicionado: `use crate::entities::counter_update::CounterUpdate;`.

### Cadeia E6 ↔ E4 (test 6)

Test 6 é sentinela cláusula gate substancial. Confirma que após
promote, `compute_labelled` Equation arm (P195D) continua a ler
`state.get_flat("equation")` durante walk. Mutação legacy
preservada → resolved_text "Equação (1)" produzido em paralelo
ao Tag::Labelled (P195D pos-recursão). Cadeia E6 ↔ E4 funcional.

---

## §8 Estado actual

- **P198 série**: A ✅ B ✅ C ✅ | D pendente.
- **E6 fechada estruturalmente** via cenário β-promote (1ª aplicação).
- **Hashes**: L0 `d25dfc47` ↔ código `f49ec9df`.
- **78 passos executados** (P198B=77 + P198C=78).

---

## §9 Pendências cumulativas

**Excepções activas pós-P198C**: **1 + 1 residuo**:
- E1 (Equation) — independente; pré-requisito `Content::SetEquationNumbering`.
- E2-residuo (Heading `headings_for_toc`) — pré-requisito sub-store `intr.headings_for_toc`.

**2 pré-requisitos restantes** (inalterado).

**M5 universal a 2 pré-requisitos do fecho** — ambos paralelos
fora série P198.

---

## §10 Próximo passo

**P198D** — encerramento série P198:
- Auditoria empírica final (12-14 verificações).
- Relatório consolidado `typst-passo-198-relatorio-consolidado.md` (9 secções padrão).
- Nota DEBT M5-residual actualizada (1 excepção activa + 1 residuo).
- Verificação estrutural final.

Magnitude **S puro**.

---

## §11 Linhagem

- **Pattern arquitectural stylesheet**: ADR-0069.
- **Variante operacional nova**: cenário β-promote (1ª aplicação P198C).
- **3 variantes operacionais consolidadas**: P195D (não-locatable + snapshot) + P196B (locatable + emitted_loc) + cenário α (P197B, P198B) + cenário β-promote (P198C).
- **5ª aplicação ADR-0069 stylesheet**: P195D + P196B + P197B + P198B + **P198C**.
- **Sub-store consumido**: `intr.counters` (CounterRegistry P184B).
- **Cadeia E6 ↔ E4**: `compute_labelled` Equation arm (P195D) preservado — lê `state.get_flat("equation")` durante walk.
- **L0 tocado**: `00_nucleo/prompts/rules/introspect.md` hash `d25dfc47`.
- **Código tocado**: 5 ficheiros `01_core/src/`:
  - `entities/element_payload.rs` (variant nova).
  - `entities/element_kind.rs` (variant nova).
  - `rules/introspect/locatable.rs` (locatable activo).
  - `rules/introspect/extract_payload.rs` (arm novo).
  - `rules/introspect/from_tags.rs` (arm novo + import).
  - `rules/introspect.rs` (comentário inline; hash `f49ec9df`).
- **Padrão diagnóstico-primeiro**: 20ª aplicação consecutiva (P198A diagnóstico).
- **Métricas finais P198C**:
  - LOC produção: ~80.
  - LOC teste: ~150.
  - LOC L0: ~80.
  - +6 testes workspace.
  - +1 ElementPayload variant (11 → 12).
  - +1 ElementKind variant (9 → 10).
