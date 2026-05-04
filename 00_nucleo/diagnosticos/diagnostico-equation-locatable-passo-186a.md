# Diagnóstico — Equation locatable (Passo P186A)

**Data**: 2026-05-03
**Magnitude**: S (diagnóstico)
**ADR vinculada**: nenhuma (replicação de padrão P181/P182C/P184B).
**Pré-condição**: P185 série fechada; ADR-0068 ACEITE.

---

## §1 Validação do estado actual

### §1.1 — `Content::Equation` (entities/content.rs:84-87)

```rust
Equation {
    body:  Box<Content>,
    block: bool,
}
```

Apenas 2 campos. Nenhum `numbering`, `label`, ou kind.
Numeração de equações é controlada externamente (em
produção via `state.numbering_active["equation"]`).

### §1.2 — `ElementPayload` (entities/element_payload.rs:33-105)

9 variantes existentes:

```text
Heading, Figure, Citation, Metadata, State, StateUpdate,
Outline, Bibliography
```

**Equation: ausente.** Adicionar variant é obrigatório.

### §1.3 — `is_locatable` (rules/introspect/locatable.rs:60)

`Content::Equation { .. }` está no bloco "Não-locatable
(47 variants)" → `false`. P185D test `.C`
(`gating_locator_apenas_em_locatables`) confirmou
empiricamente que Layouter Locator não avança em Equation.

### §1.4 — `extract_payload` (rules/introspect/extract_payload.rs:83)

Catch-all `_ => None` cobre `Content::Equation`. Não há arm
explícito.

### §1.5 — `from_tags` arm Equation

Ausente. `ElementPayload::Equation` ainda não existe, logo
não pode haver arm.

### §1.6 — `ElementKind` (entities/element_kind.rs)

8 variantes:

```text
Heading, Figure, Citation, Metadata, State, StateUpdate,
Outline, Bibliography
```

**Equation: ausente.** Adicionar variante para indexar
locations de equações em `kind_index`.

### §1.7 — Walk legacy (rules/introspect.rs:377-382)

```rust
Content::Equation { block, body } => {
    if *block && state.is_numbering_active("equation") {
        state.step_flat("equation");
    }
    walk(body, state, locator, tags, None);
}
```

Gate: `block && is_numbering_active("equation")`. Sem
`Content::SetEquationNumbering`, `numbering_active["equation"]`
é populado externamente (tests via insert directo, eval
futuro).

### §1.8 — Vanilla typst

`lab/typst-original/.../math/equation.rs:234`:

```rust
Counter::of(EquationElem::ELEM)
```

Chave única simples — sem sub-kind. Confirma que cláusula 3
deve usar chave `"equation"` literal (sem sufixo).

### §1.9 — C2 consumer (rules/layout/equation.rs:25-33, 96-100)

```rust
let is_numbered = block
    && (self.introspector.is_numbering_active("numbering_active:equation")
        || self.counter.is_numbering_active("equation"));
// ...
if is_numbered {
    let n = self.counter.get_flat("equation");
    self.layout_content(&Content::text(format!("({})", n)));
}
```

Comentário inline na produção: *"cristalino ainda não tem
variant `Content::SetEquationNumbering`, logo o Introspector
path nunca activa para equation"*. Confirma que **a infra
P186 fica dormente em produção** até passo dedicado para
equation-set-rule.

### §1.10 — `CounterRegistry::apply_at` (entities/counter_registry.rs:156-161)

```rust
pub(crate) fn apply_at(&mut self, key: String, update: CounterUpdate, location: Location) {
    self.apply(key.clone(), update);
    if let Some(current) = self.inner.get(&key) {
        self.history.entry(key).or_default().push((location, current.clone()));
    }
}
```

`apply` usa `or_insert` default → auto-init na primeira
ocorrência. Cláusula 4 (auto-init explícito) **não é
necessária**.

---

## §2 Decisões cláusula 1–6

### §2.1 — Cláusula 1: forma do `ElementPayload::Equation`

**Decisão fixada**: **Opção B** — payload paralelo a Figure
(P184B):

```rust
Equation {
    block:          bool,
    counter_update: CounterUpdate,
}
```

**O1 — Inputs verificáveis**: §1.1 (Content::Equation tem
apenas `body, block`); §1.2 (ElementPayload variants);
§1.7 (walk legacy gate).

**O2 — Alternativas**:
- Opção A — payload mínimo `{ block }` apenas. Insuficiente:
  perde simetria com Figure que tem `counter_update`
  explícito.
- Opção B — `{ block, counter_update: Step }`. Aceite.
- Opção C — `{ block, label: Option<Label> }`. `label` não
  é campo de Content::Equation; recolhido via wrapping
  `Content::Labelled` em walk arm dedicado, fora do escopo.

**O3 — Critério**: simetria com `ElementPayload::Figure`
(P184B). `counter_update` reservado para futura flexibilidade
(ex.: equation set rule poder definir `Reset` ou
`StepBy(2)`).

**O4 — Magnitude**: trivial. ~5 LOC adicionando variant.

**O5 — Reversibilidade**: ALTA. Variant nova, sem
consumers ainda.

### §2.2 — Cláusula 2: sub-store alvo

**Decisão fixada**: **Opção 1** — `CounterRegistry`. Chave
`"equation"`.

**O1**: §1.10 (apply_at API pronta).

**O2**:
- Opção 1 — `CounterRegistry` reuso (per P184B padrão).
  Aceite.
- Opção 2 — sub-store dedicado `EquationStore`. Rejeitada:
  Equation não tem dados ricos como Bibliography (que tem
  entries + numbers); apenas counter.

**O3**: Equation tem counter incremental simples. Reuso de
`CounterRegistry` evita inflar entidades L1 sem ganho.

**O4 — Magnitude**: trivial. Sem novo sub-store.

**O5 — Reversibilidade**: ALTA.

### §2.3 — Cláusula 3: convenção de chave

**Decisão fixada**: **Opção A** — chave `"equation"` simples
(sem sufixo).

**O1**: §1.7 (walk legacy usa `"equation"`); §1.8 (vanilla
usa Counter::of(ELEM) sem sub-kind); §1.9 (consumer C2 lê
`get_flat("equation")`).

**O2**:
- Opção A — `"equation"` simples. Aceite.
- Opção B — `"equation:counter"` com sufixo. Rejeitada:
  P182A padrão `<feature>:<sub-feature>` é para distinguir
  variantes (ex.: `numbering_active:heading` vs
  `numbering_active:equation`); equations não têm variantes.
- Opção C — alinhamento com chave legacy. Resultado é
  literalmente `"equation"`, mesmo que Opção A.

**O3**: paridade literal com walk legacy + vanilla.

**O4 — Magnitude**: trivial.

**O5 — Reversibilidade**: ALTA enquanto não há consumers
externos. Após P188, mudar a chave exige refactor C2.

### §2.4 — Cláusula 4: auto-init em `from_tags`

**Decisão fixada**: **não é necessário**. `CounterRegistry::apply_at`
já é defensivo via `or_insert` default (§1.10). Nenhum
tratamento especial análogo a P182C
(`SetHeadingNumbering` → auto-init em `StateRegistry`).

### §2.5 — Cláusula 5: forma de migração C2 (P188)

**Decisão registada para P188** (não materializada em P186):

```rust
let n = self.introspector
    .flat_counter_at("equation", self.current_location.unwrap())
    .or_else(|| self.counter.get_flat("equation"))
    .unwrap_or(0);
```

Padrão P184D substitution-with-fallback. Trait method
`flat_counter_at` existe per P185B. Field
`current_location` existe per P185C (ADR-0068 ACEITE).

### §2.6 — Cláusula 6: critério de fecho de P186

**Decisão fixada**: **Opção 2** — replica padrão P184E.

P186 fecha quando:

1. `ElementPayload::Equation` adicionado ✓
2. `ElementKind::Equation` adicionado ✓
3. `is_locatable(Content::Equation { .. })` retorna `true` ✓
4. `extract_payload(Content::Equation)` produz `Some(...)` ✓
5. `from_tags` arm popula `kind_index[Equation]` e (gated)
   `counters["equation"]` ✓
6. Tests integration confirmam Tag emitida + sub-store
   populado correctamente ✓
7. Tests E2E confirmam paridade `flat_counter_at("equation",
   loc)` retorna valor correcto via pipeline real (com
   state pré-populado para `numbering_active:equation`) ✓

---

## §3 Plano de sub-passos

Sequência fixa B → C → D → E → F. Sem cláusulas
condicionais.

| Sub-passo | Escopo | Magnitude | Depende |
|-----------|--------|-----------|---------|
| **P186B** | Adicionar `ElementPayload::Equation { block, counter_update }` + `ElementKind::Equation` + L0s. Tests unit dos enums. | S | — |
| **P186C** | Modificar `is_locatable`: arm `Content::Equation { .. } => true` + L0. Test pontual. | trivial | — |
| **P186D** | Adicionar arm em `extract_payload` para `Content::Equation`. Tests `equation_produz_some_payload`. | S | `.B`, `.C` |
| **P186E** | Adicionar arm em `from_tags` para `ElementPayload::Equation`. Gate `block && state.value_at("numbering_active:equation", loc) == Some(Bool(true))`. Tests `equation_arm_popula_kind_index_e_counter`. | S | `.B`, `.D` |
| **P186F** | Tests E2E paridade `flat_counter_at("equation", loc)` via pipeline real (com state pré-populado). Relatório consolidado P186. | S | `.B`–`.E` |

Total agregado: ~80-150 LOC produção + ~80-120 LOC tests
≈ S puro.

---

## §4 Magnitude consolidada

P186 série: **S agregado** (B+C+D+E+F = 4×S + 1×trivial).

Diferente de P185 (M agregado por P185C arquitectural). P186
é replicação de padrão P181/P182C/P184B com escopo bem
delimitado a 4 sítios uniformes.

---

## §5 ADR avaliação

**Sem ADR criada.** Justificação:
- Promoção a locatable é refino dentro de Content enum
  fechado (ADR-0026) — não decisão arquitectural nova.
- Forma do payload replica P184B (Figure) — padrão
  estabelecido.
- Sub-store alvo `CounterRegistry` reuso — não inova.
- Convenção de chave `"equation"` segue paridade vanilla +
  legacy — não decisão.

---

## §6 DEBT avaliação

**Sem novo DEBT.**

DEBT M4-residual existente (cobre apenas C1 + C2) **não
fecha em P186** — fecha após P187 (C1) + P188 (C2).

Nota: dependência implícita de `Content::SetEquationNumbering`
(que não existe) torna o introspector path **dormente em
produção**. Esta condição já é documentada em `equation.rs:25-29`
e L0 layout. P186 não cria DEBT novo — herda condição
pré-existente. Quando equation-set-rule materializar (passo
fora da série P186-P188), o introspector path activa
automaticamente sem mudanças adicionais ao código de P186.

---

## §7 Relação com P183C bloqueio

P183C identificou 2 eixos para bloqueio C2:

- **Eixo 1 — semântica temporal**: snapshot-during-walk vs
  snapshot-final.
  - Resolvido em P185 (ADR-0068 ACEITE) via
    `current_location` no Layouter + `flat_counter_at`
    location-aware (P185B).
- **Eixo 2 — dados em sub-store**: `CounterRegistry` nunca
  recebe entry para chave `"equation"`.
  - **Resolvido em P186** — `from_tags` arm popula `counters`
    quando state-active.

Após P186F, **ambos eixos atendidos**. P188 fica desbloqueado
para migração consumer C2.

---

## §8 Próximo sub-passo

**P186B** — adicionar variant `ElementPayload::Equation` +
`ElementKind::Equation`:

- Editar `01_core/src/entities/element_payload.rs`:
  - Adicionar variant `Equation { block: bool, counter_update: CounterUpdate }`.
  - Actualizar tests existentes (verificar exaustividade
    em testes que enumeram variantes).
- Editar `01_core/src/entities/element_kind.rs`:
  - Adicionar variante `Equation`.
- Actualizar L0s correspondentes:
  - `00_nucleo/prompts/entities/element_payload.md`.
  - `00_nucleo/prompts/entities/element_kind.md`.
- Tests unit nos enums (constroi + Eq + Hash distinguishable).

Magnitude: S puro. Sem cláusulas condicionais.
