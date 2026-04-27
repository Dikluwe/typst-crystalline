# Passo P160 — Diagnóstico Introspection (módulo mais fraco 17%)

Passo arquitectural de diagnóstico precedendo materialização de
Introspection. **Não materializa código**. Análogo estrutural
a **P157/P158/P159 base** (diagnósticos focados precedendo
materialização). **Quarto diagnóstico de módulo** + **primeira
mudança de módulo cross-domínio Model → Introspection** desde
início da série granular P156C.

**Décima segunda aplicação concreta de ADR-0065 critério #5**
(scope determinado por inventário) com **diversidade nova**:
P157/P158/P159 inventariaram features Model; P159B inventariou
expansão Model amplo; **P160 inventaria módulo cross-domínio
diferente**.

---

## Estado actual antes de começar

- 63 ADRs após P159G (28 EM VIGOR; ADR-0060 IMPLEMENTADO;
  ADR-0017 reserva sem ficheiro mantida; ADR-0062 reserva
  sem ficheiro mantida).
- Layout: 78% (inalterado). Cobertura arquitectural total 82%.
- Cobertura Model agregada: ~50% (impl + impl⁺ inalterada).
  Cobertura ampla 77% inalterada.
- **Cobertura Introspection: 17%** (módulo mais fraco per
  diagnóstico P156B inventário 148 + confirmações em
  P159B §3 categoria A).
- Hash actual `entities/content.rs`: `ec58d849` (preservado em
  **17 passos consecutivos** P156L → P159G via L0-baseline).
- 1480 tests (lib+integ+diagnostic; workspace 1501); zero
  violations linter.
- 58 variants Content; 48 stdlib funcs.
- Padrões consolidados pós-P159G: granularidade N=21;
  inventariar N=23; Smart→Option Caso A patamar N=7
  (43/57 Layout/Model); §análise risco N=23; estabilidade
  hash L0 content.rs N=17; tipo entity em ficheiro próprio
  N=5; infraestrutura state lookup N=3 (limiar formalização);
  P155 cross-feature N=1; refino tipo entity sem alteração
  Content N=3 (limiar formalização); refactor de field para
  Option N=1; helper `optional_str` cumulativo N=12
  (largamente promovível).

**Estado factual Introspection antes de P160**:
- ADR-0017 "Introspection runtime adiada" reserva sem ficheiro
  pré-existente — confirmada em P159B §3 categoria A.
- `01_core/src/rules/introspect.rs` existe com walk single-pass
  + counters por kind + materialize_time. **Trabalho até agora
  foi uso, não materialização nova**:
  - P75 figure counters por kind.
  - P157A figure-table counters.
  - P158A auto-detect kind.
  - P159A walk Cite single-pass.
  - P158B `state.lang` infraestrutura state lookup.
  - P159C `state.bib_entries` idem.
  - P159F `state.bib_numbers` idem (subpadrão #15 N=3).
- `entities/counter_state.rs` cresceu cumulativamente com
  fields aditivos para state lookup.

**Política "sem novas reservas" preservada** — P160 não cria
reservas para passos pós-P160.

**Leituras prévias obrigatórias**:
- `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`
  — inventário 148; secção Introspection com cobertura 17%.
- `00_nucleo/diagnosticos/diagnostico-expansao-159-passo-159b.md`
  — §3 matriz dependências cruzadas; categoria A "Introspection
  runtime (ADR-0017)" com features bloqueadas.
- `00_nucleo/materialization/typst-passo-156b-relatorio.md` —
  precedente de diagnóstico amplo (Layout) por contraste.
- `00_nucleo/materialization/typst-passo-157-relatorio.md` —
  precedente de diagnóstico focado (paridade P160).
- `00_nucleo/materialization/typst-passo-158-relatorio.md` —
  idem.
- `00_nucleo/materialization/typst-passo-159-relatorio.md` —
  idem (par acoplado bibliography+cite).
- ADR-0017 referência (sem ficheiro; mencionar em README).
- ADR-0034 sobre estrutura de diagnóstico canónica.
- `01_core/src/rules/introspect.rs` — código actual completo.
- `01_core/src/entities/counter_state.rs` — state actual.
- `lab/typst-original/crates/typst-library/src/introspection/`
  (vanilla, quarentena) — referência paridade.

---

## Natureza do passo

**Tamanho**: S+ (alinhado com P157/P158/P159 base).

**Justificação**: Inventário focado de **um módulo** (Introspection)
com 7 itens canónicos ADR-0034 + 3 itens específicos
para cross-módulo dependencies + análise de tecto Introspection
+ recomendação primária. Trabalho documental puro. Sem
modificação de código, sem ADR nova.

**Diferenciador vs P157/P158/P159 base**:
- P157/P158/P159 base: diagnóstico de **uma feature** Model
  específica.
- P160: diagnóstico de **módulo completo** Introspection.
- Scope intermédio entre focado (P157/P158/P159 base) e amplo
  (P156B/P159B).

Granularidade preservada: 1 deliverable diagnóstico → mantém
peso S+ análogo aos P157/P158/P159 base.

**Risco baixo**: passo previne risco em sub-passos seguintes
detectando dependências cruzadas cedo. Particularmente
importante para cite cross-document refs (família 159 fora
Bloco A) + measure (Layout 100% caminho).

---

## Decisões já tomadas

- **Identificador P160**: paridade numérica com P157/P158/P159
  base (módulo focado per ADR-0065 critério #5). Sub-passos
  substantivos seguintes (P160A, P160B, etc.) a decidir per
  inventário.
- **Natureza diagnóstica**: P160 inventaria, não materializa.
- **Sem código alterado**: passo puramente documental.
- **Sem ADR nova**: ADRs existentes (0017, 0033, 0054, 0034,
  0060, 0061, 0062, 0064, 0065) lidas, não criadas.
- **Sem novas reservas**: paridade política P158/P159.
- **Foco em Introspection**: P160 NÃO inventaria Layout Fase 3
  ou outros módulos. Decisão deliberada para preservar scope.

## Decisões diferidas

- **Subset materializável de Introspection** sem entrar
  noutros módulos: a decidir em §4.
- **Tecto realista Introspection puro** vs pós-resolver
  dependências: a decidir em §4 análogo a P159B §4.
- **Ordem de execução** sub-passos materializáveis: a decidir
  em §5.
- **Identificadores concretos** sub-passos seguintes (P160A,
  P160B, etc., ou paragem se inventário sugerir): a decidir
  em §6.
- **Comparação com Layout Fase 3 columns/colbreak** como
  alternativa: explicitamente FORA do scope de P160 (decisão
  diferida a passo seguinte se Introspection saturar
  rapidamente).

---

## Sub-passos

### .1 Inventário ADRs/DEBTs Introspection

Localizar e ler:
- ADR-0017 conteúdo (reserva sem ficheiro — apenas menção em
  README e relatórios; documentar estado factual).
- DEBTs relacionados a Introspection (procurar em
  `00_nucleo/DEBT.md`):
  - DEBT cross-document refs?
  - DEBT measure?
  - DEBT counter timing?
  - DEBT state lookup limits?
- ADR-0033 paridade observable (counters; numbering).
- ADR-0054 graded (subset minimal).
- Comentários ou notas em `introspect.rs` que mencionem
  diferimentos.

Output: secção §1 do diagnóstico — mapa ADR/DEBT Introspection.

### .2 Inventário código actual Introspection

Inspecção de `01_core/src/rules/introspect.rs` +
`01_core/src/entities/counter_state.rs`:

Para cada estrutura, identificar:
- Funcionalidades já materializadas (walk, counters, materialize_time,
  state lookup).
- Estruturas internas (`CounterState`, fields aditivos
  cumulativos).
- Helpers privados e públicos.
- Hash actual dos ficheiros relevantes.
- Comportamento single-pass vs runtime referido em comentários.

Específicos:
- Walk algorithm (DFS? BFS? limitations?).
- Counters: por kind? por entry? cumulativos?
- materialize_time: fase do pipeline em que corre.
- State lookup: ADRs referenciadas? subpadrão #15 já N=3.

Output: secção §2 do diagnóstico — inventário código actual.

### .3 Inventário features Introspection vanilla

Inspecção de `lab/typst-original/crates/typst-library/src/introspection/`
(vanilla, quarentena):

Features vanilla a categorizar:
- `counter()` runtime queries.
- `query()` runtime introspection.
- `locate()` position-aware computations.
- `measure()` element measurement.
- `here()` current location.
- `state()` runtime state.
- `metadata()` arbitrary metadata.
- Cross-document references.
- Page-aware introspection.
- Layout-aware introspection.

Para cada feature:
- Cobertura cristalina actual (ausente / parcial / impl /
  impl⁺).
- Dependência runtime (ADR-0017 hard).
- Dependência measure (Layout integration hard).
- Dependência multi-pass (refactor pipeline).

Output: secção §3 do diagnóstico — features Introspection
vanilla com cobertura cristalina.

### .4 Análise tecto Introspection

Síntese de §1-§3 para responder:

1. Que features Introspection podem ser materializadas
   **sem promover ADR-0017** (single-pass viável)?
2. Que features exigem ADR-0017 promovida (runtime queries
   genuínas)?
3. Que features exigem `measure()` cross-módulo (depende
   Layout integration)?
4. Que features exigem refactor pipeline (multi-pass)?
5. Cobertura Introspection atingível com features puramente
   single-pass: estimativa numérica (17% → ~?%).
6. Cobertura atingível pós-ADR-0017 promovida: estimativa.
7. Diferença entre tecto Introspection puro vs tecto
   pós-resolver dependências.

**Análogo a P159B §4** mas para Introspection com base factual
acumulada (state lookup N=3; counters por kind validados).

**Distinção operacional** (paridade P159B):
- "Refino qualitativo" — extende pattern existente sem nova
  feature.
- "Materialização nova" — adiciona walk/counter/state arm
  novo.
- "Diferimento" — explicitar como ADR-0054 graded e parar.

Output: secção §4 do diagnóstico — análise tecto Introspection.

### .5 Sequenciar sub-passos materializáveis

Com base em §4, ordenar features Introspection materializáveis:

- **Bloco A — Features sem dependência ADR-0017**:
  single-pass viável; sem measure; sem multi-pass. Listar
  candidatos S+/M aplicáveis.
- **Bloco B — Features com dependência ADR-0017**: precedidas
  de promoção ADR-0017. Listar para informação.
- **Bloco C — Features com dependência cross-módulo**: NÃO
  materializáveis em Introspection puro. Listar para
  informação.

Para cada candidato em A:
- Identificador sugerido (P160A, P160B, etc.).
- Tamanho estimado.
- Subset minimal preservando granularidade.
- Tests Δ esperado.
- Hash impacto (counter_state.rs? content.rs? introspect.rs?).
- Aplicação ADR-0064/0065 esperada.
- Reuso pattern existente (e.g. state lookup N=3 → 4 se
  adicionado field state).

**Não criar reservas** — apenas listar candidatos com
informação; decisão sobre ordem real fica para sessão
posterior (paridade P159B §5).

Output: secção §5 do diagnóstico — sequência candidata.

### .6 Decisão sobre próximo passo concreto

Com base em §5 (Bloco A ordenado):
- Recomendar primeiro candidato a executar pós-P160.
- Identificar passo administrativo XS necessário antes (e.g.
  ADR-0017-create se Bloco B fica reservado para futuro).
- Estimar quantos sub-passos Introspection são alcançáveis
  antes de saturação (atingir tecto §4).

**Validação humana após este passo**: §6 do diagnóstico é
explicitamente recomendação, não decisão final. Tu validas
(paridade P159B §6).

**Cenário possível**: Bloco A vazio (todas features Introspection
materializáveis exigem ADR-0017) → recomendação muda para
"promover ADR-0017 via passo administrativo XS antes de qualquer
materialização Introspection". Caminho válido per spec P159B.

Output: secção §6 do diagnóstico — recomendação de execução.

### .7 Actualizar ADR-0061 §"Aplicações cumulativas"

Anotar P160 como passo diagnóstico cross-domínio. Tabela slope
cumulativo ganha linha P160 com slope Layout/Model "—" e tests
Δ "0" e nota "+ Introspection diagnóstico".

Padrões metodológicos:
- Inventariar primeiro N=23 → 24 (ADR-0065 critério #5
  décima segunda aplicação concreta com diversidade
  cross-domínio nova).
- §análise risco N=23 → 24.

### .8 Actualizar README ADRs

Sem ADR nova; entrada cronológica de P160 adicionada antes
de P159G.

---

## Verificação

Numerada para reporte de conclusão:

1. Diagnóstico
   `00_nucleo/diagnosticos/diagnostico-introspection-passo-160.md`
   produzido com 6 secções (§1 ADRs/DEBTs Introspection;
   §2 código actual; §3 features vanilla; §4 tecto
   Introspection; §5 sequência candidata; §6 recomendação).
2. Mapa ADR/DEBT Introspection documentado em §1 (ADR-0017
   confirmada como reserva sem ficheiro; DEBTs identificados
   e listados).
3. Inventário código actual factual em §2 (não inferido;
   referência explícita a `introspect.rs` + `counter_state.rs`).
4. Features Introspection vanilla categorizadas em §3 com
   cobertura cristalina por feature (counter/query/locate/
   measure/here/state/metadata/cross-document/page-aware/
   layout-aware).
5. Análise tecto Introspection em §4 com estimativas numéricas
   (cobertura puramente single-pass; pós-ADR-0017; diferença).
6. Sequência candidata em §5 com Bloco A populado (ou vazio
   se nenhuma feature single-pass material restante).
7. Recomendação concreta em §6 com primeiro candidato a
   executar OU recomendação de promoção ADR-0017 se Bloco A
   vazio.
8. **Sem novas reservas** criadas em P160 (paridade política
   P158/P159).
9. ADR-0061 §"Aplicações cumulativas" actualizada com linha
   P160.
10. `crystalline-lint`: zero violations (sem código alterado).
11. **Sem alteração de hashes** — `entities/content.rs`
    mantém `ec58d849` (18º passo consecutivo com interpretação
    L0-baseline).

---

## Critério de conclusão

- Verificações 1-11 passam.
- Relatório separado em
  `00_nucleo/materialization/typst-passo-160-relatorio.md`
  produzido com:
  - Resumo do diagnóstico (síntese das 6 secções).
  - Recomendação concreta para passo seguinte (de §6).
  - Listagem completa de candidatos em Bloco A com informação
    suficiente para tu decidires.
  - Listagem de candidatos em Bloco B/C com bloqueadores
    explícitos.
  - §análise de risco (padrão N=23 → 24).
  - Confirmação: ADR-0065 critério #5 décima segunda aplicação
    concreta com diversidade cross-domínio nova.
  - **Decisão crítica**: tecto Introspection puro vs
    pós-ADR-0017 — registar com estimativas (paridade P159B
    §4).
  - Confirmação: ADR-0017 estado factual (reserva sem ficheiro
    mantida; promoção a PROPOSTO via passo administrativo XS
    sugerida ou diferida per recomendação §6).

---

## O que pode sair errado

**Cenários gerais**:
- ADR-0017 ter conteúdo concreto não visto antes (e.g. esboço
  em comentário de introspect.rs ou nota antiga) — atalho
  possível: P160 pode descobrir conteúdo factual; documentar.
- DEBT-XX relacionado a Introspection ter scope mais amplo do
  que esperado (e.g. cobrir measure também) — ajustar matriz
  §3; documentar.
- Features vanilla Introspection serem mais elaboradas do que
  inventário 148 sugere (e.g. `query()` ter sub-features
  múltiplas) — categorizar minimalmente; diferir por ADR-0054
  graded.

**Cenários específicos**:
- **Bloco A vazio** → todas features Introspection materializáveis
  exigem ADR-0017 → recomendação §6 muda para "promover
  ADR-0017 via XS administrativo primeiro". **Caminho válido**;
  paridade ADR-0062-create análogo.
- Bloco A só 1-2 candidatos → Introspection puro saturado
  rapidamente; recomendar mudança após esses 1-2 passos.
- Cobertura Introspection actual ser maior que 17% após
  inventário factual (counters por kind + state lookup já
  acumulam) — recalcular cobertura; documentar.
- Counters por kind serem suficientes para ~90% Introspection
  features → tecto puro alto inesperado; ajustar §4.
- `measure()` revelar-se materializável sem cross-módulo
  (e.g. via state lookup com pre-computed dimensions) →
  excepção positiva; documentar.
- Múltiplos candidatos com tamanho similar e sem dependência →
  matriz comparação adicional pode ser útil.

---

## Notas operacionais

- **Quarto diagnóstico de módulo focado**. Patamar empírico
  forte (P157/P158/P159 + **P160**). Padrão inventariar-primeiro
  consolida-se cumulativamente.
- **Primeira mudança de módulo cross-domínio** desde início
  da série granular. Diversidade ADR-0065 critério #5
  amplia-se: Model focado / Model focado / Model focado /
  Model amplo / **Introspection focado**. Modalidades
  acumuladas: divisão multi-passo / subset selection / par
  acoplado / scope amplo multi-feature / cross-domínio
  diferente.
- **§análise de risco no relatório**: passo diagnóstico baixo
  risco. Manter §análise de risco preserva precedente N=23
  → 24.
- **Política "sem novas reservas" preservada** — recomendações
  em §6 são para validação humana, não compromissos.
- **Auto-aplicação ADR-0065 critério #5**: décima segunda
  aplicação concreta com diversidade cross-domínio nova.
  Patamar consolidado largamente.
- **Recomendação §6**: explicitamente sujeita a validação
  humana. Reservar autonomia humana para a decisão final é
  paridade política P158/P159.
- **ADR-0017 estado factual**: reserva sem ficheiro pré-existente
  per confirmação P159 §1.2 + P159B §3. P160 confirma estado
  e decide se promoção a PROPOSTO via XS administrativo é
  necessária antes de Bloco B (paridade ADR-0062-create
  pendente).
- **Sequência alfabética P160 monótona**: P160 → P160A → P160B
  → ... per padrão regular (não há histórico de buracos como
  P159E).
- **Cross-referência decisão futura**: pós-P160, decisão pode
  ser Bloco A (materialização Introspection) OU promoção
  ADR-0017 (paralela a ADR-0062-create) OU mudança de módulo
  novamente (Layout Fase 3 columns/colbreak).

---

## Pós-passo

Após conclusão de P160:

**Layout fica em 78% inalterado**. **Model agregado fica em
~50% inalterado**. **Tecto Introspection puro documentado**.

**Próxima decisão (validação humana de §6)**:
- Aprovar recomendação §6 → redigir spec do passo concreto
  recomendado (P160A ou ADR-0017-create).
- Redirigir para outro candidato §5 → redigir spec.
- Mudar de módulo (Layout Fase 3 ou outro) se §4/§6 mostrar
  que tecto Introspection puro é trivialmente saturado →
  redigir P161 ou outro como diagnóstico do novo módulo.

ADR-0060 mantém-se IMPLEMENTADO. ADR-0061 mantém-se PROPOSTO.
ADR-0017 estado factual confirmado em §1 (reserva sem ficheiro
documentada). ADR-0062 estado factual mantido (reserva sem
ficheiro).

Padrão granularidade 1-2 features/passo (N=21) NÃO é desafiado
por P160 (passo diagnóstico). Pode ser desafiado por sub-passos
materializáveis seguintes consoante recomendação §6.

**Reservas pendentes** (não criadas neste passo):
- ADR-0017 Introspection runtime adiada — pré-existente.
- ADR-0062 hayagriva — pré-existente.

**Próxima decisão humana**: validação de §4 (tecto Introspection)
e §6 (recomendação) antes de redigir passo seguinte.

**Princípio operacional confirmado**: P160 documenta
empiricamente o tecto Introspection — informação útil mesmo se
escolherem mudar de módulo, porque torna a decisão informada
em vez de arbitrária (paridade princípio P159B).
