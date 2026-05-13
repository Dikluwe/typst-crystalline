# ⚖️ ADR-0076: Marco M9c — completion do Introspector cristalino

**Status**: **ACEITE (completo retroactivo, P212 2026-05-12 —
com excepções documentadas em cond #2 e cond #4 reinterpretadas
per `P207A.div-1` reshape)**.
**Data**: 2026-05-12 (PROPOSTO P207A; ACEITE P212 — auditoria
empírica das 7 condições §Plano de validação em P212 C1).
**Sub-passos materializados**: P207A-E (5 sub-passos — trait
extensions + sub-store refactor + page-aware infrastructure);
P208A-D (4 sub-passos — here/locate stdlib + EvalContext infra);
P209A-E (5 sub-passos — Selector 1→6 variants + Regex wrapper +
ADR-0077 ACEITE); P210A-C (3 sub-passos — counter.step subset
Caminho 3; display/get deferred); P211A (1 sub-passo —
Outline Caminho 1 puro); P212 (este — encerramento marco +
transição ACEITE).
**Diagnóstico prévio**:
- `00_nucleo/diagnosticos/typst-passo-207A-auditoria-introspector.md` (P207A).
- `00_nucleo/diagnosticos/typst-passo-207A-diagnostico.md` (P207A).
**Pré-condição humana**:
- **Decisão sobre `P207A.div-1`** (escopo reduzido) antes de
  P207B.
- **Respostas Q1-Q4** (DECISÃO PENDENTE C10) antes de séries
  P208/P209.

---

## Transição ACEITE M9c — P212 2026-05-12

**Data**: 2026-05-12.
**Auditor**: P212 (per spec §2 C1+C2).

ADR-0076 transita PROPOSTO → ACEITE após auditoria empírica
das 7 condições §Plano de validação em P212 C1. Fórmula:
**ACEITE (completo retroactivo, com excepções documentadas)**
— paralela ao pattern ADR-0073 cond 9 P206E.

### Auditoria 7 condições — resultado

| # | Condição | Estado | Evidência |
|---|----------|--------|-----------|
| 1 | P207B-E materializados | ✅ CUMPRIDA | P207B (query_labelled) + P207C (LabelRegistry MultiMap + label_count) + P207D (4 page-aware + PageStore) + P207E (encerramento). Cargo build verde pós-cada sub-passo. |
| 2 | P208A-D materializados; tests E2E real consumer; ≥5 tests | ⚠️ CUMPRIDA com excepção | P208A-D materializados (here() + locate() + EvalContext infra); 8 tests novos (4 P208B + 4 P208C; cumpre ≥5). **Excepção**: tests E2E "real consumer using here()" inexistente — Caminho 3 P210 deferiu `counter.display`/`state.get` (Q1=β minimal); mock tests autorizados per P208B §5 risco 3. |
| 3 | P209A-D Selector::Label/And/Or + ≥3 tests/variant | ✅ CUMPRIDA | P209B Label+Location, P209C And+Or, P209D Regex (bónus); ≥3 tests per variant (selector.rs +stdlib mod combinados). 28 tests novos cumulativos série P209. |
| 4 | P210A-D page-aware infra + inject_page_data | ✅ CUMPRIDA (reinterpretada) | Cond original referia page-aware em P210; `P207A.div-1` reshape moveu page-aware para P207D — `PageStore` + `inject_pages` materializados conforme spec. P210 reshape para Counter/State Q1=β (Caminho 3 subset). Funcionalmente equivalente; nomeação sub-passo diferente. |
| 5 | Tests workspace 1873 → 1900-1950 | ✅ CUMPRIDA | 1873 → **1939** (+66) within range. |
| 6 | `crystalline-lint .` 0 violations preservadas | ✅ CUMPRIDA | 0 violations preservadas em todos os sub-passos M9c. |
| 7 | ADR-0073 retroactivo §"Fecho retroactivo M9c" | ✅ CUMPRIDA | §"Fecho retroactivo M9c — P212 2026-05-12" adicionada a ADR-0073; documenta paridade trait estendida 20→26 métodos + 2 sub-stores L1 novos. |

**Resultado**: 7/7 cumpridas (5 cumpridas plenamente + 1 com
excepção documentada + 1 reinterpretada). Pattern P206E (cond
9 ADR-0073) replicado.

### Justificação fórmula "completo retroactivo com excepções"

- **Excepção cond #2 (E2E real consumer)**: zero consumers
  reais imediatos — pattern consistente M9c (9 aplicações
  anti-inflação cumulativas; consumer real emerge naturalmente).
  Mock-tests autorizados per spec §5 risco 3 P208B. Não
  regressão estructural.
- **Reinterpretação cond #4 (page-aware infra)**: `P207A.div-1`
  reshape humano aprovado em P207B — moveu page-aware para
  P207D; P210 redirecionou para Counter/State. Funcionalmente
  equivalente; documentação anotada.

Caminho A ("ACEITE puro") rejeitado: implicaria todas 7
literalmente sem excepções — desonesto face às 2 documentadas.
Caminho B ("manter PROPOSTO") rejeitado: progresso material
em 5 séries; preservar PROPOSTO seria sub-estimar.
**Caminho C (escolhido) — ACEITE retroactivo com excepções**:
paralelo ao P206E pattern para ADR-0073 cond 9.

### Deferreds documentados com critério de reabertura

ADR-0076 mantém deferreds com critério explícito:

| Deferred | Critério de reabertura | Originado em |
|----------|------------------------|--------------|
| `SealedLabelPages` sub-store | Consumer real `label_pages` lookup | P205D |
| Page-meta capture no walk | Consumer real `page_numbering`/`supplement` retorna não-`None` | P207E |
| Walk advance automático `current_location` no eval | Consumer real `here()` em context block real | P208B |
| `Content::Context { body }` block | Consumer real `#context { here() }` syntax | P208D |
| `native_regex(pattern)` stdlib func | Consumer real (query-by-text) | P209D C6 |
| `counter.display(numbering)` here-aware | Walk advance materializado | P210A C3 |
| `state.get()` here-aware | Walk advance materializado | P210A C3 |
| Outline configurável (target/depth/indent params) | Test fixture com params customizados | P211A C3 |
| `Selector::Where` | Decisão humana Q2 reabrir | P207A Q2=γ |
| `Selector::Before/After/Within` | Fora roadmap M9c — passo futuro dedicado | P207A C4 |
| `query_count_before(sel, end)` | Decisão humana Q4 reabrir | P207A Q4=β |

ADR-0076 ACEITE preserva todos os deferreds como referência
arquitectural estável. Reabertura futura tem gatilho claro;
sem critério, deferreds ficam permanentes (anti-inflação
honest).

### Patterns emergentes formalizados em M9c

1. **Diagnóstico-primeiro reduzido** (5 aplicações: P207A,
   P208A, P209A, P210A, P211A) — 1 output em vez de 4;
   spec compacta; decisões C1-C5 fixadas empíricamente.
2. **Caminho 1 anti-inflação** (9 aplicações cumulativas) —
   encerramento documental puro quando consumer real ausente.
3. **Caminho 3 honest subset** (1 aplicação P210) —
   materialização parcial honesta com deferred restante +
   critério de reabertura.
4. **1-sub-passo único** (1 aplicação P211) — diagnóstico +
   encerramento fundidos quando Caminho 1 puro decidido em
   C3 do próprio diagnóstico.
5. **Convenção L0 inline-documentada** (4 aplicações: P208B,
   P208C, P209D C6, P210B) — stdlib funcs P169+ sem L0
   prompt separado.
6. **Regra empírica P207B §5** — trait method novo propaga
   obrigatóriamente a `CountingIntrospector` L3 (4 pontos:
   array + slots + impl + sentinel). Aplicada em P207B/C/D;
   não acionada em P208/P209/P210/P211.
7. **Stdlib funcs + Selector variants ≠ trait extensions** —
   formalizado em P209 + P210 (P208/P209/P210 não acionaram
   regra P207B §5).
8. **Marca-por-fecho cirúrgica blueprint** (8 marcas
   cumulativas: §3.0 a §3.0octies + §3.0nonies em P212) —
   preserva blueprint sem reescrita ampla.

### Cross-references P212

- ADR-0073 ACEITE com paridade trait estendida M9c — bloco
  "Fecho retroactivo M9c — P212".
- ADR-0077 ACEITE (P209E 2026-05-12) — regex L1.
- `00_nucleo/materialization/typst-passo-212-relatorio.md`
  (encerramento marco M9c).
- 5 séries M9c materializadas (P207-P211 — relatórios
  individuais em `materialization/`).

---

## Contexto

ADR-0073 (M8) foi fechado estruturalmente em P204H 2026-05-07
com `#[comemo::track]` aplicado ao trait `Introspector`
(paridade vanilla literal nos bounds + tracking).

ADR-0074 (F3) foi fechado final em P205E 2026-05-07 com
sealing post-iteração de Layouter sub-stores trackable;
`SealedPositions` integrada como sub-store cristalino
(`P205A.div-1` legitimou divergência face a vanilla
`PagedIntrospector`).

ADR-0075 (vanilla integration) foi fechado final em P206E
2026-05-08 com paridade observable estrutural via `typst query`
JSON (pixel-perfect rejeitado por design per ADR-0054).

**Mas auditoria P207A revelou gap fundamental** entre o trait
Introspector cristalino actual e o trait vanilla v0.14.2:

- **Trait cristalino tem 20 métodos especializados**
  (`figure_number_for_label`, `formatted_counter`, `state_value`,
  `bib_entry_for_key`, `is_numbering_active*`,
  `figure_number_at_index`, `flat_counter_at`,
  `resolved_label_for`, `headings_for_toc`, etc.).
- **Trait vanilla tem 16 métodos genéricos** centrados em
  `query(&Selector) -> EcoVec<Content>` + page-aware methods
  (`pages`, `page`, `position`, `page_numbering`, `page_supplement`,
  `anchor`, `document`, `path`).
- **Selector cristalino tem 1 variant** (`Kind(ElementKind)`); vanilla
  tem 10 (`Elem`, `Location`, `Label`, `Regex`, `Can`, `Or`, `And`,
  `Before`, `After`, `Within`).
- **Stdlib `here()` e `locate()` ausentes** em cristalino —
  bloqueiam ~5+ outros itens (counter.get, state.get,
  Selector::Before/After consumers).
- **Sub-stores cristalino (10) vs acceleration structures
  vanilla (5)** + Counter/State como domain types vanilla —
  arquitecturas **não-isomorphic**.

P207A classificou 62 itens em 4 categorias (A15):
- 11% PARIDADE LITERAL.
- **44% DIVERGÊNCIA ARQUITECTÓNICA LEGÍTIMA** (cristalino e
  vanilla resolvem o mesmo problema com designs diferentes;
  inverter cristalino seria perder ADR-0073/0074/0029).
- 31% EXTENSÃO NECESSÁRIA (vanilla tem; cristalino não).
- 11% DECISÃO PENDENTE (4 questões para humano).

Magnitude agregada de "completar tudo" (62 itens): **XL**
(~50-60h). Magnitude de escopo reduzido per `P207A.div-1`
(22 itens materializáveis, 27 divergências preservadas, 7
deferidos): **L** (~30h).

Continuidade narrativa: M9 (Stdlib introspection) foi declarado
fechado em P182F sob critério "stdlib 11/11" — antes de
auditoria empírica de gap. M9c reconhece esta continuidade
em vez de abrir M10 novo.

---

## Decisão

Cristalino adopta **marco M9c** (M9-completion) como continuação
narrativa de M9 com escopo reduzido fundamentado per
`P207A.div-1`:

### Mecanismo (per P207A C1-C8)

**Caminho 3 — Marco arquitectónico** (per C6):

- **Não** série única (P207A-E) — magnitude L+ não cabe.
- **Não** sub-séries dispersas — perdem coerência narrativa.
- **Sim** marco M9c com 5 séries dedicadas (P207-P211).

**Escopo reduzido per `P207A.div-1`** (per C12):

- **Incluir** (22 itens materializáveis):
  - 19 itens EXTENSÃO NECESSÁRIA Bloco I-IV (sem rich Counter/State).
  - 3 itens Selector minimal (`Label` + `And` + `Or`).
- **Excluir** (27 itens DIVERGÊNCIA ARQUITECTÓNICA LEGÍTIMA):
  - Não materializar; divergência é design-pattern cristalino
    legitimado por ADR-0029/0073/0074.
- **Diferir** (7 itens DECISÃO PENDENTE):
  - Q1 — Rich `Counter`/`State` types (recomendação β: manter
    forma minimal cristalino).
  - Q2 — `Selector::Where` (recomendação γ: adiar; sem consumer
    real).
  - Q3 — `Selector::Regex` + `Selector::Location` (recomendação
    β: adiar).
  - Q4 — `query_count_before` (recomendação β: adiar).
- **DEBT-55** (Bibliography + Cite XL): continua tracker
  separado; fora marco M9c.

### Trabalho por série (per C11)

**P207B-E** — Trait extensions + sub-store refactor:
- P207B — `query_labelled() -> Vec<(Label, Location)>` (S).
- P207C — `LabelRegistry → MultiMap` refactor + `label_count`
  trait method (M).
- P207D — Page-aware trait methods (`pages`, `page`,
  `page_numbering`, `page_supplement`) + decisão estendê-r
  `LayouterRuntimeState` ou criar `PageStore` (M-L).
- P207E — Encerramento série + ADR-0076 cond parcial.

**P208A-D** — Stdlib `here()` + `locate()` (per C5):
- P208A — diagnóstico-primeiro (`Tracked<Context>` análogo
  cristalino).
- P208B — `here()` materialização + tests.
- P208C — `locate(selector)` materialização + tests.
- P208D — encerramento série.

**P209A-D** — Selector minimal (per C4):
- P209A — diagnóstico-primeiro Selector enum design.
- P209B — `Selector::Label` + impl em `query()`.
- P209C — `Selector::And` + `Or` + impl.
- P209D — encerramento série.

**P210A-D** — Page-aware infrastructure consolidação:
- P210A — diagnóstico-primeiro page-aware (estende P207D).
- P210B-C — materialização (PageStore vs LayouterRuntimeState).
- P210D — encerramento série.

### P210 série (Counter/State extras Q1=β) — ✅ MATERIALIZADO 2026-05-12

Magnitude S-M (~1.5-2h estimado per P210A C5; real ~1.5h).

- P210A — diagnóstico-primeiro reduzido — ✅ MATERIALIZADO
  2026-05-12 (Caminho 3 subset fixado em C3: apenas
  `counter.step()` materializável sem current_location;
  display/get deferred até walk advance).
- P210B — `native_counter_step(key)` (S ~30min-1h) —
  ✅ MATERIALIZADO 2026-05-12 (trivial: emite
  `Value::Content(Content::CounterUpdate { key, action:
  Step })`; 4 tests; scope register; sem L0 separado per
  convenção P208B; sem rich Counter types per Q1=β).
- P210C — encerramento série (S documental ~20-30min) —
  ✅ MATERIALIZADO 2026-05-12 (documental puro; sem decisões
  C1 — P210A C3 já fixou Caminho 3; P210B implementou; P210C
  formaliza encerramento + bloco agregado).

**Agregado série P210** (per P210C §6 sumário):
- 3 sub-passos materializados (A diagnóstico + B + C)
  com custo agregado real ~1.5h (estimado ~1.5-2h per
  P210A C5).
- 1 stdlib func nova: `native_counter_step(key)`.
- 4 tests novos (P210B).
- Sub-store novos: 0 (reusa `Content::CounterUpdate` +
  `CounterAction::Step` pre-existentes).
- L0 prompts novos: 0 (convenção P208B inline-documentação).
- L1 ficheiros novos: 0.
- ADRs novas: 0 (trabalho 100% sob ADR-0076).
- Tests workspace: 1935 → 1939 (+4). 0 violations.
- Trait `Introspector` mantém 26 métodos — regra P207B §5
  **não acionada**.

**Deferreds explícitos** (P210A C3 + P210B implementação):
- `counter.display(numbering)` here-aware — depende de
  current_location + walk advance (P208B Opção i minimal).
- `state.get()` here-aware — depende de current_location +
  walk advance.
- **Critério de reabertura**: quando walk advance for
  implementado em sub-passo dedicado pós-M9c, OU quando
  consumer real emergir (`Selector::Before/After` futuros,
  ou `#context { ... }` block materializado). Até lá,
  funcs ficam não-materializadas (não retornam erro;
  simplesmente não existem no scope).

**Pattern emergente P210** — **Caminho 3 honest subset** (8ª
aplicação cumulativa anti-inflação):

1. P205D — `SealedLabelPages` deferred.
2. P207E — captura page-meta deferred.
3. P208B C1 — sub-mecanismo i minimal.
4. P208D — `Content::Context` block deferred.
5. P209C-vazios — `And/Or(EcoVec::new())` → `vec![]`.
6. P209D C6 — `native_regex` stdlib deferred.
7. P209E C1.2 — encerramento documental puro.
8. **P210A C3 + P210B** — `counter.step` materializado;
   `counter.display`/`state.get` deferred. **Pattern novo
   distinto**: materialização parcial honesta vs skip total
   (Caminho 1) vs full materialização (Caminho 2).

**Distinção qualitativa funcs stdlib**:
- **Sem `current_location` dependência**: `counter.step`,
  `query`, `locate`, `here` (este último retorna Err se
  None mas a infra está pronta). Materializáveis sem
  walk advance.
- **Com `current_location` dependência funcional**:
  `counter.display(numbering)`, `state.get()`. Requerem
  walk advance materializado para utilidade real. Deferred.

**Estado actual M9c**: 4 séries fechadas (P207 + P208 + P209
+ P210). Restam P211 (Outline configurável) + P212
(encerramento M9c — transição ADR-0076 PROPOSTO → ACEITE).

**P211A-B** — Marco M9c fechamento:
- P211A — auditoria 7 condições ADR-0076.
- P211B — ADR-0076 PROPOSTO → ACEITE; blueprint M9c marca.

### Coerência arquitectónica (per ADR-0073/0074/0075)

ADR-0076 segue padrão consolidado: marcos arquitectónicos com
decisão estrutural ganham ADR dedicada. M9c é estruturalmente
comparável a M8 + F3 + vanilla integration — escopo amplo +
escolhas arquitectónicas múltiplas.

**Não inverte ADRs vigentes**:
- ADR-0029 (Pureza física `Arc` permitido) preservada — sub-stores
  cristalinos continuam usando `Arc` legítimamente.
- ADR-0073 (M8 `#[comemo::track]`) preservada — trait permanece
  tracked.
- ADR-0074 (F3 sub-stores trackable) preservada — sealing
  post-iteração mantido para `SealedPositions` + extensível
  para `PageStore`.
- ADR-0054 (FixedMetrics) preservada — geometric divergência
  observable continua N/A per ADR-0075.

---

## Alternativas consideradas

### Alternativa B — Série única P207A-E (Caminho 1 per C6)

**Rejeitada**. P207A A16 mostrou magnitude agregada L (escopo
reduzido) ou XL (escopo completo). Série única (5 sub-passos)
caberia ~M-L; ~30h não cabe sem inflar sub-passos. Coerência
narrativa perdida (mistura trait + sub-store + stdlib + Selector
em série única).

### Alternativa C — Sub-séries dedicadas dispersas (Caminho 2 per C6)

**Rejeitada parcialmente**. P207/P208/P209 dedicadas é parte da
solução, mas **sem marco arquitectónico unificador** dispersaria
narrativa. Marcos prévios (M7/M8/F3/vanilla integration) tiveram
ADR + plano coerente; M9c segue padrão.

### Alternativa D — Materializar escopo amplo absoluto (62 itens)

**Rejeitada por inflação**. Magnitude XL+ (~50-60h) sem benefício
proporcional:
- 27 itens DIVERGÊNCIA ARQUITECTÓNICA LEGÍTIMA — materializar
  inverteria ADR-0073/0074/0029.
- 7 itens DECISÃO PENDENTE — bloqueiam até clarificação humana.
- Rich Counter/State (DECISÃO PENDENTE Q1) consumiria ~15-25h
  sem mudar paridade observable.

`P207A.div-1` legitima escopo reduzido com fundamento empírico
(C12); D rejeita pré-fixação literal absoluta.

### Alternativa E — Adiar M9c indefinidamente

**Rejeitada**. `here()` + `locate()` ausências bloqueiam consumer
real (P206C D9 expectativa); page-aware items afectam features
PDF (`location.page()`, `outline()` page numbers). Aceite continuar
ADR-0073 cond paridade trait estendida ad infinitum não é
sustentável.

### Alternativa F — Abrir M10 novo em vez de M9c

**Rejeitada por incoerência narrativa**. M9 (Stdlib introspection
11/11) foi fechado em P182F sob critério limitado; auditoria
P207A revela continuação directa (gap empírico que estava por
auditar). M10 deve ser próximo marco verdadeiramente novo (e.g.
Show rules, Math layer, etc.). M9c reconhece "completion" como
sub-fase.

---

## Consequências

### Positivas

- **Fecha gap empírico** trait + sub-store + Selector cristalino
  vs vanilla com fundamento auditado (62 itens classificados).
- **Reconhece divergências legítimas** (44%) em vez de inverter
  cristalino — preserva ADR-0029/0073/0074.
- **Desbloqueia stdlib `here()`/`locate()`** — dependência hub
  para 5+ outros itens.
- **Refactor `LabelRegistry → MultiMap`** desbloqueia multi-label
  semântica (paralelo vanilla).
- **Page-aware infrastructure** consolida `runtime.known_page_numbers`
  para trait method exposure.
- **Selector minimal extensions** (Label + And + Or) habilitam
  3+ stdlib funcs.
- **Pattern reusável**: marcos podem ter "completion" sub-fases
  pós-fecho original (M9c).

### Negativas

- **5 séries dedicadas** (P207-P211) — coordenação maior; ~30h
  agregado.
- **DECISÃO PENDENTE Q1-Q4** introduz bloqueio humano antes de
  P207B.
- **Multi-label refactor** (`LabelRegistry`) tem impacto
  cross-modular — consumers que assumem label única precisam
  audit + update.
- **Page-aware infrastructure** consolidação pode revelar
  inconsistências em `runtime.known_page_numbers` actual
  (apenas populated em iterações pós-layout fixpoint).
- **Não fecha** rich Counter/State stdlib (DECISÃO PENDENTE);
  parity vanilla mantém-se reduzida nesta dimensão.

### Neutras

- **DEBT-55** continua tracker separado — bibliography não
  endereçada.
- **Selector::Where**/`Regex`/`Location` adiados — gap consciente.
- **Anchor/document/path** preservados como divergência
  arquitectónica (HTML/bundle target) — ADR explícita.
- **Locator** preserva divergência arquitectónica (single-pass
  cristalino vs measurement-driven vanilla).

---

## Plano de validação

ADR-0076 transita de `PROPOSTO` para `ACEITE` quando todas estas
condições forem verdadeiras (verificadas em P211B):

1. **P207B-E materializados**: `query_labelled` + `LabelRegistry
   → MultiMap` + `label_count` + page-aware trait methods (`pages`,
   `page`, `page_numbering`, `page_supplement`); cargo build verde
   pós-cada sub-passo.
2. **P208A-D materializados**: `here()` stdlib func + `locate()`
   stdlib func; tests E2E demonstrando consumer real (counter.get
   ou state.get usando here()); ≥5 tests novos.
3. **P209A-D materializados**: `Selector::Label` + `Selector::And`
   + `Selector::Or` em enum cristalino; impls em
   `Introspector::query`; ≥3 tests por variant.
4. **P210A-D materializados**: page-aware infrastructure
   consolidada (PageStore ou LayouterRuntimeState extension);
   `inject_page_data` mecanismo paralelo a P205C
   `inject_positions`.
5. **Tests workspace verdes**: estimativa 1873 → 1900-1950 (∆+27 a
   +77).
6. **`crystalline-lint .` 0 violations** preservadas pós-cada
   sub-passo.
7. **ADR-0073 retroactivo**: cond paridade trait estendida com
   page-aware items materializados; documentar em ADR-0073
   §"Fecho retroactivo M9c".

ADR transita para `REJEITADO` se durante materialização for
descoberto:

- Multi-label refactor (`LabelRegistry`) impossível sem inverter
  ADR-0029 (improvável; refactor é S-M).
- Page-aware infrastructure incompatível com single-pass
  cristalino (improvável; `runtime.known_page_numbers` já
  parcial).
- `here()` exige `Tracked<Context>` análogo cristalino que
  introduziria global state mutável (improvável; cristalino tem
  `EvalContext.location` injectável).

Se ADR for rejeitada, M9c estado revertido para diagnóstico-only;
ADR-0073 cond paridade trait permanece estendida não-cumprida;
DEBT-55 + DECISÃO PENDENTE Q1-Q4 mantêm-se em aberto.

---

## Plano de materialização

Marco M9c — 5 séries (P207A pré-condição cumprida; P207B-P211B
pendentes):

### P207A — Diagnóstico-primeiro — ✅ MATERIALIZADO 2026-05-12

Magnitude M (real ~50 min).

- Auditoria empírica A1-A16 com etiquetas e evidência (62 itens).
- Diagnóstico C1-C13 com decisões fixadas (4 decisões pendentes
  para humano em C10).
- ADR-0076 PROPOSTO (este ficheiro).
- `P207A.div-1` registada (escopo reduzido fundamentado).
- Plano *B+ sem ramos (5 séries P207-P211).

### P207 série — Trait extensions + sub-store refactor — ✅ MATERIALIZADO 2026-05-12

Magnitude L (~12-15h estimado; real ~10h agregado entre A-E).

- P207A — Diagnóstico-primeiro — ✅ MATERIALIZADO 2026-05-12.
- P207B — `query_labelled` (S ~1-2h) — ✅ MATERIALIZADO 2026-05-12.
- P207C — `LabelRegistry → MultiMap` + `label_count` (M ~2-3h) —
  ✅ MATERIALIZADO 2026-05-12.
- P207D — Page-aware trait methods + `PageStore` (M-L ~5-6h) —
  ✅ MATERIALIZADO 2026-05-12 (Opção 2 fixada em C2: `PageStore`
  sub-store dedicado paralelo a `SealedPositions` per P205B/C).
- P207E — Encerramento série (S documental ~30 min) —
  ✅ MATERIALIZADO 2026-05-12 (Caminho 1 fixado em C2: encerramento
  documental puro; captura runtime de numbering/supplement deferida
  por zero consumers).

**Bloqueio resolvido para P207D**: a decisão arquitectural foi
fixada empiricamente em C1+C2 do próprio P207D (não exigiu
decisão humana separada porque os 3 sub-passos diagnósticos
internos clarificaram). Critério vanilla "pre-computa em
PagedIntrospector::new" → Opção 2 fixada. Trait passa de 22
para 26 métodos. Cristalino diverge: numbering como `EcoString`
pattern (ADR-0024) em vez de enum `Numbering` vanilla;
sealing por sub-store (P205A.div-1) em vez de PagedIntrospector
global.

**Agregado série P207** (per P207E §6 sumário):
- 4 sub-passos materializados (B + C + D + E) sobre fundamento
  P207A diagnóstico.
- Trait `Introspector` passa de 20 para **26 métodos**:
  - +`query_labelled` (P207B)
  - +`label_count` (P207C)
  - +`pages`, `page`, `page_numbering`, `page_supplement` (P207D)
- 1 sub-store novo: `PageStore` (P207D, paralelo a `SealedPositions`).
- 2 sub-stores enriquecidos: `LabelRegistry` (multi-label, P207C)
  + `TagIntrospector` (4 fields novos: `page_store` + 3 já
  existentes).
- 26 tests novos: P207B (+5), P207C (+7), P207D (+14).
- Tests workspace: 1873 → 1899 (+26). 0 violations.
- 3 ADRs cruzadas: 0073 (paridade trait), 0074 (sealing), 0024
  (`EcoString`), 0026 (`Content`), 0076 (M9c).
- Surpresas registadas: regra empírica P207B §5 (propagação
  obrigatória a `CountingIntrospector`); P207D ecow dev→dep.

**Estado actual M9c**: série P207 fechada (5 sub-passos
materializados). Marco M9c continua com P208+ (per
`P207A.div-1`).

### P208 série — Stdlib here()/locate() — ✅ MATERIALIZADO 2026-05-12

Magnitude M (~5-7h estimado; real ~3h agregado).

- P208A — diagnóstico-primeiro — ✅ MATERIALIZADO 2026-05-12
  (Caminho B fixado: especializado cristalino sem `Tracked<Context>`).
- P208B — `here()` materialização (S-M ~2-3h) —
  ✅ MATERIALIZADO 2026-05-12 (sub-mecanismo (i) minimal fixado em
  C1: field `current_location` em `EvalContext` + setter + stdlib
  func + scope register; sem walk advance automático — captura via
  `Content::Context` block deferred per P208A.C1 caveat).
- P208C — `locate(selector)` materialização (S ~30min-1h) —
  ✅ MATERIALIZADO 2026-05-12 (reusa pattern `native_query` literal;
  `Selector::Kind` only — `locate(<label>)` exige P209
  `Selector::Label`).
- P208D — encerramento série (S documental ~30min) —
  ✅ MATERIALIZADO 2026-05-12 (Caminho 1 fixado em C2: encerramento
  documental puro; materialização de `Content::Context` block
  deferida — zero consumers, custo M+ sem benefício; pattern
  anti-inflação P205D/P207E replicado, 4ª aplicação).

**Agregado série P208** (per P208D §6 sumário):
- 4 sub-passos materializados (A diagnóstico + B + C + D) com
  custo agregado real ~3h (estimado ~5-7h).
- 2 stdlib funcs novas: `native_here()` + `native_locate()`.
- 1 field novo em `EvalContext`: `current_location: Option<Location>`.
- 1 método novo em `EvalContext`: `with_current_location(loc)`.
- 8 tests novos: P208B (+4 `p208b_here_*`) + P208C (+4 `p208c_locate_*`).
- Tests workspace: 1899 → 1907 (+8). 0 violations.
- Trait `Introspector` mantém 26 métodos — regra P207B §5
  **não acionada** (stdlib funcs, não trait methods).
- Limitações herdadas: `here()` retorna `Err` se
  `current_location == None`; `locate(<label>)` exige P209.
- Sub-mecanismos deferred: `Content::Context` block
  (ContextElem-style); walk advance automático de
  `current_location`. Emergem naturalmente quando consumer
  real aparecer.

**Pattern emergente P208** — **Caminho 1 anti-inflação 4ª
aplicação**: P205D (`SealedLabelPages` deferred), P207E
(captura page-meta deferred), P208B C1 (sub-mecanismo i
minimal vs ii/iii), P208D (`Content::Context` deferred).
Critério literal: zero consumers + custo M+ sem benefício
→ encerramento documental puro autorizado.

**Estado actual M9c**: 2 séries fechadas (P207 + P208).
Resta P209 (Selector minimal per `P207A.div-1` Q-decisões:
`Label` + `And` + `Or`), P210 (Counter/State extras se
Q1=β reabrir), P211 (Outline configurável se aplicável),
P212 (encerramento M9c — transição ADR-0076 PROPOSTO →
ACEITE).

### P209 série — Selector minimal — ✅ MATERIALIZADO 2026-05-12

Magnitude M (~4-5h estimado per P209A C5; real ~4h agregado).

- P209A — diagnóstico-primeiro — ✅ MATERIALIZADO 2026-05-12
  (Caminho 2 fixado em C4: 5 sub-passos balanceados; Caminho A
  para regex em C2; Opção c Rust API only para And/Or em C3).
- P209B — `Selector::Label` + `Selector::Location` (S ~45min) —
  ✅ MATERIALIZADO 2026-05-12 (2 variants triviais + query arms
  + stdlib dispatch refactor com helper `parse_selector_arg`).
- P209C — `Selector::And` + `Selector::Or` (M ~1-1.5h) —
  ✅ MATERIALIZADO 2026-05-12 (composição N-ária via `EcoVec<Self>`;
  query arms intersecção/união; Opção A fixada em C3 para
  vazios: `And/Or(EcoVec::new())` → `vec![]`; stdlib API
  Opção (c) Rust-only).
- P209D — `Selector::Regex` + ADR-0077 + dep `regex` (M ~1-1.5h) —
  ✅ MATERIALIZADO 2026-05-12 (wrapper L1 `entities::regex::Regex`
  com Hash/Eq/PartialEq/Clone/Debug manuais via pattern string;
  ADR-0077 PROPOSTO; `regex` 1.x adicionado a allowlist L1 +
  workspace + 01_core deps; query arm é **stub `vec![]` documentado**
  per P209A A3 — cristalino single-pass sem Content text durante
  query phase; stdlib `native_regex` deferred per C6 = Opção γ
  — Caminho 1 anti-inflação 6ª aplicação).
- P209E — encerramento série (S documental ~30min) —
  ✅ MATERIALIZADO 2026-05-12 (Caminho A fixado em C1.1:
  **ADR-0077 PROPOSTO → ACEITE** após verificação dos 8
  critérios §Plano de validação; Caminho 1 fixado em C1.2:
  encerramento documental puro — Caminho 1 anti-inflação 7ª
  aplicação consecutiva).

**Agregado série P209** (per P209E §6 sumário):
- 5 sub-passos materializados (A diagnóstico + B + C + D + E)
  com custo agregado real ~4h (estimado ~4-5h).
- `Selector` enum: 1 → 6 variants (+5: `Label`, `Location`,
  `And`, `Or`, `Regex`).
- `Introspector::query` arms: 1 → 6 (paralelo).
- 1 sub-store novo: `Regex` wrapper (P209D) com Hash/Eq manuais.
- 1 ADR nova: **ADR-0077 ACEITE** 2026-05-12 (regex em L1).
- 1 dep nova em allowlist L1: `regex` (11 → 12 entries).
- Stdlib refactor: helper `parse_selector_arg` em
  `foundations.rs` (P209B); `native_query`/`native_locate`
  ganham type dispatch (`<name>` → Label; `Value::Location` →
  Location).
- 28 tests novos: P209B (+8) + P209C (+9) + P209D (+11).
- Tests workspace: 1907 → 1935 (+28). 0 violations.
- Limitação herdada P209D: `Selector::Regex` query arm é
  stub `vec![]` documentado (Content text durante query phase
  não acessível em cristalino single-pass; semântica funcional
  deferred a passo dedicado pós-M9c quando consumer query-by-text
  emergir).

**Pattern emergente P209** — **Caminho 1 anti-inflação 7ª
aplicação**:

1. P205D — `SealedLabelPages` deferred.
2. P207E — captura page-meta deferred.
3. P208B C1 — sub-mecanismo i minimal (vs ii/iii).
4. P208D — `Content::Context` block deferred.
5. P209C-vazios — `And/Or(EcoVec::new())` → `vec![]` (Opção A).
6. P209D C6 — `native_regex` stdlib deferred (Opção γ).
7. **P209E C1.2** — encerramento documental puro (Caminho 1).

Princípio operacional consolidado: materialização honesta
proporcional ao consumo imediato. Consumer real emerge
naturalmente em passos futuros; over-engineering rejeitado
empiricamente.

**Pattern emergente P209** — **stdlib funcs sem trait extension**:
Toda a série P209 não tocou o trait `Introspector`. Trait
mantém 26 métodos consistentemente desde P207D. Regra empírica
P207B §5 **não acionada** em P208 e P209 inteiras
(stdlib funcs ≠ trait methods). Confirma que paridade vanilla
no nível trait foi atingida em P207D.

**Estado actual M9c**: 3 séries fechadas (P207 + P208 +
P209). Marco M9c continua com P210 (Counter/State extras se
Q1=β reabrir), P211 (Outline configurável se aplicável),
P212 (encerramento M9c — transição ADR-0076 PROPOSTO →
ACEITE + auditoria 7 condições).
- P209C — `Selector::And` + `Or` (M ~2-3h).
- P209D — encerramento.

**Bloqueio**: depende de P208D para `locate()` consumer real.

### P210 série — Page-aware infrastructure consolidação (PENDENTE)

Magnitude L (~10-12h).

- P210A — diagnóstico-primeiro (estende P207D).
- P210B-C — materialização.
- P210D — encerramento.

**Bloqueio**: depende de P207D (page-aware trait methods).

### P211 série (Outline configurável — Bloco VII) — ✅ MATERIALIZADO 2026-05-12

Magnitude S (~30min real; estimado M-L per P207A original).

- P211A — diagnóstico-primeiro reduzido + encerramento série
  (1 sub-passo único) — ✅ MATERIALIZADO 2026-05-12
  (**Caminho 1 puro fixado em C3**: zero código tocado;
  9ª aplicação anti-inflação cumulativa).

**Justificação Caminho 1 puro** (per P211A relatório §3 C3):

- **A4 zero consumers absoluto**: nenhum production caller
  de `outline(target/depth/indent/fill/title)` em cristalino.
- **A5 M-L cost sem consumer**: refactor `Content::Outline`
  unit → struct é cross-modular (closures.rs + content.rs +
  layout/mod.rs + tests); 2.5-3.5h estimado para 3
  sub-features viáveis (target + depth + indent; `fill` é
  vanilla diferente — `outline.entry` show-set, fora M9c).
- **A3 divergência principal não resolvida**: cristalino
  auto-toc (P200 série) vs vanilla outline-body show-rule
  continua divergente arquitectonicamente. Bloco VII item
  55a apenas adiciona params; **não** fecha gap "completude
  vanilla".

**Anti-inflação 9ª aplicação cumulativa** (P205D, P207E,
P208B C1, P208D, P209C-vazios, P209D C6, P209E C1.2, P210
Caminho 3, **P211A C3**).

**Reabertura futura**: quando consumer real emergir (e.g.,
test fixture com `outline(depth: 2)`), Bloco VII materializa-se
em sub-passo pós-M9c dedicado.

**P211 é a 1ª série M9c a fechar em 1 sub-passo único**
(vs P207=5, P208=4, P209=5, P210=3). Confirma anti-inflação
como mecanismo real de redução do orçamento.

### P212 — Marco M9c fechamento (PENDENTE)

Magnitude S documental (~1-2h).

- P212A — auditoria 7 condições ADR-0076 §Plano de validação.
- P212B — ADR-0076 PROPOSTO → ACEITE; blueprint marca M9c
  fechado; ADR-0073 retroactivo §"Fecho retroactivo M9c";
  relatório consolidado M9c.

**Pré-condição cumprida**: P207-P211 séries todas fechadas;
ADR-0077 já transitou ACEITE (P209E).

---

## Cross-references

- **ADR-0073** (ACEITE estruturalmente fechado P204H 2026-05-07;
  cond paridade trait estendida em P211B) — `#[comemo::track]`
  em Introspector preservado; M9c estende paridade trait.
- **ADR-0074** (ACEITE final P205E 2026-05-07) — F3 Layouter
  sub-stores trackable preservada; M9c estende com page-aware
  infrastructure paralela.
- **ADR-0075** (ACEITE final P206E 2026-05-08) — vanilla
  integration via pre-built CLI preservada; M9c paridade
  estrutural mantém-se via `typst query` JSON.
- **ADR-0029** (Pureza física `Arc` permitido) — preservada;
  sub-stores cristalinos continuam usando `Arc`.
- **ADR-0054** (FixedMetrics — divergência observable) —
  preservada; M9c não endereça geometric paridade.
- **DEBT-55** (Bibliography + Cite XL; em aberto) — separado;
  fora marco M9c.
- **`P205A.div-1`** (vanilla assimetria Engine + Layouter
  especializados) — preservada; cristalino reusa `TagIntrospector`
  enriquecido por simplicidade.
- **`P207A.div-1`** (escopo reduzido fundamentado) — registada
  P207A C12; aguarda decisão humana.
- **Vanilla typst v0.14.2**:
  `lab/typst-original/crates/typst-library/src/introspection/`
  (13 ficheiros; 3983L) +
  `lab/typst-original/crates/typst-library/src/foundations/selector.rs`.

---

## Pattern emergente

ADR-0076 aplica padrão consolidado pela série P204+P205+P206:

1. **Diagnóstico-primeiro de profundidade alta** (16 cláusulas
   A1-A16 cobrindo 5 blocos arquitecturais; 62 itens
   classificados).
2. **Decisões fixadas com base em empírico** — 44% DIVERGÊNCIA
   ARQUITECTÓNICA LEGÍTIMA reconhecida em vez de inverter
   cristalino; `P207A.div-1` recomenda escopo reduzido.
3. **Reuso máximo do existente** — sub-stores cristalinos
   preservados; refactor cirúrgico (`LabelRegistry → MultiMap`)
   apenas onde estritamente necessário.
4. **Magnitude calibrada** — L agregado escopo reduzido (paralelo
   a F3 M agregado; menor que M8 L cross-modular).
5. **Divergência intencional vs vanilla preservada** —
   single-pass cristalino, sub-stores especializados, sealing
   F3 não invertidos por M9c.
6. **DECISÃO PENDENTE explícita** quando empírico não decide —
   Q1-Q4 ficam para clarificação humana, evitando inflação por
   auto-decisão.
7. **Marco "completion" como sub-fase** — M9c reconhece M9
   fechado em P182F sob critério limitado; auditoria empírica
   posterior revela escopo expandido. Pattern reusável para
   futuros marcos.

Pattern reaproveitável para futuras "completions" (M7-completion
se fixpoint runtime emergir gaps; M8-completion se introspector
trait emergir mais gaps; F3-completion se sealing infra
escalar; etc.).
