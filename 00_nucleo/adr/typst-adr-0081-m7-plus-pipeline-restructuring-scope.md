# ⚖️ ADR-0081: M7+ pipeline restructuring scope — reabertura M-fase pós-M9c para walk-time eval Func dispatch + bloqueadores cumulativos Fase 5 Layout

**Status**: `IMPLEMENTADO parcial` (M7+1 ✓ P240; M7+2 a M7+5 pendentes)
**Data**: 2026-05-14 (PROPOSTO P239; **IMPLEMENTADO parcial P240**)
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
