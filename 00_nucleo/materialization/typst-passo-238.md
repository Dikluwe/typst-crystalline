# Passo 238 (reescrito) — Auditoria metodológica falhanços `P236.div-1` + `P238.div-1` + Plano realista Cobertura Layout pós-P237

**Série**: 238 reescrita (vigésimo-quarto sub-passo Layout
pós-M9c; **passo administrativo documental** — distinto
de materialização; paridade pattern P225 encerramento
Fase 4 documental + P229 promoção ADR-0080 administrativa).
**Marco**: nenhum status ADR; **auditoria metodológica
formal dos dois falhanços spec arquiteturais consecutivos**
(P236.div-1 + P238.div-1); **plano realista cobertura
Layout pós-P237** identificando bloqueadores arquiteturais
vs sub-passos viáveis; refino lição metodológica P236.div-1
N=1 → **2 cumulativo** (spec audit prévio obrigatório
para sub-passos walk-time/runtime).
**Tipo**: passo administrativo documental — **zero código
tocado**; 1 ficheiro auditoria/plano output; ADR-0079
anotação cumulativa.
**Magnitude**: S (~1h auditoria + plano).
**Pré-condição**: P237 concluído (D.1 state_at refino
estendido; paralelismo state↔counter completo eval-time;
2150 verdes); **P238 original spec falhou** via P238.div-1
formal (audit revelou Content::State zero-size + Func::call
não-existe + StateUpdate::Func P172 stub blocker
arquitectural); humano fixou P238 reescrito como auditoria
+ plano (decisão literal pós-P238.div-1).
**Output**: 1 ficheiro auditoria + plano
(`00_nucleo/materialization/typst-passo-238-auditoria.md`
~12-15 KB) + ADR-0079 anotação **P238 reescrito auditoria
metodológica + plano cobertura Layout** + footnote ⁵⁷
inventário 148.

---

## §1 Trabalho

P238 original spec assumiu hipóteses incorrectas sobre
baseline state runtime walk-time. Audit C1 obrigatório
revelou contradição factual material:
1. `Content::State { .. } => {}` em `layout/mod.rs:352`
   — Content::State é **zero-size em layout** (não
   renderiza valor real). P171 baseline é apenas init
   marker; valor obtido via queries (`state_value` /
   `state_final_value`).
2. `Func::call` **não existe** — só `Func::native`
   constructor. Eval real requer `EvalContext + Engine +
   World + FileId + figure_numbering` que **não estão
   disponíveis durante walk**.
3. `StateUpdate::Func` é **stub em P172** — documentado
   explicitamente: "from_tags reconhece a variant mas
   **não avalia** a closure — Func::call requer EvalContext
   + Engine que não estão disponíveis em walk nem em
   from_tags". Mesmo blocker arquitectural.

P236.div-1 (P171/M9 state runtime já-materializado pre-passo)
+ P238.div-1 (walk-time callback dispatch arquiteturalmente
bloqueado) constituem **dois falhanços consecutivos** de
spec arquitectural maior pós-M9c. Análise honesta exigida.

**P238 reescrito materializa**:
- **Auditoria metodológica formal** dos dois falhanços
  (causas raiz; padrões emergentes; lição refinada).
- **Estado factual cobertura Layout pós-P237** (sub-passos
  materializados vs pendentes; bloqueadores arquiteturais).
- **Plano realista cobertura Layout** identificando o que
  é viável pós-P237 sem refactor M7+ vs o que requer
  pipeline restructuring.
- **Refino lição metodológica P236.div-1** via P238.div-1
  empírico (spec audit prévio obrigatório para sub-passos
  walk-time/runtime).

---

## §2 Parte 1 — Auditoria metodológica falhanços P236.div-1 + P238.div-1

### §2.1 Causa raiz P236.div-1

**Spec P236 hipotetizou**: state runtime ausente cristalino;
ADR-0066 PROPOSTO ainda activo; D.1 materializa feature
nova (`Value::State` variant + 2 módulos novos + Layouter
+1 field + 4 stdlib funcs).

**Estado real revelado audit C1**: state runtime
**substancialmente materializado** em P171/M9 + P172 +
P190C/D:
- `Content::State { key, init }` baseline P171.
- `Content::StateUpdate { key, update }` baseline P171/P172.
- `entities/state_registry.rs` + `entities/state_update.rs`
  + `entities/layouter_runtime_state.rs` baseline.
- 3 stdlib funcs (`native_state`, `native_state_update`,
  `native_state_update_with`) baseline.
- Pipeline activo via `Introspector::state_final_value` +
  `state_value(key, location)` + from_tags walk.
- **ADR-0066 SUPERSEDED-BY 0073** desde P204H (2026-05-07).

**Spec previa 8 decisões + 12 cláusulas** baseadas em
hipótese factual incorrecta. **Decisão 8** (L0 partial
tocado primeira excepção justificada) particularmente
inválida — refino aditivo real qualifica Opção γ literal.

**Causa raiz metodológica**: sumário de contexto que
guiou spec P236 **não incluía P171/M9 + P204H**. Spec
assumiu baseline pré-M9c sem audit prévio.

### §2.2 Causa raiz P238.div-1

**Spec P238 hipotetizou (mesmo com C1 obrigatório
bloqueante)**: 
- `Content::State` baseline P171 renderiza valor durante
  walk (Opção α refino aditivo +callback field provável).
- `Func::call(args)` baseline existe e funciona durante
  walk.
- Walk integration arm Content::State pre-existente em
  `layout/mod.rs` pode receber callback dispatch.

**Estado real revelado audit C1**:
- `Content::State` é **zero-size em layout** — arm é
  `Content::State { .. } => {}` literalmente vazio em
  `layout/mod.rs:352`. P171 baseline é init marker; valor
  obtido **fora** do walk via queries.
- `Func::call` **não existe** — só `Func::native`
  constructor. Eval requer contexto completo
  (`EvalContext + Engine + World + FileId +
  figure_numbering`) **indisponível durante walk**.
- `StateUpdate::Func` (P172) já documentou explicitamente
  este blocker: "from_tags reconhece a variant mas não
  avalia a closure — Func::call requer EvalContext + Engine
  que não estão disponíveis em walk nem em from_tags".

**Spec previa walk-time render-mediated callback** que é
**arquiteturalmente impossível sem pipeline restructuring
M7+**.

**Causa raiz metodológica**: spec **incluiu C1 audit
obrigatório bloqueante** (Decisão 0 — lição P236.div-1)
mas **fixou decisões C2-C8 prováveis baseadas em hipóteses
análogas** (Categoria A.3 GridCell P230 +1 field; pattern
paralelo). Hipóteses análogas eval-time aplicaram-se
incorretamente a walk-time/runtime integration.

### §2.3 Padrão comum aos dois falhanços

| Aspecto | P236 | P238 |
|---------|------|------|
| Hipótese spec | Feature ausente | Feature parcialmente presente extensível |
| Estado real | Feature materializada pre-passo | Feature stub bloqueada arquiteturalmente |
| Causa raiz | Sumário contexto incompleto | Hipóteses análogas falsas para walk-time |
| Audit C1 spec | Não-obrigatório (P236 spec original) | Obrigatório (lição P237) mas insuficiente |
| Decisões C2+ fixadas | Sim (8 decisões hipóteses) | Sim (8 decisões "sujeitas C1") |
| Resolução | div-1 + Opção 2 humana (refino aditivo state_final) | Pendente (este passo) |

**Padrão metodológico identificado**: **spec fixar
decisões C2-C8 "sujeitas a C1" cria viés cognitivo**.
Decisões já-fixadas resistem revisão mesmo quando audit
revela contradição. Pattern "C1 audit obrigatório
bloqueante" P237 funcionou para refino trivial wrapper
(state_at paralelo counter_at) mas **falhou para sub-passo
walk-time/runtime arquitectural** (state_display).

### §2.4 Distinção crítica entre tipos de refino

| Tipo refino | Risco hipótese | Exemplos | Spec viável |
|-------------|----------------|----------|-------------|
| **Eval-time wrapper trivial** | Baixo | P236 state_final; P237 state_at | Hipóteses prováveis aceitáveis |
| **Refino aditivo cosmético variant** | Médio | A.1 stroke P227; A.2 fill P228 | Hipóteses prováveis aceitáveis |
| **Refino algorítmico per-cell** | Médio | B.3 GridCell P235 | Hipóteses prováveis com audit obrigatório |
| **Walk-time render-mediated** | **Alto** | **P238 state_display tentado** | **Audit prévio obrigatório ANTES de fixar decisões** |
| **Runtime callback dispatch** | **Alto** | P238 callback walk eval | **Audit prévio obrigatório** |
| **Pipeline restructuring** | **Crítico** | M7 fixpoint; multi-region | **Reabertura M-fase necessária** |

**Lição refinada**: spec não deve fixar decisões prováveis
em sub-passos com risco **alto/crítico** (walk-time/runtime
arquitectural). Audit prévio obrigatório **ANTES** de
fixar decisões C2-C8 — não apenas "sujeitas a C1".

### §2.5 Refino lição metodológica P236.div-1

**Lição original P236.div-1** (aplicada P237 com sucesso):
> "Spec C1 audit obrigatório bloqueante como primeira
> cláusula antes de fixar decisões arquiteturais C2+."

**Refino pós-P238.div-1**:
> "Para sub-passos com risco alto/crítico (walk-time;
> runtime callback dispatch; pipeline integration), spec
> deve fazer audit prévio **ANTES** de redigir decisões
> C2-C8. Decisões hipotéticas 'sujeitas a C1' criam viés
> cognitivo que resiste revisão pós-audit. Para refinos
> de risco baixo/médio (eval-time wrappers; cosméticos;
> algorítmicos isolados), C1 audit bloqueante como primeira
> cláusula é suficiente."

**Pattern emergente refinado "spec audit prévio obrigatório
para sub-passos walk-time/runtime" N=1 → 2 cumulativo**
(refino lição P236.div-1).

### §2.6 Atomização preventiva: prep-passo audit-only + materialização-passo

**Recomendação metodológica**: para sub-passos risco
alto/crítico, atomizar em:
1. **Prep-passo audit-only** (XS-S magnitude) — apenas
   audit + relatório + identificação bloqueadores; sem
   decisões fixadas.
2. **Materialização-passo** (magnitude conforme audit) —
   decisões fixadas baseadas no prep-passo audit.

Paridade pattern P226 (diagnóstico amplo + ADR PROPOSTO
+ roadmap) que precedeu materialização Fase 5 P227+.
**Distinto** de C1 audit dentro do passo (que é tarde
demais para sub-passos arquiteturais).

---

## §3 Parte 2 — Estado factual Cobertura Layout pós-P237

### §3.1 Sub-passos Fase 5 Layout materializados

**Categoria A — Cosméticos (5/5 ✓ FECHADA)**:
- A.1 stroke Grid+Table (P227) ✓.
- A.2 fill Grid+Table (P228) ✓.
- A.3 per-cell GridCell+TableCell stroke/fill (P230) ✓.
- A.4 outset/radius/clip Block+Boxed (P231 graded) ✓
  estructural; renderização parcial (outset/radius/clip
  semantic adiada).
- A.5 Place per-cell alignment override (P232) ✓.

**Categoria B — Algorítmicos isolados (3/3 ✓ FECHADA)**:
- B.1 DEBT-34d Auto track sizing fix (P233) ✓ — fecho
  DEBT preservado P224.div-1.
- B.2 Consumer geometric place_cells → Layouter (P234)
  ✓ — colspan/rowspan funcionais.
- B.3 GridCell+TableCell align/inset/breakable per-cell
  (P235) ✓ — pattern `.or()` N=3 atinge limiar.

**Categoria D — Runtime queries (1.5/? sub-passos)**:
- D.1 state runtime — substancialmente materializado
  pre-P236 (P171/M9 + P172 + P190C/D) com refinos aditivos
  eval-time pós-divergências:
  - state_final P236 (eval-time wrapper).
  - state_at P237 (eval-time wrapper paralelo counter_at).
- D.2 state.display walk-time **BLOQUEADO** arquiteturalmente
  (P238.div-1).

**Categoria C — Estruturais reabrindo (0/2)**:
- C.1 Place float real — reabre Opção B P219; bloqueado
  per ADR-0079 graded.
- C.2 Multi-region completa — reabre P216B; DEBT-56b
  candidato; bloqueador arquitectural M7+ pipeline.

### §3.2 Distribuição numérica cumulativa pós-P237

- **Sub-passos materializados**: 10/13-15 (~67-77%).
- **Sub-passos bloqueados arquiteturalmente**: 1+ (D.2
  state.display walk-time; C.1; C.2; possíveis D.3+).
- **Sub-passos viáveis sem refactor M7+**: identificados
  na §4.

### §3.3 Cobertura Layout per metodologia

- **§A.5 distribuição**: 12/4/2/0/0 = 18 preservada pós-P237.
- **Cobertura Layout per metodologia**: 89% preservada
  pós-P237 (refinos qualitativos cumulativos).
- **Cobertura user-facing total**: 67% preservada pós-P237.

### §3.4 Métricas cumulativas pós-P237

| Métrica | Valor |
|---------|-------|
| Tests workspace | 2150 verdes |
| Violations | 0 |
| Content variants | 60 |
| Value variants | 55 |
| Stdlib funcs | 62 |
| Saldo DEBTs abertos | 11 |
| ADRs PROPOSTO | 12 |
| ADRs EM VIGOR | 29 |
| ADRs IMPLEMENTADO | 21 |
| Anti-inflação aplicações | 29 |

---

## §4 Parte 3 — Plano realista cobertura Layout pós-P237

### §4.1 Bloqueadores arquiteturais identificados

**Bloqueador A — Walk-time eval Func dispatch**:
- `Func::call` não existe; só `Func::native` constructor.
- Eval real requer `EvalContext + Engine + World + FileId
  + figure_numbering` indisponíveis durante walk.
- `StateUpdate::Func` (P172) documenta blocker explicitamente.
- **Sub-passos afectados**: D.2 state.display callback;
  `counter.display` callback paralelo; **possivelmente
  D.3+ runtime features que requerem callback dispatch**.
- **Resolução**: pipeline restructuring M7+ (eval+walk
  integrado OR fixpoint completion).

**Bloqueador B — Multi-region completion**:
- DEBT-56 sub-fase a (P216A) fechada; sub-fase b
  (P217-P220) materializada.
- **DEBT-56b candidato** se C.2 Multi-region completa
  reabrir — pipeline multi-region cell-level (cell rowspan
  que cruza pagination).
- **Sub-passos afectados**: C.2 Multi-region; possível
  parte de A.4 (breakable per-cell render real P235
  graded).
- **Resolução**: refactor multi-region cell-level fora
  escopo refino aditivo.

**Bloqueador C — Place float real**:
- Opção B P219 graded baseline (flow contorna);
  implementação real requer reabertura.
- **Sub-passos afectados**: C.1 Place float real.
- **Resolução**: refactor Place float real magnitude L+
  reabre ADR-0079 graded.

**Bloqueador D — Pipeline runtime two-pass walk**:
- `state.final()` semantic vanilla requer two-pass walk
  (pass 1 computes final; pass 2 resolves).
- P236 implementou `state_final` retornando valor corrente
  via `Introspector::state_final_value` (não-real two-pass).
- **Sub-passos afectados**: refino `state.final()` real
  two-pass; semantic adiada graded preserva escopo.
- **Resolução**: pipeline two-pass walk infrastructure
  M7+.

### §4.2 Sub-passos viáveis sem refactor M7+

#### Categoria D — Refinos eval-time aditivos restantes

**D.X1 — `counter.display(counter, callback)` paralelo state.display**:
- Mesmo blocker arquitectural P238.div-1 (walk-time callback
  dispatch).
- **VIÁVEL via stub paridade P172** — Content::CounterDisplay
  variant + native_counter_display stdlib func; walk arm
  armazena callback sem eval; renderiza placeholder.
- **Não-recomendado se D.2 state.display também stub** —
  duplica limitação sem benefício utilizador real.
- Magnitude: S-M (~1.5-2h).

**D.X2 — `query(target)` runtime refinos**:
- Baseline P175/P179 materializado.
- Refinos eval-time wrappers viáveis (paridade state_at/
  state_final patterns N=2).
- **Audit prévio obrigatório** (lição refinada §2.6) para
  identificar exactamente o que falta vs baseline P175/P179.
- Magnitude: prep-passo audit (XS); materialização
  conforme (S-M).

**D.X3 — `numbering(...)` runtime refinos**:
- Possível baseline; audit prévio obrigatório.
- Magnitude: prep-passo audit (XS); materialização
  conforme.

#### Categoria A — Refinos cosméticos restantes

**A.4 refino — Block/Boxed outset/radius/clip real
rendering**:
- P231 baseline implementou variant fields graded
  (semantic adiada).
- Render real:
  - **outset**: ~viable; requer ajuste bounds emit
    (paridade pattern inset P156G real). Audit prévio
    obrigatório.
  - **radius**: requer infraestrutura geometry corners
    rendering. Possível bloqueador arquitectural.
  - **clip**: requer infraestrutura clipping mask. Possível
    bloqueador arquitectural.
- Magnitude: prep-passo audit (XS-S); materialização
  conforme audit (S-M se outset apenas; L+ se inclui
  radius+clip).

**A.X — fill/stroke Block/Boxed** (não estavam em
diagnóstico P226 Categoria A.4; Categoria A separada
futura):
- Paridade pattern P227+P228 estructural a Block/Boxed.
- Render real viável (Block fill = background; distinct
  de Grid cells fill).
- Magnitude: S+ (~1-1.5h).

#### Categoria B — Refinos algorítmicos restantes

**B.X1 — DEBT-34d-rest** (se atomização C1 P233 revelou
sub-itens):
- P233 audit revelou DEBT-34d unitário (não-amplo); sem
  sub-itens pendentes.
- **N/A** — DEBT-34d FECHADO P233 completo.

**B.X2 — Refinos geometric place_cells**:
- B.2 completo P234; sem refinos pendentes identificados.

**B.X3 — Outros algorítmicos**:
- `Block.spacing` / `above` / `below` / `sticky` —
  atributos de fluxo distintos. Categoria B candidato
  refino futuro mas magnitude L+ por integração flow.

### §4.3 Sub-passos bloqueados arquiteturalmente

| Sub-passo | Bloqueador | Resolução requerida |
|-----------|------------|---------------------|
| D.2 state.display real | Walk-time Func dispatch | M7+ pipeline restructuring |
| `counter.display` real | Walk-time Func dispatch | M7+ pipeline restructuring |
| `state.final()` real two-pass | Pipeline two-pass walk | M7+ infrastructure |
| C.1 Place float real | Reabertura Opção B P219 | Refactor magnitude L+ |
| C.2 Multi-region completa | DEBT-56b + pipeline cell-level | Refactor M7+ |
| A.4 radius render real | Geometry corners infrastructure | Possível M7+ |
| A.4 clip render real | Clipping mask infrastructure | Possível M7+ |
| A.4 breakable per-cell real (P235) | Multi-region cell-level | Refactor multi-region |
| A.4 outset/radius/clip Block+Boxed real (P231 graded) | Mesma infraestrutura A.4 graded | Conforme A.4 |

### §4.4 Caminho realista pós-P237

**Opção curto-prazo (4-6h)** — fechar viáveis sem M7+:
- Prep-passo audit prévio (XS) — identificar exactamente
  state runtime baseline + query baseline + outset render
  baseline.
- A.X fill/stroke Block/Boxed (S+).
- D.X2 query refinos (S-M conforme audit).
- ADR meta admin XS — promoção formal patterns sólidos
  (`.or()` N=3; paralelo N=5; Smart→Option N=12;
  semantic adiada N=8; EM VIGOR aplicação automática N=8).

**Opção médio-prazo (~10-15h)** — abrir Categoria A
restantes + A.4 real conforme audit:
- A.4 outset render real (S-M).
- A.4 radius/clip conforme audit infraestrutura geometry.

**Opção longo-prazo (M7+ refactor)** — pipeline
restructuring:
- M7+ fixpoint completion OR eval+walk integrado.
- Desbloquear D.2 state.display real + counter.display +
  state.final two-pass.
- Magnitude: XL+ (refactor arquitectural maior).

**Opção pivot** — outro módulo (Visualize 54%, Text 52%,
Model 50%) com hipótese spec audit prévio obrigatório
para sub-passos walk-time/runtime.

### §4.5 Estimativa fecho realista Fase 5 Layout

**Sem refactor M7+**: Fase 5 Layout candidata fica em
**10-12/13-15 sub-passos materializados** (~67-85%);
sub-passos bloqueados arquiteturalmente preservados como
graded/scope-out documentados. Saldo DEBTs cresce por
DEBT-state-display (D.2 bloqueada) + DEBT-counter-display
(análogo) se documentado.

**Com refactor M7+**: Fase 5 Layout candidata pode
materializar 13-15/13-15 (100% interno) mas magnitude
cumulativa **L+ a XL+** (M7+ refactor maior).

**Decisão arquitectural pendente**: humano decide se Fase
5 Layout fecha graded a ~80% (preservando bloqueadores
como scope-out documentado) OU reabre M-fase para refactor
pipeline.

---

## §5 Parte 4 — Recomendações metodológicas futuras

### §5.1 Aplicação da lição refinada

**Para sub-passos risco baixo/médio** (eval-time wrappers;
cosméticos; algorítmicos isolados):
- C1 audit obrigatório bloqueante como primeira cláusula
  (lição P236.div-1 original).
- Decisões C2-C8 prováveis fixadas com "sujeitas a C1"
  marker.
- Pattern P237 baseline (state_at refino paralelo
  counter_at sucesso).

**Para sub-passos risco alto/crítico** (walk-time;
runtime callback dispatch; pipeline integration; multi-region):
- **Atomizar em prep-passo audit-only** (XS-S magnitude;
  paridade P215 diagnóstico Fase 3 + P226 diagnóstico
  Fase 5).
- Prep-passo output: relatório audit + identificação
  bloqueadores arquiteturais + estimativa magnitude
  materialização.
- Materialização-passo subsequente fixa decisões C2-C8
  baseadas em audit empírico.

### §5.2 Sinais de risco alto/crítico

Sub-passo merece prep-passo audit-only quando envolve:
- Walk-time integration (não apenas eval-time).
- Func callback dispatch.
- Pipeline restructuring (multi-region; two-pass walk).
- Refactor estructural (variants existentes com semantic
  mudanças).
- Cross-módulo dependencies (Layouter + Introspector +
  Engine + EvalContext).
- ADR PROPOSTO/SUPERSEDED/ACEITE chain reabertura.

Sinais ausentes (refino seguro):
- Stdlib func nova wrapper trivial paralela existing.
- Refino aditivo +1 field paridade pattern estabelecido.
- Renderização Z-order extension paridade existing.

### §5.3 Pattern emergente "spec audit prévio para sub-passos walk-time/runtime"

Pattern emergente "spec audit prévio obrigatório para
sub-passos walk-time/runtime" N=1 inaugurado P238 reescrito
(refino lição P236.div-1):
- P237 (eval-time wrapper) — C1 audit bloqueante suficiente.
- P238 (walk-time render) — prep-passo audit prévio
  necessário; C1 audit bloqueante insuficiente.

Aplicação cumulativa esperada:
- Próximos sub-passos eval-time (D.X2 query refinos;
  state_final two-pass refino se viável; outros wrappers)
  — C1 audit bloqueante.
- Próximos sub-passos walk-time/runtime — **prep-passo
  audit prévio obrigatório**.

---

## §6 Saída cumulativa pós-P238 reescrito

**Sem código tocado**:
- Tests workspace: 2150 verdes preservado.
- Violations: 0 preservadas.
- Content variants: 60 preservado.
- Value variants: 55 preservado.
- Stdlib funcs: 62 preservado.
- Layouter fields: preservados.
- §A.5 distribuição: `12/4/2/0/0 = 18` preservada.
- Cobertura Layout per metodologia: **89% preservada**.
- Cobertura user-facing total: 67% preservada.
- ADRs: PROPOSTO 12; EM VIGOR 29; IMPLEMENTADO 21; total
  67.
- Saldo DEBTs: 11 preservado.

**Anti-inflação 30ª aplicação cumulativa** pós-P205D —
Opção "auditoria documental" (não materializar código
quando bloqueio arquitectural identificado) + Opção γ L0
NÃO tocado + Opção α sem promoção ADR + Opção α sem
marco cirúrgico blueprint + paridade pattern P225/P229
administrativo + Decisão refino lição P236.div-1.

**Patterns emergentes inaugurados/consolidados**:
- **Pattern "spec audit prévio obrigatório para sub-passos
  walk-time/runtime" N=1 inaugurado P238 reescrito** —
  refino lição P236.div-1.
- **Pattern "atomização prep-passo audit-only + materialização-passo
  para sub-passos risco alto/crítico" N=1 inaugurado P238
  reescrito**.
- **Pattern "P238.div-1 paridade P236.div-1 — div-N
  cumulativo para falhanços spec arquitectural maior"
  N=1 → 2 cumulativo** (P236.div-1 + **P238.div-1**).
- **Pattern "passo administrativo documental para auditoria
  metodológica pós-divergência" N=1 inaugurado P238
  reescrito** — distinto de P225 (encerramento Fase) +
  P229 (promoção ADR-0080).
- **Pattern "L0 minimal para refactors" aplicação
  automática N=8 preservado** (P230-P237; P238 reescrito
  documental não-incrementa).
- **Pattern emergente "Fase candidata fecha graded a
  bloqueadores arquiteturais identificados" N=1 inaugurado
  P238 reescrito** — Fase 5 Layout candidata pode fechar
  ~80% preservando bloqueadores como scope-out documentado.

**Próximo sub-passo — caminhos candidatos pós-P238
reescrito**:

| Caminho | Trabalho | Magnitude | Prioridade |
|---------|----------|-----------|------------|
| **Prep-passo audit query baseline** | Audit prévio query(target) refinos D.X2 conforme lição §5.1 | XS (~30min) | **alta** (lição metodológica aplicada; audit pre-materialização) |
| **A.X fill/stroke Block/Boxed** | Refino aditivo estructural paridade A.1+A.2 a Block/Boxed | S+ (~1-1.5h) | média (cosmético; render real viável) |
| **ADR meta admin XS** | Promoção formal patterns sólidos paridade P229 (singular ou batch) | XS por pattern | média (consolidação meta) |
| **A.4 refino outset render real** | Audit prévio + materialização conforme | XS audit + S-M materialização | média (refino A.4 graded P231) |
| **Pivot outro módulo** | Visualize 54%; Text 52%; Model 50% | varia | média (Fase 5 ~67-77% cumulativo) |
| **Reabertura M-fase** | M7+ pipeline restructuring para desbloquear D.2+ | XL+ | baixa (magnitude maior) |

**Recomendação subjectiva**: **Prep-passo audit query
baseline** — primeira aplicação real da lição refinada
P238 reescrito (audit prévio para refinos eval-time
runtime; identifica exactamente o que falta vs baseline
P175/P179 antes de fixar decisões). Magnitude XS;
metodologicamente sólida; previne futuro div-N.

**Alternativa subjectiva**: **A.X fill/stroke Block/Boxed**
refino aditivo cosmético directo sem audit prévio (risco
baixo; pattern P227+P228 estabelecido) se humano priorizar
material visível imediato.

**Decisão humana fica em aberto literal** pós-P238 reescrito.

---

## §7 Critério aceitação P238 reescrito

- 1 ficheiro auditoria + plano output
  (`typst-passo-238-auditoria.md` ~12-15 KB).
- ADR-0079 anotação **P238 reescrito auditoria metodológica
  + plano cobertura Layout**.
- Inventário 148 footnote ⁵⁷ adicionada documentando
  P238.div-1 + auditoria + plano realista cobertura.
- **Zero código tocado**.
- Tests 2150 verdes preservados (paridade pattern
  administrativo P225/P229).
- 0 violations preservadas.
- Saldo DEBTs 11 preservado.
- ADRs distribuição preservada.
- Lição metodológica refinada documentada formalmente.
- Plano realista cobertura Layout pós-P237 identificado
  e priorizado.

**Output esperado**:
- `00_nucleo/materialization/typst-passo-238-auditoria.md`
  (~12-15 KB; 7 §s estructurais; auditoria + plano).
- Editado: `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`
  (footnote ⁵⁷ P238 reescrito).
- Editado: `00_nucleo/adr/typst-adr-0079-layout-fase-5-roadmap.md`
  (+ anotação P238 reescrito auditoria + plano cobertura
  Layout).

**Sem novos ficheiros código**. Sem L0 prompts tocados.
Sem novos tests. Sem refactor.
