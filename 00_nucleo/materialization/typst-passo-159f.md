# Passo P159F — Numbering numérico Bibliography (Model bibliography+cite sub-passo 4)

Quarto sub-passo substantivo de Bibliography + Cite per
candidato Bloco A do diagnóstico P159B §3.5. Materializa
counter local de bib entries para render `[1]`/`[2]`/`[3]` em
Cite (substituindo ou co-existindo com placeholder `[key]`
default per decisão deferida em .1). **Décima nona aplicação
consecutiva de materialização** desde início da série granular
P156C.

**Último candidato do Bloco A** após P158B/C/P159C/D — após
este passo, Bloco A esgota-se. Próximas direcções pós-P159F
exigem Bloco B (com ADR-0062 promovida), Bloco C cross-módulo,
ou mudança de módulo.

---

## Estado actual antes de começar

- 63 ADRs após P158C (28 EM VIGOR; ADR-0060 IMPLEMENTADO;
  ADR-0062 reserva sem ficheiro mantida).
- Layout: 78% (inalterado). Cobertura arquitectural total 82%.
- Cobertura Model agregada: ~50% (impl + impl⁺ inalterada).
  Cobertura ampla 77% inalterada.
- Hash actual `entities/content.rs`: `ec58d849` (preservado
  em **14 passos consecutivos** P156L → P158C via L0-baseline).
- Hash `entities/bib_entry.rs`: `5a2c0ebd` (preservado).
- Hash `entities/citation_form.rs`: `677849cb` (P159C).
- Hash `entities/counter_state.rs`: `4b8e4f02` (P158B/P159C).
- 1453 tests (lib+integ+diagnostic; workspace 1474); zero
  violations linter.
- 58 variants Content; 48 stdlib funcs.
- Padrões consolidados pós-P158C: granularidade N=18;
  inventariar N=20; Smart→Option Caso A patamar N=7
  (43/57 Layout/Model); §análise risco N=20; estabilidade
  hash L0 content.rs N=14; tipo entity em ficheiro próprio
  N=5; infraestrutura state lookup N=2; P155 cross-feature
  N=1; refino tipo entity sem alteração Content N=1; refactor
  de field para Option N=1.

**Diagnóstico P159B** §3.5 (esboço P159F):
- Refino: counter local de bib entries; render `[1]`/`[2]`/
  `[3]` em vez de `[key]`.
- Sem dependência cruzada hard (counter local single-pass).
- Hash `content.rs` preservado per L0-baseline (refino
  layout + introspect; sem alteração ao variant Content).
- Tests Δ: +10-15.
- Granularidade: M preservado.

**Política "sem novas reservas" preservada** — P159F não
cria reservas para passos pós-P159F.

**Leituras prévias obrigatórias**:
- `00_nucleo/diagnosticos/diagnostico-expansao-159-passo-159b.md`
  — §3.5 esboço P159F.
- `00_nucleo/materialization/typst-passo-159a-relatorio.md` —
  precedente directo (Cite render placeholder `[key]`;
  walk single-pass).
- `00_nucleo/materialization/typst-passo-159c-relatorio.md` —
  precedente Cite refino estrutural (form variants;
  `state.bib_entries` infraestrutura).
- `00_nucleo/materialization/typst-passo-159d-relatorio.md` —
  precedente BibEntry fields adicionais.
- `00_nucleo/materialization/typst-passo-157a-relatorio.md` —
  precedente counters por kind (figure; pattern de counter
  local single-pass per ADR-0017 graded).
- `00_nucleo/adr/typst-adr-0064-smart-para-option-default.md`
  — Caso A se aplicável (Bibliography.style ou similar).
- `00_nucleo/adr/typst-adr-0033-paridade-observavel.md` —
  fundamento para style "numeric" como default minimal.
- `00_nucleo/adr/typst-adr-0054-perfil-graded.md` — fundamento
  para subset de styles (numeric apenas; outros diferidos).
- `01_core/src/entities/counter_state.rs` — `state.bib_entries`
  populado por walk (P159C); pattern de counter por kind a
  replicar.
- `01_core/src/rules/introspect.rs` — counters por kind +
  walk Bibliography.
- `01_core/src/rules/layout/mod.rs` — render Cite actual
  (placeholder `[key]` ou variantes form per P159C).
- `lab/typst-original/crates/typst-library/src/model/cite.rs`
  + bibliography numbering — referência paridade.

---

## Natureza do passo

**Tamanho**: M.

**Justificação**: 1 feature (numbering numérico). Counter
local em `CounterState` ou map separado + população por walk
em arm Bibliography + lookup em arm Cite + decisão de
default/coexistência com placeholder `[key]`. Sem nova
estrutura entity. Refino comportamental análogo a P158B
(supplement por lang) + refino estrutural de state lookup
(N=2 → 3).

Granularidade preservada: 1 feature → mantém N=18 do padrão.

**Risco baixo-médio**:
- **Baixo** porque é refino comportamental + extensão de
  infraestrutura state lookup (precedente N=2 P158B `state.lang`
  + P159C `state.bib_entries`).
- **Médio** porque envolve **decisão arquitectural-chave** sobre
  comportamento default (numeric substitui `[key]`? ou
  co-existe via Bibliography.style field novo?). Decisão
  deferida a sub-passo .1.

---

## Decisões já tomadas

- **Counter local single-pass**: paridade pattern P75 figure
  counters por kind + P157A figure-table counters. Counter
  vive em `CounterState` (paridade `state.bib_entries`
  P159C). **NÃO usa Introspection runtime cross-document**
  (ADR-0017 continua bloqueador para refs cross-document).

- **Algoritmo de numbering**:
  - Walk arm Bibliography popula counter assignment:
    `bib_numbers: HashMap<String, u32>` (key → número 1-based).
  - Cada entry recebe número na ordem de aparecimento na
    primeira Bibliography do documento.
  - Entries em Bibliographies adicionais continuam a numeração
    (paridade decisão multi-Bibliography P159C — concatenação
    na ordem de aparecimento).
  - Lookup em arm Cite: `state.bib_numbers.get(key)` → render
    `[N]` se encontrado.

- **Comportamento default**: **decisão deferida a .1**. Três
  opções:
  - **Opção A — substituir `[key]` por `[N]` sempre**:
    paridade vanilla com style "numeric" default; quebra
    backwards compat dos tests P159A/C que verificam `[key]`.
  - **Opção B — co-existir via Bibliography.style field novo**:
    `Bibliography.style: Option<BibliographyStyle>` com enum
    `Numeric`/`KeyPlaceholder`; default `KeyPlaceholder`
    preserva backwards compat. Adiciona variant field.
  - **Opção C — co-existir via Cite.form interaction**:
    quando `Cite.form == Normal` (ou None default), usa numeric
    se `state.bib_numbers` populado; senão `[key]`. Sem alteração
    de variant.

  Cada opção com pros/cons documentados em §"Sub-passo .1".

- **Sem alteração ao variant `Content::Cite`**: refactor cosmético
  ao layout Cite arm (paridade P158C abordagem). Cite.form
  variant inalterado.

- **Multi-Bibliography**: numeração contínua (1, 2, 3, ...,
  N entries totais) ou independente por Bibliography? **Decisão
  deferida a .1**. Pré-decisão: contínua per padrão concatenação
  P159C.

## Decisões diferidas

- **Comportamento default** (Opção A/B/C): decidida em .1.

- **Numeração multi-Bibliography**: contínua vs independente.
  Decidida em .1.

- **Bibliography.style field novo** (Opção B): só se Opção B
  for escolhida. NÃO reservado.

- **Outras styles** (autor-ano, IEEE, APA, Chicago): NÃO
  reservadas. Bloco B (hayagriva) requerido.

- **Cite numbering em outras forms** (P159C: Prose, Author,
  Year): inalterado em P159F. `Prose` continua a renderizar
  "Author (Year)" se entry encontrada; só `Normal/None`
  ganha numeração. Confirmar em .1.

- **Promoção de helper de numbering a `pub(super)`**: diferida
  per política consistente N=3-4.

---

## Sub-passos

### .1 Inventário pré-materialização (obrigatório per ADR-0065)

Diagnóstico em
`00_nucleo/diagnosticos/diagnostico-bibliography-numbering-passo-159f.md`
com 7 itens canónicos (ADR-0034) + 3 itens específicos para
decisão arquitectural-chave default + multi-Bibliography +
interação Cite.form:

1. Assinatura vanilla `BibliographyElem.style` — confirmar se
   é `Smart<Style>` ou enum directo; styles minimais
   (`numeric`/`alphanumeric`/`author-date`/etc.).
2. Comportamento observable vanilla (style numeric → cite
   renderiza `[1]`; style author-date → cite renderiza
   `(Author, Year)`).
3. ADR-0064 caso aplicável: depende decisão Opção B (Caso A
   se Bibliography.style for `Smart<Style>`) vs Opção A/C
   (não aplicável directamente).
4. Variants Content existentes a estender: depende decisão
   Opção B (Bibliography.style field novo) vs Opção A/C
   (sem alteração).
5. Helpers stdlib reusáveis: `extract_bib_entries` (P159A) +
   `extract_citation_form` (P159C). Helper novo se Opção B
   escolhida: `extract_bibliography_style`.
6. Limitações aceites (apenas style numeric per ADR-0054
   graded; outros styles diferidos pendentes Bloco B).
7. Tests planeados (counter populado correctamente em walk +
   render numbered em layout + multi-Bibliography contínua/
   independente per decisão + interação Cite.form per decisão
   — range 10-15 per esboço P159B §3.5).
8. **(Específico decisão arquitectural-chave default)**
   Documentar Opção A/B/C com pros/cons:
   - **Opção A** — substituir `[key]` por `[N]` sempre:
     - Pro: paridade vanilla style numeric default; comportamento
       imediato sem field novo.
     - Con: quebra tests P159A/C que verificam `[key]` literal;
       adaptações cascading necessárias.
   - **Opção B** — Bibliography.style field novo:
     - Pro: configurável; backwards compat preservada (default
       KeyPlaceholder); extensível para outros styles futuros.
     - Con: alteração estrutural Bibliography variant; quebra
       hash content.rs (per regra L0-baseline a confirmar — pode
       preservar se prompt não mencionar style); refactor cascading
       em construtores.
   - **Opção C** — Cite.form interaction:
     - Pro: sem alteração estrutural; aproveita infraestrutura
       P159C; default mantém backwards compat se `state.bib_numbers`
       não populado.
     - Con: comportamento implícito (depende ordem walk); pode
       ser inesperado.
   **Recomendação**: documentar e escolher. Pré-recomendação:
   **Opção C** (sem alteração estrutural; aproveita N=2
   subpadrão state lookup; default backwards compat trivial).
9. **(Específico multi-Bibliography numbering)** Decisão
   contínua vs independente:
   - Contínua: paridade vanilla; mais simples; multi-Bibliography
     comportamento intuitivo "numeração global".
   - Independente: cada Bibliography tem própria numeração
     1..N; mais flexível.
   Pré-recomendação: contínua per paridade vanilla.
10. **(Específico interação Cite.form)** Confirmar
    comportamento por form (P159C):
    - `Normal/None`: numbered se possível, senão `[key]`.
    - `Prose`: "Author (Year)" se entry encontrada, senão
      `[key]` (P159C inalterado).
    - `Author`: "Author" se entry encontrada, senão `[key]`.
    - `Year`: "Year" se entry encontrada, senão `[key]`.
    Recomendação: numeração só substitui `[key]` em `Normal/
    None` (preserva forms diferenciadas).

### .2 Adicionar field `bib_numbers` em `CounterState`

`01_core/src/entities/counter_state.rs`:
- Adicionar field `pub bib_numbers: HashMap<String, u32>`
  (paridade aditiva `state.lang` P158B + `state.bib_entries`
  P159C).
- Default `HashMap::new()`.
- Populado por walk em arm Bibliography (sub-passo .3).

### .3 Modificar walk em `introspect.rs`

`01_core/src/rules/introspect.rs`:
- Walk arm `Content::Bibliography { entries, ... }`:
  - Iterate entries em ordem.
  - Atribuir número 1-based: `state.bib_numbers.insert(entry.key,
    state.bib_numbers.len() as u32 + 1)`.
  - Multi-Bibliography contínua: `len()` reflecte total
    cumulativo. Independente: precedente `state.bib_entries`
    P159C continua independente.
  - Decisão final per .1 §9.

### .4 Modificar render Cite em `layout/mod.rs`

`01_core/src/rules/layout/mod.rs`:
- Pattern arm `Content::Cite { key, supplement, form }`:
  - Resolver form: `form.unwrap_or_default()` ou
    `form.unwrap_or(Normal)`.
  - **Para form `Normal/None`** per decisão Opção C:
    - Lookup `state.bib_numbers.get(key)`.
    - Se encontrado: render `format!("[{}]", n)` + supplement.
    - Senão: render `format!("[{}]", key)` + supplement
      (paridade P159A backwards compat).
  - **Para forms `Prose/Author/Year`**: comportamento P159C
    inalterado.

- **`layout_grid` NÃO modificado** (paridade P157A/B/C +
  P159A/C/D).

### .5 Tests

- **Unit tests `CounterState`** em `entities/counter_state.rs`
  ou `entities/counter_state_tests.rs` (~2):
  - Constructor default `bib_numbers` empty.
  - Insertion preserva ordem.

- **Stdlib tests** em `stdlib/mod.rs` (~3):
  - Bibliography com 3 entries + Cite usando entries → numbered
    em walk.
  - Multi-Bibliography contínua (decisão .1 §9).
  - Cite.form Prose ainda renderiza "Author (Year)" sem
    numbering (P159C preservado).

- **Layout E2E tests** em `layout/tests.rs` (~5-7):
  - `layout_cite_normal_renderiza_numero_quando_bib_populada`
    — `figure(...)` antes; `bibliography(...)` depois; Cite
    primeiro.
  - `layout_cite_normal_fallback_placeholder_quando_bib_vazia`
    — Cite sem Bibliography precedente continua a renderizar
    `[key]`.
  - `layout_cite_normal_multiple_entries_numeradas_em_ordem`
    — 3 entries; cite primeira → `[1]`; cite segunda → `[2]`.
  - `layout_cite_form_prose_inalterada_com_bib_numerada`
    (regression P159C).
  - `layout_cite_unknown_key_fallback_placeholder` — Cite
    com key não em Bibliography → `[unknown_key]`.

**Δ esperado**: +10-15 tests (alinhado com esboço P159B
§3.5).

### .6 Propagação de hashes

`crystalline-lint --fix-hashes .`:
- `counter_state.rs` hash: per regra L0-baseline preservado
  (campo aditivo via doc-comment); confirmar.
- `content.rs` hash: preservado `ec58d849` (15º passo
  consecutivo se confirmado).
- `introspect.rs`/`layout/mod.rs`: refactor interno; preserva
  L0 a menos que prompt L0 mencione estrutura específica.

---

## Verificação

Numerada para reporte de conclusão:

1. `cargo test` em workspace: **1453 + Δ** tests, zero falhas
   (Δ esperado +10-15).
2. `crystalline-lint`: zero violations.
3. Contagem variants Content: **58** (inalterada — refino
   layout/introspect).
4. Contagem stdlib funcs: **48** (inalterada — sem stdlib
   nova).
5. **Hash `entities/content.rs` preservado** `ec58d849` —
   **15º passo consecutivo** via L0-baseline interpretation.
6. Decisão arquitectural-chave default (Opção A/B/C) registada
   no relatório §"Decisões tomadas em .1".
7. Decisão multi-Bibliography (contínua/independente) registada.
8. Decisão interação Cite.form registada (P159C preservado).
9. Tests pré-existentes Cite Normal (P159A/C) passam:
   - `[key]` quando Bibliography vazia ou Cite key não
     encontrada (regression).
   - `[N]` quando Bibliography populada e key encontrada
     (novo comportamento P159F).
10. Tests pré-existentes Cite forms Prose/Author/Year (P159C)
    passam inalterados (regression).
11. Multi-Bibliography contínua (ou independente per decisão)
    funciona correctamente em E2E test.
12. **Sem novas reservas** criadas (paridade política P158).
13. ADR-0017 não promovida (counter single-pass viável; sem
    cross-document refs).
14. `layout_grid` original NÃO modificado (paridade P157A/B/C
    + P159A/C/D).

---

## Critério de conclusão

- Verificações 1-14 passam.
- Relatório separado em
  `00_nucleo/materialization/typst-passo-159f-relatorio.md`
  produzido com:
  - Resumo do executado.
  - Confirmação de cada verificação.
  - **§análise de risco** (padrão N=20 → 21).
  - Slope cumulativo Model (mesa P155-P159F).
  - ADR-0061 §"Aplicações cumulativas" anotada com P159F.
  - **Confirmação**: estabilidade hash L0 content.rs N=14 →
    15; subpadrão #15 "infraestrutura state lookup" cresce
    N=2 → 3.
  - **Decisão arquitectural-chave** (Opção A/B/C) documentada
    com justificação multi-critério.
  - **Bloco A do diagnóstico P159B esgotado** após P159F —
    marca conceptual importante; próximas direcções exigem
    Bloco B (hayagriva), Bloco C (cross-módulo) ou mudança
    de módulo.

---

## O que pode sair errado

**Cenários gerais**:
- Inventário .1 revela que vanilla `BibliographyElem.style`
  é tipo complexo (e.g. CSL style file path) → simplificar
  para enum `numeric` apenas per ADR-0054 graded; documentar.
- Decisão Opção A/B/C ter ambiguidade não-resolvida em .1 →
  escalar decisão antes de avançar; pré-recomendação Opção C
  serve como fallback se decisão escalada não for tomada.

**Cenários específicos**:
- Walk multi-Bibliography duplicar keys (mesmo key em duas
  Bibliographies) → decisão: primeiro vence (paridade
  HashMap insert; documentar).
- Cite com key não em Bibliography → fallback `[key]` (P159A
  preservado).
- Cite.form Normal interagir mal com decisão Opção C (e.g.
  numeração não acontecer porque walk não popula antes do
  layout) → confirmar ordem walk → layout em .1; pode exigir
  refactor pequeno de pipeline.
- Tests E2E mais sensíveis a ordem de items no documento
  (Cite antes de Bibliography vs depois) → documentar
  comportamento esperado em ambos os casos.
- L0-baseline NÃO preservar hash `content.rs` se layout arm
  Cite for documentado em prompt — verificar prompt
  `content.md` actual antes de confirmar.

---

## Notas operacionais

- **Décima nona aplicação de materialização**. Patamar empírico
  forte. Sem reformulação esperada.
- **§análise de risco no relatório** com peso real (N=20 → 21).
  Decisão arquitectural-chave (Opção A/B/C) eleva risco
  marginalmente vs refactors cosméticos anteriores; sem
  exceder previsão.
- **Subpadrão #15 "infraestrutura state lookup"** cresce N=2
  → 3 (P158B `state.lang` + P159C `state.bib_entries` +
  **P159F `state.bib_numbers`**). Patamar moderado;
  candidato a formalização ADR meta se atingir N=4-5.
- **Helper de numbering**: trivial inline (insert na walk +
  lookup na layout); sem helper novo significativo.
- **ADR-0064**: aplicabilidade depende decisão Opção A/B/C:
  - Opção A: NÃO aplicável (substitui directo).
  - Opção B: SIM aplicável (Caso A para style; patamar N=7
    → 8).
  - Opção C: NÃO aplicável (sem field novo).
  Pré-recomendação Opção C → ADR-0064 patamar inalterado.
- **Política "sem novas reservas" preservada** — outras styles
  (autor-ano, IEEE, APA, Chicago) permanecem candidatos
  NÃO-reservados; pendentes Bloco B (hayagriva).
- **Marca conceptual**: P159F **esgota Bloco A** do diagnóstico
  P159B. Pós-P159F, Model puro está saturado per recomendação
  do diagnóstico. Próximas direcções:
  - Bloco B (hayagriva): exige ADR-0062 promovida + 5 sub-passos
    (P159G/H/I/J + ADR-0062-create XS).
  - Bloco C (cross-módulo): refactor multi-region L+ (DEBT-34e
    + DEBT-56) ou Introspection P160.
  - Refinos Model fora do Bloco A original (e.g. mais langs
    em supplement; url/doi em BibEntry; etc.).
  - Mudança de módulo.

---

## Pós-passo

Após conclusão de P159F:

**Layout fica em 78% inalterado**. **Model agregado fica em
~50% inalterado** (refino layout/introspect). **Hash
`entities/content.rs` provavelmente preservado** (15º passo
consecutivo via L0-baseline).

**Bloco A do diagnóstico P159B esgota-se** — marca conceptual
importante. **Tecto Model puro estimado em P159B (~55-60%)
atingido empiricamente** com cobertura ampla 24 entradas
parciais.

**Próxima decisão** (sem candidata pré-acordada — política
"sem novas reservas"):

- **ADR-0062-create** — XS administrativo; desbloqueia Bloco B.
- **Bloco B**: começar com P159G (Cargo.toml + crystalline.toml
  hayagriva) após ADR-0062 PROPOSTO; depois P159H (hayagriva
  integration) → P159I (CSL APA) → P159J (CSL adicionais).
- **Bloco C**: refactor multi-region L+ (DEBT-34e + DEBT-56)
  ou Introspection P160 (mudança de módulo).
- **Refinos Model fora Bloco A**: mais langs em
  `figure_supplement_for_lang`; `url`/`doi` em BibEntry; etc.
- **Mudança de módulo**: Introspection P160 ou Layout Fase 3
  (columns/colbreak).
- **Passos administrativos XS**: actualizar L0 prompt
  `content.md` mencionando variants Bibliography/Cite/Cite.form;
  promover ADR-0060 a R1; ADR meta saturação ADR-0064.

ADR-0060 mantém-se `IMPLEMENTADO`. ADR-0061 mantém-se
`PROPOSTO`. ADR-0062 mantém-se reserva sem ficheiro.

Padrão granularidade 1-2 features/passo (N=18 com P159F se
fechar sem reformulação) **NÃO** é formalizado em ADR.
Continua candidato.

**Pausa natural após P159F — Bibliography ganha numbering
numérico; Bloco A esgotado; tecto Model puro atingido (~55-60%
estimado com 24 entradas parciais). Decisão humana sobre
próxima direcção tem máxima informação acumulada — informação
útil para escolher entre Bloco B/C, refinos não-listados,
mudança de módulo, ou passos administrativos.**
