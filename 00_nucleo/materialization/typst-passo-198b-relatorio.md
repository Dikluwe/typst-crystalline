# Relatório P198B — Walk arm SetHeadingNumbering (cenário α)

**Data**: 2026-05-04
**Estado**: ✅ Completo (6 sub-passos A-F)
**Magnitude**: S puro (declaração formal — sem refactor de código produção)
**Pattern arquitectural**: ADR-0069 stylesheet — 4ª aplicação concreta (2ª em cenário α).

---

## §1 Sumário executivo

P198B declara formalmente **E5 fechada estruturalmente** via
cenário α. Caminho Introspector já estava activo desde P182C
(`extract_payload` → `Some(StateUpdate { key:
"numbering_active:heading" })` → `from_tags` arm StateUpdate
popula StateRegistry).

**Sem código produção modificado** além de comentário inline:
- Walk arm continua a chamar `state.numbering_active.insert("heading", *active)`.
- Mutação legacy preservada como write paralelo M5 — necessária
  porque `compute_heading_auto_toc` (P196B) e walk arm Equation
  lêem `state.numbering_active` durante walk.
- **Sem helper extraído** — distinção do padrão P195D/P196B/P197B
  porque mutação é trivial (1 linha).

**Output observable em produção**: inalterado.

---

## §2 Mutação preservada

| # | Mutação | Estado pós-P198B |
|---|---------|------------------|
| 1 | `state.numbering_active.insert("heading".to_string(), *active);` | **Preservada** (write paralelo M5) — `compute_heading_auto_toc` P196B + walk arm Equation lêem durante walk. Cleanup orgânico em M6. |

---

## §3 Comentário inline P198B (introspect.rs:611-624)

```rust
Content::SetHeadingNumbering { active } => {
    // P198B — E5 fechada estruturalmente (cenário α).
    // Caminho Introspector já activo desde P182C
    // (extract_payload → ElementPayload::StateUpdate sob
    // chave numbering_active:heading → from_tags arm
    // StateUpdate popula StateRegistry).
    //
    // Mutação legacy preservada como write paralelo M5:
    // compute_heading_auto_toc P196B + walk arm Equation
    // lêem state.numbering_active(_active) durante walk
    // para resolver auto-toc text e gate equation counter.
    // Cadeia E5 preservada. Cleanup orgânico em M6.
    state.numbering_active.insert("heading".to_string(), *active);
}
```

**Diferença vs P195D/P196B/P197B**: nenhum helper extraído.
Mutação é 1 linha; extracção não acrescenta valor estilístico.

---

## §4 Tests sentinela cenário α (5 testes novos)

| # | Test | Cobre |
|---|------|-------|
| 1 | `set_heading_numbering_extract_payload_emite_state_update` | `extract_payload(SetHeadingNumbering)` retorna `Some(StateUpdate { key: "numbering_active:heading", update: Set(Bool(true)) })` (P182C). |
| 2 | `set_heading_numbering_from_tags_popula_state_registry` | Pipeline walk + from_tags popula `intr.state` com chave canónica via P171/P182C arm. |
| 3 | `set_heading_numbering_paridade_legacy_vs_introspector` | Write paralelo: `state.is_numbering_active("heading") == true` ↔ `intr.is_numbering_active("numbering_active:heading") == true`. |
| 4 | `compute_heading_auto_toc_le_numbering_active_legacy` | Cadeia E5 — sem SetHeadingNumbering: resolved_text vazia (P196B §3); com SetHeadingNumbering: "Secção 1". |
| 5 | `walk_arm_set_heading_preserva_write_paralelo_para_compute_helpers` | Sentinela cláusula gate: mutação legacy preservada → consumer C4 P194B recebe `Some("Secção 1")` para auto-toc-1. |

---

## §5 L0 actualizado

`00_nucleo/prompts/rules/introspect.md` (hash novo `96597cb6`):

- Tabela "Excepções M5": linha **E5** → "**Fechou
  estruturalmente em P198B (cenário α — caminho Introspector
  activo desde P182C)**".
- Lista "Ordem inversa à mutação": passo 7 marcado ✅
  (P198B); estado P198B 2026-05-04 actualizado; passo 8
  ainda pendente (P198C — CounterUpdate).
- Nova secção **"Walk arm SetHeadingNumbering migrado (P198B,
  cenário α)"** — análoga a "Walk arm Figure migrado (P197B,
  cenário α)".
- Cross-references explícitas: P171, P173, P182C, P185B,
  P196B, P197B, ADR-0069.

---

## §6 Verificações finais (.E — 17 checks)

| # | Verificação | Resultado |
|---|-------------|-----------|
| 1 | `cargo check --workspace` | ✅ ok |
| 2 | `cargo test --workspace` | ✅ 1853 verdes (Δ vs baseline 1848: **+5**) |
| 3 | `crystalline-lint .` | ✅ 0 violations |
| 4 | Tests P198B passam isoladamente | ✅ 5/5 verde |
| 5 | Tests existentes não regridem | ✅ (1848 inalterado; sentinela E5 P189B preservada) |
| 6 | Comentário inline P198B presente | ✅ linhas 612-621 |
| 7 | Mutação legacy preservada | ✅ linha 622 |
| 8 | `extract_payload` arm SetHeadingNumbering NÃO modificado (P182C) | ✅ |
| 9 | `from_tags` arm StateUpdate NÃO modificado (P171) | ✅ |
| 10 | Variant `ElementPayload::StateUpdate` NÃO modificado | ✅ |
| 11 | L0 secção "Walk arm SetHeadingNumbering migrado" | ✅ presente |
| 12 | Tabela Excepções M5 com E5 fechada estruturalmente | ✅ |
| 13 | `compute_heading_auto_toc` P196B NÃO modificado | ✅ |
| 14 | Trait `Introspector` NÃO modificado | ✅ |
| 15 | Consumer C3 P184D / C4 P194B NÃO modificados | ✅ |
| 16 | Snapshot tests verdes | ✅ |
| 17 | Linter passa final | ✅ |

**17/17 verde.**

---

## §7 Decisões de execução notáveis

### Sem helper extraído

P195D/P196B/P197B extraíram helpers privados para consistência
com pattern ADR-0069 stylesheet. P198B não extrai porque:
- Mutação é 1 linha: `state.numbering_active.insert("heading", *active);`
- Não há lógica conditional, format, ou cálculo a isolar.
- Helper estilístico só introduziria boilerplate.

Cenário α aceita ambas formas (com ou sem helper). Distinção
documentada em L0 §"Walk arm SetHeadingNumbering migrado".

### Test 4 — sentinela cadeia E5

Test confirma comportamento empírico de `compute_heading_auto_toc`
(P196B): quando `numbering_active = false` (sem
SetHeadingNumbering precedente), helper retorna `(label, "")`
per P196B §3 paridade legacy. Confirma que mutação legacy é
necessária — sem ela, todos auto-toc labels teriam texto vazio.

---

## §8 Estado actual

- **P198 série**: A ✅ B ✅ | C-D pendentes.
- **E5 fechada estruturalmente** via cenário α — declaração
  formal sem código modificado.
- **Hashes**: L0 `96597cb6` ↔ código `ba7c22f6`.
- **77 passos executados** (P198A=76 + P198B=77).

---

## §9 Pendências cumulativas

**Excepções activas pós-P198B**: 2 + 1 residuo:
- E1 (Equation) — independente; pré-requisito `Content::SetEquationNumbering`.
- E2-residuo (Heading `headings_for_toc`) — pré-requisito sub-store `intr.headings_for_toc`.
- **E6 (CounterUpdate)** — pendente para P198C.

**2 pré-requisitos restantes** (inalterado).

---

## §10 Próximo passo

**P198C** — promote `Content::CounterUpdate` (cenário β-promote):
1. Adicionar variant `ElementPayload::CounterUpdate { key, action }`.
2. Mover `Content::CounterUpdate` para locatable.
3. Adicionar arm em `extract_payload.rs`.
4. Adicionar arm em `from_tags.rs` aplicando `apply_at` / `apply_hierarchical_at`.
5. Walk arm preservado (write paralelo M5).
6. L0 actualizada.
7. 5-6 tests E2E.

Magnitude **M** (5 ficheiros tocados; nova ElementPayload variant).

---

## §11 Linhagem

- **Pattern arquitectural stylesheet**: ADR-0069.
- **Variante operacional**: cenário α (P197B + **P198B**).
- **Reuso `ElementPayload::StateUpdate`** (P171/P173) sob chave canónica `numbering_active:heading` (P182C).
- **Sub-store consumido**: `intr.state` (StateRegistry P171/P182).
- **Consumer C5**: `is_numbering_active` (trait P185B/P171) — caminho activo.
- **Cadeia E5**: `compute_heading_auto_toc` (P196B) + walk arm Equation (linha 517) lêem `state.numbering_active` legacy — write paralelo preservado.
- **L0 tocado**: `00_nucleo/prompts/rules/introspect.md` hash `96597cb6`.
- **Código tocado**: `01_core/src/rules/introspect.rs` hash `ba7c22f6`.
- **Padrão diagnóstico-primeiro**: 20ª aplicação consecutiva (P198A diagnóstico).
- **2 aplicações cenário α consolidadas**: P197B (Figure) + P198B (SetHeadingNumbering).
- **4 aplicações ADR-0069 stylesheet**: P195D + P196B + P197B + P198B.
