# Passo P159C — `Cite.form` variants (Model bibliography+cite sub-passo 2)

Segundo sub-passo substantivo de Bibliography + Cite per
candidato Bloco A do diagnóstico P159B §3.2. Materializa enum
`CitationForm` + field `form: Option<CitationForm>` em
`Content::Cite`. **Décima sexta aplicação consecutiva de
materialização** desde início da série granular P156C.

Refino comportamental + estrutural — expande variant Cite
existente (P159A) com novo field controlado por enum dedicado.
**ADR-0064 Caso A** aplicado para `form` (vanilla
`Smart<CitationForm>` → cristalino `Option<CitationForm>`) —
patamar Caso A cresce N=5 → 6.

**Quebra padrão "estabilidade hash content.rs"** após 11 passos
consecutivos (P156L → P158B). P159C é primeiro a modificar
variant Content desde P159A.

---

## Estado actual antes de começar

- 63 ADRs após P158B (28 EM VIGOR; ADR-0060 IMPLEMENTADO;
  ADR-0062 reserva sem ficheiro mantida).
- Layout: 78% (inalterado). Cobertura arquitectural total 82%.
- Cobertura Model agregada: ~50% (impl + impl⁺ inalterada).
  Cobertura ampla 77% inalterada.
- Hash actual `entities/content.rs`: `ec58d849` (preservado
  em **11 passos consecutivos** P156L → P158B; **quebra
  esperada em P159C** — variant Cite expande field).
- Hash `figure_supplement.rs`: `4426dbc0` (P158B).
- Hash `bib_entry.rs`: `5a2c0ebd` (P159A).
- 1428 tests (lib+integ+diagnostic; workspace 1449); zero
  violations linter.
- 58 variants Content; 48 stdlib funcs.
- Padrões consolidados pós-P158B: granularidade N=15;
  inventariar N=17; Smart→Option Caso A patamar N=5; §análise
  risco N=17; estabilidade hash L0 N=11; subpadrão P155
  cross-feature N=1.

**Diagnóstico P159B** §3.2 (esboço P159C):
- Refino: enum `CitationForm { Normal, Prose, Author, Year }`
  + field `form: Option<CitationForm>` em `Content::Cite`.
- Sem dependência cruzada hard.
- Hash `content.rs` quebrado (variant Cite expande field).
- ADR-0064 Caso A aplicado — patamar Caso A cresce N=5 → 6.
- Layout placeholder: render diferente por form.
- Tests Δ: +10-15.
- Granularidade: M preservado.

**Política "sem novas reservas" preservada** — P159C não cria
reservas para passos pós-P159C.

**Leituras prévias obrigatórias**:
- `00_nucleo/diagnosticos/diagnostico-expansao-159-passo-159b.md`
  — §3.2 esboço P159C.
- `00_nucleo/materialization/typst-passo-159a-relatorio.md` —
  precedente directo (variant Cite original).
- `00_nucleo/adr/typst-adr-0064-smart-para-option-default.md`
  — Caso A definição.
- `00_nucleo/adr/typst-adr-0033-paridade-observavel.md` —
  fundamento para placeholder render diferenciado.
- `00_nucleo/adr/typst-adr-0054-perfil-graded.md` — fundamento
  para subset minimal.
- `01_core/src/entities/content.rs` — variant `Content::Cite`
  actual (P159A).
- `01_core/src/rules/stdlib/structural.rs` — `native_cite`
  actual (P159A).
- `01_core/src/rules/layout/mod.rs` — pattern arm Cite
  actual (placeholder `[key]`).
- `lab/typst-original/crates/typst-library/src/model/cite.rs`
  (vanilla, quarentena) — `CiteForm` enum referência.

---

## Natureza do passo

**Tamanho**: M.

**Justificação**: 1 feature (Cite.form). 1 enum novo + 1
field novo no variant Cite + modificação stdlib + modificação
layout + tests. Sem refactor de tipo existente além de expandir
variant. Reuso de `extract_*` patterns para parse de form.

Granularidade preservada: 1 feature → mantém N=16 do padrão.

**Risco baixo-médio**:
- **Baixo** porque é refino aditivo de variant existente
  (paridade P156F skew; P156L pad refino sides).
- **Médio** porque é primeiro Cite refino + decisão de tipo
  enum dedicado vs Option<String> simples + quebra de padrão
  "estabilidade hash content.rs".
- ADR-0064 Caso A já validado em N=5 aplicações — risco de
  ambiguidade arquitectural baixo.

---

## Decisões já tomadas

- **Enum novo `CitationForm`**:
  ```rust
  // 01_core/src/entities/citation_form.rs (ficheiro novo)
  #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  pub enum CitationForm {
      Normal,  // [key] — default placeholder
      Prose,   // Author (Year)
      Author,  // Author
      Year,    // Year
  }
  ```
  Subset minimal de forms vanilla — Normal/Prose/Author/Year
  são as 4 forms universais. Forms vanilla adicionais
  (`YearOnly`, `Full`, etc.) **diferidas** per ADR-0054
  graded.

- **Field `form` em `Content::Cite`**:
  ```rust
  Cite {
      key:        String,
      supplement: Option<Box<Content>>,
      form:       Option<CitationForm>,  // P159C novo
  }
  ```
  ADR-0064 Caso A: vanilla `Smart<CiteForm>` "auto = computa
  do contexto (default Normal)" → cristalino `Option<CitationForm>`;
  `None` ↔ Auto (resolvido em layout para Normal).

- **Localização**: `entities/citation_form.rs` ficheiro novo
  per padrão `entities/parity.rs` (P156E), `entities/dir.rs`
  (P156I), `entities/bib_entry.rs` (P159A). Enum dedicado em
  ficheiro próprio, não inline em content.rs.

- **`Default` para `CitationForm`**: derivado ou explícito
  como `Normal` (paridade vanilla `CiteForm::Normal` default).
  Permite usar `unwrap_or_default()` em layout.

- **Stdlib `native_cite`**:
  - Aceita `form: auto/none/Str` named opcional.
  - Helper `extract_citation_form` privado parsea `Value::Str`
    para `CitationForm` ("normal" → Normal; "prose" → Prose;
    etc.).
  - Validação: string desconhecida rejeitada com mensagem
    listando forms válidas.

- **Layout placeholder por form**:
  - `Normal` (ou None default): `[key]` (paridade P159A;
    inalterado).
  - `Prose`: lookup entry por key; render `Author (Year)`;
    fallback `[key]` se key não encontrada.
  - `Author`: lookup entry; render `Author`; fallback `[key]`.
  - `Year`: lookup entry; render `Year`; fallback `[key]`.
  - **Lookup é cross-reference Bibliography↔Cite** —
    decisão sobre como executar lookup deferida a sub-passo
    .1 (provavelmente walk single-pass com BibEntries
    coletadas em primeira pass; ou direct lookup se há
    contexto activo de Bibliography).

- **Sem alteração ao algoritmo cross-document**: ADR-0017
  Introspection runtime continua bloqueador para refs
  cross-document. P159C resolve apenas same-document
  via walk.

## Decisões diferidas

- **Forms vanilla adicionais** (`YearOnly`, `Full`, etc.):
  diferidas per ADR-0054 graded. NÃO reservadas.

- **Cross-document forward refs**: bloqueado por ADR-0017.
  Diferido (NÃO reservado).

- **`style` field per Cite** (override de Bibliography style):
  diferido (depende hayagriva).

- **Promoção de `extract_citation_form` a helper público**:
  diferida per política consistente N=3-4.

- **Lookup algorithm exacto** entre Cite.key e Bibliography.entries:
  decisão deferida a sub-passo .1 com base em estado actual
  de walk single-pass de Cite (P159A).

---

## Sub-passos

### .1 Inventário pré-materialização (obrigatório per ADR-0065)

Diagnóstico em
`00_nucleo/diagnosticos/diagnostico-cite-form-passo-159c.md`
com 7 itens canónicos (ADR-0034) + 3 itens específicos para
enum dedicado novo + cross-reference resolution + quebra de
hash:

1. Assinatura vanilla `CiteForm` enum — confirmar forms
   minimais (Normal/Prose/Author/Year); forms diferidos
   (YearOnly/Full/etc. per ADR-0054 graded).
2. Comportamento observável (Cite com form=Prose deve
   renderizar "Author (Year)" se key existe em Bibliography
   mesmo documento).
3. ADR-0064 Caso A confirmado: `Smart<CiteForm>` →
   `Option<CitationForm>` (None ↔ Auto = Normal default).
4. Variants Content existentes a estender: **`Content::Cite`
   expansão de field**. Quebra hash content.rs esperada.
5. Helpers stdlib reusáveis: nenhum directo (parse de string
   para enum custom é trivial); helper novo
   `extract_citation_form`.
6. Limitações aceites (forms vanilla diferidos; cross-document
   refs bloqueados ADR-0017).
7. Tests planeados (form variants happy paths + lookup
   Bibliography + fallback `[key]` quando key não encontrada
   + invalidações — range 10-15 per esboço P159B §3.2).
8. **(Específico enum novo)** Localização do enum:
   `entities/citation_form.rs` ficheiro novo (paridade
   P156E parity.rs, P156I dir.rs, P159A bib_entry.rs).
   Confirmar.
9. **(Específico cross-reference)** Algoritmo de lookup
   Cite.key ↔ Bibliography.entries: walk single-pass com
   collect first pass + lookup second pass? Ou contexto
   activo? Decidir e documentar.
10. **(Específico quebra hash)** Reconhecer explicitamente
    que P159C quebra padrão "estabilidade hash content.rs"
    (11 passos consecutivos terminam). Hash novo a gerar.

### .2 Adicionar enum `CitationForm`

`01_core/src/entities/citation_form.rs` (ficheiro novo):
- Enum per assinatura em §"Decisões já tomadas".
- Derives: `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`.
- Default explícito como `Normal`.
- Métodos básicos: `as_str()` para serialização inversa
  ("Normal"→"normal" para tests/debug).
- `pub mod citation_form;` em `entities/mod.rs`.

### .3 Expandir variant `Content::Cite`

`01_core/src/entities/content.rs`:
- Adicionar field `form: Option<CitationForm>` ao variant
  `Cite { key, supplement, form }`.
- Cobrir todos os 9 sítios pattern-match Content existentes
  (paridade P157A/B/C/P159A):
  - Variant declaration + construtor expandido.
  - `is_empty()`: cite continua sempre `false` (key não-vazia).
  - `plain_text()`: continua emitir `"[{key}]"` ou variação
    por form per layout decisão; verificar se plain_text
    deve seguir layout ou simplificar para `[key]` sempre.
  - `PartialEq`: cobre 3 fields agora.
  - `map_content`: recurse em supplement; preserva key/form.
  - `map_text`: idem.
  - `introspect.rs::materialize_time`: idem.
  - `introspect.rs::walk`: idem.
  - `layout/mod.rs::layout_content`: arm expandido por form.

- Construtor `Content::cite(key, supplement, form)` actualizado.

- **Hash quebra esperada**: aceitar.

### .4 Modificar `native_cite`

`01_core/src/rules/stdlib/structural.rs`:
- Adicionar parsing de `form: Str` named opcional.
- Helper privado `extract_citation_form(val: Option<&Value>)
  -> SourceResult<Option<CitationForm>>`:
  - `None` ou `Value::Auto` → `None`.
  - `Value::Str("normal")` → `Some(Normal)`.
  - `Value::Str("prose")` → `Some(Prose)`.
  - `Value::Str("author")` → `Some(Author)`.
  - `Value::Str("year")` → `Some(Year)`.
  - Outros → erro hard com mensagem listando forms válidas.
- Construtor `Content::cite(key, supplement, form)` chamado
  com novo field.

### .5 Layout placeholder por form

`01_core/src/rules/layout/mod.rs`:
- Pattern arm `Content::Cite { key, supplement, form }`
  expandido:
  - Resolver form: `form.unwrap_or_default()` ou `form.unwrap_or(Normal)`.
  - Lookup entry por key (algoritmo decidido em .1).
  - Match form:
    - `Normal`: render `"[{key}]"` + supplement (paridade
      P159A inalterada).
    - `Prose`: se entry encontrada, render `"{author} ({year})"`
      + supplement; senão fallback `[key]` + supplement.
    - `Author`: se entry encontrada, render `"{author}"` +
      supplement; senão fallback.
    - `Year`: se entry encontrada, render `"{year}"` +
      supplement; senão fallback.

- **`layout_grid` NÃO modificado** (paridade P157A/B/C/P159A).

### .6 Tests

- **Unit tests `CitationForm`** em `entities/citation_form.rs`
  (~3):
  - Constructor cada variant.
  - PartialEq.
  - Default = Normal.

- **Unit tests `Content::Cite` com form** em
  `entities/content.rs` (~4):
  - Constructor com form=Some.
  - Constructor com form=None default.
  - PartialEq cobre 3 fields agora.
  - map_text preserva form.

- **Stdlib tests** (~6):
  - Parse "normal"/"prose"/"author"/"year" cada um.
  - form=auto → None.
  - form=Str inválida rejeitada.

- **Layout E2E tests** em `layout/tests.rs` (~4):
  - `cite_normal_renderiza_placeholder` (regression P159A).
  - `cite_prose_renderiza_author_year_quando_key_existe`.
  - `cite_prose_fallback_placeholder_quando_key_nao_existe`.
  - `cite_author_year_renderizam_correctamente`.

**Δ esperado**: +12-17 tests (alinhado com esboço P159B
§3.2; range 10-15 ligeiramente alargado por enum + 4 forms +
fallback tests).

### .7 Propagação de hashes

`crystalline-lint --fix-hashes .` para:
- Gerar hash inicial de `citation_form.rs` novo.
- Propagar hash novo de `entities/content.rs` (`ec58d849` →
  novo hash).

**Quebra padrão "estabilidade hash content.rs" confirmada**:
**11 passos consecutivos terminam** com P159C. Documentar
no relatório.

Verificar se prompt L0 `content.md` precisa de actualização
(mencionar variants Bibliography/Cite/Cite.form) — decidido
em .1 ou subsequente passo administrativo XS.

---

## Verificação

Numerada para reporte de conclusão:

1. `cargo test` em workspace: **1428 + Δ** tests, zero falhas
   (Δ esperado +12-17).
2. `crystalline-lint`: zero violations.
3. Contagem variants Content: **58** (inalterada — refino
   de variant existente, sem variant novo).
4. Contagem stdlib funcs: **48** (inalterada — `native_cite`
   modificada, não nova).
5. **Enum entity novo `CitationForm`** adicionado a
   `01_core/src/entities/citation_form.rs`.
6. Cobertura Model agregada: ~50% inalterada (refino
   qualitativo). Cobertura ampla pode crescer ligeiramente
   (entrada `cite` ganha refino).
7. Hash actualizado em prompts L0 (`crystalline-lint
   --check-hashes` passa).
8. **Hash `entities/content.rs` quebra padrão de 11 passos**:
   `ec58d849 → novo`. **Primeiro break em série P156L →
   P158B**. Documentado.
9. ADR-0064 Caso A aplicado em `form`; patamar Caso A cresce
   **N=5 → 6** (P156G/H/I + P157B + P159A title/supplement +
   P159C form).
10. Algoritmo lookup Cite↔Bibliography decidido em .1 e
    documentado no relatório.
11. **Sem novas reservas** criadas (paridade política P158).
12. ADR-0017 não promovida (cross-document refs continuam
    diferidos).
13. `layout_grid` original NÃO modificado (paridade P157A/B/C
    + P159A).
14. Tests pré-existentes de Cite (P159A) continuam a passar
    inalterados (regression).

---

## Critério de conclusão

- Verificações 1-14 passam.
- Relatório separado em
  `00_nucleo/materialization/typst-passo-159c-relatorio.md`
  produzido com:
  - Resumo do executado.
  - Confirmação de cada verificação.
  - **§análise de risco** (padrão N=17 → 18; primeira
    aplicação concreta de enum dedicado novo em domínio
    Model + primeira quebra de "estabilidade hash content.rs"
    desde série iniciar).
  - Slope cumulativo Model (mesa P155-P159C).
  - ADR-0061 §"Aplicações cumulativas" anotada com P159C.
  - **Confirmação**: ADR-0064 Caso A patamar **N=6**;
    quebra de padrão hash documentada.
  - **Reset de "estabilidade hash content.rs"**: contador
    reinicia em P159C.

---

## O que pode sair errado

**Cenários gerais**:
- Inventário .1 revela que vanilla `CiteForm` tem mais variants
  além das 4 minimais (e.g. `YearOnly` separado de `Year`)
  → ajustar enum cristalino para incluir; ou diferir e
  documentar como ADR-0054 graded.
- Inventário .1 revela que cross-reference Cite↔Bibliography
  é mais complexo do que walk single-pass (e.g. requer
  Bibliography contextual via Style cascade) → simplificar
  para "Bibliography mais próximo no documento" como heurística;
  documentar.

**Cenários específicos**:
- Helper `extract_citation_form` ter complexidade superior
  (e.g. case-insensitive matching, abreviações tipo "n"
  para "normal") → manter strict matching; case-sensitive
  e literal. Documentar.
- Pattern-match exhaustive falhar fora de `content.rs`
  (paridade P157A/B/C + P159A) → grep por `Content::Cite`
  em todo o crate.
- Tests de Bibliography E2E (P159A) quebrarem por mudança
  de assinatura `Content::cite` → adaptar tests; documentar
  regression.
- Layout fallback `[key]` quando entry não encontrada não
  ser idempotente com Normal — verificar que Normal e
  fallback produzem mesmo output em casos sem entry.
- Lookup Cite↔Bibliography ter ambiguidade quando há
  múltiplas Bibliography no documento — decidir comportamento
  (primeira? última? combinadas?) e documentar.

---

## Notas operacionais

- **Décima sexta aplicação de materialização**. Patamar
  empírico forte. Padrão "0 reformulações mid-passo" preserva-se
  em N=15 aplicações de materialização.
- **§análise de risco no relatório** com peso real (N=17 → 18).
  Primeira quebra de "estabilidade hash content.rs" desde
  série granular iniciar.
- **Quebra padrão hash content.rs**: 11 passos consecutivos
  terminam em P159C. Reset do contador. Padrão emergente
  "passos aditivos preservam hash" ainda preserva-se
  conceptualmente; mas P159C é variant **expansão de field**,
  não variant aditivo puro — distinção mais subtil.
- **ADR-0064 Caso A patamar cresce N=6**: P156G/H/I (Layout)
  + P157B (Model TableCell.x/y) + P159A (Model
  Bibliography.title + Cite.supplement) + **P159C (Model
  Cite.form)**. Diversidade cross-domínio reforçada
  cumulativamente. Caso A é o caso mais aplicado.
- **Enum entity novo `CitationForm`**: terceira aplicação
  de "tipo entity em ficheiro próprio" desde série granular
  (`Sides<T>` P156C; `Parity` P156E; `Dir` P156I; `BibEntry`
  P159A; agora `CitationForm`).
- **Política "sem novas reservas" preservada** — refinos
  futuros (forms adicionais, cross-document refs) permanecem
  candidatos NÃO-reservados.
- **Subpadrão emergente**: "expansão de field em variant
  Content já existente". Precedentes: P156F skew (expandiu
  TransformMatrix); P156L pad refino sides (expandiu Pad).
  P159C é terceiro do tipo; promoção a subpadrão consolidado
  diferida até N=4.

---

## Pós-passo

Após conclusão de P159C:

**Layout fica em 78% inalterado**. **Model agregado fica em
~50% inalterado** (refino estrutural sem mover counts).
**Hash `entities/content.rs` quebra padrão de 11 passos** —
contador hash reinicia.

**Próxima decisão** (sem candidata pré-acordada — política
"sem novas reservas"):

- ADR-0062-create (XS administrativo; ainda pendente após
  P158B + P159C; pode ser feito a qualquer momento).
- Continuar Bloco A (P159D BibEntry fields adicionais; P159F
  numbering numérico Bibliography).
- Mudança de módulo (Introspection P160).
- Outras direcções pendentes.

ADR-0060 mantém-se `IMPLEMENTADO`. ADR-0061 mantém-se
`PROPOSTO`. ADR-0062 mantém-se reserva sem ficheiro.

Padrão granularidade 1-2 features/passo (N=16 com P159C se
fechar sem reformulação) **NÃO** é formalizado em ADR.
Continua candidato.

**Pausa natural após P159C — Cite ganha forms variant; ADR-0064
Caso A patamar N=6 com diversidade cross-domínio reforçada;
política "sem novas reservas" preservada; primeiro reset de
contador hash content.rs desde série granular iniciar.
Decisão humana sobre próxima direcção tem máxima informação.**
