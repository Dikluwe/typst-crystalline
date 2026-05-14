# ⚖️ ADR-0081: M7+ pipeline restructuring scope — reabertura M-fase pós-M9c para walk-time eval Func dispatch + bloqueadores cumulativos Fase 5 Layout

**Status**: `IMPLEMENTADO parcial` (M7+1 ✓ P240; M7+2 ✓ P241; M7+5 ✓ P242; **M7+3 fase (a) ✓ P243**; M7+3 fase (b) + M7+4 pendentes)
**Data**: 2026-05-14 (PROPOSTO P239; IMPLEMENTADO parcial P240; 2/5 P241; 3/5 P242; **4/5 P243 fase (a)**)
**Autor**: Humano + IA
**Validado**: audit empírico P239 prep-passo audit-only
(`00_nucleo/materialization/typst-passo-239-audit-m7-reabertura.md`);
4 bloqueadores arquiteturais identificados P238 reescrito + 1
bloqueador adicional identificado P239 (radius/clip geometry).
**Reservado para**: meta-documental formaliza escopo M-fase nova
pós-M9c para desbloquear sub-passos Fase 5 Layout pendentes
(D.2 state.display; counter.display; state.final two-pass real;
C.1 Place float real; C.2 Multi-region completion; A.4
radius/clip render real).
**Pré-condição**: P237 concluído (Fase 5 Layout candidata
10/13-15 sub-passos materializados; ADR-0079 PROPOSTO mantido);
P238 reescrito concluído (auditoria metodológica + plano
cobertura Layout; 4 bloqueadores identificados); P239 prep-passo
audit-only concluído (audit empírico + roadmap atomização).

---

## Contexto

Fase 5 Layout candidata (ADR-0079 PROPOSTO) materializou
sucessivamente Categoria A 5/5 ✓ + Categoria B 3/3 ✓ + Categoria
D 1/? refino estendido completo (state_final P236 + state_at
P237; paralelismo state↔counter completo). Restam **5+
sub-passos materialização** bloqueados arquiteturalmente, todos
exigindo refactor pipeline maior:

- **D.2 state.display(callback)** — render-mediated state
  callback durante walk/layout.
- **counter.display(counter, callback)** — paralelo state.display.
- **state.final()** semantic vanilla real two-pass.
- **C.1 Place float real** — reabertura Opção B P219 graded.
- **C.2 Multi-region completion cell-level** — DEBT-56b
  candidato.
- **A.4 outset/radius/clip render real Block+Boxed** — P231
  graded; geometry infrastructure.

P238 reescrito (auditoria metodológica pós-`P238.div-1`)
identificou 4 bloqueadores arquiteturais comuns; P239 prep-passo
audit-only refinou a 5 bloqueadores empíricamente. Audit P239
revelou **achado material**: hipótese P238 reescrito
"`Func::call` não existe" precisa refino — `apply_state_funcs`
JÁ avalia `StateUpdate::Func` em fixpoint loop pós-walk
(`01_core/src/rules/introspect/from_tags.rs:48`). O blocker
real é **layout-time Engine+ctx indisponíveis**, não
walk-time Func dispatch.

---

## Decisão

Reabrir M-fase pós-M9c para refactor pipeline cumulativo
desbloqueando bloqueadores arquiteturais identificados. Nomenclatura
preliminar **M9d** (continuação M9c pattern N=2; pattern "completion
continua M-fase retroactivo" extends) sujeita ratificação humana.

### Escopo

Materializar **5 sub-passos cumulativos** cobrindo 5 bloqueadores
arquiteturais identificados:

| Sub-passo | Escopo | Magnitude estimada | Dependencies | Desbloqueia |
|-----------|--------|--------------------|--------------|-------------|
| **M7+1** | Pipeline walk-time eval via Opção γ: `apply_state_displays` pré-eval em fixpoint paralelo a `apply_state_funcs`; `Content::StateDisplay { key, callback }` variant; walk arm pre-render | **L (~5-8h)** | Nenhuma | D.2 state.display real; state.final two-pass real |
| **M7+2** | counter.display paralelo M7+1 — `Content::CounterDisplay { key, callback }` + `apply_counter_displays` | **M (~2-4h)** | M7+1 (reuso pattern) | counter.display real |
| **M7+3** | Multi-region completion cell-level — extensão `Regions { current, backlog, last }`; refactor `place_cells` + Layouter | **L+ (~8-12h)** | Independente | C.2 Multi-region completa; A.4 breakable per-cell real (P235 graded) |
| **M7+4** | Place float real — reabertura Opção B P219 graded; flow contorna multi-pass OR flow secundário | **L (~5-8h)** | Independente | C.1 Place float real |
| **M7+5** | A.4 radius/clip infrastructure — `ShapeKind::RoundedRect { radii: Corners<Length> }` + `Corners<T>` type paridade `Sides<T>` + exportador PDF paths arredondados | **M-L (~3-5h)** | Independente | A.4 radius + clip render real (Block+Boxed P231 graded) |

**Total cumulativo M7+**: **~23-37h materialização**. Magnitude
paridade M-fases anteriores cumulativas (M7 + M8 + M9 cumulativos
similar).

### Atomização ADR-0036 aplicável

Pattern ADR-0036 (atomização progressiva) estabelecido P224 +
P233 aplica-se a M7+ sub-passos. Cada sub-passo materialização
separado com:

- Spec próprio (Pxxx.md em `00_nucleo/materialization/`).
- Relatório próprio (Pxxx-relatorio.md).
- Decisões C2+ fixadas baseadas em audit empírico per sub-passo.
- Pré-condições obrigatórias §"Pré-condições obrigatórias"
  abaixo verificadas.

---

## Pré-condições obrigatórias

Três pré-condições mandatórias para cada sub-passo materialização
M7+:

### 1. Testes baseline preservados

- **2150 verdes pré-M7+1** baseline pós-P237.
- Cada sub-passo materialização preserva tests existentes.
- Adaptações intencionais N>0 documentadas explicitamente per
  spec.
- Regressões reais N=0 mandatório.

### 2. Comemo memoization invariants ADR-0073/0074 preservados

- Refactor pipeline walk-time **não-pode quebrar memoization
  correctness**.
- `Introspector` trait paridade vanilla com 20+ métodos M9c
  (ADR-0073 ACEITE) preservado.
- Sub-stores trackable F3 (ADR-0074 ACEITE) preservados.
- Caller `fixpoint::run_fixpoint` (P174) com loop convergência
  preservado.
- Verificação: tests memoization existentes 2150 verdes
  preservados.

### 3. Backward compat eval-time

- P236 `state_final` wrapper eval-time continua funcionar (refino
  semantic two-pass real em M7+1 é refactor non-breaking).
- P237 `state_at` wrapper eval-time continua funcionar.
- 4 stdlib funcs counter (P176/P177/P210B/P190X) preservadas.
- Stdlib funcs 62 baseline pós-P237 preservadas.

---

## Dependencies / ordem proposta

- **M7+1** primeiro (recomendação subjectiva P239) — maior
  desbloqueio cumulativo; pattern Opção γ paridade existente
  `apply_state_funcs`.
- **M7+2** após M7+1 (reuso pattern).
- **M7+3, M7+4, M7+5** independentes — ordem subjectiva conforme
  prioridade humana.

Ordem alternativa válida: M7+5 primeiro (menor magnitude;
infraestrutura geometry isolada; ganho user-facing imediato
radius+clip render real).

---

## Não-objectivos

- **Categoria A render adiada graded P231** — outset/radius/clip
  Block+Boxed semantic adiada preservado per ADR-0079; M7+5
  resolve infraestrutura radius/clip, não outset (outset é
  pattern P156G real separado).
- **Outros sub-passos Fase 5 fora M7+** — Categoria A 5/5 + B
  3/3 + D 1/? refino estendido completo já materializado
  P227-P237.
- **Performance/optimização refactor** — escopo M7+ é
  correctness + funcionalidade apenas; performance refactor
  separado candidato pós-M7+.
- **Reabrir ADR-0066 SUPERSEDED-BY 0073** — chain terminal
  preservado.
- **Show rules mecanismo recursivo** — Opção δ §3.2 audit P239
  rejeitada per coerência arquitectural (Layouter puro sem
  Engine+ctx preservado).
- **Promoção formal patterns emergentes** — promoções formais
  separadas candidatos XS pós-M7+.

---

## Alternativas consideradas

### Alternativa A — M7+ refactor monolítico (rejeitada)

Single sub-passo cobrindo todos os 5 bloqueadores. Magnitude
cumulativa XL+ (~23-37h) não-atomizada; risco grande de
quebrar tests baseline; sem capacidade pausa per sub-passo.
**Rejeitada** per pattern ADR-0036 atomização.

### Alternativa B — ADR-0073/0074 chain extensão suficiente (rejeitada)

Reabrir ADR-0073 (comemo introspector) ou ADR-0074 (F3 layouter
substores trackable) para incluir M7+ escopo. **Rejeitada** por
incoerência narrativa — chain ADR-0073/0074 fechada P205B+C+E;
M7+ scope é nova M-fase pós-M9c ortogonal.

### Alternativa C — Estender ADR-0079 escopo (rejeitada)

Incorporar M7+ scope em ADR-0079 (Layout Fase 5 roadmap).
**Rejeitada** por incoerência semantic — ADR-0079 é Fase 5
Layout candidata user-facing sub-passos materialização; M7+ é
M-fase arquitectural ortogonal cujo output é desbloquear Fase 5
sub-passos pendentes.

### Alternativa D — Pivot outro módulo sem M7+ (alternativa válida)

Pausa M7+; pivot Visualize (54%); Text (52%); Model (50%).
Fase 5 Layout candidata fecha graded a ~80% preservando
bloqueadores como scope-out documentado. **Alternativa válida**
— decisão humana pendente per recomendação P239 §7.

### Alternativa E — Opção α walk-time eval (massivo refactor) — preterida

Pass `Engine + ctx` para Layouter signature. Magnitude L+; risco
quebrar comemo invariants ADR-0073/0074. **Preterida** face a
Opção γ (audit P239 §3.2) — Opção γ paridade pattern existente
`apply_state_funcs`, baixo risco, magnitude L.

### Alternativa F — Opção β two-pass walk completo — preterida

Walk pass 1 emite tags; walk pass 2 re-entra eval para display
callbacks. Magnitude XL+; signature refactor cumulativo. **Preterida**
face a Opção γ por mesma razão.

### Alternativa G — Opção δ Show rule mecanismo recursivo — preterida

`state.display(fn)` synthetic Show rule re-entrando eval pipeline.
Magnitude L+; refactor Show rules pipeline. **Preterida** por
incoerência arquitectural (Cristalino pós-M9c não tem Show rules
recursivo paralelo vanilla).

---

## Decisões pendentes (sob-decisões formalizadas)

### D1 — Nomenclatura M-fase

3 opções identificadas P239 §3.1:

| Opção | Nomenclatura | Coerência                                                              |
|-------|--------------|------------------------------------------------------------------------|
| α     | **M9d**      | Continuação M9c pattern N=2 — completion continua para walk-time real  |
| β     | M10          | Marco verdadeiramente novo (mas pipeline restructuring continua M9-line) |
| γ     | M-pipeline   | Nome semanticamente claro; sem numeração                               |

**Recomendação subjectiva — Opção α (M9d)**: extends pattern
M9c. Decisão humana pendente formaliza per ratificação ADR-0081.

### D2 — Ordem primeira materialização

Recomendação P239 §7: M7+1 primeiro. Alternativa válida: M7+5
primeiro (menor magnitude; geometry isolada).

### D3 — Promoção pós-M7+

Pós-conclusão M7+ refactor cumulativo:

- ADR-0081 PROPOSTO → IMPLEMENTADO (se todos 5 sub-passos
  materializados).
- ADR-0079 PROPOSTO → IMPLEMENTADO (se todos sub-passos Fase 5
  Layout candidata materializados, incluindo desbloqueados
  pós-M7+).

---

## Reaberturas arquiteturais (registo explícito)

### Reabertura 1 — Opção B P219 graded (Sub-passo M7+4)

- **Decisão original P219**: column flow Opção B graded
  (single-region width reduzida; pattern N=5 "Field armazenado
  semantic adiada").
- **M7+4 materialização**: Place float real flow contorna
  (multi-pass layout ou flow secundário topo/fundo).
- **Nota**: Opção B P219 preservada literal para column flow;
  M7+4 introduz flow ortogonal (Place float ≠ column flow).
  Sem conflito directo arquitectural.

### Reabertura 2 — P216B Regions minimal (Sub-passo M7+3)

- **Decisão original P216B**: `Regions { current: Region }`
  minimal; `backlog`/`last` diferidos.
- **M7+3 materialização**: `Regions { current, backlog, last }`
  completo (multi-region real para cell rowspan cross-pagination).
- **Nota**: P216B preservada literal como baseline histórica;
  M7+3 estende para Opção A real.

### Reabertura 3 — DEBT-56 ENCERRADA (DEBT-56b candidato)

- **DEBT-56 P221**: ENCERRADA literal (CLOSED via materialização
  Opção B graded).
- **M7+3 materialização**: introduz **DEBT-56b novo** para
  "refino Opção A multi-region cell-level pós-fecho DEBT-56".
- **Nota**: DEBT-56 preservada CLOSED literal; DEBT-56b novo
  aberto se/quando M7+3 materializar.

### Reabertura 4 — `apply_state_funcs` pattern (Sub-passo M7+1+M7+2)

- **Decisão original P191B (ADR-0071)**: `apply_state_funcs`
  slim post-pass para `StateUpdate::Func` apenas.
- **M7+1 materialização**: `apply_state_displays` paralelo para
  `state.display` callbacks (Opção γ audit P239 §3.2).
- **M7+2 materialização**: `apply_counter_displays` paralelo.
- **Nota**: ADR-0071 preservada literal; M7+1+M7+2 extendem
  pattern paralelo. Comemo invariants ADR-0073/0074 preservados
  per pré-condição §"Pré-condições obrigatórias" item 2.

---

## Status

**`PROPOSTO`** — autorização arquitectural concedida em princípio;
sub-passos materialização M7+1+ ficam abertos para decisão humana
caso-a-caso. **Política "sem novas reservas" P158 preservada
literal** — 5 sub-passos identificados mas NÃO reservados.

Sub-passo materializado pós-P239: **0** (M7+ refactor pendente
decisão humana primeira materialização).

---

## Aplicações cumulativas pós-P239

### P240 anotação — M7+1 Pipeline walk-time eval via Opção γ `apply_state_displays` materializado; primeira sub-passo M9d materializado; primeira excepção justificada à aplicação automática ADR-0080 EM VIGOR pós-P229

**Data**: 2026-05-14.

**Primeira sub-passo materialização M9d / M7+ pós-P239
prep-passo audit-only** — primeira aplicação real do pattern
"atomização prep-passo audit-only + materialização-passo"
inaugurado P238 reescrito (N=1 → 2 cumulativo).

**P240 materializa M7+1 Opção γ** (per audit P239 §3.2):

- **`Content::StateDisplay { key, callback: Option<Func> }`**
  variant novo em `entities/content.rs`. Content variants:
  60 → **61** (+StateDisplay).
- **`ElementPayload::StateDisplay { key, callback }`** variant
  novo em `entities/element_payload.rs` (não `Tag::StateDisplay`
  directo — audit C1 P240 revelou Tag enum é `Tag::Start(Location,
  ElementInfo)` com payload via ElementInfo; ajuste signature
  trivial sem `P240.div-N`).
- **`ElementKind::StateDisplay`** variant novo em
  `entities/element_kind.rs`.
- **`apply_state_displays`** fixpoint function nova em
  `rules/introspect/from_tags.rs:80+` — paralelo absoluto
  `apply_state_funcs` P191B; chama `apply_func(callback,
  [value], ctx, engine)` pós-walk com Engine+ctx disponíveis.
- **`Introspector::state_display_value(key, location)`** trait
  method novo em `entities/introspector.rs` + impl em
  `TagIntrospector` + adapter em
  `03_infra/src/measurements.rs::CountingIntrospector`.
- **`TagIntrospector.state_displays:
  HashMap<(String, Location), Content>`** storage novo.
- **`native_state_display(key, [callback])`** stdlib func nova
  em `rules/stdlib/foundations.rs` + scope register em
  `rules/eval/mod.rs:618` + re-export em `rules/stdlib/mod.rs`.
  Stdlib funcs: 62 → **63** (+state_display).
- **Walk integration layout-time arm `Content::StateDisplay`**
  em `rules/layout/mod.rs:355+` — consome via
  `Introspector::state_display_value`; Layouter permanece puro
  (sem Engine+ctx em signature).
- **`extract_payload` arm StateDisplay** em
  `rules/introspect/extract_payload.rs` emite Tag pós-walk.
- **`populate_intr_from_tag_start` arm StateDisplay** em
  `rules/introspect.rs` regista loc em kind_index.
- **Caller** `apply_state_displays(&tags, &mut introspector,
  engine, ctx)` em `fixpoint::run_fixpoint` após
  `apply_state_funcs`.

**Achado material audit C1 P240 (cenário α confirmed para
state.final)**: `state_final_value` baseline delega a
`state.final_value(key)` que retorna `history.last()`. Após
`apply_state_funcs` em fixpoint, `history` reflete valor final
two-pass real cumulativo. **`state_final` semantic já é
two-pass real pós-P240** — paridade vanilla `state.final()`
sem refactor adicional (refino docs apenas; cenário α audit
C7 spec).

**Sobreposição bloqueador A + D desbloqueada via M7+1
sozinho**: walk-time eval Func dispatch + state.final two-pass
real ambos resolvidos via Opção γ paralelo `apply_state_funcs`
existente.

**Pré-condições obrigatórias verificadas P240**:
1. **Tests baseline preservados**: 2150 verdes pré-P240 → **2162
   verdes** (+12 novos P240; 0 regressões; 0 adaptações).
2. **Comemo memoization invariants ADR-0073/0074 preservados**:
   trait `Introspector` `#[comemo::track]` continua válido;
   `state_display_value(String, Location) -> Option<Content>`
   compatível com track macro; sub-stores trackable F3
   preservados.
3. **Backward compat eval-time**: P236 `state_final` wrapper
   eval-time + P237 `state_at` wrapper continuam funcionar
   inalterados (zero modificações em foundations.rs além de
   docs refino state_final + new native_state_display).

**8 decisões fixadas P240** (Decisão 0 = lição N=3 cumulativo
P237 + P238 reescrito + P240):
- Decisão 0 — C1 audit obrigatório bloqueante (lição refinada
  `P236.div-1` → `P238.div-1` → P239 audit aplicada);
  pattern N=2 → **3 cumulativo**.
- Decisão 1 — Opção γ confirmada (apply_state_displays
  paralelo apply_state_funcs).
- Decisão 2 — Opção β Content::StateDisplay variant novo
  (não α refino Content::State coerência).
- Decisão 3 — Opção α refinada empíricamente: **`ElementPayload::
  StateDisplay`** (não `Tag::StateDisplay` directo per audit C1
  P240 revelou Tag enum estructura).
- Decisão 4 — Opção β paralelismo absoluto apply_state_funcs.
- Decisão 5 — Walk integration layout-time via Introspector
  trait method (Layouter puro preservado).
- Decisão 6 — native_state_display 1-2 arg conforme spec.
- Decisão 7 — Cenário α confirmado: state.final two-pass real
  trivial (docs apenas; sem refactor adicional;
  `state_final_value` baseline já two-pass real pós-fixpoint).
- Decisão 8 — **L0 partial tocado** (3 ficheiros) — **primeira
  excepção justificada à aplicação automática ADR-0080 EM
  VIGOR pós-P229**; pattern emergente "L0 tocado para features
  runtime novas + walk integration" N=1 inaugurado P240
  (ADR-0080 §"Excepção P240" documentada formalmente).

**Patterns emergentes inaugurados/consolidados em P240** (4):

- **"L0 tocado para features runtime novas + walk integration"
  N=1 inaugurado P240** — primeira aplicação real (P236 spec
  original hipotetizou; rejeitada empíricamente pós-divergência).
- **"refino aditivo paralelo entre callers fixpoint" N=1
  inaugurado P240** — extensão pattern P171/M9
  `apply_state_funcs` baseline para
  `apply_state_displays`.
- **"spec C1 audit obrigatório bloqueante pós-`P236.div-1`"**
  N=2 → **3 cumulativo** (P237 + P238 reescrito + P240).
- **"atomização prep-passo audit-only + materialização-passo"**
  N=1 → **2 cumulativo** (P238 reescrito → P239; P239 → P240
  validação empírica).

**Resultado P240**:
- Tests workspace: 2150 → **2162 verdes** (+12).
- Violations: 0 preservadas.
- Content variants: 60 → **61** (+StateDisplay).
- ElementPayload variants: +1 (StateDisplay).
- ElementKind variants: +1 (StateDisplay).
- Stdlib funcs: 62 → **63** (+state_display).
- TagIntrospector fields: +1 (state_displays storage).
- Introspector trait methods: +1 (state_display_value).
- L0 prompts tocados partial: 3 ficheiros (content.md,
  rules/stdlib.md, rules/introspect.md) — **primeira excepção
  ADR-0080 justificada**.
- ADR-0080 §"Excepção P240" anotada formalmente.
- ADR-0079 Categoria D 1/? → 2/? sub-passos materializados.

**M9d / M7+ progresso**: **1/5 sub-passos materializados**
(M7+1 ✓; M7+2 + M7+3 + M7+4 + M7+5 pendentes).

**Status ADR-0081**: PROPOSTO → **IMPLEMENTADO parcial**.
Promoção PROPOSTO → IMPLEMENTADO completo só pós M7+2 a M7+5
cumulativos materializados.

**Próximo sub-passo candidato pós-P240**:
- **M7+2 counter.display paralelo M7+1** (recomendação
  subjectiva; magnitude M ~2-4h; reuso pattern absoluto;
  completa Categoria D 2/? → 3/? real).
- **M7+5 A.4 radius/clip infrastructure** (alternativa
  subjectiva; menor magnitude M-L ~3-5h; geometry isolada).
- M7+3 multi-region completion cell-level (L+ ~8-12h).
- M7+4 Place float real (L ~5-8h).
- Pivot outro módulo OR pausa M-fase.

Decisão humana pendente literal pós-P240.

### P241 anotação — M7+2 counter.display walk-time eval via Opção γ `apply_counter_displays` paralelo absoluto P240 M7+1; D 2/? → 3/?; IMPLEMENTADO parcial 1/5 → 2/5

**Data**: 2026-05-14.

**Segunda sub-passo materialização M9d / M7+** — paralelo
absoluto P240 M7+1 substituindo `state_display` por
`counter_display`. Pattern "refino aditivo paralelo entre
callers fixpoint" N=1 → **2 cumulativo** (P240 + P241).

**P241 materializa M7+2 Opção γ** (per ADR-0081 IMPLEMENTADO
parcial M7+1 P240 + spec P241 §4-§6):

- **`Content::CounterDisplayCallback { key, callback: Option<Func> }`**
  variant novo (distinto de `Content::CounterDisplay { kind }`
  legacy single-pass que coexiste preservada — Decisão 1 P241
  Opção α naming explícito). Content variants: 61 → **62**.
- **`ElementPayload::CounterDisplay { key, callback }`** variant
  novo paralelo `ElementPayload::StateDisplay` P240.
- **`ElementKind::CounterDisplay`** variant novo + "counter_display"
  as_str/from_name.
- **`apply_counter_displays`** fixpoint function nova em
  `rules/introspect/from_tags.rs` — paralelo absoluto
  `apply_state_displays` P240. Converte
  `intr.counters.value_at(key, loc)` (Option<&[usize]>) para
  `Value::Array(Vec<Value::Int>)` (paridade vanilla
  CounterState = SmallVec<[u64; 3]>) e aplica callback via
  `apply_func`.
- **`Introspector::counter_display_value(key, location) ->
  Option<Content>`** trait method novo + impl em TagIntrospector
  + CountingIntrospector adapter.
- **`TagIntrospector.counter_displays:
  HashMap<(String, Location), Content>`** storage novo.
- **`native_counter_display(key, [callback])`** stdlib func nova.
  Stdlib funcs: 63 → **64**.
- **Walk integration layout-time arm
  `Content::CounterDisplayCallback`** consome Content
  pre-rendered via `counter_display_value`. Layouter permanece
  puro (paridade arquitectural P240).
- **Caller** `apply_counter_displays` em
  `fixpoint::run_fixpoint` após `apply_state_displays`.

**Forma do Value passado ao callback** (Decisão 4 P241):
`Value::Array(Vec<Value::Int>)` representando counter state
actual. Counter inexistente: `Value::Array(vec![])`. Sem
callback: formato default "1.2.3" via join "."; counter
inexistente: `Content::Empty`.

**Pré-condições obrigatórias verificadas P241** (per ADR-0081
§"Pré-condições obrigatórias"):
1. **Tests baseline preservados**: 2162 verdes pré-P241 → **2175
   verdes pós-P241** (+13 novos; 0 regressões; 0 adaptações).
2. **Comemo memoization invariants ADR-0073/0074 preservados**:
   trait `Introspector` `#[comemo::track]` continua válido com
   novo method `counter_display_value(String, Location) ->
   Option<Content>` compatível com macro (paridade P240).
3. **Backward compat**: `Content::CounterDisplay { kind }` legacy
   preservada inalterada; todos os tests pré-P241 que usam
   variant legacy continuam intactos.

**8 decisões fixadas P241** (Decisão 0 = lição N=4 cumulativo
P237 + P238 reescrito + P240 + P241):
- Decisão 0 — C1 audit obrigatório bloqueante (N=4 cumulativo;
  audit refinou naming `CounterDisplayCallback`).
- Decisão 1 — Opção α variant nova paralela (não β refino legacy).
- Decisão 2 — ElementPayload::CounterDisplay paralelo
  StateDisplay.
- Decisão 3 — ElementKind::CounterDisplay paralelo.
- Decisão 4 — Value::Array para counter state (paridade vanilla
  CounterState).
- Decisão 5 — Counter inexistente → Value::Array(vec![]) +
  Content::Empty sem callback.
- Decisão 6 — `native_counter_display` 1-2 arg paridade
  `native_state_display`.
- Decisão 7 — **L0 partial tocado** (3 ficheiros) — **segunda
  excepção justificada à aplicação automática ADR-0080 EM
  VIGOR pós-P229**; pattern N=1 → 2 cumulativo.
- Decisão 8 — Tests materializados no mesmo passo (sem stubs
  diferidos).

**Patterns emergentes inaugurados/consolidados em P241** (3):

- **"L0 tocado para features runtime novas + walk integration"**
  N=1 → **2 cumulativo** (P240 + P241).
- **"Refino aditivo paralelo entre callers fixpoint"** N=1 →
  **2 cumulativo** (P240 `apply_state_displays` + P241
  `apply_counter_displays`).
- **"Spec C1 audit obrigatório bloqueante pós-`P236.div-1`"**
  N=3 → **4 cumulativo** (P237 + P238 reescrito + P240 + P241).

**Resultado P241**:
- Tests workspace: 2162 → **2175 verdes** (+13; spec previa
  +10-14).
- Violations: 0 preservadas.
- Content variants: 61 → **62** (+CounterDisplayCallback).
- ElementPayload variants: +1 (CounterDisplay).
- ElementKind variants: +1 (CounterDisplay).
- Stdlib funcs: 63 → **64** (+counter_display).
- TagIntrospector fields: +1 (counter_displays storage).
- Introspector trait methods: +1 (counter_display_value).
- L0 prompts tocados partial: 3 ficheiros — **segunda excepção
  ADR-0080 justificada** documentada formalmente.
- ADR-0079 Categoria D 2/? → **3/?** anotado.

**M9d / M7+ progresso**: **2/5 sub-passos materializados**
(M7+1 ✓ P240; **M7+2 ✓ P241**; M7+3 + M7+4 + M7+5 pendentes
— cumulativa restante ~16-25h).

**Status ADR-0081**: IMPLEMENTADO parcial 1/5 → **2/5**.
Promoção PROPOSTO → IMPLEMENTADO completo só pós M7+3 a M7+5
cumulativos materializados. **Distribuição ADRs preservada**:
sem novos ADRs criados; sem transições PROPOSTO ↔ IMPLEMENTADO
adicionais P241 (apenas anotação cumulativa interna 1/5 → 2/5).

**Próximo sub-passo candidato pós-P241**:
- **M7+5 A.4 radius/clip infrastructure** (recomendação
  subjectiva spec P241 §8; menor magnitude restante M-L
  ~3-5h; geometry isolada).
- M7+3 multi-region completion cell-level (L+ ~8-12h).
- M7+4 Place float real (L ~5-8h).
- ADR meta admin XS (promoção formal patterns N=2 cumulativos
  P240+P241).
- Pivot outro módulo OR pausa M-fase.

Decisão humana pendente literal pós-P241.

### P242 anotação — M7+5 A.4 radius/clip infrastructure materializado; promoção real graded scope-out P156G/H P231 → semantic concreta; D 3/? preservado, A.4 transita scope-out → materializado parcial; IMPLEMENTADO parcial 2/5 → 3/5

**Data**: 2026-05-14.

**Terceira sub-passo materialização M9d / M7+** — **primeira
sub-passo M7+ não-pipeline** (P240/P241 foram walk-time refactor;
P242 é geometry isolada). Materializa M7+5 per spec P242: refino
tipo `Content::Block.radius` + `Content::Boxed.radius` per-corner
+ `ShapeKind::RoundedRect` novo + PDF exporter Bezier 4 corners
+ Layouter emite `FrameItem::Group` com clip_mask.

**P242 materializa M7+5** (per ADR-0081 IMPLEMENTADO parcial
M7+1 P240 + M7+2 P241 + spec P242):

- **`Corners<T>`** tipo entity novo em
  `01_core/src/entities/corners.rs` (paralelo absoluto `Sides<T>`
  P156C). Sub-padrão #14 "Tipo entity em ficheiro próprio" N=5
  → **6 cumulativo** (Sides → Parity → Dir → BibEntry →
  CitationForm → **Corners**).
- **`ShapeKind::RoundedRect { radii: Corners<Length> }`** variant
  novo em `entities/geometry.rs`. ShapeKind variants: 4 → **5**.
- **Refino tipo `Content::Block.radius`** `Option<Length>` →
  `Corners<Length>` per-corner. Audit C1 P242 refinou hipótese
  spec (spec assumiu "5 fields → 7" mas Block/Boxed já tinham
  8 fields P231; ajuste real = refine field type). **Sem
  `P242.div-N`** — ajuste paridade lição N=5 cumulativo.
- **Refino tipo `Content::Boxed.radius`** idem paralelo.
- **`extract_corners_length_value`** helper novo em
  `rules/stdlib/layout.rs` (paralelo `extract_sides_lengths`
  P156L; sub-padrão "Reuso template helpers extract_*" N=3 →
  **4 cumulativo**).
- **stdlib `block(radius:)` + `box(radius:)`** aceitam Length
  uniforme OR Dict por canto (top-left/top-right/bottom-right/
  bottom-left/top/bottom/left/right/rest; precedência específico
  > eixo > rest).
- **Layouter Block arm**: `clip == true` emite
  `FrameItem::Group { clip_mask: Some(ShapeKind::RoundedRect
  { radii: radius }) }` (radius non-zero) OR
  `clip_mask: Some(ShapeKind::Rect)` (radius zero; paridade
  DEBT-30 P79). `clip == false` preserva inline behavior original.
- **PDF exporter** `emit_rounded_rect_ops` helper novo desenha
  Bezier 4 corners path em 5 sítios cross-arm (Shape global +
  Shape local 2× + Group clip_mask path 2×). Kappa
  `0.552_284_749_831` paridade ShapeKind::Ellipse mesmo ficheiro.

**Promoção real graded ADR-0054 P156G/H → semantic concreta P242**:

- P156G/P156H declararam `radius` + `clip` scope-out com rejeição
  hard em stdlib.
- P231 promoveu para fields graded ("semantic adiada"): `radius:
  Option<Length>` + `clip: bool` aceites em stdlib mas sem render
  real.
- **P242 materializa semantic real**: `radius` refinada per-corner;
  `clip` emite clip_mask via Layouter; PDF exporter desenha Bezier
  path. Sub-padrão emergente **"promoção real scope-out ADR-0054
  graded" N=1 inaugurado P242** — distinto de refinos qualitativos
  ou cosméticos.

**Categoria A.4 ADR-0079** transita scope-out P231 →
**materializado parcial P242** (radius + clip ✓; outset + fill +
stroke restantes N=3 permanecem scope-out P156G/H).

**Pré-condições obrigatórias verificadas P242**:
1. Tests baseline preservados: **2175 → 2190 verdes** (+15 novos;
   0 regressões reais; 7 adaptações triviais tests pré-existentes
   P231 que usavam `radius: Some(len)` → `radius:
   Corners::uniform(len)`).
2. Comemo memoization invariants ADR-0073/0074 preservados —
   P242 NÃO toca trait Introspector nem methods (refino
   geometry isolada).
3. Backward compat: stdlib `block(radius: 5pt)` continua a
   funcionar (Length uniforme via `Corners::uniform`); tests
   P231 adaptados via `Corners::uniform(Length::ZERO/pt(N))`.

**9 decisões fixadas P242** (Decisão 0 = lição N=5 cumulativo):
- Decisão 0 — C1 audit obrigatório bloqueante (N=5 cumulativo;
  audit refinou hipótese fields).
- Decisão 1 — `Corners<T>` paralelo absoluto `Sides<T>`.
- Decisão 2 — `ShapeKind::RoundedRect { radii: Corners<Length> }`.
- Decisão 3 — Refino tipo radius (não add).
- Decisão 4 — Opção α radius accepts Length OR Dict.
- Decisão 5 — clip Bool com semantic materializada (clip_mask emit).
- Decisão 6 — radius sem clip armazenado mas sem clip_mask emit
  (semantic radius isolada continua graded).
- Decisão 7 — L0 partial tocado (terceira excepção ADR-0080
  sub-categoria geometry/exporter).
- Decisão 8 — Promoção real graded scope-out (sub-padrão N=1).
- Decisão 9 — Sem fechamento Fase 5 graded (M7+3/M7+4 pendentes).

**Patterns emergentes inaugurados/consolidados em P242** (4):

- **"Promoção real scope-out ADR-0054 graded" N=1 inaugurado**
  — sub-padrão novo distinto de refinos qualitativos/cosméticos.
- **"Tipo entity em ficheiro próprio" (sub-padrão #14)** N=5 →
  **6 cumulativo** (Corners adiciona-se a Sides/Parity/Dir/
  BibEntry/CitationForm).
- **"Reuso template helpers extract_*"** N=3 → **4 cumulativo**
  (extract_corners_length_value via extract_sides_lengths
  template).
- **"Spec C1 audit obrigatório bloqueante pós-P236.div-1"** N=4
  → **5 cumulativo** (P237 + P238 reescrito + P240 + P241 +
  P242).

**Resultado P242**:
- Tests workspace: 2175 → **2190 verdes** (+15).
- Violations: 0 preservadas.
- Content variants: 62 preservado (refino field-add não-add).
- ShapeKind variants: 4 → **5** (+RoundedRect).
- Block fields: 8 preservado (radius refinado type).
- Boxed fields: 8 preservado (radius refinado type).
- Tipos entity novos: **+1 Corners<T>**.
- Stdlib funcs: 64 preservado.
- Helpers stdlib novos: **+1 `extract_corners_length_value`**.
- L0 prompts tocados partial: 4 ficheiros (corners.md NOVO +
  geometry.md + content.md + export.md).
- ADR-0079 Categoria A.4 scope-out P231 → **materializado parcial
  P242** anotado.
- ADR-0080 §"Excepção P242" anotada (terceira excepção N=3 sub-
  categoria geometry/exporter).

**M9d / M7+ progresso**: **3/5 sub-passos materializados**
(M7+1 ✓ P240; M7+2 ✓ P241; **M7+5 ✓ P242**; M7+3 + M7+4
pendentes — cumulativa restante ~13-20h).

**Status ADR-0081**: IMPLEMENTADO parcial 2/5 → **3/5**.

**Próximo sub-passo candidato pós-P242**:
- **M7+3 multi-region completion cell-level** (recomendação
  subjectiva spec P242 §8; magnitude L+ ~8-12h; maior desbloqueio
  cumulativo restante — C.2 + A.4 breakable per-cell).
- M7+4 Place float real (L ~5-8h; isolada; desbloqueia C.1).
- Refino A.4 — outset/fill/stroke em Block/Boxed (S-M por attr).
- ADR meta admin XS (promoção patterns N=2-4 acumulados).
- Pivot outro módulo OR pausa M-fase.

Decisão humana pendente literal pós-P242.

### P243 anotação — M7+3 fase (a) infrastructure: `Regions { backlog, last }` extensão + promoção real ≥3 scope-outs multi-region; **primeira sub-passo M7+ não-pipeline #2**; IMPLEMENTADO parcial 3/5 → 4/5

**Data**: 2026-05-14.

**Quarta sub-passo materialização M9d / M7+** — fase (a) do plano
duas-fases DEBT-56 §"Notas" ("introduzir `Regions { current,
backlog, last }` mantendo comportamento single-region"). Fase (b)
DEBT-56 (`Content::Columns` + `Content::Colbreak` + consumer
multi-column) pendente para passo subsequente.

**P243 materializa M7+3 fase (a)** (per ADR-0081 IMPLEMENTADO
parcial 3/5 + spec P243):

- **Extensão `Regions` struct** em `01_core/src/entities/region.rs`:
  - `pub backlog: Vec<Region>` field novo (fase (b) populated).
  - `pub last: Option<Region>` field novo (fase (b) populated).
  - `pub fn advance(&mut self) -> Option<Region>` method novo.
- **Promoção real ≥3 scope-outs** via `regions.current.width`
  save/restore em `01_core/src/rules/layout/mod.rs`:
  - `Pad.right` scope-out P156C → semantic real P243 (`regions.current.width -= right` durante body).
  - `Block.width` semantic adiada P156G → semantic real P243
    (`regions.current.width = (line_start + w_pt)` durante body).
  - `Boxed.width` semantic adiada P156H → semantic real P243
    (paralelo Block via `cursor_x + w_pt`).

**Achado material audit C1 P243**: spec hipotetizou refactor
profundo cross-module L+ (5-7 fields migrar + ~30-50 sítios
adaptação). Reality empírica: refactor field-agregation **já feito
em P216A + P216B** (Region struct + Regions wrapper + Layouter
field). P243 reduz para extensão `Regions` com `backlog`/`last`
+ promoção scope-outs. **Magnitude real M (~2-3h)** face L+
(~8-12h) hipotetizado. **Sem `P243.div-N`** — paridade lição
N=6 cumulativo precedente P237/P240/P241/P242.

**Pré-condições obrigatórias verificadas P243**:
1. Tests baseline preservados: **2190 → 2198 verdes** (+8 novos;
   0 regressões reais; **0 adaptações** — extensão aditiva).
2. Comemo memoization invariants ADR-0073/0074 preservados —
   P243 NÃO toca trait Introspector nem methods.
3. Backward compat: stdlib `block(width: 100pt)` continua a
   funcionar (semantic agora real); tests pré-P243 que usavam
   Block.width como scope-out preservados (não-disruptive).

**10 decisões fixadas P243** (Decisão 0 = lição N=6 cumulativo):
- Decisão 0 — C1 audit obrigatório bloqueante (N=6 cumulativo;
  audit refinou hipótese fields já-aggregados).
- Decisão 1 — `Regions` extensão (paralelo conceptual
  `LayouterRuntimeState` P190C).
- Decisão 2 — Migração field-by-field do Layouter — **já feita
  P216A/B** (audit C1 finding).
- Decisão 3 — Fase (a) preserva single-region observable literal.
- Decisão 4 — Promoção real ≥3 scope-outs (Pad.right + Block.width
  + Boxed.width).
- Decisão 5 — Sem `Content::Columns`/`Colbreak` em P243 (fase (b)
  pendente).
- Decisão 6 — Sem ADR dedicada column flow em P243 (fase (b)).
- Decisão 7 — `cell_available_h` integration diferida (passo
  futuro NÃO reservado).
- Decisão 8 — Nova sub-categoria ADR-0080 "Layouter internal
  refactor".
- Decisão 9 — Tests focam preservação observable.
- Decisão 10 — Sem fechamento Fase 5 / ADR-0061 / DEBT-56.

**Patterns emergentes inaugurados/consolidados em P243** (4):

- **"Refactor profundo Layouter internal" N=1 inaugurado P243**
  — sub-padrão novo (mas magnitude real reduzida por P216A/B
  precedente).
- **"Sub-categoria ADR-0080 nova" N=2 → 3 cumulativo** (walk-time
  P240+P241; geometry/exporter P242; **Layouter internal
  refactor P243**).
- **"Promoção real scope-out ADR-0054 graded"** N=1 → **2
  cumulativo** (P242 radius/clip + **P243 multi-region attrs
  Pad.right + Block.width + Boxed.width**).
- **"Spec C1 audit obrigatório bloqueante pós-P236.div-1"** N=5
  → **6 cumulativo** (P237 + P238 reescrito + P240 + P241 +
  P242 + P243).

**Resultado P243**:
- Tests workspace: 2190 → **2198 verdes** (+8).
- Violations: 0 preservadas.
- Content variants: 62 preservado.
- ShapeKind variants: 5 preservado.
- Layouter fields: preservados (migração já feita P216A/B).
- **Regions fields**: 1 → **3** (+backlog +last).
- **Regions methods**: +1 `advance`.
- **Scope-outs promovidos**: 3 (Pad.right + Block.width + Boxed.width).
- Tipos entity novos: 0 (Regions já existia; P243 estende).
- Stdlib funcs: 64 preservado.
- L0 prompts tocados partial: 2 ficheiros (`region.md` extensão +
  `content.md` secção promoção scope-outs).
- ADR-0079 Categoria A.4 preservada parcial pós-P242.
- ADR-0080 §"Excepção P243" anotada (N=4 cumulativo sub-categoria
  nova "Layouter internal refactor").

**M9d / M7+ progresso**: **4/5 sub-passos materializados** (M7+1
✓ P240; M7+2 ✓ P241; **M7+3 fase (a) ✓ P243**; M7+5 ✓ P242;
M7+3 fase (b) + M7+4 pendentes).

**Status ADR-0081**: IMPLEMENTADO parcial 3/5 → **4/5**.

**DEBT-56 §"Plano" checklist** anotado pós-P243:
- ✓ "Refactor minimal `Layouter` para multi-region" — P243 fase
  (a) (extensão Regions backlog+last).
- ✗ ADR dedicada column flow — fase (b) pendente.
- ✗ `Content::Columns` + `Content::Colbreak` — fase (b) pendente.
- ✗ `native_columns` + `native_colbreak` — fase (b) pendente.
- ✗ Layouter consumer multi-column — fase (b) pendente.
- ✗ Tests + inventário 148 + DEBT fecho — fase (b) pendente.

**Próximo sub-passo candidato pós-P243**:
- **M7+3 fase (b)** (recomendação subjectiva; magnitude L ~5-8h;
  fecha DEBT-56 + completa M7+3 + promove potencialmente
  ADR-0061 → IMPLEMENTADO).
- M7+4 Place float real (L ~5-8h; isolada).
- Cell layout migration → `regions.current.height` (M ~2-4h;
  activa A.4 breakable per-cell — Decisão 7 P243 diferida).
- Refino A.4 outset/fill/stroke (S-M).
- ADR meta admin XS (promoção patterns N=2-4 acumulados).
- Pivot outro módulo OR pausa M-fase.

Decisão humana pendente literal pós-P243.
