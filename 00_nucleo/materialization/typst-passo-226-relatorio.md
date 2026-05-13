# Relatório do passo P226 — ADR meta L0 minimal N=7 + diagnóstico amplo Fase 5 Layout + ADR-0079 PROPOSTO

**Data**: 2026-05-13.
**Spec**: `00_nucleo/materialization/typst-passo-226.md`.
**Tipo**: passo administrativo + diagnóstico amplo agregado num
único passo per decisão humana literal H2. **Encerramento
documental puro** — sem código tocado.
**Magnitude planeada**: M+ (~3-4h). **Magnitude real**: M+
(~2h — eficiência por reuso massivo P215+P156B+ADR-0034
patterns).
**Marco**: **abertura série β "completar Layout" Fase 5
candidata** — primeira abertura formal pós-M9c (complementar
aos N=2 encerramentos P221+P225); pattern "diagnóstico amplo
+ ADR PROPOSTO + roadmap" N=4 → **5 cumulativo**; pattern
"ADR meta + diagnóstico agregado" N=1 **inaugurado P226**.

---

## §1 O que foi feito

P226 cumpre **3 trabalhos agregados** per Decisão H2 humana:

1. **ADR-0080 PROPOSTO** criada — formaliza pattern
   emergente "L0 minimal para refactors aditivos pós-M9c"
   N=7 cumulativo (P217+P218+P219+P220+P222+P223+P224 todos
   Opção γ; P224 divergência consciente vs spec C6 Opção α
   reforçou em vez de suspender). Numeração ADR-0080 escolhida
   após `P226.div-1` registado (spec hipótese ADR-0067 mas
   slot ocupado por `attribute-grammar-scoping`).
2. **Diagnóstico amplo Fase 5**
   `diagnostico-layout-fase-5-completar.md` criado cobrindo
   **4 categorias A+B+C+D** com 13-15 sub-passos
   cumulativos identificados. Magnitudes cumulativas L+ a
   XL (~37-64h total). 3-4 reaberturas arquiteturais
   registadas explicitamente.
3. **ADR-0079 PROPOSTO** criada — "Layout Fase 5 roadmap
   completar Layout (Tudo A+B+C+D)". Paridade estrutural
   ADR-0078. Roadmap identificado mas **NÃO reservado** per
   política P158. Critério promoção PROPOSTO → IMPLEMENTADO
   documentado.

**Tests workspace**: **2039 verdes preservados** (zero
código tocado); **0 violations**; **"Nothing to fix"** hashes.

---

## §2 Auditoria pré-P226 + audit N=7 (C1)

**Audit ADR slots disponíveis**:
- ADR-0067: **ocupado** (`attribute-grammar-scoping`).
- ADR-0068-0078: **todos ocupados** (location-aware-layouter,
  post-recursion-tag-emission, eliminação-counter-state-legacy,
  walk-pipeline-redesign, m7-fixpoint-runtime-fechado,
  comemo-introspector, f3-layouter-substores-trackable,
  vanilla-integration, introspector-completion, regex-l1,
  column-flow-algorithm).
- ADR-0063: **disponível** (slot livre entre 0062 e 0064).
- ADR-0079: **disponível**.
- ADR-0080+: **disponíveis**.

**Decisão de numeração** (`P226.div-1`):
- ADR-0079 para Layout Fase 5 roadmap (paridade spec C7).
- ADR-0080 para meta documental L0 minimal (em vez de
  spec hipótese ADR-0067).

**Audit pattern N=7 confirmado**:
- P217 Content::Columns variant novo — L0 não tocado ✓.
- P218 native_columns stdlib — L0 não tocado ✓ (spec C6
  Opção α rejeitada).
- P219 Layouter arm refactor — L0 não tocado ✓ (spec C7
  Opção α rejeitada).
- P220 Content::Colbreak agregado — L0 não tocado ✓.
- P222 native_measure stdlib + visibility — L0 não tocado ✓.
- P223 Content::Place +float+clearance — L0 não tocado ✓.
- P224 Content::Grid refino substantivo + 3 variants +
  módulo — L0 não tocado ✓ (spec C6 Opção α rejeitada).

**N=7 cumulativo confirmado empiricamente**.

---

## §3 ADR-0080 meta documental L0 minimal N=7 (C2)

`00_nucleo/adr/typst-adr-0080-l0-minimal-para-refactors.md`
criado (~250 linhas) com estrutura paridade ADR-0034 /
ADR-0065 (ADRs meta documentais cristalinos):

- **Status PROPOSTO** (paridade ADR-0034/0065 inicial;
  pode transitar EM VIGOR em passo administrativo XS
  dedicado).
- **Data 2026-05-13**.
- **Validado**: 7 aplicações cumulativas pós-M9c (lista
  empírica completa em tabela).
- **§Contexto**: pattern N=7 emergente; 3 divergências
  conscientes (P218, P219, P224) face a specs Opção α.
- **§Decisão**: refactors aditivos pós-M9c NÃO actualizam
  L0 prompts por defeito; documentação distribuída
  (inline-doc + inventário + ADR + blueprint).
- **§Escopo**: definição precisa de "refactor aditivo"
  (variant novo, field novo, stdlib aditiva/refinada,
  helper visibility, módulo L1 novo, arms cascade) vs
  "refactor não-aditivo" (mudança signature L0-doc, refactor
  estrutural tipo, reabertura decisão arquitectural).
- **§7 Aplicações cumulativas**: tabela com 7 entradas
  detalhadas + nota "divergência consciente N=3" (P218,
  P219, P224).
- **§Consequências**: positivas (overhead reduzido) +
  negativas (L0 não reflecte estado real cumulativo;
  auditor externo cruza fontes).
- **§Alternativas**: Alt A (actualizar todos refactors;
  rejeitada N=7 precedente forte), Alt B (apenas variants
  Content novos; rejeitada N=4 já viola), Alt C (apenas
  módulos L1 novos; rejeitada P224 N=1 já viola).
- **§Cross-references**: ADR-0033/0034/0065 + P217-P224.
- **§Promoção**: PROPOSTO → EM VIGOR via N=8+ sem decisão
  contrária OU passo administrativo XS dedicado.

**`P226.div-1` documentado** em §"Nota numeração":
"spec P226 hipótese previa ADR-0067 mas ADR-0067 já estava
ocupado (`attribute-grammar-scoping`). ADR-0080 escolhido
como próximo slot disponível após ADR-0079 (Layout Fase 5
roadmap)."

---

## §4 Diagnóstico amplo Fase 5 — Categorias A + B (C3 + C4)

`00_nucleo/diagnosticos/diagnostico-layout-fase-5-completar.md`
criado (~450 linhas) cobrindo:

### Categoria A — Cosméticos (sem reabrir decisões)
**5 sub-passos identificados não-reservados**:
- **A.1** stroke Grid + Table inheritance (S+ a M).
- **A.2** fill Grid + Table (S+ a M).
- **A.3** stroke/fill GridCell per-cell (M; depende A.1+A.2).
- **A.4** Block/Boxed outset/radius/clip (P156G+H scope-outs;
  M cada).
- **A.5** Place per-cell alignment override (S+).

**Total Categoria A**: magnitude cumulativa M-L (~6-9h).
**Ordem sugerida**: A.1→A.2→A.3; A.4, A.5 ortogonais.

### Categoria B — Algorítmicos isolados (sem reabrir decisões)
**3 sub-passos identificados não-reservados**:
- **B.1** DEBT-34d Auto track sizing fix (M; **fecha
  DEBT-34d** preservado per `P224.div-1`).
- **B.2** Consumer geometric integration P224.C (M).
- **B.3** Per-cell GridCell atributos `align`/`inset`/
  `breakable` (M; depende B.2).

**Total Categoria B**: magnitude cumulativa M+ a L (~6-9h).
**Ordem sugerida**: B.1→B.2→B.3 sequencial.

---

## §5 Diagnóstico amplo Fase 5 — Categorias C + D (C5 + C6)

### Categoria C — Estruturais reabrindo decisões (maior risco)
**2 sub-passos identificados** com alta complexidade:
- **C.1** Place float real flow contorna (L+;
  **reabre Opção B P219 graded**).
- **C.2** Opção A multi-region completa columns/colbreak
  real (L+ a XL; **reabre P216B** + **DEBT-56b novo**;
  DEBT-56 preservada CLOSED literal).

**Total Categoria C**: magnitude cumulativa L+ a XL
(~15-28h). **Ordem sugerida**: C.1 ⊥ C.2 (ortogonais; C.2
facilita C.1).

### Categoria D — Runtime queries (reabertura ADR-0066)
**5-6 sub-passos identificados** (P160B-F candidatos
pré-existentes):
- **D.1** state(key, init) runtime mutable (M;
  **desbloqueia promoção ADR-0066 PROPOSTO → IMPLEMENTADO**).
- **D.2** metadata(value) attaching (S+; depende D.1).
- **D.3** here()/locate() location-aware (M; depende D.1).
- **D.4** query(target) runtime introspection (M+;
  depende D.1+D.2).
- **D.5** position(target) location-aware (S+; depende
  D.3+D.4).
- **D.6** Cross-document cite refs (L+; ortogonal D).

**Total Categoria D**: magnitude cumulativa L+ a XL
(~10-18h). **Cobertura Introspection bonus**: 17% → ~50%.

### §6+§7 Diagnóstico — Matriz cumulativa + reaberturas
- **Roadmap total**: 13-15 sub-passos cumulativos
  magnitude ~37-64h (L+ a XL).
- **4 reaberturas arquiteturais** registadas explicitamente
  com nota de não-violação de decisões fechadas literais:
  Opção B P219 (preservada; C.1 ortogonal); P216B
  (preservada; C.2 estende); DEBT-56 (preservada CLOSED;
  DEBT-56b novo); ADR-0066 (PROPOSTO → IMPLEMENTADO formal
  pós-D.1).
- **§8 Trade-offs cumulativos** + **§9 Decisão humana
  caso-a-caso** (cenários "baixo risco primeiro" /
  "alto valor primeiro" / "selectivo") documentados.

---

## §6 ADR-0079 Fase 5 Layout PROPOSTO (C7)

`00_nucleo/adr/typst-adr-0079-layout-fase-5-roadmap.md`
criado (~200 linhas) com estrutura paridade ADR-0078:

- **Status PROPOSTO**.
- **Validado**: diagnóstico amplo P226 + decisão humana
  literal P225 §8 + P226 pré-spec.
- **§Contexto**: Layout pós-P225 estado terminal estructural
  reconhecido + decisão humana "completar Layout" Tudo
  A+B+C+D.
- **§Decisão**: materializar 13-15 sub-passos cumulativos
  cobrindo 4 categorias; roadmap identificado mas NÃO
  reservado per política P158.
- **§Reaberturas arquiteturais**: 4 explícitas (Opção B
  P219; P216B; DEBT-56b derivada; ADR-0066) com notas de
  preservação literal das decisões fechadas históricas.
- **§Trade-off cumulativo**: ~37-64h L+ a XL; cobertura
  Layout 89% → 100% literal; Introspection bonus +33pp.
- **§Critério de promoção**: PROPOSTO → IMPLEMENTADO
  (todos materializados OU scope-out parcial formal).
  PROPOSTO → REJEITADA (abandono).
- **§Cross-references**: ADR-0061+0066+0078+0080+0054 +
  P156B+P215+P226 + diagnóstico amplo.
- **§Próximos passos**: lista sub-passos A.1-A.5, B.1-B.3,
  C.1-C.2, D.1-D.6 com magnitudes individuais.

**Política "sem novas reservas" P158 preservada literal**
— 13-15 sub-passos identificados mas NÃO reservados.

---

## §7 Resultados verificação (C8 + C9)

| Critério | Esperado | Real |
|----------|----------|------|
| `cargo test --workspace` | 2039 preservado | **2039 verdes** ✓ (zero código tocado) |
| `crystalline-lint .` | 0 violations | **0 violations** ✓ |
| `crystalline-lint --fix-hashes` | "Nothing to fix" | **"Nothing to fix"** ✓ (L0 prompts não tocados) |
| ADR-0080 criado | sim | ✓ ~250 linhas |
| ADR-0079 criado | sim | ✓ ~200 linhas |
| Diagnóstico amplo criado | sim | ✓ ~450 linhas com 4 categorias + matriz dependências + reaberturas |
| Blueprint §3.0quaterdecies | sim | ✓ ~140 linhas |
| README ADRs distribuição | PROPOSTO 11 → 13 | ✓ actualizado |

---

## §8 Blueprint marca + próximo sub-passo (C10)

**Blueprint §3.0quaterdecies** marca abertura série β
adicionada após §3.0terdecies (P225) e antes de §3.1:
- Distinção qualitativa face a marcas anteriores
  (§3.0duodecies+§3.0terdecies encerramentos; §3.0quaterdecies
  **abertura inaugurada**).
- 3 trabalhos agregados detalhados (ADR-0080 + diagnóstico
  + ADR-0079).
- 4 reaberturas arquiteturais com notas explícitas.
- 5 patterns emergentes consolidados (incluindo N=7
  formalizado + N=5 diagnóstico amplo + N=1 ADR meta
  agregado).
- Distribuição ADRs pós-P226: PROPOSTO 13 (+2);
  IMPLEMENTADO 21; total 67 (+2).
- Saldo DEBTs: 12 preservado.

**Próximo sub-passo — decisão humana**:

| Caminho | Trabalho | Magnitude | Prioridade |
|---------|----------|-----------|------------|
| **A.1** (cosméticos) | stroke Grid + Table inheritance | S+ a M | alta (baixo risco; valida pattern ADR-0080 Opção γ; momentum imediato P226) |
| **B.1** (algorítmicos) | DEBT-34d Auto track sizing fix | M | média-alta (fecha DEBT-34d preservado; algorítmico isolado) |
| **D.1** (runtime) | state(key, init) | M | média (desbloqueia ADR-0066 IMPLEMENTADO + +33pp Introspection) |
| **Promoção ADR-0080** EM VIGOR | passo administrativo XS | XS (~30min) | baixa (paridade ADR-0034/0065 PROPOSTO → EM VIGOR via N=8 ou XS dedicado) |
| Pivot outro módulo | Visualize 54%; Text 52%; Model 50% | varia | baixa-média |

**Recomendação subjectiva**: **A.1** (stroke Grid) — baixo
risco; valida pattern ADR-0080 Opção γ em refactor aditivo
cosmético; momentum cumulativo natural pós-P226 diagnóstico
amplo. Alternativa: **D.1** (state) se humano priorizar
desbloqueio ADR-0066 + Introspection.

**Decisão humana fica em aberto literal** pós-P226.

**Estado pós-P226**:
- Tests workspace: **2039 verdes preservados**.
- Content variants: 59 preservado.
- Stdlib funcs: 59 preservado.
- ADRs: PROPOSTO 11 → **13** (+ADR-0079 + ADR-0080);
  IMPLEMENTADO 21 preservado; total 65 → **67**.
- Saldo DEBTs: **12 preservado**.
- **Layout em estado terminal estructural reconhecido +
  roadmap "completar Layout" Fase 5 candidata formalizado**.
- Política "sem novas reservas" P158 preservada literal.
- **Trajectória aberta**: 13-15 sub-passos A/B/C/D
  identificados não-reservados; decisão humana caso-a-caso.

**Patterns emergentes pós-P226**:
- **"L0 minimal para refactors" N=7 formalizado** em
  ADR-0080 PROPOSTO.
- **"Diagnóstico amplo + ADR PROPOSTO + roadmap" N=4 → 5
  cumulativo** (P156B + P159B + P160 + P215 + P226).
- **"ADR meta + diagnóstico agregado" N=1 inaugurado P226**.
- **"ADR PROPOSTO com materialização parcial graded"
  estendido** — 3 ADRs PROPOSTAS cumulativas (ADR-0066 +
  ADR-0079 + ADR-0080).
- **"Divergência factual material `Pxxx.div-N`" N=2 → 3
  cumulativo** (P215.div-1 + P224.div-1 + **P226.div-1**).
- **"Abertura de Fase Layout pós-M9c" N=1 inaugurado P226**
  (complementar aos N=2 encerramentos §3.0duodecies P221
  + §3.0terdecies P225).
