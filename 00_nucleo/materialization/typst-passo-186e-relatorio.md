# Relatório P186E — `from_tags` arm `Equation` com gate

**Data**: 2026-05-03
**Magnitude**: S puro
**Pré-condição**: P186D concluído ✅; tests workspace 1.793
verdes; zero violations; invariante `is_locatable ↔
extract_payload.is_some()` íntegra; `kind_index[Equation]`
populado por stub estendido em P186D.

---

## Resumo

Arm `from_tags::Equation` estendido de stub (apenas
`kind_index` populate, P186D) para arm completo com counter
logic gated por:

```rust
if *block
    && matches!(
        intr.state.value_at("numbering_active:equation", *loc),
        Some(Value::Bool(true)),
    )
{
    intr.counters.apply_at(
        "equation".to_string(),
        counter_update.clone(),
        *loc,
    );
}
```

**Decisão arquitectural** (cláusula `.B`): **Opção B**
location-aware. Inlining de `is_numbering_active_at`
(P185B) via `state.value_at(...)` directo + match
`Some(Value::Bool(true))`. Alinhada com direcção
arquitectural P185 + futureproof para quando
`Content::SetEquationNumbering` materializar com toggle
on/off por location.

**Gate dormente em produção** (caso central, registado
honestamente): `Content::SetEquationNumbering` não existe em
cristalino (P186A §11.2). State `numbering_active:equation`
nunca é populado pelo walk real → `state.value_at(..., _)`
sempre `None` → gate bloqueia → `counters["equation"]`
permanece vazio. P188 substitution-with-fallback cobre via
legacy `state.get_flat("equation")`.

L0 `from_tags.md` actualizado (entrada na lógica de match
+ 3 entradas em Histórico de Revisões cobrindo P186B/D/E).

4 tests unitários novos cobrem:
- Caso central produção (state ausente + block → gate
  dorme).
- Gate dispara (state activo + block → counter populado).
- Inline (state activo + block=false → gate dorme).
- Sequencialização (3 block consecutivos → counter [1,2,3]).

---

## Confirmação `.F` (11/11)

| # | Verificação | Estado |
|---|-------------|--------|
| 1 | `cargo check --workspace` passa | ✅ |
| 2 | `cargo test --workspace` passa (Δ vs 1793: +4) | ✅ 1797 verdes; range planeado +3 a +4 |
| 3 | `crystalline-lint .` zero violations | ✅ |
| 4 | Arm `ElementPayload::Equation` adicionado em `from_tags` | ✅ `from_tags.rs:225-242` |
| 5 | Gate `block && state-active` (location-aware) funcional | ✅ matches `Some(Value::Bool(true))` |
| 6 | `counters.apply_at("equation", ...)` chamado quando gate dispara | ✅ test `equation_arm_gate_dispara_block_e_state_active` |
| 7 | Em produção (state dormente): counter permanece vazio | ✅ test `equation_arm_gate_dorme_state_ausente_caso_producao` |
| 8 | `flat_counter_at("equation", loc)` retorna valor quando gate disparou; `None` em produção | ✅ implícito em #6 e #7 (counters.value_at é o backing) |
| 9 | Walk arm legacy intocado | ✅ `introspect.rs:377-382` confirmado por `git diff` |
| 10 | Snapshot tests ADR-0033 verdes | ✅ incluídos em workspace |
| 11 | Linter passa final | ✅ |

---

## Δ tests vs baseline

- Baseline P186D: **1793** verdes.
- Após P186E: **1797** verdes.
- Δ: **+4** (limite superior do range planeado +3 a +4).

Distribuição:
- `equation_arm_gate_dispara_block_e_state_active` ✅
  — gate semântica positiva.
- `equation_arm_gate_dorme_inline_mesmo_com_state_active` ✅
  — block=false bloqueia mesmo com state activo.
- `equation_arm_gate_dorme_state_ausente_caso_producao` ✅
  — **caso central da produção** validado empiricamente.
- `equation_arm_multiplas_block_sequencializam_counter` ✅
  — counter avança correctamente em sequência.

---

## Hashes finais

L0 modificado: `00_nucleo/prompts/rules/introspect/from_tags.md`

- Hash do código (registado no L0): `3a8f291a`
- Hash do prompt (`@prompt-hash` do `.rs`): `1164f135`

`crystalline-lint --fix-hashes .` aplicado uma vez. Análise
final ✅ 0 drift warnings remaining.

---

## Decisões de execução notáveis

### Opção B (location-aware) escolhida — `.B`

A spec ofereceu duas opções de gate:

- **Opção A**: `state.is_numbering_active(key)` (snapshot
  final).
- **Opção B**: `state.value_at(key, loc) == Some(Bool(true))`
  (location-aware).

**Escolha**: Opção B inlined via `matches!`. Razões:
- Em produção actual (gate dormente), Opção A e B são
  equivalentes em comportamento — ambas retornam `false`/None.
- Para tests E2E em P186F que injectam state: Opção B
  funciona melhor para futureproof toggle on/off por
  location.
- Coerência com P185 (location-aware é a direcção
  arquitectural ratificada por ADR-0068 ACEITE).
- Sem custo adicional em produção.

Implementação inlined em vez de chamar
`Introspector::is_numbering_active_at` para evitar import
do trait em `from_tags.rs` (que é o construtor do
TagIntrospector — não-circular mas estilisticamente
inversão). Lógica idêntica.

### Sem cláusula gate substancial

- `apply_at` API confirmada (per P184B).
- `state.value_at` acessível directamente (per
  `StateUpdate::Set` arm já existente em from_tags).
- `Value` import adicionado (cláusula gate trivial).
- Ordem em `from_tags`: tags processadas linearmente; uma
  tag StateUpdate antes da tag Equation popula state
  correctamente. Test `equation_arm_gate_dispara_*`
  confirma empiricamente.

### Comportamento em produção honestamente registado

- L0 `from_tags.md` linha sobre Equation declara
  explicitamente "**Gate dormente em produção** porque
  `Content::SetEquationNumbering` ausente em cristalino".
- Comentário inline no `.rs` explica:
  - Caminho funcional gated.
  - Ausência de produtor de `numbering_active:equation`.
  - Eixo 2 P183C resolvido **estruturalmente** (counter
    populável); resolução **funcional** depende de equation
    set rule (passo dedicado fora da série P186).
- Test `equation_arm_gate_dorme_state_ausente_caso_producao`
  é literalmente "caso central da produção" — passa por
  defeito (gate bloqueia) confirmando que P186E não
  introduz regressão observable.

### Histórico L0 com 3 entradas — coerência cronológica

Adicionei entradas em `from_tags.md` Histórico para:
- P186B (stub no-op original).
- P186D (stub estendido com `kind_index` populate via
  cláusula gate trivial — vide P186D §"Decisões").
- P186E (counter logic + gate location-aware).

Permite que leitores futuros sigam a evolução do arm sem
ter que cross-referenciar relatórios individuais.

---

## Estado actual

- **P186 série**: A ✅ B ✅ C ✅ D ✅ E ✅ | F pendente.
- **Eixo 2 do bloqueio P183C**: **resolvido estruturalmente**.
  Counter `equation` é populável quando state-active está
  presente; em produção dormente até equation set rule
  materializar.
- **Tests workspace**: 1.793 → **1.797** (+4).
- **Trait `Introspector`**: 18 métodos (inalterado).
- **`ElementPayload`**: 10 variants (inalterado).
- **`ElementKind`**: 9 variants (inalterado).
- **52 passos executados**.
- **DEBT M4-residual**: cobre C1 + C2 (inalterado; fecha
  após P187 + P188).
- **Invariante `is_locatable ↔ extract_payload.is_some()`**:
  íntegra (P186D restaurou).

---

## Pendências cumulativas

Inalteradas em P186E, com adição já documentada em P186D:

- ~~Janela invariante quebrada (Equation)~~ — fechada em
  P186D.
- **`Content::SetEquationNumbering` ausente** — pré-existente,
  documentado em P186A §11.2 e P186E `.B/.C/.D`. Não é
  DEBT P186 mas é trabalho identificado para passo
  dedicado fora da série P186-P188.
- P183B (C1 heading prefix) — depende P187.
- P183C (C2 equation counter) — depende P186F + P188.
- 4 sites M4-fora-de-escopo — fora de escopo P186.

P186 ainda pendente:
- P186F: tests E2E + relatório consolidado P186.

---

## Próximo passo

**P186F** — encerramento da série P186:

- Tests E2E `flat_counter_at("equation", loc)` via pipeline
  real (com state pré-populado para `numbering_active:equation`).
- Tests confirmam `flat_counter_at` retorna valor correcto
  para block-equations no estado activo.
- Tests confirmam fallback path (state ausente → None →
  P188 cobrirá com legacy).
- Relatório consolidado P186 (9 secções padrão
  P181J/P182F/P184F/P185-consolidado).

Após P186F, série P186 fecha. P187 (C1 migration) e P188
(C2 migration) ficam desbloqueados — ambos com infra
location-aware (P185) + variants e walk apropriados (P186).
