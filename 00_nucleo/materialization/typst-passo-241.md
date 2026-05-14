# Spec do passo P241 — M7+2 counter.display walk-time eval via Opção γ `apply_counter_displays` (M9d segunda sub-passo; paralelo absoluto P240)

**Data**: 2026-05-14.
**Tipo**: feature runtime nova + walk integration via pattern
reusado (`apply_state_displays` baseline P240/M9d); +1 Content
variant (`CounterDisplayCallback` — naming a confirmar em audit
C1; ver §3 Decisão 1); +1 ElementPayload variant; +1 ElementKind
variant; +1 stdlib func; +1 fixpoint function; +1 Introspector
trait method + storage; walk arm; **L0 partial tocado** (estimado
3 ficheiros, paridade absoluta P240).
**Magnitude planeada**: M (~2-4h). Inferida pelo §8 do relatório
P240 que recomendou M7+2 como reuso pattern absoluto.
**Marco**: **segunda sub-passo materialização M9d / M7+ pós-P240**;
N=2 cumulativo do pattern "refino aditivo paralelo entre callers
fixpoint" inaugurado P240; **segunda excepção justificada à
aplicação automática ADR-0080 EM VIGOR pós-P229** (L0 partial
tocado); quarta aplicação cumulativa pattern "spec C1 audit
obrigatório bloqueante pós-P236.div-1" N=3 → 4 cumulativo.

---

## §1 O que será feito

P241 materializa M7+2 Opção γ `apply_counter_displays` per
ADR-0081 IMPLEMENTADO parcial (M7+1 ✓ P240; M7+2 alvo aqui).
Adiciona feature runtime walk-time real `counter.display(callback)`
via **pattern paralelo absoluto** `apply_state_displays` baseline
P240/M9d — pre-evaluate em fixpoint pós-walk com Engine+ctx
disponíveis; layout arm consome Content pre-rendered via
Introspector trait method (Layouter permanece puro). Desbloqueia
D.3 counter.display walk-time real.

**Tests esperados**: 2162 → ~2174 verdes (+12 baseline; range
+10-14). Zero regressões esperadas (pattern paralelo absoluto
não-disruptive paridade P240).

**Audit C1 P241** deve refinar 1-2 hipóteses spec (paridade
método com P240 audit C1 que refinou hipótese C3
`ElementPayload::StateDisplay`); ajustes triviais sem
`P241.div-N` formal.

---

## §2 Auditoria pré-P241 OBRIGATÓRIA BLOQUEANTE (C1) — lição N=4 cumulativo

**Audit empírico obrigatório** antes de qualquer código tocado
(paralelo lição refinada `P236.div-1 → P238.div-1 → P239 audit
→ P240 audit` N=4 cumulativo). Aspectos críticos a inventariar:

| Aspecto a auditar | Hipótese a confirmar | Implicação se falhar |
|-------------------|---------------------|----------------------|
| `Content::CounterDisplay { kind }` legacy existente | Variant pré-P241 usa `kind: String` sem callback (per `layout_counters.md` L0 + P60s históricos). Manter inalterada — variant nova paralela em vez de refino | Refino vs paralelo é Decisão C3 — ver §3 abaixo |
| `apply_state_displays` baseline P240 | Signature `(tags, intr, engine, ctx)` em `from_tags.rs` preservado pós-P240 | Pattern espelho directo para `apply_counter_displays` |
| `run_fixpoint` caller pós-P240 | Caller actualizado para chamar `apply_state_displays` | Adicionar chamada `apply_counter_displays` na mesma sequência |
| `ElementPayload::CounterUpdate { key, action }` (P198C) | Variant existente sem callback (paralelo `ElementPayload::StateUpdate`) | **Spec C3 hipótese**: nova variant `ElementPayload::CounterDisplay { key, callback }` em vez de refinar `CounterUpdate` existente |
| `CounterRegistry.value_at(key, loc)` | Retorna `Option<&[usize]>` (history-aware per P177) | Conversão para `Value` antes de invocar callback — Vec<usize> → `Value::Array` ou formato string |
| Forma do `Value` passado ao callback | Vanilla typst passa `CounterState` (Vec<usize>) ao callback; equivalente cristalino é `Value::Array(Vec<Value::Int>)` ou string formatada | Decisão C4 — ver §3 |
| `Func` derives + container `Content::CounterDisplay` novo | Manual `PartialEq` arm necessário (paridade P240) | Adicionar arm explícito `Content::CounterDisplay` PartialEq |
| `is_locatable` legacy `Content::CounterDisplay` | Confirmar estado actual (provavelmente `false` — variant legacy não passa por tags) | Variant nova deve ser locatable; manter legacy como está |
| Tests baseline pré-P241 | **2162 verdes esperado** (pós-P240) | Baseline para verificação +10-14 |

**Sem `P241.div-N` formal antecipado** — ajustes triviais
paralelos a P240 audit C1 (lição N=4 cumulativo). Se audit
revelar bloqueador material (e.g. estado `Content::CounterDisplay`
legacy incompatível com refino), criar `P241.div-1` formal e
parar.

---

## §3 Decisões fixadas P241 — 8 decisões

### Decisão 1 — Naming variant Content nova

**Opção α (recomendada)**: criar `Content::CounterDisplay2 { key,
callback: Option<Func> }` ou `Content::CounterDisplayCallback {
key, callback }` paralela; manter legacy `Content::CounterDisplay
{ kind }` inalterada. Justificação: variant legacy continua útil
em produção single-pass (paridade P240 que NÃO refinou
`Content::State` existente, criou `Content::StateDisplay` paralela).

**Opção β**: refinar `Content::CounterDisplay { kind, callback:
Option<Func> }` adicionando field. Justificação: menos variants;
reuso semântico. Contra: breaks 60+ tests existentes; viola
política ADR-0080 minimal "L0 minimal para refactors"
extensivamente.

**Decisão fixada P241**: **Opção α** — variant nova paralela
(coerência N=2 cumulativa pattern P240).

**Naming preliminar**: `Content::CounterDisplayCallback` (mais
explícito que `CounterDisplay2`); decisão final delegada a
audit C1 + Claude Code (paralela "ElementPayload::StateDisplay vs
Tag::StateDisplay" P240 audit C1).

### Decisão 2 — ElementPayload variant nova

`ElementPayload::CounterDisplay { key, callback }` paralela a
`ElementPayload::StateDisplay` P240. Distinta de
`ElementPayload::CounterUpdate { key, action }` existente
(P198C) — CounterDisplay é leitura+callback; CounterUpdate é
escrita.

### Decisão 3 — ElementKind variant nova

`ElementKind::CounterDisplay` + `"counter_display"` as_str/from_name.
Paralelo `ElementKind::StateDisplay` P240.

### Decisão 4 — Forma do `Value` passado ao callback

**Opção α (recomendada)**: passar `Value::Array(Vec<Value::Int>)`
representando o counter state actual (paridade vanilla
`CounterState` = `SmallVec<[u64; 3]>`). Permite callbacks ricos
como `counter("heading").display(nums => nums.map(str).join("."))`.

**Opção β**: passar `Value::Str(format)` já formatado (string
"1.2.3"). Mais simples mas perde semântica vanilla.

**Decisão fixada P241**: **Opção α** — paridade vanilla literal.

### Decisão 5 — Fallback semantic quando counter inexistente

`counter.value_at(key, loc)` retorna `None` → callback recebe
`Value::Array(vec![])` (vector vazio = counter não inicializado).
Paralelo P240 onde `state.value_at` `None` → `Value::None` passado.

Cenário sem callback: retorna `Content::Empty` (paralelo P240
state.display sem callback + state inexistente).

### Decisão 6 — Naming stdlib func

`native_counter_display(key, callback?)` registado como
`counter_display` em stdlib. Paralelo `native_state_display`
P240. Stdlib funcs: 63 → 64 (+counter_display).

**Nota syntax Typst**: vanilla expressa como
`counter("heading").display(callback)`. Cristalino pode usar
forma flat `counter_display("heading", callback)` (paridade
método sugar deferido) ou eventualmente refactor field access
em variant `counter()` quando M9+ counter completo for revisitado.
**Forma flat preferida P241** para preservar granularidade
(`counter` field access é trabalho separado fora M7+).

### Decisão 7 — Opção γ L0 partial tocado (segunda excepção ADR-0080)

L0 a tocar (estimado 3 ficheiros, paridade absoluta P240):
- `entities/content.md` (+ secção `CounterDisplayCallback` ou
  variant nome final decidido em audit C1).
- `entities/element_payload.md` (+ variant `CounterDisplay`).
- `entities/element_kind.md` (+ variant `CounterDisplay`).
- Possivelmente `entities/introspector.md` (+ method
  `counter_display_value`).

**Segunda excepção justificada ADR-0080 EM VIGOR pós-P229** — N=1
(P240) → 2 (P241) cumulativo. Padrão "L0 tocado para features
runtime novas + walk integration" promove-se a N=2; **anotação
ADR-0080 §"Excepções"** acumula entrada P241.

### Decisão 8 — Sem materialização hipotética

Tests E2E walk-time são parte da materialização. Sem stubs ou
"adicionar mais tarde" — paridade absoluta P240 que materializou
todos os 12 tests no mesmo passo.

---

## §4 `Content::CounterDisplayCallback` + `ElementPayload::CounterDisplay` (C2+C3)

Forma esperada em `01_core/src/entities/content.rs` (naming final
em audit C1):

```rust
Content::CounterDisplayCallback {
    key:      String,
    callback: Option<crate::entities::func::Func>,
}
```

Arms cascata compiler-driven (paridade absoluta P240):
- `Content::PartialEq` arm explícito (Func ptr_eq).
- `Content::plain_text` arm `String::new()`.
- `Content::map_content` / `map_text` arms `self.clone()`.
- `Content::materialize_time` arm `content.clone()` (resolução
  real via `apply_counter_displays` + layout arm).
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

**Counts esperados pós-P241**:
- Content variants: 61 → **62** (+1).
- ElementPayload variants: +1.
- ElementKind variants: +1.

---

## §5 `apply_counter_displays` fixpoint function + Introspector storage + `counter_display_value` method (C4)

`01_core/src/rules/introspect/from_tags.rs` — paridade absoluta
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
                let counter_value: Value = intr.counters.value_at(key, *loc)
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
                    None => {
                        // Sem callback: formato default "1.2.3" via join "."
                        match intr.counters.value_at(key, *loc) {
                            Some(slice) => {
                                let s = slice.iter()
                                    .map(|n| n.to_string())
                                    .collect::<Vec<_>>()
                                    .join(".");
                                Content::text(&s)
                            }
                            None => Content::Empty,
                        }
                    }
                };
                intr.counter_displays.insert((key.clone(), *loc), pre_rendered);
            }
        }
    }
}
```

**Paralelismo absoluto `apply_state_displays`**: mesma signature,
mesma estrutura iteração tags, mesma defensive ignore em Err.
**Diferença mínima**: converte counter slice para `Value::Array`
antes de passar ao callback; fallback sem callback formata
"1.2.3" via join.

Caller `run_fixpoint` em `fixpoint.rs` (paridade P240): adicionar
chamada `apply_counter_displays` após `apply_state_displays` na
mesma sequência pós-walk pré-hash.

`TagIntrospector` em `entities/introspector.rs`:
- novo field `pub counter_displays: HashMap<(String, Location), Content>`
  (paralelo `state_displays` P240).
- novo trait method `counter_display_value(&self, key: String,
  location: Location) -> Option<Content>` (paralelo
  `state_display_value` P240).

---

## §6 Stdlib `native_counter_display` + walk arm + layout arm (C5+C6+C7)

`01_core/src/rules/stdlib/mod.rs`:

```rust
fn native_counter_display(_ctx: &mut EvalContext<'_>, args: &Args) -> SourceResult<Value> {
    let key = match args.items.first() {
        Some(Value::Str(s)) => s.to_string(),
        _ => return Err(error!("counter_display: 1º arg deve ser string (key)")),
    };
    let callback = match args.items.get(1) {
        Some(Value::Func(f)) => Some(f.clone()),
        None                 => None,
        _ => return Err(error!("counter_display: 2º arg opcional deve ser função")),
    };
    Ok(Value::Content(Content::CounterDisplayCallback { key, callback }))
}
```

Registado em `make_stdlib` como `counter_display`. Stdlib funcs:
63 → **64**.

Walk arm em `rules/introspect.rs` para `Content::CounterDisplayCallback`:
- emit tag no topo via `extract_payload` retornando
  `Some(ElementPayload::CounterDisplay { key, callback })`.
- não recurse (terminal).

Layout arm em `rules/layout/mod.rs` (paralelo `Content::StateDisplay`
P240):

```rust
Content::CounterDisplayCallback { key, .. } => {
    if let Some(loc) = self.current_location {
        if let Some(content) = self.introspector.counter_display_value(key.clone(), loc) {
            return self.layout_content(&content, ...);
        }
    }
    // Fallback: nada renderizado
}
```

---

## §7 Critério aceitação P241 (C8+C9+C10)

| Critério | Esperado |
|----------|----------|
| `cargo build --workspace` | verde |
| `cargo test --workspace` | **~2174 verdes** (range 2172-2176; +10-14 vs 2162 baseline P240) |
| `crystalline-lint .` | 0 violations |
| `crystalline-lint --fix-hashes` | 3 L0 hashes + ~7-10 ficheiros L1 actualizados |
| Adaptações pre-existentes | N=0 esperado (pattern paralelo absoluto não-disruptive paridade P240) |
| Content variants | 61 → **62** |
| ElementPayload variants | +1 (CounterDisplay) |
| ElementKind variants | +1 (CounterDisplay) |
| Stdlib funcs | 63 → **64** |
| TagIntrospector fields | +1 (counter_displays) |
| Introspector trait methods | +1 (counter_display_value) |
| ADR-0081 status | IMPLEMENTADO parcial 1/5 → 2/5 |
| ADR-0079 Categoria D | 2/? → **3/?** anotado |
| ADR-0080 §"Excepção P241" | anotada (segunda excepção justificada) |
| L0 partial tocado | ~3 ficheiros |
| Regressões reais | **0** |

**Tests P241** (estimativa ~12 unit + cenários canónicos, paridade
absoluta P240):

**Unit content** (3 tests em `entities/content.rs`):
- `p241_content_counter_display_callback_partial_eq_sem_callback`.
- `p241_content_counter_display_callback_partial_eq_com_callback_ptr_eq`.
- `p241_content_counter_display_callback_plain_text_vazio`.

**Unit stdlib** (4 tests em `rules/stdlib/mod.rs`):
- `p241_native_counter_display_sem_callback_constroi_variant`.
- `p241_native_counter_display_com_callback_constroi_some`.
- `p241_native_counter_display_arg_tipo_errado_rejeita`.
- `p241_native_counter_display_arity_errada_rejeita`.

**Unit introspect/fixpoint** (5 tests em
`rules/introspect/from_tags.rs`):
- `p241_apply_counter_displays_sem_callback_renderiza_formato_default`
  (counter `[1, 2, 3]` → `Content::text("1.2.3")`).
- `p241_apply_counter_displays_com_callback_aplica_func` (callback
  recebe `Value::Array([Int(1), Int(2)])` → retorna `Value::Str`).
- `p241_apply_counter_displays_callback_erro_retorna_content_empty`.
- `p241_apply_counter_displays_locations_diferentes_valores_diferentes`
  (paridade history-aware `value_at`).
- `p241_apply_counter_displays_counter_inexistente_array_vazio`
  (`Value::Array(vec![])` fallback → callback recebe array vazio).

**3 pré-condições obrigatórias verificadas** (per ADR-0081
§"Pré-condições obrigatórias" P239):
1. **Tests baseline preservados**: 2162 verdes pré-P241 → ~2174
   verdes pós-P241 (+10-14; 0 regressões esperadas).
2. **Comemo memoization invariants ADR-0073/0074 preservados**:
   novo method `counter_display_value(String, Location) ->
   Option<Content>` compatível com `#[comemo::track]` (parâmetros
   owned; return type owned, paridade P240).
3. **Backward compat**: `Content::CounterDisplay { kind }` legacy
   preservada inalterada — todos os tests pré-P241 que usam
   variant legacy continuam intactos.

**Promoções ADR esperadas**:
- ADR-0081 IMPLEMENTADO parcial **1/5 → 2/5** (M7+2 ✓; M7+3 a
  M7+5 pendentes). Distribuição ADRs: preservada (sem novos
  ADRs criados; apenas anotação cumulativa).
- ADR-0079 Categoria D 2/? → **3/?** anotado (D.3
  counter.display walk-time real).
- ADR-0080 §"Excepções" entrada P241 anotada (N=2 cumulativo).
- ADR-0066 SUPERSEDED-BY 0073 preservado.

**Inventário 148 footnote ⁶⁰** adicionada (~150 linhas estimadas)
documentando: M7+2 Opção γ materializado paralelo absoluto P240;
lição refinada N=4 cumulativo validada; pattern "refino aditivo
paralelo entre callers fixpoint" N=1 → 2 cumulativo;
D.3 counter.display real materializado primeira vez pós-M9c;
segunda excepção justificada ADR-0080 EM VIGOR.

---

## §8 Próximo sub-passo pós-P241

P241 completa M7+2 (M9d segunda sub-passo). Restantes 3 sub-passos
M9d/M7+ pendentes (magnitude cumulativa restante ~16-25h estimada
per §8 P240 relatório).

| Caminho | Trabalho | Magnitude | Prioridade |
|---------|----------|-----------|------------|
| **M7+5 A.4 radius/clip infrastructure** | `ShapeKind::RoundedRect` + `Corners<T>` + PDF | M-L (~3-5h) | **alta** (geometry isolada; A.4 graded P231 promoção real; ganho user-facing imediato) |
| M7+3 multi-region completion cell-level | `Regions { current, backlog, last }`; candidato fechar DEBT-56b | L+ (~8-12h) | média (desbloqueia C.2 + A.4 breakable per-cell) |
| M7+4 Place float real | Reabertura Opção B P219 graded | L (~5-8h) | média (desbloqueia C.1) |
| ADR meta admin XS | Promoção formal patterns N=3+ acumulados pós-P241 | XS por pattern | baixa-média |
| Pivot outro módulo | Visualize 54%; Text 52%; Model 50% | varia | baixa |
| Pausa M-fase | Fase 5 graded ~85-87% (10 + M7+1 ✓ + M7+2 ✓ = 12/13-15) | XS | baixa |

**Recomendação subjectiva pós-P241**: **M7+5 A.4 radius/clip**.
Menor magnitude restante M-L (~3-5h); geometry isolada (sem
dependências pipeline); ganho user-facing imediato (A.4 graded
P231 promoção real). Alternativa: M7+3 multi-region (desbloqueio
maior C.2+A.4 breakable; magnitude L+).

**Decisão humana fica em aberto literal** pós-P241.

**Estado esperado pós-P241**:
- Tests workspace: 2162 → **~2174 verdes** (+12 P241).
- Content variants: 61 → **62**.
- ElementPayload variants: +1.
- ElementKind variants: +1.
- **Stdlib funcs: 63 → 64** (+counter_display).
- Value variants: 55 preservado.
- §A.5 distribuição: `12/4/2/0/0 = 18` preservada.
- Cobertura Layout per metodologia: **89% preservada** (M7+2 é
  Introspection refino + walk integration; não Layout estrutural).
- Cobertura user-facing total: ~68-70% → **~70-72%** (D.3
  counter.display real bonus).
- **ADRs distribuição**: PROPOSTO 12 preservado; EM VIGOR 29
  preservado; IMPLEMENTADO 22 preservado (ADR-0081 transita 1/5
  → 2/5 internamente; sem ADR novo); total **68 preservado**.
  ADR-0079 Categoria D **3/?** anotada. ADR-0080 §"Excepções"
  P241 anotada.
- **Saldo DEBTs: 11 preservado**.
- **33 aplicações cumulativas anti-inflação** pós-P205D.
- **Patterns emergentes** (paralelos P241):
  - "L0 tocado para features runtime novas + walk integration"
    N=1 → **2 cumulativo** (P240 + P241).
  - "Refino aditivo paralelo entre callers fixpoint" N=1 →
    **2 cumulativo** (P240 + P241).
  - "Spec C1 audit obrigatório bloqueante" N=3 → **4 cumulativo**.
  - "Atomização prep-passo audit-only + materialização-passo"
    preservado N=2 (sem nova aplicação P241).
- **Categoria D Fase 5 Layout: 2/? → 3/? sub-passos
  materializados** (D.1 eval-time wrappers; D.2 walk-time real
  P240; **D.3 counter.display real P241**).
- **Fase 5 Layout candidata: 11/13-15 → 12/13-15 sub-passos
  materializados** (~80-92% cumulativo).
- **M9d / M7+ progresso**: **2/5 sub-passos materializados**
  (M7+1 ✓; **M7+2 ✓**; M7+3 + M7+4 + M7+5 pendentes; cumulativa
  restante ~16-25h).

---

## §9 Notas operacionais para o executor

1. **Audit C1 PRIMEIRO** — não tocar código antes de validar
   empíricamente os 9 aspectos da tabela §2. Lição N=4 cumulativo.

2. **Pattern absoluto paralelo P240** — ler
   `typst-passo-240-relatorio.md` §§3-5 e replicar estrutura
   literal substituindo `state_display` → `counter_display`,
   `state_displays` → `counter_displays`, `StateDisplay` →
   `CounterDisplay`/`CounterDisplayCallback`.

3. **Naming final do variant Content** decidido em audit C1.
   `CounterDisplayCallback` é proposta; alternativas
   `CounterDisplay2`, `CounterDisplayFn` aceitáveis. Critério:
   distinguibilidade clara vs `CounterDisplay` legacy.

4. **Sem `P241.div-N`** antecipado, mas se audit revelar
   bloqueador material (e.g. legacy variant precisa migrar
   antes de coexistência), criar `P241.div-1` formal e parar
   para decisão humana.

5. **L0 partial tocado é excepção justificada** ADR-0080 — anotar
   em `typst-adr-0080-l0-minimal-refactors.md` §"Excepções" como
   N=2 cumulativo pós-P240.

6. **Tests devem incluir cenário history-aware** — `value_at(key,
   loc1)` ≠ `value_at(key, loc2)` para locations distintas com
   mesma key (counter incremental). Paralelo cenário α P240
   state.final.
