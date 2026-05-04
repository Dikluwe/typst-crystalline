# Relatório P185C — Layouter integration (`locator` + `current_location`)

**Data**: 2026-05-03
**Magnitude**: M (executada como M genuíno — primeira introdução
de `Locator` no Layouter; mecanismo M3 da ADR-0068 PROPOSTO)
**Pré-condição**: P185B concluído ✅ (trait `Introspector` 18
métodos; `is_numbering_active_at` + `flat_counter_at`
disponíveis).

---

## Resumo

`Layouter` ganha 2 fields:

- `locator: Locator` — gerador determinístico inicializado em
  `Layouter::new()` via `Locator::new()`.
- `current_location: Option<Location>` — `None` antes de
  qualquer locatable; `Some(loc)` após cada gating disparar.

Gating atómico no topo de `layout_content` via novo helper
`advance_locator_if_locatable(content)`:

```rust
fn advance_locator_if_locatable(&mut self, content: &Content) {
    if is_locatable(content) {
        self.current_location = Some(self.locator.next());
    }
}

pub fn layout_content(&mut self, content: &Content) {
    self.advance_locator_if_locatable(content);
    match content { /* arms inalterados */ }
}
```

Sincronização-por-construção com walk de introspect: ambos
disparam `locator.next()` exactamente nos mesmos pontos
(`is_locatable(c) ↔ extract_payload(c).is_some()`,
invariante de `locatable.rs:11`). `Locator::new()` em ambos
sítios produz a mesma sequência de `Location`s pela
determinismo do counter (P185A §3.3, provado por test).

Nenhum consumer migra ainda — Layouter ganha apenas a infra.
Tests E2E confirmando sincronização ficam para P185D.

---

## Confirmação `.H` (12/12)

| # | Verificação | Estado |
|---|-------------|--------|
| 1 | `cargo check --workspace` passa | ✅ |
| 2 | `cargo test --workspace` passa (Δ 0 vs 1779) | ✅ 1779 verdes |
| 3 | `crystalline-lint .` zero violations | ✅ |
| 4 | Layouter struct ganha `locator` + `current_location` | ✅ `mod.rs:131-141` |
| 5 | `layout_content` faz gating no topo via `is_locatable` | ✅ `mod.rs:236-240` (helper em `mod.rs:226-234`) |
| 6 | Walk de introspect **NÃO** modificado | ✅ `git diff` confirma |
| 7 | `Locator` API **NÃO** modificada | ✅ idem |
| 8 | Trait `Introspector` **NÃO** modificado | ✅ idem |
| 9 | Sub-stores (`StateRegistry`, `CounterRegistry`) **NÃO** modificados | ✅ idem |
| 10 | C1 e C2 **NÃO** migrados (P187/P188) | ✅ heading-arm e equation-arm intactos |
| 11 | Snapshot tests ADR-0033 verdes | ✅ incluídos em `cargo test --workspace` |
| 12 | Linter passa final | ✅ |

---

## Δ tests vs baseline

- Baseline P185B: **1779** verdes
- Após P185C: **1779** verdes
- Δ: **0** (esperado — `current_location` sem consumer não
  afecta nenhum observable; `Locator::new()` + gating são
  state interno).

Sem regressão. Tests existentes não mudam.

---

## Hashes finais

L0 modificado: `00_nucleo/prompts/rules/layout.md`

- Hash do código (registado no L0): `2b8010ce`
- Hash do prompt (registado em `@prompt-hash` dos `.rs`):
  `20d03fe5`

`crystalline-lint --fix-hashes .` aplicado uma vez. 9 ficheiros
de `01_core/src/rules/layout/` sincronizados (todos partilham o
mesmo prompt L0 — bump em cascata após edição da L0).

---

## Decisões de execução notáveis

### Cláusula 1 (inicialização do Locator) — Opção A

`Locator::new()` por defeito em `Layouter::new()`.
Determinismo do `Locator` (per `locator.rs:67-72`,
`duas_instancias_paralelas_produzem_sequencias_iguais`)
garante que `Locator::new()` em Layouter produz a mesma
sequência de `Location`s que `Locator::new()` no walk de
introspect — sincronização-por-construção sem partilha por
referência. Opção B (passar como argumento) e Opção C
(referência partilhada) descartadas por introduzirem
acoplamento desnecessário e mudança de API pública.

### Cláusula 2 (gating em `layout_content`) — Opção α

Gating atómico no topo de `layout_content`, antes do match.
Helper privado `advance_locator_if_locatable(content)` para
clareza e simetria com walk de introspect (que tem o gating
equivalente em `introspect.rs:329-339`). Opção β (gating por
arm) descartada por duplicação e risco de divergência. Opção
γ (wrapper com closure) descartada por overhead sem ganho.

### Cláusula 3 (save/restore para scoping) — Opção 2

Sem save/restore. `current_location` avança monotónico,
alinhado com walk de introspect. Auditoria `.A` não detectou
nenhum arm que precisasse de scoping local (nenhum consumer
existe ainda — questão é teórica até P187/P188).

Caso P185D detecte regressão por necessidade de scoping
léxico, o passo escala para revisão. Por agora a sincronização
literal (mesma forma do walk) é o caminho mais seguro.

### Tipo do field `current_location`: `Option<Location>` em vez de `Location`

`Location` não tem `Default` derivado e o construtor
`from_raw` é `pub(crate)`. Usar `Location::from_raw(0)` como
sentinel cria ambiguidade — `from_raw(0)` é a `Location` real
do **primeiro** locatable produzido por `Locator::next()`.

`Option<Location>` é unambíguo: `None` = "antes do primeiro
locatable"; `Some(loc)` = "último locatable processado".
Caller (P187/P188) consulta via pattern match, com
comportamento explícito para o caso "nada processado ainda".

### Locator não-Clone vs Layouter não-Clone

`Locator` é deliberadamente não-`Clone` (`locator.rs:23-24`)
para preservar o invariante de unicidade. `Layouter` actual
**também não deriva `Clone`** (verificado em `.A`), portanto
não há regressão. `layout_with_introspector` cria novos
Layouters via `Layouter::new()` em cada iteração do fixpoint
loop (`mod.rs:1453`) — cada iteração ganha um `Locator` fresh,
sincronização preservada por determinismo.

### Sem cláusula gate substancial disparada

Auditoria `.A` confirmou todas as suposições do plano:
- `is_locatable` é função pura sem side-effects.
- `Locator::next` API é a esperada.
- `layout_content` é o método único de despacho.
- Layouter não tem `Locator` implícito (nenhum field shadow).
- Tests existentes não regridem.

---

## Estado actual

- **P185 série**: A ✅ B ✅ C ✅ | D-E pendentes.
- **Layouter location-aware estruturalmente** — `locator` +
  `current_location` disponíveis; **sem consumer ainda**.
- **Trait `Introspector`**: 18 métodos (inalterado vs P185B).
- **Tests workspace**: 1779 (inalterado).
- **46 passos executados** (após P185B).
- **M5/M4 progresso**: 6/12 read-sites migrados — inalterado;
  C1 + C2 ainda bloqueados, desbloqueio em P187 + P188.
- **DEBT M4-residual**: cobre apenas C1 + C2 (inalterado per
  P184F cenário B).
- **ADR-0068**: PROPOSTO. Candidato a transitar PROPOSTO →
  ACEITE após validação P185D.

---

## Pendências cumulativas

Inalteradas em P185C:

- P183B (C1 heading prefix migration) — depende de P187
  (consumer migra `is_numbering_active` snapshot-final →
  `is_numbering_active_at(key, current_location)`).
- P183C (C2 equation counter migration) — depende de P188
  (consumer migra `state.get_flat("equation")` legacy →
  `flat_counter_at("equation", current_location)`).
- 4 sites M4-fora-de-escopo (TOC, fixpoint side-channels,
  resolved labels) — fora de escopo P185.

---

## Próximo passo

**P185D** — Tests E2E validando sincronização Locator do
Layouter ↔ Locator do walk de introspect. Exemplo de
verificação esperada:

1. Construir `Content` com sequência conhecida de locatables
   (heading + figure + cite).
2. Correr walk de introspect → recolher `Vec<Location>`
   das tags emitidas.
3. Correr layout instrumentado → recolher `Vec<Location>`
   visitadas (snapshot de `current_location` em cada
   transição).
4. Assertar que as duas sequências são iguais
   (sincronização-por-construção empiricamente confirmada).

Após P185D, ADR-0068 transita PROPOSTO → ACEITE em P185E
(encerramento da série P185).

P187 (C1 migration) e P188 (C2 migration) ficam para depois
de P185E.
