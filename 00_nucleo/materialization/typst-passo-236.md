# Passo 236 — D.1 `state(key, init)` runtime mutable (Fase 5 Layout candidata Categoria D 1/?; promove **ADR-0066 PROPOSTO → IMPLEMENTADO**; **primeira excepção justificada à aplicação automática ADR-0080 EM VIGOR** pós-P229)

**Série**: 236 (vigésimo-segundo sub-passo Layout pós-M9c;
**nono sub-passo materialização Fase 5 Layout candidata**
per ADR-0079 PROPOSTO; **primeiro sub-passo Categoria D**
"runtime queries"; **primeira feature runtime aditiva
pós-Fase 5 cosmética/algorítmica**).
**Marco**: **transição ADR-0066 PROPOSTO → IMPLEMENTADO**
(paridade ADR-0078 P221 promoção pós-materialização);
**primeira excepção justificada à aplicação automática
ADR-0080 EM VIGOR pós-P229** (pattern N=6 cumulativo
preservado mas P236 NÃO incrementa N=7 — L0 tocado para
feature runtime nova); **+33pp Introspection** (cobertura
módulo Introspection actualizada); pattern emergente
"L0 tocado para features runtime novas (não aplicação
automática ADR-0080)" N=1 inaugurado P236; pattern
"Promoção ADR PROPOSTO → IMPLEMENTADO integrada em
sub-passo materialização" N=1 → 2 cumulativo (P221
ADR-0078; **P236 ADR-0066**).
**Tipo**: feature runtime nova — **+1 Value variant
(`Value::State`)** + entity `State` struct + **+1 Content
variant (`Content::StateUpdate`)** + 4 stdlib funcs novas
+ Layouter +1 field HashMap; L0 tocado partial (~2-3
ficheiros: `entities/value.md` + `rules/stdlib.md` +
possível `entities/state.md` novo).
**Magnitude**: M+ (~3-4h; acima limite M paridade
diagnóstico P226 por escopo runtime + L0 tocado).
**Pré-condição**: P235 concluído (B.3 GridCell algorítmico;
Categoria B 3/3 ✓ FECHADA; 2137 verdes; 0 violations;
saldo DEBTs 11; ADR-0079 Categoria A 5/5 + Categoria B
3/3); humano fixou D.1 (decisão literal pós-P235 §8);
**ADR-0066 PROPOSTO** baseline P226 (audit C1 obrigatório
ler escopo completo); `Value` enum baseline com 55 variants;
`Content` enum baseline com 59 variants; Layouter struct
baseline com `cell_origin_*` (P84.6) + `cell_align` (P232);
Locator P139+P140 baseline (precedência para feature
runtime stateful); pattern "Field armazenado semantic
adiada" N=8 baseline P235 (state.final candidato N=8 → 9);
**ADR-0080 EM VIGOR** baseline P229 (audit C1 §"Escopo"
para confirmar excepção justificada).
**Output**: 1 ficheiro relatório curto + código alterado em
~6-8 ficheiros L1 (`entities/value.rs`, `entities/content.rs`,
`entities/state.rs` novo, `rules/stdlib/state.rs` novo,
`rules/layout/mod.rs`, `rules/introspect.rs`) + **L0
TOCADO partial** (~2-3 ficheiros: `entities/value.md` +
`rules/stdlib.md` + possível `entities/state.md` novo) +
inventário 148 anotação cumulativa (footnote ⁵⁵) + ADR-0066
status PROPOSTO → IMPLEMENTADO + ADR-0079 anotação
**Categoria D 1/? sub-passos materializados; ADR-0066
PROPOSTO → IMPLEMENTADO via D.1**.

---

## §1 Trabalho

P226 diagnóstico Categoria D.1 marcou literal: "**D.1
state(key, init) (M); desbloqueia ADR-0066 PROPOSTO →
IMPLEMENTADO; +33pp Introspection**".

ADR-0066 PROPOSTO baseline P226 documenta escopo runtime
queries (audit C1 obrigatório). D.1 materializa subset
minimal funcional que satisfaz "implementado" semantic
para ADR-0066.

Vanilla `state(key, init)` API:
- `state(key, init) -> State` — constructor.
- `state.get()` — lê valor actual durante walk.
- `state.update(value) -> Content::StateUpdate` — actualiza
  (walk-time side-effect).
- `state.at(location)` — lê valor em location específica.
- `state.final()` — valor final pós-walk.
- `state.display(...)` — render com função opcional.

**P236 materializa D.1 subset minimal**:
- **`Value::State(State)`** variant novo (paridade pattern
  P227 `Value::Stroke`).
- **Entity `State { key: String, init: Box<Value> }`**
  struct nova em `entities/state.rs` (módulo novo).
- **`Content::StateUpdate { key: String, value: Box<Value>
  }`** variant novo (walk-time mutation).
- **4 stdlib funcs novas**: `state(key, init)`,
  `state.get(state)`, `state.update(state, value)`,
  `state.final(state)` (state.final adiada graded).
- **Layouter +1 field**: `state_table: HashMap<String,
  Value>` para persistir valores entre walks.
- **L0 tocado partial**: `entities/value.md` (+State
  variant), `rules/stdlib.md` (+4 funcs), possível
  `entities/state.md` novo.

**Decisão arquitectural central — 8 decisões fixadas**:

### Decisão 1 — Escopo Opção α (subset minimal 4 operations)

3 opções consideradas:

| Opção | Operations | Trade-off |
|-------|-----------|-----------|
| **α** | `state(key, init)` + `state.get()` + `state.update(value)` + `state.final()` graded | Subset minimal funcional; `state.at(location)` + `state.display(...)` refinos futuros |
| β | + `state.at(location)` + `state.display(...)` | Inflacionário; magnitude L+ |
| γ | Apenas `state(key, init)` + `state.update` (sem `get`/`final`) | Insuficiente para uso prático |

**Decisão fixada — Opção α** (subset minimal funcional).
`state.final()` armazenado graded; `state.at(location)`
+ `state.display(...)` refinos futuros não-bloqueadores
(candidatos D.2+).

**Pattern emergente "subset minimal aditivo: 4 operations
primário; refinos futuros não-bloqueadores" N=1 inaugurado
P236**.

### Decisão 2 — `Value::State(State)` variant novo Opção β

3 opções consideradas:

| Opção | Mecânica | Trade-off |
|-------|----------|-----------|
| α | `Content::State { key, init }` variant novo | Confunde semantic Content (renderizável) vs Value (não-renderizável) |
| **β** | `Value::State(State)` + `State` struct nova | Paridade pattern P227 `Value::Stroke`; coerente |
| γ | Stateful via Locator interno sem variant novo | Inflexível; viola pattern Value-explícito |

**Decisão fixada — Opção β** (Value::State variant novo):

```rust
// entities/value.rs:
pub enum Value {
    // ... existing 55 variants ...
    State(State),  // P236
}

// entities/state.rs (módulo novo):
pub struct State {
    pub key: String,
    pub init: Box<Value>,
}
```

Value variants: 55 → **56** (+State).

**Pattern "Value variant novo para tipo composto stdlib-construído"
N=1 → 2 cumulativo** (P227 Stroke; **P236 State**).

### Decisão 3 — Infraestrutura mutation Opção α (Layouter HashMap)

3 opções:

| Opção | Mecânica | Trade-off |
|-------|----------|-----------|
| **α** | Layouter `state_table: HashMap<String, Value>` +1 field | Simplicidade pragmática; reuso Layouter scope baseline |
| β | HashMap embutido em Locator P139+P140 | Acoplamento desnecessário com Locator |
| γ | `StateRegistry` nova struct dedicada | Inflacionário; Layouter scope suficiente |

**Decisão fixada — Opção α**:

```rust
// rules/layout/mod.rs:
pub struct Layouter<'a> {
    // ... existing ...
    pub cell_origin_x: Option<f64>,    // P84.6
    pub cell_origin_y: Option<f64>,    // P84.6
    pub cell_origin_w: Option<f64>,    // P84.6
    pub cell_align: Option<Align2D>,   // P232
    /// P236 — state_table associa keys a valores mutáveis
    /// durante walk. `state(key, init)` cria entry se ausente;
    /// `state.update(state, value)` actualiza; `state.get(state)`
    /// lê.
    pub state_table: HashMap<String, Value>,
}
```

**Pattern emergente "Layouter +1 field HashMap para feature
runtime" N=1 inaugurado P236**.

### Decisão 4 — `state.update(value)` via Content::StateUpdate Opção γ

3 opções:

| Opção | Mecânica | Trade-off |
|-------|----------|-----------|
| α | Function call mutation directly `update_state(key, value)` em stdlib | Side-effect fora walk; difícil rastrear |
| β | Method-style `state.update(value)` requer dispatch em Value::State | Inflexível; Value não-dispatcheable |
| **γ** | `Content::StateUpdate { key, value }` variant novo (walk-time mutation) | Coerente com pattern walk-based |

**Decisão fixada — Opção γ**:

```rust
// entities/content.rs:
pub enum Content {
    // ... existing 59 variants ...
    StateUpdate {                       // P236
        key: String,
        value: Box<Value>,
    },
}
```

Content variants: 59 → **60** (+StateUpdate).

`stdlib::state.update(state, value)` retorna
`Content::StateUpdate { key: state.key, value }`; walk
encontra esta Content variant e aplica mutation no
`Layouter.state_table`.

### Decisão 5 — `state.final()` Opção β graded

3 opções:

| Opção | Mecânica | Trade-off |
|-------|----------|-----------|
| α | Two-pass walk completo agora (pass 1: compute final; pass 2: resolve final) | Refactor arquitectural maior; magnitude L+ |
| **β** | `state.final()` armazenado adiada graded paridade pattern N=8 baseline | Pattern N=8 → 9 cumulativo; semantic adiada per ADR-0054 |
| γ | `state.final()` retorna valor corrente (não-final) | Semantically wrong; viola paridade vanilla |

**Decisão fixada — Opção β** (graded):

Pattern "Field/operation armazenado semantic adiada" N=8
→ **9 cumulativo** (+state.final P236):
- weak P156D + weak P156E + breakable P156G + float P223
  + repeat P224.B + outset+radius+clip P231 + breakable
  per-cell P235 + **state.final P236**.

Two-pass walk = sub-passo **D.2** candidato (não-reservado
per política P158).

### Decisão 6 — Stdlib funcs Opção α (4 funcs novas)

4 stdlib funcs novas em `rules/stdlib/state.rs` (módulo
novo):

```rust
// rules/stdlib/state.rs (módulo novo):

pub(super) fn native_state(args: Args) -> SourceResult<Value> {
    // state(key, init) -> Value::State.
    let key = extract_string(args.positional[0])?;
    let init = args.positional[1].clone();
    Ok(Value::State(State { key, init: Box::new(init) }))
}

pub(super) fn native_state_get(args: Args, layouter: &Layouter) -> SourceResult<Value> {
    // state.get(state) -> Value (valor corrente; init se ausente).
    let state = extract_state(args.positional[0])?;
    Ok(layouter.state_table.get(&state.key).cloned()
        .unwrap_or_else(|| (*state.init).clone()))
}

pub(super) fn native_state_update(args: Args) -> SourceResult<Value> {
    // state.update(state, value) -> Content::StateUpdate.
    let state = extract_state(args.positional[0])?;
    let value = args.positional[1].clone();
    Ok(Value::Content(Content::StateUpdate {
        key: state.key.clone(),
        value: Box::new(value),
    }))
}

pub(super) fn native_state_final(args: Args, layouter: &Layouter) -> SourceResult<Value> {
    // state.final(state) -> Value adiada graded.
    // Pattern N=8 → 9: armazenado semantic adiada.
    // P236 returns valor corrente (≠ vanilla final semantic).
    // Refino futuro D.2 candidato two-pass walk real.
    let state = extract_state(args.positional[0])?;
    Ok(layouter.state_table.get(&state.key).cloned()
        .unwrap_or_else(|| (*state.init).clone()))
}
```

Stdlib funcs: 60 → **64** (+4 funcs novas).

**Pattern emergente "Módulo stdlib novo para feature runtime
arquitectural" N=1 inaugurado P236** — distinto de
`stdlib/layout.rs` + `stdlib/structural.rs` baseline.

### Decisão 7 — ADR-0066 PROPOSTO → IMPLEMENTADO Opção α

3 opções:

| Opção | Mecânica | Trade-off |
|-------|----------|-----------|
| **α** | Promover ADR-0066 PROPOSTO → IMPLEMENTADO em P236 (integrada com materialização) | Paridade ADR-0078 P221; subset minimal suficiente para "implementado" |
| β | Materializar D.1 + promover P237 administrativo XS separado | Atomização excessiva (ambos são coerentes) |
| γ | Manter ADR-0066 PROPOSTO permanente | Viola intent "desbloquear ADR-0066 via D.1" P226 |

**Decisão fixada — Opção α** (promoção integrada):
- Paridade pattern P221 (materialização Fase 3 + promoção
  ADR-0078 PROPOSTO → IMPLEMENTADO).
- Subset minimal de state suficiente para semantic
  "implementado" (`state.final` adiada graded é refino
  futuro candidato; não impede "implementado").

**Pattern "Promoção ADR PROPOSTO → IMPLEMENTADO integrada
em sub-passo materialização" N=1 → 2 cumulativo** (P221
ADR-0078; **P236 ADR-0066**).

### Decisão 8 — L0 TOCADO partial (primeira excepção justificada à aplicação automática ADR-0080 EM VIGOR pós-P229)

**Decisão crítica fixada — Opção α (L0 tocado partial)**:

ADR-0080 EM VIGOR §"Escopo" (audit C1 obrigatório) provavelmente
limita-se a "refactors aditivos a variants/fields existentes".
P236 adiciona:
- `Value::State` variant novo (Value enum expansão).
- `Content::StateUpdate` variant novo.
- 4 stdlib funcs novas (módulo `stdlib/state.rs` novo).
- Entity `State` struct nova (módulo `entities/state.rs`
  novo).
- Layouter +1 field HashMap.

**Não-trivial estructural**; L0 deve documentar feature
runtime:
- `entities/value.md` — `State` variant novo documented.
- `rules/stdlib.md` — 4 funcs novas documented.
- `entities/state.md` — **novo ficheiro L0** (possível;
  audit C1 confirma se necessário; alternativa documentar
  inline em `value.md`).

**Pattern emergente "L0 tocado para features runtime novas
(não aplicação automática ADR-0080)" N=1 inaugurado P236**
— **primeira excepção justificada à aplicação automática
pós-P229**.

**Pattern "aplicação automática ADR-0080 EM VIGOR" N=6
baseline preservado** mas **não-incrementa em P236**
(N=6 → 6 estável; P236 é excepção justificada).

ADR-0080 §"Escopo" deve documentar excepção: refactors
aditivos a variants existentes ≠ features runtime novas;
L0 tocado para features novas é norma metodológica.

Reuso de dados (sem recolha nova):

- `Value` enum baseline 55 variants.
- `Content` enum baseline 59 variants.
- `Layouter` struct baseline em `layout/mod.rs`.
- Locator P139+P140 baseline (precedência conceitual).
- Pattern "Field armazenado semantic adiada" N=8 baseline.
- Pattern "Value variant novo para tipo composto" N=1
  baseline P227.
- Pattern "Promoção ADR PROPOSTO → IMPLEMENTADO integrada"
  N=1 baseline P221.
- Pattern "aplicação automática ADR-0080 EM VIGOR" N=6
  baseline P230-P235.
- **ADR-0066 PROPOSTO** baseline P226 (audit C1).
- ADR-0079 PROPOSTO Categoria A 5/5 + Categoria B 3/3
  baseline P235.

---

## §2 Cláusulas (12 — atomização paridade P235)

### C1 — Auditoria pré-P236 CRÍTICA: ler ADR-0066 + ADR-0080 §"Escopo" + Layouter + Value/Content variants count

Audit obrigatório:

```
grep -A 50 "ADR-0066\|attribute-grammar\|state runtime" 00_nucleo/adr/typst-adr-0066-*.md
grep -A 30 "§\"Escopo\"\|aditivos\|L0" 00_nucleo/adr/typst-adr-0080-*.md
grep -B 2 -A 30 "pub struct Layouter" 01_core/src/rules/layout/mod.rs
grep -n "pub enum Value" 01_core/src/entities/value.rs
grep -n "pub enum Content" 01_core/src/entities/content.rs
ls 01_core/src/entities/ 01_core/src/rules/stdlib/
grep -n "Locator\|LocatorRegistry" 01_core/src/entities/ 01_core/src/rules/
```

Hipótese:
- ADR-0066 PROPOSTO documenta state runtime mutable scope.
- ADR-0080 §"Escopo" limita-se a refactors aditivos a
  variants existentes (não cobre features runtime novas).
- Layouter struct em `layout/mod.rs` linha ~160 com
  cell_origin_* baseline.
- Value enum baseline 55 variants em `value.rs`.
- Content enum baseline 59 variants em `content.rs`.
- `entities/` baseline (audit C1 lista ficheiros existentes).
- `rules/stdlib/` baseline (audit C1 lista ficheiros).
- Locator P139+P140 em `entities/locator.rs` ou similar.

**Decisões críticas C1**:
1. **ADR-0066 escopo completo**: confirma subset minimal
   4 operations é suficiente para "implementado" semantic.
   Se ADR-0066 fixa escopo mais amplo (e.g., requer
   state.at + state.display obrigatórios), P236 atomiza
   ou estende.
2. **ADR-0080 §"Escopo" excepção justificada**: confirma
   features runtime novas merecem L0 tocado; documentar
   excepção em ADR-0080 (anotar paridade P229 promoção).
3. **Locator pattern reuso**: state pode reusar Locator
   infraestrutura (HashMap interno) ou criar paralelo.
   Decisão pragmática: Layouter HashMap simples
   (Opção α Decisão 3).

Se ADR-0066 escopo diverge significativamente: registar
`P236.div-N` formal.

### C2 — Criar `entities/state.rs` + `Value::State(State)` variant

Editar `01_core/src/entities/value.rs`:

```rust
pub enum Value {
    // ... existing 55 variants ...
    /// P236 — feature runtime state(key, init).
    State(State),
}

// + impls From<State> for Value, type_name "state", etc.
```

Criar `01_core/src/entities/state.rs` (módulo novo):

```rust
//! P236 — Feature runtime state(key, init) per ADR-0066
//! IMPLEMENTADO. Subset minimal: state + state.get +
//! state.update + state.final (graded adiada).

use crate::entities::value::Value;

#[derive(Clone, Debug, PartialEq)]
pub struct State {
    pub key: String,
    pub init: Box<Value>,
}

impl State {
    pub fn new(key: String, init: Value) -> Self {
        Self { key, init: Box::new(init) }
    }
}
```

Adicionar `pub mod state;` em `entities/mod.rs`.

Value variants: 55 → **56**.

### C3 — Adicionar `Content::StateUpdate` variant

Editar `01_core/src/entities/content.rs`:

```rust
pub enum Content {
    // ... existing 59 variants ...
    /// P236 — walk-time mutation: aplica `value` à `key`
    /// no Layouter.state_table.
    StateUpdate {
        key: String,
        value: Box<Value>,
    },
}
```

Content variants: 59 → **60**.

### C4 — Arms cascata exhaustivos (compiler-driven)

Total arms refino P236:

**`entities/value.rs`** Value impls:
- `Display` arm State.
- `type_name` arm State.
- `Clone`/`PartialEq` derive auto.

**`entities/content.rs`** Content arms:
- `is_empty` arm StateUpdate (semantic vacío? trivialmente
  no porque StateUpdate é side-effect; preservar layout
  behaviour).
- `plain_text` arm StateUpdate (sem texto).
- `PartialEq::eq` arm StateUpdate.
- `map_content` arm StateUpdate (preserva).
- `map_text` arm StateUpdate (preserva).

**`rules/introspect.rs`**:
- `materialize_time` arm StateUpdate (apply mutation
  during walk).
- `walk` arm StateUpdate.

**`rules/layout/mod.rs`**:
- `layout_content` arm StateUpdate (apply mutation no
  Layouter.state_table; sem render output).

**`rules/introspect/locatable.rs`** (catch-all preserva).

Total: **~12-15 arms** cumulativos. Compiler-driven.

### C5 — Criar `rules/stdlib/state.rs` (módulo novo) + 4 funcs

Criar `01_core/src/rules/stdlib/state.rs` (módulo novo):

```rust
//! P236 — Stdlib funcs state(key, init) + state.get +
//! state.update + state.final per ADR-0066 IMPLEMENTADO.
//! Subset minimal funcional (4 operations); state.at +
//! state.display refinos futuros candidatos D.2+.

use crate::entities::{Value, State, Content};
use crate::rules::layout::Layouter;

pub(super) fn native_state(args: Args) -> SourceResult<Value> { ... }
pub(super) fn native_state_get(args: Args, layouter: &Layouter)
    -> SourceResult<Value> { ... }
pub(super) fn native_state_update(args: Args) -> SourceResult<Value> { ... }
pub(super) fn native_state_final(args: Args, layouter: &Layouter)
    -> SourceResult<Value> { ... }
```

Adicionar `pub mod state;` em `rules/stdlib/mod.rs`.

Registo scope: 4 funcs registadas paridade pattern P227
`native_stroke` registo.

Stdlib funcs: 60 → **64** (+4 novas).

Magnitude C5: **M (~1h)** — 4 funcs + módulo novo + scope
register.

### C6 — Layouter +1 field state_table

Editar `01_core/src/rules/layout/mod.rs`:

```rust
use std::collections::HashMap;

pub struct Layouter<'a> {
    // ... existing ...
    pub state_table: HashMap<String, Value>,  // P236
}

impl<'a> Default for Layouter<'a> {
    fn default() -> Self {
        Self {
            // ... existing defaults ...
            state_table: HashMap::new(),  // P236
        }
    }
}
```

Magnitude C6: **XS (~10min)**.

### C7 — Renderização state.get/update walk integration

Editar `01_core/src/rules/layout/mod.rs::layout_content`:

```rust
Content::StateUpdate { key, value } => {
    // P236 — walk-time mutation: actualiza state_table.
    self.state_table.insert(key.clone(), *value.clone());
    // Sem render output (StateUpdate é puro side-effect).
}
```

`state.get(state)` lê `state_table` via parâmetro
`&Layouter` passed à stdlib func (audit C1 confirma
signature).

Magnitude C7: **S (~30min)**.

### C8 — Sentinelas P236

Tests P236 (~15-20 tests):

**Unit entities** (~3 tests):
- `p236_value_state_variant_aceita_state`.
- `p236_state_struct_constructor`.
- `p236_content_stateupdate_variant_aceita`.

**Unit stdlib** (~6 tests):
- `p236_native_state_constructor_aceita_key_init`.
- `p236_native_state_get_retorna_init_se_ausente`.
- `p236_native_state_get_retorna_valor_actualizado`.
- `p236_native_state_update_retorna_content_stateupdate`.
- `p236_native_state_final_adiada_retorna_corrente_graded`.
- `p236_native_state_4_funcs_simultaneas`.

**Layout E2E** (~6-8 tests crítico):
- `p236_state_init_chamada_walk_table_vacio_retorna_init`.
- `p236_state_update_aplicado_walk_actualiza_table`.
- `p236_state_get_pós_update_retorna_valor_actualizado`.
- `p236_state_final_adiada_returna_corrente`.
- `p236_state_multiple_keys_independentes`.
- `p236_state_update_walk_integration_no_renderize_output`.
- `p236_state_via_introspection_walk_completo`.
- `p236_state_baseline_layout_preservado`.

Total tests P236: **~15-17 tests**. Esperado pós-P236:
**2137 + 17 = ~2154 verdes** (paridade hipótese; ajuste
pós-implementação).

### C9 — L0 TOCADO partial (excepção justificada ADR-0080)

**Decisão fixada — L0 partial tocado**:

Editar L0 prompts:
- `00_nucleo/prompts/entities/value.md` — adicionar
  `State` variant doc + struct nova.
- `00_nucleo/prompts/rules/stdlib.md` — adicionar 4
  state funcs doc.
- **Possivelmente novo**: `00_nucleo/prompts/entities/state.md`
  — entity State documentation (audit C1 confirma se
  necessário; alternativa documentar inline em
  `value.md`).

ADR-0080 §"Escopo" anotada com excepção P236:

```markdown
## Excepção P236 — features runtime novas

ADR-0080 EM VIGOR aplica-se a refactors aditivos a
variants/fields existentes. **Features runtime novas
(novos Value variants + novas stdlib funcs + nova entity)
merecem L0 tocado**. P236 inaugura precedente.

Pattern "L0 tocado para features runtime novas (não
aplicação automática ADR-0080)" N=1 inaugurado P236.
```

**Pattern "aplicação automática ADR-0080 EM VIGOR" N=6
preservado** (P230-P235 baseline) mas **não-incrementa
em P236**.

Magnitude C9: **S (~30min)** — L0 partial tocado +
ADR-0080 anotação.

### C10 — Verificação tests workspace + lint

```
cargo test --workspace 2>&1 | tail -3
crystalline-lint .
crystalline-lint --fix-hashes .
```

Critério:
- 2137 verdes pré-P236 + ~15-17 novos = **~2154 verdes**.
- 0 violations preservadas.
- Hashes propagados em ~6-8 ficheiros L1.
- **L0 prompts TOCADOS partial** — hashes para 2-3
  ficheiros L0 actualizados; "Nothing to fix" para o resto.

**Risco regressão**: tests baseline pre-P236. Hipótese
N=0-2 adaptações (Value::State novo variant não-conflita
com baseline; Content::StateUpdate idem).

### C11 — ADR-0066 PROPOSTO → IMPLEMENTADO + Inventário 148 footnote ⁵⁵ + ADR-0079 anotação Categoria D 1/?

**ADR-0066**:
- Status `PROPOSTO` → **`IMPLEMENTADO`** (P236 promoção
  integrada).
- Bloco `## Implementado P236 (2026-05-13)` adicionado:
  - 4 operations subset minimal funcional.
  - state.final adiada graded (Pattern N=8 → 9; refino
    futuro D.2 candidato).
  - 4 stdlib funcs novas em `rules/stdlib/state.rs`
    (módulo novo).
  - Layouter +1 field HashMap state_table.
  - ~15-17 tests novos verdes.
  - L0 tocado partial (~2-3 ficheiros).

**Inventário 148**:
- §A.X Introspection: cobertura **+33pp** documentada.
- Footnote ⁵⁵ adicionada (~150 linhas) documentando D.1
  materializado + 8 decisões + ADR-0066 IMPLEMENTADO +
  L0 tocado partial (excepção justificada) + patterns
  emergentes inaugurados/consolidados.

**ADR-0079**:
- §"Aplicações cumulativas" anotado com bloco
  `### P236 anotação — Categoria D sub-passo 1 (state
  runtime mutable); ADR-0066 PROPOSTO → IMPLEMENTADO via
  D.1; +33pp Introspection`.
- Status ADR-0079 mantido PROPOSTO (9/13-15 sub-passos
  cumulativos; **Categoria A 5/5 ✓ + Categoria B 3/3 ✓
  + Categoria D 1/? + Categoria C 0/? pendentes**).

### C12 — Critério aceitação P236

- ~15-17 tests novos verdes.
- 2137 tests pre-existentes preservados (após N=0-2
  adaptações).
- 0 violations.
- Value variants: 55 → **56** (+State).
- Content variants: 59 → **60** (+StateUpdate).
- Stdlib funcs: 60 → **64** (+4 state funcs).
- Layouter +1 field HashMap state_table.
- 2 módulos novos (`entities/state.rs` + `rules/stdlib/state.rs`).
- **ADR-0066 PROPOSTO → IMPLEMENTADO** anotado.
- **L0 tocado partial** (~2-3 ficheiros).
- ADR-0079 Categoria D 1/? anotado.
- ADR-0080 §"Escopo" anotada com excepção P236.
- Cobertura Introspection **+33pp**.
- Cobertura Layout 89% preservada (refino qualitativo —
  D.1 é Introspection, não Layout).

---

## §3 Output

1 ficheiro relatório curto:
`00_nucleo/materialization/typst-passo-236-relatorio.md`.

Estrutura (~7-9 KB) com 8 §s:

- §1 O que foi feito (sumário 3-5 linhas).
- §2 Auditoria pré-P236 + ADR-0066 escopo + ADR-0080
  excepção (C1).
- §3 Value::State + entity State + Content::StateUpdate
  variants novos (C2+C3).
- §4 4 stdlib funcs novas em `rules/stdlib/state.rs`
  módulo novo (C5).
- §5 Layouter +1 field HashMap + walk integration
  StateUpdate (C6+C7).
- §6 Decisões substantivas (8 decisões fixadas) +
  **primeira excepção justificada à aplicação automática
  ADR-0080 EM VIGOR**.
- §7 **ADR-0066 PROPOSTO → IMPLEMENTADO** + L0 tocado
  partial + inventário 148 footnote ⁵⁵ + ADR-0079
  Categoria D 1/? (C9+C11).
- §8 Próximo sub-passo (P237 candidatos: D.2 state.at/
  display/two-pass walk; D.3 query; D.4 counter; C.1/C.2;
  ADR meta admin; pivot).

Código alterado:
- **Criado**: `01_core/src/entities/state.rs` (módulo
  novo; State struct + impls).
- **Editado**: `01_core/src/entities/mod.rs` (+ `pub mod
  state;`).
- **Editado**: `01_core/src/entities/value.rs` (+ Value::State
  variant + impls + ~3 unit tests).
- **Editado**: `01_core/src/entities/content.rs` (+
  Content::StateUpdate variant + arms cascata + ~2 unit
  tests).
- **Criado**: `01_core/src/rules/stdlib/state.rs` (módulo
  novo; 4 funcs + ~6 unit tests).
- **Editado**: `01_core/src/rules/stdlib/mod.rs` (+ `pub
  mod state;`).
- **Editado**: `01_core/src/rules/layout/mod.rs` (+
  state_table HashMap + walk integration StateUpdate).
- **Editado**: `01_core/src/rules/introspect.rs` (arms
  StateUpdate).
- **Editado**: `01_core/src/rules/layout/tests.rs` (+~6-8
  E2E tests).
- **L0 EDITADO partial**: `00_nucleo/prompts/entities/value.md`
  + `00_nucleo/prompts/rules/stdlib.md` + possível
  `entities/state.md` novo.
- **Editado**: `00_nucleo/adr/typst-adr-0066-*.md` (status
  PROPOSTO → IMPLEMENTADO; bloco P236 implementado).
- **Editado**: `00_nucleo/adr/typst-adr-0080-*.md` (§"Escopo"
  anotada com excepção P236).
- **Editado**: `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`
  (footnote ⁵⁵ P236 + cobertura Introspection +33pp).
- **Editado**: `00_nucleo/adr/typst-adr-0079-layout-fase-5-roadmap.md`
  (+ anotação Categoria D 1/? P236).

**2 novos ficheiros**: `entities/state.rs` + `rules/stdlib/state.rs`.
Possível 3º novo ficheiro L0: `entities/state.md`.

---

## §4 Não-objectivos

- `state.at(location)` — refino D.2+ candidato (não-bloqueador
  IMPLEMENTADO).
- `state.display(...)` — refino D.2+ candidato.
- Two-pass walk real para `state.final()` — sub-passo D.2
  candidato (não-reservado).
- `query(target)` runtime — sub-passo D.3 candidato
  separado.
- `counter(key)` runtime — sub-passo D.4 candidato
  separado.
- Promover ADR-0079 PROPOSTO → IMPLEMENTADO — só pós
  Categorias A + B + C + D completas.
- Reabrir decisões arquiteturais Categoria A/B — D.1 é
  Categoria D distinta.
- Show rules `#show state: ...` — fora escopo Fase 5.
- Promoção formal patterns `.or()` N=3 / refino paralelo
  N=5 / Smart→Option N=12 / semantic adiada N=9 / etc. —
  passos administrativos XS separados candidatos.
- Refactor Locator P139+P140 para state — Layouter HashMap
  baseline suficiente.
- ADR-0080 §"Escopo" reescrita completa — apenas anotação
  excepção P236.
- L0 tocado completo para todos sub-passos D — apenas
  ficheiros directly afectados por P236 (value.md + stdlib.md
  + possível state.md).
- Marco cirúrgico blueprint para promoção ADR-0066
  IMPLEMENTADO — paridade pattern P221 (sem marco
  cirúrgico para promoção integrada).

---

## §5 Riscos a evitar

1. **ADR-0066 escopo diverge significativamente da
   hipótese**: audit C1 crítico. Mitigação: registar
   `P236.div-N` formal se escopo amplo; atomização
   ADR-0036 se necessário.
2. **ADR-0080 §"Escopo" cobre features runtime**: se
   ADR-0080 já documenta L0 tocado opcional, P236 não
   é "excepção" mas "aplicação esperada". Mitigação: audit
   C1 confirma; anotação adapta-se a estado real.
3. **state.update side-effect quebra walk determinístico**:
   ordem de walk afecta state final. Mitigação: documentar
   semantic walk-order paridade vanilla.
4. **Tests baseline com Layouter default**: hipótese N=0-2
   adaptações (Layouter `state_table: HashMap::new()` é
   default-additivo; preserva tests).
5. **L0 tocado parcial confunde hashes "Nothing to fix"**:
   2-3 ficheiros L0 actualizados; resto preservado.
   Mitigação: `crystalline-lint --fix-hashes` actualiza
   só ficheiros tocados.
6. **`state.final()` graded retorna valor corrente
   confunde utilizador**: semantic adiada. Mitigação:
   error message ou comment doc claro "state.final é
   refino futuro D.2; retorna valor corrente subset
   minimal P236".
7. **Magnitude exceder M+ (~3-4h)**: feature runtime
   maior. Hipótese real M+ (~3.5h). Se exceder L,
   atomização P236A (Value::State + entity) + P236B
   (stdlib funcs + walk integration).
8. **Promoção ADR-0066 IMPLEMENTADO prematura**: subset
   minimal pode não cobrir escopo ADR-0066 completo.
   Mitigação: audit C1 confirma; se ADR-0066 requer mais,
   manter PROPOSTO e P237+ continuam.
9. **Pattern "aplicação automática ADR EM VIGOR" N=6
   quebra-se incorrectamente**: P236 é excepção, não-quebra.
   Mitigação: documentar excepção justificada;
   pattern preservado N=6.
10. **Helper `extract_state` ausente**: parsing inline
    `Value::State(s) => Ok(s.clone())` trivial.
11. **HashMap performance**: state_table cresce com
    keys; para subset minimal HashMap simples suficiente.
    Optimização refino futuro.
12. **Locator dependency**: state pode precisar Locator
    para walk tracking. Mitigação: Layouter HashMap
    independente; Locator não-tocado P236.

---

## §6 Hipótese provável

C1 (audit obrigatório) confirmará ADR-0066 PROPOSTO
documenta runtime queries scope; subset minimal 4 operations
suficiente para "implementado"; ADR-0080 §"Escopo" cobre
refactors aditivos a variants existentes (excepção P236
justificada); Layouter struct em `layout/mod.rs` linha
~160; Value enum 55 variants; Content enum 59 variants;
entities/ + stdlib/ baseline confirmado.

C2 criará entity State + Value::State variant.

C3 adicionará Content::StateUpdate variant.

C4 cobrirá ~12-15 arms cumulativos.

C5 criará 4 stdlib funcs novas em módulo `stdlib/state.rs`.

C6 adicionará Layouter +1 field HashMap.

C7 implementará walk integration StateUpdate.

C8 criará ~15-17 tests novos.

C9 tocará L0 partial (2-3 ficheiros).

C10 reportará ~2154 verdes; 0 violations.

C11 promoverá ADR-0066 IMPLEMENTADO + footnote ⁵⁵ +
ADR-0079 Categoria D 1/?.

C12 verifica critério aceitação.

Custo real: **M+ (~3.5h)** — feature runtime maior +
módulos novos + L0 tocado.

Mas é hipótese, não decisão. C1-C12 fixam-se empíricamente.
Possível `P236.div-N` se ADR-0066 escopo diverge.

---

## §7 Particularidade P236

P236 é estruturalmente distinto **muito significativo**
na trajectória pós-M9c:

- **Nono sub-passo materialização Fase 5 Layout candidata**
  — primeiro Categoria D runtime queries pós-fecho
  Categoria A + B (cosmético + algorítmico).
- **Primeira feature runtime aditiva pós-Fase 5
  cosmética/algorítmica** — distinto cumulativo de A.1-A.5
  + B.1-B.3. Categoria D abre runtime arquitecturalmente
  distinta.
- **Promoção ADR-0066 PROPOSTO → IMPLEMENTADO integrada**
  — paridade ADR-0078 P221. Pattern N=1 → 2 cumulativo.
- **Primeira excepção justificada à aplicação automática
  ADR-0080 EM VIGOR pós-promoção P229** — L0 tocado
  partial. Pattern "aplicação automática" N=6 baseline
  preservado mas **não-incrementa P236** (excepção justificada).
- **2 módulos novos**: `entities/state.rs` + `rules/stdlib/state.rs`.
  Distinto cumulativo de A.1-A.5 + B.1-B.3 que não
  criaram módulos novos.
- **+1 Value variant + +1 Content variant + 4 stdlib funcs
  + +1 Layouter field** — escopo aditivo mais amplo que
  qualquer sub-passo Fase 5 anterior.
- **Pattern "L0 tocado para features runtime novas" N=1
  inaugurado P236** — metodológico crítico para sub-passos
  D.2+ e C.1+ runtime/estructurais.
- **Pattern "Promoção ADR PROPOSTO → IMPLEMENTADO integrada
  em sub-passo materialização" N=1 → 2 cumulativo** (P221
  ADR-0078; **P236 ADR-0066**).
- **Pattern "Value variant novo para tipo composto
  stdlib-construído" N=1 → 2 cumulativo** (P227 Stroke;
  **P236 State**).
- **Pattern "Field/operation armazenado semantic adiada"
  N=8 → 9 cumulativo** (+state.final P236).
- **Pattern emergente "subset minimal aditivo: 4 operations
  primário; refinos futuros não-bloqueadores" N=1 inaugurado
  P236**.
- **Pattern emergente "Layouter +1 field HashMap para
  feature runtime" N=1 inaugurado P236**.
- **Pattern emergente "Módulo stdlib novo para feature
  runtime arquitectural" N=1 inaugurado P236**.
- **+33pp Introspection** — cobertura módulo Introspection
  actualizada (single sub-passo gera maior incremento
  cobertura pós-M9c).
- **Cobertura Layout per metodologia preservada 89%**
  — D.1 é Introspection, não Layout.
- **Anti-inflação 28ª aplicação cumulativa** pós-P205D —
  Opção α subset minimal + Opção β Value::State variant
  + Opção α Layouter HashMap simples + Opção γ
  Content::StateUpdate + Opção β state.final graded +
  Opção α 4 stdlib funcs subset + Opção α ADR-0066
  promoção integrada + Opção α L0 tocado partial.

Por isso §5 risco 1 (ADR-0066 escopo amplo) é o mais
provável. Mitigação: audit C1 obrigatório imediato; se
ADR-0066 requer escopo maior (state.at + state.display
mandatórios para "implementado"), P236 atomiza imediato
ou mantém PROPOSTO.

**Critério de aceitação P236**:
- ~15-17 tests novos verdes.
- 2137 tests pre-existentes preservados.
- 0 violations.
- +1 Value variant (State).
- +1 Content variant (StateUpdate).
- +4 stdlib funcs (state + state.get + state.update +
  state.final).
- +1 Layouter field (state_table HashMap).
- 2 módulos novos criados.
- **ADR-0066 PROPOSTO → IMPLEMENTADO** ✓.
- **L0 tocado partial** (~2-3 ficheiros).
- ADR-0080 §"Escopo" anotada com excepção P236.
- ADR-0079 Categoria D 1/? anotado.
- Cobertura Introspection **+33pp**.

**Estado pós-P236 esperado**:
- Tests workspace: 2137 → **~2154 verdes** (+15-17).
- **Stdlib funcs: 60 → 64** (+4 state).
- **Content variants: 59 → 60** (+StateUpdate).
- **Value variants: 55 → 56** (+State).
- GridCell/TableCell/Block/Boxed/Place fields preservados.
- Layouter fields: n+1 → **n+2** (+state_table).
- §A.5 distribuição: `12/4/2/0/0 = 18` preservada (Layout
  Fase 5).
- **Cobertura Introspection: +33pp**.
- Cobertura Layout per metodologia: **89% preservada**.
- Cobertura user-facing total: 67% → **~70-72%** (+33pp
  Introspection cumulativo).
- **ADR-0066 IMPLEMENTADO** ✓ (PROPOSTO → IMPLEMENTADO).
- ADR-0061 IMPLEMENTADO; ADR-0078 IMPLEMENTADO; ADR-0079
  PROPOSTO (9/13-15; Categoria A 5/5 ✓ + Categoria B 3/3 ✓
  + Categoria D 1/? + Categoria C 0/?); ADR-0080 EM VIGOR
  (anotada §"Escopo" com excepção P236).
- **Distribuição ADRs**: PROPOSTO **11** (-1: ADR-0066
  transita); EM VIGOR 29; IMPLEMENTADO **22** (+1:
  ADR-0066); total 67 preservado.
- Saldo DEBTs: 11 preservado.
- **28 aplicações cumulativas anti-inflação** pós-P205D.
- **Pattern "L0 minimal para refactors" aplicação
  automática N=6 cumulativo preservado** (P230-P235) mas
  **NÃO incrementa P236** (excepção justificada).
- **Pattern "L0 tocado para features runtime novas" N=1
  inaugurado P236** — primeira excepção pós-P229.
- **Pattern "Promoção ADR PROPOSTO → IMPLEMENTADO integrada
  em sub-passo materialização" N=1 → 2 cumulativo**.
- **Pattern "Value variant novo para tipo composto" N=1
  → 2 cumulativo** (P227 Stroke; P236 State).
- **Pattern "Field/operation armazenado semantic adiada"
  N=8 → 9 cumulativo** (+state.final P236).
- **Patterns inaugurados P236** (3):
  - "Subset minimal aditivo: 4 operations primário; refinos
    futuros não-bloqueadores" N=1.
  - "Layouter +1 field HashMap para feature runtime" N=1.
  - "Módulo stdlib novo para feature runtime arquitectural"
    N=1.
- **Fase 5 Layout candidata: 9/13-15 sub-passos
  materializados** (~60-69% cumulativo; **Categoria A
  100% + Categoria B 100% + Categoria D 1/?**).
- **Marco interno: ADR-0066 IMPLEMENTADO via D.1 +33pp
  Introspection** — primeira transição arquitectural
  maior pós-Fase 5 Categoria B fechada.
