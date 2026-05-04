# Relatório P186D — Activar `is_locatable(Content::Equation) = true`

**Data**: 2026-05-03
**Magnitude**: trivial-S (executada como S por cláusula gate
trivial em `from_tags` — vide §"Decisões")
**Pré-condição**: P186C concluído ✅; tests workspace 1.793
verdes; zero violations; janela de invariante quebrada
activa para Equation.

---

## Resumo

Mudança central: arm `Content::Equation { .. }` em
`is_locatable.rs` movido de não-locatable para locatable,
retornando `true`. Acompanha:

- L0 `rules/introspect/locatable.md` actualizado (10
  locatable, 46 não-locatable; entrada `Equation` em
  Histórico).
- Fixture P185D `gating_locator_apenas_em_locatables`
  restaurado: `Content::Equation` re-incluída; expected
  count 3 → **4** locatables. Comentário transitório
  removido.
- Helper `build_minimal_for_each_variant` em
  `locatable.rs::tests` estendido com Equation —
  fecha lacuna pré-existente identificada em P186C.
- **Cláusula gate trivial em `from_tags`** (§"Decisões"):
  stub no-op P186B substituído por populate mínimo de
  `kind_index` (sem counter logic, que fica para P186E).

**Invariante reposta**: `is_locatable(Equation) ↔
extract_payload(Equation).is_some()` ambos `true`.
Sincronização Locator Layouter ↔ walk íntegra.

Walk emite Tag para Equations com payload válido. Sub-store
`CounterRegistry` para chave `"equation"` ainda **não**
populado (P186E activa); apenas `kind_index[Equation]`
ganha entries.

---

## Confirmação `.F` (12/13 + 1 ⚠️ cláusula gate trivial)

| # | Verificação | Estado | Nota |
|---|-------------|--------|------|
| 1 | `cargo check --workspace` passa | ✅ | |
| 2 | `cargo test --workspace` passa (Δ vs 1793: 0) | ✅ 1793 verdes | range planeado 0 a +1 |
| 3 | `crystalline-lint .` zero violations | ✅ | |
| 4 | `is_locatable(Content::Equation)` retorna `true` | ✅ `locatable.rs:59` | |
| 5 | **Invariante reposta**: `is_locatable ↔ extract_payload.is_some()` | ✅ | ambos `true` para Equation; test invariante passa |
| 6 | Fixture P185D restaurado com Equation | ✅ `tests.rs` `gating_locator_apenas_em_locatables` | expected 4 locatables |
| 7 | `build_minimal_for_each_variant` cobre Equation | ✅ `locatable.rs::tests` | lacuna pré-existente fechada |
| 8 | Test `gating_locator_apenas_em_locatables` passa com 4 locatables | ✅ | walk_locs == layout_locs == 4 |
| 9 | Test invariante passa para Equation | ✅ | iterando `build_minimal_for_each_variant` actualizado |
| 10 | Walk arm legacy intocado | ✅ `introspect.rs:377-382` | |
| 11 | `from_tags` stub no-op intocado | ⚠️ **modificado** (cláusula gate trivial — vide §"Decisões") | populate de `kind_index` adicionado; counter logic continua para P186E |
| 12 | Snapshot tests ADR-0033 verdes | ✅ | |
| 13 | Linter passa final | ✅ | |

---

## Δ tests vs baseline

- Baseline P186C: **1793** verdes.
- Após P186D: **1793** verdes.
- Δ: **0**.

Mudanças neutras em count:
- Test `gating_locator_apenas_em_locatables` ajustado
  (mesmo test, fixture diferente — count inalterado).
- `build_minimal_for_each_variant` itera mais 1 entry
  (Equation), mas é loop interno do test invariante —
  count de tests inalterado.
- Sem tests novos.

---

## Hashes finais

L0 modificado: `00_nucleo/prompts/rules/introspect/locatable.md`

- Hash do código (registado no L0): `4b2a29e5`
- Hash do prompt (`@prompt-hash` do `.rs`): `aaf16c83`

Outros 2 ficheiros mexidos sem L0 dedicado (tests + from_tags
stub) — `from_tags.rs` partilha hash com L0 from_tags.md
(sem mudança no L0 porque modificação é stub mínimo, não
arquitectural).

`crystalline-lint --fix-hashes .` aplicado uma vez. Análise
final ✅ 0 drift warnings remaining.

---

## Decisões de execução notáveis

### Inversão de ordem P186C/D não eliminou janela quebrada

Como descoberto em P186C `.A.6` empírico: walk de introspect
gateia em `extract_payload.is_some()`, **não** em
`is_locatable`. A inversão sugerida pela spec (P186C
`extract_payload` antes de P186D `is_locatable`) apenas
inverteu o **sentido** da quebra:

- **Sem inversão (ordem original)**: P186C activava
  `is_locatable=true` → Layouter avançaria Locator para
  Equation → walk não emitiria tag (porque
  `extract_payload` retornava `None`). Sequências:
  Layouter > walk.
- **Com inversão (executada)**: P186C activou
  `extract_payload` → walk emite tag → Layouter não avança
  (porque `is_locatable=false`). Sequências: walk >
  Layouter.

Em ambos os cenários a janela existe. Pragmatismo P186C
(remover Equation do fixture) preservou tests durante a
janela; P186D fecha-a activando `is_locatable` e restaura
fixture.

### Cláusula gate trivial em `from_tags` (§"Restrições" violada)

A spec P186D restringe explicitamente:
> **Não** modificar `from_tags` — P186E.

Mas a verificação `.F.8` exige:
> Test P185D `gating_locator_apenas_em_locatables` passa
> com 4 locatables.

**Inconsistência interna**: o test agrega
`intr.kind_index.values().flatten()` (helper
`collect_walk_locations`). Sem populate de `kind_index`
para Equation, walk_locs.len() = 3 enquanto layout_locs.len()
= 4 — test falha.

Acção tomada: estender o stub no-op (P186B) com populate
mínimo de `kind_index`, mantendo o counter logic em
suspensão para P186E:

```rust
ElementPayload::Equation { .. } => {
    intr.kind_index
        .entry(ElementKind::Equation)
        .or_default()
        .push(*loc);
}
```

Justificação:
- `kind_index` populate é estrutural e incondicional
  (igual aos outros arms do match — Heading, Figure, Cite,
  etc.).
- Counter logic (gate `block && state-active`) é a parte
  substantiva de P186E.
- Sem este populate, P186D não consegue cumprir
  verificação `.F.8`. Spec internamente inconsistente.

Comentário inline em `from_tags.rs` documenta a razão e
referencia que o counter logic vem em P186E.

### Lacuna pré-existente em `build_minimal_for_each_variant`

Auditor P186A identificou que `build_minimal_for_each_variant`
(helper do test invariante) não incluía Equation. Isto era
uma **lacuna silenciosa** — qualquer divergência entre
`is_locatable(Equation)` e `extract_payload(Equation).is_some()`
seria escondida porque o test não testaria Equation.

P186D fecha a lacuna adicionando
`Content::Equation { body: Box::new(Content::Empty), block: true }`
ao helper. Comentário inline regista que era lacuna
pré-existente (não criada por P186).

Outras variants podem ter lacuna semelhante (apenas as
explicitamente listadas no helper são testadas). Verificação
fora de escopo P186 — registo informal para passos futuros.

### Sem cláusula gate substancial disparada

- `is_locatable.rs` mudança é puramente estrutural (mover
  arm de bloco non-locatable para locatable).
- Fixture P185D restauro é operação inversa do que P186C
  fez — sem regressão.
- `build_minimal_for_each_variant` extensão é aditiva.

---

## Estado actual

- **P186 série**: A ✅ B ✅ C ✅ D ✅ | E-F pendentes.
- **Invariante reposta**: `is_locatable ↔ extract_payload.is_some()`
  para Equation. Sincronização Locator Layouter ↔ walk
  íntegra.
- **`is_locatable`**: 9 → **10 locatable**, 47 → **46
  não-locatable**.
- **`kind_index`**: agora popula para Equation no walk.
- **`CounterRegistry`** para chave `"equation"`: ainda **não**
  populado — fica para P186E (gate `block && state-active`).
- **Tests workspace**: 1.793 (inalterado).
- **51 passos executados**.
- **DEBT M4-residual**: cobre C1 + C2 (inalterado).

---

## Pendências cumulativas

Inalteradas em P186D, com fechamento:

- ~~Janela invariante quebrada (Equation)~~ — **fechada
  em P186D**.
- P183B (C1 heading prefix) — depende P187.
- P183C (C2 equation counter) — depende P186F + P188.
- 4 sites M4-fora-de-escopo — fora de escopo P186.

P186 ainda pendente:
- P186E: substituir stub `kind_index`-only em `from_tags`
  por arm completo com counter gate `block && state.value_at("numbering_active:equation", loc) == Some(Bool(true))`.
  Este gate dormente em produção até `Content::SetEquationNumbering`
  materializar.
- P186F: tests E2E + relatório consolidado.

---

## Próximo passo

**P186E** — substituir stub `kind_index`-only em `from_tags`
por arm funcional:

- Editar `01_core/src/rules/introspect/from_tags.rs`:
  - Stub actual popula apenas `kind_index`.
  - Estender com gate `block && state-active` chamando
    `counters.apply_at("equation", counter_update.clone(), *loc)`.
- Actualizar L0 `rules/introspect/from_tags.md`.
- Tests integration:
  - `equation_arm_popula_kind_index_e_counter` (com state
    pré-populado para `numbering_active:equation`).
  - `equation_inline_nao_popula_counter` (block=false, sem
    counter step).
  - `equation_block_sem_state_nao_popula_counter` (block=true
    mas state ausente — gate bloqueia).

Após P186E, infra Equation está completa: locatable + walk
populates `kind_index` + counter gated por block+state.
P186F consolidará com tests E2E paridade `flat_counter_at`
e relatório consolidado P186.
