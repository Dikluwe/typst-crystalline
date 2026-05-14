# Audit P239 — Prep-passo audit-only reabertura M-fase para M7+ refactor

**Data**: 2026-05-14.
**Spec**: `00_nucleo/materialization/typst-passo-239.md` (P239 prep-passo).
**Tipo**: passo administrativo documental audit-only — **zero código tocado**.
**Magnitude planeada**: S (~1-1.5h auditoria). **Magnitude real**: S
(~1h auditoria empírica + redação).
**Marco**: primeira aplicação real do pattern "atomização prep-passo
audit-only + materialização-passo" inaugurado P238 reescrito;
primeira reabertura M-fase pós-M9c iniciada metodologicamente
correctamente (lição `P236.div-1` → `P238.div-1` refinada
aplicada literal); pattern emergente "prep-passo audit-only
preventivo para reabertura M-fase" N=1 inaugurado P239.

---

## §1 O que foi feito

P239 prep-passo audit-only materializa **audit empírico cumulativo
dos 4 bloqueadores arquiteturais** identificados em P238 reescrito
para fechar Fase 5 Layout completa: (A) walk-time eval Func
dispatch; (B) multi-region completion; (C) Place float real;
(D) pipeline runtime two-pass walk. Audit revela **achado material
inesperado em bloqueador A**: `apply_state_funcs` já existe e
avalia `StateUpdate::Func` via fixpoint loop pós-walk — hipótese
P238 reescrito "Func::call não existe" precisa refino (existe
`apply_func` em `eval/closures.rs:59`; o blocker real é Engine+ctx
indisponíveis em **layout**, não em pipeline introspect). Output
proposto: roadmap atomização **4 sub-passos materialização M7+
(M7+1 a M7+4)**; ADR meta novo PROPOSTO `typst-adr-0081-m7-plus-pipeline-restructuring-scope.md`;
3 pré-condições obrigatórias formalizadas; nomenclatura M-fase
preliminarmente **M9d** (continuação M9c) sujeita decisão humana.

---

## §2 Decisão arquitectural: prep-passo audit-only (lição P238.div-1 aplicada literal)

**Pattern "spec audit prévio obrigatório para sub-passos
walk-time/runtime" N=1 → 2 cumulativo** (P238 reescrito + **P239**).

Reabertura M-fase é caso máximo risco crítico. Decisões C2-C8 do
P239 spec **NÃO fixam decisões C2-C8 dos sub-passos materialização
M7+1+**. P239 produz audit empírico; sub-passos materialização
M7+1+ fixam decisões C2+ baseadas no audit empírico. Magnitude
materialização per sub-passo é responsabilidade de spec separado.

**8 decisões fixadas P239** (per spec §1):

- **Decisão 0** — Prep-passo audit-only obrigatório (lição
  `P238.div-1` aplicada literal). Pattern N=1 → 2.
- **Decisão 1** — Opção α escopo audit cumulativo (M-fase história
  + 4 bloqueadores).
- **Decisão 2** — Opção α ADR meta novo PROPOSTO criado (confirmado
  empíricamente per §6 abaixo).
- **Decisão 3** — Opção α atomização ADR-0036 aplicável (audit C5
  propõe 4 sub-passos atomizados).
- **Decisão 4** — 3 pré-condições obrigatórias formais (audit C6).
- **Decisão 5** — Magnitude cumulativa estimada XL+ refinada
  pós-audit empírico (§4.3).
- **Decisão 6** — Opção γ L0 NÃO tocado (P239 administrativo
  documental).
- **Decisão 7** — Opção β saldo DEBTs preservado/decresce (M7+
  pode fechar DEBT-56b).
- **Decisão 8** — Sem promoção ADR-0079 status; sem fecho Fase 5;
  Fase 5 candidata permanece **10/13-15 sub-passos materializados**
  baseline pós-P237.

---

## §3 Auditoria M-fase + bloqueadores arquiteturais

### §3.1 Audit M-fase histórico (C1)

| M-fase | Escopo principal       | Estado terminal     | ADRs relacionados      | Fecho factual                        |
|--------|------------------------|---------------------|------------------------|--------------------------------------|
| M5     | Universal completo     | Fechado P200B       | ADR-0072 §90           | P200B 2026-05-06                     |
| M6     | Show rules + Counter cleanup | Fechado P190I  | ADR-0070; ADR-0072 §92 | P190I 2026-05-05                     |
| M7     | Fixpoint runtime       | **Estruturalmente fechado** P192B | **ADR-0072 ACEITE** | P192B 2026-05-05 (hash-based) |
| M8     | comemo memoization adopt | **ACEITE** completo retroactivo M9c | ADR-0073 ACEITE; ADR-0074 ACEITE | P205B+C+E 2026-05-07 |
| M9     | Stdlib introspection 11/11 | Fechado P182F prematuro | ADR-0066 SUPERSEDED-BY 0073 | P182F (gap empírico revelou) |
| M9c    | M9-completion (gap trait + sub-stores) | **ACEITE retroactivo** P212 | ADR-0076 ACEITE completo retroactivo | P212 2026-05-12      |

**Achado factual material C1.1**: M7 fechado **estruturalmente**
P192B com hash-based memoization intermediário; M8 substituiu por
comemo (ADR-0073). M7 NÃO é "Func dispatch refactor" — M7 é fixpoint
loop (sem refactor pipeline walk). **Reabertura M-fase para
walk-time eval Func dispatch é nova M-fase**, não reabertura M7.

**Achado factual material C1.2 — nomenclatura M-fase**: ADR-0076
§"Alternativa F" rejeita "M10 novo" para M9c (`M10 deve ser
próximo marco verdadeiramente novo (e.g. Show rules, Math layer,
etc.)`). Sub-fase pipeline restructuring para walk-time eval
**continua trabalho M9** (state runtime + introspector
completion) — paridade conceitual com M9c (continuation de M9).

**Decisão preliminar nomenclatura M-fase** (sujeita ratificação
humana via ADR meta novo):

| Opção | Nomenclatura | Coerência |
|-------|-------------|-----------|
| α     | **M9d**     | Continuação M9c pattern N=2 — "completion" continua para walk-time real |
| β     | **M10**     | Marco verdadeiramente novo per ADR-0076 §F (mas pipeline restructuring continua M9-line) |
| γ     | M-pipeline  | Nome semanticamente claro; sem numeração |

**Recomendação subjectiva — Opção α (M9d)**: pattern M9c
("completion" continua M9 retroactivo) extends N=1 → 2; coerência
narrativa preservada. ADR meta novo formaliza decisão.

### §3.2 Audit blocker arquitectural walk-time eval Func dispatch (C2)

**Achado factual material C2.1 — `apply_state_funcs` JÁ EXISTE**:

Localização: `01_core/src/rules/introspect/from_tags.rs:48`.
Assinatura:

```rust
pub fn apply_state_funcs(
    tags:   &[Tag],
    intr:   &mut TagIntrospector,
    engine: &mut Engine<'_>,
    ctx:    &mut EvalContext,
)
```

Caller: `fixpoint::run_fixpoint` em
`01_core/src/rules/introspect/fixpoint.rs:101`. Operação:

1. Walk emite tags (incluindo `Tag::Start(loc, info)` com
   `ElementPayload::StateUpdate { update: StateUpdate::Func(_) }`).
2. Pós-walk, `apply_state_funcs(tags, intr, engine, ctx)` chama
   `apply_func(func.clone(), args, ctx, engine)` (em
   `eval/closures.rs:59`) com Engine+ctx disponíveis.
3. Resultado actualiza `intr.state` via `intr.state.update(key,
   value, loc)`.
4. Fixpoint converge quando hash(tags) idempotente.

**Achado factual material C2.2 — hipótese P238 reescrito precisa
refino**:

- ✗ Hipótese P238 reescrito: "`Func::call` não existe — só
  `Func::native` constructor".
- ✓ Realidade: `Func::call` método não existe **como método em
  Func**, mas mecanismo de chamada é `closures::apply_func(func,
  args, ctx, engine)` que existe e funciona. `Func` tem
  `repr()` retornando `FuncRepr::Closure | FuncRepr::Native`;
  `apply_func` despacha para `apply_closure` ou nativo.
- ✓ Realidade: state runtime Funcs **JÁ avaliam** durante fixpoint
  loop pós-walk via `apply_state_funcs`.

**Achado factual material C2.3 — o blocker REAL é layout-time
Engine+ctx**:

- `Content::State { .. } => {}` arm em `layout/mod.rs:352` é
  literalmente vazio porque `Content::State` é init marker (não
  renderiza output user-visible).
- `Content::StateUpdate { .. } => {}` arm em `layout/mod.rs:353`
  é literalmente vazio porque updates são emitidos como tags
  durante walk, não renderizados como output.
- **Para `state.display(callback)` semantic real**: precisa
  variant novo `Content::StateDisplay { key, callback: Func }`
  OU campo `display` em `Content::State`; durante layout walk,
  resolver `state_value(key, loc)` via Layouter introspector +
  aplicar `callback` para produzir Content renderizável.
- **Bloqueador material**: Layouter **não tem `Engine + ctx`**
  na sua signature. Layouter é puro layout sem acesso eval —
  paridade arquitectural com vanilla typst onde Show rules
  re-entram pipeline eval, mas Cristalino Layouter pós-M9c
  não tem este mecanismo recursivo.

**Resolução estructural — 4 opções**:

| Opção | Mecânica                                                       | Magnitude  | Risco                                              |
|-------|----------------------------------------------------------------|------------|-----------------------------------------------------|
| α     | Pass `Engine + ctx` para Layouter signature massivo            | L+         | Comemo invariants ADR-0073/0074 risco quebrar       |
| β     | Two-pass walk: pass 1 = walk + apply Funcs em fixpoint; pass 2 = layout walk re-entra eval para display callbacks | XL+ | Comemo invariants; signature refactor cumulativo |
| **γ** | **Pre-evaluate display callbacks em fixpoint pré-layout**: novo `apply_state_displays` em `from_tags` paralelo a `apply_state_funcs`; layout consome Content pré-renderizado | **L (~5-8h)** | **Paridade pattern existente; baixo risco**     |
| δ     | Show rule mecanismo: `state.display(fn)` synthetic Show rule re-entrando eval pipeline | L+ | Refactor Show rules pipeline |

**Recomendação subjectiva — Opção γ**: extende pattern existente
`apply_state_funcs` para `apply_state_displays`. Layout permanece
puro (sem Engine+ctx). Magnitude L (~5-8h). Paridade conceitual
estrita.

### §3.3 Auditoria blockers arquiteturais relacionados (C3)

**Bloqueador B — Multi-region completion** (C3.1):

- DEBT-56 ENCERRADA P221 literal (column flow Opção B graded).
- DEBT-56b NÃO existe **ainda** — candidato se C.2 materializar
  (per ADR-0079 §"Reabertura 3").
- Pipeline `Regions` baseline P216B é minimal (`{ current: Region }`);
  `backlog`/`last` diferidos.
- **Escopo refactor**: estender `Regions` para multi-region
  cell-level (cell rowspan que cruza pagination); refactor `place_cells`
  + Layouter multi-region.
- Magnitude estimada: **L+ (~8-12h)**.
- **Independente parcial** do bloqueador A (não partilha
  walk-time eval; partilha pipeline Layouter mas em domínio
  ortogonal).

**Bloqueador C — Place float real** (C3.2):

- Opção B P219 graded (single-region width reduzida).
- Place float real requer flow contorna (multi-pass layout OR
  flow secundário topo/fundo).
- **Independente** dos outros bloqueadores (refactor Place specific).
- Magnitude estimada: **L (~5-8h)**.

**Bloqueador D — state.final two-pass walk** (C3.3):

- P236 implementou `state_final` retornando valor corrente via
  `Introspector::state_final_value` (não-real two-pass).
- Vanilla `state.final()` requer two-pass walk:
  - Pass 1: compute final state value cumulativo (último update).
  - Pass 2: resolve `state.final()` queries para esse valor.
- Cristalino `state_final_value` actualmente devolve último valor
  no introspector (post-fixpoint convergência); para semantic
  real, requer **mesma infraestrutura que Opção γ acima**
  (apply_state_funcs já materializou state values com Engine+ctx).
- **Sobreposição grande com Bloqueador A** (Opção γ): mesmo
  refactor desbloqueia ambos.
- Magnitude marginal pós-A: **M (~3-5h)** — apenas refino
  semantic `state_final` para reflectir two-pass real (Pass 2
  resolve para valor final cumulativo, não valor corrente).

**Bloqueador E (novo identificado P239) — A.4 radius/clip
infrastructure** (C3.4):

- `ShapeKind` actual (`01_core/src/entities/geometry.rs:32`):
  4 variants `Rect | Ellipse | Line | Path`. **NÃO existe
  `RoundedRect`**.
- `Group::clip_mask: Option<ShapeKind>` JÁ EXISTE
  (`layout_types.rs:235`) — clip infrastructure baseline.
- **Escopo refactor**: adicionar `ShapeKind::RoundedRect { radii: Corners<Length> }`;
  adicionar `Corners<T>` type paridade `Sides<T>`; estender
  exportador PDF para emitir paths com cantos arredondados.
- **Independente** dos outros bloqueadores (geometry primitives).
- Magnitude estimada: **M-L (~3-5h)**.

### §3.4 Sobreposições entre bloqueadores (C4)

| Bloqueador            | Sobreposição walk-time Func dispatch  | Sobreposição multi-region | Sobreposição radius/clip |
|-----------------------|----------------------------------------|---------------------------|--------------------------|
| A — walk-time Func    | —                                      | Não                       | Não                      |
| B — multi-region      | Não                                    | —                         | Não                      |
| C — Place float       | Não                                    | Parcial (flow secundário shares pipeline) | Não |
| D — state.final two-pass | **SIM grande** (mesmo refactor Opção γ) | Não                  | Não                      |
| E — radius/clip       | Não                                    | Não                       | —                        |

**Conclusão sobreposições**: bloqueadores A + D partilham
refactor Opção γ (`apply_state_displays` pre-eval em fixpoint).
Resto independente. Atomização proposta (§4) reflecte sobreposições.

---

## §4 Plano atomização sub-passos materialização M7+

### §4.1 Atomização proposta

| Sub-passo | Escopo                                          | Magnitude estimada | Dependencies   | Desbloqueia                            |
|-----------|-------------------------------------------------|--------------------|----------------|----------------------------------------|
| **M7+1**  | Pipeline walk-time eval (Opção γ): `apply_state_displays` + `Content::StateDisplay` variant + walk arm pre-render | **L (~5-8h)** | Nenhuma | D.2 state.display real; state.final two-pass real |
| **M7+2**  | counter.display paralelo (paridade M7+1)        | **M (~2-4h)**      | M7+1           | counter.display real                   |
| **M7+3**  | Multi-region completion cell-level              | **L+ (~8-12h)**    | Independente   | C.2 + A.4 breakable per-cell real      |
| **M7+4**  | Place float real (reabertura Opção B P219)      | **L (~5-8h)**      | Independente   | C.1                                    |
| **M7+5**  | A.4 radius/clip infrastructure (`ShapeKind::RoundedRect` + `Corners<T>`) | **M-L (~3-5h)** | Independente | A.4 radius + clip render real |

**Total cumulativo M7+**: **~23-37h materialização**. Magnitude
paridade M-fases anteriores cumulativas (M7 + M8 + M9 cumulativos
similar). **Sem refactor M7+, Fase 5 Layout candidata fica em
10-12/13-15 sub-passos materializados** (~67-85%).

### §4.2 Ordem proposta (recomendação subjectiva)

1. **M7+1** primeiro (maior desbloqueio cumulativo; D.2 +
   state.final two-pass; pattern Opção γ paridade
   `apply_state_funcs`).
2. **M7+2** segundo (após M7+1; reuso pattern).
3. **M7+3 / M7+4 / M7+5** independentes — ordem subjectiva
   conforme prioridade humana.

### §4.3 Estimativa refinada cumulativa M7+ refactor

- Magnitude cumulativa estimada **L+ a XL (~23-37h cumulativos
  materialização)**. Refinamento pós-audit empírico: **menor
  que P238 reescrito hipotetizou (XL+ ~20-40h)** porque Opção γ
  reusa pattern existente (`apply_state_funcs`).
- Sub-passos atomizados N=5 (M7+1 a M7+5) permitem materialização
  incremental + pausa per sub-passo para reavaliar.

---

## §5 Pré-condições obrigatórias (C6)

3 pré-condições obrigatórias formalizadas (Decisão 4):

### §5.1 Testes baseline preservados

- **2150 verdes pré-M7+1** (baseline pós-P237).
- Cada sub-passo materialização M7+1+ preserva tests existentes
  (adaptações N>0 documentadas explicitamente per spec).
- Regressões reais N=0 mandatório.

### §5.2 Comemo memoization invariants ADR-0073/0074 preservados

- Refactor pipeline walk-time **não-pode quebrar memoization
  correctness**. Audit estrito de invariants:
  - `Introspector` trait paridade vanilla com 20+ métodos M9c
    (ADR-0073 ACEITE).
  - Sub-stores trackable F3 (ADR-0074 ACEITE) preservados.
  - Caller `run_fixpoint` (P174) com loop fixpoint convergência
    preservado.
- Verificação: tests memoization existentes 2150 verdes preservados.

### §5.3 Backward compat eval-time

- P236 `state_final` wrapper eval-time continua funcionar (refino
  semantic two-pass real é refactor non-breaking).
- P237 `state_at` wrapper eval-time continua funcionar.
- 4 stdlib funcs counter (P176/P177/P210B/P190X) preservadas.
- Stdlib funcs 62 baseline pós-P237 preservadas.

---

## §6 ADR meta novo PROPOSTO (C7)

**Decisão C7 confirmada empíricamente — Opção α**: ADR meta novo
PROPOSTO criado em P239.

**Ficheiro**:
`00_nucleo/adr/typst-adr-0081-m7-plus-pipeline-restructuring-scope.md`
PROPOSTO.

**Conteúdo principal**:

- Status: PROPOSTO.
- Escopo: 4 bloqueadores arquiteturais cumulativos (A walk-time
  Func; B multi-region; C Place float; D state.final two-pass)
  + 1 bloqueador adicional identificado P239 (E radius/clip).
- Atomização: 5 sub-passos materialização M7+1 a M7+5.
- Pré-condições obrigatórias: 3 acima.
- Dependencies/ordem propostos: M7+1 → M7+2; M7+3, M7+4, M7+5
  independentes.
- Magnitude cumulativa estimada: L+ a XL (~23-37h cumulativos).
- Não-objectivos:
  - Categoria A render adiada graded P231 (outset/radius/clip
    Block+Boxed semantic adiada preservado per ADR-0079).
  - Outros sub-passos Fase 5 fora M7+.
  - Performance/optimização refactor (escopo correctness +
    funcionalidade apenas).
- Nomenclatura M-fase: **M9d** preliminar (sujeita ratificação
  humana via ADR sob-decisão D1).

**Status ADR-0079** mantido PROPOSTO (preservado pós-P239
documental). **Status ADR-0073/0074** mantidos ACEITE (M8 + F3
chain preservada).

---

## §7 Decisão humana pendente próximo sub-passo materialização M7+ (C10)

P239 produz audit + roadmap. Decisão humana pendente sobre
**primeiro sub-passo materialização M7+1+**:

| Caminho | Trabalho                                         | Magnitude estimada | Desbloqueia                              |
|---------|--------------------------------------------------|--------------------|------------------------------------------|
| **M7+1** | Pipeline walk-time eval (Opção γ apply_state_displays) | **L (~5-8h)** | **D.2 state.display + state.final two-pass real** |
| M7+2    | counter.display paralelo M7+1                    | M (~2-4h)          | counter.display real (depende M7+1)      |
| M7+3    | Multi-region completion cell-level               | L+ (~8-12h)        | C.2 + A.4 breakable per-cell real        |
| M7+4    | Place float real                                 | L (~5-8h)          | C.1                                      |
| M7+5    | A.4 radius/clip infrastructure                   | M-L (~3-5h)        | A.4 radius+clip render real              |
| Pivot   | Outro módulo (Visualize 54%; Text 52%; Model 50%) | varia              | —                                        |
| Pausa M-fase | Fase 5 candidata fecha graded a ~80% preservando bloqueadores como scope-out documentado | XS | — |

**Recomendação subjectiva — M7+1 primeiro**: maior desbloqueio
cumulativo (D.2 state.display + state.final two-pass real via
sobreposição); pattern Opção γ paridade existente
`apply_state_funcs`; magnitude L (~5-8h) — sub-passo materialização
inicial razoável M-fase pipeline.

**Alternativa subjectiva — M7+5 primeiro**: menor magnitude (~3-5h);
infraestrutura geometry isolada (sem dependências pipeline);
ganho user-facing imediato (radius+clip render real).

**Decisão humana fica em aberto literal** pós-P239 materialização.

---

## §8 Critério aceitação P239

| Critério                                                                                              | Esperado                | Real                          |
|-------------------------------------------------------------------------------------------------------|-------------------------|-------------------------------|
| 1 ficheiro audit + roadmap output (`typst-passo-239-audit-m7-reabertura.md` ~15-20 KB)                | ✓                       | ✓ (este ficheiro)             |
| ADR meta novo PROPOSTO criado (`typst-adr-0081-m7-plus-pipeline-restructuring-scope.md`)              | ✓ (per audit C7)        | ✓                             |
| ADR-0079 anotação P239                                                                                | ✓                       | ✓                             |
| Inventário 148 footnote ⁵⁸                                                                            | ✓                       | ✓                             |
| Zero código tocado                                                                                    | ✓                       | ✓                             |
| Tests 2150 verdes preservados                                                                         | 2150                    | **2150**                      |
| 0 violations preservadas                                                                              | 0                       | **0**                         |
| Saldo DEBTs 11 preservado                                                                             | 11                      | **11**                        |
| Roadmap atomização M7+ identificado + priorizado                                                      | ✓                       | ✓ (§4)                        |
| 3 pré-condições obrigatórias formalizadas                                                             | ✓                       | ✓ (§5)                        |
| ADRs distribuição                                                                                     | 12→13 PROPOSTO; 29 EM VIGOR; 21 IMPL; total 67→68 | **13/29/21=68** |

---

## §9 Patterns emergentes inaugurados/consolidados P239

- **"spec audit prévio obrigatório para sub-passos
  walk-time/runtime" N=1 → 2 cumulativo** (P238 reescrito + P239).
- **"prep-passo audit-only preventivo para reabertura M-fase"
  N=1 inaugurado P239** — extensão pattern P238 reescrito.
- **"passo administrativo documental" N=3 → 4 cumulativo**
  (P225; P229; P238 reescrito; **P239**).
- **"ADR meta novo PROPOSTO para reabertura M-fase" N=1
  inaugurado P239**.
- **"atomização prep-passo audit-only + materialização-passo"
  N=1 → 2 cumulativo** (P238 reescrito + **P239**).
- **"L0 minimal para refactors" aplicação automática N=8
  preservado** (P239 administrativo não-incrementa).
- **"audit empírico refina hipótese spec"** N=2 → 3 cumulativo
  (P236.div-1; P237 audit C1; **P239 audit C2.1+C2.2
  `apply_state_funcs` já existe**).

**Anti-inflação 31ª aplicação cumulativa** pós-P205D — Opção α
audit-only + Opção α ADR meta novo + Opção α atomização ADR-0036
+ Opção β saldo DEBTs preservado/decresce + Opção α pré-condições
formalizadas + Opção γ L0 NÃO tocado + Opção α sem promoção
ADR-0079 + Opção α sem materialização imediata.

**Estado pós-P239**:

- Tests workspace: 2150 verdes preservado.
- Violations: 0 preservadas.
- Content variants: 60 preservado.
- Value variants: 55 preservado.
- Stdlib funcs: 62 preservado.
- Layouter fields: preservados.
- §A.5 distribuição: `12/4/2/0/0 = 18` preservada.
- Cobertura Layout per metodologia: **89% preservada**.
- Cobertura user-facing total: 67% preservada.
- **ADRs distribuição**: PROPOSTO **12 → 13** (+ADR-0081 M7+
  scope PROPOSTO); EM VIGOR 29; IMPLEMENTADO 21; total
  **67 → 68**. ADR-0066 SUPERSEDED-BY 0073 preservado.
- Saldo DEBTs: 11 preservado.
- **Fase 5 Layout candidata: 10/13-15 sub-passos materializados
  preservado**; M7+ refactor posterior desbloqueia 3-5
  sub-passos adicionais.
