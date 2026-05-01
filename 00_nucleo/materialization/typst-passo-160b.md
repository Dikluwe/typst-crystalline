# Passo P160B — `state(key, init)` runtime mutable state (Introspection Bloco B sub-passo 1)

Primeiro sub-passo substantivo de Introspection runtime per
Bloco B do diagnóstico P160 (recomendação primária §6
pós-promoção ADR-0066). Materializa **runtime mutable state**
— feature genuinamente nova após 19 passos consecutivos de
trabalho em Model/Layout. **Vigésima segunda aplicação consecutiva
de materialização** desde início da série granular P156C
(passos administrativos não contabilizados).

**Primeira materialização real Introspection** desde série
granular P156C. **Decisão arquitectural-chave consciente**: este
passo continua subpadrão #15 ("infraestrutura state lookup")
crescendo N=3→4 — pré-recomendação primária **Opção C**
(field aditivo em CounterState + walk arm) per consistência
com pattern já validado em P158B (`state.lang`), P159C
(`state.bib_entries`), P159F (`state.bib_numbers`).

---

## Nota arquitectural prévia

Decisão humana pós-debate: continuar subpadrão #15 como caminho
pragmático para construir Introspection de forma sólida agora,
**reconhecendo deriva arquitectural acumulada** vs meta original
de isolamento melhor que vanilla. `CounterState` cresceu para
14 fields públicos ao longo de P75-P159F — empilhamento que
fere meta de isolamento mas é o caminho consolidado actual.

**Trabalho futuro registado** (não reservado per política):
melhorar arquitectura via refactor para tipos entity isolados
(subpadrão #14) substituindo fields acumulados em CounterState.
Candidato a ADR meta XS futuro + refactor cumulativo XL pós
decisão estratégica.

P160B avança com Opção C consciente da deriva. Prioridade
actual: Introspection sólido.

---

## Estado actual antes de começar

- 65 ADRs após P160A (28 EM VIGOR; 19 IMPLEMENTADO; 13 PROPOSTO
  incluindo ADR-0066 e ADR-0062 sem promoção).
- Layout: 78% (inalterado). Cobertura arquitectural total 82%.
- Cobertura Model agregada: ~50% inalterada.
- **Cobertura Introspection: 17% saturada por tecto puro
  single-pass** (per P160 §4).
- Hash actual `entities/content.rs`: `ec58d849` (preservado em
  **19 passos consecutivos** P156L → P160A via L0-baseline).
- 1480 tests lib+integ+diag (workspace 1501); zero violations
  linter.
- 58 variants Content; 48 stdlib funcs.
- Padrões consolidados pós-P160A: granularidade N=21;
  inventariar N=25; Smart→Option Caso A patamar N=7
  (43/57 Layout/Model); §análise risco N=25; estabilidade
  hash L0 content.rs N=19; tipo entity em ficheiro próprio
  N=5; **infraestrutura state lookup N=3 (limiar formalização)**;
  P155 cross-feature N=1; refino tipo entity sem alteração
  Content N=3 (limiar formalização); refactor de field para
  Option N=1; helper `optional_str` cumulativo N=12 (largamente
  promovível); subpadrão "passo administrativo XS criar ADR
  PROPOSTO" N=2 (meio-caminho limiar formalização).

**Diagnóstico P160** §3-§6 (esboço P160B; era P160A no relatório
P160 antes da renomeação):
- Refino: `state(key, init)` runtime mutable state.
- Pré-condição: ADR-0066 PROPOSTO ✓ (P160A executado).
- Tamanho: M.
- Cobertura Δ esperada: +6-8pp Introspection (~17% → ~23-25%).
- Tests Δ: +10-15.
- Granularidade: M preservado.

**Política "sem novas reservas" preservada** — P160B não cria
reservas para passos pós-P160B.

**Leituras prévias obrigatórias**:
- `00_nucleo/diagnosticos/diagnostico-introspection-passo-160.md`
  — §3 features vanilla + §5 sequência candidata + §6
  recomendação.
- `00_nucleo/adr/typst-adr-0066-introspection-runtime-adiada.md`
  — ADR-0066 contexto + decisão + plano promoção.
- `00_nucleo/materialization/typst-passo-160-relatorio.md` —
  Bloco A vazio + Bloco B candidatos detalhados.
- `00_nucleo/materialization/typst-passo-160a-relatorio.md` —
  ADR-0066 PROPOSTO formalizada.
- `00_nucleo/materialization/typst-passo-158b-relatorio.md` —
  precedente subpadrão #15 N=1 (`state.lang`).
- `00_nucleo/materialization/typst-passo-159c-relatorio.md` —
  precedente subpadrão #15 N=2 (`state.bib_entries`).
- `00_nucleo/materialization/typst-passo-159f-relatorio.md` —
  precedente subpadrão #15 N=3 (`state.bib_numbers`).
- `01_core/src/rules/introspect.rs` — pipeline actual (1108
  linhas); walk single-pass.
- `01_core/src/entities/counter_state.rs` — `CounterState`
  actual (333 linhas) com 14 fields públicos cumulativos.
- `lab/typst-original/crates/typst-library/src/introspection/state.rs`
  + `model.rs::StateElem` (vanilla, quarentena) — referência
  paridade observable.

---

## Natureza do passo

**Tamanho**: M.

**Justificação**: 1 feature (state runtime). Field aditivo
`runtime_states` em `CounterState` (continuação directa
subpadrão #15) + helper de extracção stdlib + walk arm para
populate state map + layout arm para resolver state em uso.
Tests ~10-15.

**Decisão arquitectural-chave (deferida a .1 com pré-recomendação
forte Opção C)**:

- **Opção C (PRÉ-RECOMENDADA) — Field aditivo em CounterState**:
  - **Continuação directa subpadrão #15** já validado N=3
    (P158B `state.lang`; P159C `state.bib_entries`; P159F
    `state.bib_numbers`).
  - Subpadrão #15 cresce **N=3 → 4** atinge limiar formalização
    forte.
  - **Não cria variant Content novo** — preserva contagem 58.
  - **Hash content.rs preservado** per L0-baseline (continua
    19→20 passos consecutivos).
  - Mecânica idêntica aos 3 precedentes: walk popula via arm
    selectivo; layout arm consulta via `state.runtime_states`.
  - Pro: consistência total; contra: contribui para deriva
    de CounterState como god-struct.

- **Opção A (alternativa avaliada, NÃO recomendada) — Variant
  Content novo `State`**:
  - Paridade vanilla mais directa (`StateElem`).
  - Quebra hash content.rs (variant 58→59); quebra streak 19
    consecutivos.
  - Justificação fraca: subpadrão #15 já tem paridade observable
    suficiente (`state.bib_numbers` resolve `[N]` em Cite via
    walk + layout — mesmo pattern mecânico).

- **Opção B (alternativa avaliada, NÃO recomendada) — Stdlib
  sem variant**:
  - Single-pass trivial.
  - Perde paridade observable parcial (state mutável requer
    visibilidade walk→layout).
  - Incompatível com update futuro.

Granularidade preservada: 1 feature → mantém N=21 do padrão.

**Risco baixo-médio**:
- **Baixo** porque Opção C reusa pattern validado N=3.
- **Médio** porque é primeira materialização Introspection
  substantiva (ainda exige decisões cosméticas em scope
  minimal: tipos init aceites; update incluído ou diferido;
  paridade observable test E2E).

---

## Decisões já tomadas

- **Pré-recomendação Opção C**: field aditivo
  `pub runtime_states: HashMap<String, Value>` em `CounterState`
  per pattern subpadrão #15. Decisão final em .1 com matriz
  multi-critério; pré-recomendação forte registada.

- **Subpadrão #15 cresce N=3 → 4**: continuação directa de
  pattern validado.

- **Cobertura mínima**: paridade observable vanilla `state()`
  com read directo. Subset minimal:
  - `state(key: Str, init: Value) -> ContextualValue` cria
    state via Content marker ou stdlib retornando proxy
    resolvível.
  - Update deferido em P160B → P160C+ (pré-decisão; alternativa
    incluir minimal em .1).
  - Read em contexto produz init se sem update.

- **Sem cross-document state**: paridade single-document apenas
  per ADR-0033 minimal. Cross-document refs continuam diferidos
  (ADR-0066 §"Plano promoção").

- **Pipeline single-pass extendido**: walk passa a popular
  `runtime_states` map além de counters/bib_*. Layout arm
  consulta map por key (paridade mecânica P159F numbering).

- **Promoção ADR-0066 PROPOSTO → IMPLEMENTADO mantida em
  PROPOSTO**: P160B é primeiro materializador mas não cobre
  scope completo (metadata/here/locate/query/position
  pendentes). Promoção deferida para passo final Bloco B.

## Decisões diferidas

- **Variant `Content::State` ou só Content marker** (Opção C
  pode ter sub-variantes): se Opção C, ainda decidir se
  `Content::State { key, init }` é variant aditivo OU se
  state é resolvido na stdlib retornando ContextualValue
  proxy sem variant. **A decidir em .1**. Pré-decisão: variant
  marker minimal `Content::State { key, init }` para suportar
  walk arm dedicado; aceita-se como variant aditivo (60→não;
  fica em 58 sem variant) OU 58→59 (com variant marker).
  Trade-off em .1.

- **`update(key, fn)` em P160B vs P160C+**: pré-decisão deferir
  update se Opção C simples sem update viável.

- **Tipos `init` aceites**: subset minimal `Value::Str`,
  `Value::Int`, `Value::None` per ADR-0054 graded. Tipos
  estruturados (Array, Dict) diferidos.

- **State scoping**: paridade observable vanilla scope global
  per documento. Local scoping diferido.

- **Promoção `optional_str` a helper público** (N=12
  cumulativos): diferida em passo administrativo XS futuro
  NÃO reservado.

- **Refactor arquitectural CounterState → tipos entity
  isolados**: trabalho futuro registado em §"Nota arquitectural
  prévia" mas NÃO reservado.

---

## Sub-passos

### .1 Inventário pré-materialização (obrigatório per ADR-0065)

Diagnóstico em
`00_nucleo/diagnosticos/diagnostico-state-runtime-passo-160b.md`
com 7 itens canónicos (ADR-0034) + 4 itens específicos:

1. Assinatura vanilla `state(key, init)` — confirmar tipos
   aceites; behavior em ContextualValue vs Value directo;
   semântica scope.
2. Comportamento observable vanilla (state cria value default;
   read em contexto produz init).
3. ADR-0064 caso aplicável: **NÃO directamente em P160B**
   (state init é Value não Option).
4. Variants Content existentes a estender ou criar novos:
   confirmar Opção C sub-variante (variant marker 58→59 ou
   stdlib proxy sem variant) em §10.
5. Helpers stdlib reusáveis: pattern P159F `bib_numbers`
   walk → layout (mecânica idêntica).
6. Limitações aceites: subset minimal init types; sem cross-
   document; sem local scoping; update deferido.
7. Tests planeados (state cria; read produz init; multi-state
   independência — range 10-15).
8. **(Específico Opção C confirmação)** Validar pré-recomendação
   contra alternativas A/B com matriz multi-critério explícita:
   - **Opção A** (variant `State` aditivo): pro paridade
     vanilla; con quebra hash; con cresce enum.
   - **Opção B** (stdlib sem variant): pro single-pass; con
     perde paridade observable.
   - **Opção C** (field CounterState): pro consistência
     subpadrão #15; pro preserva hash; con engorda CounterState.
   Decisão final + sub-variante (marker variant 58→59 OU
   stdlib proxy) registada com justificação.

9. **(Específico scope minimal)** Decidir update incluído em
   P160B ou deferido a P160C+. Pré-decisão: deferir.

10. **(Específico tipo init)** Confirmar subset Value::Str/Int/None;
    estruturados diferidos.

11. **(Específico promoção ADR-0066)** Confirmar manter
    PROPOSTO até Bloco B saturado. Pré-decisão: manter.

### .2 Field aditivo em `CounterState`

`01_core/src/entities/counter_state.rs`:
- Adicionar `pub runtime_states: HashMap<String, Value>`
  (paridade aditiva subpadrão #15; **N=3 → 4**).
- Default `HashMap::new()`.
- Doc-comment do field menciona ADR-0066 + paridade observable.
- Hash counter_state.rs **preservado per L0-baseline** (paridade
  P158B/P159C/P159F resultados).

### .3 Variant marker `Content::State` (sub-variante Opção C +1)

Per decisão em .1 §8:

**Se variant marker 58→59**: `01_core/src/entities/content.rs`:
- Adicionar variant `State { key: String, init: Box<Content> }`
  marker minimal.
- Cobrir 9 sítios pattern-match Content (paridade P157A/B/C +
  P159A/C):
  - Variant declaration + constructor.
  - `is_empty()`: state declarações sempre `false` (são
    marcadores activos).
  - `plain_text()`: emite init plain_text.
  - `PartialEq`: cobre 2 fields.
  - `map_content`: recurse em init.
  - `map_text`: idem.
  - `introspect.rs::materialize_time`: idem.
  - `introspect.rs::walk`: arm State popula `runtime_states`.
  - `layout/mod.rs::layout_content`: arm State resolve via
    runtime_states lookup.
- **Hash content.rs quebra esperada se variant marker** —
  decisão consciente registada (primeiro reset desde N=19
  consecutivos).

**Se stdlib proxy sem variant**: implementar per decisão .1
sem alterar content.rs (preserva hash 19→20 consecutivos).

### .4 Walk arm em `introspect.rs`

`01_core/src/rules/introspect.rs`:
- Walk arm State (se variant marker) ou arm equivalente (se
  stdlib proxy):
  - Insert `state.runtime_states.insert(key, init)`.
  - Apenas primeira declaração vence (paridade `or_insert`).

### .5 Stdlib `native_state`

`01_core/src/rules/stdlib/state.rs` (ficheiro novo) ou
`01_core/src/rules/stdlib/structural.rs` (extensão):
- `native_state(args)` aceita:
  - `key: Str` posicional obrigatório (vazio rejeitado).
  - `init: Value` posicional obrigatório (subset minimal types).
- Retorna `Content::state(key, init)` (se variant marker) OU
  resolução in-place (se stdlib proxy).

### .6 Layout para state em uso

`01_core/src/rules/layout/mod.rs`:
- Pattern arm State (se variant marker) ou resolução em
  ContextualValue (se stdlib proxy):
  - Lookup `state.runtime_states.get(&key)`.
  - Se encontrado: resolve para current value.
  - Senão: render init directly (fallback paridade P159F).

### .7 Tests

- **Unit tests `Content::State`** em `entities/content.rs`
  (~3 if variant marker):
  - Constructor.
  - PartialEq.
  - is_empty + map_text/map_content.

- **Stdlib tests** (~3):
  - `state("counter", 0)` cria state.
  - Tipos inválidos init rejeitados.
  - Key vazia rejeitada.

- **Layout E2E tests** em `layout/tests.rs` (~3-4):
  - State sem ler produz init.
  - State lido em contexto produz init (sem update).
  - Multi-state independência.

- **Walk tests** em `introspect.rs` (~1-2):
  - Walk popula `runtime_states` correctamente.
  - Multi-state preserva primeira declaração.

**Δ esperado**: +10-15 tests.

### .8 Propagação de hashes + decisão promoção ADR-0066

`crystalline-lint --fix-hashes .`:
- `content.rs` hash: per Opção C sub-variante decisão .1.
  **Se variant marker**: quebra esperada (consciente).
  **Se stdlib proxy**: preservado L0-baseline (20º consecutivo).
- `counter_state.rs`: preservado L0-baseline (paridade
  P158B/P159C/P159F).
- `introspect.rs`/`layout/mod.rs`: refactor interno; preserva
  L0.
- Stdlib state file: hash novo se ficheiro novo.

Decisão promoção ADR-0066 PROPOSTO → IMPLEMENTADO documentada
em .1 (pré-decisão: manter PROPOSTO).

---

## Verificação

Numerada para reporte de conclusão:

1. `cargo test` em workspace: **1501 + Δ** tests, zero falhas
   (Δ esperado +10-15).
2. `crystalline-lint`: zero violations.
3. Contagem variants Content: **58** (stdlib proxy) ou **59**
   (variant marker) per decisão .1.
4. Contagem stdlib funcs: **48 ou 49** (depende `native_state`
   ficheiro novo ou extensão).
5. Decisão arquitectural Opção C confirmada com sub-variante
   registada em .1 (variant marker OU stdlib proxy).
6. Decisão sobre `update()` (incluído em P160B ou deferido)
   registada em .1.
7. Decisão sobre promoção ADR-0066 PROPOSTO → IMPLEMENTADO
   registada em .1 (mantém-se PROPOSTO ou promove).
8. **Hash `entities/content.rs`**: per sub-variante .1.
   **Se stdlib proxy**: preservado (20º consecutivo).
   **Se variant marker**: quebra consciente (primeiro reset).
9. **Hash `entities/counter_state.rs` preservado** L0-baseline
   (field aditivo via doc-comment paridade P158B/P159C/P159F).
10. **Sem novas reservas** criadas (paridade política
    P158/P159).
11. Cobertura Introspection: 17% → ~23-25% (subset minimal
    state). Documentar em tabela.
12. **Subpadrão #15 cresce N=3 → 4** (state.runtime_states
    adicionado a infraestrutura state lookup) — atinge limiar
    formalização forte.
13. Tests E2E paridade observable vanilla state cumprem.
14. ADR-0066 PROPOSTO referenciada como pré-condição cumprida.

---

## Critério de conclusão

- Verificações 1-14 passam.
- Relatório separado em
  `00_nucleo/materialization/typst-passo-160b-relatorio.md`
  produzido com:
  - Resumo do executado.
  - Confirmação de cada verificação.
  - **§análise de risco** (padrão N=25 → 26; primeira
    materialização real Introspection desde série granular).
  - Slope cumulativo Introspection (mesa nova começando em
    P160B).
  - ADR-0061 §"Aplicações cumulativas" anotada com P160B.
  - **Confirmação**: Bloco B subset minimal iniciado;
    subpadrão #15 cresce N=3 → 4 (limiar formalização forte).
  - **Decisão arquitectural-chave registada**: Opção C +
    sub-variante (variant marker OU stdlib proxy) com
    justificação multi-critério.
  - **Nota arquitectural**: continuação consciente subpadrão
    #15 reconhecendo deriva acumulada vs meta original
    isolamento. Trabalho futuro registado mas NÃO reservado.

---

## O que pode sair errado

**Cenários gerais**:
- Inventário .1 revela que vanilla `state()` exige multi-pass
  genuíno — ajustar Opção C para multi-pass-aware OU diferir
  update; documentar como graded per ADR-0054.
- Inventário .1 revela que `state()` vanilla aceita
  ContextualValue como init (não Value literal) — ajustar
  tipos init per decisão .1.

**Cenários específicos**:
- Walk popular `runtime_states` antes do layout vs durante —
  decisão de timing crítica; pode ser único walk pass com
  duas etapas (declarations primeiro, layout depois).
- Multi-state com mesmas key — `or_insert` paridade vanilla
  primeira declaração vence (paridade P159F bib_numbers).
- Tests E2E paridade vanilla falhar por edge case (state
  acessado antes da declaração no source) — decidir
  comportamento consistente per ADR-0033 minimal.
- Sub-variante variant marker vs stdlib proxy ambígua em .1 —
  matriz multi-critério em §8 do diagnóstico resolve.
- ADR-0066 PROPOSTO promoção precoce: P160B sozinho não cobre
  scope completo; manter PROPOSTO é decisão conservadora.
- L0-baseline NÃO preservar hash counter_state.rs — verificar;
  paridade P158B/P159C/P159F garantida via doc-comment field
  aditivo.

---

## Notas operacionais

- **Vigésima segunda aplicação de materialização**. Patamar
  empírico forte. **Primeira materialização real Introspection**
  — sem precedente directo na série.
- **§análise de risco no relatório** com peso baixo-médio.
  Vigésima sexta aplicação consecutiva preserva precedente.
- **Decisão arquitectural-chave** com pré-recomendação forte
  Opção C. Matriz multi-critério em .1 protege contra
  ambiguidade entre sub-variantes (variant marker vs stdlib
  proxy).
- **Subpadrão #15 "infraestrutura state lookup"** atinge **N=4**
  — limiar formalização **forte**. Promoção a ADR meta
  candidato a passo administrativo XS futuro NÃO reservado.
- **ADR-0066 promoção**: P160B é primeiro materializador.
  Pré-decisão **manter PROPOSTO** até Bloco B saturado
  (P160B-F).
- **Hash content.rs**: per sub-variante .1. Stdlib proxy
  preserva 20º consecutivo; variant marker quebra (primeiro
  reset desde N=19).
- **ADR-0064 NÃO directamente aplicável**: state init é Value
  não Option.
- **Nota arquitectural de deriva** (registada em §"Nota
  arquitectural prévia" deste enunciado e a propagar em
  relatório): subpadrão #15 N=4 atinge limiar forte mas
  representa **anti-padrão de catch-all** vs meta original
  isolamento. Decisão consciente de continuação pragmática.
  Trabalho futuro de refactor para tipos entity isolados
  (subpadrão #14) NÃO reservado mas registado.
- **Política "sem novas reservas" preservada**: refinos
  pós-P160B (update; restantes Bloco B P160C-F; refactor
  arquitectural) permanecem candidatos NÃO-reservados.

---

## Pós-passo

Após conclusão de P160B:

**Layout fica em 78% inalterado**. **Model fica em ~50%
inalterado**. **Introspection: 17% → ~23-25%** (subset minimal
state). **Hash `entities/content.rs`**: preservado (stdlib
proxy) ou quebra consciente (variant marker) per .1. **Subpadrão
#15 cresce N=3 → 4** (limiar formalização forte).

**Próxima decisão** (sem candidata pré-acordada — política
"sem novas reservas"):

- **P160C** (continuação Bloco B): metadata. S+; +3-5pp
  Introspection.
- **P160B-update** se update não incluído em P160B: refino
  XS adicional para state.update.
- **Restantes Bloco B**: P160D here/locate (M); P160E query
  (M+); P160F position (S+).
- **Conjunto administrativo XS** acumulado: `optional_str`
  N=12 promoção a helper público; ADR meta subpadrão #15
  (agora N=4 forte); ADR meta subpadrão #16 (N=3 limiar);
  L0 content.md update; ADR-0060 a R1; ADR meta saturação
  ADR-0064.
- **ADR meta deriva arquitectural** (passo administrativo
  XS NÃO reservado): registar deriva CounterState como
  god-struct + plano refactor para tipos entity isolados
  subpadrão #14.
- **Mudança de módulo**: Layout Fase 3 columns/colbreak ou
  outro.

ADR-0060 mantém-se `IMPLEMENTADO`. ADR-0061 mantém-se
`PROPOSTO`. **ADR-0066** estado per decisão .1 (PROPOSTO
mantido per pré-decisão; alternativa promover registada).
**ADR-0017 existente** mantém-se IMPLEMENTADO (eval deferral;
tópico distinto). ADR-0062 PROPOSTO.

Padrão granularidade 1-2 features/passo (N=21 com P160B se
fechar sem reformulação) **NÃO** é formalizado em ADR. Continua
candidato.

**Pausa natural após P160B — primeira materialização real
Introspection desde série granular; subset minimal state
runtime cumprido; subpadrão #15 atinge limiar formalização
forte (N=4); deriva arquitectural registada conscientemente
sem refactor imediato (prioridade Introspection sólido); hash
content.rs preservado (stdlib proxy) ou primeiro reset (variant
marker). Decisão humana sobre próxima direcção tem máxima
informação.**
