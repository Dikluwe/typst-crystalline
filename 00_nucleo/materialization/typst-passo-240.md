# Passo 240 — M7+1 Pipeline walk-time eval via Opção γ `apply_state_displays` (M9d primeira materialização sub-passo; desbloqueia D.2 state.display real + state.final two-pass real via sobreposição bloqueadores A+D; **primeira excepção justificada à aplicação automática ADR-0080 EM VIGOR pós-P229**)

**Série**: 240 (vigésimo-sexto sub-passo Layout pós-M9c;
**décimo-segundo sub-passo materialização Fase 5 Layout
candidata** per ADR-0079 PROPOSTO; **primeira materialização
sub-passo M9d / M7+** per ADR-0081 PROPOSTO P239;
**primeira aplicação real do pattern atomização prep-passo
audit-only + materialização-passo inaugurado P238 reescrito
N=2 cumulativo**; **terceira aplicação cumulativa pattern
"spec C1 audit obrigatório bloqueante pós-P236.div-1"**
N=2 → 3 cumulativo).
**Marco**: **primeira sub-passo materialização M9d/M7+
pós-P239 audit-only**; **primeira excepção justificada à
aplicação automática ADR-0080 EM VIGOR pós-promoção P229**
(L0 partial tocado para feature runtime + walk integration);
pattern emergente "feature runtime + walk integration
requer L0 partial tocado" N=1 inaugurado P240 (paridade
hipótese P236 spec original rejeitada empíricamente
pós-divergência); **desbloqueia D.2 state.display real
+ state.final two-pass real** via sobreposição grande
bloqueadores A+D identificada P239 audit C4; pattern
"refino aditivo paralelo entre callers fixpoint"
(`apply_state_funcs` baseline → `apply_state_displays`)
N=1 inaugurado P240; **possível promoção ADR-0079
Categoria D 1/? → 2/? pós-M7+1** (sub-passo D.2 fica
materializado real walk-time).
**Tipo**: feature runtime nova + walk integration via
**pattern reusado existente** (`apply_state_funcs` baseline
P171/M9); +1 Content variant (`Content::StateDisplay`);
+1 Tag variant (`Tag::StateDisplay`); +1 stdlib func
(`native_state_display`); +1 fixpoint function
(`apply_state_displays`); +1 Introspector storage method
(`state_display_value`); **L0 partial tocado** (3 ficheiros);
refino state.final two-pass real se trivial (sujeito audit
C1; atomização M7+1A + M7+1B candidato se não-trivial).
**Magnitude**: L (~5-8h; paridade P239 audit estimate;
maior que P236+P237 eval-time wrappers + maior que P238
reescrito documental + paridade hipótese P236 spec original).
**Pré-condição**: P239 prep-passo audit-only concluído
(ADR-0081 PROPOSTO criado; nomenclatura M9d preliminar;
roadmap atomização 5 sub-passos M7+1 a M7+5 identificado;
3 pré-condições obrigatórias formalizadas; 2150 verdes
baseline preservadas); humano fixou M7+1 (decisão literal
pós-P239 §7); `apply_state_funcs` baseline P171/M9 em
`from_tags.rs:48`; `fixpoint::run_fixpoint` caller em
`fixpoint.rs:101`; `apply_func(func, args, ctx, engine)`
em `closures.rs:59`; `Content::State` + `Content::StateUpdate`
baseline preservados; `Func` + `Engine` + `EvalContext`
interfaces estáveis; ADR-0066 SUPERSEDED-BY 0073 terminal
preservado; lição refinada P236.div-1 → P238.div-1
aplicada literal (P237 + P238 reescrito + P239 cumulativo
validado).
**Output**: 1 ficheiro relatório curto + código alterado
em ~7-10 ficheiros L1 (`entities/content.rs`, `entities/tag.rs`
ou similar, `rules/stdlib/foundations.rs`, `rules/eval/mod.rs`,
`rules/introspect/from_tags.rs`, `rules/introspect/fixpoint.rs`,
`entities/introspector.rs`, `rules/layout/mod.rs`,
possíveis outros) + **L0 partial TOCADO** (3 ficheiros:
`entities/content.md`, `rules/stdlib.md`, `rules/introspect.md`
ou similar) + inventário 148 anotação cumulativa (footnote
⁵⁹) + ADR-0081 status PROPOSTO → **IMPLEMENTADO parcial**
(M7+1 materializado; M7+2 a M7+5 pendentes) + ADR-0079
anotação **Categoria D 1/? → 2/? sub-passos materializados**.

---

## §1 Trabalho

P239 prep-passo audit-only identificou **Opção γ** como
resolução estructural arquitecturalmente correcta para
bloqueador A (walk-time eval Func dispatch): **pre-evaluate
display callbacks em fixpoint pré-layout** via novo
`apply_state_displays` paralelo absoluto a `apply_state_funcs`
baseline P171/M9. Layout permanece puro (sem Engine+ctx
em signature); paridade arquitectural estrita preservada;
comemo invariants ADR-0073/0074 preservados.

P239 audit C4 identificou **sobreposição grande bloqueador
A + bloqueador D** (state.final two-pass walk): mesmo
refactor desbloqueia ambos. M7+1 materializa Opção γ
incluindo refino state.final two-pass real se trivial.

**P240 (M7+1) materializa**:
- **`Content::StateDisplay { key, callback }`** variant
  novo paralelo `Content::StateUpdate` baseline P171/P172.
- **`Tag::StateDisplay { loc, key, callback }`** variant
  novo paralelo `Tag::StateUpdate` baseline.
- **`apply_state_displays`** função nova em `from_tags.rs`
  paralelo absoluto `apply_state_funcs` baseline; chama
  `apply_func(callback, vec![value], ctx, engine)` pós-fixpoint
  com Engine+ctx disponíveis.
- **`Introspector.state_displays: HashMap<(String,
  Location), Content>`** storage pre-rendered.
- **`Introspector::state_display_value(key, loc)`** lookup
  method.
- **`native_state_display(key, callback?)`** stdlib func
  nova; construtor para Content::StateDisplay.
- **Walk integration**: arm `Content::StateDisplay` em
  layout consulta `state_display_value(key, current_loc)`;
  renderiza pre-rendered Content.
- **Refino `state_final` two-pass real** se trivial
  (sujeito audit C1; atomização M7+1A + M7+1B candidato
  se não-trivial).
- **L0 partial tocado** (3 ficheiros): primeira excepção
  justificada à aplicação automática ADR-0080 EM VIGOR
  pós-P229.

**Decisão arquitectural central — 8 decisões fixadas**:

### Decisão 0 — C1 audit obrigatório bloqueante (lição P236.div-1 → P238.div-1 N=2 → 3 cumulativo)

**Pattern "spec C1 audit obrigatório bloqueante pós-P236.div-1"
N=2 → 3 cumulativo** (P237 + P238 reescrito + **P240**).

Audit C1 confirma:
- `apply_state_funcs` signature + caller intacto pós-P239.
- `Content::State` + `Content::StateUpdate` baseline
  preservados.
- `Func` + `apply_func` + `Engine + EvalContext` interfaces
  preservadas.
- `fixpoint::run_fixpoint` integration point preservado.
- `Introspector::state_value(key, location)` baseline
  P171 preservado.
- 2150 verdes preservados pré-P240.

Se audit revela divergência: `P240.div-N` formal +
questionário humano paridade P236.div-1 + P238.div-1.

### Decisão 1 — Escopo M7+1 Opção γ apply_state_displays

3 opções confirmadas P239 audit C2:

| Opção | Mecânica | Magnitude | Risco |
|-------|----------|-----------|-------|
| α | Pass Engine+ctx para Layouter signature massivo | L+ | Comemo invariants risco |
| β | Two-pass walk explícito | XL+ | Refactor cumulativo |
| **γ** | **Pre-evaluate em fixpoint paralelo apply_state_funcs** | **L (~5-8h)** | **Paridade pattern; baixo risco** |
| δ | Show rule mecanismo synthetic | L+ | Refactor Show rules |

**Decisão fixada — Opção γ** confirmada por P239 audit.
Pattern emergente "refino aditivo paralelo entre callers
fixpoint" N=1 inaugurado P240 (extensão pattern P171/M9
`apply_state_funcs`).

### Decisão 2 — `Content::StateDisplay { key, callback }` variant novo Opção β

3 opções:

| Opção | Mecânica | Trade-off |
|-------|----------|-----------|
| α | Refino `Content::State` baseline +callback field | Viola coerência (State é init marker) |
| **β** | Novo variant `Content::StateDisplay { key, callback }` | Coerência clara; semantic distinto |
| γ | Sem variant; stdlib func retorna `Content::State` modificado | Insuficiente |

**Decisão fixada — Opção β**:

```rust
// entities/content.rs:
Content::StateDisplay {
    key: String,
    /// P240 — callback opcional renderiza Value→Content.
    /// None = renderiza Value::Content directo (paridade
    /// vanilla state.display() sem callback).
    callback: Option<Func>,
}
```

Content variants: 60 → **61** (+StateDisplay).

### Decisão 3 — Tag emit `Tag::StateDisplay` Opção α

3 opções:

| Opção | Mecânica | Trade-off |
|-------|----------|-----------|
| **α** | `Tag::StateDisplay { loc, key, callback }` variant novo paralelo Tag::StateUpdate | Pattern existente reusado |
| β | Reuso Tag::StateUpdate com discriminator | Sobrecarga semantic |
| γ | Sem tag; render direct durante layout | Viola arquitectura layout puro |

**Decisão fixada — Opção α**:

```rust
// entities/tag.rs ou equivalente:
pub enum Tag {
    // ... existing variants ...
    StateDisplay {
        loc: Location,
        key: String,
        callback: Option<Func>,
    },
}
```

**Audit C1 confirma**: localização exacta Tag enum +
existing variants pattern.

### Decisão 4 — Pre-render storage Opção β (apply_state_displays paralelo)

3 opções:

| Opção | Mecânica | Trade-off |
|-------|----------|-----------|
| α | `intr.state_displays: HashMap<(String, Location), Content>` direct storage | Storage explícito |
| **β** | Tag emit walk + `apply_state_displays` pós-fixpoint pre-renderiza + storage Introspector | Paridade absoluta apply_state_funcs |
| γ | Lazy evaluation durante layout | Requer Engine+ctx (violação) |

**Decisão fixada — Opção β** (paralelismo absoluto):

```rust
// rules/introspect/from_tags.rs:
pub fn apply_state_displays(
    tags: &[Tag],
    intr: &mut TagIntrospector,
    engine: &mut Engine<'_>,
    ctx: &mut EvalContext,
) {
    for tag in tags {
        if let Tag::Start(loc, info) = tag {
            if let ElementPayload::StateDisplay { key, callback } = &info.payload {
                let value = intr.state.value_at(key, *loc).cloned()
                    .unwrap_or(Value::None);
                let pre_rendered = match callback {
                    Some(func) => match apply_func(func.clone(), vec![value.clone()], ctx, engine) {
                        Ok(Value::Content(c)) => c,
                        Ok(other) => Content::from_value(&other),  // fallback
                        Err(_) => Content::None,  // graceful degradation
                    },
                    None => Content::from_value(&value),
                };
                intr.state_displays.insert((key.clone(), *loc), pre_rendered);
            }
        }
    }
}
```

**Caller**: `fixpoint::run_fixpoint` (paridade
`apply_state_funcs`):

```rust
// Em run_fixpoint after apply_state_funcs:
apply_state_funcs(&tags, intr, engine, ctx);
apply_state_displays(&tags, intr, engine, ctx);
```

### Decisão 5 — Walk integration layout-time

Layout arm:

```rust
Content::StateDisplay { key, callback: _ } => {
    let loc = self.current_location();
    let pre_rendered = self.introspector
        .state_display_value(key, loc)
        .cloned()
        .unwrap_or(Content::None);
    self.layout_content(&pre_rendered)?;
}
```

Layout permanece puro (sem Engine+ctx); resolução via
Introspector storage materializado em fixpoint.

### Decisão 6 — Stdlib func `native_state_display`

```rust
pub fn native_state_display(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn World,
    _current_file: FileId,
    _figure_numbering: Option<&str>,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Str(key)] => {
            // Sem callback (renderiza Value→Content directo).
            Ok(Value::Content(Content::StateDisplay {
                key: key.to_string(),
                callback: None,
            }))
        }
        [Value::Str(key), Value::Func(callback)] => {
            // Com callback.
            Ok(Value::Content(Content::StateDisplay {
                key: key.to_string(),
                callback: Some(callback.clone()),
            }))
        }
        [_, other] if !matches!(other, Value::Func(_)) => err(format!(
            "state_display() requer Func como segundo argumento (callback), recebeu {}",
            other.type_name()
        )),
        [other, ..] if !matches!(other, Value::Str(_)) => err(format!(
            "state_display() requer string como primeiro argumento (key), recebeu {}",
            other.type_name()
        )),
        _ => err(format!(
            "state_display() requer 1-2 argumentos (key, [callback]), recebeu {}",
            args.items.len()
        )),
    }
}
```

Stdlib funcs: 62 → **63** (+state_display).

### Decisão 7 — Refino state.final two-pass real (sobreposição bloqueador D; sujeito audit C1)

P239 audit C3.3 confirmou sobreposição grande bloqueador
A + D. P240 (M7+1) materializa Opção γ (`apply_state_funcs`
+ `apply_state_displays`) que já materializa state values
cumulativos pós-fixpoint convergência.

**Refino `state_final` necessário**:
- P236 baseline: `state_final` retorna `Introspector::state_final_value`
  (último update no introspector).
- **Pós-M7+1 (sujeito audit C1)**: confirmar se
  `state_final_value` baseline já retorna valor final
  cumulativo pós-fixpoint (i.e., two-pass real já implícito)
  OR requer refino explícito.

**3 cenários possíveis (audit C1 decide)**:

| Cenário | Refino state.final | Magnitude marginal |
|---------|-------------------|--------------------|
| α | Trivial (state_final_value já retorna pós-fixpoint) | XS (~10min docs) |
| **β** | Trivial-ajuste (refino docs + 1-2 tests semantic) | XS-S (~30min) |
| γ | Não-trivial (requer refactor introspector) | M (~2-3h) → **atomização M7+1A + M7+1B** |

**Decisão fixada (sujeita audit C1) — cenário β provável**:
refino state.final incluído em M7+1 se XS-S. Atomização
M7+1A (apply_state_displays) + M7+1B (state.final refino)
se cenário γ revelado.

### Decisão 8 — L0 TOCADO partial (primeira excepção justificada à aplicação automática ADR-0080 EM VIGOR pós-P229)

**Decisão crítica fixada — L0 partial tocado**:

ADR-0080 EM VIGOR §"Escopo" (audit C1 reconfirmar) limita-se
a refactors aditivos a variants/fields existentes. P240
adiciona feature runtime + walk integration:
- `Content::StateDisplay` variant novo.
- `Tag::StateDisplay` variant novo.
- `apply_state_displays` fixpoint function nova.
- `state_display_value` Introspector method novo.
- 4-5 entidades novas cumulativas.

**Não-trivial estructural** — paridade conceitual hipótese
P236 spec original (que foi rejeitada empíricamente
pós-P236.div-1; mas pattern proposto naquele spec aplica-se
aqui: features runtime novas merecem L0 tocado).

**L0 tocado partial**:
- `entities/content.md` — `Content::StateDisplay` variant
  novo documented.
- `rules/stdlib.md` — `state_display` func novo documented.
- `rules/introspect.md` ou similar — `apply_state_displays`
  + `state_display_value` documented.

**Pattern emergente "L0 tocado para features runtime novas
+ walk integration" N=1 inaugurado P240** — distinto de
pattern P236 original (que ficou em hipótese teórica).

**Pattern "aplicação automática ADR-0080 EM VIGOR" N=8
baseline preservado** mas **não-incrementa P240** (excepção
justificada).

ADR-0080 §"Escopo" anotada com excepção P240 (paridade
hipótese P236 spec original; primeira aplicação real).

Reuso de dados (sem recolha nova):

- `apply_state_funcs` baseline P171/M9 em `from_tags.rs:48`.
- `fixpoint::run_fixpoint` caller em `fixpoint.rs:101`.
- `apply_func(func, args, ctx, engine)` em `closures.rs:59`.
- `Content::State` + `Content::StateUpdate` baseline
  P171/P172.
- `Tag` enum baseline (audit C1 confirma localização).
- `Introspector::state_value(key, location)` baseline P171.
- `Engine` + `EvalContext` + `World` + `FileId` +
  `figure_numbering` interfaces.
- Pattern "spec C1 audit obrigatório bloqueante" N=2
  baseline P237 + P238 reescrito.
- Pattern "atomização prep-passo audit-only +
  materialização-passo" N=1 baseline P238 reescrito (com
  P239 N=2 cumulativo).
- ADR-0081 PROPOSTO baseline P239.
- 3 pré-condições obrigatórias formalizadas P239 §5.
- 2150 verdes baseline pré-P240.

---

## §2 Cláusulas (12 — atomização paridade P236+P240 magnitude L)

### C1 — AUDITORIA OBRIGATÓRIA BLOQUEANTE (lição P236.div-1 → P238.div-1 N=3 cumulativo)

**CRÍTICA absoluta** — primeira cláusula bloqueante; spec
C2+ depende output C1.

Audit empírico imediato:

```
grep -B 2 -A 30 "fn apply_state_funcs\|apply_state_funcs" 01_core/src/rules/introspect/from_tags.rs
grep -B 2 -A 20 "fn run_fixpoint\|run_fixpoint" 01_core/src/rules/introspect/fixpoint.rs
grep -B 2 -A 10 "fn apply_func\|apply_func" 01_core/src/rules/eval/closures.rs
grep -B 2 -A 5 "Content::State {" 01_core/src/entities/content.rs
grep -B 2 -A 5 "pub enum Tag\|Tag::Start\|Tag::StateUpdate" 01_core/src/
grep -B 2 -A 5 "ElementPayload::\|StateUpdate\|StateDisplay" 01_core/src/
grep -B 2 -A 10 "fn state_value\|state_final_value" 01_core/src/entities/introspector.rs
grep -B 2 -A 5 "fn state\b\|fn state_final\b\|fn state_at\b" 01_core/src/rules/stdlib/foundations.rs
grep -n "Content::StateUpdate\|Content::State {" 01_core/src/rules/layout/mod.rs
```

**Hipóteses sujeitas a confirmação empírica**:
- `apply_state_funcs(tags, intr, engine, ctx)` signature
  preservada P171/M9.
- `run_fixpoint` caller integração intacta.
- `Content::State { key, init }` + `Content::StateUpdate
  { key, update }` baseline preservados (audit P236 §2
  + P239 §3.2).
- `Tag` enum existing variants + `Tag::Start(loc, info)`
  com `ElementPayload::StateUpdate { ... }` semantic.
- `Introspector::state_value(key: &str, location: Location)
  -> Option<&Value>` baseline P171 + P237 audit.
- `Introspector::state_final_value(key: &str) -> Option<&Value>`
  baseline P171.

**Decisões críticas C1**:
1. **Tag enum localização exacta** — `entities/tag.rs`?
   `rules/introspect/tags.rs`? audit confirma.
2. **`Tag::Start(loc, info)` ou `Tag::StateUpdate` variant
   directo?** — audit confirma pattern existente.
3. **`ElementPayload` enum** — audit confirma existência
   + variants.
4. **`state_final_value` baseline retorna valor pós-fixpoint
   ou último update?** — Decisão 7 cenário α/β/γ depende
   desta resposta.
5. **`Content::from_value(value)` ou método similar?**
   — audit confirma; alternativa pattern inline.

**Critérios de divergência crítica**:
1. **Se `apply_state_funcs` signature divergente** —
   refactor pattern adaptado; sem `P240.div-N` se ajuste
   trivial.
2. **Se Tag enum estructura inesperada** — adaptar Tag
   variant novo; possível `P240.div-N` se refactor
   estructural maior necessário.
3. **Se `state_final_value` semantic diverge** — refino
   state.final cenário α/β/γ decidido.
4. **Se 2150 verdes não-preservados pré-P240** — pausar
   imediato; investigar regressão.

Se contradição material significativa: **PAUSAR e registar
`P240.div-N` formal** + questionário humano paridade
P236.div-1 + P238.div-1.

### C2 — Adicionar `Content::StateDisplay { key, callback }` variant + arms cascata

Editar `01_core/src/entities/content.rs`:

```rust
Content::StateDisplay {
    key: String,
    callback: Option<Func>,
}
```

Content variants: 60 → **61** (+StateDisplay).

**Arms cascata exhaustivos** (compiler-driven; ~10-15 arms):
- `entities/content.rs::PartialEq::eq` arm (Func PartialEq
  — audit C1 confirma; skip Func compare se necessário).
- `entities/content.rs::map_content` arm (preserva fields).
- `entities/content.rs::map_text` arm (preserva).
- `entities/content.rs::is_empty` arm (preservar layout
  behaviour).
- `entities/content.rs::plain_text` arm (sem texto direct).
- `rules/introspect.rs::materialize_time` arm StateDisplay
  (emit Tag::StateDisplay).
- `rules/layout/mod.rs::layout_content` arm StateDisplay
  (resolve state_display_value).
- `rules/introspect/locatable.rs` catch-all preserva.
- Outros arms compiler-driven identificados.

### C3 — Adicionar `Tag::StateDisplay` variant + ElementPayload arms

Audit C1 confirma estructura. Hipótese:

```rust
// entities/tag.rs ou equivalente:
pub enum ElementPayload {
    // ... existing variants ...
    StateDisplay {
        key: String,
        callback: Option<Func>,
    },
}
```

OR Tag directo:

```rust
pub enum Tag {
    // ... existing ...
    StateDisplay {
        loc: Location,
        key: String,
        callback: Option<Func>,
    },
}
```

Arms cascata Tag handling (audit C1 identifica points).

### C4 — `apply_state_displays` fixpoint function nova + Introspector storage

Editar `01_core/src/rules/introspect/from_tags.rs`:

```rust
pub fn apply_state_displays(
    tags: &[Tag],
    intr: &mut TagIntrospector,
    engine: &mut Engine<'_>,
    ctx: &mut EvalContext,
) {
    for tag in tags {
        if let Tag::Start(loc, info) = tag {
            if let ElementPayload::StateDisplay { key, callback } = &info.payload {
                let value = intr.state.value_at(key, *loc).cloned()
                    .unwrap_or(Value::None);
                let pre_rendered = match callback {
                    Some(func) => match apply_func(func.clone(), vec![value.clone()], ctx, engine) {
                        Ok(Value::Content(c)) => c,
                        Ok(other) => Content::from_value(&other),
                        Err(_) => Content::None,  // graceful degradation
                    },
                    None => Content::from_value(&value),
                };
                intr.state_displays.insert((key.clone(), *loc), pre_rendered);
            }
        }
    }
}
```

Editar `01_core/src/entities/introspector.rs`:

```rust
pub struct TagIntrospector {
    // ... existing ...
    pub state_displays: HashMap<(String, Location), Content>,  // P240
}

impl Introspector {
    pub fn state_display_value(&self, key: &str, loc: Location) -> Option<&Content> {
        self.state_displays.get(&(key.to_string(), loc))
    }
}
```

Editar `01_core/src/rules/introspect/fixpoint.rs::run_fixpoint`:

```rust
// Após apply_state_funcs:
apply_state_funcs(&tags, intr, engine, ctx);
apply_state_displays(&tags, intr, engine, ctx);  // P240
```

Magnitude C4: **M (~1-1.5h)**.

### C5 — Implementar `native_state_display` stdlib func

Editar `01_core/src/rules/stdlib/foundations.rs`:
(código completo Decisão 6 acima).

Registo scope em `01_core/src/rules/eval/mod.rs`:

```rust
scope.define("state_display", Value::Func(Func::native("state_display", native_state_display)));
```

Stdlib funcs: 62 → **63** (+state_display).

Magnitude C5: **S (~30min)**.

### C6 — Walk integration layout-time arm Content::StateDisplay

Editar `01_core/src/rules/layout/mod.rs::layout_content`:

```rust
Content::StateDisplay { key, callback: _ } => {
    let loc = self.current_location();  // audit C1 confirma método
    let pre_rendered = self.introspector
        .state_display_value(key, loc)
        .cloned()
        .unwrap_or(Content::None);
    self.layout_content(&pre_rendered)?;
}
```

Magnitude C6: **S+ (~45min-1h)**.

### C7 — Refino `state_final` two-pass real (cenário α/β/γ sujeito audit C1)

**Cenário α (trivial)**: `state_final_value` baseline já
retorna valor pós-fixpoint convergência. Refino: actualizar
doc-comment `state_final` para reflectir two-pass real
semantic + 1-2 tests confirmação semantic. **XS (~10min)**.

**Cenário β (trivial-ajuste)**: refino código state_final
para garantir cumulativo cuminative (pode ser ajuste
single-line). **XS-S (~30min)**.

**Cenário γ (não-trivial)**: refactor introspector
state_final semantic. **M (~2-3h)**. **Atomização M7+1A
(apply_state_displays) + M7+1B (state.final refino) ativada**.

**Decisão fixada (sujeita audit C1)**: cenário β provável;
atomização ativada se cenário γ revelado.

### C8 — Tests P240

Tests P240 (~15-20 mix unit + E2E walk-time render):

**Unit content** (~3 tests):
- `p240_content_statedisplay_variant_aceita_key_callback`.
- `p240_content_statedisplay_partial_eq_funcional`.
- `p240_content_statedisplay_map_content_preserva`.

**Unit stdlib** (~4 tests):
- `p240_native_state_display_sem_callback_constroi_variant`.
- `p240_native_state_display_com_callback_constroi_some`.
- `p240_native_state_display_arg_tipo_errado_rejeita`.
- `p240_native_state_display_arity_errada_rejeita`.

**Unit introspect/fixpoint** (~4 tests):
- `p240_apply_state_displays_sem_callback_renderiza_value`.
- `p240_apply_state_displays_com_callback_aplica_func`.
- `p240_state_display_value_lookup_retorna_pre_rendered`.
- `p240_apply_state_displays_callback_erro_retorna_content_none`.

**E2E walk-time render** (~4-7 tests crítico):
- `p240_state_display_walk_renderiza_valor_actual`.
- `p240_state_display_walk_callback_aplicado_correctamente`.
- `p240_state_display_walk_locations_diferentes_valores_diferentes`.
- `p240_state_display_walk_baseline_state_runtime_preservado`.
- `p240_state_final_two_pass_real_paridade_vanilla` (refino
  state.final).
- `p240_state_display_E2E_full_pipeline_render`.

Total tests P240: **~15-20 tests**. Esperado pós-P240:
**2150 + 18 = ~2168 verdes** (paridade hipótese; ajuste
pós-implementação).

### C9 — L0 TOCADO partial (Decisão 8)

**Decisão fixada — L0 partial tocado** (3 ficheiros):

- `00_nucleo/prompts/entities/content.md` — adicionar
  `Content::StateDisplay` variant documented.
- `00_nucleo/prompts/rules/stdlib.md` — adicionar
  `state_display` func documented.
- `00_nucleo/prompts/rules/introspect.md` ou similar —
  adicionar `apply_state_displays` + `state_display_value`
  documented.

ADR-0080 §"Escopo" anotada com excepção P240 (primeira
aplicação real pattern "L0 tocado para features runtime
novas + walk integration"):

```markdown
## Excepção P240 — features runtime + walk integration

ADR-0080 EM VIGOR aplica-se a refactors aditivos a
variants/fields existentes. **Features runtime + walk
integration** (novos Content variants + novos Tag variants +
funções fixpoint novas + Introspector methods novos) merecem
L0 tocado partial.

P236 spec original hipotetizou este pattern; rejeitado
pós-divergência P236.div-1 (state runtime já-materializado).
**P240 inaugura aplicação real do pattern** — primeira excepção
justificada à aplicação automática ADR-0080 EM VIGOR
pós-promoção P229.

Pattern "L0 tocado para features runtime novas + walk
integration" N=1 inaugurado P240.
```

**Pattern "aplicação automática ADR-0080 EM VIGOR" N=8
preservado** mas **não-incrementa P240** (excepção
justificada).

### C10 — Verificação tests workspace + lint

```
cargo test --workspace 2>&1 | tail -3
crystalline-lint .
crystalline-lint --fix-hashes .
```

Critério:
- 2150 verdes pré-P240 + ~15-20 novos = **~2168 verdes**.
- 0 violations preservadas.
- Hashes propagados em ~7-10 ficheiros L1.
- **L0 prompts TOCADOS partial** — hashes 3 ficheiros L0
  actualizados; "Nothing to fix" para o resto.

**Risco regressão**: tests baseline P171/M9 + P236 + P237
que constroem Content::State directly NÃO afectados (variant
novo distinto). Hipótese N=1-3 adaptações (tag handling
ajustes; arms cascata).

**3 pré-condições obrigatórias** (P239 §5) verificação
formal:
- 2150 verdes preservados pré-P240 + ~18 novos = ~2168
  pós-P240. **Verificar adaptações documentadas**.
- Comemo memoization invariants ADR-0073/0074 preservados.
  **Verificar fixpoint integration intacta**.
- Backward compat eval-time P236 state_final + P237
  state_at funcionam. **Verificar tests baseline P236+P237
  preservados**.

### C11 — ADR-0081 PROPOSTO → IMPLEMENTADO parcial + ADR-0079 anotação + Inventário 148 footnote ⁵⁹

**ADR-0081**:
- Status `PROPOSTO` → **`IMPLEMENTADO parcial`** (M7+1
  materializado; M7+2 a M7+5 pendentes).
- Bloco `## Implementado parcial P240 (2026-05-14)` adicionado:
  - M7+1 Opção γ `apply_state_displays` materializado.
  - Content::StateDisplay variant novo.
  - Tag::StateDisplay variant novo.
  - apply_state_displays fixpoint function nova.
  - Introspector.state_displays storage + state_display_value
    method.
  - native_state_display stdlib func nova.
  - Walk integration layout-time.
  - Refino state.final two-pass real (cenário α/β/γ
    aplicado conforme audit C1).
  - 3 pré-condições obrigatórias verificadas.
  - L0 tocado partial (3 ficheiros).
  - ~15-20 tests novos verdes.
  - Magnitude real verificada vs estimativa L (~5-8h).

**ADR-0079**:
- §"Aplicações cumulativas" anotado com bloco
  `### P240 anotação — M7+1 Pipeline walk-time eval Opção γ;
  Categoria D 1/? → 2/? sub-passos materializados; D.2
  state.display real materializado via M7+1`.
- Status ADR-0079 mantido PROPOSTO (11/13-15 sub-passos
  cumulativos; **Categoria A 5/5 ✓ + Categoria B 3/3 ✓ +
  Categoria D 2/? + Categoria C 0/?**).

**Inventário 148**:
- §A.X Introspection: footnote ⁵⁸ → ⁵⁸ ⁵⁹.
- Footnote ⁵⁹ adicionada (~150 linhas) documentando:
  - M7+1 Opção γ materializado.
  - Lição refinada P236.div-1 → P238.div-1 → P239 audit
    validada N=3 cumulativo.
  - Pattern emergente "refino aditivo paralelo entre
    callers fixpoint" N=1 inaugurado.
  - Pattern emergente "L0 tocado para features runtime
    novas + walk integration" N=1 inaugurado.
  - D.2 state.display real materializado primeira vez
    pós-M9c.
  - Refino state.final two-pass real (cenário α/β/γ
    aplicado).

### C12 — Critério aceitação P240

- ~15-20 tests novos verdes.
- 2150 tests pre-existentes preservados (após N=1-3
  adaptações intencionais documentadas).
- 0 violations.
- Content variants: 60 → **61** (+StateDisplay).
- Tag variants: +1 (StateDisplay).
- Stdlib funcs: 62 → **63** (+state_display).
- apply_state_displays fixpoint function nova.
- Introspector.state_displays storage + state_display_value
  method.
- Walk integration layout-time arm Content::StateDisplay
  funcional.
- Refino state.final two-pass real (cenário α/β/γ aplicado
  conforme audit C1).
- **L0 partial tocado** (3 ficheiros).
- **ADR-0081 PROPOSTO → IMPLEMENTADO parcial** (M7+1 ✓;
  M7+2 a M7+5 pendentes).
- ADR-0079 Categoria D 2/? anotado.
- ADR-0080 §"Escopo" anotada com excepção P240.
- **3 pré-condições obrigatórias verificadas formalmente**.
- Cobertura Layout per metodologia preservada 89%.
- Cobertura user-facing total: 67% → **~70%+** (D.2
  state.display real + state.final two-pass real bonus
  cumulativo).

---

## §3 Output

1 ficheiro relatório curto:
`00_nucleo/materialization/typst-passo-240-relatorio.md`.

Estrutura (~8-10 KB; magnitude L justifica maior que
P236+P237 cumulativos) com 8 §s:

- §1 O que foi feito (sumário 3-5 linhas).
- §2 Auditoria pré-P240 OBRIGATÓRIA BLOQUEANTE (C1) —
  output audit empírico documentado; cenário α/β/γ
  state.final decidido.
- §3 Content::StateDisplay + Tag::StateDisplay variants
  novos (C2+C3).
- §4 apply_state_displays fixpoint function + Introspector
  storage + state_display_value method (C4).
- §5 native_state_display stdlib + walk integration
  layout-time (C5+C6).
- §6 Decisões substantivas (8 decisões fixadas incl.
  Decisão 0 lição P236.div-1 N=3 cumulativo) + **primeira
  excepção justificada à aplicação automática ADR-0080
  EM VIGOR pós-P229**.
- §7 Resultados verificação + tests E2E walk-time +
  pré-condições obrigatórias (C8+C10) + ADR-0081 IMPLEMENTADO
  parcial + L0 tocado partial + footnote ⁵⁹ + ADR-0079
  Categoria D 2/?.
- §8 Próximo sub-passo (M7+2 counter.display paralelo
  M7+1; M7+3 multi-region; M7+4 Place float; M7+5 A.4
  radius/clip; ou D.2 graded fechado se M7+1 cobre).

Código alterado:
- **Editado**: `01_core/src/entities/content.rs` (+
  Content::StateDisplay variant + arms cascata + ~3
  unit tests).
- **Editado**: `01_core/src/entities/tag.rs` ou similar
  (+ Tag::StateDisplay variant + arms cascata).
- **Editado**: `01_core/src/entities/introspector.rs`
  (+ state_displays HashMap + state_display_value method).
- **Editado**: `01_core/src/rules/stdlib/foundations.rs`
  (+ native_state_display ~40 linhas + ~4 unit tests).
- **Editado**: `01_core/src/rules/eval/mod.rs` (+ scope
  define state_display).
- **Editado**: `01_core/src/rules/introspect/from_tags.rs`
  (+ apply_state_displays function ~30 linhas + ~4 unit
  tests).
- **Editado**: `01_core/src/rules/introspect/fixpoint.rs`
  (+ apply_state_displays call em run_fixpoint).
- **Editado**: `01_core/src/rules/layout/mod.rs` (arm
  Content::StateDisplay walk integration).
- **Editado**: `01_core/src/rules/introspect.rs` (refino
  state.final two-pass real conforme cenário α/β/γ).
- **Editado**: `01_core/src/rules/layout/tests.rs` (+~4-7
  E2E walk tests).
- **L0 EDITADO partial**: `00_nucleo/prompts/entities/content.md`
  + `00_nucleo/prompts/rules/stdlib.md` + `00_nucleo/prompts/rules/introspect.md`
  ou similar.
- **Editado**: `00_nucleo/adr/typst-adr-0081-m7-plus-pipeline-restructuring-scope.md`
  (status PROPOSTO → IMPLEMENTADO parcial; bloco P240
  materializado).
- **Editado**: `00_nucleo/adr/typst-adr-0080-*.md` (§"Escopo"
  anotada com excepção P240).
- **Editado**: `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`
  (footnote ⁵⁹ P240 + cobertura Introspection refino).
- **Editado**: `00_nucleo/adr/typst-adr-0079-layout-fase-5-roadmap.md`
  (+ anotação Categoria D 2/? P240).

**Sem novos ficheiros código** (refino aditivo a baseline).

---

## §4 Não-objectivos

- M7+2 counter.display paralelo — sub-passo separado
  candidato pós-M7+1 (paridade pattern M7+1; magnitude
  M ~2-4h).
- M7+3 multi-region completion cell-level — sub-passo
  separado candidato (magnitude L+).
- M7+4 Place float real — sub-passo separado candidato
  (magnitude L).
- M7+5 A.4 radius/clip infrastructure — sub-passo separado
  candidato (magnitude M-L).
- Promover ADR-0081 PROPOSTO → IMPLEMENTADO completo —
  só pós M7+1 a M7+5 cumulativos materializados (M7+1
  ✓ ⇒ IMPLEMENTADO parcial).
- Promover ADR-0079 PROPOSTO → IMPLEMENTADO — só pós
  Categorias A + B + C + D completas.
- Tocar ADR-0066 — SUPERSEDED-BY 0073 terminal preservado.
- ADR-0080 reescrita completa §"Escopo" — apenas anotação
  excepção P240.
- Refactor Show rules mecanismo synthetic — Opção δ rejeitada
  P239 audit.
- Refactor Layouter signature pass Engine+ctx — Opção α
  rejeitada P239 audit (comemo invariants risco).
- Two-pass walk explícito — Opção β rejeitada P239 audit
  (XL+ magnitude desnecessária).
- Promoção formal patterns emergentes consolidados
  (`.or()` N=3 limiar; refino paralelo N=5; Smart→Option
  N=12; semantic adiada N=8; aplicação automática EM VIGOR
  N=8) — passos administrativos XS separados candidatos
  paridade P229.
- Reabrir decisões P236.div-1 + P238.div-1 — lição refinada
  validada N=3 cumulativo P240.
- Marco cirúrgico blueprint para materialização M7+1 —
  paridade pattern P236+P237 (refinos aditivos não-marcam
  blueprint; mas M7+ pode merecer marco se Fase 5 fecha
  completo pós-M7+5).
- Performance optimization apply_state_displays — escopo
  P240 é correctness; optimization candidato futuro.

---

## §5 Riscos a evitar

1. **Audit C1 revela `apply_state_funcs` signature
   divergente pós-P239**: improvável (P239 audit recente).
   Mitigação: adaptar signature pattern ou `P240.div-N`.
2. **Tag enum estructura inesperada**: audit C1 confirma
   localização + variants. Mitigação: refactor Tag pattern
   adaptado.
3. **Cenário γ revelado state.final não-trivial**:
   atomização M7+1A + M7+1B ativada. Mitigação: spec C7
   antecipa atomização.
4. **Func PartialEq problemas**: audit C1 + alternative
   skip Func compare ou Arc pointer eq paridade hipótese
   P238 spec original §5 risco 4.
5. **Comemo memoization invariants quebradas**: pré-condição
   obrigatória C10. Mitigação: testes memoization existentes
   2150 verdes preservados; refactor `apply_state_displays`
   paralelo absoluto `apply_state_funcs` (que preservou
   invariants P171/M9 baseline).
6. **Tests baseline pré-P240 não-preservados**: pré-condição
   obrigatória C10. Mitigação: adaptações documentadas
   N>0 N<5 esperadas.
7. **Backward compat eval-time P236+P237 wrappers quebrados**:
   pré-condição obrigatória C10. Mitigação: refino state.final
   non-breaking (semantic refino sem mudança signature).
8. **Magnitude exceder L (~5-8h)**: feature runtime + walk
   integration. Hipótese real L (~6h). Se exceder L+,
   atomização M7+1A + M7+1B ativada.
9. **L0 tocado partial confunde hashes**: 3 ficheiros L0
   actualizados; resto preservado. Mitigação:
   `crystalline-lint --fix-hashes` actualiza só ficheiros
   tocados.
10. **Callback retorna não-Content tipo**: graceful
    degradation (`Content::from_value(other)` fallback)
    OR `Content::None`. Mitigação: tests específicos C8
    cobertura cenários edge.
11. **`Content::from_value` ausente**: audit C1 confirma
    método; alternativa pattern inline trivial.
12. **Promoção ADR-0079 prematura via D.2 materializado**:
    rejeitada — só pós Categorias A + B + C + D completas.
    Categoria C pendente; promoção prematura inflacionária.
13. **D.2 marca completion antes M7+5**: tentação por
    "M7+1 cobre D.2 completo". Defesa: D.2 inclui também
    counter.display paralelo (M7+2) per paridade vanilla
    semantic; D.2 fica incompleto até M7+2 materializado.

---

## §6 Hipótese provável

C1 (audit obrigatório bloqueante) confirmará:
- `apply_state_funcs` signature preservada baseline P171/M9.
- `run_fixpoint` caller integração intacta.
- Content::State + Content::StateUpdate baseline preservados.
- `Tag` enum localização + estructura.
- `apply_func` em closures.rs:59 preservado.
- Cenário β provável state.final refino (trivial-ajuste).
- 2150 verdes preservados pré-P240.

C2 adicionará Content::StateDisplay variant + arms cascata
~10-15 arms.

C3 adicionará Tag::StateDisplay variant + arms cascata.

C4 implementará apply_state_displays paralelo absoluto
apply_state_funcs + Introspector storage + state_display_value
method.

C5 implementará native_state_display ~40 linhas.

C6 implementará walk integration layout-time arm
Content::StateDisplay.

C7 refinará state.final two-pass real cenário β trivial-ajuste.

C8 criará ~15-20 tests novos (3 unit content + 4 unit
stdlib + 4 unit introspect/fixpoint + 4-7 E2E walk-time).

C9 tocará L0 partial (3 ficheiros) — primeira excepção
justificada à aplicação automática ADR-0080 EM VIGOR
pós-P229.

C10 reportará ~2168 verdes; 0 violations; possíveis N=1-3
adaptações.

C11 promoverá ADR-0081 PROPOSTO → IMPLEMENTADO parcial +
ADR-0079 Categoria D 2/? + footnote ⁵⁹.

C12 verifica critério aceitação.

Custo real: **L (~5-8h)** — feature runtime + walk
integration paralelismo absoluto apply_state_funcs +
testes E2E.

Mas é hipótese, não decisão. C1-C12 fixam-se empíricamente.
Possível `P240.div-N` se audit C1 diverge significativamente
paridade lição P236.div-1 + P238.div-1 + P239 cumulativo.

---

## §7 Particularidade P240

P240 é estruturalmente distinto **muito significativo**
na trajectória pós-M9c:

- **Vigésimo-sexto sub-passo Layout pós-M9c**.
- **Primeira sub-passo materialização M9d / M7+ pós-P239
  audit-only** — primeira aplicação real do pattern
  "atomização prep-passo audit-only + materialização-passo"
  inaugurado P238 reescrito N=1 → 2 cumulativo (validado
  empíricamente).
- **Terceira aplicação cumulativa "spec C1 audit obrigatório
  bloqueante pós-P236.div-1"** N=2 → 3 cumulativo (P237
  + P238 reescrito + **P240**).
- **Primeira excepção justificada à aplicação automática
  ADR-0080 EM VIGOR pós-promoção P229** — L0 partial tocado
  para feature runtime + walk integration. Pattern emergente
  "L0 tocado para features runtime novas + walk integration"
  N=1 inaugurado P240 — primeira aplicação real (P236
  spec original hipotetizou; rejeitada empíricamente
  pós-divergência).
- **Pattern "aplicação automática ADR-0080 EM VIGOR" N=8
  preservado** mas **não-incrementa P240** (excepção
  justificada documentada formalmente em ADR-0080 §"Escopo").
- **Pattern emergente "refino aditivo paralelo entre
  callers fixpoint" N=1 inaugurado P240** — extensão
  pattern P171/M9 `apply_state_funcs` baseline.
- **D.2 state.display real materializado primeira vez
  pós-M9c** — distinto cumulativo de P236+P237 eval-time
  wrappers + P238 reescrito documental.
- **state.final two-pass real refino** — sobreposição
  bloqueador D + A via Opção γ identificada P239 audit.
- **ADR-0081 PROPOSTO → IMPLEMENTADO parcial** — primeira
  promoção ADR meta pós-P229 (promoveu ADR-0080 EM VIGOR
  singular). Distribuição ADRs PROPOSTO 13 → 12 (-1 transita);
  IMPLEMENTADO 21 → 22 (+1; mas parcial — semantic distinta
  vs P221 ADR-0078 IMPLEMENTADO completo + P236 ADR-0066
  PROPOSTO → IMPLEMENTADO).
- **Pattern emergente "Promoção ADR PROPOSTO → IMPLEMENTADO
  parcial em sub-passo materialização" N=1 inaugurado
  P240** — distinto cumulativo de P221 (ADR-0078
  IMPLEMENTADO completo) + P236 (ADR-0066 PROPOSTO →
  IMPLEMENTADO tentado mas SUPERSEDED). Pattern necessário
  porque M7+ atomização permite IMPLEMENTADO parcial
  cumulativo.
- **Cobertura Introspection refino**: D.2 state.display
  real + state.final two-pass real adicionam cobertura
  cumulativa. Cobertura user-facing total: 67% → **~70%+**.
- **Cobertura Layout per metodologia preservada 89%**
  — M7+1 é Introspection refino + walk integration; não
  Layout.
- **Anti-inflação 32ª aplicação cumulativa** pós-P205D
  — Opção γ pattern reusado (não α/β/δ inflacionárias)
  + Opção β Content::StateDisplay variant (não α refino
  Content::State coerência) + Opção α Tag::StateDisplay
  paralelo + Opção β paralelismo absoluto apply_state_funcs
  + Opção γ L0 partial (não α extenso) + cenário β refino
  state.final trivial-ajuste (não γ atomização) + ADR-0081
  IMPLEMENTADO parcial (não completo prematuro).

Por isso §5 risco 8 (magnitude exceder L) é o mais provável
empíricamente. Mitigação: atomização M7+1A + M7+1B activada
se exceder L+. Pattern emergente "atomização in-flight
sub-passo materialização" N=1 inaugurado P240 se ativado.

**Critério de aceitação P240**:
- ~15-20 tests novos verdes.
- 2150 tests pre-existentes preservados (após N=1-3
  adaptações).
- 0 violations.
- Content variants 60 → 61; Tag variants +1; Stdlib funcs
  62 → 63.
- apply_state_displays fixpoint function + Introspector
  storage + state_display_value method.
- Walk integration funcional.
- Refino state.final two-pass real.
- **L0 partial tocado** (3 ficheiros) — primeira excepção
  justificada.
- **ADR-0081 PROPOSTO → IMPLEMENTADO parcial** (M7+1 ✓;
  M7+2 a M7+5 pendentes).
- ADR-0079 Categoria D 2/? anotado.
- ADR-0080 §"Escopo" anotada com excepção P240.
- 3 pré-condições obrigatórias verificadas formalmente.
- Cobertura Layout per metodologia preservada 89%.
- Cobertura user-facing total: 67% → **~70%+**.

**Estado pós-P240 esperado**:
- Tests workspace: 2150 → **~2168 verdes** (+15-20).
- **Content variants: 60 → 61** (+StateDisplay).
- **Tag variants: +1** (StateDisplay).
- **Stdlib funcs: 62 → 63** (+state_display).
- Value variants: 55 preservado.
- Grid/Table/Cell/Block/Boxed/Place fields preservados.
- Layouter fields: preservados.
- §A.5 distribuição: `12/4/2/0/0 = 18` preservada.
- **Cobertura Introspection refino +D.2 state.display
  real + state.final two-pass real**.
- Cobertura Layout per metodologia: **89% preservada**.
- Cobertura user-facing total: 67% → **~70%+**.
- **ADR-0066 SUPERSEDED-BY 0073 preservado**.
- **ADR-0081 status**: PROPOSTO → **IMPLEMENTADO parcial**
  (M7+1 ✓).
- ADR-0079 PROPOSTO Categoria A 5/5 + B 3/3 + D 2/? + C 0/?.
- ADR-0080 EM VIGOR §"Escopo" anotada excepção P240.
- **Distribuição ADRs**: PROPOSTO **13 → 12** (-1: ADR-0081
  transita parcial); EM VIGOR 29; IMPLEMENTADO **22 →
  23** (+1: ADR-0081 parcial); total 68 preservado.
- Saldo DEBTs: 11 preservado (M7+1 não-fecha DEBTs; M7+3
  pode fechar DEBT-56b candidato).
- **32 aplicações cumulativas anti-inflação** pós-P205D.
- **Pattern "L0 minimal para refactors" aplicação
  automática N=8 preservado** (P230-P237; P240 excepção
  justificada não-incrementa).
- **Pattern "L0 tocado para features runtime novas + walk
  integration" N=1 inaugurado P240** — primeira aplicação
  real.
- **Pattern "spec C1 audit obrigatório bloqueante" N=2
  → 3 cumulativo** (P237 + P238 reescrito + **P240**).
- **Pattern "atomização prep-passo audit-only +
  materialização-passo" N=1 → 2 cumulativo** (P238 reescrito
  + P239 + **P240 validado**).
- **Pattern "refino aditivo paralelo entre callers fixpoint"
  N=1 inaugurado P240**.
- **Pattern "Promoção ADR PROPOSTO → IMPLEMENTADO parcial
  em sub-passo materialização" N=1 inaugurado P240** —
  distinto de P221 + P236.
- **Patterns inaugurados P240** (3-4):
  - "L0 tocado para features runtime novas + walk
    integration" N=1.
  - "Refino aditivo paralelo entre callers fixpoint" N=1.
  - "Promoção ADR PROPOSTO → IMPLEMENTADO parcial" N=1.
  - "Atomização in-flight sub-passo materialização" N=1
    (SE cenário γ state.final activado).
- **Categoria D Fase 5 Layout: 1/? → 2/? sub-passos
  materializados** (D.1 state_final eval-time + state_at
  eval-time; **D.2 state.display walk-time real materializado
  via M7+1**).
- **Fase 5 Layout candidata: 10/13-15 → 11/13-15 sub-passos
  materializados** (~73-85% cumulativo; **Categoria A
  100% + Categoria B 100% + Categoria D 2/? + Categoria
  C 0/?**).
- **M9d / M7+ progresso**: 1/5 sub-passos materializados
  (M7+1 ✓; M7+2 + M7+3 + M7+4 + M7+5 pendentes).
- **Magnitude cumulativa M9d/M7+ restante**: ~18-29h
  (M7+2 M+2-4h; M7+3 L+8-12h; M7+4 L+5-8h; M7+5 M-L+3-5h).
- **Marco interno implícito**: primeira sub-passo
  materialização M-fase pós-M9c reabertura iniciada
  metodologicamente correctamente — lição refinada
  P236.div-1 → P238.div-1 → P239 audit validada empíricamente
  N=3 cumulativo via materialização real funcional.

**Próximo sub-passo pós-P240 candidatos**:

| Caminho | Trabalho | Magnitude | Prioridade |
|---------|----------|-----------|------------|
| **M7+2 counter.display paralelo M7+1** | Paridade pattern M7+1 absoluta (Content::CounterDisplay + Tag + apply_counter_displays + native_counter_display + walk integration) | **M (~2-4h)** | **alta** (completa pattern; D.2 totalmente real) |
| **M7+3 multi-region completion cell-level** | Pipeline cell-level multi-region; DEBT-56b candidato | L+ (~8-12h) | alta (desbloqueia C.2 + A.4 breakable per-cell) |
| **M7+4 Place float real** | Reabertura Opção B P219 graded | L (~5-8h) | média (desbloqueia C.1) |
| **M7+5 A.4 radius/clip infrastructure** | ShapeKind::RoundedRect + Corners<T> + PDF export | M-L (~3-5h) | média (A.4 graded P231 promoção real) |
| **ADR meta admin XS** | Promoção formal patterns sólidos paridade P229 | XS por pattern | baixa-média |
| Pivot outro módulo | Visualize/Text/Model | varia | baixa |
| Pausa M-fase | Fase 5 candidata fecha graded ~80-85% | XS | baixa |

**Recomendação subjectiva pós-P240**: **M7+2 counter.display
paralelo M7+1** — completa D.2 totalmente real (state.display
+ counter.display paridade vanilla); pattern absoluto
reuso; magnitude M (~2-4h) mínima. Permite ADR-0081
IMPLEMENTADO parcial cumulativo 2/5.

**Alternativa subjectiva**: **M7+5 A.4 radius/clip** —
menor magnitude (M-L ~3-5h); infraestrutura geometry
isolada; ganho user-facing imediato A.4 graded P231
promoção real.

**Decisão humana fica em aberto literal** pós-P240
materialização.
