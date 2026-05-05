# Passo 191A — Instrução Claude Code

## Contexto mínimo

Typst Cristalino é re-implementação atómica do projecto
`typst/typst` em Rust com arquitectura camadas L0–L4.
Vanilla original está em quarentena em `lab/typst-original/`.
Cristalino vive em `01_core/`, `02_shell/`, `03_infra/`,
`04_wiring/`. ADRs em `00_nucleo/adr/`.

**Snapshot de partida** (a confirmar empiricamente em
sub-passo .A):

- Tests workspace 1.855 verdes; zero violations.
- M9 ✅ 11/11.
- M5 universal completo (P200B).
- M6 série P190 **em pausa** após P190F:
  - P190A diagnóstico ✅.
  - P190B Bibliography ✅ (2 fields eliminados).
  - P190C Page tracking ✅ (LayouterRuntimeState
    criada; 2 fields movidos).
  - P190D Document metadata ✅ (parcial; `lang`
    deferido).
  - P190E Numbering active ✅ (parcial; field
    deferido — Caso 1).
  - P190F Counters core ⚠️ **escopo reduzido** —
    barreira arquitectural identificada.
- `CounterStateLegacy`: 10 fields.
- `LayouterRuntimeState`: 3 fields.
- Defers acumulados: `lang`, `numbering_active`,
  `flat`, `hierarchical`.

P191 abre **ramo paralelo** ao trabalho M6 série
P190. Diagnóstico do **redesign do pipeline walk**
para resolver barreira arquitectural identificada
em P190F §3.

**Barreira identificada em P190F**:
> Walk fn não tem acesso a `Introspector` —
> Introspector é construído POST-walk via
> `from_tags::from_tags(&tags)`. Helpers chamados
> durante walk não podem queryar Introspector.

**5 fields têm walk readers** (per P190F §3):
- `state.flat` — lido por `compute_labelled`.
- `state.hierarchical` — lido por
  `compute_heading_auto_toc`.
- `state.figure_numbers` — lido por
  `compute_labelled`.
- `state.lang` — lido por `compute_labelled`.
- `state.numbering_active` — lido por
  `compute_heading_auto_toc` + walk arm Equation
  gate.

**P191A é diagnóstico**. Magnitude esperada **S-M**
(análoga a P190A). Implementação P191B+ depende de
cláusula 1 (escolha de mecanismo).

**Lembrete crítico — incluir em todos os relatórios
P191**: P190 série está **em pausa**. Após P191
fechar, retomar P190G — categorias 6, 7, P190I + 4
defers acumulados.

**Material de partida** verificado:

- `00_nucleo/materialization/typst-passo-190f-relatorio.md`
  §3 — barreira arquitectural documentada.
- `00_nucleo/materialization/typst-passo-190f-relatorio.md`
  §10 plano de redesign — 3 opções (A/B/C).
- `00_nucleo/auditoria-fresh-projecto.md` F1.
- `00_nucleo/m1-lacunas-captura.md` — lacunas
  ortogonais.
- `00_nucleo/adr/typst-adr-0070-eliminacao-counter-state-legacy.md`
  PROPOSTO (P190A.N).

P191A é pesquisa arquitectural. P191B+ implementa
mecanismo escolhido. Após P191 série fechar:
- Helpers chamados durante walk podem queryar
  Introspector (ou equivalente).
- Walk arm gates podem usar Introspector
  location-aware durante walk.
- Pré-condição arquitectural para fechamento
  efectivo de P190G/H/I + cleanup defers.

---

## Postura do auditor / executor

P191A é passo **L0-puro / diagnóstico-primeiro**.

- **Zero código tocado** em camadas cristalinas.
- **Zero testes** modificados.
- **Pode criar** ADR `PROPOSTO` — **provável**
  ADR-0071 sobre mecanismo de walk pipeline
  redesign.
- **Pode abrir DEBT** se trabalho identificado for
  adiado.
- **Não modifica** walk fn signature, helpers,
  trait — P191B+.

**Magnitude diagnóstico**: S-M. Decisões expandidas
porque:
- 4 opções arquiteturais (A/B/C/D).
- Cada opção tem trade-offs de magnitude diferente.
- Decisão afecta pipeline walk inteiro.
- Padrão sem precedente directo no projecto.

**Particularidade de P191**: trabalho é **redesign
arquitectural**, não cleanup incremental. Análoga a
ADR-0068 P185A (mecanismo location-aware Layouter
— também redesign).

---

## Escopo

**Primário**: planear mecanismo de walk pipeline
redesign que permita helpers chamados durante walk
queryar Introspector (ou equivalente).

**Confirmação**: validar inventário factual:
- Walk fn signature actual.
- 4 helpers `compute_*` — quais leem state legacy
  durante walk; quais não.
- Walk arm gates — quais leem state legacy.
- Walk pipeline de execução — qual ordem; onde
  exactamente Introspector seria usado.
- Pre-condições para cada opção (A/B/C/D).

**Decisões a tomar** — 9 cláusulas:

1. **Mecanismo de redesign**:
   - **Opção A** (walk recebe `&mut TagIntrospector`):
     replicar `from_tags` logic durante walk;
     Introspector populated incrementalmente.
     Magnitude implementação: M+.
   - **Opção B** (two-pass walk):
     1ª pass emit Tags com payloads parciais; build
     Introspector; 2ª pass fill-in payloads
     definitivos.
     Magnitude implementação: L (2 passes).
   - **Opção C** (eliminate helpers; embed inline):
     `compute_*` helpers eliminados; lógica
     embedded directamente em walk arms ou movida
     para `from_tags`.
     Magnitude implementação: M (eliminação) mas
     pode requerer refactor cross-cutting.
   - **Opção D** (deferred resolution):
     Tags emitidas com payload parcial (sem
     `formatted_counter` etc.); resolution ocorre
     na fase de Layouter via Introspector queries.
     Magnitude implementação: M+ (mudança
     semântica).

2. **Helpers a manter vs eliminar**:
   - `compute_labelled` — manter, eliminar, ou
     migrar?
   - `compute_heading_auto_toc` — idem.
   - `compute_figure` — walk-internal, possivelmente
     já elimináveis em P190H.
   - `compute_heading_for_toc` — walk-internal,
     possivelmente já elimináveis em P190H.

3. **Walk arm Equation gate**:
   - Mantém-se com `state.is_numbering_active`
     (per opção escolhida).
   - Migra para `intr.is_numbering_active_at`
     (Opção A).
   - Migra para resolution Layouter-side (Opção
     D).

4. **Compatibilidade com pattern ADR-0069**:
   - 5 variantes operacionais ADR-0069 mantêm-se
     funcionais?
   - Padrão "post-recursion-tag-emission" altera-se?

5. **Compatibilidade com `from_tags`**:
   - Mecanismo escolhido afecta `from_tags`?
   - `from_tags` continua único construtor de
     Introspector ou partilha responsabilidade?

6. **Pre-condições para implementação**:
   - Quais sub-stores Introspector já estão
     populated no momento certo do walk?
   - Quais precisam de redesign de population
     timing?

7. **Estratégia de migração incremental**:
   - Implementar mecanismo + migrar 1 helper como
     prova de conceito (P191B)?
   - Implementar mecanismo + migrar todos
     simultaneamente (P191B único)?

8. **Tests**:
   - Tests existentes preservam-se?
   - Tests novos necessários para mecanismo?

9. **Critério de fecho de P191 série**:
   - Mecanismo implementado e testado.
   - Pelo menos 1 helper migrado como validação.
   - Walk arm Equation gate migrado.
   - Pré-condição arquitectural para retomar
     P190G/H/I cumprida.
   - ADR-0071 ACEITE.

**Fora de escopo**:

- Eliminação de fields `CounterStateLegacy`
  remanescentes — P190G/H/I.
- Eliminação de struct `CounterStateLegacy` —
  P190I.
- 4 helpers `compute_*` eliminação completa —
  P190G/H.
- Cleanup `lang`, `numbering_active`, `flat`,
  `hierarchical` — P190G/H/I após P191 fechar.
- Lacunas residuais (#1, #1b, #2).
- M7 (loop fixpoint), M8 (memoização comemo).

---

## Critérios objectivos

### O1 — Inputs verificáveis

- `grep -n "fn walk" 01_core/src/rules/introspect.rs`
  para signature actual.
- `grep -rn "compute_labelled\|compute_heading_auto_toc"
  01_core/src/`.
- `grep -n "from_tags::from_tags" 01_core/src/`
  para construtor Introspector.
- Inspeccionar `from_tags.rs` para inventário de
  populations.

### O2 — Alternativas

Cláusula 1 tem 4 opções (A/B/C/D). Demais cláusulas
têm caminho preferido decidível após cláusula 1.

### O3 — Critério de escolha

Trade-off entre magnitude implementação vs ganho
arquitectural:
- Opção A: signature change único; mais simples;
  pode duplicar parte de from_tags logic.
- Opção B: 2 passes; conceptual mas complexo.
- Opção C: eliminação mas pode requerer refactor
  cross-cutting.
- Opção D: mudança semântica; resolution Layouter.

Sugestão preliminar: **Opção A** ou **Opção D** —
A é mais directa; D é mais alinhada com vanilla
typst (que faz resolution lazy).

### O4 — Magnitude

P191 implementação:
- **Opção A**: M+ (signature change + populate
  logic incremental).
- **Opção B**: L (2 passes).
- **Opção C**: M (eliminação) + L (refactor).
- **Opção D**: M+ (mudança semântica).

P191A diagnóstico: S-M. P191B+ implementação total:
M+ a L dependendo da opção escolhida.

### O5 — Reversibilidade

Reversível mas custoso. Reverter exige restaurar
walk pipeline anterior + helpers walk-readers
intactos.

---

## Critérios qualitativos

### Q1 — Consistência com padrão estabelecido

P191 introduz **mecanismo arquitectural novo** sem
precedente directo no projecto. Análoga a ADR-0068
P185A (location-aware) — também mecanismo novo.

ADR-0071 PROPOSTO recomendada.

### Q2 — Honestidade de magnitude

P191A diagnóstico é S-M. P191B+ implementação é
M+ a L. **Não disfarçar** — redesign do pipeline
walk é trabalho substancial.

### Q3 — Cobertura sem regressão

Output observable preservado por construção:
- Tags emitidas mantêm payloads válidos.
- Introspector queries retornam mesmos resultados.
- Tests existentes adaptados conforme necessário.

### Q4 — Compatibilidade com M6

Após P191 fechar:
- P190G/H podem retomar com fields walk-readable
  resolúveis.
- P190I final pode eliminar struct
  `CounterStateLegacy`.
- F1 fecha (após P190I).

### Q5 — Granularidade

P191 série tem **2-3 sub-passos esperados**:
- P191A diagnóstico.
- P191B implementação mecanismo + 1 helper migrado
  como validação.
- P191C cleanup + relatório consolidado + ADR-0071
  ACEITE.

Total agregado: M+ a L cross-modular.

---

## Sub-passos de P191A

### Sub-passo 191A.A — Validação do estado actual + inventário

Auditor confirma empiricamente:

#### Estado consolidado pós-P190F

1. Tests workspace 1.855 verdes.
2. Linter zero violations.
3. P190 série em pausa (A-F executados; G-I
   pendentes).

#### Inventário walk pipeline actual

4. Localizar `fn walk` em
   `01_core/src/rules/introspect.rs`:
   - Signature exacta.
   - Parameters: `state`, `locator`, `tags`,
     `label_from_parent`.

5. Localizar `introspect_with_introspector` ou
   função análoga:
   - Confirmar ordem: walk → from_tags → return.
   - Identificar onde Introspector seria
     introduzido em cada opção (A/B/C/D).

#### Inventário 4 helpers

6. Para cada helper, identificar:
   - Reads de `state` durante walk.
   - Caller context (acesso disponível ao
     Introspector se signature mudasse).

#### Inventário walk arm gates

7. Walk arm Equation gate (per P190F §3):
   `state.is_numbering_active("equation")`.
8. Outros walk arm gates similares.

#### Inventário from_tags

9. `from_tags::from_tags` populates:
   - Quais sub-stores TagIntrospector.
   - Em que ordem.
   - Qual a logic dependency entre populations.

#### L0 alvos

10. Identificar L0s a tocar em P191B:
    - `rules/introspect.md` (signature change ou
      mecanismo novo).
    - Possivelmente novo L0 dedicado ao mecanismo.

Output: tabela com item + estado verificado.

**Critério de saída**:
- Walk pipeline mapeado.
- 4 helpers categorizados (walk-readers vs
  walk-internal).
- Walk arm gates listados.
- from_tags inventário completo.

### Sub-passo 191A.B — Decisão cláusula 1 (mecanismo)

Conforme `.A`:

4 opções:
- A (walk recebe `&mut TagIntrospector`).
- B (two-pass walk).
- C (eliminate helpers; embed inline).
- D (deferred resolution Layouter-side).

**Sugestão preliminar**: depende de empírico.
Análise de trade-offs com 5 dimensões:
- Magnitude implementação.
- Compatibilidade ADR-0069 stylesheet.
- Compatibilidade `from_tags` actual.
- Alinhamento com vanilla typst.
- Reversibilidade.

Output: opção fixada com justificação literal.

### Sub-passo 191A.C — Decisão cláusula 2 (helpers)

Conforme `.A.6`:

Decisão por helper:
- `compute_labelled` — migrar / manter / eliminar.
- `compute_heading_auto_toc` — idem.
- `compute_figure` — confirmar walk-internal.
- `compute_heading_for_toc` — idem.

Output: decisão por helper.

### Sub-passo 191A.D — Decisão cláusula 3 (walk arm gates)

Conforme `.A.7` + `.A.8`:

Estratégia para walk arm Equation gate (e outros)
per opção 1.

Output: estratégia fixada.

### Sub-passo 191A.E — Decisão cláusula 4 (compatibilidade ADR-0069)

5 variantes operacionais ADR-0069 ainda funcionam?
Sub-stores Introspector populated no momento certo?

Output: análise empírica.

### Sub-passo 191A.F — Decisão cláusula 5 (`from_tags`)

`from_tags` continua único construtor ou partilha
responsabilidade?

Output: estratégia fixada.

### Sub-passo 191A.G — Decisão cláusula 6 (pre-condições)

Quais sub-stores precisam de redesign de population
timing?

Output: lista empírica.

### Sub-passo 191A.H — Decisão cláusula 7 (estratégia migração)

Sugestão preliminar: implementar mecanismo + migrar
1 helper como prova de conceito (P191B); migrar
todos simultaneamente após validação (em P190G/H
após retomar série).

Output: estratégia fixada.

### Sub-passo 191A.I — Decisão cláusula 8 (tests)

Tests novos necessários para mecanismo +
preservação de tests existentes via padrão pragmático
auditor #1.

Output: estratégia fixada.

### Sub-passo 191A.J — Decisão cláusula 9 (critério fecho)

P191 série fecha quando:
- Mecanismo implementado e testado.
- 1 helper migrado como validação.
- Walk arm Equation gate migrado.
- Tests workspace verdes.
- Pré-condição arquitectural para retomar
  P190G/H/I cumprida.
- ADR-0071 ACEITE.

Output: critério literal verificável.

### Sub-passo 191A.K — Validação do plano de sub-passos

Sub-passos esperados:

| Sub-passo | Escopo | Magnitude |
|---|---|---|
| `.B` | Implementar mecanismo (per opção 1) + migrar 1 helper como validação + walk arm Equation gate migrado + tests novos | M+ |
| `.C` | Relatório consolidado P191 + ADR-0071 ACEITE + lembrete formal de retomar P190G/H/I | S |

Total agregado: M+ (validação) + S (encerramento) =
**M+** para P191 série.

Output: plano detalhado fixado per `.A`.

### Sub-passo 191A.L — ADR-0071 PROPOSTO

Avaliar:

- Decisão arquitectural substancial: redesign do
  walk pipeline.
- Análoga a ADR-0068 (location-aware Layouter).
- ADR-0071 PROPOSTO recomendada.

**Título proposto**: "Walk pipeline com Introspector
acessível durante execução" (ou similar per opção
escolhida).

**Estado**: PROPOSTO em P191A.N. ACEITE em P191C
após validação empírica.

Output: ADR-0071 esboço.

### Sub-passo 191A.M — DEBT + lembrete formal

P191 fecha **barreira arquitectural P190F §3**.
Após P191:
- Pre-condição cumprida para retomar P190G/H/I.
- 4 defers acumulados resolvíveis (`lang`,
  `numbering_active`, `flat`, `hierarchical`).
- F1 fecha após P190I (não em P191).

**Lembrete formal CRÍTICO**: P190 série tem 3
sub-passos restantes (G, H, I) + cleanup 4 defers
acumulados. Após P191 fechar, retomar P190G.

Output: lembrete + estado actualizado.

### Sub-passo 191A.N — Outputs

Produzir 4 ficheiros:

1. **`00_nucleo/diagnosticos/diagnostico-walk-pipeline-redesign-passo-191a.md`**
   — diagnóstico com 9 secções.

2. **`00_nucleo/materialization/typst-passo-191a-relatorio.md`**
   — relatório com 14 secções (padrão P181A/etc.).

3. **`00_nucleo/adr/typst-adr-0071-walk-pipeline-redesign.md`**
   PROPOSTO — esboço.

4. **Lembrete formal** em
   `00_nucleo/m1-lacunas-captura.md` ou ficheiro
   dedicado: "P190 série em pausa após P190F.
   Retomar P190G após P191 fechar. 3 sub-passos
   pendentes (G, H, I) + cleanup 4 defers acumulados
   (`lang`, `numbering_active`, `flat`,
   `hierarchical`)".

---

## Restrições

- **Zero código tocado** em qualquer ficheiro fora
  de `00_nucleo/`.
- **Zero testes** modificados.
- **Não modificar walk fn** — P191B+.
- **Não tocar helpers `compute_*`** — P191B+.
- **Não modificar trait `Introspector`** — P191B+
  (talvez não seja necessário modificar nunca).
- **Não modificar `TagIntrospector`** — P191B+.
- **Não modificar `from_tags`** — P191B+.
- **Não modificar Layouter** — P191B+.
- **Não eliminar `CounterStateLegacy`** — P190I
  após P191 fechar.
- **Não eliminar campos** — P190G/H/I após P191
  fechar.
- **Não materializar lacunas residuais**.
- **Não inflar linguagem**.
- **Aplicar regra dos 2 eixos**.
- **Reaproveitar pattern ADR-0069 + 5 variantes
  operacionais** se aplicável.
- **Sem cláusulas condicionais nos sub-passos `.B`+
  do plano** (mas plano pode ter ramificação por
  opção escolhida).

---

## Critério de conclusão

- Diagnóstico em
  `00_nucleo/diagnosticos/diagnostico-walk-pipeline-redesign-passo-191a.md`
  com 9 secções produzido.
- Relatório em
  `00_nucleo/materialization/typst-passo-191a-relatorio.md`
  com 14 secções produzido.
- ADR-0071 PROPOSTO criada.
- Lembrete formal P190 série retomar (em ficheiro
  dedicado).
- 9 cláusulas fechadas com decisão literal.
- Plano de sub-passos sem condicionais (esperado:
  2 sub-passos B-C).
- Magnitude consolidada confirmada (M+ agregado).
- Critério de fecho P191 fixado.
- Regra dos 2 eixos aplicada.
- Pattern ADR-0069 analisado para compatibilidade.
- Nenhum ficheiro de cristalino tocado.
- Tests workspace 1.855 inalterados.
- `crystalline-lint .` zero violations.

P191A é instrumento. Implementação concreta de
P191B é mecanismo arquitectural novo + 1 helper
migrado como validação.

**Após P191 série fechar**: pré-condição
arquitectural cumprida. **Retomar P190 série** —
3 sub-passos restantes (G, H, I) + cleanup 4
defers acumulados (`lang`, `numbering_active`,
`flat`, `hierarchical`).

**Risco arquitectural moderado**: redesign do walk
pipeline é trabalho sem precedente directo no
projecto. Cláusulas gate substanciais prováveis em
P191B. Auditoria diagnóstico-primeiro em P191A
reduz incerteza mas não a elimina.

**LEMBRETE CRÍTICO**: NÃO esquecer que P190 série
está em pausa. Após P191 fechar, retomar P190G.
