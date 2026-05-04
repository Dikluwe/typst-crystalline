# Diagnóstico — C2 equation counter migration (Passo P188a)

**Data**: 2026-05-04
**Magnitude**: S (diagnóstico)
**ADR vinculada**: nenhuma (replicação de padrão P187B).
**Pré-condição**: P187 série fechada; M5/M4 progresso 7/12;
DEBT M4-residual cobre apenas C2 (cenário B).

---

## §1 Validação do estado actual

### §1.1 — Site C2 actual (`equation.rs:97`)

```rust
if is_numbered {
    let n = self.counter.get_flat("equation");
    self.layout_content(&Content::text(format!("({})", n)));
    self.flush_line();
}
```

C2 = **`self.counter.get_flat("equation")`** em
`equation.rs:97`. Retorna `usize` directamente (não
Option). Usado para formatar prefixo `"({n})"` inline após
a equação block.

### §1.2 — Receptor (`self` é Layouter — Cenário 1)

`equation.rs:19`:
```rust
impl<M: FontMetrics, S: ImageSizer> super::Layouter<M, S> {
    pub(super) fn layout_equation(&mut self, body: &Content, block: bool) {
```

`self` é `Layouter<M, S>` directamente. Acesso a
`self.introspector` (P168) e `self.current_location`
(P185C) é inline sem reorganização.

### §1.3 — `flat_counter_at` API (P185B)

`introspector.rs`:
```rust
fn flat_counter_at(&self, key: &str, location: Location) -> Option<usize>;
```

Retorna `Option<usize>`. Implementação delega a
`counters.value_at(key, loc).?.last().copied()`.

### §1.4 — `current_location` no site

Equation é locatable após P186D
(`Content::Equation { .. } => true` em `locatable.rs:59`).
Gating `advance_locator_if_locatable` em `layout_content`
(P185C) precede match arm. Quando o arm Equation é
processado, `current_location = Some(loc_da_equation)`.

`layout_equation` é chamado pelo arm Equation em `mod.rs`.
Logo no site `equation.rs:97`, `self.current_location` é
`Some(loc)`.

### §1.5 — `get_flat` legacy

`counter_state_legacy.rs:148`:
```rust
pub fn get_flat(&self, key: &str) -> usize {
    self.flat.get(key).copied().unwrap_or(0)
}
```

Retorna `usize` directamente — **não Option**. Default 0 se
key ausente.

**Consequência**: substitution-with-fallback exige
`unwrap_or_else` (não `or_else` como P187B), porque
`flat_counter_at` retorna `Option<usize>` e `get_flat`
retorna `usize`:

```rust
self.current_location
    .and_then(|loc| self.introspector.flat_counter_at("equation", loc))
    .unwrap_or_else(|| self.counter.get_flat("equation"))
```

### §1.6 — Como counter equation é usado downstream

`equation.rs:97-98`:
```rust
let n = self.counter.get_flat("equation");
self.layout_content(&Content::text(format!("({})", n)));
```

`n: usize` formatado como `"(N)"` inline. Exemplo: equation
6 produz texto `"(6)"`. Substituição é local — apenas linha
97 muda; linha 98 continua a usar `n`.

### §1.7 — Tests existentes que cobrem equation counter

- `layout_equation_bloco_numerada` em `tests.rs:966-979` —
  empiricamente popula `state.numbering_active["equation"]
  = true` antes de `layout()`; verifica output `"(1)"`.
- Outros tests math (ex. `p185b_*` em introspector.rs)
  exercitam `flat_counter_at` em isolation.

P188B deve preservar `layout_equation_bloco_numerada` sem
regressão.

### §1.8 — `Content::SetEquationNumbering` confirmado ausente

`grep -rn "SetEquationNumbering" 01_core/src/` retorna zero
hits em produção. Apenas:
- `equation.rs:25-29` — comentário inline documentando
  ausência.
- `element_payload.rs:113` — comentário no L0.
- `locatable.rs:55` — comentário sobre gate dormente.

Comentário inline em `equation.rs`:
```rust
// Introspector primeiro (chave `numbering_active:equation`);
// cristalino ainda não tem variant `Content::SetEquationNumbering`,
// logo o introspector path nunca activa para equation,
// até passo dedicado equation-set-rule.
```

P186A §11.2 documentação confirmada empiricamente.

### §1.9 — DEBT M4-residual cenário

Per P187 §7: DEBT cobre apenas **C2** após P187B (era C1+C2
nas notas anteriores). Cenário B per P187A §8 — sem DEBT
formal aberto. P188B actualiza nota indicando que DEBT
**fica vazio em prática** (C2 fechado estruturalmente +
fallback legacy funcional permanente).

---

## §2 Decisões cláusulas 1–7

### §2.1 — Cláusula 1: forma da expressão

**Decisão fixada**: **Opção A** inline directo, paralelo
P187B com `unwrap_or_else` (porque legacy retorna `usize`):

```rust
let n = self.current_location
    .and_then(|loc| self.introspector.flat_counter_at("equation", loc))
    .unwrap_or_else(|| self.counter.get_flat("equation"));
```

**O1**: §1.1-1.5 (current_location é Some no site; legacy
retorna usize não Option).

**O2**:
- Opção A inline directo (aceite).
- Opção B variável intermédia: rejeitada — paralelo P187B
  já é variável local `let n = ...`.
- Opção C match explícito: rejeitada — verbose para 1
  expressão.

**O3**: P187B padrão replicado. Diferença empírica:
`unwrap_or_else` em vez de `or_else` para colmatar
divergência de tipo `Option<usize> → usize`.

**O4 — Magnitude**: trivial. ~3 LOC mudança.

**O5 — Reversibilidade**: ALTA.

### §2.2 — Cláusula 2: tratamento `None` do Introspector

**Decisão fixada**: **Opção A** — `unwrap_or_else` para
legacy `get_flat`.

**Diferença honesta face a C1 (P187B)**:
- Em C1: fallback é defensivo; raramente disparado em
  produção.
- Em C2: fallback é o **caminho funcional permanente** em
  produção. `flat_counter_at` retorna `None` sempre porque
  gate dormente em P186E nunca dispara (sem
  `Content::SetEquationNumbering`).

Decisão é a mesma; semântica documentada é diferente.

### §2.3 — Cláusula 3: tratamento `None` do `current_location`

**Decisão fixada**: **Opção B** — `and_then` defensivo.
Replica P187B.

Per §1.4, `current_location` é `Some` no site C2 (Equation
locatable; gating precede arm). `and_then` é defensiva sem
panic.

### §2.4 — Cláusula 4: receptor (Cenário 1)

**Confirmado empiricamente**: **Cenário 1**. `self` é
`Layouter<M, S>` directamente. `self.introspector` e
`self.current_location` acessíveis inline (per `equation.rs:19`
impl block).

Sem reorganização necessária.

### §2.5 — Cláusula 5: forma de retorno

**Decisão fixada**: **`usize`** preservado.

`flat_counter_at` retorna `Option<usize>`; `get_flat`
retorna `usize`. Conversão via `unwrap_or_else`. Variável
final `n: usize` mantém-se idêntica para uso downstream
(linha 98 inalterada).

Sem conversão de tipo. Apenas resolução do `Option`.

### §2.6 — Cláusula 6: documentação inline

**Decisão fixada**: **Opção A** — comentário curto inline
referenciando P186A §11.2 + P186E gate dormente.

Forma sugerida:
```rust
// P188B: substitution-with-fallback location-aware.
// Introspector path (`flat_counter_at`, P185B) é estrutural
// mas **dormente em produção** — gate em from_tags arm
// Equation (P186E) bloqueia porque
// `Content::SetEquationNumbering` ausente em cristalino
// (P186A §11.2). Fallback legacy `get_flat("equation")`
// é caminho funcional **permanente** até equation set rule
// materializar.
```

### §2.7 — Cláusula 7: critério de fecho

**Decisão fixada**: **Opção 3**. P188 fecha quando:
1. Consumer C2 migrado em `equation.rs:97`.
2. Tests E2E confirmam:
   - Path Introspector funcional quando state injectado
     (via `Content::StateUpdate` para
     `numbering_active:equation`).
   - Path fallback legacy funcional em produção (sem
     state).
   - Paridade observable nos dois paths.
3. Tests existentes (`layout_equation_bloco_numerada`,
   etc.) não regridem.
4. DEBT M4-residual nota actualizada: **vazio em prática**
   após P188.

---

## §3 Plano de sub-passos

Sub-passo único agregado (similar a P187B).

| Sub-passo | Escopo | Magnitude | Depende |
|-----------|--------|-----------|---------|
| **P188B** | Migrar `equation.rs:97` + L0 `rules/layout.md` + tests E2E paridade + comentário inline + actualização nota DEBT M4-residual + relatório consolidado P188 | S | — |

Total agregado P188B: ~5 LOC produção + ~80-120 LOC tests
+ documentação ≈ S puro.

---

## §4 Magnitude consolidada

P188 série: **S puro** (1×S agregado). Idêntico a P187.

Diferente de P186 (S agregado em 6 sub-passos para
infraestrutura). P188 é migração de consumer único — infra
já existe (P185 + P186).

---

## §5 ADR avaliação

**Sem ADR criada.** Justificação:
- Substitution-with-fallback é padrão P187B — não decisão
  arquitectural nova.
- `flat_counter_at` já existe (P185B) — sem nova primitiva.
- `current_location` já existe (P185C) — sem nova infra.
- Estado dormente é honestidade documental, não decisão
  arquitectural.
- `unwrap_or_else` vs `or_else` é detalhe de tipo, não
  decisão arquitectural.

---

## §6 DEBT avaliação (DEBT M4-residual vazio em prática)

### Cenário identificado: **B (continuação)**

Per P187A §8: DEBT M4-residual em cenário B (sem DEBT
formal aberto; apenas notas preventivas em relatórios
consolidados).

P187B reduziu cobertura de C1+C2 → C2.

P188B fecha C2 estruturalmente (Introspector path migrado;
fallback legacy permanente). DEBT M4-residual fica **vazio
em prática**:
- C1 fechado em P187B (Introspector funcional).
- C2 fechado em P188B (Introspector dormente; fallback
  legacy é caminho funcional).

P183F formal pode ser **dispensado** (DEBT vazio antes de
abrir formalmente), ou abrir e arquivar imediatamente.
Decisão fica para passo subsequente.

---

## §7 Estado dormente honestamente documentado

P188 é o **primeiro consumer onde Introspector path migra
mas não é caminho funcional em produção**. Comparação:

| Caso | Introspector em produção | Caminho funcional |
|------|---------------------------|-------------------|
| C3 Figure (P184D) | activo (figure_progress + idx) | Introspector |
| C1 Heading prefix (P187B) | activo (P182C SetHeadingNumbering existe) | Introspector |
| **C2 Equation counter (P188B)** | **dormente** (SetEquationNumbering ausente) | **fallback legacy permanente** |

Documentação obrigatória em 4 pontos:
1. Comentário inline em `equation.rs:97`.
2. Secção dedicada em L0 `rules/layout.md`.
3. Tests E2E `gate_dormente_caso_producao` que valida
   empiricamente caminho fallback.
4. Relatório consolidado P188 §"Estado dormente".

---

## §8 Próximo sub-passo

**P188B** — migração C2 + tests E2E + nota DEBT:

- Editar `01_core/src/rules/layout/equation.rs:97`:
  ```rust
  let n = self.current_location
      .and_then(|loc| self.introspector.flat_counter_at("equation", loc))
      .unwrap_or_else(|| self.counter.get_flat("equation"));
  ```
  Adicionar comentário inline (cláusula 6).

- Editar L0 `00_nucleo/prompts/rules/layout.md`:
  Secção "C2 equation counter migrado (P188B)" — paralela
  à secção P187B mas com nota explícita sobre estado
  dormente.

- Tests E2E em submódulo `p188b_c2_equation_counter`:
  - `c2_equation_counter_via_introspector_path_quando_state_injectado`.
  - `c2_equation_counter_via_fallback_legacy_caso_producao`
    (caso central — sem state populated).
  - `c2_equation_counter_paridade_legacy_vs_introspector`
    (quando ambos paths active, valores convergem).
  - Verificar que `layout_equation_bloco_numerada` existente
    não regride.

- Actualizar nota DEBT M4-residual no relatório consolidado
  P188.

Magnitude: S puro. Sem cláusulas condicionais.
