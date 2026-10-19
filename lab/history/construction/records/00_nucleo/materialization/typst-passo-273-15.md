# Passo P273.15 — Bbox medido pós-layout (Fase A com verificação demanda empírica)

**Tipo**: refino qualitativo opcional — substituir Decisão 3γ.2.γ (popular `parent_bbox` apenas com dimensions literais) por 3γ.2.β (medição via `measure_content_constrained` para containers sem dimensions).
**Magnitude estimada**: **indeterminada** — S-M se viável + útil; **trabalho prévio externo ou scope-out** se inviável (Fase A decide).
**Pré-requisitos**: P273.14 fechado (CMYK-ICC scope-out reconfirmado).
**Cluster**: Visualize / Gradient (sexto sub-passo na sequência terminar cluster).
**Aplica ADRs**: ADR-0091 (centro de aplicação ColorSpace; décima quinta anotação cumulativa se materializado); ADR-0029 (pureza física L1); ADR-0093 (Pattern 2 anotação cumulativa); ADR-0054 (graded — caso GO ou NO-GO ambos legítimos).

---

## §0 — Contexto

P273.6 §A.3 documentou três opções para semântica de `parent_bbox` em containers:

- **3γ.2.α — bbox pré-layout (estimativa)**: campo populated antes de `layout_content(body)` com `width`/`height` resolvidos ou estimativa (page_width − cursor.x para w; ∞ ou altura conservadora para h). Disponível ao body durante layout. **Custo**: alto risco de bbox errada quando `width=None` e o body determina a largura.

- **3γ.2.β — bbox pós-layout (medição)**: medir o body com `measure_content_constrained`, popular `parent_bbox`, depois fazer `layout_content(body)` novamente para emitir. **Custo**: layout duplo.

- **3γ.2.γ — bbox pré-layout com fields conhecidos** (escolha actual): popular `parent_bbox` apenas quando `width` *e* `height` são `Some(...)`; caso contrário deixar `None` (cai no fallback page_bbox do callsite L3 P273.5). **Custo**: gradient `relative=parent` aninhado em Block sem dimensions explícitas continua a usar page_bbox.

P273.6 escolheu 3γ.2.γ para Block. P273.9 estendeu 3γ.2.γ para Grid cell (com dimensions sempre literais via `body_w`/`body_h`) + 3γ.2.β para Stack/Pad (layout duplo aceito).

**P273.15 propõe estender 3γ.2.β para Block sem dimensions literais** — quando `width=None` ou `height=None`, medir o body via `measure_content_constrained` antes de popular `parent_bbox`.

### Análise de demanda empírica

A pendência registada em P273.6 §A.3 + P273.7 §8 + P273.9 §8 + P273.10 §8 + P273.12 §8 + P273.13 §8 + P273.14 §6:

> "**P273.X-bis** — Bbox medido pós-layout (refino 3γ.2.β/α se 3γ.2.γ for empiricamente insuficiente)."

A condição "se 3γ.2.γ for empiricamente insuficiente" foi **condicional desde o início**. Em 8 sub-passos consecutivos, nenhum relatório registou caso real onde 3γ.2.γ produziu output incorrecto. Boxed (P273.7) usa 3γ.2.γ; Block (P273.6) usa 3γ.2.γ; Grid (P273.9) usa 3γ.2.γ (com dimensions sempre literais por construção); Stack+Pad (P273.9) usa 3γ.2.β (porque sem dimensions literais é o caminho natural).

**Lacuna concreta**: Block sem dimensions literais com gradient `relative=parent`. Comportamento actual: cai no fallback page_bbox (P273.5). Comportamento "vanilla-correcto": bbox real medida do Block.

### Trade-offs conhecidos

| Aspecto | 3γ.2.γ actual (Block sem dims) | 3γ.2.β proposto (P273.15) |
|---|---|---|
| Bbox para gradient relative=parent | page_bbox fallback (identity transform) | bbox real medida do Block |
| Custo perf | zero (sem layout duplo) | layout duplo via `measure_content_constrained` |
| Risco bbox errada | nenhum (não tenta) | dependente de `measure_content_constrained` accuracy |
| Demanda registada | nenhuma | nenhuma |

### Predição factual da Fase A

Trajectória provável da Fase A (auto-avaliação honesta):

- §A.1 inventário demanda empírica → zero casos registados em 8 sub-passos onde 3γ.2.γ produziu output incorrecto observable.
- §A.2 análise custo perf → layout duplo para todos os Blocks sem dimensions tem custo non-trivial, executado **sempre** que se entra num Block dimensions-less (não apenas quando há gradient `relative=parent` interno).
- §A.3 decisão → **NO-GO provável**, com sub-padrão "Scope-out reconfirmado por Fase A" cresce N=1 → N=2 cumulativo.

**Esta predição não é decisão prévia** — a Fase A deve fazer o seu trabalho empírico. Mas a spec reconhece honestamente o que vai descobrir, paralelo a P273.14 §0.

### Comparação com P273.14

| Aspecto | P273.14 (CMYK-ICC) | P273.15 (Bbox pós-layout) |
|---|---|---|
| Tipo de pendência | Refino qualitativo opcional | Refino qualitativo opcional |
| Bloqueador identificado | Profile licensing + crate externa | Custo perf sem demanda |
| Caminhos de viabilidade | 3 (crate / hardcoded / scope-out) | 3 (eager / lazy / scope-out) |
| Outcome esperado Fase A | NO-GO (confirmado) | NO-GO (provável) |

Ambos os passos compartilham estrutura go/no-go. P273.15 reaplica o mecanismo P273.14 — sub-padrão "Scope-out reconfirmado por Fase A" cresce N=1 → N=2 cumulativo se NO-GO confirmado.

---

## §1 — Sub-passo P273.15.A — Fase A diagnóstico (com decisão go/no-go)

**Magnitude**: S documental (~25-35 min).
**Output**: `00_nucleo/diagnosticos/typst-passo-273-15-diagnostico.md`.

### §A.1 — Inventário da demanda empírica

Listar literal:

- Casos registados em P273.6/P273.7/P273.9/P273.10/P273.12/P273.13 onde 3γ.2.γ produziu output incorrecto observable: **zero esperado**. Fase A confirma empíricamente.
- Tests existentes que exercitam gradient `relative=parent` aninhado em Block sem dimensions: confirmar via `grep`.
- Output PDF actual desses tests: page_bbox fallback (identity transform). Comportamento aceito ou registado como issue?

### §A.2 — Inventário dos 3 caminhos

Análogo a P273.14 §A.1:

#### Caminho 1: Eager measurement (todos os Blocks sem dimensions)

- L1 arm `Content::Block` ganha lógica condicional:
  - Se `width.is_some() && height.is_some()`: 3γ.2.γ literal (já existente P273.6).
  - Se um ou ambos `None`: invocar `measure_content_constrained(body, available_w)` → popular `parent_bbox` com `(cursor_x, cursor_y, measured_w, measured_h)`.
- Custo perf: layout duplo **sempre**, mesmo quando não há gradient `relative=parent` interno (Layouter não sabe a priori).
- Magnitude: ~30-50 LOC L1.

#### Caminho 2: Lazy measurement (only when gradient relative=parent interno detectado)

- Pre-walk no body para detectar se há gradient `relative=parent`.
- Se sim: medir e popular bbox.
- Se não: 3γ.2.γ literal (cai no fallback page_bbox).
- Custo perf: walk extra **sempre**, layout duplo só quando necessário.
- Magnitude: ~60-100 LOC L1 (walker novo).

#### Caminho 3: Scope-out preserved

- 3γ.2.γ continua a ser a forma actual para Block sem dimensions literais.
- Decisão P273.6 mantida literal.
- Relatório de trabalho prévio externo: cenários hipotéticos onde 3γ.2.β seria útil + condições para reabrir P273.15 como GO futuro.

### §A.3 — Decisão go/no-go primária

A Fase A toma uma decisão binária:

- **GO**: materializa Caminho 1 ou 2. Magnitude S-M consoante caminho. Fase C executa.
- **NO-GO**: scope-out preserved. Fase B+C reduzidas a documentação:
  - **Anotação cumulativa ADR-0091**: regista que P273.15 foi tentado, viabilidade verificada, scope-out reconfirmado.
  - **Relatório de trabalho prévio externo**: documento descrevendo cenários hipotéticos onde valeria a pena reabrir.

### §A.4 — Critério para GO

A Fase A só decide GO se cumpridas todas as condições:

1. **Demanda empírica concreta identificada** — pelo menos um caso real (test ou documento utilizador) onde 3γ.2.γ produz output observável incorrecto que cause problema concreto.
2. **Caminho escolhido** com magnitude + custo perf aceito.
3. **Tests E2E construídos** que verificam observable diff vs comportamento actual.

Caso 1 não cumprido → **NO-GO** automaticamente. Sem demanda, refino é over-engineering.

### §A.5 — Critério para NO-GO

A Fase A decide NO-GO se:

1. §A.1 confirma zero demanda empírica registada (esperado).
2. Caminho 1 (eager) tem custo perf inaceitável (layout duplo sempre).
3. Caminho 2 (lazy) tem custo de implementação desproporcional ao benefício (walker novo).
4. 3γ.2.γ é aceito por ADR-0054 graded — refino "menor mudança suficiente" preserved.

NO-GO **não é falha do passo** — é cumprimento honesto do critério "verificar empíricamente" registado em todos os relatórios anteriores.

### §A.6 — Análise de risco

| Risco | Fonte | Mitigação |
|---|---|---|
| Refino sem demanda empírica vira over-engineering | Caminho 1 ou 2 sem caso real | §A.4 critério 1 obrigatório |
| Custo perf escondido | Caminho 1 eager mede sempre | §A.5 critério 2 explícito |
| Regressão tests P273.6 Block 3γ.2.γ | Mudança em arm Block | Defaults preservam: `width.is_some() && height.is_some()` continua a path 3γ.2.γ literal |
| Scope-out parece falha | NO-GO confundido com regressão | §A.5 explicita: NO-GO é cumprimento honesto |
| `measure_content_constrained` valores divergentes do layout real | Layout duplo pode produzir bbox diferente do real | Tests E2E (se GO) verificam consistência |

### §A.7 — Decisões a fixar na Fase A

1. **Decisão 1** (caminho): 1 / 2 / 3 consoante §A.1-A.5.
2. **Decisão 2 (apenas se GO)**: detalhes de implementação consoante caminho escolhido.
3. **Decisão 3 (sempre)**: documento de trabalho prévio externo se NO-GO.

### §A.8 — Critério de aceitação Fase A

Independente de go/no-go:

- §A.1 inventário de demanda empírica com factos literais (zero ou >0 casos registados).
- §A.2 inventário dos 3 caminhos com custo perf concreto.
- §A.3 decisão go/no-go fixada com fundamento literal.
- Se NO-GO: documento de trabalho prévio externo produzido.

---

## §2 — Sub-passo P273.15.B — Anotação cumulativa ADR-0091

**Magnitude**: XS documental (independente de go/no-go).

Anotar ADR-0091 — décima quinta anotação consecutiva.

### Template se GO

```
## Anotação cumulativa P273.15 — Bbox medido pós-layout para Block sem dimensions

**Data**: 2026-05-XX.
**Decisão**: GO via caminho [1 eager / 2 lazy].
**Caso empírico**: [descrição literal do caso que justificou GO].
**Custo perf**: [layout duplo sempre / quando gradient relative=parent detectado].
**Sub-padrão "Refino qualitativo opcional materializado"** N=0 → N=1 inaugural.
**Defaults preservam**: Block com `width+height` literais → 3γ.2.γ literal preserved;
Block sem dimensions sem gradient interno → preserved page_bbox fallback ou
medição consoante caminho.
```

### Template se NO-GO

```
## Anotação cumulativa P273.15 — Bbox pós-layout scope-out reconfirmado

**Data**: 2026-05-XX.
**Decisão**: NO-GO via §A.5 critério [literal].
**Razão concreta**: [zero demanda empírica + custo perf inaceitável + 3γ.2.γ
aceito per ADR-0054 graded].
**Trabalho prévio externo identificado**: ver
`00_nucleo/diagnosticos/typst-passo-273-15-trabalho-previo-externo.md`.
**Decisão P273.6 §A.3 (3γ.2.γ) preserved literal** — 8 sub-passos sem
contraproba.
**Sub-padrão "Scope-out reconfirmado por Fase A"** N=1 → **N=2 cumulativo
emergente** (P273.14 inaugural + P273.15 reaplicação). Padrão consolidado
para refinos qualitativos opcionais sem demanda empírica.
**Cluster Gradient avança sem este refino** — 3γ.2.γ preserved como caminho
actual para Block sem dimensions; comportamento page_bbox fallback aceito.
```

---

## §3 — Sub-passo P273.15.C — Materialização (só se GO)

**Magnitude**: S-M consoante caminho escolhido.

Se GO via Caminho 1 (eager):

- L1 arm `Content::Block` ganha lógica condicional.
- Tests E2E confirmam observable diff vs 3γ.2.γ.

Se GO via Caminho 2 (lazy):

- L1 walker novo para detectar `gradient relative=parent` no body.
- L1 arm `Content::Block` ganha pre-walk + medição condicional.
- Tests E2E confirmam observable diff + custo perf within bounds.

### Cap LOC (só se GO)

- **L1 hard cap (Caminho 1)**: ≤ 50 LOC. **Soft cap**: ≤ 35 LOC.
- **L1 hard cap (Caminho 2)**: ≤ 100 LOC. **Soft cap**: ≤ 70 LOC.
- **L3 hard cap**: 0 LOC.
- **Tests hard cap**: ≤ 8. **Soft cap**: ≤ 5.

### Tests propostos (só se GO)

1. `p273_15_block_no_dims_gradient_parent_uses_measured_bbox` — Block sem dimensions + gradient `relative=parent` interno emit usa bbox medida (não page_bbox).
2. `p273_15_block_with_dims_unchanged` — Block com `width+height` literais preserved 3γ.2.γ bit-exact P273.6.
3. `p273_15_block_no_dims_no_gradient_unchanged` (caminho 2) — Block sem dimensions sem gradient interno preserved literal (sem custo measurement).
4. `p273_15_observable_diff` — E2E confirma bytes PDF diferentes vs P273.14.
5. Regressão integrada: 2644 verdes preserved + tests novos.

---

## §4 — Sub-padrões cumulativos pós-P273.15

### Se GO

| Sub-padrão | Pós-P273.14 | Pós-P273.15 (GO) |
|---|---|---|
| Anotação cumulativa em vez de ADR nova | 21 | 22 |
| Cap LOC hard vs soft explícito | 16 | 17 |
| Aplicação meta-ADR (ADR-0094) | 12 | 13 |
| Sub-passos consecutivos do mesmo cluster | N=10 | **N=11 cumulativo emergente** |
| Diagnóstico imutável | 30 | 31 (26º consumo) |
| **Refino qualitativo opcional materializado** | N=0 | **N=1 inaugural** |
| Layout duplo arquitectural aceite | N=1 | **N=2 cumulativo** (P273.9 Stack/Pad + P273.15 Block sem dims) |

### Se NO-GO

| Sub-padrão | Pós-P273.14 | Pós-P273.15 (NO-GO) |
|---|---|---|
| Anotação cumulativa em vez de ADR nova | 21 | 22 |
| Aplicação meta-ADR (ADR-0094) | 12 | 12 (preserved — sem cap aplicado) |
| Sub-passos consecutivos do mesmo cluster | N=10 | **N=11 cumulativo emergente** |
| Diagnóstico imutável | 30 | 31 (26º consumo) |
| **Scope-out reconfirmado por Fase A** | N=1 inaugural | **N=2 cumulativo emergente** |

Sub-padrão "Scope-out reconfirmado por Fase A" cresce N=1 → N=2 se NO-GO confirmado — primeira reaplicação consolida o padrão. Limiar formalização N=3-4 ainda longe.

---

## §5 — Limitações conscientes P273.15

Se GO:
- Custo perf de layout duplo (Caminho 1) ou walker extra (Caminho 2).
- `measure_content_constrained` accuracy depende de paridade com layout real.

Se NO-GO:
- 3γ.2.γ continua a ser caminho para Block sem dimensions — gradient `relative=parent` aninhado em Block sem dims continua a usar page_bbox fallback.
- Refino futuro candidato se demanda empírica aparecer.

---

## §6 — Workflow operacional

1. Utilizador lê esta spec.
2. Utilizador executa Fase A em Claude Code → diagnóstico **com decisão go/no-go**.
3. Utilizador upload do diagnóstico.
4. Claude web valida critério §A.8.
5. **Se GO**: utilizador executa P273.15.B + P273.15.C → relatório com materialização.
6. **Se NO-GO**: utilizador executa P273.15.B + produz `trabalho-previo-externo.md` → relatório com NO-GO documentado.
7. Utilizador upload do relatório.
8. Claude web analisa + propõe **P273.16** (Bbox.y topo-exacto inline; **bloqueado DEBT-56**).

---

## §7 — Pendências preservadas pós-P273.15

Inalteradas vs P273.14:

- **ADR-0055bis variant-aware fonts** (M).
- **P-Footnote-N** (M).
- **DEBT-33 Bézier bbox** (S+M).
- **Stroke\<Length\> / Curve / Polygon** (S+M).
- **Tiling activação** (Paint::Tiling).
- **Outro cluster — saída Visualize/Gradient**.

Sequência para fechar cluster Gradient pós-P273.15:

- ✓ P273.5/6/7/8/9/10/11/12/13/14 (fechados).
- **P273.15** — Bbox medido pós-layout (este passo; GO ou NO-GO).
- **P273.16** — Bbox.y topo-exacto inline (M-L; **BLOQUEADO** por DEBT-56).

Predição: cluster termina em P273.15 (NO-GO esperado) ou em P273.16 (bloqueador externo confirmado).

---

## §8 — Critério de fecho do passo

P273.15 fecha com **IMPLEMENTADO** (GO) ou **SCOPE-OUT-RECONFIRMED** (NO-GO).

### IMPLEMENTADO (GO)

- Fase A produzida + critério §A.8 cumprido + decisão GO.
- ADR-0091 anotada (décima quinta consecutiva — versão GO).
- L1 alterado dentro do cap LOC.
- Tests workspace verdes (cap respeitado).
- Lint zero.
- Tests P262-P273.14 inalterados bit-exact (defaults preserved).
- DEBT saldo 10 preserved.
- Test E2E confirma observable diff.

### SCOPE-OUT-RECONFIRMED (NO-GO)

- Fase A produzida + critério §A.8 cumprido + decisão NO-GO.
- ADR-0091 anotada (décima quinta consecutiva — versão NO-GO).
- Documento `trabalho-previo-externo.md` produzido.
- Zero alterações ao código L1/L3.
- Tests workspace 2644 preserved.
- Sub-padrão "Scope-out reconfirmado por Fase A" N=2 cumulativo emergente.

---

## §9 — Numeração

Spec usa **P273.15** continuando a sequência decimal. Sexto sub-passo materializado da sub-sequência "terminar cluster Gradient" (escopo máximo).

Sequência prevista:

- ✓ P273.5-P273.14 (fechados, com P273.14 = SCOPE-OUT-RECONFIRMED).
- **P273.15** — Bbox medido pós-layout (este passo; GO ou NO-GO).
- P273.16 — Bbox.y topo-exacto inline (M-L; **bloqueado DEBT-56**; predição NO-GO obrigatório).

Predição: cluster Gradient termina entre P273.15 e P273.16 — P273.15 pode ser último materializado se NO-GO; P273.16 será NO-GO bloqueado por trabalho externo (DEBT-56) por construção.
