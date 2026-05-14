# Relatório do passo P241 — M7+2 counter.display walk-time eval via Opção γ `apply_counter_displays` (M9d segunda sub-passo; paralelo absoluto P240; segunda excepção justificada ADR-0080 EM VIGOR)

**Data**: 2026-05-14.
**Spec**: `00_nucleo/materialization/typst-passo-241.md`.
**Tipo**: feature runtime nova + walk integration via pattern reusado
(`apply_state_displays` baseline P240); +1 Content variant
(`CounterDisplayCallback`); +1 ElementPayload variant; +1 ElementKind
variant; +1 stdlib func; +1 fixpoint function; +1 Introspector
trait method + storage; walk arm; **L0 partial tocado** (3 ficheiros).
**Magnitude planeada**: M (~2-4h). **Magnitude real**: M- (~1.5h
audit + implementação + tests + L0 + relatorio; paralelismo absoluto
P240 acelerou implementação).
**Marco**: **segunda sub-passo materialização M9d / M7+ pós-P240**;
pattern "refino aditivo paralelo entre callers fixpoint" N=1 → 2
cumulativo (P240 + P241); pattern "L0 tocado para features runtime
novas + walk integration" N=1 → 2 cumulativo; **quarta aplicação
cumulativa pattern "spec C1 audit obrigatório bloqueante
pós-P236.div-1"** N=3 → 4 cumulativo.

---

## §1 O que foi feito

P241 materializa M7+2 Opção γ `apply_counter_displays` per
ADR-0081 IMPLEMENTADO parcial (M7+1 ✓ P240 → M7+2 ✓ aqui). Paralelo
absoluto P240 substituindo `state_display` por `counter_display`:
adiciona `Content::CounterDisplayCallback` variant + correspondente
ElementPayload + ElementKind + `apply_counter_displays` fixpoint
function (paralelo absoluto `apply_state_displays`) + Introspector
trait method `counter_display_value` + storage `counter_displays`
+ `native_counter_display` stdlib func + walk integration
layout-time. Converte `intr.counters.value_at(key, loc)` para
`Value::Array(Vec<Value::Int>)` (paridade vanilla `CounterState =
SmallVec<[u64; 3]>`) e aplica callback via `apply_func` pós-fixpoint.
**2162 → 2175 verdes** (+13; 0 regressões; 0 adaptações; spec
previa +10-14). Audit C1 P241 refinou naming variant
(`CounterDisplayCallback` vs `CounterDisplay2`); ajuste trivial sem
`P241.div-N` formal.

---

## §2 Auditoria pré-P241 OBRIGATÓRIA BLOQUEANTE (C1) — lição N=4 cumulativo

**Audit empírico** (paralelo lição refinada `P236.div-1 → P238.div-1
→ P239 audit → P240 audit` N=4 cumulativo):

| Aspecto auditado | Achado empírico | Implicação |
|------------------|-----------------|------------|
| `Content::CounterDisplay { kind }` legacy | Existe em `content.rs:193` com 1 field `kind: String` (sem callback); **not locatable** (não passa em is_locatable=true) | Variant nova paralela `CounterDisplayCallback`; legacy preservada inalterada (Decisão 1 Opção α) |
| `CounterRegistry.value_at(key, loc)` | `Option<&[usize]>` em `counter_registry.rs:127` (slice do último snapshot ≤ loc) | Conversão para `Value::Array(Vec<Value::Int>)` (Decisão 4) |
| `Value::Array(Vec<Value>)` | Existe em `value.rs:284` | OK para passar ao callback |
| `apply_state_displays` baseline P240 | Signature `(tags, intr, engine, ctx)` em `from_tags.rs:80+` preservado | Pattern espelho directo para `apply_counter_displays` |
| `run_fixpoint` caller pós-P240 | Sequência `apply_state_funcs → apply_state_displays` em `fixpoint.rs:101+` | Adicionar `apply_counter_displays` em sequência |
| `ElementPayload::CounterUpdate { key, action }` (P198C) | Existe sem callback (variant escrita; CounterDisplay nova é leitura+callback) | Variant `ElementPayload::CounterDisplay` distinta |
| `ElementKind::CounterDisplay` | Não existe (apenas `CounterUpdate`) | Criar novo + "counter_display" as_str/from_name |
| Naming Content variant | Spec sugeriu `CounterDisplayCallback` ou `CounterDisplay2` | Audit C1 decidiu `CounterDisplayCallback` (mais explícito) |
| Tests baseline pré-P241 | **2162 verdes** confirmado | Baseline para +10-14 |

**Sem `P241.div-N` formal** — ajuste signature trivial (naming
final `CounterDisplayCallback` pós-audit) coerente com lição N=4
cumulativo (paridade P237 audit C1 + P238 reescrito + P240 audit C1
ajustes triviais sem div-N).

---

## §3 Content::CounterDisplayCallback + ElementPayload::CounterDisplay + ElementKind::CounterDisplay (C2+C3)

`01_core/src/entities/content.rs`:

```rust
Content::CounterDisplayCallback {
    key:      String,
    callback: Option<crate::entities::func::Func>,
}
```

Arms cascata compiler-driven (paralelo absoluto P240):
- `Content::PartialEq` arm explícito (Func ptr_eq).
- `Content::plain_text` arm `String::new()`.
- `Content::map_content` / `map_text` arms `self.clone()`.
- `Content::materialize_time` arm `content.clone()` (terminal).
- Walk arm `=> {}` em `rules/introspect.rs` (tag emitido no topo).
- `is_locatable` arm `=> true`.

`01_core/src/entities/element_payload.rs`:

```rust
ElementPayload::CounterDisplay {
    key:      String,
    callback: Option<crate::entities::func::Func>,
}
```

Hash auto via Debug-string format; PartialEq auto-derive; Eq
marker preservado.

`01_core/src/entities/element_kind.rs`:

```rust
ElementKind::CounterDisplay   // + "counter_display" as_str/from_name
```

**Counts pós-P241**:
- Content variants: 61 → **62** (+CounterDisplayCallback).
- ElementPayload variants: +1 (CounterDisplay).
- ElementKind variants: +1 (CounterDisplay).

**Coexistência preservada**: `Content::CounterDisplay { kind }`
legacy inalterada (Decisão 1 Opção α — variant nova paralela);
legacy continua útil em paths single-pass directo no Layouter
sem callback.

---

## §4 apply_counter_displays fixpoint function + Introspector storage + counter_display_value method (C4)

`01_core/src/rules/introspect/from_tags.rs` — paralelo absoluto
`apply_state_displays` P240:

```rust
pub fn apply_counter_displays(
    tags:   &[Tag],
    intr:   &mut TagIntrospector,
    engine: &mut Engine<'_>,
    ctx:    &mut EvalContext,
) {
    for tag in tags {
        if let Tag::Start(loc, info) = tag {
            if let ElementPayload::CounterDisplay { key, callback } = &info.payload {
                let counter_slice_opt = intr.counters.value_at(key, *loc);
                let counter_value: Value = counter_slice_opt
                    .map(|slice| Value::Array(
                        slice.iter().map(|&n| Value::Int(n as i64)).collect()
                    ))
                    .unwrap_or(Value::Array(vec![]));
                let pre_rendered = match callback {
                    Some(func) => {
                        let args = Args::positional(vec![counter_value]);
                        match apply_func(func.clone(), args, ctx, engine) {
                            Ok(Value::Content(c))  => c,
                            Ok(Value::Str(s))      => Content::text(s.as_str()),
                            Ok(_)                  => Content::Empty,
                            Err(_)                 => Content::Empty,
                        }
                    }
                    None => match counter_slice_opt {
                        Some(slice) => {
                            let s = slice.iter()
                                .map(|n| n.to_string())
                                .collect::<Vec<_>>()
                                .join(".");
                            Content::text(&s)
                        }
                        None => Content::Empty,
                    },
                };
                intr.counter_displays.insert((key.clone(), *loc), pre_rendered);
            }
        }
    }
}
```

**Paralelismo absoluto `apply_state_displays`**: mesma signature,
mesma estrutura, mesma defensive ignore em Err. **Diferenças
mínimas**:
- Converte counter slice → `Value::Array(Vec<Value::Int>)` (vs
  `state.value_at` → `Value` directo).
- Sem callback: formato default join "." (vs state passa-through
  `Value::Content`/`Value::Str`).
- Counter inexistente: `Value::Array(vec![])` (vs state inexistente
  `Value::None`).

`01_core/src/entities/introspector.rs`:

```rust
// Trait method (paralelo state_display_value):
fn counter_display_value(
    &self,
    key: String,
    location: Location,
) -> Option<crate::entities::content::Content>;

// TagIntrospector field:
pub counter_displays: HashMap<(String, Location), Content>,

// Impl:
fn counter_display_value(
    &self,
    key: String,
    location: Location,
) -> Option<Content> {
    self.counter_displays.get(&(key, location)).cloned()
}
```

`03_infra/src/measurements.rs::CountingIntrospector`: adapter
delegação trivial paridade outros métodos.

`01_core/src/rules/introspect/fixpoint.rs::run_fixpoint`:

```rust
let curr_hash = compute_tags_hash(&tags);
apply_state_funcs(&tags, &mut introspector, engine, ctx);
apply_state_displays(&tags, &mut introspector, engine, ctx);
// P241 (M9d/M7+2): pre-render `Content::CounterDisplayCallback`
// paralelo absoluto `apply_state_displays` (ADR-0081 M7+2).
apply_counter_displays(&tags, &mut introspector, engine, ctx);
```

---

## §5 native_counter_display stdlib + walk integration layout-time (C5+C6)

`01_core/src/rules/stdlib/foundations.rs`:

```rust
pub fn native_counter_display(
    _ctx:                &mut EvalContext,
    args:                &Args,
    _world:              &dyn World,
    _current_file:       FileId,
    _figure_numbering:   Option<&str>,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Str(key)] => Ok(Value::Content(Content::CounterDisplayCallback {
            key: key.to_string(), callback: None })),
        [Value::Str(key), Value::Func(callback)] => Ok(Value::Content(Content::CounterDisplayCallback {
            key: key.to_string(), callback: Some(callback.clone()) })),
        [Value::Str(_), other] => err(format!(
            "counter_display() requer função como segundo argumento (callback), recebeu {}", other.type_name())),
        [other, ..] => err(format!(
            "counter_display() requer string como primeiro argumento (key), recebeu {}", other.type_name())),
        _ => err(format!(
            "counter_display() requer 1-2 argumentos (key, [callback]), recebeu {}", args.items.len())),
    }
}
```

Registo scope `01_core/src/rules/eval/mod.rs:624`:
```rust
scope.define("counter_display", Value::Func(Func::native("counter_display", native_counter_display)));
```

Re-export `01_core/src/rules/stdlib/mod.rs:36`: `native_counter_display`
adicionado.

Stdlib funcs: **63 → 64** (+counter_display).

Walk integration `01_core/src/rules/layout/mod.rs`:

```rust
Content::CounterDisplayCallback { key, callback: _ } => {
    use crate::entities::introspector::Introspector;
    if let Some(loc) = self.current_location {
        let pre_rendered_opt = self.introspector
            .counter_display_value(key.clone(), loc);
        if let Some(pre_rendered) = pre_rendered_opt {
            self.layout_content(&pre_rendered);
        }
    }
}
```

**Layouter permanece puro** — sem Engine+ctx em signature; paridade
arquitectural estrita preservada P240 (Opção γ vs α/β/δ P239 audit).

---

## §6 Decisões substantivas (8 decisões fixadas incl. Decisão 0 lição N=4 cumulativo) + segunda excepção justificada ADR-0080 EM VIGOR

**8 decisões fixadas P241** (Decisão 0 = lição N=4 cumulativo
P237 + P238 reescrito + P240 + P241):

| # | Decisão | Opção fixada |
|---|---------|--------------|
| 0 | C1 audit obrigatório bloqueante | lição N=4 aplicada; sem `P241.div-N` (audit converge com refino naming) |
| 1 | Content variant nova | **Opção α** `CounterDisplayCallback` paralela (não β refino legacy) |
| 2 | ElementPayload variant | **Opção α** `CounterDisplay` paralelo `StateDisplay` |
| 3 | ElementKind variant | **Opção α** `CounterDisplay` paralelo `StateDisplay` |
| 4 | Forma Value ao callback | **Opção α** `Value::Array(Vec<Value::Int>)` paridade vanilla |
| 5 | Counter inexistente | **Opção α** Array vazio + Content::Empty fallback |
| 6 | native_counter_display arity | 1-2 arg paridade `native_state_display` |
| 7 | L0 partial tocado | **Segunda excepção justificada ADR-0080 EM VIGOR pós-P229** (N=1 → 2 cumulativo) |
| 8 | Tests materializados | No mesmo passo (sem stubs diferidos) |

**Segunda excepção justificada ADR-0080 EM VIGOR pós-P229**:

Critério §"Excepção P240" preservado literal: P241 satisfaz
"4+ entidades novas cumulativas + walk integration arquitectural +
pipeline restructuring + feature M-fase nova" (~6 entidades novas).

L0 partial tocado (3 ficheiros paralelos P240):
- `00_nucleo/prompts/entities/content.md` — bloco
  `Content::CounterDisplayCallback` documentado.
- `00_nucleo/prompts/rules/stdlib.md` — bloco
  `counter_display(key, [callback])` documentado.
- `00_nucleo/prompts/rules/introspect.md` — bloco
  `apply_counter_displays` + `Introspector::counter_display_value`
  documentado.

**ADR-0080 §"Excepção P241"** anotada formalmente cristalizando
N=2 cumulativo.

**Pattern emergente "L0 tocado para features runtime novas + walk
integration" N=1 → 2 cumulativo** (P240 + P241). Promoção a
sub-categoria ADR-0080 candidata se N=3 atinge em sub-passo M7+
futuro.

**Pattern "aplicação automática ADR-0080 EM VIGOR" N=8 preservado**
mas **não-incrementa P241** (excepção justificada documentada
formalmente).

**Anti-inflação 33ª aplicação cumulativa** pós-P205D — Opção γ
pattern reusado (paridade absoluta P240) + Opção α variant nova
paralela + Opção α naming explícito + Opção α Value::Array paridade
vanilla + Opção γ L0 partial (segunda excepção N=2) + ADR-0081
IMPLEMENTADO parcial 2/5.

---

## §7 Resultados verificação + tests E2E walk-time + pré-condições obrigatórias (C7+C10)

| Critério | Esperado | Real |
|----------|----------|------|
| `cargo build --workspace` | verde | ✓ verde |
| `cargo test --workspace` | ~2174 verdes (range 2172-2176) | **2175 verdes** (1886+242+24+2+21) ✓ |
| `crystalline-lint .` | 0 violations | **0 violations** ✓ |
| `crystalline-lint --fix-hashes` | 3 L0 hashes + ~7-10 ficheiros L1 actualizados | ✓ |
| Adaptações pre-existentes | N=0 (pattern paralelo absoluto não-disruptive) | **N=0** ✓ |
| Content variants | 61 → 62 | ✓ |
| ElementPayload variants | +1 (CounterDisplay) | ✓ |
| ElementKind variants | +1 (CounterDisplay) | ✓ |
| Stdlib funcs | 63 → 64 | ✓ |
| TagIntrospector fields | +1 (counter_displays) | ✓ |
| Introspector trait methods | +1 (counter_display_value) | ✓ |
| ADR-0081 status | IMPLEMENTADO parcial 1/5 → **2/5** | ✓ (M7+2 ✓; M7+3-M7+5 pendentes) |
| ADR-0079 Categoria D | 2/? → **3/?** | ✓ |
| ADR-0080 §"Excepção P241" | anotada N=2 cumulativo | ✓ |
| L0 partial tocado | 3 ficheiros | ✓ |
| Regressões reais | 0 | **0** |

**Tests P241** (13 unit + cenários canónicos paridade P240):

**Unit content** (4 tests em `entities/content.rs`):
- `p241_content_counter_display_callback_partial_eq_sem_callback`.
- `p241_content_counter_display_callback_partial_eq_com_callback_ptr_eq`.
- `p241_content_counter_display_callback_plain_text_vazio`.
- `p241_content_counter_display_callback_distinto_de_legacy_counter_display`
  (verifica coexistência variant nova vs legacy).

**Unit stdlib** (4 tests em `rules/stdlib/mod.rs`):
- `p241_native_counter_display_sem_callback_constroi_variant`.
- `p241_native_counter_display_com_callback_constroi_some`.
- `p241_native_counter_display_arg_tipo_errado_rejeita`.
- `p241_native_counter_display_arity_errada_rejeita`.

**Unit introspect/fixpoint** (5 tests em
`rules/introspect/from_tags.rs`):
- `p241_apply_counter_displays_sem_callback_renderiza_formato_default`
  (counter `[1, 1]` hierárquico → `Content::text("1.1")`).
- `p241_apply_counter_displays_com_callback_aplica_func` (callback
  recebe `Value::Array([Int(1), Int(1)])` → retorna formato custom).
- `p241_apply_counter_displays_callback_erro_retorna_content_empty`
  (defensive ignore Err → Content::Empty paridade P240).
- `p241_apply_counter_displays_locations_diferentes_valores_diferentes`
  (paridade history-aware `value_at` location-monotónica).
- `p241_apply_counter_displays_counter_inexistente_array_vazio`
  (`Value::Array(vec![])` fallback → callback recebe array vazio
  `len=0`).

**3 pré-condições obrigatórias verificadas** (per ADR-0081
§"Pré-condições obrigatórias"):
1. **Tests baseline preservados**: 2162 verdes pré-P241 → 2175
   verdes pós-P241 (+13 novos; 0 regressões reais; 0 adaptações
   intencionais).
2. **Comemo memoization invariants ADR-0073/0074 preservados**:
   trait `Introspector` `#[comemo::track]` continua válido com
   novo method `counter_display_value(String, Location) ->
   Option<Content>` compatível (paridade P240).
3. **Backward compat**: `Content::CounterDisplay { kind }` legacy
   preservada inalterada — todos os tests pré-P241 que usam
   variant legacy continuam intactos; P240 wrappers `state_display`
   + tests preservados.

**Promoções ADR**:
- **ADR-0081 IMPLEMENTADO parcial 1/5 → 2/5** (M7+2 ✓; M7+3-M7+5
  pendentes). Distribuição ADRs preservada literal — sem novos
  ADRs criados; sem PROPOSTO ↔ IMPLEMENTADO. PROPOSTO 12; EM
  VIGOR 29; IMPLEMENTADO 22; total **68 preservado**.
- ADR-0079 Categoria D 2/? → **3/?** anotado (D.3 counter.display
  walk-time real).
- ADR-0080 §"Excepção P241" N=2 cumulativo anotada.
- ADR-0066 SUPERSEDED-BY 0073 preservado.

**Inventário 148 footnote ⁶⁰** adicionada (~220 linhas) documentando:
M7+2 Opção γ materializado paralelo absoluto P240; lição N=4
cumulativo validada; pattern "refino aditivo paralelo entre callers
fixpoint" N=2 cumulativo; D.3 counter.display real materializado
primeira vez pós-M9c; segunda excepção justificada ADR-0080 EM
VIGOR; 8 decisões fixadas; 3 patterns emergentes.

---

## §8 Próximo sub-passo pós-P241

P241 completa M7+2 (M9d segunda sub-passo). Restantes 3 sub-passos
M9d/M7+ pendentes (magnitude cumulativa restante ~16-25h).

| Caminho | Trabalho | Magnitude | Prioridade |
|---------|----------|-----------|------------|
| **M7+5 A.4 radius/clip infrastructure** | `ShapeKind::RoundedRect { radii: Corners<Length> }` + `Corners<T>` paridade `Sides<T>` + PDF exportador | **M-L (~3-5h)** | **alta** (geometry isolada; A.4 graded P231 promoção real; ganho user-facing imediato) |
| M7+3 multi-region completion cell-level | `Regions { current, backlog, last }` completo; DEBT-56b candidato | L+ (~8-12h) | média (desbloqueia C.2 + A.4 breakable per-cell) |
| M7+4 Place float real | Reabertura Opção B P219 graded | L (~5-8h) | média (desbloqueia C.1) |
| ADR meta admin XS | Promoção formal patterns N=2 cumulativos pós-P241 ("L0 tocado features runtime + walk" + "refino aditivo paralelo callers fixpoint") | XS por pattern | baixa-média |
| Pivot outro módulo | Visualize 54%; Text 52%; Model 50% | varia | baixa |
| Pausa M-fase | Fase 5 graded ~80-92% (10 + M7+1 ✓ + M7+2 ✓ = 12/13-15) | XS | baixa |

**Recomendação subjectiva pós-P241**: **M7+5 A.4 radius/clip**.
Menor magnitude restante M-L (~3-5h); geometry isolada (sem
dependências pipeline); ganho user-facing imediato (A.4 graded
P231 promoção real). Alternativa: M7+3 multi-region (desbloqueio
maior C.2 + A.4 breakable; magnitude L+).

**Decisão humana fica em aberto literal** pós-P241.

**Estado pós-P241**:
- Tests workspace: 2162 → **2175 verdes** (+13 P241).
- Content variants: 61 → **62**.
- ElementPayload variants: +1.
- ElementKind variants: +1.
- **Stdlib funcs: 63 → 64** (+counter_display).
- Value variants: 55 preservado.
- §A.5 distribuição: `12/4/2/0/0 = 18` preservada.
- Cobertura Layout per metodologia: **89% preservada** (M7+2 é
  Introspection refino + walk integration).
- Cobertura user-facing total: ~70% → **~71-72%** (D.3
  counter.display real bonus marginal).
- **ADRs distribuição preservada**: PROPOSTO 12; EM VIGOR 29;
  IMPLEMENTADO 22; total **68 preservado**. ADR-0081 transita
  1/5 → **2/5** internamente; sem ADR novo. ADR-0079 Categoria
  D **3/?** anotada. ADR-0080 §"Excepção P241" anotada.
- **Saldo DEBTs: 11 preservado**.
- **33 aplicações cumulativas anti-inflação** pós-P205D.
- **Patterns emergentes inaugurados/consolidados P241** (3):
  - "L0 tocado para features runtime novas + walk integration"
    N=1 → **2 cumulativo** (P240 + P241).
  - "Refino aditivo paralelo entre callers fixpoint" N=1 → **2
    cumulativo** (P240 `apply_state_displays` + P241
    `apply_counter_displays`).
  - "Spec C1 audit obrigatório bloqueante" N=3 → **4 cumulativo**.
- **Categoria D Fase 5 Layout: 2/? → 3/? sub-passos
  materializados** (D.1 eval-time wrappers; D.2 walk-time real;
  **D.3 counter.display walk-time real**).
- **Fase 5 Layout candidata: 11/13-15 → 12/13-15 sub-passos
  materializados** (~80-92% cumulativo).
- **M9d / M7+ progresso**: **2/5 sub-passos materializados**
  (M7+1 ✓; **M7+2 ✓**; M7+3 + M7+4 + M7+5 pendentes; cumulativa
  restante ~16-25h).
- **Marco interno**: segunda sub-passo materialização M9d
  validada — pattern "refino aditivo paralelo entre callers
  fixpoint" N=2 cumulativo confirmado empíricamente sem
  divergências factuais materiais. Lição N=4 cumulativo C1 audit
  bloqueante refinou naming variant pós-audit sem div-N (paridade
  P237/P240 precedente).
