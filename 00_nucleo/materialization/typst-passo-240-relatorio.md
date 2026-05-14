# Relatório do passo P240 — M7+1 Pipeline walk-time eval via Opção γ `apply_state_displays` (M9d primeira sub-passo; primeira excepção justificada ADR-0080 EM VIGOR pós-P229)

**Data**: 2026-05-14.
**Spec**: `00_nucleo/materialization/typst-passo-240.md`.
**Tipo**: feature runtime nova + walk integration via pattern reusado
(`apply_state_funcs` baseline P171/M9); +1 Content variant
(`StateDisplay`); +1 ElementPayload variant; +1 ElementKind
variant; +1 stdlib func; +1 fixpoint function; +1 Introspector
trait method + storage; walk arm; **L0 partial tocado** (3 ficheiros).
**Magnitude planeada**: L (~5-8h). **Magnitude real**: M+ (~3h
auditoria + implementação + tests; menor que estimativa por
sobreposição grande bloqueador A+D resolvida via Opção γ + cenário
α state.final two-pass trivial confirmed).
**Marco**: **primeira sub-passo materialização M9d / M7+ pós-P239
audit-only**; primeira aplicação real do pattern "atomização
prep-passo audit-only + materialização-passo" inaugurado P238
reescrito N=1 → 2 cumulativo; terceira aplicação cumulativa pattern
"spec C1 audit obrigatório bloqueante pós-P236.div-1" N=2 → 3
cumulativo; **primeira excepção justificada à aplicação automática
ADR-0080 EM VIGOR pós-P229** (L0 partial tocado).

---

## §1 O que foi feito

P240 materializa M7+1 Opção γ apply_state_displays per ADR-0081
PROPOSTO P239 audit §3.2. Adiciona feature runtime walk-time real
`state.display(callback)` via pattern paralelo absoluto
`apply_state_funcs` baseline P191B/M9 — pre-evaluate em fixpoint
pós-walk com Engine+ctx disponíveis; layout arm consome Content
pre-rendered via Introspector trait method (Layouter permanece
puro). Desbloqueia D.2 state.display walk-time real + state.final
two-pass real via sobreposição bloqueador A+D identificada P239
audit C4. **2150 → 2162 verdes** (+12; 0 regressões; 0 adaptações).
Audit C1 P240 refinou hipótese spec C3 (`ElementPayload::StateDisplay`
em vez de `Tag::StateDisplay` directo — Tag enum é
`Tag::Start(Location, ElementInfo)` com payload via ElementInfo);
ajuste signature trivial sem `P240.div-N` formal.

---

## §2 Auditoria pré-P240 OBRIGATÓRIA BLOQUEANTE (C1) — lição N=3 cumulativo aplicada

**Audit empírico** (paralelo lição refinada `P236.div-1 → P238.div-1
→ P239 audit` N=3 cumulativo P237 + P238 reescrito + P240):

| Aspecto auditado | Achado empírico | Implicação para P240 |
|------------------|-----------------|----------------------|
| `apply_state_funcs` signature | `pub fn apply_state_funcs(tags, intr, engine, ctx)` em `from_tags.rs:48` preservado | Pattern espelho directo para `apply_state_displays` |
| `run_fixpoint` caller | em `fixpoint.rs:101`; chamado após walk + hash compute | Caller espelho directo |
| `apply_func` signature | `apply_func(func, args, ctx, engine)` em `closures.rs:59`; toma `Args` (não `Vec<Value>`) | Adaptação `Args::positional(vec![v])` |
| `Func` derives | `Clone` only + manual `Debug ("<function>")` + manual `PartialEq` (Arc::ptr_eq) | Container `Content::StateDisplay` requer arm manual PartialEq |
| `Tag` enum | `Tag::Start(Location, ElementInfo)` — payload via `ElementInfo::payload` | **Spec C3 refinado**: `ElementPayload::StateDisplay` em vez de `Tag::StateDisplay` directo |
| `ElementPayload` enum | `derive(PartialEq)` auto + manual Hash via Debug-string + manual Eq marker | Pattern preserves; adicionar variant directo |
| `Content::State` + `StateUpdate` em PartialEq | **NÃO têm arm explícito**; fall-through `_ => false` | Adicionar arm explícito `Content::StateDisplay` com Func::PartialEq |
| `state_final_value` semantic | `state.final_value(key)` → `history.last()`; após `apply_state_funcs` em fixpoint = valor cumulativo two-pass real | **Cenário α confirmed**: refino state.final two-pass trivial (apenas docs) |
| `Layouter` introspector access | `comemo::Tracked<dyn Introspector>` | Method `state_display_value` deve estar na trait, retornar Owned `Content` (Tracked não permite `&Content`) |
| `Layouter.current_location` | `Option<Location>` field; set durante walk synchronization | Layout arm acessa directo |
| `Content::from_value` | NÃO existe | Conversão inline `Value::Content(c)=>c; Value::Str(s)=>Content::text(s); _=>Content::Empty` |
| Tests baseline pré-P240 | **2150 verdes** (1861+242+24+2+21) | Baseline preservado para verificação |

**Sem `P240.div-N` formal** — ajuste signature trivial (Decisão 3:
`ElementPayload::StateDisplay` paridade pattern existente) coerente
com lição N=3 cumulativo (P237 audit C1 + P238 reescrito + P240).

---

## §3 Content::StateDisplay + ElementPayload::StateDisplay variants novos (C2+C3)

`01_core/src/entities/content.rs`:

```rust
Content::StateDisplay {
    key:      String,
    callback: Option<crate::entities::func::Func>,
}
```

Arms cascata compiler-driven (todos exhaustive non-exhaustive
warnings resolvidos):
- `Content::PartialEq` arm explícito (Func ptr_eq via
  `Func::PartialEq` impl em `func.rs:163`).
- `Content::plain_text` arm `String::new()` (paridade State +
  StateUpdate baseline).
- `Content::map_content` arm `self.clone()` (terminal).
- `Content::map_text` arm `self.clone()` (terminal).
- `Content::materialize_time` arm `content.clone()` (terminal —
  resolução real via `apply_state_displays` + layout arm).
- `Content` walk arm `=> {}` em `rules/introspect.rs:1126` (tag
  emitido no topo via extract_payload).
- `is_locatable` arm `=> true` em `rules/introspect/locatable.rs`.

`01_core/src/entities/element_payload.rs`:

```rust
ElementPayload::StateDisplay {
    key:      String,
    callback: Option<crate::entities::func::Func>,
}
```

Hash auto via Debug-string format (paridade pattern existente);
PartialEq auto-derive (Func tem manual PartialEq via Arc::ptr_eq);
Eq marker preservado.

`01_core/src/entities/element_kind.rs`:

```rust
ElementKind::StateDisplay   // + "state_display" as_str/from_name
```

Content variants: **60 → 61** (+StateDisplay).
ElementPayload variants: +1 (StateDisplay).
ElementKind variants: +1 (StateDisplay).

---

## §4 apply_state_displays fixpoint function + Introspector storage + state_display_value method (C4)

`01_core/src/rules/introspect/from_tags.rs`:

```rust
pub fn apply_state_displays(
    tags:   &[Tag],
    intr:   &mut TagIntrospector,
    engine: &mut Engine<'_>,
    ctx:    &mut EvalContext,
) {
    for tag in tags {
        if let Tag::Start(loc, info) = tag {
            if let ElementPayload::StateDisplay { key, callback } = &info.payload {
                let value = intr.state.value_at(key, *loc).cloned()
                    .unwrap_or(Value::None);
                let pre_rendered = match callback {
                    Some(func) => {
                        let args = Args::positional(vec![value]);
                        match apply_func(func.clone(), args, ctx, engine) {
                            Ok(Value::Content(c))  => c,
                            Ok(Value::Str(s))      => Content::text(s.as_str()),
                            Ok(_)                  => Content::Empty,
                            Err(_)                 => Content::Empty,
                        }
                    }
                    None => match value {
                        Value::Content(c) => c,
                        Value::Str(s)     => Content::text(s.as_str()),
                        _                 => Content::Empty,
                    },
                };
                intr.state_displays.insert((key.clone(), *loc), pre_rendered);
            }
        }
    }
}
```

**Paralelismo absoluto `apply_state_funcs` P191B** — mesma
signature, mesma estrutura iteração tags, mesma defensive ignore
em Err. Diferença: armazena `pre_rendered: Content` em
`state_displays` (em vez de actualizar `intr.state` via
`intr.state.update`).

`01_core/src/entities/introspector.rs`:

```rust
// Trait method (paralelo state_value):
fn state_display_value(
    &self,
    key: String,        // Owned (não &str) — Tracked-compatible
    location: Location,
) -> Option<crate::entities::content::Content>;  // Owned (clone) — Tracked-compatible

// TagIntrospector field:
pub state_displays: HashMap<(String, Location), Content>,

// Impl:
fn state_display_value(
    &self,
    key: String,
    location: Location,
) -> Option<Content> {
    self.state_displays.get(&(key, location)).cloned()
}
```

`03_infra/src/measurements.rs::CountingIntrospector`: adapter
delegação trivial paridade outros métodos state_*.

`01_core/src/rules/introspect/fixpoint.rs::run_fixpoint`:

```rust
let curr_hash = compute_tags_hash(&tags);
apply_state_funcs(&tags, &mut introspector, engine, ctx);
// P240 (M9d/M7+1): pre-render `Content::StateDisplay` callbacks
// paralelo `apply_state_funcs` (Opção γ ADR-0081 PROPOSTO P239).
apply_state_displays(&tags, &mut introspector, engine, ctx);
```

---

## §5 native_state_display stdlib + walk integration layout-time (C5+C6)

`01_core/src/rules/stdlib/foundations.rs`:

```rust
pub fn native_state_display(
    _ctx:                &mut EvalContext,
    args:                &Args,
    _world:              &dyn World,
    _current_file:       FileId,
    _figure_numbering:   Option<&str>,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Str(key)] => Ok(Value::Content(Content::StateDisplay {
            key: key.to_string(), callback: None })),
        [Value::Str(key), Value::Func(callback)] => Ok(Value::Content(Content::StateDisplay {
            key: key.to_string(), callback: Some(callback.clone()) })),
        [Value::Str(_), other] => err(format!(
            "state_display() requer função como segundo argumento (callback), recebeu {}", other.type_name())),
        [other, ..] => err(format!(
            "state_display() requer string como primeiro argumento (key), recebeu {}", other.type_name())),
        _ => err(format!(
            "state_display() requer 1-2 argumentos (key, [callback]), recebeu {}", args.items.len())),
    }
}
```

Registo scope `01_core/src/rules/eval/mod.rs:618`:
```rust
scope.define("state_display", Value::Func(Func::native("state_display", native_state_display)));
```

Re-export `01_core/src/rules/stdlib/mod.rs:35-38`: `native_state_display`
adicionado a lista pub use.

Stdlib funcs: **62 → 63** (+state_display).

Walk integration `01_core/src/rules/layout/mod.rs:355+`:

```rust
Content::StateDisplay { key, callback: _ } => {
    use crate::entities::introspector::Introspector;
    if let Some(loc) = self.current_location {
        let pre_rendered_opt = self.introspector
            .state_display_value(key.clone(), loc);
        if let Some(pre_rendered) = pre_rendered_opt {
            self.layout_content(&pre_rendered);
        }
        // Sem pre_rendered: defensive ignore (fixpoint pre-walk
        // ainda não convergiu OR Func errored OR key inexistente).
    }
    // Sem current_location: defensive ignore (walk pre-Locator).
}
```

**Layouter permanece puro** — sem Engine+ctx em signature;
paridade arquitectural estrita preservada (Opção γ vs α/β/δ
P239 audit). `layout_content` returns `()` (não `Result`)
portanto sem `?` operator.

---

## §6 Decisões substantivas (8 decisões fixadas incl. Decisão 0 lição N=3 cumulativo) + primeira excepção justificada ADR-0080 EM VIGOR pós-P229

**8 decisões fixadas P240** (Decisão 0 = lição P236.div-1 →
P238.div-1 → P239 audit aplicada literal N=3 cumulativo):

| # | Decisão | Opção fixada |
|---|---------|--------------|
| 0 | C1 audit obrigatório bloqueante | lição N=3 aplicada; sem `P240.div-N` (audit converge com refino Decisão 3) |
| 1 | Escopo M7+1 | **Opção γ** apply_state_displays (confirmada P239 audit + verificada P240 implementação) |
| 2 | Content::StateDisplay variant | **Opção β** variant novo (não α refino Content::State coerência) |
| 3 | Tag emit | **Opção α refinada empíricamente**: `ElementPayload::StateDisplay` (não `Tag::StateDisplay` directo per audit C1 P240) |
| 4 | Pre-render storage | **Opção β** paralelismo absoluto apply_state_funcs + Introspector trait method |
| 5 | Walk integration layout-time | via Introspector trait method (Layouter puro preservado) |
| 6 | Stdlib func native_state_display | 1-2 arg conforme spec |
| 7 | Refino state.final two-pass real | **Cenário α confirmed**: trivial (docs apenas; `state_final_value` baseline já two-pass real pós-`apply_state_funcs` em fixpoint) |
| 8 | L0 partial tocado | **Primeira excepção justificada à aplicação automática ADR-0080 EM VIGOR pós-P229** (3 ficheiros) |

**Primeira excepção justificada ADR-0080 EM VIGOR pós-P229**:

ADR-0080 §"Escopo" aplica-se a refactors aditivos a variants/fields
existentes. **Features runtime + walk integration** (4-5 entidades
novas cumulativas: Content variant + ElementPayload variant +
ElementKind variant + fixpoint function + Introspector trait method
+ storage + stdlib func + walk arm) merecem L0 tocado partial.

L0 partial tocado (3 ficheiros):
- `00_nucleo/prompts/entities/content.md` — bloco
  `Content::StateDisplay` documented.
- `00_nucleo/prompts/rules/stdlib.md` — bloco
  `state_display(key, [callback])` documented.
- `00_nucleo/prompts/rules/introspect.md` — bloco
  `apply_state_displays` + `Introspector::state_display_value`
  documented.

**ADR-0080 §"Excepção P240"** anotada formalmente cristalizando
critério para excepções futuras.

**Pattern emergente "L0 tocado para features runtime novas +
walk integration" N=1 inaugurado P240** — primeira aplicação real
(P236 spec original hipotetizou; rejeitada empíricamente
pós-`P236.div-1` divergência state runtime já-materializado).

**Pattern "aplicação automática ADR-0080 EM VIGOR" N=8 preservado**
mas **não-incrementa P240** (excepção justificada documentada
formalmente).

**Anti-inflação 32ª aplicação cumulativa** pós-P205D — Opção γ
pattern reusado (não α/β/δ inflacionárias) + Opção β
Content::StateDisplay variant novo + Opção α refinada
ElementPayload (não Tag::StateDisplay) + Opção β paralelismo
absoluto + Opção γ L0 partial + Cenário α state.final trivial +
ADR-0081 IMPLEMENTADO parcial.

---

## §7 Resultados verificação + tests E2E walk-time + pré-condições obrigatórias (C8+C10) + ADR-0081 IMPLEMENTADO parcial + L0 tocado partial + footnote ⁵⁹ + ADR-0079 Categoria D 2/?

| Critério | Esperado | Real |
|----------|----------|------|
| `cargo build --workspace` | verde | ✓ verde |
| `cargo test --workspace` | ~2162-2168 verdes | **2162 verdes** (1873+242+24+2+21) ✓ |
| `crystalline-lint .` | 0 violations | **0 violations** ✓ |
| `crystalline-lint --fix-hashes` | 3 L0 hashes + ~7-10 ficheiros L1 actualizados | ✓ (8 ficheiros L1 + 3 ficheiros L0 hashes propagados) |
| Adaptações pre-existentes | N=0-3 | **N=0** ✓ (zero regressões; pattern paralelo absoluto não-disruptive) |
| Content variants | 60 → 61 | ✓ |
| ElementPayload variants | +1 | ✓ |
| ElementKind variants | +1 | ✓ |
| Stdlib funcs | 62 → 63 | ✓ |
| TagIntrospector fields | +1 (state_displays) | ✓ |
| Introspector trait methods | +1 (state_display_value) | ✓ |
| ADR-0081 status | PROPOSTO → IMPLEMENTADO parcial | ✓ (M7+1 ✓; M7+2-M7+5 pendentes) |
| ADR-0079 Categoria D | 1/? → 2/? | ✓ |
| ADR-0080 §"Excepção P240" | anotada | ✓ |
| L0 partial tocado | 3 ficheiros | ✓ |
| Regressões reais | 0 | **0** |

**Tests P240** (12 unit + cenários canónicos):

**Unit content** (3 tests em `entities/content.rs`):
- `p240_content_statedisplay_partial_eq_sem_callback`.
- `p240_content_statedisplay_partial_eq_com_callback_ptr_eq`.
- `p240_content_statedisplay_plain_text_vazio`.

**Unit stdlib** (4 tests em `rules/stdlib/mod.rs`):
- `p240_native_state_display_sem_callback_constroi_variant`.
- `p240_native_state_display_com_callback_constroi_some`.
- `p240_native_state_display_arg_tipo_errado_rejeita`.
- `p240_native_state_display_arity_errada_rejeita`.

**Unit introspect/fixpoint** (5 tests em
`rules/introspect/from_tags.rs`):
- `p240_apply_state_displays_sem_callback_renderiza_value`.
- `p240_apply_state_displays_com_callback_aplica_func` (callback
  retorna Value::Str("v=42") → Content::text("v=42")).
- `p240_apply_state_displays_callback_erro_retorna_content_empty`
  (defensive ignore Err → Content::Empty).
- `p240_apply_state_displays_locations_diferentes_valores_diferentes`
  (paridade location-monotónica state.value_at).
- `p240_apply_state_displays_state_inexistente_value_none`
  (Value::None fallback → Content::Empty).

**3 pré-condições obrigatórias verificadas** (per ADR-0081
§"Pré-condições obrigatórias" P239):
1. **Tests baseline preservados**: 2150 verdes pré-P240 → 2162
   verdes pós-P240 (+12; 0 regressões reais; 0 adaptações
   intencionais).
2. **Comemo memoization invariants ADR-0073/0074 preservados**:
   trait `Introspector` `#[comemo::track]` continua válido com
   novo method `state_display_value(String, Location) ->
   Option<Content>` compatível com macro (parâmetros owned;
   return type owned).
3. **Backward compat eval-time**: P236 state_final wrapper +
   P237 state_at wrapper continuam funcionar inalterados; tests
   P236+P237 preservados intactos.

**Promoções ADR**:
- **ADR-0081 PROPOSTO → IMPLEMENTADO parcial** (M7+1 ✓; M7+2
  a M7+5 pendentes). Distribuição ADRs: PROPOSTO 13 → **12** (-1
  transita); IMPLEMENTADO 21 → **22** (+1 parcial); EM VIGOR
  29; total **68 preservado**.
- ADR-0079 Categoria D 1/? → **2/?** anotado (D.2
  state.display walk-time real).
- ADR-0080 §"Excepção P240" anotada (primeira excepção
  justificada formalmente cristalizada).
- ADR-0066 SUPERSEDED-BY 0073 preservado.

**Inventário 148 footnote ⁵⁹** adicionada (~180 linhas)
documentando: M7+1 Opção γ materializado; lição refinada
P236.div-1 → P238.div-1 → P239 audit validada N=3 cumulativo;
4 patterns emergentes inaugurados/consolidados; D.2 state.display
real materializado primeira vez pós-M9c; refino state.final
two-pass real cenário α confirmed empíricamente; primeira
excepção justificada ADR-0080 EM VIGOR.

---

## §8 Próximo sub-passo

P240 completa M7+1 (M9d primeira sub-passo); restantes 4
sub-passos M9d/M7+ pendentes (magnitude cumulativa restante
~18-29h).

| Caminho | Trabalho | Magnitude | Prioridade |
|---------|----------|-----------|------------|
| **M7+2 counter.display paralelo M7+1** | Paridade pattern M7+1 absoluta (`Content::CounterDisplay { key, callback }` + `ElementPayload::CounterDisplay` + `apply_counter_displays` paralelo `apply_state_displays` + `native_counter_display` + walk integration) | **M (~2-4h)** | **alta** (completa D 2/? → 3/? totalmente real; pattern absoluto reuso) |
| M7+5 A.4 radius/clip infrastructure | `ShapeKind::RoundedRect { radii: Corners<Length> }` + `Corners<T>` paridade `Sides<T>` + PDF exportador | M-L (~3-5h) | média (geometry isolada; A.4 graded P231 promoção real) |
| M7+3 multi-region completion cell-level | `Regions { current, backlog, last }` completo; DEBT-56b candidato | L+ (~8-12h) | média (desbloqueia C.2 + A.4 breakable per-cell) |
| M7+4 Place float real | Reabertura Opção B P219 graded | L (~5-8h) | média (desbloqueia C.1) |
| ADR meta admin XS | Promoção formal patterns sólidos (`.or()` N=3; refino paralelo N=5; Smart→Option N=12; "L0 tocado features runtime + walk integration" N=1 novo P240) | XS por pattern | baixa-média |
| Pivot outro módulo | Visualize 54%; Text 52%; Model 50% | varia | baixa |
| Pausa M-fase | Fase 5 graded ~80-85% (10 + M7+1 ✓ = 11/13-15 sub-passos) | XS | baixa |

**Recomendação subjectiva pós-P240**: **M7+2 counter.display
paralelo M7+1**. Reuso pattern absoluto (P240 inaugurou pattern
"refino aditivo paralelo entre callers fixpoint" N=1; M7+2 = N=2
cumulativo); magnitude M ~2-4h mínima; completa D.2 → D 3/?
totalmente real (state.display + counter.display paridade vanilla
semantic). Permite ADR-0081 IMPLEMENTADO parcial cumulativo 2/5.

**Alternativa subjectiva**: **M7+5 A.4 radius/clip** — menor
magnitude isolated; ganho user-facing imediato (A.4 graded P231
promoção real); infraestrutura geometry independente.

**Decisão humana fica em aberto literal** pós-P240.

**Estado pós-P240**:
- Tests workspace: 2150 → **2162 verdes** (+12 P240).
- Content variants: 60 → **61**.
- ElementPayload variants: +1.
- ElementKind variants: +1.
- **Stdlib funcs: 62 → 63** (+state_display).
- Value variants: 55 preservado.
- Grid/Table/Cell/Block/Boxed/Place fields preservados.
- Layouter fields: preservados (current_location pre-existente).
- §A.5 distribuição: `12/4/2/0/0 = 18` preservada.
- Cobertura Layout per metodologia: **89% preservada** (M7+1 é
  Introspection refino + walk integration; não Layout estrutural).
- Cobertura user-facing total: 67% → **~68-70%** (D.2
  state.display real + state.final two-pass real bonus marginal).
- **ADRs distribuição**: PROPOSTO 13 → **12** (-1: ADR-0081
  transita parcial); EM VIGOR 29; IMPLEMENTADO 21 → **22** (+1:
  ADR-0081 parcial); total **68 preservado**. ADR-0066
  SUPERSEDED-BY 0073 preservado. ADR-0079 Categoria D 2/?
  anotada. ADR-0080 §"Excepção P240" anotada.
- **Saldo DEBTs: 11 preservado**.
- **32 aplicações cumulativas anti-inflação** pós-P205D.
- **Patterns emergentes** (4 P240):
  - "L0 tocado para features runtime novas + walk integration"
    N=1 inaugurado P240.
  - "Refino aditivo paralelo entre callers fixpoint" N=1
    inaugurado P240.
  - "Spec C1 audit obrigatório bloqueante" N=2 → 3 cumulativo.
  - "Atomização prep-passo audit-only + materialização-passo"
    N=1 → 2 cumulativo (P238 reescrito → P239 → P240 validação
    empírica).
- **Categoria D Fase 5 Layout: 1/? → 2/? sub-passos
  materializados** (D.1 eval-time wrappers; **D.2 walk-time real
  P240**).
- **Fase 5 Layout candidata: 10/13-15 → 11/13-15 sub-passos
  materializados** (~73-85% cumulativo).
- **M9d / M7+ progresso**: **1/5 sub-passos materializados**
  (M7+1 ✓; M7+2 + M7+3 + M7+4 + M7+5 pendentes; cumulativa
  restante ~18-29h).
- **Marco interno**: primeira sub-passo materialização M-fase
  pós-M9c reabertura iniciada metodologicamente correctamente
  — lição refinada P236.div-1 → P238.div-1 → P239 audit validada
  empíricamente N=3 cumulativo via materialização real funcional.
