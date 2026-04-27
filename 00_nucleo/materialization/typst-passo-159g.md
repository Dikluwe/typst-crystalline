# Passo P159G — `BibEntry` 6 fields restantes (refino família 159 fora Bloco A)

Refino estrutural de tipo entity `BibEntry` adicionando os 6
fields restantes mais comuns de `hayagriva::Entry` (`editor`,
`series`, `note`, `isbn`, `location`, `organization`). **Vigésima
primeira aplicação consecutiva de materialização** desde início
da série granular P156C.

**Segundo sub-passo família 159 fora Bloco A** após P159E (par
url+doi). Pattern P159D replicado fielmente pela terceira vez
— consolida subpadrão #16 ("refino tipo entity sem alteração
Content") a **N=3** (atinge limiar formalização N=3-4).

Per decisão humana pós-P159E: 6 fields num passo M, paridade
P159D.

---

## Estado actual antes de começar

- 63 ADRs após P159E (28 EM VIGOR; ADR-0060 IMPLEMENTADO;
  ADR-0062 reserva sem ficheiro mantida).
- Layout: 78% (inalterado). Cobertura arquitectural total 82%.
- Cobertura Model agregada: ~50% (impl + impl⁺ inalterada).
  Cobertura ampla 77% inalterada.
- Hash actual `entities/content.rs`: `ec58d849` (preservado em
  **16 passos consecutivos** P156L → P159E via L0-baseline).
- Hash `entities/bib_entry.rs`: `5a2c0ebd` (P159A; preservado
  P159D+P159E via L0-baseline).
- Hash `entities/citation_form.rs`: `677849cb` (P159C).
- Hash `entities/counter_state.rs`: `4b8e4f02` (P158B/P159C/F).
- 1469 tests (lib+integ+diagnostic; workspace 1490); zero
  violations linter.
- 58 variants Content; 48 stdlib funcs.
- BibEntry: **10 fields** (4 obrigatórios + 6 opcionais —
  P159A 4 + P159D 4 + P159E 2).
- Padrões consolidados pós-P159E: granularidade N=20;
  inventariar N=22; Smart→Option Caso A patamar N=7
  (43/57 Layout/Model); §análise risco N=22; estabilidade
  hash L0 content.rs N=16; tipo entity em ficheiro próprio
  N=5; infraestrutura state lookup N=3 (limiar formalização);
  P155 cross-feature N=1; refino tipo entity sem alteração
  Content N=2; refactor de field para Option N=1; helper
  `optional_str` cumulativo N=4 (limiar promoção).

**Decisão de scope P159G**: 6 fields restantes num único passo
M, paridade P159D. Justificação: granularidade aceitável + 1
feature lógica ("completar fields BibEntry comuns") + replica
subpadrão #16 N=2→3 atingindo limiar formalização.

**Decisão de identificador**: P159G porque P159F já existe
(numbering numérico, último Bloco A) e P159E foi preenchido por
url+doi. Família 159 sub-passos: A (par acoplado) → B
(diagnóstico amplo) → C (Cite.form) → D (BibEntry 4 fields) → F
(numbering) → E (url+doi) → **G (6 fields restantes)**.

Sequência alfabética não-monótona (E veio depois de F) é facto
histórico registado; preserva slot E para refinos família 159
que surgiram após P158C ocupar identificador alternativo.

**Política "sem novas reservas" preservada** — P159G não cria
reservas para passos pós-P159G.

**Leituras prévias obrigatórias**:
- `00_nucleo/materialization/typst-passo-159d-relatorio.md` —
  pattern original (4 fields adicionais; helper `optional_str`
  inline; builder fluente).
- `00_nucleo/materialization/typst-passo-159e-relatorio.md` —
  pattern replicado (par url+doi; consolidou subpadrão #16
  N=1→2).
- `00_nucleo/diagnosticos/diagnostico-bibentry-fields-passo-159d.md`
  §9.3 — alternativas avaliadas (incluindo estes 6 fields)
  com justificação para diferimento original.
- `00_nucleo/adr/typst-adr-0054-perfil-graded.md` — fundamento
  para subset minimal (sem validation; sem semântica
  estruturada).
- `00_nucleo/adr/typst-adr-0033-paridade-observavel.md` —
  fundamento para layout APA-like.
- `01_core/src/entities/bib_entry.rs` — struct actual P159A+
  P159D+P159E com 10 fields.
- `01_core/src/rules/stdlib/structural.rs` — `extract_bib_entries`
  helper actual + `optional_str` inline.
- `01_core/src/rules/layout/mod.rs` — `format_bib_entry`
  helper privado P159D+P159E.
- `lab/typst-original/crates/typst-library/src/model/bibliography.rs`
  + `hayagriva::Entry` (vanilla, quarentena) — referência para
  semântica dos 6 fields.

---

## Natureza do passo

**Tamanho**: M.

**Justificação**: 1 feature lógica (completar fields BibEntry
comuns). Pattern idêntico a P159D — 6 fields Optional<String>.
Modificação trivial em `extract_bib_entries` para parse de 6
fields novos (reuso do helper inline `optional_str` já criado
em P159D — agora N=4 → 10 usos cumulativos). Modificação em
`format_bib_entry` para concatenação condicional APA-like
extendida. Tests ~8-12.

**P159D era S+** com 4 fields; **P159E era M** com 2 fields
(par natural com decisões cosméticas). **P159G é M** com 6
fields porque:
- 6 fields têm semântica diversa (editor/series/note/isbn/
  location/organization) — cada um com decisão de ordem layout
  e formato.
- Decisões cosméticas mais elaboradas que P159E.
- Replica subpadrão #16 pela terceira vez — consolida patamar.
- Δ tests +8-12 maior que P159D (+8) e P159E (+8) por causa
  de cobertura mais ampla.

Granularidade preservada: 1 feature lógica → mantém N=20 do
padrão.

**Risco baixo**:
- Pattern validado N=2 (P159D + P159E).
- ADR-0064 NÃO aplicável (Optional<String> directo, paridade
  P159D/E).
- Tests pré-existentes (P159A+P159D+P159E) preservam-se via
  fields novos default `None`.
- Decisões cosméticas em .1 sem impacto estrutural.

---

## Decisões já tomadas

- **6 fields adicionais Optional<String>**:
  ```rust
  pub struct BibEntry {
      // P159A:
      pub key:          String,
      pub author:       String,
      pub title:        String,
      pub year:         u32,
      // P159D:
      pub volume:       Option<String>,
      pub pages:        Option<String>,
      pub journal:      Option<String>,
      pub publisher:    Option<String>,
      // P159E:
      pub url:          Option<String>,
      pub doi:          Option<String>,
      // P159G:
      pub editor:       Option<String>,
      pub series:       Option<String>,
      pub note:         Option<String>,
      pub isbn:         Option<String>,
      pub location:     Option<String>,
      pub organization: Option<String>,
  }
  ```
  Total **16 fields** (4 obrigatórios + 12 opcionais).

- **Tipos `Option<String>`**: paridade P159D/E. Sem validation
  (ISBN tem checksum mas validation diferida per ADR-0054
  graded). Sem semântica estruturada (e.g. editor como
  separate person record diferido).

- **Default `None`**: paridade P159D/E. Backwards compat
  trivial via fields novos default `None` — `BibEntry::new(...)`
  original com 4 args + builder fluente continua a funcionar.

- **Builder pattern fluente extendido**:
  ```rust
  impl BibEntry {
      pub fn with_editor(mut self, editor: impl Into<String>) -> Self {...}
      pub fn with_series(mut self, series: impl Into<String>) -> Self {...}
      pub fn with_note(mut self, note: impl Into<String>) -> Self {...}
      pub fn with_isbn(mut self, isbn: impl Into<String>) -> Self {...}
      pub fn with_location(mut self, location: impl Into<String>) -> Self {...}
      pub fn with_organization(mut self, organization: impl Into<String>) -> Self {...}
  }
  ```
  Paridade pattern P159D/E.

- **Helper `extract_bib_entries` extendido**:
  - Reuso do helper inline `optional_str` privado (P159D/E).
  - Adicionar 6 fields opcionais à parsing chain.
  - **Total `optional_str` usos cumulativos: 4 (P159D) + 2
    (P159E) + 6 (P159G) = 12 usos**. Limiar promoção a
    helper público (N=3-4) ultrapassado largamente — promoção
    diferida a passo administrativo XS NÃO reservado.

- **Layout `format_bib_entry` extendido**:
  - Concatenação condicional APA-like.
  - **Ordem deferida a .1**. Pré-decisão preliminar baseada
    em paridade APA:
    - editor: depois de title (`title (Ed. editor).`); ou
      antes do title (`Ed. editor. title`) — alternativas
      em .1.
    - series: depois de title (`title. (series)`).
    - location: antes de publisher (`location: publisher`).
    - isbn: ao fim, antes ou depois de doi (`isbn:XXX,
      doi:YYY`).
    - note: ao final entre `[note]`.
    - organization: substitutivo a publisher se publisher
      ausente, ou separado.

  Decisões cosméticas em .1 com matriz multi-critério.

## Decisões diferidas

- **Ordem layout dos 6 fields**: a decidir em .1 com paridade
  vanilla APA + matriz multi-critério se opções concorrentes.

- **Formato editor**: `Ed. {editor}` vs `{editor} (Ed.)` vs
  `Edited by {editor}`. Pré-decisão: `Ed. {editor}` per APA
  compactness.

- **Formato series**: `{series}` literal vs `({series})`
  parenteses. Pré-decisão: parenteses per APA distinção.

- **Formato location**: `{location}:` antes de publisher
  (`{location}: {publisher}`). Pré-decisão: este formato.

- **Formato isbn**: `isbn:{isbn}` vs `ISBN {isbn}`. Pré-decisão:
  `isbn:{isbn}` per paridade doi (P159E lowercase prefix).

- **Formato note**: `[{note}]` brackets vs `({note})` parenteses.
  Pré-decisão: brackets per distinção visual.

- **Formato organization**: tratamento como publisher
  alternativo. Pré-decisão: usar organization se publisher
  ausente; senão substituir publisher pela organization
  (e.g. tech reports).

- **Promoção `optional_str` a helper público**: diferida em
  passo administrativo XS NÃO reservado (limiar N=12 cumulativos
  ultrapassado largamente).

- **Restantes fields BibEntry vanilla** (`booktitle`/`address`/
  `chapter`/`type`/`institution`/etc.): NÃO reservados.
  Candidatos a refinos futuros se prioritários.

- **Validation** (ISBN checksum, location codes, etc.): diferida
  per ADR-0054 graded.

---

## Sub-passos

### .1 Inventário pré-materialização (obrigatório per ADR-0065)

Diagnóstico em
`00_nucleo/diagnosticos/diagnostico-bibentry-restantes-passo-159g.md`
com 7 itens canónicos (ADR-0034) + 2 itens específicos para
ordem layout + formato (paridade P159E):

1. Assinatura vanilla `hayagriva::Entry` para os 6 fields —
   confirmar Option<String> ou estruturas mais complexas.
2. Comportamento observable vanilla (formato APA específico
   para cada field; confirmar paridade).
3. ADR-0064 caso aplicável: NÃO directamente (Optional<String>
   directo, paridade P159D/E).
4. Variants Content existentes a estender: nenhum. `BibEntry`
   refino estrutural (paridade P159D/E).
5. Helpers stdlib reusáveis: `optional_str` inline P159D
   (cumulativo N=4 → 10 usos pós-P159G).
6. Limitações aceites (sem validation; semântica estruturada
   diferida; restantes vanilla fields diferidos).
7. Tests planeados (constructor com fields novos + parse stdlib
   + render layout extendido + regression P159A+P159D+P159E
   sem fields — range 8-12).
8. **(Específico ordem layout)** Confirmar paridade vanilla
   APA para ordem dos 6 fields:
   - editor: posição relativa a author/title.
   - series: posição relativa a title.
   - location: posição relativa a publisher.
   - isbn: posição relativa a doi/url.
   - note: posição final ou intercalar.
   - organization: substitutivo ou complementar a publisher.
   Decisão final em .1 com matriz multi-critério se necessário.
9. **(Específico formatos individuais)** Confirmar prefixos/
   separadores per pré-decisões §"Decisões diferidas":
   - editor: `Ed. {editor}`.
   - series: `({series})`.
   - location: `{location}:`.
   - isbn: `isbn:{isbn}`.
   - note: `[{note}]`.
   - organization: substitutivo a publisher.
   Decisão final em .1 com paridade vanilla.

### .2 Expandir struct `BibEntry`

`01_core/src/entities/bib_entry.rs`:
- Adicionar 6 fields Optional<String>.
- Builder pattern extendido com 6 métodos `with_*`.
- Constructor `new(...)` original inalterado (paridade
  P159D/E).
- Derives mantidos: `Debug`, `Clone`, `PartialEq`, `Eq`.

### .3 Extender `extract_bib_entries`

`01_core/src/rules/stdlib/structural.rs`:
- Helper `optional_str(field)` reusado para os 6 fields.
- Validação tipo `Value::Str`; outros tipos rejeitados com
  diagnóstico claro mencionando field específico.
- Constructor `BibEntry::new(...).with_editor(...)...` via
  builder fluente.

### .4 Refinar layout `format_bib_entry`

`01_core/src/rules/layout/mod.rs`:
- Concatenação condicional para os 6 fields per ordem decidida
  em .1.
- Match nas combinações de presença Some/None para evitar
  separadores vazios.
- Backwards compat: quando todos os 6 fields novos `None`,
  output P159E preservado exactamente.

### .5 Tests

- **Unit tests `BibEntry`** em `entities/bib_entry.rs` (~3-4):
  - Constructor com os 6 fields via builder.
  - PartialEq cobre 16 fields agora.
  - Backwards compat: `new(4 args)` continua a funcionar
    (regression P159A+P159D+P159E).
  - Builder pattern combinando subset (e.g. só editor + isbn)
    funciona.

- **Stdlib tests** em `stdlib/mod.rs` (~3-4):
  - Parse com todos os 6 fields presentes.
  - Parse com subset (3 fields).
  - Parse sem fields novos (regression P159E).
  - Tipo errado em isbn rejeitado.

- **Layout E2E tests** em `layout/tests.rs` (~3-4):
  - Bibliography com entry completa (16 fields) renderiza
    formato extendido.
  - Bibliography com entry intermédia (P159E + editor
    apenas) renderiza formato parcialmente extendido.
  - Bibliography com entry mínima (sem fields novos)
    renderiza formato P159E original (regression).
  - Bibliography com organization sem publisher renderiza
    organization no slot publisher.

**Δ esperado**: +8 a +12 tests (M; cobertura mais ampla que
P159D/E).

### .6 Propagação de hashes

`crystalline-lint --fix-hashes .`:
- `bib_entry.rs` hash: per regra L0-baseline preservado se
  prompt `bib_entry.md` não mencionar fields individualmente
  (paridade P159D/E resultado).
- Outros ficheiros: refactor interno; preserva L0.

**Esperado "Nothing to fix" se interpretação L0 mantém** —
lição P159A/C/D/P158C/F/E aplicada conscientemente. Refactor
de tipo entity sem alteração de prompt L0 preserva hash.

---

## Verificação

Numerada para reporte de conclusão:

1. `cargo test` em workspace: **1490 + Δ** tests, zero falhas
   (Δ esperado +8-12).
2. `crystalline-lint`: zero violations.
3. Contagem variants Content: **58** (inalterada — refino
   tipo entity).
4. Contagem stdlib funcs: **48** (inalterada).
5. **Hash `entities/content.rs` preservado** `ec58d849` —
   **17º passo consecutivo** via L0-baseline interpretation.
6. Hash `entities/bib_entry.rs` per L0-baseline (preservado
   se prompt L0 não modificado, paridade P159D+P159E).
7. Decisão sobre ordem layout dos 6 fields documentada no
   relatório §"Decisões tomadas em .1" com justificação per
   paridade APA + matriz multi-critério se aplicável.
8. Decisões sobre formato individual de cada field documentadas
   com justificação.
9. **Sem novas reservas** criadas (paridade política
   P158/P159).
10. Tests pré-existentes Bibliography (P159A+P159D+P159E)
    passam inalterados — fields novos default None produz
    output P159E original.
11. `layout_grid` original NÃO modificado (paridade P157A/B/C
    + P159A/C/D/F/E).
12. **Helper `optional_str` cumulativo N=4 → 10 usos** —
    limiar promoção (N=3-4) ultrapassado largamente; promoção
    diferida em passo administrativo XS NÃO reservado.
13. **Subpadrão #16 "refino tipo entity sem alteração Content"
    cresce N=2 → 3** — atinge limiar formalização N=3-4.
14. Restantes fields BibEntry vanilla (`booktitle`/`address`/
    `chapter`/`type`/`institution`/etc.) NÃO materializados
    (NÃO reservados; candidatos futuros).

---

## Critério de conclusão

- Verificações 1-14 passam.
- Relatório separado em
  `00_nucleo/materialization/typst-passo-159g-relatorio.md`
  produzido com:
  - Resumo do executado.
  - Confirmação de cada verificação.
  - **§análise de risco** (padrão N=22 → 23).
  - Slope cumulativo Model (mesa P155-P159G).
  - ADR-0061 §"Aplicações cumulativas" anotada com P159G.
  - **Confirmação**: subpadrão #16 atinge N=3 (limiar
    formalização); helper `optional_str` cumulativo N=10
    (largamente promovível); estabilidade hash content.rs
    N=16 → 17.
  - **Decisões de ordem layout + formato registadas** com
    justificação para refinos futuros e referência futura
    se outras citation styles forem implementadas (Bloco B
    hayagriva).

---

## O que pode sair errado

**Cenários gerais**:
- Inventário .1 revela que vanilla usa estruturas complexas
  para algum field (e.g. `editor: Person` em vez de String) —
  simplificar para `Option<String>` per ADR-0054 graded;
  documentar.
- Inventário .1 revela que ordem APA é não-trivial para
  algum field (e.g. note posição depende do tipo de entry) —
  simplificar para posição fixa per ADR-0033 minimal;
  documentar.

**Cenários específicos**:
- Layout output muito longo em uma linha (16 fields possíveis)
  — refactor multi-line diferido (depende multi-region
  DEBT-56).
- Tests pré-existentes (P159A+P159D+P159E) esperarem formato
  exacto conflitante — backwards compat preservada via fields
  novos default None; sem ajuste de tests existentes esperado.
- Organization vs publisher conflito (ambos presentes) — decidir
  se ambos renderizam ou organization substitui; documentar
  em .1.
- Note position dependente do tipo de entry — simplificar para
  posição fixa final per ADR-0054 graded.
- Some/None match exhaustivo crescer factorialmente com 6
  fields opcionais — usar concatenação condicional in-place
  em vez de match exhaustivo.
- L0-baseline NÃO preservar hash `bib_entry.rs` — reconhecer
  e documentar; quebra excepcional não bloqueante.

---

## Notas operacionais

- **Vigésima primeira aplicação de materialização**. Patamar
  empírico forte. Sem reformulação esperada.
- **§análise de risco no relatório** com peso baixo (refino
  pattern validado N=2). Vigésima terceira aplicação consecutiva
  preserva precedente.
- **Pattern P159D replicado pela terceira vez**: 3º refino
  estrutural de tipo entity sem alteração ao variant Content.
  Subpadrão #16 cresce N=2 → 3 — atinge limiar formalização
  N=3-4. Próxima aplicação (se houver) consolida patamar
  forte.
- **Helper `optional_str` cumulativo**: P159D N=2 + P159E N=2
  + **P159G N=6** = N=10 cumulativos no mesmo helper inline.
  Largamente acima do limiar promoção N=3-4. Promoção a
  helper público diferida em passo administrativo XS NÃO
  reservado.
- **ADR-0064 NÃO aplicável**: fields são `Option<String>`
  directos sem mapping `Smart<T>`. Pattern Optional trivial.
- **BibEntry pós-P159G**: 16 fields total (4 obrigatórios +
  12 opcionais). Cobertura aproximadamente 70-75% dos fields
  hayagriva universais; restantes 4-6 fields menos comuns
  diferidos.
- **Sequência alfabética identificadores família 159 não-monótona**:
  A → B → C → D → F → E → G. Facto histórico registado em
  ADR-0061 §"Aplicações cumulativas". Preserva slot E para
  refinos família 159 que surgiram após P158C ocupar
  identificador alternativo.

---

## Pós-passo

Após conclusão de P159G:

**Layout fica em 78% inalterado**. **Model agregado fica em
~50% inalterado** (refino tipo entity). **Hash
`entities/content.rs` provavelmente preservado** (17º passo
consecutivo via L0-baseline). **BibEntry com 16 fields**
(cobertura ~70-75% hayagriva universais).

**Próxima decisão** (sem candidata pré-acordada — política
"sem novas reservas"):

- **Restantes fields BibEntry vanilla** (booktitle/address/
  chapter/type/institution/etc.): NÃO reservados. Candidatos
  a refinos futuros se prioritários.
- **ADR-0062-create** — XS administrativo; desbloqueia Bloco B.
- **Bloco B (hayagriva)**: P159H após ADR-0062 PROPOSTO.
- **Bloco C (cross-módulo)**: refactor multi-region L+
  (DEBT-34e + DEBT-56) ou Introspection P160.
- **Refinos Model fora Bloco A continuação** (mais langs em
  `figure_supplement_for_lang`; etc.).
- **Mudança de módulo**: Layout Fase 3 (columns/colbreak) ou
  Introspection P160.
- **Passos administrativos XS atingidos múltiplos limiares**:
  - Promoção `optional_str` a helper público (N=10 cumulativos
    largamente atingem limiar).
  - ADR meta subpadrão #16 (refino tipo entity sem Content;
    N=3 atinge limiar formalização).
  - ADR meta subpadrão #15 (state lookup; N=3 atinge limiar).
  - L0 content.md update.
  - Promover ADR-0060 a R1.
  - ADR meta saturação ADR-0064.

ADR-0060 mantém-se `IMPLEMENTADO`. ADR-0061 mantém-se
`PROPOSTO`. ADR-0062 mantém-se reserva sem ficheiro.

Padrão granularidade 1-2 features/passo (N=20 com P159G se
fechar sem reformulação) **NÃO** é formalizado em ADR.
Continua candidato.

**Pausa natural após P159G — BibEntry com 16 fields (cobertura
~70-75% hayagriva); pattern P159D replicado pela terceira vez;
subpadrão #16 atinge N=3 (limiar formalização); helper
`optional_str` cumulativo N=10 (largamente promovível);
sequência alfabética P159 não-monótona estabelecida e registada.
Decisão humana sobre próxima direcção tem máxima informação.**
