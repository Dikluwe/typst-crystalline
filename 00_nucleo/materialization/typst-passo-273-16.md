# Passo P273.16 — Bbox.y topo-exacto inline (Fase A com verificação bloqueador DEBT-56)

**Tipo**: refino qualitativo opcional — substituir Decisão 1 P273.7 (bbox.y baseline-relative em Boxed inline) por bbox.y topo-relative correctamente computado.
**Magnitude estimada**: **indeterminada** — M-L se viável; **bloqueado por DEBT-56** com predição NO-GO obrigatório por construção.
**Pré-requisitos**: P273.15 fechado (SCOPE-OUT-RECONFIRMED por ausência de demanda).
**Cluster**: Visualize / Gradient (sétimo e último sub-passo na sequência terminar cluster — escopo máximo decidido).
**Aplica ADRs**: ADR-0091 (centro de aplicação ColorSpace; décima sexta anotação cumulativa); ADR-0029 (pureza física L1); ADR-0093 (Pattern 2 anotação cumulativa); ADR-0054 (graded — NO-GO esperado).

---

## §0 — Contexto

P273.7 §A.3 documentou três opções para semântica de `bbox.y` em Boxed inline:

- **3γ.2.γ-inline-baseline-y** (escolha P273.7): `bbox.y = cursor.y` literal (baseline-relative). Aproximação aceitável; coerente com limitação consciente P156H "height em contexto inline alteraria line_height — refino futuro".
- **3γ.2.γ-inline-topo-estimado**: `bbox.y = cursor.y - ascender` (subtraindo ascender da font corrente). Mais correcto semanticamente; introduz dependência adicional na font_metrics no arm save/restore.
- **3γ.2.γ-inline-defer**: Não popular `parent_bbox` no Boxed enquanto refactor inline line_height não existir.

P273.7 escolheu **3γ.2.γ-inline-baseline-y** (Decisão 1) — aproximação documentada como limitação consciente. Pendência registada P273.7 §8 + preservada em P273.9-P273.15:

> "**P273.X-bis2** — Bbox.y topo-exacto inline (refino Decisão 1 se aproximação baseline-y for visualmente insuficiente; **requer refactor inline line_height — diferido permanente per ADR-0054 graded; coerente com P156H**)."

### Bloqueador DEBT-56

**DEBT-56** registado em Passo 156B (2026-04-25) durante diagnóstico Layout (Fase X). Bloqueia:
- Fase 3 Layout (columns/colbreak).
- Refactor multi-region do `Layouter`.
- Refino inline line_height (referenciado por P156H limitação consciente e por P273.7 Decisão 1).

DEBT-56 é **escopo L+** (~5-8 horas em passo dedicado). Tem **6 pré-requisitos** próprios (ADR dedicada column flow algorithm, refactor Layouter Region/Regions, etc.). Está **EM ABERTO** desde 2026-04-25.

Pós-P273.7, bbox.y topo-exacto inline depende de:

1. Refactor `Layouter` para suportar line_height ajustável por arm inline (parte do refactor multi-region per DEBT-56).
2. **OU** acesso à `font_metrics.ascender` no arm Boxed sem refactor (caminho menos ambicioso; pode funcionar).

### Diferença com P273.14 e P273.15

| Aspecto | P273.14 | P273.15 | P273.16 (este) |
|---|---|---|---|
| Bloqueador | Constraints externas (licensing + invariante L0) | Constraints internas (perf + demanda) | **Bloqueador estrutural registado (DEBT-56)** |
| Predição da spec | NO-GO provável | NO-GO provável | **NO-GO obrigatório por construção** |
| Decisão Fase A | Verificar empíricamente | Verificar empíricamente | **Confirmar bloqueador + documentar** |
| Pré-requisitos GO | 3 (ADR + profile + licença) | 2 (demanda + decisão perf) | **DEBT-56 fechado** (ou caminho alternativo viável) |

P273.16 é **diferente em natureza** — não é "verificar se há demanda" nem "verificar se viável", é **confirmar bloqueador estrutural documentado**. A Fase A pode ainda assim descobrir caminho alternativo (e.g. via font_metrics sem refactor Layouter), mas o resultado esperado é NO-GO bloqueado por DEBT-56.

### Predição factual da Fase A

Trajectória provável da Fase A (auto-avaliação honesta):

- §A.1 inventário bloqueador → DEBT-56 confirmado registado em `00_nucleo/DEBT.md`; coerência com P156H limitação consciente.
- §A.2 caminho alternativo via font_metrics ascender → possível mas requer dependência adicional no arm save/restore + risco de divergir do que vai vir quando DEBT-56 for fechado.
- §A.3 decisão → **NO-GO obrigatório bloqueado por trabalho externo (DEBT-56)**, com sub-padrão "Scope-out reconfirmado por Fase A" cresce N=2 → N=3 cumulativo crossing limiar formalização N=3-4.

**Esta predição é mais firme que P273.14/P273.15** — o bloqueador é factual desde 2026-04-25.

---

## §1 — Sub-passo P273.16.A — Fase A diagnóstico (com decisão go/no-go)

**Magnitude**: S documental (~20-30 min — mais curto que P273.14/P273.15 porque bloqueador já está bem documentado).
**Output**: `00_nucleo/diagnosticos/typst-passo-273-16-diagnostico.md`.

### §A.1 — Inventário do bloqueador DEBT-56

Listar literal em `00_nucleo/DEBT.md`:

- DEBT-56 — Column flow Fase 3 Layout (L+; refactor multi-region do Layouter). EM ABERTO desde Passo 156B (2026-04-25).
- Pré-requisitos próprios (6 items documentados).
- Bloqueia: Fase 3 Layout columns/colbreak + refino multi-region inline.

Listar literal em `00_nucleo/prompts/entities/content.md` (P156H Boxed limitações conscientes):

- "`height` em contexto inline alteraria line_height — refino futuro."
- "`baseline` exige offset vertical mid-linha — não suportado por cursor.rs actual."
- "`inset.top`/`inset.bottom` em contexto inline são complexos; armazenados mas não aplicados."

Confirmação empírica: DEBT-56 ainda está EM ABERTO; nenhum sub-passo P273.x tocou nele.

### §A.2 — Inventário de caminhos alternativos

3 caminhos possíveis para P273.16:

#### Caminho 1: Refactor inline line_height (resolve DEBT-56 parcialmente)

- Magnitude: L+ (escopo de DEBT-56 inteiro ou sub-fase).
- Pré-requisito: ADR dedicada Layout multi-region.
- **Fora do escopo P273.16** (escopo S-M cluster Gradient).

#### Caminho 2: font_metrics.ascender no arm Boxed (sem refactor)

- L1 arm `Content::Boxed` ganha acesso a `self.font_metrics.ascender` no momento do save/restore.
- bbox.y = `cursor.y - ascender_pt` (em vez de `cursor.y` literal).
- Magnitude: S (~15-25 LOC L1).
- **Risco**: caminho ad-hoc que pode divergir de quando DEBT-56 for fechado. Cria dívida invisível.

#### Caminho 3: Scope-out preserved

- 3γ.2.γ-inline-baseline-y P273.7 preserved literal.
- Decisão P273.7 §A.3 mantida.
- Trabalho prévio externo documentado: DEBT-56 fechamento.

### §A.3 — Decisão go/no-go primária

A Fase A toma uma decisão binária:

- **GO**: materializa Caminho 1 ou 2. **Predição negada** — caminho 1 fora de escopo; caminho 2 requer cuidado para não criar dívida invisível.
- **NO-GO**: scope-out preserved bloqueado por DEBT-56. Fase B+C reduzidas a documentação.

### §A.4 — Critério para GO

A Fase A só decide GO se:

1. Caminho 2 (font_metrics) escolhido E demanda empírica concreta para bbox.y topo-exacto (não apenas "ficou pendente").
2. Decisão deliberada de que caminho ad-hoc não cria dívida invisível (e.g. fácil de remover quando DEBT-56 fechar).

Caso 1 não cumprido → **NO-GO automaticamente**.

### §A.5 — Critério para NO-GO

A Fase A decide NO-GO se:

1. §A.1 confirma DEBT-56 EM ABERTO (esperado factualmente).
2. Caminho 1 fora de escopo P273.16 (escopo S-M vs DEBT-56 L+).
3. Caminho 2 cria dívida ad-hoc invisível ou sem demanda empírica.
4. 3γ.2.γ-inline-baseline-y P273.7 aceito por ADR-0054 graded e coerente com P156H limitação consciente.

NO-GO **não é falha** — é cumprimento honesto do critério "verificar bloqueador" registado em P273.7-P273.15.

### §A.6 — Análise de risco

| Risco | Fonte | Mitigação |
|---|---|---|
| Dívida invisível por caminho ad-hoc | Caminho 2 cria mecânica que não se alinha com refactor futuro | §A.4 critério 2 explícito |
| Refino sem demanda vira over-engineering | Análogo P273.15 | §A.4 critério 1 obrigatório |
| Bloqueador interpretado como falha | NO-GO confundido com regressão | §A.5 explicita |
| DEBT-56 fechamento não está no horizonte próximo | Trabalho externo necessário | Sub-padrão "Scope-out reconfirmado por Fase A" N=3 cumulativo crossing limiar formalização |

### §A.7 — Decisões a fixar na Fase A

1. **Decisão 1** (caminho): 1 / 2 / 3 consoante §A.1-A.5.
2. **Decisão 2 (apenas se GO)**: detalhes de implementação consoante caminho escolhido.
3. **Decisão 3 (sempre)**: documento de trabalho prévio externo se NO-GO — referência a DEBT-56 + pré-requisitos.

### §A.8 — Critério de aceitação Fase A

Independente de go/no-go:

- §A.1 cita DEBT-56 literal (path:linha em `00_nucleo/DEBT.md`).
- §A.2 inventário dos 3 caminhos com magnitude e risco concretos.
- §A.3 decisão go/no-go fixada com fundamento literal.
- Se NO-GO: documento de trabalho prévio externo produzido referenciando DEBT-56.

---

## §2 — Sub-passo P273.16.B — Anotação cumulativa ADR-0091

**Magnitude**: XS documental (independente de go/no-go).

Anotar ADR-0091 — décima sexta anotação consecutiva.

### Template se GO (improvável)

```
## Anotação cumulativa P273.16 — Bbox.y topo-exacto inline (caminho 2 font_metrics)

**Data**: 2026-05-XX.
**Decisão**: GO via caminho 2 (font_metrics.ascender no arm Boxed).
**Caso empírico**: [descrição literal].
**Risco de dívida invisível mitigado**: [como o caminho ad-hoc se alinha com
DEBT-56 futuro fechamento].
**Sub-padrão "Refino qualitativo opcional materializado"** N=0 → N=1 inaugural.
**Layout duplo arquitectural aceite** N=1 (P273.9 Stack/Pad preserved; P273.16
não introduz layout duplo — só font_metrics access).
```

### Template se NO-GO (esperado)

```
## Anotação cumulativa P273.16 — Bbox.y topo-exacto scope-out reconfirmado
(bloqueado DEBT-56)

**Data**: 2026-05-XX.
**Decisão**: NO-GO via §A.5 critério [DEBT-56 EM ABERTO + caminho 1 fora de
escopo + caminho 2 dívida ad-hoc + 3γ.2.γ-inline-baseline-y aceito ADR-0054
graded].
**Trabalho prévio externo necessário**: ver
`00_nucleo/diagnosticos/typst-passo-273-16-trabalho-previo-externo.md`.
**Decisão P273.7 Fase A (3γ.2.γ-inline-baseline-y) preserved literal** —
P156H limitação consciente reconfirmada.
**Sub-padrão "Scope-out reconfirmado por Fase A"** N=2 → **N=3 cumulativo
crossing limiar formalização N=3-4**. Padrão consolidado tripla aplicação:
- N=1 (P273.14): constraints externas (CMYK-ICC profile licensing).
- N=2 (P273.15): constraints internas (custo perf + ausência demanda).
- N=3 (P273.16): bloqueador estrutural registado (DEBT-56).

**Cluster Gradient atinge feature-complete declarável** — 3 sub-passos
consecutivos scope-out reconfirmados sinalizam saturação completa do escopo
cluster.
```

---

## §3 — Sub-passo P273.16.C — Materialização (só se GO, improvável)

**Magnitude**: S consoante caminho 2.

Caminho 2 (font_metrics) — improvável dada predição NO-GO obrigatório.

### Cap LOC (só se GO)

- **L1 hard cap**: ≤ 30 LOC.
- **L1 soft cap**: ≤ 20 LOC.
- **L3 hard cap**: 0 LOC.
- **Tests hard cap**: ≤ 5.
- **Tests soft cap**: ≤ 3.

---

## §4 — Sub-padrões cumulativos pós-P273.16

### Se GO (improvável)

| Sub-padrão | Pós-P273.15 | Pós-P273.16 (GO) |
|---|---|---|
| Anotação cumulativa em vez de ADR nova | 22 | 23 |
| Cap LOC hard vs soft explícito | 16 | 17 |
| Aplicação meta-ADR (ADR-0094) | 12 | 13 |
| Sub-passos consecutivos do mesmo cluster | N=11 | **N=12 cumulativo emergente** |
| Diagnóstico imutável | 31 | 32 (27º consumo) |
| **Refino qualitativo opcional materializado** | N=0 | **N=1 inaugural** |

### Se NO-GO (esperado)

| Sub-padrão | Pós-P273.15 | Pós-P273.16 (NO-GO) |
|---|---|---|
| Anotação cumulativa em vez de ADR nova | 22 | 23 |
| Aplicação meta-ADR (ADR-0094) | 12 | 12 (preserved — sem cap aplicado) |
| Sub-passos consecutivos do mesmo cluster | N=11 | **N=12 cumulativo emergente** |
| Diagnóstico imutável | 31 | 32 (27º consumo) |
| **Scope-out reconfirmado por Fase A** | N=2 cumulativo | **N=3 cumulativo crossing limiar formalização N=3-4** |

Sub-padrão **"Scope-out reconfirmado por Fase A" atinge N=3** se NO-GO confirmado — limiar formalização ADR meta atingido com 3 aplicações:
- N=1 P273.14: constraints externas.
- N=2 P273.15: constraints internas (custo perf).
- N=3 P273.16: bloqueador estrutural registado.

3 razões NO-GO distintas. **Candidato meta-ADR formalização NÃO reservado**.

---

## §5 — Limitações conscientes P273.16

Se GO:
- Caminho 2 (font_metrics) cria mecânica ad-hoc que pode divergir do refactor DEBT-56 quando materializado.
- Risco de dívida invisível — código adicionado agora pode precisar reverter quando DEBT-56 fechar.

Se NO-GO:
- 3γ.2.γ-inline-baseline-y continua a ser bbox.y para gradient `relative=parent` aninhado em Boxed inline.
- Aproximação aceitável per ADR-0054 graded.
- Refino futuro depende fechamento DEBT-56 (escopo L+; 6 pré-requisitos próprios).

---

## §6 — Workflow operacional

1. Utilizador lê esta spec.
2. Utilizador executa Fase A em Claude Code → diagnóstico **com decisão go/no-go**.
3. Utilizador upload do diagnóstico.
4. Claude web valida critério §A.8.
5. **Se GO**: utilizador executa P273.16.B + P273.16.C → relatório com materialização.
6. **Se NO-GO**: utilizador executa P273.16.B + produz `trabalho-previo-externo.md` → relatório com NO-GO documentado.
7. Utilizador upload do relatório.
8. Claude web analisa + **declara cluster Gradient feature-complete** (sequência fixada esgotada).

---

## §7 — Pendências preservadas pós-P273.16

Inalteradas vs P273.15:

- **ADR-0055bis variant-aware fonts** (M).
- **P-Footnote-N** (M).
- **DEBT-33 Bézier bbox** (S+M).
- **Stroke\<Length\> / Curve / Polygon** (S+M).
- **Tiling activação** (Paint::Tiling).
- **Outro cluster — saída Visualize/Gradient**.

Pendências específicas pós-P273.16:

- **P273.X-bis-helper-group-bbox** — extract helper compartilhado 3 sítios (P273.13 §9). NÃO reservado.
- **P-Gradient-CMYK-ICC** preserved scope-out (P273.14 NO-GO).
- **P273.X-bis-bbox-medido-pos-layout** preserved scope-out (P273.15 NO-GO).
- **P273.X-bis2-bbox-y-topo-exacto-inline** preserved scope-out (P273.16 NO-GO esperado).

Pendência implícita exposta P273.13 §9 segundo bullet (fora cluster):
- **P273.X-bis-draw-item-local-text-image** — Text + Image em Groups silenciosamente descartados.

**Cluster Gradient declarável feature-complete pós-P273.16**:
- ✓ Feature-complete user-facing (cross-variant runtime fields 3/3; gradient.linear/radial/conic).
- ✓ Adaptive N qualitativo (P274).
- ✓ Refino estrutural extensivo (P273.5-P273.13 cobrindo Block + Boxed + Grid + Stack + Pad + Group).
- ✓ Cleanup intra-cluster (P273.8 + P273.11).
- ✓ Dedup bbox-aware (P273.12).
- ✓ Render real Groups via pattern dict (P273.13).
- ✓ CMYK-ICC scope-out reconfirmado (P273.14).
- ✓ Bbox medido pós-layout scope-out reconfirmado (P273.15).
- ✓ Bbox.y topo-exacto scope-out reconfirmado (P273.16; esperado).

---

## §8 — Critério de fecho do passo

P273.16 fecha com **IMPLEMENTADO** (GO; improvável) ou **SCOPE-OUT-RECONFIRMED** (NO-GO; esperado).

### IMPLEMENTADO (GO; improvável)

- Fase A produzida + critério §A.8 cumprido + decisão GO.
- ADR-0091 anotada (décima sexta consecutiva — versão GO).
- L1 alterado dentro do cap LOC.
- Tests workspace verdes (cap respeitado).
- Lint zero.
- Tests P262-P273.15 inalterados bit-exact.

### SCOPE-OUT-RECONFIRMED (NO-GO; esperado)

- Fase A produzida + critério §A.8 cumprido + decisão NO-GO.
- ADR-0091 anotada (décima sexta consecutiva — versão NO-GO).
- Documento `trabalho-previo-externo.md` produzido (referenciando DEBT-56).
- Zero alterações ao código L1/L3.
- Tests workspace 2644 preserved.
- **Sub-padrão "Scope-out reconfirmado por Fase A" N=3 cumulativo crossing limiar formalização N=3-4** — candidato meta-ADR.

---

## §9 — Numeração — último da sequência

Spec usa **P273.16** completando a sequência decimal pós-inserção P273.13. **Sétimo e último** sub-passo materializado da sub-sequência "terminar cluster Gradient" (escopo máximo decidido).

Sequência completa:

- ✓ P273.5 — relative cross-variant.
- ✓ P273.6 — Block save/restore.
- ✓ P273.7 — Boxed save/restore.
- ✓ P273.8 — Cleanup 4 warnings.
- ✓ P273.9 — Grid + Stack + Pad.
- ✓ P273.10 — Group L3-only scan.
- ✓ P273.11 — Extract Stack helper.
- ✓ P273.12 — Dedup bbox-aware.
- ✓ P273.13 — Fix draw_item_local Group (INSERIDO).
- ✓ P273.14 — CMYK-ICC paridade (SCOPE-OUT-RECONFIRMED).
- ✓ P273.15 — Bbox medido pós-layout (SCOPE-OUT-RECONFIRMED).
- **P273.16** — Bbox.y topo-exacto inline (este passo; SCOPE-OUT-RECONFIRMED esperado).

**Total**: 12 sub-passos consecutivos do cluster Gradient (P273.5-P273.16).

Pós-P273.16, sequência "terminar cluster Gradient" considera-se esgotada. Cluster declarável feature-complete + qualitativo + refino estrutural extensivo + cleanup + dedup bbox-aware + render Groups + 3 scope-outs documentados (CMYK-ICC + Bbox pós-layout + Bbox.y topo-exacto).

Próximo passo natural após P273.16: **sair do cluster Gradient** definitivamente. Pendências restantes do projecto (ADR-0055bis fonts, P-Footnote-N, DEBT-33 Bézier, Stroke/Curve/Polygon, Tiling, outro cluster) ficam disponíveis.

---

## §10 — Reflexão metodológica

A sequência P273.5-P273.16 documentou:

- **9 sub-passos materializados** (P273.5-P273.13).
- **3 sub-passos scope-out reconfirmados** (P273.14-P273.16 se predição confirmada).
- **12 sub-passos consecutivos** total — caminho mais longo documentado no projecto cristalino.
- **15 anotações cumulativas ADR-0091** (sub-padrão consolidação consolidada).
- **N sub-padrões emergentes** inaugurados ou consolidados ao longo da sequência.

P273.16 é o passo que **fecha o ciclo metodológico** — sub-padrão "Scope-out reconfirmado por Fase A" atingirá N=3 (limiar formalização). Cluster Gradient torna-se laboratório metodológico documentado do projecto.

Trajectória honesta: cluster começou em P262 como feature básica; cresceu para refino estrutural completo via P273-P273.13; consolidou-se com 3 scope-outs documentados como cumprimento de critérios "verificar empíricamente". Output final é cluster funcional + dívida documentada + sub-padrões consolidados.
