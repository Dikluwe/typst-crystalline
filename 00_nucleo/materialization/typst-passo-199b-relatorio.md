# Relatório P199B — Materialização `Content::SetEquationNumbering`

**Data**: 2026-05-04
**Estado**: ✅ Completo (11 sub-passos A-K)
**Magnitude**: M genuína — Reserva 1 (P189B) materializada após >12 passos de espera.
**Pattern arquitectural**: ADR-0069 stylesheet — 6ª aplicação concreta (cenário α por construção 1ª aplicação).

---

## §1 Sumário executivo

P199B materializa `Content::SetEquationNumbering` — Reserva 1
desde P189B. **E1 fecha estruturalmente** via cenário α por
construção: variant nova adicionada, infraestrutura downstream
**activa imediatamente em produção real**:
- StateRegistry populated via Tag::StateUpdate (chave `numbering_active:equation`).
- Gate em `from_tags::Equation` (P186E) **agora dispara** em produção (antes dormente).
- CounterRegistry para `equation` populated.
- Layouter `equation.rs:32-33` first branch (substitution-with-fallback antes adormecida) **activa**.

**M5 universal a 1 pré-requisito paralelo do fecho**: sub-store
`intr.headings_for_toc` (E2-residuo).

**Output observable em produção**: alterado **apenas para activar**
o caminho Introspector — paridade observable preservada (write
paralelo legacy fornece valores idênticos).

---

## §2 Distinção arquitectural — cenário α por construção

P199B introduz **5ª variante operacional ADR-0069**:

| Variante | Pré-passo | Trabalho |
|----------|-----------|----------|
| P195D (não-locatable) | Caminho inactivo | Tag pós-recursão + snapshot+find_map |
| P196B (locatable + body) | Caminho inactivo | Tag pós-recursão + emitted_loc directo |
| Cenário α (P197B, P198B) | Caminho activo | Refactor estilístico ou declaração formal |
| **Cenário α por construção (P199B)** | **Caminho activável** | **Materializar variant — caminho activa imediatamente** |
| Cenário β-promote (P198C) | Caminho inactivo | Promote completo (variant + locatable + 2 arms) |

P199B é distinto de cenário α padrão (P198B):
- P198B: variant **já existia**; cenário α era declaração formal sobre código existente.
- P199B: variant **não existia**; toda a infraestrutura downstream (arm StateUpdate genérica P171; Layouter substitution-with-fallback antes adormecida) já estava preparada **a aguardar a materialização da variant**.

---

## §3 Trabalho concreto

| # | Ficheiro | Mudança |
|---|----------|---------|
| 1 | `entities/content.rs` | Variant `SetEquationNumbering { active: bool }` adicionada após `SetHeadingNumbering`; comentário documenta DEBT-10 + materialização P199B. |
| 2 | `entities/content.rs` | 4 match arms exhaustivos cobertos: `plain_text`, comparação `eq`, 2 listas de "terminais sem effect em counters". |
| 3 | `rules/introspect/locatable.rs` | Arm `Content::SetEquationNumbering { .. } => true`. |
| 4 | `rules/introspect/extract_payload.rs` | Arm retorna `Some(StateUpdate { key: "numbering_active:equation", update: Set(Bool(*active)) })`. |
| 5 | `rules/introspect.rs` | Walk arm muta `state.numbering_active.insert("equation", *active)` + comentário inline P199B; arm de "terminais" em `materialize_time` actualizado. |
| 6 | `rules/layout/counters.rs` | Helper `layout_set_equation_numbering(counter, active)` paralelo a `layout_set_heading_numbering`. |
| 7 | `rules/layout/mod.rs` | Layouter consumer arm `Content::SetEquationNumbering { active }` chama helper. |
| 8 | `prompts/rules/introspect.md` | Tabela Excepções E1 fechada estruturalmente; secção nova "Variant SetEquationNumbering materializada (P199B, cenário α por construção)"; ordem inversa passo 9 ✅. |

**`from_tags` arm StateUpdate (P171) NÃO modificado** — genérica, processa `numbering_active:equation` transparentemente.

---

## §4 Variant nova `Content::SetEquationNumbering`

```rust
/// Activa ou desactiva a numeração automática de equations.
/// Análoga a `SetHeadingNumbering` (P57). Materializada em P199B —
/// fecha Reserva 1 (E1 P189B) estruturalmente. Cenário α por
/// construção (ADR-0069): caminho Introspector activa
/// imediatamente porque arm `from_tags::StateUpdate` (P171)
/// é genérica e Layouter `equation.rs:32-33` já tem
/// substitution-with-fallback implementada.
/// DEBT-10: substituir por StyleChain quando o motor de
/// introspecção completo for implementado.
SetEquationNumbering { active: bool },
```

---

## §5 Walk arm + comentário inline

```rust
Content::SetEquationNumbering { active } => {
    // P199B — E1 fechada estruturalmente (cenário α por
    // construção). Materializa Reserva 1 desde P189B.
    //
    // Caminho Introspector activado por construção desde a
    // materialização: extract_payload →
    // ElementPayload::StateUpdate sob chave
    // numbering_active:equation → from_tags arm StateUpdate
    // (P171, genérica) popula StateRegistry. Layouter
    // equation.rs:32-33 first branch
    // (substitution-with-fallback antes adormecida) activa
    // em produção real.
    //
    // Mutação legacy preservada como write paralelo M5:
    // walk arm Equation (introspect.rs gate em is_numbering_active
    // para counter step) + compute_labelled Equation arm (P195D)
    // lêem state.is_numbering_active("equation") /
    // state.get_flat("equation") durante walk para gating + format.
    // Cadeia E1 preservada. Cleanup orgânico em M6.
    state.numbering_active.insert("equation".to_string(), *active);
}
```

---

## §6 Tests E2E (5 testes novos)

| # | Test | Cobre |
|---|------|-------|
| 1 | `set_equation_numbering_extract_payload_emite_state_update` | `extract_payload(SetEquationNumbering)` retorna `Some(StateUpdate { key: "numbering_active:equation", update: Set(Bool(true)) })`. |
| 2 | `set_equation_numbering_from_tags_popula_state_registry` | Pipeline walk+from_tags popula `intr.state` com chave canónica via P171 arm StateUpdate genérica (sem modificação). |
| 3 | `set_equation_numbering_paridade_legacy_vs_introspector` | Write paralelo: `state.is_numbering_active("equation") == true` ↔ `intr.is_numbering_active("numbering_active:equation") == true`. |
| 4 | `walk_arm_equation_le_numbering_active_legacy_apos_set` | Cadeia E1 — com SetEquationNumbering: counter equation avança para 1; sem: counter fica 0. |
| 5 | `consumer_layouter_equation_activa_via_introspector` | Activação por construção — `compute_labelled` Equation arm produz "Equação (1)" (legacy); `intr.resolved_labels` populated; `intr.is_numbering_active("numbering_active:equation") == true`. |

---

## §7 Verificações finais (.J — 21 checks)

| # | Verificação | Resultado |
|---|-------------|-----------|
| 1 | `cargo check --workspace` | ✅ ok |
| 2 | `cargo test --workspace` | ✅ 1864 verdes (Δ vs baseline 1859: **+5**) |
| 3 | `crystalline-lint .` | ✅ 0 violations |
| 4 | Tests P199B passam isoladamente | ✅ 5/5 verde |
| 5 | Tests existentes não regridem | ✅ |
| 6 | Tests adaptados | N/A (sem regressões) |
| 7 | Variant `Content::SetEquationNumbering` presente | ✅ |
| 8 | Comentário DEBT-10 presente | ✅ |
| 9 | `is_locatable(SetEquationNumbering) = true` | ✅ |
| 10 | `extract_payload` arm com chave canónica | ✅ |
| 11 | Walk arm com comentário inline P199B | ✅ |
| 12 | Mutação legacy preservada | ✅ |
| 13 | `from_tags` arm StateUpdate **NÃO** modificado | ✅ |
| 14 | Layouter `equation.rs:32-33` **NÃO** modificado | ✅ (apenas adicionado novo arm em `mod.rs` para consumer SetEquationNumbering — independente) |
| 15 | `compute_labelled` Equation arm **NÃO** modificado | ✅ |
| 16 | Walk arm Equation **NÃO** modificado | ✅ |
| 17 | L0 entries novas presentes | ✅ |
| 18 | Tabela Excepções M5 com E1 fechada estruturalmente | ✅ |
| 19 | Trait `Introspector` **NÃO** modificado | ✅ |
| 20 | Snapshot tests verdes | ✅ |
| 21 | Linter passa final | ✅ |

**21/21 verde.**

---

## §8 Decisões de execução notáveis

### Match arms induzidos cobertos via cargo check

`cargo check` revelou 7 sítios non-exhaustive após adicionar a variant:
- `content.rs:980` (`plain_text`).
- `content.rs:1200` (comparação `eq`).
- `content.rs:1483, 1694` (2 listas de "terminais sem effect em counters").
- `introspect.rs:101` (lista terminais em `materialize_time`).
- `introspect.rs:453` (walk match — coberto adicionando walk arm em §5).
- `layout/mod.rs:257` (Layouter dispatch — coberto adicionando consumer arm).

Todos cobertos replicando padrão de `SetHeadingNumbering` + adição de helper Layouter `layout_set_equation_numbering` para paralelismo.

### Helper Layouter adicionado (extensão fora P199A escopo)

P199A não previu adição de helper Layouter `layout_set_equation_numbering`. Foi necessário porque o consumer arm em `layout/mod.rs` requer função correspondente. Decisão: replicar pattern de `layout_set_heading_numbering` (1 linha + comentário). Magnitude trivial.

### DEBT-10 introduzida no comentário

Variant comenta DEBT-10 (StyleChain futuro per vanilla typst). Auditor pode formalizar entry em `m1-lacunas-captura.md` se desejar — não obrigatório porque DEBT-10 já existe documentada em `SetHeadingNumbering` (precedente).

### Layouter substitution-with-fallback activada por construção

Confirmado empiricamente que `equation.rs:32-33` first branch retorna `Some` em testes pós-P199B — caminho Introspector activado imediatamente. Padrão arquitectural notável: planeamento antecipado de fallback que activa no momento da materialização da variant trigger.

---

## §9 Estado actual

- **P199 série**: A ✅ B ✅ | C pendente.
- **E1 fechada estruturalmente** via cenário α por construção (1ª aplicação).
- **Hashes**: L0 `603170c8` ↔ código `0092886d`.
- **Reserva 1 desde P189B materializada** após >12 séries.

---

## §10 Pendências cumulativas

**Excepções activas pós-P199B**: **0 + 1 residuo**:
- E2-residuo (Heading `headings_for_toc`) — pré-requisito sub-store `intr.headings_for_toc` (lacuna #3).

**1 pré-requisito restante**.

**M5 universal a 1 passo paralelo do fecho** — sub-store
`headings_for_toc` é único restante. Após esse passo, M5
universal completo desbloqueia M6 (P190A reescrita do zero).

---

## §11 Próximo passo

**P199C** — encerramento série P199:
- Auditoria empírica final (12-13 verificações).
- Relatório consolidado `typst-passo-199-relatorio-consolidado.md` (9 secções padrão).
- Nota DEBT M5-residual actualizada (0 excepções + 1 residuo).
- Verificação estrutural final.

Magnitude **S puro**.

---

## §12 Linhagem

- **Pattern arquitectural stylesheet**: ADR-0069.
- **Variante operacional nova**: cenário α por construção (1ª aplicação P199B; sub-variante de cenário α padrão).
- **5 variantes operacionais ADR-0069 consolidadas**:
  - P195D (não-locatable + snapshot+find_map).
  - P196B (locatable + body + emitted_loc directo).
  - Cenário α (P197B, P198B).
  - **Cenário α por construção (P199B)** — distinguido formalmente.
  - Cenário β-promote (P198C).
- **6 aplicações concretas ADR-0069 stylesheet**: P195D + P196B + P197B + P198B + P198C + **P199B**.
- **Template primário**: P182C (`SetHeadingNumbering`) replicado literalmente com chave `equation`.
- **Reuso `ElementPayload::StateUpdate`** (P171/P173) sob chave canónica `numbering_active:equation`.
- **Reuso arm `from_tags::StateUpdate`** (P171) — genérica.
- **Sub-store consumido**: `intr.state` (StateRegistry P171/P182).
- **Consumer Layouter activado**: `equation.rs:32-33` substitution-with-fallback antes adormecida — first branch retorna Some pós-P199B.
- **Cadeia E1**: walk arm Equation (gate counter step) + `compute_labelled` Equation arm (P195D format) — ambos preservados; lêem state legacy.
- **L0 tocado**: `00_nucleo/prompts/rules/introspect.md` hash `603170c8`.
- **Código tocado**: 7 ficheiros `01_core/src/`:
  - `entities/content.rs` (variant + 4 match arms).
  - `rules/introspect/locatable.rs` (locatable arm).
  - `rules/introspect/extract_payload.rs` (extract arm).
  - `rules/introspect.rs` (walk arm + comentário inline + 1 lista terminais; hash `0092886d`).
  - `rules/layout/counters.rs` (helper novo).
  - `rules/layout/mod.rs` (consumer arm Layouter).
- **Padrão diagnóstico-primeiro**: 21ª aplicação consecutiva (P199A diagnóstico).
- **Marco arquitectural**: M5 universal a 1 passo paralelo do fecho (E2-residuo) após P199 fechar.
- **Métricas finais P199B**:
  - LOC produção: ~70 (variant + 3 arms + walk arm + helper Layouter + consumer Layouter + 4 match arms induzidos).
  - LOC teste: ~150.
  - LOC L0: ~80.
  - +5 testes workspace.
  - +1 Content variant.
