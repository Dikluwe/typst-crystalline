# Relatório Passo P160 — Diagnóstico Introspection

Materialização do **quarto diagnóstico de módulo focado** após
P157/P158/P159 base + **primeira mudança de módulo cross-domínio
Model → Introspection** desde início da série granular P156C.
**Vigésima quarta aplicação consecutiva** do padrão
diagnóstico-primeiro. Passo puramente documental — sem código
alterado, sem ADR nova, sem novas reservas.

---

## Resumo do diagnóstico

Diagnóstico em
`00_nucleo/diagnosticos/diagnostico-introspection-passo-160.md`
com 6 secções:

### §1 ADRs/DEBTs Introspection
- **ADR-0017 confirmada como reserva sem ficheiro** pré-existente
  ("Introspection runtime adiada"). Sem ficheiro
  `00_nucleo/adr/typst-adr-0017-*.md`. Confirmações cumulativas:
  inventário 148 §A.9 + P159B §3 categoria A + P159A
  cross-reference adiada.
- **Sem DEBTs Introspection formais** — ADR-0017 captura tudo.
  DEBT-18 já fechado por `materialize_time` (P66).
- ADRs aplicáveis: 0017 (reserva), 0033 (paridade observable),
  0054 (graded), 0065 critério #5.

### §2 Inventário código actual
- `01_core/src/rules/introspect.rs`: **1108 linhas**.
- `01_core/src/entities/counter_state.rs`: **333 linhas**.
- 14 fields públicos em `CounterState` cumulativos (numbering_active,
  resolved_labels, headings_for_toc, label_pages, has_outline,
  is_readonly, figure_numbers, figure_label_numbers,
  local_figure_counters + **subpadrão #15 N=3**: lang/bib_entries/
  bib_numbers).
- Pipeline single-pass: `eval` → `introspect` → `layout` →
  `export_pdf`.
- Walk arm cobre 100% dos variants Content existentes (58).
- Comportamento single-pass divergente do vanilla multi-pass
  (DEBT-10 comentário antigo; trabalho parcial cumprido via
  `materialize_time`).

### §3 Features Introspection vanilla
- 13 ficheiros vanilla `introspection/`; **3983 LOC totais**.
- **Cobertura cristalina**: 1/13 implementado (counter); 1/13
  parcial (measure helper privado); 11/13 ausentes.
- **Cobertura observable A.9**: 1/6 = ~17%.
- Categorização: counter sem dependência (já ✓); 11 features
  exigem ADR-0017; measure exige cross-módulo Layout.

### §4 Análise tecto Introspection
- **Tecto puro single-pass**: ~17% inalterado (saturado por
  counter()).
- **Tecto pós-ADR-0017 subset minimal**: ~50%.
- **Tecto pós-ADR-0017 + measure**: ~83-100%.
- **Diferença empírica**:
  - +0pp com refinos qualitativos puros.
  - +33-66pp pós-ADR-0017.
  - +8-16pp adicionais pós-measure.

**Decisão arquitectural-chave §4**: tecto puro **EFECTIVAMENTE
SATURADO**. **Bloco A vazio** — caminho válido per spec P160
"Cenários".

### §5 Sequência candidata
- **Bloco A VAZIO** (tecto puro saturado).
  - 3 refinos qualitativos infraestrutura possíveis (R1/R2/R3 —
    state.figure_kinds, state.heading_levels_present,
    state.equations_count) mas sem features observable.
- **Bloco B** (5 candidatos pós-ADR-0017): P160A (state) M;
  P160B (metadata) S+; P160C (here/locate) M; P160D (query) M+;
  P160E (position) S+.
- **Bloco C** (cross-módulo): measure stdlib expose; cross-doc
  refs; layout-aware introspection.

### §6 Recomendação de execução
**Recomendação primária**: **`ADR-0017-create`** (XS administrativo;
paridade `ADR-0062-create`).
**Pós-promoção**: P160A (state runtime) primeiro candidato
Bloco B.
**Alternativa válida**: **Opção C** (Layout Fase 3 columns/colbreak
ou outro módulo) se prioridade observable Layout for maior.

---

## Confirmação das verificações (1-11)

1. **Diagnóstico produzido** ✓ —
   `00_nucleo/diagnosticos/diagnostico-introspection-passo-160.md`
   com 6 secções canónicas.

2. **Mapa ADR/DEBT documentado** ✓ — §1 confirma ADR-0017
   reserva sem ficheiro + DEBT-18 fechado + sem DEBTs
   Introspection formais.

3. **Inventário código actual factual** ✓ — §2 referência
   explícita a `introspect.rs` 1108 linhas + `counter_state.rs`
   333 linhas + 14 fields públicos enumerados.

4. **Features vanilla categorizadas** ✓ — §3 tabela com 13
   features × cobertura cristalina × 3 dependências (ADR-0017
   / measure / multi-pass).

5. **Análise tecto com estimativas** ✓ — §4 cobertura puramente
   single-pass (~17% saturado) + pós-ADR-0017 (~50%) + pós-measure
   (~83-100%) + diferença empírica (+0pp puro / +33-66pp pós-ADR
   / +8-16pp pós-measure).

6. **Sequência candidata em §5** ✓ — Bloco A VAZIO honestamente
   registado; Bloco B 5 candidatos populado com pré-condições;
   Bloco C cross-módulo listado.

7. **Recomendação concreta em §6** ✓ — recomendação primária
   `ADR-0017-create` (paridade `ADR-0062-create`); alternativa
   Opção C documentada; sujeita a validação humana.

8. **Sem novas reservas criadas** ✓ — política P158/P159
   preservada.

9. **ADR-0061 §"Aplicações cumulativas" actualizada** ✓ — linha
   P160 adicionada com slope "—" (passo documental); padrão
   cross-domínio anotado.

10. **`crystalline-lint`**: ✓ No violations found (sem código
    alterado).

11. **Sem alteração de hashes** ✓ — `entities/content.rs`
    mantém `ec58d849` (**18º passo consecutivo** com interpretação
    L0-baseline).

---

## §Análise de risco (N=24)

**Vigésima quarta aplicação consecutiva** do padrão "§análise
de risco no relatório" (P156F/G/H/I/J/K/L + P157/A/B/C +
P158/A/B/C + P159/A/B/C/D/F/E/G + ADR-0062-create + **P160**).

**Risco realizado**: **baixo** (alinhado com previsão da spec
§"Natureza do passo" — passo diagnóstico puramente documental).

**Eixos avaliados**:

| Eixo | Risco | Justificação |
|------|-------|--------------|
| Estrutura | nulo | Passo documental; sem código alterado. |
| Backwards compat | nulo | Sem alteração de tipos, helpers, ou variants. |
| Hash content.rs | nulo | Preservado `ec58d849` 18º consecutivo (sem alteração). |
| Tests | nulo | 1501 workspace inalterado; sem novos tests. |
| Decisão recomendação §6 | mínimo | Sujeita a validação humana; alternativa Opção C documentada. |
| Bloco A vazio inesperado | aceite | Cenário previsto na spec §"O que pode sair errado"; caminho válido. |
| Cobertura ~17% saturada | aceite | Confirmação factual; não é regressão. |

**Cenários da spec §"O que pode sair errado"**:
- ADR-0017 ter conteúdo concreto não visto antes — **não
  realizado**: confirmado como reserva sem ficheiro pre-existente.
- DEBT Introspection scope amplo — **não realizado**: DEBT-18
  fechado; DEBT-10 comentário antigo já parcial cumprido.
- Features vanilla mais elaboradas que inventário 148 — **não
  realizado**: 1/13 implementado consistente com inventário 148
  §A.9.
- **Bloco A vazio** — **REALIZADO**: tecto puro saturado;
  caminho válido per spec P160. Recomendação muda para
  `ADR-0017-create` análogo a ADR-0062-create.
- Bloco A só 1-2 candidatos — alternativa: 3 refinos qualitativos
  (R1/R2/R3) listados mas adiados sem prioridade observable.
- Cobertura recalculada > 17% — **não realizado**: counters por
  kind contam dentro de counter() já implementado; subpadrão #15
  N=3 é infraestrutura sem cobertura observable.

**Padrão consolidado**: **24 aplicações consecutivas** sem
materialização que exceda risco previsto.

---

## ADR-0061 §"Aplicações cumulativas" anotada

Linha P160 adicionada à tabela:

| Passo | Feature(s) | Slope | Cobertura cumulativa | Tests Δ |
|-------|-----------|------:|---------------------:|--------:|
| P160 | (diagnóstico Introspection — módulo mais fraco 17%) | — | — (sem código; passo documental; **+ Introspection diagnóstico** primeira mudança cross-domínio) | 0 |

Padrões atualizados:
- Granularidade 1-2 features/passo: **N=21 inalterada** (passo
  diagnóstico).
- Inventariar primeiro: **N=23 → 24** (ADR-0065 critério #5
  décima segunda aplicação concreta com diversidade cross-domínio
  nova).
- §análise de risco no relatório: **N=23 → 24**.
- ADR-0064: NÃO directamente aplicável em P160 (passo diagnóstico).
- Subpadrão #15 (infraestrutura state lookup): **N=3 confirmado**
  como infraestrutura única materializável sem ADR-0017.
- Subpadrões #14/#16/P155 cross-feature/refactor field para
  Option: N inalterados (passo diagnóstico).

---

## Confirmações finais

- **ADR-0065 critério #5 décima segunda aplicação concreta**: ✓
  confirmada com diversidade cross-domínio nova (primeira
  mudança Model → Introspection desde série granular P156C).
- **Tecto Introspection puro vs pós-ADR-0017 documentado**:
  ✓ §4 estimativas numéricas (paridade P159B §4 metodologia).
- **ADR-0017 estado factual confirmado**: ✓ reserva sem ficheiro
  mantida; promoção a PROPOSTO via XS administrativo
  recomendada (sujeita a validação humana).

**Decisão crítica registada**: tecto Introspection puro
**EFECTIVAMENTE SATURADO** em ~17% (counter() já cobre o
atingível sem ADR-0017). Bloco A vazio. Recomendação primária
`ADR-0017-create` ou Opção C (Layout Fase 3) — validação humana
final.

---

## Listagem completa de candidatos para decisão humana

### Bloco A — Refinos qualitativos infraestrutura (sem features observable)

3 candidatos XS — não materializáveis em P160 puro per
recomendação §6.1; listados para informação:

| ID | Feature | Tamanho | Subpadrão #15 N=3→? | Justificação adiamento |
|----|---------|:-------:|:-------------------:|------------------------|
| R1 | `state.figure_kinds: HashSet<String>` | XS | N=4 | Sem feature observable; pré-condição query() futura |
| R2 | `state.heading_levels_present: BTreeSet<u8>` | XS | N=4 | Idem |
| R3 | `state.equations_count: u32` | XS | N=4 | Idem |

### Bloco B — Pós-ADR-0017 PROPOSTO (5 candidatos)

| ID | Feature | Tamanho | Pré-condição | Cobertura Δ esperada |
|----|---------|:-------:|--------------|:---------------------:|
| P160A | `state(key, init)` runtime mutable state | M | ADR-0017 PROPOSTO | +6-8pp |
| P160B | `metadata(value)` arbitrary attaching | S+ | ADR-0017 PROPOSTO | +3-5pp |
| P160C | `here()` / `locate()` current location | M | ADR-0017 PROPOSTO + Location type | +6-8pp |
| P160D | `query(target)` runtime introspection | M+ | ADR-0017 PROPOSTO + Location + query engine | +8-10pp |
| P160E | `position(target)` location-aware | S+ | depende P160C | +3-5pp |

**Saturação esperada**: cobertura ~17% → ~50% pós-Bloco B
subset minimal.

### Bloco C — Cross-módulo (não materializável Introspection puro)

| ID | Feature | Bloqueador |
|----|---------|------------|
| (Bloco C) | `measure(body)` stdlib expose | Layout integration (`measure_content` privado) |
| (Bloco C) | Cross-document cite refs | ADR-0017 + multi-document pipeline |
| (Bloco C) | Layout-aware introspection | Layout 2-pass refactor |

---

## Estado pós-P160

- **Layout**: 78% inalterada.
- **Model agregado**: ~50% inalterada.
- **Cobertura ampla impl+impl⁺+parcial**: 77% (inalterada).
- **Cobertura arquitectural**: **82%** inalterada (passo
  documental; sem código alterado).
- **Cobertura Introspection**: **17% confirmada como saturada
  por tecto puro** (counter() já cobre o atingível sem ADR-0017).
- **Hash `entities/content.rs`**: `ec58d849` (**18º passo
  consecutivo** preservado via L0-baseline).
- **63 ADRs** (28 EM VIGOR; ADR-0060 IMPLEMENTADO; 12 PROPOSTO
  incluindo 0062 sem promoção; **ADR-0017 reserva sem ficheiro
  confirmada**).
- **Tests**: 1501 workspace inalterado.
- **Variants Content**: 58 (inalterada).
- **Stdlib funcs**: 48 (inalterada).
- **Padrões consolidados**:
  - Granularidade N=21 (inalterada — passo diagnóstico).
  - **Inventariar primeiro N=23 → 24** (ADR-0065 critério #5
    décima segunda aplicação concreta com diversidade
    cross-domínio nova).
  - Smart→Option Caso A patamar N=7 (inalterado).
  - **§análise risco N=23 → 24**.
  - Estabilidade hash L0 content.rs **N=18**.
  - Tipo entity em ficheiro próprio N=5 (inalterado).
  - Infraestrutura state lookup **N=3 confirmado** como
    infraestrutura única materializável sem ADR-0017.
  - Subpadrão #16 (refino tipo entity sem alteração Content):
    N=3 (inalterado — limiar formalização atingido).
  - P155 cross-feature N=1 (inalterado).
  - Refactor de field para Option N=1 (inalterado).
  - Helper `optional_str` cumulativo N=12 (inalterado).

**ADR-0060 mantém-se `IMPLEMENTADO`**. **ADR-0061 mantém-se
`PROPOSTO`**. **ADR-0017 estado factual confirmado** (reserva
sem ficheiro mantida). **ADR-0062 mantém-se reserva PROPOSTO
sem promoção**.

**Próxima decisão (validação humana de §6)**:

- **Aprovar recomendação §6** → redigir spec `ADR-0017-create`
  (XS administrativo; paridade `ADR-0062-create`).
- **Pós-`ADR-0017-create`**: redirigir para P160A (state runtime)
  primeiro candidato Bloco B.
- **Redirigir para Opção C** → mudar de módulo (Layout Fase 3
  columns/colbreak ou outro) se prioridade observable Layout
  for maior.
- **Listar Bloco A R1/R2/R3** se preferência for refinos
  qualitativos infraestrutura (sem cobertura observable).

**Pausa natural após P160 — Introspection puro confirmado como
saturado em 17%; Bloco A vazio; recomendação primária
`ADR-0017-create` paridade `ADR-0062-create`; alternativa
Opção C válida; primeira mudança de módulo cross-domínio Model
→ Introspection registada; padrão diagnóstico-primeiro N=24
consolidado. Decisão humana sobre próxima direcção tem máxima
informação acumulada.**
