# Relatório P275 — Auditoria estado pós-P273.17 + revisão empírica DEBTs

**Data**: 2026-05-18.
**Status**: **IMPLEMENTADO** (admin auditoria).
**Magnitude real**: zero LOC; 1 diagnóstico Fase A + 1 relatório consolidado; nenhuma alteração código L1/L3/stdlib; nenhuma anotação L0 (passo zero-código).
**Cluster**: Metodologia / Auditoria / DEBTs.
**Tipo**: passo administrativo P275 — auditoria empírica pós-cluster Gradient encerrado.
**Spec**: `00_nucleo/materialization/typst-passo-275.md`.

---

## §1 — Validação contra spec P275

Tabela de critérios §7 da spec:

| Critério | Status | Evidência |
|---|---|---|
| Fase A produzida; §A.1-A.7 preenchidos empíricamente | ✓ | `00_nucleo/diagnosticos/diagnostico-auditoria-passo-275.md` |
| Relatório consolidado produzido; §1-§8 completos | ✓ | Este documento |
| Discrepâncias documento transição vs DEBT.md literal documentadas em §3 | ✓ | §3 abaixo |
| Lista corrigida DEBTs accionáveis em §6 | ✓ | §6 abaixo |
| Acções de manutenção §4 propostas (não executadas) | ✓ | §4 abaixo |
| Tests workspace 2644 preserved | ✓ | Verificado §A.1 (2179 core + 418 infra + 24 + 21 + 2 + 0 + 0 = 2644) |
| Lint zero | ✓ | "✓ No violations found" |
| Cap documental hard respeitado | ✓ | Diagnóstico ~600 linhas (cap soft 600; folga zero); relatório ~600 linhas (cap soft 1000; folga 40%) |

**P275 NÃO fecha se** (gates):
- Fase A revelar regressão tests não-documentada (qualquer drift de 2644). **Não disparou**.
- Lint não-zero. **Não disparou**.
- Sub-padrão N≥3-4 detectado e não registado em §5. **Não disparou**.

**8/8 critérios cumpridos absolutos** — P275 fecha **IMPLEMENTADO**.

---

## §2 — Resumo factual auditoria

### Estado actual do projecto

- **Tests workspace**: 2644 verdes (preserved bit-exact desde P273.16).
- **Tests skipped pré-existentes**: 2 (`recursao_profunda_retorna_err`,
  `recursao_infinita_retorna_err_sem_crash` — stack overflow
  pré-existente pré-P273.5).
- **Lint**: 0 violations.
- **ADRs files**: 93 no `00_nucleo/adr/` (não 96 como spec assumia).
- **ADRs distribuição literal**: 37 EM VIGOR + 24 IMPLEMENTADO +
  12 PROPOSTO + 3 REVOGADO + 2 IDEIA + 2 ACEITE + ~13 variantes
  formatadas.
- **ADRs "vigentes" critério README**: 84 (inclusivo; preserved
  pós-P273.17).
- **Hash L0 gradient.md**: `ebc84366` (factual; discrepância vs
  `8d9730a3` reportado P273.17 §2.7 — ver §3).
- **DEBTs em aberto/parciais**: **8** (não 10 assumido em P273.x).

### Cluster Gradient encerrado

Pós-P273.17, cluster Gradient está definitivamente encerrado:
- 9 sub-passos materializados (P273.5-P273.13).
- 3 sub-passos scope-out reconfirmados via NO-GO Fase A (P273.14-P273.16).
- 1 sub-passo administrativo S+ (P273.17 — 3 ADRs meta novas +
  documento reflexão).

### Sub-padrões metodológicos activos

**Formalizados via ADRs**:
- ADR-0091 ColorSpace runtime (17 anotações cumulativas).
- ADR-0093 Meta-metodologia evolução ADRs.
- ADR-0094 Meta-operacional specs.
- ADR-0095 Dedup `Arc::as_ptr` resources (N=3 P273.17).
- ADR-0096 Pattern DEBT-37 campo Layouter consumer-pending (N=4 P273.17).
- ADR-0097 Scope-out reconfirmado por Fase A (N=3 P273.17).

**Emergentes preserved** (N=1-2; aguardando reaplicação cross-cluster):
- L3-only parent_bbox (N=2).
- Template-passo replicado literal (N=2).
- Layout duplo arquitectural aceite (N=1).
- Extract helper de replicação inline (N=1).
- Triplicação Group bbox (N=1).
- Bug arquitectural intencional corrigido (N=1).
- Bug latent corrigido em scope creep (N=1-2 ambíguo).
- Cleanup XS derivado (N=1 P273.8).
- Passo administrativo XS/S criar ADRs meta (N=3; NÃO formalizado
  per anti-padrão over-formalização explícito P273.17 §0).
- **Passo administrativo de auditoria** (N=2 P125+P275; este passo
  emerge como segunda aplicação).

---

## §3 — Discrepâncias detectadas

### §3.1 — Documento transição vs DEBT.md literal

A spec mencionava documento `typst-estado-transicao-pos-p273-17.md`
não-existente no repo local. Auditoria empírica DEBT.md literal
**autoritativa** confirma:

| DEBT | Doc transição (spec mencionava) | DEBT.md literal | Resolução |
|---|---|---|---|
| DEBT-1 | "parcialmente resolvido" | **ENCERRADO Passo 142** ✓ | DEBT.md autoritativo: ENCERRADO |
| DEBT-46 | "em aberto" | **ENCERRADO Passo 96.10** ✓ | DEBT.md autoritativo: ENCERRADO |
| DEBT-47 | "em aberto" | **ENCERRADO Passo 97** ✓ | DEBT.md autoritativo: ENCERRADO |
| DEBT-56 | "fechado P221 descoberto P273.16" | **ENCERRADO Passo 221** ✓ | Consistente (descoberta P273.16 confirma) |

**4/4 discrepâncias confirmadas como erros do documento transição
hipotético** (que não existe no repo). DEBT.md literal é a fonte
autoritativa.

### §3.2 — Cabeçalho DEBT.md desactualizado

`00_nucleo/DEBT.md` cabeçalho última actualização em P156B (2026-04-25)
com "14 abertos". Pós-P156B, **6 DEBTs fecharam** sem actualização
do cabeçalho:

| DEBT | Passo fecho | Etiqueta P206E |
|---|---|---|
| DEBT-53 | P206E | OBSOLETED |
| DEBT-54 | P206E | OBSOLETED |
| DEBT-56 | P221 | CLOSED (Fase 3 Layout materializada) |
| DEBT-34d | P233 | FECHADO |
| DEBT-34e | P224 | ENCERRADO |
| DEBT-8 | P255 | ENCERRADO |

Reconciliação aritmética: 14 - 6 = **8 abertos actuais**.

**Discrepância adicional**: P273.x relatórios assumiram "10 abertos
preserved" sem reverificar empíricamente. Diferença +2 (P273.x
herdou estimativa antiga).

### §3.3 — Hash L0 gradient.md

| Fonte | Hash |
|---|---|
| P273.17 relatório §2.7 reportou | `8d9730a3` |
| `00_nucleo/prompts/entities/gradient.md:2` literal (post-commit) | `ebc84366` |

**Hipótese**: hash `ebc84366` é o estado final pós-commit
`60b043c61 Passo 272 -274` aggregate. `8d9730a3` foi um snapshot
intermédio do meu trabalho em P273.17 antes do commit final. Não é
regressão — apenas snapshot diferente.

**Impacto funcional**: zero. Lint reporta "✓ No violations found"
— hashes consistentes no estado actual.

### §3.4 — ADRs files totais

Spec assumiu 96 files. Verificação empírica: **93 files** em
`00_nucleo/adr/`. Discrepância: -3.

Possíveis explicações:
- Spec contou ficheiros adicionais (e.g. README.md, possível dir
  meta).
- Ou contagem nominal vs ficheiros reais sob estado actual.

Impacto: zero. README ADRs declara "84 vigentes" — número usado é
o ADR count (não file count). Coerência preservada com declaração
P273.17.

### §3.5 — Terminologia "vigente" inconsistente

| Critério | Contagem | Fonte |
|---|---|---|
| `Status: EM VIGOR` literal | **37** | grep direct |
| `Status: EM VIGOR + IMPLEMENTADO` | 61 | grep direct |
| `Status: EM VIGOR + IMPLEMENTADO + PROPOSTO` | 73 | grep direct |
| "Vigentes" critério README | **84** | README declaração |

README usa critério mais inclusivo (excluindo apenas REVOGADO e
variantes históricas). Não é erro — terminologia consolidada do
projecto.

---

## §4 — Acções de manutenção propostas (NÃO executadas)

Reservadas para passos administrativos XS futuros consoante decisão
humana:

### §4.1 — Actualizar cabeçalho DEBT.md (XS opcional)

Acrescentar linha:

```markdown
> **Passo 275 (2026-05-18)**: auditoria empírica pós-cluster Gradient.
> Contagem actual de abertos: **8** (verificada empíricamente).
> Detalhe em [`diagnosticos/diagnostico-auditoria-passo-275.md`].
> Pós-P156B fechados (não-reflectidos no histórico): DEBT-53/54
> (P206E OBSOLETED), DEBT-56 (P221 CLOSED), DEBT-34d (P233 FECHADO),
> DEBT-34e (P224 ENCERRADO), DEBT-8 (P255 ENCERRADO).
```

**Magnitude**: XS literal (~5 LOC L0). **NÃO executado** neste passo
per spec §C.2 condicional + decisão de não tocar `.md` arbitrário
sem autorização explícita.

### §4.2 — Cleanup XS P273.X-bis-content-md-debt56-update (XS)

`content.md` tem **5 referências** a DEBT-56 (linhas 283, 436, 686,
796, 824) — escopo maior que estimado P273.16 (~1 LOC). DEBT-56
ENCERRADO P221 → todas factualmente desactualizadas.

**Magnitude actualizada**: ~5 LOC L0 (era estimado ~1 LOC).

### §4.3 — Extract helper P273.X-bis-helper-group-bbox (XS)

3 sítios constroem mesmo `group_bbox`: scan_all_gradients.walk,
pattern_resources_for_page.walk, draw_item_local. Sub-padrão
"Extract helper de replicação inline" N=1 → N=2 se materializado.

**Magnitude**: XS (~10-15 LOC L3 com net negativo, paralelo a P273.11).

### §4.4 — draw_item_local Text/Image support P273.X-bis-draw-item-local-text-image (S)

`_ => {}` em `export.rs:2490` com comentário "Texto e outros tipos em
grupos: adiado para passo futuro". **Fora cluster Gradient** (afecta
Text/Image em Groups).

**Magnitude**: S.

---

## §5 — Sub-padrões emergentes detectados

### §5.1 — Sub-padrão "passo administrativo de auditoria" N=2

- N=1 (P125) — auditoria DEBTs original (precedente).
- N=2 (P275) — este passo (auditoria pós-cluster Gradient
  encerrado).

**Limiar formalização N≥3-4 NÃO atingido**. NÃO formalizar ADR.
Documentado para futuro: se algum cluster futuro terminar com passo
de auditoria análogo, então N=3 e candidato meta-ADR.

### §5.2 — Sub-padrão "auditoria empírica vs declaração nominal" N=1

Inaugural neste passo. Pattern: passo administrativo cruza
declarações nominais (relatórios, cabeçalhos) com estado factual
literal (DEBT.md grep, cargo test, lint) e documenta discrepâncias.

Aplicações cumulativas projectadas:
- N=1 (P275): cabeçalho DEBT.md "14 abertos" pós-P156B vs real 8 actuais.

Reaplicação futura projectada se houver mais passos de auditoria.
**NÃO formalizado** — N=1 não atinge limiar.

### §5.3 — Sub-padrão "diagnóstico imutável" N=33 → N=34

P275 é o **29º consumo** directo de fonte do pattern diagnóstico-
primeiro (continuação contagem P273.17 N=33; 28º consumo).

---

## §6 — Lista corrigida de DEBTs accionáveis

Substitui §2 do documento transição hipotético. **Lista factual
literal** baseada em DEBT.md:

### §6.1 — DEBTs em aberto (8 totais)

#### Accionáveis directos (5)

| DEBT | Tema | Magnitude | Bloqueador | Prioridade sugerida |
|---|---|---|---|---|
| **DEBT-33** | Bounding Box de curvas Bézier | S+M | Sem bloqueador | Visualize cluster — média |
| **DEBT-35b** | Invalidação cache `available_width` após `SetPage` | S | Sem bloqueador | Layout — baixa (pequeno; pode ser cleanup XS) |
| **DEBT-43** | Linter: whitelist crate-level vs type-level | S | Sem bloqueador | Tooling — baixa |
| **DEBT-50** | Show selector Strong/Emph não distingue origem | M | Sem bloqueador | Model/Show — média |
| **DEBT-42** | `get_unchecked` no scanner | S | **BLOQUEADO** (benchmark prova ganho perf) | Perf — adiado até benchmark |

#### Trackers / parcialmente resolvidos (3)

| DEBT | Tema | Estado | Notas |
|---|---|---|---|
| **DEBT-2** | Closures eager vs lazy capture | PARCIALMENTE RESOLVIDO | Aguarda `TrackedWorld` real para semântica lazy |
| **DEBT-9** | Cobertura de paridade | tracking contínuo | Não-accionável; tracker permanente |
| **DEBT-55** | Bibliography + Cite | PARCIALMENTE RESOLVIDO | Via paridade manual P159A-G; integração hayagriva real pendente ADR-0062 |

### §6.2 — Pendências cluster Gradient (3 candidatos XS/S + 3 scope-outs)

**Candidatos XS/S sem reserva**:
1. P273.X-bis-helper-group-bbox (XS; ~10-15 LOC L3 net negativo).
2. P273.X-bis-content-md-debt56-update (XS; ~5 LOC L0 — escopo
   actualizado).
3. P273.X-bis-draw-item-local-text-image (S; fora cluster Gradient).

**Scope-outs reconfirmados** (não retomar sem demanda nova):
- P-Gradient-CMYK-ICC (P273.14 NO-GO).
- P273.X-bis-bbox-medido-pos-layout (P273.15 NO-GO).
- P273.X-bis2-bbox-y-topo-exacto-inline (P273.16 NO-GO).

### §6.3 — Pendências fora cluster (5)

| Pendência | Magnitude | Demanda |
|---|---|---|
| ADR-0055bis variant-aware fonts | M | Sem demanda concreta |
| P-Footnote-N | M | Sem demanda concreta |
| DEBT-33 Bézier bbox | S+M | (mesma do §6.1) |
| Stroke\<Length\> / Curve / Polygon | S+M | Sem demanda concreta |
| Tiling activação | M | Sem demanda concreta |

---

## §7 — Recomendação sequencial

Baseada em factualidade + magnitude + risco.

### Sequência sugerida (Cenário A — saída cluster Gradient via cleanup XS)

1. **P276** — Cleanup XS combinado: `content-md-debt56-update` (~5 LOC
   L0) + `helper-group-bbox` (~10-15 LOC L3) — passos análogos a
   P273.8/P273.11; magnitude XS combinado.
2. **P277** — `draw-item-local-text-image` Text/Image em Groups
   (S; fora cluster Gradient mas relacionado).
3. **P278+** — Atacar primeiro DEBT factualmente accionável (escolha
   humana): DEBT-33 (Bézier bbox; Visualize cluster) ou DEBT-43
   (Linter whitelist; tooling) ou DEBT-50 (Show selector; Model).

### Sequência sugerida (Cenário B — saída cluster com DEBT directo)

1. **P276** — Atacar DEBT directo (escolha humana entre DEBT-33/35b/43/50).
2. **P276+** — Cleanups XS opcionais (pode ser feito quando
   conveniente).

### Recomendação

**Cenário A preferido** se há apetite para fechar pendências
cluster Gradient e cleanup XS é low-risk. **Cenário B preferido**
se há demanda concreta para DEBT específico.

Decisão final fica para **humano**. Este relatório não compromete
sequência.

---

## §8 — Próximos passos

1. **Utilizador valida relatório** e decide entre Cenário A / B.
2. **Claude web propõe primeiro passo da sequência escolhida** com
   spec dedicada.
3. **Execução continua** após aprovação humana.

Pendências preservadas:
- Cabeçalho DEBT.md desactualizado (acção §4.1 XS opcional).
- 8 DEBTs abertos accionáveis (§6.1).
- 3 candidatos cluster Gradient (§6.2).
- 5 pendências fora cluster (§6.3).
- 3 scope-outs reconfirmados (não retomar; per ADR-0097).

### Sub-padrões emergentes para reaplicação cross-cluster

- L3-only parent_bbox N=2 (aguardar reaplicação).
- Template-passo replicado literal N=2 (idem).
- Layout duplo arquitectural aceite N=1.
- Extract helper de replicação inline N=1.
- Triplicação Group bbox N=1.
- Bug arquitectural intencional corrigido N=1.
- Cleanup XS derivado N=1.
- Passo administrativo de auditoria N=2 (este passo).
- Auditoria empírica vs declaração nominal N=1 (inaugural P275).

---

## §9 — Métricas finais

| Métrica | Pré-P275 | Pós-P275 | Delta |
|---|---|---|---|
| Tests workspace (verdes) | 2644 | **2644** | 0 (zero alterações código) |
| ADRs files | 93 | 93 | 0 |
| ADRs "vigentes" critério README | 84 | 84 | 0 |
| DEBTs em aberto/parciais | 10 (assumido em P273.x) | **8** (verificado empíricamente) | **-2 reconciliados** |
| Lint violations | 0 | 0 | 0 |
| Hashes L0 propagados | — | 0 | 0 (zero código tocado) |
| Documentos novos | — | 2 | Diagnóstico Fase A + Relatório |

### §política condições verificadas

- ✓ Cap LOC L1/L3/stdlib hard 0 — real 0; literal.
- ✓ Cap documental Fase A hard 800 — real ~600; folga 25%.
- ✓ Cap documental relatório hard 1500 — real ~600; folga 60%.
- ✓ Tests workspace 2644 preserved bit-exact.
- ✓ Lint zero preserved.
- ✓ Zero alterações código L1/L3/stdlib.
- ✓ Zero alterações L0 prompts (apenas diagnóstico + relatório em
  `00_nucleo/diagnosticos/` + `00_nucleo/materialization/`).
- ✓ ADR-0029 pureza física L1 preserved (absoluto — zero código).
- ✓ Discrepâncias documentadas (§3).
- ✓ Acções manutenção propostas sem execução (§4).

**10 condições §política verificadas — 10 satisfeitas absolutas**
per ADR-0094 Pattern 1 (passo administrativo sem cap LOC aplicado).

---

## §10 — Anti-padrões evitados

### §10.1 — Não criar ADRs novas (per spec §0.4)

P275 inaugura sub-padrões emergentes (auditoria empírica N=1) mas
**NÃO formaliza** via ADR — coerente com anti-padrão over-formalização
explícito P273.17. Sub-padrões preserved aguardando consolidação
N≥3-4.

### §10.2 — Não executar acções de manutenção (per spec §C.1 §4)

P275 **identifica** 4 acções de manutenção (§4 acima) mas **NÃO
executa** nenhuma. Reservadas para passos administrativos XS
futuros consoante decisão humana. Disciplina anti-scope-creep
preservada.

### §10.3 — Não tocar código L1/L3/stdlib (per spec §0.1)

Zero alterações `.rs`. Confirmado via `git status` (working tree
clean exceto novos `.md` em `00_nucleo/diagnosticos/` +
`00_nucleo/materialization/`).

---

## §11 — Marco final P275

**Auditoria empírica pós-P273.17 cumprida**:

- 7 métricas factuais verificadas (5 ✓ + 2 ⚠ soft warnings
  documentadas).
- 8 DEBTs em aberto/parciais enumerados literal (vs 10 assumido em
  P273.x — reconciliação aritmética §A.3.2).
- 3/3 candidatos XS/S cluster Gradient confirmados factualmente
  válidos.
- 5/5 pendências fora cluster preserved sem alteração.
- 4 acções de manutenção propostas para passos futuros (não
  executadas).
- 2 sub-padrões emergentes registados (auditoria N=2 + auditoria
  empírica vs nominal N=1).

Sub-padrão **"diagnóstico imutável" N=33 → N=34 cumulativo** —
29º consumo directo de fonte.

Sub-padrão **"passo administrativo de auditoria" N=1 → N=2
cumulativo emergente** (P125 + P275). Limiar formalização N≥3-4
NÃO atingido. NÃO formalizar ADR.

Cluster Gradient permanece **encerrado definitivamente pós-P273.17**.
P275 confirma estado saudável factual e identifica trabalho residual
sem comprometer sequência.

**Próximo passo natural**: decisão humana entre Cenário A (cleanup
XS combinado) ou Cenário B (atacar DEBT directo) — vide §7.

---

## §12 — Referências

- **Spec P275** — `00_nucleo/materialization/typst-passo-275.md`.
- **Diagnóstico Fase A** —
  `00_nucleo/diagnosticos/diagnostico-auditoria-passo-275.md`.
- **DEBT.md literal** — `00_nucleo/DEBT.md` (fonte autoritativa).
- **ADRs vigentes** — `00_nucleo/adr/typst-adr-*.md` (93 files; 84
  vigentes critério README).
- **ADR-0085** — Diagnóstico imutável (29º consumo).
- **ADR-0029** — Pureza física L1 (preserved absoluto; P275 zero
  código).
- **ADR-0054** — Critério fecho DEBT-1 graded (NO-GO outputs P273.14-16
  preserved per ADR-0097).
- **P273.17 relatório** —
  `00_nucleo/materialization/typst-passo-273-17-relatorio.md`
  (precedente directo; cluster Gradient encerrado definitivamente).
- **Documento reflexão cluster Gradient** —
  `00_nucleo/diagnosticos/typst-cluster-gradient-reflexao.md`
  (output P273.17 standalone).
- **P125** — auditoria DEBTs original (N=1 sub-padrão
  "auditoria"; precedente directo).
- **P156B** — última actualização cabeçalho DEBT.md ("14 abertos");
  desactualizado desde então.
- **P206E** — pattern fecho 3-caminhos (CLOSED / REPLACED-BY /
  OBSOLETED).
- **P206A** — pattern "auditoria empírica revela hipótese inválida"
  (disponível mas não disparado neste passo).

---

*Relatório imutável produzido em 2026-05-18 como output legítimo do
passo administrativo P275. Auditoria empírica pós-cluster Gradient
encerrado confirma estado saudável (2644 tests preserved + lint zero
+ ADR-0029 preserved absoluto); 8 DEBTs em aberto reais (não 10
assumido); 3 candidatos XS/S válidos; 5 pendências fora cluster
preserved; 4 acções manutenção propostas (zero executadas);
sub-padrões emergentes registados para reaplicação cross-cluster
futura. Decisão humana sobre sequência (Cenário A cleanup XS
combinado ou Cenário B DEBT directo) preserved como output do passo
— próximo passo dependente de input humano.*
