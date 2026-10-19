# P207A — Diagnóstico: cláusulas de decisão C1–C13

**Data**: 2026-05-12.
**Spec**: `00_nucleo/materialization/typst-passo-207A.md`.
**Output 2 de 4** (diagnóstico).
**Auditoria empírica**:
`00_nucleo/diagnosticos/typst-passo-207A-auditoria-introspector.md`.

---

## §1 C1 — Trait gaps prioritários

**Decisão fixada: 6 itens trait EXTENSÃO NECESSÁRIA priorizados**
(per A11 + A15 + A16, ordem por dependência ascendente).

| Prioridade | Método trait | Magnitude | Bloqueia | Dependências |
|------------|--------------|-----------|----------|--------------|
| 1 | `query_labelled() -> Vec<(Label, Location)>` | S (~30 min) | nada | nenhuma |
| 2 | `label_count(label) -> usize` | M (~2h) + refactor `LabelRegistry` | multi-label semântica | refactor sub-store |
| 3 | `pages(loc) -> Option<NonZeroUsize>` | M (~2-3h) | accessors stdlib `location.pages()` | page-aware infra |
| 4 | `page(loc) -> Option<NonZeroUsize>` | M (~2-3h) | accessors stdlib `location.page()` | page-aware infra (reusa `runtime.known_page_numbers`) |
| 5 | `page_numbering(loc) -> Option<&Numbering>` | M (~2h) | `outline()` page-numbering | page-aware infra |
| 6 | `page_supplement(loc) -> Option<&Content>` | M (~2h) | accessors raros | page-aware infra |

**Justificação literal** (per A11 + A16):

- **Não-prioritários**: `locator(key, base)` (DIVERGÊNCIA
  ARQUITECTÓNICA LEGÍTIMA — cristalino single-pass; vanilla
  measurement-driven), `anchor(loc)` (HTML target apenas),
  `document(loc)` / `path(loc)` (bundle target apenas).
  Mantidos como divergências legítimas; **não materializar** em P207.
- **`query_count_before(sel, end)`** (DECISÃO PENDENTE) — fica
  fora; sem consumer cristalino actual.

Hipótese spec específica per P205D D3 ("page-relevantes +
`label_count` são EXTENSÃO NECESSÁRIA prováveis"):
**CONFIRMADA** literalmente — 5/6 prioritários são page-aware ou
multi-label.

---

## §2 C2 — Sub-stores gaps prioritários

**Decisão fixada: 1 sub-store estrutural + sub-stores adicionais
deferidos**.

Per A4-A6 + A15 (item 37) + A11:

| Prioridade | Sub-store | Tipo trabalho | Magnitude | Bloqueia |
|------------|-----------|---------------|-----------|----------|
| 1 | `LabelRegistry` → `MultiMap<Label, Location>` | Refactor | M (~2h + cross-modular updates) | `label_count` + multi-label use-cases |

**Sub-stores adicionais NÃO criados**:
- **Page-aware store** (esperado para `page`/`pages`/`page_numbering`/
  `page_supplement`): designar **`PageStore`** novo seria opção; mas
  cristalino já tem `runtime.known_page_numbers` parcial.
  C8 fixa: estender `LayouterRuntimeState` ou criar `PageStore`
  dedicado — decisão deferred para P207B.
- **`SealedLabelPages`** (mencionado em hipótese spec C2 P205D
  deferred): per A11 — sem consumer real ainda. **Mantém deferred**;
  reabrir só se `pages()` ou consumer real emergir.

Hipótese spec específica P205D ("`SealedLabelPages` foi deferido
em P205D; P207A pode reabrir"): **NÃO REABRE**. Empírico A11+A16
mostra que `pages(loc)` (item 9) não tem consumer cristalino real
imediato; reabrir `SealedLabelPages` agora seria especulativo.

---

## §3 C3 — Consumers prioritários

**Decisão fixada: 2 consumers críticos + outros deferidos**.

Per A7+A8+A11+A16 + bloqueios identificados:

| Prioridade | Consumer | Magnitude | Desbloqueia |
|------------|----------|-----------|-------------|
| 1 | `here()` stdlib | M (~3-4h) | `counter.get()`, `state.get()`, `Selector::Before/After` consumers, `position_of` accessor real |
| 2 | `locate(selector)` stdlib | M (~2h) | resolução de location via Selector::Label (precisa C4 abaixo) |

**Justificação literal**:

- `here()` é **dependência hub**: 4+ outros itens da auditoria
  bloqueiam-se por ela. Magnitude M relativamente baixa (~3-4h
  contra L-XL agregada de tudo o resto); razão custo/desbloqueio
  alta. Hipótese spec P206C D9 (`here()` desbloqueia consumer
  real para `position_of`) **confirmada** por A11.
- `locate(selector)` é trivial dado `Selector::Label` (C4) +
  `here()` (C3.1).
- **Não-prioritários** em P207:
  - `outline(target, depth, ...)` configurável — bloqueada por
    Selector::Elem (DECISÃO PENDENTE 58).
  - Rich `Counter`/`State` types — DECISÃO PENDENTE 49+50;
    magnitude L-XL.
  - Bibliography expansion — DEBT-55 separado (XL).

---

## §4 C4 — Selector enum extensions

**Decisão fixada: Materializar minimal subset (`Label` + `And` +
`Or`)**.

Per A12-A14 + A15 (itens 57+59) + A16 Bloco III:

Itens incluídos:
- `Selector::Label(Label)` — S (~1h) — desbloqueia
  `locate(<label>)` + queries por label.
- `Selector::And(EcoVec<Self>)` — M parcial (~1.5h) — semântica
  intersecção.
- `Selector::Or(EcoVec<Self>)` — M parcial (~1.5h) — semântica
  união.

Itens **excluídos** (DECISÃO PENDENTE ou DIVERGÊNCIA):
- `Selector::Where` (item 58) — exige Element field reflection
  que cristalino não tem (`ElementKind` enumerado, não reflection).
  DECISÃO PENDENTE — humano clarifica em C10.
- `Selector::Regex` (item 61) — cristalino sem regex domain.
  DECISÃO PENDENTE.
- `Selector::Before/After` (item 60) — bloqueada por `here()`
  para uso real; estrutural M sem consumer imediato.
- `Selector::Within` (non-exposed em vanilla) — N/A.
- `Selector::Location` — DECISÃO PENDENTE; sem consumer
  cristalino actual.
- `Selector::Can` (capability) — DIVERGÊNCIA ARQUITECTÓNICA
  LEGÍTIMA (cristalino sem capability system).
- `Selector::Elem(Element, fields)` — DIVERGÊNCIA ARQUITECTÓNICA
  LEGÍTIMA; cristalino tem `Kind(ElementKind)` enumerado.

C4 fixa **uma**: minimal subset (Label + And + Or). **Não**
materializar todos; **não** adiar tudo.

Justificação: A14 mostrou que `Selector::Label` desbloqueia
`locate(<...>)` (consumer real); `And`/`Or` são triviais
estruturalmente e habilitam queries compostas. Custo agregado
~4h contra benefício de 3+ stdlib funcs ficando funcionais.

---

## §5 C5 — Stdlib expansion

**Decisão fixada: Materializar `here()` + `locate()` em série
dedicada P208** (não em P207).

Per A7+A8+A16 Bloco IV + C3:

Justificação literal:
- Magnitude `here()` (M ~3-4h) + `locate()` (S-M ~2h) = M
  agregado ~5-6h. **Cabe em série dedicada**.
- P207 dedicada a trait extensions (C1) + sub-stores (C2) +
  Selector minimal (C4) — magnitude já agregada M-L.
- Mantê-las em P207 inflaria magnitude para L-XL sem benefício
  (são features ortogonais).

P208 (proposto):
- P208A — diagnóstico-primeiro `here()` (Tracked<Context>
  análogo cristalino vs `EvalContext.location`).
- P208B-C — materialização `here()` + `locate()` + tests.
- P208D — encerramento + ADR.

C5 fixa **uma**: série dedicada P208. **Não** materializar em
P207 (overflow). **Não** adiar indefinidamente (consumer real
desbloqueia 4+ items).

---

## §6 C6 — Estrutura da trajectória

**Decisão fixada: Caminho 3 — Marco arquitectónico inteiro
(M9-completion ou similar)**.

Per A15 + A16 + C8 + decisão `P207A.div-1` em C12:

Justificação literal:
- A16 magnitude agregada **L** (sem rich types) ou **XL** (com
  rich Counter/State).
- C5 já isola `here()`/`locate()` para P208 — separa series.
- Trabalho restante (C1 trait extensions + C2 sub-store refactor +
  C4 Selector minimal + page-aware infra) é per se ~L magnitude.

Caminho 1 (série única P207A-E): **rejeitado**. Magnitude L+
não cabe; já provado por A16.

Caminho 2 (sub-séries dedicadas P207/P208/...): **rejeitado por
incompletude**. Sub-séries dispersas perdem coerência narrativa
(há ~8 itens correlacionados); marco arquitectónico unifica.

Caminho 3 (marco arquitectónico): **fixado**. Cria marco com:
- P207 — trait extensions + sub-store refactor (C1+C2).
- P208 — stdlib `here()`/`locate()` (C5).
- P209 — Selector minimal extensions (C4).
- P210 — page-aware infrastructure consolidação.
- P211 — encerramento + ADR transition + métricas finais.

---

## §7 C7 — Nome do marco arquitectónico

**Decisão fixada: M9-completion** (etiqueta literal: **M9c**).

Per padrão observável em blueprint + ADRs anteriores:

- **M7** (Fixpoint runtime) — fechado P192B.
- **M8** (`#[comemo::track]` em Introspector + Position) — fechado
  P204H.
- **M9** (Stdlib introspection 11/11) — fechado P182F (per
  blueprint §3.0).
- **F3** (Layouter sub-stores trackable / sealing) — fechado
  P205E.

Convenção observada:
- M = milestone arquitectónico (Major).
- F = feature/refactor concreto.

Itens C1-C5 são **continuação directa de M9** (Stdlib
introspection): completar trait Introspector para paridade
funcional vs vanilla. Não é F (não é refactor); não é M novo
(continua trabalho M9 que foi declarado fechado prematuramente
em P182F antes de auditoria empírica de gap).

**M9c** (M9-completion): designação que reconhece que M9 foi
fechado em P182F sob critério "stdlib 11/11", mas auditoria
P207A revelou gap trait + sub-stores + Selector que continua
o esforço M9 com escopo expandido empírico-fundamentado.

Alternativas consideradas:
- **M10** (continuação numérica): rejeitada — M10 é etiqueta
  para próximo marco verdadeiramente novo (e.g. Show rules,
  Math layer, etc.); P207 trabalha sobre M9 existente.
- **F4** (continuação de F3): rejeitada — F é refactor; P207 é
  extensão funcional + alguns refactors menores.
- **M9b**: rejeitada — sugere "patch ortogonal"; "completion"
  é semântica mais clara.

---

## §8 C8 — Magnitude agregada e orçamento

**Output: L agregado** (~30h sem rich Counter/State; XL ~50h
com rich types — DECISÃO PENDENTE C10 fixa qual).

Decomposição per spec C8 + A16:

| Sub-passo (M9c) | Tipo | Magnitude | Tempo |
|-----------------|------|-----------|-------|
| P207A | Diagnóstico (este) | M | ~45 min |
| P207B+ | Trait extensions (C1) + sub-store refactor (C2) | L | ~12-15h |
| P208A-D | `here()`/`locate()` (C5) | M | ~5-7h |
| P209A-C | Selector minimal (C4 + items 57+59) | S-M | ~4-5h |
| P210A-D | Page-aware infrastructure consolidação | L | ~10-12h |
| P211A-B | Encerramento + ADR-0076 ACEITE + métricas | S documental | ~1-2h |

**Total série M9c**: ~33-42h. Magnitude **L agregado**.

Range possível (per spec):
- **L** (escopo reduzido per `P207A.div-1`): ~30h.
- **XL** (escopo completo com rich Counter/State + outline
  configurável + Selector::Where): ~50-60h.

C10 abaixo lista decisões pendentes que distinguem L vs XL.

Hipótese spec §10 ("magnitude pode aproximar-se de L dado
escopo amplo"): **CONFIRMADA** — L agregado para escopo
reduzido pragmático.

---

## §9 C9 — ADR proposta?

**Decisão fixada: SIM — ADR-0076 PROPOSTO em P207A**.

Justificação literal (per spec C9 critério "decisão arquitectural
com alternativas reais"):

- **Caminho 1 (série única) vs Caminho 2 (sub-séries) vs Caminho 3
  (marco)**: alternativa real (C6).
- **Materializar tudo vs reduzir escopo per `P207A.div-1`**:
  alternativa real (C12).
- **Selector minimal vs full vs adiar**: alternativa real (C4).
- **Stdlib em P207 vs série dedicada P208**: alternativa real (C5).
- **Rich Counter/State types vs continuar minimal stdlib**:
  alternativa real (DECISÃO PENDENTE 49+50; C10).
- **Page-aware via PageStore novo vs estender `LayouterRuntimeState`**:
  alternativa real (C2).

Padrão consolidado: marcos arquitectónicos com decisão estrutural
ganham ADR dedicada — ADR-0072 (M7), ADR-0073 (M8), ADR-0074
(F3), ADR-0075 (vanilla integration). M9-completion é
estruturalmente comparável (escopo amplo + escolhas
arquitectónicas múltiplas) — merece ADR-0076.

Conteúdo ADR-0076 PROPOSTO em
`00_nucleo/adr/typst-adr-0076-introspector-completion.md`
(produzido como Output 4 de P207A).

Plano de validação ADR-0076 — 7 condições paralelas a ADR-0073:
- P207B-* materializados (trait + sub-store refactor).
- P208A-D `here()`/`locate()` materializados.
- P209A-C Selector minimal materializado.
- P210A-D page-aware infrastructure consolidada.
- Tests workspace verdes (estimativa 1873 → 1900-1950; ∆+27 a +77).
- `crystalline-lint .` 0 violations preservadas.
- ADR-0073 retroactivo: cond paridade trait estendida com
  page-aware items.

---

## §10 C10 — Decisões pendentes para humano

**Output: 4 questões formuladas para humano antes de P207B**.

Per spec C10 + A15 itens DECISÃO PENDENTE (7 itens):

### Q1 — Rich `Counter` / `State` types?

**Itens A15**: 49, 50.

Vanilla expõe `Counter` e `State` como **tipos de domínio
completos** com 7-15 métodos cada (`step`, `update`, `display`,
`get`, `at`, `final_`, `construct`). Cristalino actualmente
expõe apenas `native_counter_at`/`native_counter_final` (2
funções flat) + `native_state`/`native_state_update`/`_with`
(stub).

**Opção α**: materializar rich types em série dedicada
(magnitude L-XL agregado ~15-25h). Aproxima paridade vanilla
mas inflaciona M9c.

**Opção β**: manter forma minimal cristalino (`native_*` flat
funcs) + apenas adicionar `counter.step` / `counter.display`
como funcs separadas. Magnitude S-M (~5h).

**Opção γ**: adiar completamente; consumer real ainda inexistente.

**Recomendação P207A**: **β** — cristalino segue estilo "funcs
flat" estabelecido em P171/P175/P177; rich types seriam
inversão de design.

### Q2 — `Selector::Where` (Element field filter)?

**Item A15**: 58.

Vanilla `Selector::Elem(Element, Option<fields>)` permite
queries como `heading.where(level: 1)`. Cristalino tem
`Kind(ElementKind)` enumerado, não reflection sobre
`ElementInfo` fields.

**Opção α**: materializar `Selector::Where { kind: ElementKind,
predicate: ... }` com predicate-em-string (limitado).

**Opção β**: materializar reflection sobre `ElementInfo`
(magnitude L cross-modular).

**Opção γ**: adiar; sem consumer real ainda.

**Recomendação P207A**: **γ** — sem consumer cristalino real
imediato; bloqueia muito trabalho de reflection.

### Q3 — `Selector::Regex` e `Selector::Location`?

**Itens A15**: 61, parcial 60.

Cristalino sem regex domain. `Selector::Location` usado em
vanilla principalmente para `query_first(&Selector::Location(loc))`.

**Opção α**: materializar ambos.

**Opção β**: adiar ambos.

**Recomendação P207A**: **β** — sem consumer real imediato.

### Q4 — `query_count_before(sel, end)`?

**Item A15**: 6.

Vanilla optimização para counters/state. Cristalino caller
faria scan manual.

**Opção α**: materializar (~2h).

**Opção β**: adiar; medir performance impact primeiro.

**Recomendação P207A**: **β** — sem evidência de bottleneck.

---

## §11 C11 — Sub-passos `*B+` (plano de série)

**Plano fixado** (sub-passos da série P207, per C6 marco M9c):

### P207B — Trait extensions baixo-custo

Magnitude S (~1-2h).

- Materializar `query_labelled() -> Vec<(Label, Location)>` (item 5).
- Adicionar `LabelRegistry::iter()` ou similar para suportar.
- Tests: 3-4 unit (vazio; populated; ordem de inserção).
- L0 update: `00_nucleo/prompts/entities/introspector.md` + L0
  `label_registry.md`.
- Outputs: 2 ficheiros (inventário + relatório).

### P207C — Sub-store refactor `LabelRegistry` → `MultiMap`

Magnitude M (~2-3h).

- Refactor `LabelRegistry` para suportar multi-label (item 7+37).
- Materializar `label_count(label)` trait method.
- Update consumers que assumem label única (audit cross-modular).
- Tests: 5-7 unit.
- L0 update: `label_registry.md` + `introspector.md`.
- Outputs: 2 ficheiros.

### P207D — Page-aware trait methods (parcial)

Magnitude M-L (~5-6h).

- Materializar `pages(loc)`, `page(loc)`, `page_numbering(loc)`,
  `page_supplement(loc)` trait methods (itens 9+10+12+13).
- Decidir entre estender `LayouterRuntimeState` ou criar `PageStore`
  dedicado (clarificação humano em P207D arranque).
- Inject mecanismo paralelo a P205C `inject_positions`.
- Tests: 8-10 unit + 2-3 integration.
- L0 update: `introspector.md` + sub-store relevante.
- Outputs: 3 ficheiros.

### P207E — Encerramento + ADR transição parcial

Magnitude S documental (~30 min).

- Auditoria conds parciais ADR-0076.
- Estado: PROPOSTO → PARCIAL (aguarda P208/P209/P210).
- Relatório consolidado P207A-E.
- Outputs: 2 ficheiros.

### P208 série — `here()` + `locate()` (per C5)

Plano sub-séries:
- P208A — diagnóstico-primeiro (`here()` design + `Tracked<Context>`
  análogo).
- P208B — `here()` materialização + tests.
- P208C — `locate(selector)` materialização + tests.
- P208D — encerramento + ADR-0076 cond stdlib cumprida.

### P209 série — Selector minimal (per C4)

Plano sub-séries:
- P209A — diagnóstico-primeiro.
- P209B — Selector::Label + impl em `query()`.
- P209C — Selector::And + Or.
- P209D — encerramento + ADR cond Selector cumprida.

### P210 série — Page-aware infrastructure consolidação

Plano sub-séries:
- P210A — diagnóstico-primeiro page-aware.
- P210B-C — materialização (estende P207D).
- P210D — encerramento.

### P211 — Marco M9c fechamento

Plano:
- P211A — auditoria 7 condições ADR-0076.
- P211B — ADR-0076 PROPOSTO → ACEITE; blueprint M9c marca.

---

## §12 C12 — `P207A.div-1` (escopo reduzido)

**Decisão fixada: REGISTAR `P207A.div-1`** sobre redução de escopo.

Justificação per A16 + A15:

A pré-fixação ("escopo amplo: trait + sub-stores + consumers")
foi guidance, não constrangimento absoluto. Auditoria empírica
A15 + A16 mostram:

1. **44% dos itens são DIVERGÊNCIA ARQUITECTÓNICA LEGÍTIMA** —
   tentar "completar" estes seria inverter ADR-0073/0074 e perder
   propriedades cristalinas (single-pass, sub-stores especializados,
   sealing F3).
2. **11% são DECISÃO PENDENTE** — bloqueiam P207B até clarificação
   humana (C10).
3. **Apenas 31% são EXTENSÃO NECESSÁRIA** com magnitude tractável.

Escopo amplo literal absoluto (62 itens) seria XL+ (~50-60h)
sem benefício proporcional — DEBT-55 (bibliography XL) e rich
Counter/State (XL) consumiriam maioria do tempo sem mudar
paridade observable cristalino vs vanilla (que já é parcial-OK
per ADR-0075 P206E).

**`P207A.div-1` recomenda escopo reduzido fundamentado**:
- **Incluir**: 19 itens EXTENSÃO NECESSÁRIA (Bloco I-IV sem
  rich types) + 3 itens Selector minimal (C4) = 22 itens
  materializáveis.
- **Excluir**: 27 itens DIVERGÊNCIA ARQUITECTÓNICA LEGÍTIMA
  (não materializar — divergência é design-pattern cristalino).
- **Diferir**: 7 itens DECISÃO PENDENTE para C10 humano.
- **DEBT-55** continua separado (XL fora marco M9c).

Magnitude resultante: **L** (~30h) em vez de XL (~50-60h). Marco
M9c factível em ~5 séries (P207-P211) em vez de ~8 séries.

Pré-fixação não absorve obrigação de inflar quando empírico
mostra inviabilidade. C12 cumpre spec §10:

> "Pré-fixação do escopo é guidance, não constrangimento
> absoluto. Se a auditoria mostrar que escopo amplo é XL sem
> benefício proporcional, C12 legitima recomendação de redução
> com fundamento empírico."

---

## §13 C13 — Sem cláusulas condicionais

C1–C12 fixadas com valores concretos baseados em auditoria
empírica. Sem ramos.

Pontos onde C13 poderia ter ficado condicional, agora fixados:
- C2 (sub-store novo `PageStore` vs estender `LayouterRuntimeState`):
  resolvido como **decisão deferida para P207D arranque com
  clarificação humana**. Não fica condicional em P207A — fica
  pergunta humana explícita em P207D.
- C5 (stdlib em P207 vs série dedicada): **fixado série dedicada
  P208**. Sem ramo.
- C6 (estrutura): **fixado Caminho 3 marco arquitectónico**.
  Sem ramo.

---

## §14 Decisões durante a leitura

### D1 — Vanilla trait tem 16 métodos, cristalino 20 — paridade não é numérica

Esperava-se que cristalino fosse subset de vanilla. Empírico:
cristalino tem **mais métodos** (20 vs 16), mas com semântica
**mais especializada** (figure_number_for_label,
formatted_counter_at, is_numbering_active, etc.). Vanilla é
**mais genérico** (query via Selector polymorphic).

Implicação: classificação A15 não é "X tem, Y não tem" — é
"X resolve via specialização, Y resolve via genericidade". 44%
DIVERGÊNCIA ARQUITECTÓNICA LEGÍTIMA reflecte isto.

### D2 — Sub-stores cristalino têm divergência arquitectónica fundamental

Cristalino tem 9 sub-stores nomeados; vanilla tem 5 acceleration
structures internas + Counter/State como domain types. **Não
são isomorphic**. Tentar reduzir cristalino a "ElementIntrospector
único" inverteria ADR-0029/0073/0074.

### D3 — `here()` é dependência hub mais crítica

A11 + A16 mostram que `here()` (item 47) bloqueia ~5+ outros
itens. Razão custo (M ~3-4h) / desbloqueio (5+ items) é a
mais alta da auditoria. Por isso C5 fixa série P208 dedicada.

### D4 — `LabelRegistry` single-label é único refactor estrutural necessário

Restantes sub-stores são divergências legítimas. Apenas
`LabelRegistry → MultiMap` (item 37) precisa refactor para
desbloquear `label_count` (item 7) — único trabalho de "refactor
sub-store" em P207.

### D5 — Marco M9c reconhece M9 fechado prematuramente

P182F fechou M9 sob critério "stdlib 11/11". Auditoria P207A
revela trait + sub-store + Selector gaps que continuam M9 com
escopo expandido. **Em vez de abrir M10 novo**, M9c reconhece
continuidade narrativa. Pattern reusável: marcos podem ter
"completion" sub-fases pós-fecho original.

### D6 — Bibliography continua em DEBT-55 separado

DEBT-55 ("Bibliography + Cite XL") continua tracker próprio.
P207 NÃO endereça hayagriva integration. Cristalino bib_*
methods (P181F) continuam parcial; aceitável fora P207.

### D7 — Pixel-perfect / observable divergence preservada (per ADR-0075)

C12 redução de escopo NÃO inverte ADR-0075 (paridade observable
estrutural-only via `typst query`). Maioria dos page-aware items
(C1) afecta layout PDF, mas geometric divergência (FixedMetrics
vs FontBookMetrics) permanece N/A.

---

## §15 Resumo — métricas previstas

| Métrica | Valor |
|---------|-------|
| Marco arquitectónico | **M9c** (M9-completion) |
| Caminho fixado (C6) | **3 — marco arquitectónico** |
| Selector enum decisão (C4) | **minimal subset (Label + And + Or)** |
| Stdlib decisão (C5) | **série dedicada P208** |
| Sub-passos M9c | **5 séries (P207-P211)** |
| Magnitude agregada (C8) | **L** (~30h escopo reduzido) |
| ADR-0076 PROPOSTO (C9) | **SIM** |
| `P207A.div-1` (C12) | **SIM — redução fundamentada** |
| Itens DECISÃO PENDENTE (C10) | **4 questões humano** |
| Tests workspace antes | 1873 |
| Tests workspace depois (estimativa M9c completo) | 1900-1950 (∆+27 a +77) |
| Itens EXTENSÃO NECESSÁRIA materializáveis | **22** (19 trait + 3 Selector) |
| Itens DIVERGÊNCIA ARQUITECTÓNICA LEGÍTIMA preservados | **27** |
| Itens deferidos (DECISÃO PENDENTE) | **7** |
| LOC novas (código) estimadas M9c | ~600-1000 |
| LOC novas (docs) estimadas M9c | ~5000+ |
