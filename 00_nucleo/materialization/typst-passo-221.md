# Passo 221 — Encerramento Fase 3 Layout (DEBT-56 fecha; ADRs IMPLEMENTADO)

**Série**: 221 (sétimo sub-passo Layout Fase 3; **encerramento
documental** — fecha série P217-P220 + DEBT-56 + transita 2
ADRs).
**Marco**: **fecho de série Layout Fase 3** (paridade fecho
de série P156C-L para Fase 1+2; primeiro encerramento de
fase Layout pós-M9c).
**Tipo**: passo documental puro — sem código tocado.
Transições de status ADR + DEBT + recálculos cumulativos
documentais.
**Magnitude**: S (~30min-1h).
**Pré-condição**: P220 concluído (sub-fase b DEBT-56 4/4 ✓;
Content variants 56; stdlib funcs 55; 1987 tests verdes;
0 violations; §A.5 zero ausentes Layout; **descoberta
empírica P220** corrigiu footnote ⁴⁰ retroactivamente);
ADR-0078 PROPOSTO + ADR-0061 PROPOSTO ambas com critérios
de fecho satisfeitos; humano fixou Caminho 1 P220 §8
(encerramento Fase 3 imediatamente); Opção γ fixada para
blueprint §2.1 ("78% per metodologia + nota explícita
distribuição").
**Output**: 1 ficheiro relatório curto + 4 ficheiros canónicos
editados (ADR-0078, ADR-0061, DEBT.md, inventário 148,
blueprint).

---

## §1 Trabalho

P217-P220 materializaram cumulativamente: variant
`Content::Columns` (P217); stdlib `native_columns` (P218);
consumer real graded Layouter Opção B (P219); variant +
stdlib + arm `Content::Colbreak` agregado (P220). **Sub-fase
(b) DEBT-56 estructuralmente completa (4/4)**. P221 fecha
documentalmente:

- **ADR-0078 PROPOSTO → IMPLEMENTADO** (column flow algorithm
  materializado per Opção B graded; refino multi-region flow
  real fica como scope-out documentado).
- **ADR-0061 PROPOSTO → IMPLEMENTADO** (Layout Fase 1 + 2 +
  3 cumpridas estructuralmente; refinos `measure`/`place`
  Fase 3 ficam como Fase 4 candidata futura).
- **DEBT-56 → ENCERRADO** (sub-fases a + b completas; multi-region
  flow real scope-out documentado).
- **Inventário 148** Tabela B.2 actualização cumulativa
  (Columns + Colbreak variants); §A.5 final `12/1/5/0/0 =
  18` (zero ausentes); footnote ⁴² P221 consolidada
  (P217-P220 cumulativo).
- **Blueprint §2.1** Layout: anotação Opção γ (78% per
  metodologia + nota explícita "12 impl + 5 parcial").
- **Marca §3.0duodecies** blueprint — encerramento Fase 3
  + DEBT-56 + transições ADR.

**Decisão arquitectural central — fecho de série pós-M9c**:

P221 é o **primeiro encerramento de série Layout pós-M9c**.
Paridade estrutural com fecho de série P156I (Fase 2 Layout
fechada) + P155 (Fase 1 Model fechada) + P204H/P205E (M8/F3
ACEITE finais). Forma metodológica:

- Transições de status formais (PROPOSTO → IMPLEMENTADO).
- Anotações cumulativas em ADRs (preserva histórico).
- Reclassificação documental cumulativa.
- Marca cirúrgica blueprint (paridade §3.0duodecies pattern
  marca-por-fecho).
- Zero código tocado.

Reuso de dados (sem recolha nova):

- ADR-0078 anotações P217 + P218 + P219 + P220
  cumulativas.
- ADR-0061 §"Status" caminho 1 (já 50% concluído pós-P156J;
  P221 cumpre 100% sub-fase (b) — caminho 1 100% mais
  scope-out formal de refinos `measure`/`place`).
- DEBT-56 critério de fecho (5 itens) — todos cumpridos
  P216A+B (refactor a) + P217-P220 (consumer b).
- Footnote ⁴⁰ P219 + recontagem auditada P220 — base para
  ⁴² consolidada.
- §3.0undecies P214 marca formato — paridade para
  §3.0duodecies.

---

## §2 Cláusulas (10)

### C1 — Auditoria estado factual pré-P221

Verificação empírica antes de transições:

```
cargo test --workspace 2>&1 | tail -3
crystalline-lint .
grep -c "^    [A-Z][A-Za-z]\+\(\s\|{\|(\)" 01_core/src/entities/content.rs
grep -n "Content::Columns\|Content::Colbreak" 01_core/src/rules/layout/mod.rs
```

Critério:
- Tests **1987 verdes** (P220 baseline confirmado).
- **0 violations** preservadas.
- **56 variants** Content (Columns + Colbreak adicionados).
- Arms `Columns`/`Colbreak` presentes em `layout_content` +
  `measure_content_constrained`.

Se algum critério divergir: registar `P221.div-1` e
investigar antes de prosseguir.

### C2 — ADR-0078 PROPOSTO → IMPLEMENTADO

Editar `00_nucleo/adr/typst-adr-0078-column-flow-algorithm.md`:

- **Status**: `PROPOSTO` → **`IMPLEMENTADO`**.
- **Data**: anotar "PROPOSTO 2026-05-12 → IMPLEMENTADO
  2026-05-12 (P221)" (mesmo dia; trajectória rápida pós-M9c).
- **Materialização** (paridade ADR-0073/0074/0075 pattern):
  - `00_nucleo/materialization/typst-passo-216A-relatorio.md`
  - `00_nucleo/materialization/typst-passo-216B-relatorio.md`
  - `00_nucleo/materialization/typst-passo-217-relatorio.md`
  - `00_nucleo/materialization/typst-passo-218-relatorio.md`
  - `00_nucleo/materialization/typst-passo-219-relatorio.md`
  - `00_nucleo/materialization/typst-passo-220-relatorio.md`
  - `00_nucleo/materialization/typst-passo-221-relatorio.md`
    (encerramento série).
- **Validado**: anotar "P217-P220 6 sub-passos materializados;
  1987 tests verdes; 0 violations; Opção B graded literal;
  multi-region flow real Opção A scope-out documentado".
- **Bloco final** anotação `### P221 encerramento série
  2026-05-12`:

```markdown
**Série P217-P220 fechada estructuralmente**:
- P216A+B: Region + Regions sub-fase (a) (refactor).
- P217: Content::Columns variant.
- P218: native_columns stdlib.
- P219: consumer multi-column real graded (Opção B).
- P220: Content::Colbreak agregado + arm downgrade graded.

**6 condições §"Plano materialização" satisfeitas**:
1. ✓ Region + Regions abstractions (P216A+B).
2. ✓ Content::Columns + Content::Colbreak variants (P217+P220).
3. ✓ native_columns + native_colbreak stdlib (P218+P220).
4. ✓ Layouter consumer Opção B graded (P219+P220).
5. ✓ Test suite multi-column verde (15 P218 + 8 P219 + 15
   P220 + 6 P217 = 44 tests Fase 3 cumulativos).
6. ✓ Multi-region flow real scope-out documentado (Opção
   A diferida a P-Layout-Fase4 candidato; não-reservada).

**Transição PROPOSTO → IMPLEMENTADO ratificada**.

DEBT-56 fecha simultaneamente (ver DEBT.md). ADR-0061
transita simultaneamente (ver ADR-0061).
```

### C3 — ADR-0061 PROPOSTO → IMPLEMENTADO

Editar `00_nucleo/adr/typst-adr-0061-layout-fase-x-roadmap.md`:

- **Status**: `PROPOSTO` → **`IMPLEMENTADO`**.
- **§"Status"** actualização: Caminho 1 (Fase 3 materializada)
  cumprido 100% em sub-fase (b); refinos `measure`/`place`
  ficam como **Fase 4 candidata futura** (paridade Caminho 2
  scope-out formal).
- **Data**: anotar "PROPOSTO 2026-04-25 → IMPLEMENTADO
  2026-05-12 (P221)".
- **Materialização cumulativa série Layout** (paridade ADR-0060
  pattern Fase 1 fechada P155):
  - Fase 1: P156C (pad/hide), P156D (h/v), P156E (pagebreak),
    P156F (skew).
  - Fase 2: P156G (block), P156H (boxed), P156I (stack).
  - Fase 3 sub-1: P156J (repeat), P156L (pad refino).
  - Fase 3 sub-fase (a): P216A (Region), P216B (Regions
    minimal).
  - Fase 3 sub-fase (b): P217 (Columns variant), P218
    (native_columns), P219 (consumer real graded), P220
    (Colbreak agregado).
- **Bloco final §"Aplicações cumulativas"** anotação `###
  P221 encerramento Fase 3 2026-05-12`:

```markdown
**Fase 3 Layout fechada estructuralmente em P221**:
- columns + colbreak: materializados graded (Opção B; refino
  multi-region real fica como Fase 4 candidata futura).
- repeat: já materializado P156J (Fase 3 sub-passo 1).
- skew: já materializado P156F.
- Refinos pendentes Fase 4 candidata: `measure(body)` stdlib
  expose (Bloco A diagnóstico P215 — depende ADR-0066 ainda
  PROPOSTO; trabalho XS isolado); `place` float/clearance
  (refino column scope).

**Caminho 1 ADR-0061 §"Status" 100% cumprido** —
sub-fase (a) Region/Regions + sub-fase (b) columns/colbreak.
Refinos `measure`/`place` ficam como Fase 4 candidata
NÃO-reservada per política P158.

**Transição PROPOSTO → IMPLEMENTADO ratificada**.

Distribuição ADRs: PROPOSTO 13 → 11 (ADR-0078 + ADR-0061
transitam); IMPLEMENTADO 19 → 21.
```

### C4 — DEBT-56 ENCERRADO

Editar `00_nucleo/DEBT.md`:

- **Título** DEBT-56: adicionar `— ENCERRADO (Passo 221) ✓`.
- **Fechado em**: 2026-05-12 (P221).
- **Etiqueta de fecho**: **CLOSED** (paridade pattern P206E
  — DEBT-53 CLOSED via materialização).
- **Justificação literal**: "Sub-fases (a) refactor Region/
  Regions (P216A+B) + (b) consumer real graded
  (P217-P220) materializadas. Critério §"Plano" 5/5
  cumprido (ADR-0078 IMPLEMENTADO; columns + colbreak
  materializados graded; tests verdes; lint zero; inventário
  148 actualizado). Refino multi-region flow real fica como
  Fase 4 candidata NÃO-reservada (per política P158)".
- **Histórico preservado** (per pattern P201/P202 + P206E):
  conteúdo original DEBT-56 mantido abaixo do título com
  data de fecho.

**Saldo DEBTs**:
- Pré-P221: 14 abertos (P156B abriu DEBT-56 levando total a
  14; mantido até P221).
- Pós-P221: **13 abertos** (DEBT-56 fecha).

### C5 — Inventário 148 actualização cumulativa

Editar `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`:

**§A.5 Layout**: estado final pós-P221:
- `columns(n)`: ⁴¹ → ⁴² **`parcial`** (consolida P217-P219).
- `colbreak()`: ⁴¹ → ⁴² **`parcial`** (consolida P220).
- Tabela A.5 final: `12/1/5/0/0 = 18` (zero ausentes;
  paridade descoberta empírica P220).

**Tabela B.2 Content variants**: actualização cumulativa
P217 + P220:
```markdown
| `Columns {count, gutter, body}` | ColumnsElem | `parcial` | Passo 221 (cumulativo P217-P219) | variant + stdlib + arm graded; multi-region flow real scope-out |
| `Colbreak {weak}` | ColbreakElem | `parcial` | Passo 220 | variant + stdlib + arm downgrade graded; sem `to` (paridade vanilla) |
```

Linha "Vanilla-only (ausentes)" actualizada: remover
`ColumnsElem` e `ColbreakElem` da lista (agora `parcial`).

**Total user-facing pós-P221**:
- §A.5 final `12/1/5/0/0 = 18` (descoberta empírica P220
  preservada).
- Total per categoria (com correção P220 cumulativa): a
  recalcular per soma das 9 linhas da Tabela A.

Recálculo aritmético P221 (paridade P214 metodologia):
- Markup syntactic ³⁹: `11/3/3/1/0 = 18`.
- let/set/show: `7/1/4/1/0 = 13`.
- Text features: `7/5/1/8/2 = 23`.
- Math: `6/6/1/0/0 = 13`.
- Layout ⁵...⁴²: `12/1/5/0/0 = 18` (P221 corrigido).
- Model: `7/4/7/4/0 = 22`.
- Visualize: `6/1/1/5/0 = 13`.
- Foundations: `9/1/4/1/0 = 15`.
- Introspection ³⁸: `3/2/1/0/0 = 6`.
- **Total: `68/24/27/20/2 = 141`**.
- Cobertura user-facing total: `(68+24)/141 ≈ **65%**`
  (paridade descoberta P220 — preservada).

**Footnote ⁴² P221** consolidada substituindo ⁴⁰ + ⁴¹:
```markdown
⁴² — Ajuste P221 (encerramento Fase 3 Layout — consolida
P217+P218+P219+P220+correcção retroactiva auditada): 2
entradas reclassificadas (`columns(n)` + `colbreak()`
ausente → parcial). Distribuição §A.5 final: `12/1/5/0/0
= 18` (zero ausentes). Cobertura Layout: 13/18 ≈ 72%
preservada (parcial fora numerador per metodologia §A.9
P213). Variants Content: 54 → 56 (+Columns P217 + Colbreak
P220). Stdlib funcs: 53 → 55. Tests workspace: 1939 →
1987 (+48 cumulativo). 0 violations preservadas. Multi-region
flow real Opção A scope-out per ADR-0078 (Fase 4 candidata
NÃO-reservada). ADR-0078 transitada PROPOSTO →
IMPLEMENTADO P221; ADR-0061 idem. DEBT-56 ENCERRADO P221.
```

Footnotes ⁴⁰ + ⁴¹ podem ser mantidas (histórico) ou
removidas (consolidação). **Decisão fixada em C5**:
**manter ⁴⁰ + ⁴¹ + ⁴²** (histórico preservado per pattern
P204H+ "histórico textual preservado"). Reduz inflação mas
mantém rastreabilidade.

### C6 — Blueprint §2.1 anotação Opção γ + §3.0duodecies marca

Editar `00_nucleo/diagnosticos/blueprint-projecto.md`:

**§2.1 linha Layout** Opção γ fixada (decisão humana):

Antes (pós-P214):
```
| Layout ⁽ᴾ²¹⁴⁾ | 78% | quase total (Fase 1+2+3 sub-passo 1 fechadas) |
```

Depois (pós-P221, Opção γ):
```
| Layout ⁽ᴾ²¹⁴⁾⁽ᴾ²²¹⁾ | 78% (12 impl + 5 parcial) | quase total — Fase 3 fechada estructuralmente; refinos `measure`/`place` Fase 4 candidata |
```

**Marca §3.0duodecies adicionada** após §3.0undecies P214,
antes de §3.1 (paridade pattern marca-por-fecho):

```markdown
### §3.0duodecies Marca de actualização — [P221] **Encerramento Fase 3 Layout (DEBT-56 fecha; ADRs IMPLEMENTADO)**

**Data**: 2026-05-12.

P221 fecha cirúrgicamente a **série Layout Fase 3
cumulativa** materializada em P217+P218+P219+P220 (sub-fase
b DEBT-56) precedida por P216A+P216B (sub-fase a DEBT-56)
e P215 (diagnóstico Fase 3). **Encerramento documental
puro** — sem código tocado em P221.

**Distinção qualitativa face a marcas anteriores**:

- Marcas §3.0quater a §3.0octies (5 marcas): cobrem
  **encerramentos de série** (P207E + P208D + P209E + P210C
  + P211A).
- Marca §3.0nonies (P212): cobre **encerramento de marco
  M9c**.
- Marca §3.0decies (P213) + §3.0undecies (P214):
  recálculos de categorias.
- **Marca §3.0duodecies (esta P221)**: cobre **encerramento
  de Fase 3 Layout pós-M9c** — pattern emergente novo N=1
  pós-M9c (paridade estrutural P156I Fase 2 fechada
  pré-M9c; precedente cumulativo agora N=2).

**Transições de status**:
- ADR-0078 column flow algorithm: PROPOSTO →
  **IMPLEMENTADO**.
- ADR-0061 Layout Fase X roadmap: PROPOSTO →
  **IMPLEMENTADO** (Fase 1 + 2 + 3 cumpridas; refinos
  `measure`/`place` Fase 4 candidata futura).
- DEBT-56 column flow Fase 3 Layout: EM ABERTO →
  **ENCERRADO** (CLOSED via materialização).

**Mudanças factuais cumulativas P216A-P220**:
- 2 variants Content novos (Columns P217 + Colbreak
  P220) — total 54 → 56.
- 2 stdlib funcs novas (native_columns P218 +
  native_colbreak P220) — total ~53 → 55.
- 1 type novo L1 (`Region` + `Regions` co-habitantes em
  `entities/region.rs` P216A+B).
- ~325 substituições mecânicas em 6 ficheiros L1
  (refactor sub-fase a; zero mudança observable).
- 1 arm refactored substantivo (`Content::Columns`
  Layouter P219; Opção B graded — width reduzida
  temporariamente; multi-region flow real scope-out).
- 1 arm aditivo (`Content::Colbreak` Layouter P220;
  Opção β graded — downgrade a pagebreak via reuso
  `Layouter::new_page` P156E).
- 48 tests novos cumulativos: 1939 → **1987**.
- **§A.5 Layout zero ausentes** pós-P221 (correcção
  retroactiva auditada P220 preservada).
- Cobertura Layout: **78% per metodologia** (12 impl + 5
  parcial; Opção γ paridade visual com nota explícita
  distribuição).

**Pattern emergente "encerramento Fase pós-M9c" N=1
formalizado**:

- P221 é primeira aplicação pós-M9c.
- Precedente estrutural pré-M9c: P156I (Fase 2 Layout
  fechada) + P155 (Fase 1 Model fechada).
- Pattern reusável para fechamentos Fase futura (Fase 4
  Layout candidata; Model Fase 3 candidata; etc.).

**Pattern emergente "L0 minimal para refactors" N=4 →
candidatura formal**:

- P217+P218+P219+P220 todos Opção γ (sem extensão L0
  formal; documentação inline).
- N=4 atinge limiar formalização N=3-4 ultrapassado.
- **Promoção a ADR meta documental fica como decisão
  diferida** (paridade política "sem novas reservas"
  P158). Trabalho XS administrativo se humano julgar
  útil.

**Política "sem novas reservas" preservada per P158**:
- Fase 4 Layout candidata (refinos `measure`/`place`)
  identificada mas NÃO reservada.
- Multi-region flow real Opção A documentada mas NÃO
  reservada.

**Estado pós-P221**:
- Sub-fase (a) DEBT-56: 2/2 ✓ (P216A + P216B).
- Sub-fase (b) DEBT-56: 4/4 ✓ (P217 + P218 + P219 +
  P220).
- DEBT-56 fechado; ADR-0078 IMPLEMENTADO; ADR-0061
  IMPLEMENTADO.
- Layout cobertura: 78% per metodologia (estável; ganho
  qualitativo via 2 reclassificações ausente → parcial;
  zero ausentes).
- Tests workspace: 1987 verdes; lint: 0 violations.
- 7 séries M9c (P207-P211) + P212 (M9c) + P213/P214
  (recálculos) + P215 (diagnóstico) + **P216A-P221
  Layout Fase 3** (7 sub-passos materialização + 1
  encerramento).
- Trajectória aberta: opções para próxima sessão
  (decisão humana).
- Marco M9c preservado como referência arquitectural
  estável.

Reescrita ampla deste blueprint mantém-se fora-de-escopo
(per padrão consolidado P204H+...+P214). Esta marca
cirúrgica regista **encerramento de Fase Layout pós-M9c**,
qualitativamente distinta de marcas-por-série + marcas-por-
marco + marcas-de-recálculo.
```

### C7 — Verificação tests workspace

Critério: 1987 verdes preservados (P221 é documental puro).

```
cargo test --workspace 2>&1 | tail -3
```

**Erro tolerado**: zero. P221 não toca código; qualquer
red indica problema externo.

Hipótese provável: 1987 verdes preservados literais.

### C8 — Verificação lint

```
crystalline-lint .
crystalline-lint --fix-hashes .
```

Critério: 0 violations. **Sem hashes propagados** —
P221 só edita ficheiros documentais (DEBT.md, ADRs,
blueprint, inventário 148); não toca L0 prompts em
`00_nucleo/prompts/`.

Esperado: "Nothing to fix" pós-`--fix-hashes`.

### C9 — Verificação cumulativa final

Auditoria empírica pós-P221:

```
# ADR status
grep "Status.*IMPLEMENTADO" 00_nucleo/adr/typst-adr-0078-column-flow-algorithm.md
grep "Status.*IMPLEMENTADO" 00_nucleo/adr/typst-adr-0061-layout-fase-x-roadmap.md

# DEBT status
grep -A 2 "DEBT-56.*ENCERRADO" 00_nucleo/DEBT.md

# Inventário 148
grep -A 1 "Layout ⁵" 00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md

# Blueprint marca
grep "§3.0duodecies" 00_nucleo/diagnosticos/blueprint-projecto.md
grep "Layout ⁽ᴾ²¹⁴⁾⁽ᴾ²²¹⁾" 00_nucleo/diagnosticos/blueprint-projecto.md
```

Critério (5 verificações):
- ADR-0078 status IMPLEMENTADO ✓
- ADR-0061 status IMPLEMENTADO ✓
- DEBT-56 ENCERRADO ✓
- Inventário 148 §A.5 Layout linha actualizada ✓
- Blueprint §3.0duodecies + §2.1 marcador P221 ✓

### C10 — Decisão sobre próximo trabalho

P221 fecha Fase 3 Layout estructuralmente. Decisão humana
sobre próxima sessão entre opções (paridade política "sem
novas reservas" P158 — todas são candidatos, NÃO reservas):

| Caminho | Trabalho | Magnitude | Prioridade subjectiva |
|---------|----------|-----------|------------------------|
| **Caminho 1** | **Fase 4 Layout** — refinos `measure`/`place` (sub-passos isolados S+ cada) | varia | média (Layout 78% → 89% se ambos materializados; isolados de refactor) |
| **Caminho 2** | Pivot Bloco C P222 — `measure(body)` stdlib expose (single feature) | S+ (~1-2h) | média (subset Caminho 1; win rápido §A.9 estricto 83% → 100%) |
| **Caminho 3** | Pivot outro módulo (Visualize 54%; Text features 52%; Markup 78%; Model 50%) | varia | baixa-média |
| **Caminho 4** | ADR meta administrativa — formalizar pattern "L0 minimal para refactors" N=4 + "encerramento Fase pós-M9c" N=1 | XS (~30min) | baixa (passo administrativo paridade P213/P214/P160A) |
| **Caminho 5** | Adiar Layout completo; outro objectivo arquitectural (e.g. paridade vanilla retomada per ADR-0073/0075 já ACEITES) | varia | baixa |

**Recomendação subjectiva**: **Caminho 1 (Fase 4 Layout)** OU
**Caminho 4 (ADR meta administrativa)** dependendo de
prioridade:
- Se humano quer continuar Layout: Caminho 1 (`measure`
  + `place` ambos isolados; sub-passos pequenos).
- Se humano quer consolidar metodologia: Caminho 4 (ADR
  meta documentaria padrões N=4 + N=1 emergentes da série
  P216A-P221).

**Estado humano fixado pré-P221**: "focar no Layout até
onde der". Caminho 1 alinha; Caminho 4 desvia para
documentação.

**Decisão humana fica em aberto literal**. P221 não
compromete trabalho subsequente per política "sem novas
reservas".

---

## §3 Output

1 ficheiro relatório:
`00_nucleo/materialization/typst-passo-221-relatorio.md`.

Estrutura (~6-8 KB) com 8 §s:

- §1 O que foi feito (sumário 3-5 linhas — encerramento
  Fase 3 Layout).
- §2 Auditoria pré-P221 (C1 verificações; estado factual
  P220 baseline).
- §3 Transições ADR (C2 + C3; ADR-0078 + ADR-0061
  IMPLEMENTADO).
- §4 DEBT-56 ENCERRADO (C4).
- §5 Inventário 148 actualização cumulativa (C5; Tabela
  B.2 + §A.5 + footnote ⁴²).
- §6 Blueprint §2.1 Opção γ + §3.0duodecies (C6;
  marca-por-fecho).
- §7 Resultados verificação (C7+C8+C9; tests + lint +
  auditoria pós).
- §8 Decisão humana caminhos 1-5 (C10; sem fixar).

Código alterado: **zero** (passo documental puro).

Ficheiros canónicos editados:
- **Editado**: `00_nucleo/adr/typst-adr-0078-column-flow-algorithm.md`
  (status + materialização + bloco encerramento).
- **Editado**: `00_nucleo/adr/typst-adr-0061-layout-fase-x-roadmap.md`
  (status + bloco encerramento Fase 3).
- **Editado**: `00_nucleo/DEBT.md` (DEBT-56 título ENCERRADO).
- **Editado**: `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`
  (Tabela B.2 + §A.5 + footnote ⁴² P221 consolidada).
- **Editado**: `00_nucleo/diagnosticos/blueprint-projecto.md`
  (§2.1 Layout Opção γ + §3.0duodecies marca).
- **Editado**: `00_nucleo/adr/README.md` (atualização
  cumulativa série Layout Fase 3 anotada — paridade
  ADR-0060 P155 pattern; distribuição ADRs PROPOSTO 13 →
  11; IMPLEMENTADO 19 → 21).

**Sem novos ficheiros**.

---

## §4 Não-objectivos

- Promover ADR-0078 a `EM VIGOR` em vez de `IMPLEMENTADO`
  — ADR-0078 é decisão técnica concreta materializada
  (`IMPLEMENTADO` correcto per vocabulário ADRs).
- Promover ADR-0061 a `EM VIGOR` — idem; é decisão técnica
  (roadmap) materializada.
- Reabrir DEBT-56 com nova hipótese multi-region flow
  real — Opção A documentada como **scope-out** + Fase 4
  candidata NÃO-reservada per política P158.
- Promover pattern "L0 minimal" a ADR meta documental em
  P221 — Caminho 4 candidato separado se humano priorizar.
- Reescrita ampla blueprint — pattern P204H+
  "fora-de-escopo" preservado.
- Tocar em código `.rs` — passo documental puro.
- Tocar em hashes L0 — sem prompts L0 alterados em P221.
- Abrir novos DEBTs — Fase 4 candidata identificada mas
  NÃO formalizada como DEBT.
- Criar ADR nova — P221 é encerramento, não nova decisão.
- Reescrita §3.1 datada 2026-04-25 — preservação histórica
  per padrão P204H+.
- Materializar `measure`/`place` refinos — diferidos a Fase
  4 candidata.

---

## §5 Riscos a evitar

1. **Transição ADR-0078 sem verificar 6 condições**: §"Plano
   de materialização" lista 6 condições; C2 deve verificar
   todas explicitamente em bloco anotação. Mitigação:
   bloco `### P221 encerramento série` lista 1-6
   explicitamente com ✓ por condição.
2. **Transição ADR-0061 sem ratificar refinos pendentes**:
   `measure`/`place` ficam pendentes. Decisão humana
   confirmou "focar Layout até onde der" sem comprometer-se
   a refinos. Anotar **explicitamente** "Fase 4 candidata
   NÃO-reservada" em ADR-0061.
3. **DEBT-56 fecha sem verificar critério fecho 5 itens**:
   §"Critério de fecho" lista 5 items. C4 deve listar
   ✓/scope-out por item.
4. **Marca §3.0duodecies inflada**: paridade §3.0undecies
   (P214) estrutura. Sem detalhe técnico excessivo —
   referenciar relatórios P217-P220 em vez de duplicar.
5. **Footnote ⁴² consolida ⁴⁰ + ⁴¹ removendo histórico**:
   Decisão fixada em C5 — **manter** ⁴⁰ + ⁴¹ + ⁴² (paridade
   P204H+ histórico preservado). Reduzir inflação só se
   redundância exacta.
6. **Distribuição ADRs incorrecta**: P221 transita 2 ADRs
   (PROPOSTO → IMPLEMENTADO). PROPOSTO 13 → 11; IMPLEMENTADO
   19 → 21. Verificar contagem global em README ADRs.
7. **Mudança observable inadvertida**: P221 é documental;
   `cargo test` deve permanecer 1987 verdes. Qualquer
   regressão indica problema externo.
8. **L0 hashes desactualizados**: P221 não toca L0 prompts;
   `crystalline-lint --fix-hashes` deve reportar "Nothing
   to fix".
9. **Opção γ §2.1 confusa em leitura rápida**: linha
   "78% (12 impl + 5 parcial)" pode ser interpretada como
   percentagem composta. Mitigação: legenda da tabela
   §2.1 já explica metodologia (impl + impl⁺ / total).
   Adicionar nota se necessário.
10. **Promover ADR meta "L0 minimal" prematuramente**:
    Caminho 4 C10 é candidato separado. P221 NÃO promove
    formalmente — apenas regista N=4 patamar em
    §3.0duodecies. Política consistente N=3-4 mínima.

---

## §6 Hipótese provável

C1 confirmará estado factual P220 baseline (1987 verdes;
0 violations; 56 variants).

C2 transita ADR-0078 PROPOSTO → IMPLEMENTADO; 6 condições
verificadas explicitamente.

C3 transita ADR-0061 PROPOSTO → IMPLEMENTADO; refinos
`measure`/`place` anotados como Fase 4 candidata.

C4 fecha DEBT-56 (CLOSED via materialização); critério
fecho 5/5.

C5 actualiza inventário 148 cumulativo (Tabela B.2 +
§A.5 + footnote ⁴² consolidada preservando ⁴⁰ + ⁴¹
histórico).

C6 actualiza blueprint §2.1 Opção γ (78% + nota explícita
distribuição) + adiciona §3.0duodecies marca-por-fecho.

C7 reporta 1987 tests verdes preservados.

C8 reporta 0 violations; "Nothing to fix".

C9 verifica 5 critérios auditoria cumulativa pós.

C10 deixa caminhos 1-5 em aberto; recomendação subjectiva
Caminho 1 ou Caminho 4 dependendo de prioridade humana.

Custo real: S (~30min-1h documental). Maior parcela em
C5 (recálculo cumulativo + footnote ⁴² consolidação) +
C6 (marca §3.0duodecies elaboração).

Mas é hipótese, não decisão. C1-C10 fixam-se empíricamente.

---

## §7 Particularidade P221

P221 é estruturalmente distinto na trajectória pós-M9c:

- **Primeiro encerramento de Fase Layout pós-M9c** —
  precedente estrutural P156I (Fase 2 Layout fechada
  pré-M9c) + P155 (Fase 1 Model fechada pré-M9c). Pattern
  N=2 cumulativo.
- **Primeiro passo documental puro pós-M9c** (paridade
  P212 M9c encerramento estrutural; P213 + P214 + P215 +
  P216A+B + P217-P220 todos com código tocado).
- **2 ADRs transitam simultaneamente** — ADR-0078 (column
  flow algorithm específica) + ADR-0061 (Layout Fase X
  roadmap geral). Coerência cumulativa material.
- **1 DEBT fecha simultaneamente** — DEBT-56. Pattern
  P206E (DEBT-53 + DEBT-54 fecharam em P206E) replicado
  parcialmente.
- **Pattern emergente "L0 minimal para refactors" N=4
  formalmente registado em §3.0duodecies** — sem promover
  a ADR meta (política P158 consistente).
- **Pattern emergente "encerramento Fase pós-M9c" N=1
  inaugurado** — formalização em §3.0duodecies; pattern
  reusável.
- **Decisão Opção γ fixada para §2.1 blueprint** — paridade
  visual com nota explícita "78% (12 impl + 5 parcial)".
  Primeira aplicação. Possível pattern futuro se humano
  julgar útil em outras categorias.
- **Cobertura Layout 78% preservada per metodologia**
  (correcção retroactiva P220 mantida: ganho qualitativo
  via 2 reclassificações ausente → parcial; zero ausentes).
- **Distribuição ADRs: PROPOSTO 13 → 11; IMPLEMENTADO 19
  → 21**.
- **Saldo DEBTs: 14 → 13** (DEBT-56 fecha).

Por isso §5 risco 1 (transição ADR-0078 sem verificar 6
condições) é o mais provável. Defesa: bloco anotação
explícito com 1-6 marcados ✓ por condição. Replica pattern
ADR-0075 (7 condições) + ADR-0073 (cond 9 fechada
retroactivamente em P206E).

**Critério de aceitação P221**:
- 2 ADRs transitam (PROPOSTO → IMPLEMENTADO).
- 1 DEBT fecha (EM ABERTO → CLOSED).
- Inventário 148 actualizado cumulativamente.
- Blueprint §3.0duodecies marca + §2.1 Opção γ.
- 1987 tests verdes preservados (zero código tocado).
- 0 violations preservadas.
- "Nothing to fix" lint.
