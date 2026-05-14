# Passo 239 — Prep-passo audit-only reabertura M-fase para M7+ refactor (primeira aplicação real do pattern "atomização prep-passo audit-only + materialização-passo" inaugurado P238 reescrito; **zero código tocado**)

**Série**: 239 (vigésimo-quinto sub-passo Layout pós-M9c;
**prep-passo audit-only** — distinto de materialização;
**primeira aplicação real do pattern "atomização prep-passo
audit-only + materialização-passo" inaugurado P238 reescrito**;
paridade conceitual P226 diagnóstico amplo + P238 reescrito
auditoria documental).
**Marco**: nenhum status ADR; **primeira reabertura M-fase
pós-M9c iniciada via prep-passo audit-only metodologicamente
correcto** (lição P236.div-1 → P238.div-1 refinada
aplicada literal); pattern emergente "prep-passo audit-only
preventivo para reabertura M-fase" N=1 inaugurado P239
(extensão pattern P238 reescrito); ADR meta novo PROPOSTO
provável para formalizar escopo M7+ refactor (sujeito
audit output).
**Tipo**: **passo administrativo documental audit-only**
— zero código tocado; output relatório audit + roadmap
+ possível ADR meta novo PROPOSTO.
**Magnitude**: S (~1-1.5h auditoria; menor que P238
reescrito porque escopo mais focado em blocker walk-time
eval Func dispatch).
**Pré-condição**: P238 reescrito concluído (auditoria
metodológica + plano cobertura Layout; lição refinada
P236.div-1 → P238.div-1 estabelecida formal; reset
metodológico cumulativo); humano fixou caminho **Reabertura
M-fase para M7+ refactor** (decisão arquitectural maior;
paridade alternativa C.1+C.2+D.2+state.final two-pass
real desbloqueio cumulativo); 2150 verdes baseline
preservadas; ADR-0066 SUPERSEDED-BY 0073 terminal preservado;
P172 StateUpdate::Func stub documentado baseline; M9c
pipeline runtime fechado terminal P192C; M8 comemo memoization
ADR-0073/0074 baseline.
**Output**: 1 ficheiro audit + roadmap output
(`00_nucleo/materialization/typst-passo-239-audit-m7-reabertura.md`
~15-20 KB) + **ADR meta novo PROPOSTO provavelmente
necessário** (`typst-adr-XXXX-m7-pipeline-restructuring-scope.md`)
formalizando escopo + ADR-0079 anotação P239 prep-passo
audit-only + footnote ⁵⁸ inventário 148.

---

## §1 Trabalho

P238 reescrito identificou 4 bloqueadores arquiteturais
para fechar Fase 5 Layout completa:
- **A — Walk-time eval Func dispatch** (P172 stub; D.2
  state.display; counter.display; possíveis D.3+).
- **B — Multi-region completion** (DEBT-56b; C.2; A.4
  breakable per-cell P235 graded).
- **C — Place float real** (Opção B P219 reabertura; C.1).
- **D — Pipeline runtime two-pass walk** (state.final
  real two-pass).

P238 reescrito identificou caminho **Reabertura M-fase
para M7+ refactor** como resolução cumulativa para
bloqueadores A + D (e possivelmente B). Magnitude
estimada XL+.

**Aplicação literal da lição refinada P236.div-1 →
P238.div-1**: para sub-passos risco **alto/crítico**,
**prep-passo audit-only obrigatório ANTES de fixar
decisões C2+**. Reabertura M-fase é o **caso máximo de
risco crítico** identificado pós-M9c.

**P239 prep-passo audit-only materializa**:
- **Audit M-fase histórico** (M6/M7/M8/M9/M9c chain).
- **Audit blocker arquitectural walk-time eval Func
  dispatch** (causa raiz; pipeline actual; o que precisa
  estructuralmente).
- **Audit blockers arquiteturais relacionados** (multi-region;
  Place float; state.final two-pass; A.4 radius/clip).
- **Plano atomização sub-passos materialização M7+**
  (identificar atomização viável; magnitude per sub-passo;
  dependencies/ordem).
- **Recomendações pré-materialização** (ADR meta novo
  necessário? atomização ADR-0036 aplicável?).

**P239 NÃO materializa código**. Decisões arquiteturais
M7+ refactor **fixam-se em sub-passos materialização
separados pós-P239**, conforme output empírico do audit.

### Decisão arquitectural central — 8 decisões fixadas

#### Decisão 0 — Prep-passo audit-only obrigatório (lição P238.div-1 aplicada literal)

**Pattern "spec audit prévio obrigatório para sub-passos
walk-time/runtime" N=1 → 2 cumulativo** (P238 reescrito
+ **P239**).

Reabertura M-fase é caso máximo risco crítico — **decisões
C2+ NÃO fixam-se em P239**. P239 produz audit empírico;
sub-passos materialização subsequentes (P240+) fixam
decisões baseadas no audit.

#### Decisão 1 — Escopo audit Opção α (M-fase história + 4 bloqueadores cumulativos)

3 opções:

| Opção | Escopo audit | Trade-off |
|-------|--------------|-----------|
| **α** | M-fase história + 4 bloqueadores (walk-time Func; multi-region; Place float; state.final two-pass) | Audit cumulativo completo permite atomização posterior |
| β | Apenas walk-time Func dispatch (bloqueador A) | Audit focado mas exclui blockers relacionados |
| γ | Audit + materialização imediata sub-passo 1 | Viola lição P238.div-1 literal |

**Decisão fixada — Opção α**: audit cumulativo dos 4
bloqueadores. Atomização sub-passos materialização
posterior baseada no audit empírico.

#### Decisão 2 — ADR meta novo PROPOSTO (provável; sujeito output audit)

3 opções:

| Opção | Mecânica | Trade-off |
|-------|----------|-----------|
| **α** | ADR meta novo PROPOSTO formaliza escopo M7+ refactor | Coerente; M-fases anteriores tiveram ADRs próprios |
| β | Sem ADR meta novo; refactor via ADRs existentes | Insuficiente para escopo XL+ |
| γ | Estender ADR-0079 escopo | ADR-0079 é Fase 5 Layout candidata; M7+ é distinto |

**Decisão fixada (sujeita output audit) — Opção α
provável**: ADR meta novo PROPOSTO formaliza escopo M7+
refactor. Audit confirma se necessário ou se ADR-0073/0074
chain extensão suficiente.

#### Decisão 3 — Atomização ADR-0036 aplicável

P224 + P233 estabeleceram pattern ADR-0036 atomização
para sub-passos algorítmicos isolados (place_cells P224.C
isolado; DEBT-34d fix unitário P233). 

3 opções:

| Opção | Mecânica | Trade-off |
|-------|----------|-----------|
| **α** | Atomização ADR-0036 aplicável a M7+ sub-passos | Pattern estabelecido; permite materialização incremental |
| β | M7+ refactor monolítico | Magnitude XL+ não-atomizada; risco grande |
| γ | Mistura — alguns sub-passos atomizados, outros monolítico | Inconsistente |

**Decisão fixada — Opção α**: atomização ADR-0036
aplicável. Audit identifica sub-passos atomizáveis +
dependencies/ordem.

#### Decisão 4 — Pré-condições obrigatórias

3 conjuntos pré-condições:

| Pré-condição | Mecânica | Crítica |
|--------------|----------|---------|
| **Testes baseline preservados** | 2150 verdes pré-M7+; cada sub-passo materialização preserva | Crítica |
| **Comemo memoization invariants ADR-0073/0074** | Não-tocar invariants ou refactor coerente preservando | Crítica |
| **Backward compat eval-time** | P236+P237 wrappers eval-time continuam funcionar | Importante |

**Decisão fixada**: 3 pré-condições obrigatórias formais.

#### Decisão 5 — Magnitude estimada cumulativa M7+ refactor

Hipótese sujeita audit empírico:

- Walk-time eval Func dispatch infrastructure: L (~5-8h).
- state.final two-pass walk infrastructure: L (~5-8h)
  (sobrepõe com walk-time eval).
- Multi-region completion: L+ (~8-12h).
- Place float real: L+ (~5-8h).
- Sub-passos materialização Categoria D walk-time (D.2;
  counter.display; outros): M-L cada.

**Total cumulativo estimado**: XL+ (~20-40h cumulativos
materialização) — paridade magnitude M-fases anteriores
(M7 + M8 + M9 cumulativos ~similar magnitude).

Audit empírico ajusta estimativas.

#### Decisão 6 — L0 NÃO tocado em P239 (prep-passo administrativo documental)

**Decisão fixada**: P239 é administrativo documental
paridade P225 + P229 + P238 reescrito (zero código tocado).
L0 prompts NÃO tocados. Pattern "L0 minimal aplicação
automática" N=8 preservado (não-incrementa P239 administrativo).

Sub-passos materialização M7+ posteriores tocam L0
conforme escopo cada (walk-time refactor provável toca
`rules/layout.md` ou similar paridade hipótese P236).

#### Decisão 7 — Saldo DEBTs durante M7+ refactor

3 opções:

| Opção | Mecânica | Trade-off |
|-------|----------|-----------|
| α | Saldo DEBTs cresce durante M7+ (novos DEBTs por refactor parcial) | Aceitável temporariamente |
| **β** | Saldo DEBTs preservado/decresce (sub-passos fecham DEBTs paralelos) | Preferível arquiteturalmente |
| γ | DEBT pause (sem novos DEBTs durante M7+) | Inflexível |

**Decisão fixada — Opção β**: saldo DEBTs preservado/decresce.
M7+ refactor pode fechar DEBT-56b (multi-region cell-level)
+ DEBTs relacionados.

#### Decisão 8 — Sem promoção ADR-0079 status; sem fecho Fase 5

P239 prep-passo administrativo — ADR-0079 mantém PROPOSTO;
Fase 5 Layout candidata permanece **10/13-15 sub-passos
materializados** baseline pós-P237.

M7+ refactor materialização posterior:
- **Desbloqueia** D.2 + counter.display + state.final
  two-pass + C.1 + C.2.
- **Pode promover** ADR-0079 PROPOSTO → IMPLEMENTADO se
  todos sub-passos materializados.
- **Pode promover** ADR meta M7+ PROPOSTO → IMPLEMENTADO
  pós-conclusão refactor.

Reuso de dados (sem recolha nova):
- M-fase história baseline (M6/M7/M8/M9/M9c).
- ADR-0066 SUPERSEDED-BY 0073 chain (P204H).
- ADR-0073/0074 comemo memoization baseline (P205B+C+E).
- P172 StateUpdate::Func stub documentado.
- P192C M9c pipeline runtime fechado terminal.
- P226 diagnóstico Fase 5 (4 categorias A+B+C+D).
- P238 reescrito auditoria + plano cobertura Layout
  (4 bloqueadores arquiteturais identificados).
- Pattern emergente "atomização prep-passo audit-only +
  materialização-passo" N=1 baseline P238 reescrito.
- ADR-0079 PROPOSTO Categoria A 5/5 + B 3/3 + D 1/?
  refino estendido baseline P237.

---

## §2 Cláusulas (10 — audit-only estructurais)

### C1 — Auditoria M-fase histórico

Audit empírico imediato:

```
grep -r "M6\|M7\|M8\|M9\|M9c" 00_nucleo/diagnosticos/ 00_nucleo/adr/ 00_nucleo/materialization/ | head -50
ls 00_nucleo/adr/ | grep -i "m[6-9]\|pipeline\|comemo\|memoization"
grep -l "M-fase\|sub-passo.*M[0-9]" 00_nucleo/
```

**Identificar empíricamente**:
- M6 escopo + sub-passos + estado terminal.
- M7 escopo + sub-passos + estado terminal.
- M8 escopo + sub-passos + estado terminal (P208 comemo
  adopt confirmado).
- M9 escopo + sub-passos + estado terminal (P171 state
  runtime; outros).
- M9c escopo + sub-passos + estado terminal (P192C pipeline
  runtime fechado).
- ADRs M-fase relacionados (ADR-0066 SUPERSEDED chain;
  ADR-0073/0074 comemo).

**Output C1**: secção §3.1 do relatório audit estructurado.

**Decisão crítica C1**: identificar onde "M7+" está
conceptualmente:
- M7 fechou pre-M8/M9 — "M7+" significa **M10** (nova
  M-fase)?
- "M7+" significa **re-abertura formal M7**? (improvável
  per chain ADR-0066 → 0073 → 0074 fechado).
- "M7+" significa **M9d** (sub-fase M9 continuação)?
- "M7+" significa **nova M-fase ortogonal** dedicada
  pipeline restructuring?

Audit C1 decide nomenclatura formal.

### C2 — Auditoria blocker arquitectural walk-time eval Func dispatch

Audit empírico:

```
grep -B 5 -A 30 "StateUpdate::Func\|Func::native\|Func::call" 01_core/src/
grep -B 5 -A 20 "EvalContext\|Engine\|World" 01_core/src/contracts/ 01_core/src/rules/eval/
grep -r "stub\|TODO.*walk\|TODO.*eval" 01_core/src/rules/layout/ 01_core/src/rules/introspect.rs
```

**Identificar empíricamente**:
- `StateUpdate::Func` stub P172 — código exacto + linhas.
- `Func::native` constructor signature.
- `EvalContext + Engine + World + FileId + figure_numbering`
  estructura disponibilidade durante walk.
- Outros stubs walk-time identificados.
- Pipeline eval+walk separation actual.

**Identificar resolução estructural necessária**:
- Opção α — Pass `EvalContext` para walk (refactor pipeline
  signatures massivo).
- Opção β — Wrapper sintético `EvalContext` durante walk
  (perigoso; comemo invariants).
- Opção γ — Two-pass walk pipeline (eval primeiro completo;
  walk depois com contexto materializado).
- Opção δ — Outro mecanismo (fixpoint; lazy evaluation;
  outros).

**Output C2**: secção §3.2 do relatório com causa raiz +
opções estructurais + estimativas magnitude per opção.

### C3 — Auditoria blockers arquiteturais relacionados

Audit empírico:

```
grep -B 5 -A 20 "DEBT-56b\|multi-region\|multi_region" 00_nucleo/DEBT.md 01_core/src/
grep -B 5 -A 20 "Place.*float\|Opção B.*P219" 00_nucleo/diagnosticos/ 00_nucleo/adr/
grep -B 5 -A 20 "state.final\|state_final_value" 01_core/src/entities/introspector.rs
grep -B 5 -A 20 "ShapeKind::RoundedRect\|Group::clip" 01_core/src/
```

**Identificar empíricamente**:
- DEBT-56b status + escopo + resolução requerida.
- Place float Opção B P219 graded baseline + reabertura
  escopo.
- state.final two-pass walk dependency (overlapping com
  walk-time eval?).
- A.4 radius/clip infraestrutura (`ShapeKind::RoundedRect`
  ausente; `Group::clip_mask` existe).

**Output C3**: secção §3.3 do relatório com escopo per
bloqueador + estimativa magnitude.

### C4 — Identificar sobreposições entre bloqueadores

**Análise crítica**:
- Walk-time Func dispatch + state.final two-pass partilham
  pipeline restructuring? Provavelmente sim — same
  underlying refactor.
- Multi-region completion partilha pipeline restructuring?
  Provavelmente parcial — pode beneficiar de novo pipeline
  mas requer trabalho cell-level adicional.
- Place float real partilha pipeline restructuring?
  Improvável — refactor Place specific (Opção B P219
  diferente).
- A.4 radius/clip partilha pipeline? Não — infraestrutura
  geometry/clip distinta.

**Identificar atomização viável**:
- Sub-passo M7+ A: pipeline walk-time Func dispatch +
  state.final two-pass (sobreposição grande).
- Sub-passo M7+ B: multi-region completion (independente
  parcial).
- Sub-passo M7+ C: Place float real (independente).
- Sub-passo M7+ D: A.4 radius/clip (independente).

**Output C4**: secção §3.4 do relatório com atomização
proposta + dependencies.

### C5 — Plano atomização sub-passos materialização M7+

Baseline audit C1-C4. Sub-passos materialização propostos:

| Sub-passo | Escopo | Magnitude estimada | Dependencies | Desbloqueia |
|-----------|--------|---------------------|--------------|-------------|
| M7+1 | Pipeline walk-time Func dispatch infrastructure | L (~5-8h) | Nenhuma | D.2 state.display real; counter.display real |
| M7+2 | state.final two-pass walk | M-L (~3-5h) | Opcionalmente M7+1 | state.final real two-pass |
| M7+3 | Multi-region completion cell-level | L+ (~8-12h) | M7+1 parcial | C.2; A.4 breakable per-cell |
| M7+4 | Place float real reabertura | L (~5-8h) | M7+1 parcial | C.1 |
| M7+5 | A.4 radius/clip infrastructure | M-L (~3-5h) | Independente | A.4 radius+clip render real |

**Total cumulativo XL+ (~24-38h materialização)**. Magnitude
paridade M-fases anteriores cumulativas.

**Output C5**: secção §4 do relatório com roadmap atomização
+ estimativas + ordem proposta.

### C6 — Pré-condições obrigatórias formalizadas

3 pré-condições obrigatórias (Decisão 4):

1. **Testes baseline preservados**: 2150 verdes pré-M7+;
   cada sub-passo materialização preserva. Adaptações
   N>0 documentadas explicitamente.
2. **Comemo memoization invariants ADR-0073/0074
   preservados**: refactor pipeline walk-time não-pode
   quebrar memoization correctness. Audit estrito.
3. **Backward compat eval-time**: P236+P237 wrappers
   eval-time (state_final + state_at) continuam funcionar
   inalterados.

**Output C6**: secção §5 do relatório com 3 pré-condições
+ verificação como cláusula obrigatória cada sub-passo
materialização.

### C7 — ADR meta novo PROPOSTO (provável; sujeito C1-C5)

Hipótese provável: ADR meta novo PROPOSTO necessário
formalizar escopo M7+.

**Decisão crítica C7 sujeita audit C1**: 
- **Opção α** — ADR meta novo PROPOSTO
  `typst-adr-XXXX-m7-pipeline-restructuring-scope.md`
  formaliza:
  - Escopo M7+ (4 bloqueadores cumulativos).
  - Atomização sub-passos materialização (M7+1 a M7+5).
  - Pré-condições obrigatórias (3 acima).
  - Dependencies/ordem propostos.
  - Magnitude cumulativa estimada XL+.
  - Não-objectivos (Categoria A render adiada graded
    P231; outros sub-passos Fase 5 fora M7+).
- **Opção β** — ADR-0073/0074 chain extensão suficiente
  (improvável; chain fechado P205B+C+E).
- **Opção γ** — Estender ADR-0079 escopo (incoerente;
  ADR-0079 é Fase 5 Layout candidata; M7+ ortogonal).

**Decisão fixada (sujeita audit C1-C5) — Opção α
provável**: ADR meta novo PROPOSTO criado em P239.

**Output C7**: ficheiro novo
`typst-adr-XXXX-m7-pipeline-restructuring-scope.md` PROPOSTO
ou justificação se desnecessário per audit.

### C8 — Inventário 148 footnote ⁵⁸ + ADR-0079 anotação P239

**Inventário 148**:
- §A.5 Layout entrada relevante (Layout overall): footnote
  ⁵⁷ → ⁵⁷ ⁵⁸.
- Footnote ⁵⁸ adicionada (~80 linhas) documentando P239
  prep-passo audit-only + lição refinada P238 reescrito
  aplicada primeira vez real + roadmap M7+ atomização +
  pré-condições + ADR meta novo PROPOSTO (se criado).

**ADR-0079**:
- §"Aplicações cumulativas" anotado com bloco `### P239
  anotação — Prep-passo audit-only reabertura M-fase para
  M7+ refactor; primeira aplicação real pattern atomização
  prep-passo audit-only + materialização-passo inaugurado
  P238 reescrito`.
- Status ADR-0079 mantido PROPOSTO (preservado pós-P239
  documental).

### C9 — Critério aceitação P239

- 1 ficheiro audit + roadmap output
  (`typst-passo-239-audit-m7-reabertura.md` ~15-20 KB).
- ADR meta novo PROPOSTO criado se audit confirma
  necessidade (provável).
- ADR-0079 anotação P239.
- Inventário 148 footnote ⁵⁸ adicionada.
- **Zero código tocado**.
- Tests 2150 verdes preservados (paridade pattern
  administrativo P225/P229/P238 reescrito).
- 0 violations preservadas.
- Saldo DEBTs 11 preservado.
- Roadmap atomização M7+ identificado + priorizado.
- 3 pré-condições obrigatórias formalizadas.

### C10 — Decisão humana pendente pós-P239

P239 produz audit + roadmap. Decisão humana pendente
sobre **primeiro sub-passo materialização M7+**:

| Caminho | Trabalho | Magnitude estimada |
|---------|----------|-----|
| M7+1 Pipeline walk-time Func dispatch | Desbloqueia D.2 + counter.display | L (~5-8h) |
| M7+2 state.final two-pass walk | Desbloqueia state.final real | M-L (~3-5h) |
| M7+3 Multi-region completion | Desbloqueia C.2 + A.4 breakable | L+ (~8-12h) |
| M7+4 Place float real | Desbloqueia C.1 | L (~5-8h) |
| M7+5 A.4 radius/clip infrastructure | Desbloqueia A.4 radius+clip | M-L (~3-5h) |

**Recomendação subjectiva pós-P239** (sujeita output
audit empírico): **M7+1 Pipeline walk-time Func dispatch
primeiro** — maior desbloqueio cumulativo + sobreposição
com M7+2 (state.final two-pass); permite Categoria D
materialização completa.

---

## §3 Output

1 ficheiro audit + roadmap:
`00_nucleo/materialization/typst-passo-239-audit-m7-reabertura.md`.

Estrutura (~15-20 KB) com 7 §s:

- §1 O que foi feito (sumário 3-5 linhas).
- §2 Decisão arquitectural: prep-passo audit-only
  (lição P238.div-1 aplicada literal).
- §3 Auditoria M-fase + bloqueadores arquiteturais
  (C1+C2+C3).
- §4 Plano atomização sub-passos materialização M7+
  (C4+C5).
- §5 Pré-condições obrigatórias (C6).
- §6 ADR meta novo PROPOSTO (C7) — link ficheiro novo.
- §7 Decisão humana pendente próximo sub-passo
  materialização M7+ (C10).

**Ficheiros novos** (provável):
- `00_nucleo/adr/typst-adr-XXXX-m7-pipeline-restructuring-scope.md`
  PROPOSTO (sujeito audit C1-C5).

**Editado**:
- `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`
  (footnote ⁵⁸ P239).
- `00_nucleo/adr/typst-adr-0079-layout-fase-5-roadmap.md`
  (+ anotação P239 prep-passo audit-only).

**Sem código L1 tocado**. Sem L0 prompts tocados. Sem
novos tests. Sem refactor.

---

## §4 Não-objectivos

- **Materializar M7+ refactor** — P239 é prep-passo
  audit-only; materialização em sub-passos M7+1+ separados.
- **Fixar decisões arquiteturais M7+** — decisões C2+
  fixam-se em sub-passos materialização baseadas em
  audit empírico (lição P238.div-1 literal).
- **Promover ADR-0079 status** — P239 administrativo
  preserva PROPOSTO.
- **Fechar Fase 5 Layout** — M7+ materialização posterior
  desbloqueia fecho potencial.
- **Tocar código L1** — zero código tocado P239.
- **Tocar L0 prompts** — pattern aplicação automática
  preservado.
- **Reabrir ADR-0066 SUPERSEDED** — chain terminal
  preservado.
- **Auditar Categoria C completa pré-M7+** — Categoria C
  bloqueadores audit em P239; materialização em sub-passos
  M7+3 + M7+4 separados.
- **Auditar Categoria A.4 graded refinos pré-M7+** —
  A.4 radius/clip audit em P239; materialização em
  sub-passo M7+5 separado.
- **Auditar pipeline performance/optimização** — escopo
  M7+ é correctness + funcionalidade; performance refactor
  separado.
- **Promoção formal patterns emergentes** — P229 estabeleceu
  pattern singular; promoções formais separadas candidatos
  XS pós-M7+.

---

## §5 Riscos a evitar

1. **Audit C1 revela M-fase nomenclatura ambígua** — "M7+"
   pode significar M10 nova OR re-abertura M7 (fechada).
   Mitigação: audit empírico decide; ADR meta novo PROPOSTO
   formaliza nomenclatura.
2. **Audit C2 revela walk-time eval mais simples que
   hipótese** — possível que `EvalContext` seja passable
   trivialmente. Mitigação: audit empírico ajusta
   estimativas.
3. **Audit C2 revela walk-time eval mais complexo que
   hipótese** — possível refactor pipeline maior. Mitigação:
   audit empírico identifica; magnitude estimada ajustada.
4. **Audit C3 revela sobreposições maiores entre bloqueadores**
   — atomização proposta C4-C5 pode ser inadequada. Mitigação:
   audit refina atomização.
5. **Audit revela bloqueadores adicionais não-identificados
   P238 reescrito** — possível 5+ bloqueador identificado.
   Mitigação: audit identifica; roadmap actualizado.
6. **Comemo memoization invariants ADR-0073/0074 quebram
   facilmente** — refactor pipeline walk-time pode invalidar
   memoization. Mitigação: pré-condição C6.2 obrigatória
   formal.
7. **Tests 2150 verdes não-preservados durante M7+
   refactor** — refactor pipeline maior pode quebrar tests
   significativos. Mitigação: pré-condição C6.1; adaptações
   N>0 documentadas explicitamente per sub-passo.
8. **Magnitude cumulativa M7+ refactor exceder XL+** —
   possível ~50+h se complexidades não-previstas. Mitigação:
   sub-passos materialização atomizados; pausa entre
   sub-passos para reavaliar.
9. **ADR meta novo PROPOSTO criado desnecessariamente** —
   se ADR-0073/0074 extensão suficiente. Mitigação: audit
   C7 confirma necessidade empírica.
10. **P239 audit insuficientemente detalhado** — sub-passos
    materialização M7+1+ encontram divergências factuais
    material (novo div-N). Mitigação: P239 dedica tempo
    auditoria empírica detalhada — magnitude S (~1-1.5h)
    é mínimo aceitável; magnitude M aceitável se audit
    revela mais.
11. **Materialização imediata pós-P239 tentada** — humano
    pode querer materializar M7+1 imediatamente. Mitigação:
    P239 spec §6 enfatiza decisão humana pendente; sub-passo
    materialização separado mandatório.
12. **Documentar M7+ em L0** — tentação por "M-fase
    importante". Rejeitada — P239 administrativo;
    documentação em footnote ⁵⁸ + ADR meta novo PROPOSTO.

---

## §6 Hipótese provável

C1 confirmará M-fase história baseline:
- M6 = compilação modular fechado.
- M7 = fechado pré-M8/M9 (audit C1 confirma escopo
  exacto).
- M8 = comemo memoization adopt fechado P205B+C+E
  (ADR-0073/0074).
- M9 = state runtime + pipeline integration baseline
  P171+P172.
- M9c = pipeline runtime fechado terminal P192C.
- **"M7+" → provavelmente M10 (nova M-fase)** dedicada
  pipeline restructuring para walk-time eval.

C2 confirmará blocker walk-time eval Func dispatch:
- `EvalContext + Engine + World + FileId + figure_numbering`
  estructura complexa.
- Opção mais provável estructural: **two-pass walk pipeline**
  (eval completo primeiro; walk depois com contexto
  materializado).

C3 confirmará blockers relacionados:
- state.final two-pass overlap walk-time eval (mesmo
  refactor).
- Multi-region completion parcial overlap.
- Place float real independente.
- A.4 radius/clip independente.

C4 identificará atomização viável:
- M7+1 walk-time Func dispatch + state.final two-pass
  (combinado).
- M7+2 multi-region completion.
- M7+3 Place float real.
- M7+4 A.4 radius/clip infrastructure.

C5 proporá roadmap atomização (4 sub-passos materialização
M7+).

C6 formalizará 3 pré-condições obrigatórias.

C7 criará ADR meta novo PROPOSTO formalizando escopo.

C8 anotará inventário 148 footnote ⁵⁸ + ADR-0079 P239.

C9 verifica critério aceitação.

C10 identificará M7+1 como recomendação subjectiva
primeiro sub-passo materialização.

Custo real: **S (~1-1.5h auditoria)** — focado em escopo
M7+ refactor.

Mas é hipótese, não decisão. C1-C10 fixam-se empíricamente.
Possível ajustes per audit empírico.

---

## §7 Particularidade P239

P239 é estruturalmente distinto **muito significativo**
na trajectória pós-M9c:

- **Vigésimo-quinto sub-passo Layout pós-M9c**.
- **Primeira aplicação real do pattern "atomização
  prep-passo audit-only + materialização-passo" inaugurado
  P238 reescrito** — N=1 inaugurado primeira vez real.
- **Primeira reabertura M-fase pós-M9c iniciada
  metodologicamente correctamente** — lição P236.div-1 →
  P238.div-1 refinada aplicada literal.
- **Primeiro prep-passo audit-only para reabertura M-fase**
  — distinto cumulativo de P226 (diagnóstico amplo
  Fase 5 candidato; não M-fase) + P238 reescrito (auditoria
  pós-falhanços; não M-fase reabertura).
- **Pattern emergente "prep-passo audit-only preventivo
  para reabertura M-fase" N=1 inaugurado P239** —
  extensão pattern P238 reescrito.
- **Sem código tocado** — paridade pattern administrativo
  P225 + P229 + P238 reescrito + **P239 cumulativo**.
- **Pattern "L0 minimal aplicação automática" N=8
  preservado** — P239 administrativo documental não-incrementa.
- **ADR meta novo PROPOSTO provável** — primeiro ADR meta
  novo pós-P229 (que promoveu ADR-0080 EM VIGOR singular).
  Distribuição ADRs PROPOSTO 12 → **13** se criado.
- **Cobertura Layout per metodologia preservada 89%
  preservada** — P239 administrativo.
- **Anti-inflação 31ª aplicação cumulativa** pós-P205D
  — Opção α audit-only + Opção α ADR meta novo (se audit
  confirma) + Opção α atomização ADR-0036 aplicável +
  Opção β saldo DEBTs preservado/decresce + Opção α
  pré-condições formalizadas + Opção γ L0 NÃO tocado +
  Opção α sem promoção ADR-0079 + Opção α sem materialização
  imediata.

Por isso §5 risco 11 (materialização imediata pós-P239
tentada) é o mais provável metodologicamente. **Defesa**:
spec §6 + §C10 enfatizam decisão humana pendente; sub-passo
materialização separado mandatório paridade lição P238.div-1
literal.

**Critério de aceitação P239**:
- 1 ficheiro audit + roadmap output (~15-20 KB).
- ADR meta novo PROPOSTO criado se audit C1-C5 confirma.
- ADR-0079 anotação P239.
- Inventário 148 footnote ⁵⁸ adicionada.
- Zero código tocado.
- Tests 2150 verdes preservados.
- 0 violations preservadas.
- Saldo DEBTs 11 preservado.
- Roadmap M7+ atomização identificado.
- 3 pré-condições obrigatórias formalizadas.

**Estado pós-P239 esperado**:
- Tests workspace: 2150 verdes preservado.
- Violations: 0 preservadas.
- Content variants: 60 preservado.
- Value variants: 55 preservado.
- Stdlib funcs: 62 preservado.
- Grid/Table/Cell/Block/Boxed/Place fields preservados.
- Layouter fields: preservados.
- §A.5 distribuição: `12/4/2/0/0 = 18` preservada.
- Cobertura Layout per metodologia: **89% preservada**.
- Cobertura user-facing total: 67% preservada.
- **ADRs distribuição**: PROPOSTO **12 → 13** (+1 ADR
  meta novo M7+ scope) SE audit confirma; EM VIGOR 29;
  IMPLEMENTADO 21; total 67 → **68** SE ADR meta novo
  criado.
- ADR-0066 SUPERSEDED-BY 0073 preservado.
- Saldo DEBTs: 11 preservado.
- **31 aplicações cumulativas anti-inflação** pós-P205D.
- **Pattern "L0 minimal para refactors" aplicação
  automática N=8 preservado** (P239 administrativo
  não-incrementa).
- **Pattern "atomização prep-passo audit-only +
  materialização-passo" N=1 → 2 cumulativo** (P238
  reescrito + **P239**).
- **Pattern emergente "prep-passo audit-only preventivo
  para reabertura M-fase" N=1 inaugurado P239** — extensão
  pattern P238 reescrito.
- **Pattern "passo administrativo documental" N=3
  cumulativo** (P225; P229; P238 reescrito; **P239**) —
  pattern empíricamente sólido.
- **Pattern emergente "ADR meta novo PROPOSTO para
  reabertura M-fase" N=1 inaugurado P239** SE audit
  C7 confirma necessidade.
- **Marco interno implícito**: primeira reabertura M-fase
  pós-M9c via prep-passo audit-only metodologicamente
  correctamente; lição refinada P236.div-1 → P238.div-1
  validada empíricamente.
- **Fase 5 Layout candidata: 10/13-15 sub-passos
  materializados preservado** (~67-77%); M7+ refactor
  posterior desbloqueia 3-5 sub-passos adicionais (D.2
  + counter.display + state.final real two-pass + C.1 +
  C.2 + A.4 graded refinos).

**Próximo sub-passo pós-P239 (decisão humana pendente)**:

Conforme output audit empírico P239:
- Sub-passo materialização M7+1 (recomendação subjectiva):
  Pipeline walk-time Func dispatch + state.final two-pass
  walk combinado.
- Magnitude estimada L (~5-8h) — primeiro sub-passo
  materialização M7+.
- Desbloqueia: D.2 state.display real + counter.display
  real + state.final real two-pass.

Alternativa: humano escolhe outro sub-passo M7+2/M7+3/
M7+4/M7+5 conforme prioridade subjectiva.

**Decisão humana fica em aberto literal** pós-P239
materialização.
