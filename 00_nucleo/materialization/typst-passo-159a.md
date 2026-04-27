# Passo P159A — Bibliography + Cite par acoplado minimal (Model bibliography + cite sub-passo 1)

Primeiro sub-passo substantivo de Bibliography + Cite per scope
decidido em diagnóstico P159 §3.5. Materializa **Estrutura A
adaptada** — par acoplado num único passo M+ sem hayagriva
(input cristalino literal `Vec<BibEntry>`). **Décima quarta
aplicação consecutiva de materialização** desde início da série
granular P156C.

**Granularidade quebrada honestamente** N=13 → M+ com precedente
P156C (par lógico pad+hide M+). **Padrão "estabilidade hash
content.rs" termina** após 8 passos consecutivos (P156L → P159)
— P159A é primeiro a modificar variant Content desde P157C.

Primeira matérialização de tipo entity novo em
`01_core/src/entities/` desde série P156. Refinos futuros
(hayagriva, CSL, form variants, numbering, cross-document
refs) **NÃO reservados** per política P158.

---

## Estado actual antes de começar

- 63 ADRs após P159 (28 EM VIGOR; ADR-0060 IMPLEMENTADO;
  ADR-0062 reserva sem ficheiro confirmada).
- Layout: 78% (inalterado). Cobertura arquitectural total 80%.
- Cobertura Model agregada: ~50% (inalterada em P157B/C/P158/A).
- Hash actual `entities/content.rs`: `ec58d849` (preservado
  em **8 passos consecutivos** P156L → P159; **quebra
  esperada em P159A**).
- 1385 tests (lib+integ+diagnostic; workspace 1407); zero
  violations linter.
- 56 variants Content; 46 stdlib funcs.
- Padrões consolidados pós-P159: granularidade N=13;
  inventariar N=13; Smart→Option N=9 (saturação); §análise
  risco N=13; estabilidade hash content.rs N=8 (subpadrão a
  quebrar).

**Diagnóstico P159** confirmou:
- Bibliography + Cite **completamente ausentes** em código
  (zero matches grep).
- Vanilla integra hayagriva profundamente (1226 linhas).
- ADR-0062 hayagriva **NÃO existe como ficheiro** (apenas
  reserva em README).
- ADR-0017 Introspection runtime NÃO bloqueia subset minimal
  (walk single-pass viável).
- DEBT-55 documenta plano completo XL com hayagriva.
- Subset escolhido contorna hayagriva com `Vec<BibEntry>`
  literal cristalino.

**Política "sem novas reservas" preservada** — P159A não cria
reservas para passos futuros. Refinos pós-P159A permanecem
candidatos NÃO-reservados.

**Leituras prévias obrigatórias**:
- `00_nucleo/diagnosticos/diagnostico-bibliography-cite-passo-159.md`
  — §§1-§5 (esboço P159A em §5).
- `00_nucleo/DEBT.md` — DEBT-55 conteúdo completo.
- `00_nucleo/adr/typst-adr-0033-paridade-observavel.md` —
  fundamento para placeholder render `[key]`.
- `00_nucleo/adr/typst-adr-0054-perfil-graded.md` — fundamento
  para subset minimal sem hayagriva.
- `00_nucleo/adr/typst-adr-0034-diagnostico-obrigatorio.md` —
  inventário .1 obrigatório.
- `00_nucleo/adr/typst-adr-0065-inventariar-primeiro.md` —
  critério #2 (escolha de tipo) aplicável a `BibEntry`.
- `01_core/src/entities/content.rs` — paridade estrutural com
  variants P157A/B/C.
- `01_core/src/rules/stdlib/structural.rs` — paridade pattern
  para stdlib funcs Model.
- `01_core/src/rules/introspect.rs` — counters figure por
  kind (paridade para walk single-pass de Cite).
- `lab/typst-original/crates/typst-library/src/model/bibliography.rs`
  (vanilla, quarentena) — referência para subset semântico
  minimal.
- `lab/typst-original/crates/typst-library/src/model/cite.rs`
  (vanilla, quarentena) — referência.

---

## Natureza do passo

**Tamanho**: M+.

**Justificação**: Par funcional acoplado (Bibliography + Cite)
que **não pode ser separado** por dependência semântica vanilla
(Cite referencia entries de Bibliography). 1 tipo entity novo
+ 2 variants Content + 2 stdlib funcs + 2 pattern arms layout
+ 2 pattern arms introspect.

**Granularidade quebrada honestamente**: N=13 → M+ par acoplado.
Precedente P156C (pad+hide) materializou par lógico em M+
similar. Diagnóstico P159 §3.5 documenta justificação completa.

**Risco médio**:
- **Médio** (não baixo) porque é primeiro passo M+ Model sem
  refactor mas com 2 variants paralelos + tipo entity novo +
  quebra de padrão "estabilidade hash content.rs".
- Reuso parcial de infraestrutura (counters walk single-pass
  paridade P75 figure).
- Sem dependência externa nova (hayagriva contornada).

---

## Decisões já tomadas

- **Tipo entity novo `BibEntry`**:
  ```rust
  // 01_core/src/entities/bib_entry.rs (ficheiro novo)
  pub struct BibEntry {
      pub key:    String,
      pub author: String,
      pub title:  String,
      pub year:   u32,
  }
  ```
  Subset minimal de fields vanilla — author/title/year são
  os 3 fields universais em todas as styles bibliográficas;
  key é identificador único cristalino. Outros fields vanilla
  (volume/publisher/url/etc.) **diferidos** per ADR-0054
  graded. ADR-0065 critério #2 (escolha de tipo) aplicado
  via inventário .1.

- **Variant `Content::Bibliography`**:
  ```rust
  Bibliography {
      entries: Vec<BibEntry>,
      title:   Option<Box<Content>>,
  }
  ```
  - `entries`: Vec literal cristalino (sem hayagriva).
  - `title`: Option<Box<Content>> per ADR-0064 Caso A
    (Smart<Content> vanilla → Option<Content> cristalino).
  - **Sem `style`** (CSL parsing diferido per ADR-0054).
  - **Sem `lang`/`region`** (i18n diferido).
  - **Sem `full`** (form discrimination diferido).

- **Variant `Content::Cite`**:
  ```rust
  Cite {
      key:        String,
      supplement: Option<Box<Content>>,
  }
  ```
  - `key`: String directo (referência a entry por key).
  - `supplement`: Option<Box<Content>> per ADR-0064 Caso A.
  - **Sem `form`** (Normal/Prose/Author/Year/etc. diferido
    per ADR-0054).
  - **Sem `style`** (override de bibliography style diferido).

- **`Box<Content>` em title/supplement**: paridade P157B
  TableCell.body, P157C TableHeader/Footer.body — single
  child via Box, não Arc.

- **Stdlib funcs**:
  - `native_bibliography(entries, title: none) -> content`.
  - `native_cite(key, supplement: none) -> content`.
  - Naming flat per padrão P157B (`bibliography` e `cite`,
    sem namespacing). FieldAccess actual não suporta
    `Value::Func.subname`.
  - Localização: `01_core/src/rules/stdlib/structural.rs`
    (continuação Model per P157A).

- **Helper privado novo `extract_bib_entries`**:
  parse de `Vec<BibEntry>` a partir de `Value::Array(Vec<Value>)`
  onde cada elemento é `Value::Dict` com keys
  `key`/`author`/`title`/`year`. Validação hard de fields
  obrigatórios. Localização: `stdlib/structural.rs` privado.

- **Layout minimal**:
  - `Content::Bibliography { entries, title }` → renderiza
    `title` (se Some) seguido de lista de entries formatadas
    como linhas `"[{key}] {author}. {title} ({year})."`.
  - `Content::Cite { key, supplement }` → renderiza placeholder
    `"[{key}]"` seguido de `supplement` (se Some). Paridade
    vanilla mínima per ADR-0033 + ADR-0054 graded.

- **Introspect/walk single-pass**:
  - Bibliography: walk em `title` e em cada entry (entry tem
    fields String, sem Content recursivo — walk trivial).
  - Cite: walk em `supplement` apenas. **Sem validação**
    `key ∈ Bibliography.entries` em walk (validação cross-
    document é cross-pass; diferida per ADR-0017 graded).

## Decisões diferidas

- **Validação cross-reference Cite.key ∈ Bibliography.keys**:
  diferida per ADR-0017 (Introspection runtime adiada).
  P159A não valida — `cite("inexistente")` produz placeholder
  `[inexistente]` sem erro.

- **Auto-detecção entries vazias**: `bibliography(entries: [])`
  é válido (renderiza só title se houver). Sem warning per
  ADR-0033 minimal.

- **Hayagriva integration** (parsing `.bib`/`.yaml`): NÃO
  reservada. Refino futuro candidato a ADR-0062 promovida +
  passo dedicado.

- **CSL styles** (numérico, autor-ano, IEEE, etc.): NÃO
  reservadas. Default é placeholder simples `[key]`.

- **Form variants** (`Normal`, `Prose`, `Author`, `Year`):
  NÃO reservadas. Default é Normal placeholder.

- **Numbering schemes** dinâmicos: diferidos.

- **Cross-document forward refs**: diferidos per ADR-0017.

- **Promoção de `extract_bib_entries` a helper público**:
  diferida per política consistente N=3-4.

---

## Sub-passos

### .1 Inventário pré-materialização (obrigatório per ADR-0065)

Diagnóstico em
`00_nucleo/diagnosticos/diagnostico-bibliography-cite-passo-159a.md`
com 7 itens canónicos (ADR-0034) + 3 itens específicos para
par acoplado novo + tipo entity novo:

1. Assinatura vanilla `BibliographyElem` e `CiteElem` minimal
   — confirmar fields críticos vs diferidos per ADR-0054
   graded.
2. Comportamento observável (Cite renderiza placeholder
   `[key]`; Bibliography renderiza lista; sem validação
   cross-reference em walk single-pass).
3. ADR-0064 caso aplicável: **Caso A** para `title` e
   `supplement` (Option<Content>).
4. **(Específico tipo entity novo)** Estrutura de `BibEntry`:
   campos minimais 4 (key/author/title/year); todos String/u32
   directos (sem Smart<T> em entry); paridade conceptual com
   vanilla `hayagriva::Entry` mas com subset extremamente
   reduzido. ADR-0065 critério #2 (escolha de tipo) aplicado.
5. Helpers stdlib reusáveis: nenhum directo (parse de
   Dict<String, Value> é específico); helper novo
   `extract_bib_entries`.
6. Limitações aceites (sem hayagriva; sem CSL; sem form
   variants; sem cross-reference validation; sem
   numbering schemes; placeholder render).
7. Tests planeados (variant present + stdlib happy path +
   defaults + edge cases — range 15-20 per esboço P159 §5;
   par simétrico Bibliography↔Cite onde aplicável; tipo
   entity tests separados).
8. **(Específico par acoplado)** Confirmar simetria onde
   aplicável: PartialEq, map_text, walk, layout. Diferenças
   intencionais entre Bibliography e Cite documentadas
   (Bibliography é container; Cite é leaf).
9. **(Específico tipo entity novo)** Localização decidida:
   `entities/bib_entry.rs` ficheiro novo vs adicionar a
   ficheiro existente. Pré-decisão: ficheiro novo per padrão
   P156C `entities/sides.rs`. Confirmar em .1.
10. **(Específico quebra hash)** Reconhecer explicitamente
    que P159A quebra padrão "estabilidade hash content.rs"
    (8 passos consecutivos). Documentar como inevitável
    (variants novos exigem alteração ao enum).

### .2 Adicionar tipo `BibEntry`

`01_core/src/entities/bib_entry.rs` (ficheiro novo per
inventário .1):
- Struct `BibEntry { key, author, title, year }` per
  §"Decisões já tomadas".
- `impl PartialEq` (derivado).
- `impl Debug` (derivado).
- `impl Clone` (derivado).
- Sem métodos próprios neste passo — entry é dados puros.
- `pub mod bib_entry;` em `entities/mod.rs`.
- Re-export se necessário per padrão P156C.

### .3 Adicionar variants `Content::Bibliography` e `Content::Cite`

`01_core/src/entities/content.rs`:
- Adicionar variant `Bibliography { entries: Vec<BibEntry>,
  title: Option<Box<Content>> }`.
- Adicionar variant `Cite { key: String, supplement:
  Option<Box<Content>> }`.
- Cobrir todos os 9 sítios pattern-match Content existentes
  (paridade P156I/J/L e P157A/B/C):
  - Variant declarations + construtores.
  - `is_empty()`: Bibliography proxy via `entries.is_empty()
    && title.is_none()`; Cite sempre `false` (key não-vazia).
  - `plain_text()`: Bibliography concatena title + entries
    formatadas; Cite emite `"[{key}]"`.
  - `PartialEq`: cobre 2 fields cada.
  - `map_content`: recurse em title (Bibliography) e
    supplement (Cite); preserva entries/key.
  - `map_text`: idem.
  - `introspect.rs::materialize_time`: recurse em
    title/supplement; preserva entries/key.
  - `introspect.rs::walk`: walk em title (Bibliography),
    iterate entries sem walk (entries são dados); walk em
    supplement (Cite).
  - `layout/mod.rs::layout_content`: arms novos para
    Bibliography (lista) e Cite (placeholder).

- Construtores `Content::bibliography(entries, title)` e
  `Content::cite(key, supplement)`.

- **Hash quebra esperada**: aceitar.

### .4 Adicionar stdlib funcs `native_bibliography` e `native_cite`

`01_core/src/rules/stdlib/structural.rs`:
- Func `bibliography(entries, title: none) -> content`.
  - Helper `extract_bib_entries(Value::Array<Value::Dict>)`.
  - Cada Dict valida fields obrigatórios `key/author/title/year`.
  - Validações: dict sem field obrigatório rejeitado; year
    não-Int rejeitado; key/author/title não-Str rejeitado;
    array vazio aceite (Bibliography vazia válida).
- Func `cite(key, supplement: none) -> content`.
  - `key: Value::Str` posicional obrigatório.
  - `supplement: Option<Content>` named.
  - Validações: key não-Str rejeitado; key vazio rejeitado.

Registadas em `eval/mod.rs::make_stdlib`. Re-exportadas em
`stdlib/mod.rs`.

### .5 Layout para Bibliography + Cite

`01_core/src/rules/layout/mod.rs`:
- Pattern arm `Content::Bibliography { entries, title }`:
  - Render title se Some (paridade Block).
  - Iterate entries; cada entry renderiza linha
    `"[{key}] {author}. {title} ({year})."`.
- Pattern arm `Content::Cite { key, supplement }`:
  - Render `"[{key}]"` placeholder.
  - Render supplement se Some (concatenado após placeholder).

**`layout_grid` NÃO modificado** (paridade verificações
P157A #8, P157B #10, P157C #10).

### .6 Tests

**Tests do tipo entity novo** (`entities/bib_entry.rs` ou
módulo de tests dedicado, ~3):
- `bib_entry_constructor` — fields acessíveis.
- `bib_entry_partial_eq` — equivalência por todos os fields.
- `bib_entry_debug` — formatting trivial.

**Unit tests `Content::Bibliography`** em
`entities/content.rs` (~5):
- Constructor default (entries vazias, title None).
- Constructor com entries e title.
- `is_empty` proxy via entries+title.
- `plain_text` concatena title + entries formatadas.
- `PartialEq` cobertura.

**Unit tests `Content::Cite`** em `entities/content.rs` (~4):
- Constructor com key.
- Constructor com key + supplement.
- `is_empty` sempre false.
- `plain_text` emite `"[{key}]"`.

**Stdlib tests** (~6 = 3 pares Bibliography↔Cite):
- Defaults (entries vazias / supplement None).
- Argumentos completos.
- Validation hard (field obrigatório ausente / key vazia).

**Layout E2E tests** em `layout/tests.rs` (~3):
- `layout_bibliography_renderiza_entries_como_lista`.
- `layout_cite_renderiza_placeholder_com_key`.
- `layout_bibliography_e_cite_no_mesmo_documento` —
  integrativo.

**Δ esperado**: +18 a +21 tests (alinhado com esboço P159 §5;
range 15-20 alargado por par acoplado e tipo entity novo).

### .7 Propagação de hashes

`crystalline-lint --fix-hashes .` para propagar hash novo de
`entities/content.rs` aos prompts L0 que o referenciam.

**Hash quebra esperada confirmada**: `entities/content.rs`
`ec58d849` → novo hash. **Primeiro hash break em 8 passos
consecutivos** (P156L → P159A). Documentar no relatório.

Verificar se há prompt L0 dedicado a `entities/bib_entry.rs`
novo — provavelmente não (módulo entity novo + L0 prompt
agregado). Decidido em .1.

---

## Verificação

Numerada para reporte de conclusão:

1. `cargo test` em workspace: **1385 + Δ** tests, zero falhas
   (Δ esperado +18 a +21).
2. `crystalline-lint`: zero violations.
3. Contagem variants Content: **58** (56 → 58; +2 par
   acoplado).
4. Contagem stdlib funcs: **48** (46 → 48; +2 par acoplado).
5. **Tipo entity novo `BibEntry`** adicionado a
   `01_core/src/entities/bib_entry.rs`.
6. Cobertura Model agregada: **avanço quantitativo esperado**
   — entradas `bibliography` e `cite` ambas `ausente →
   implementado parcial` per ADR-0054 graded. **Recálculo
   exacto em .1**: Bibliography/Cite são entradas Top-level
   na tabela A.6 Model (não sub-entradas como `table.cell`/
   `table.header`/`table.footer`); cobertura agregada Model
   pode subir ~2-3pp consoante numerador/denominador exactos.
7. Hash actualizado em prompts L0 (`crystalline-lint --check-hashes`
   passa).
8. **Hash `entities/content.rs` quebra padrão** —
   `ec58d849 → novo`. **Primeiro break em 8 passos**.
   Documentado.
9. Granularidade quebrada N=13 → M+ honestamente registada;
   precedente P156C citado.
10. ADR-0064 Caso A aplicado em title (Bibliography) e
    supplement (Cite); patamar Caso A cresce em **N=4 → 5
    aplicações** (P156G/H/I + P157B + P159A).
11. **Sem novas reservas** criadas em P159A (paridade política
    P158).
12. ADR-0017 não promovida (cross-reference validation diferida).
13. ADR-0062 não promovida (hayagriva contornada).
14. `layout_grid` original NÃO modificado (paridade P157A/B/C).

---

## Critério de conclusão

- Verificações 1-14 passam.
- Relatório separado em
  `00_nucleo/materialization/typst-passo-159a-relatorio.md`
  produzido com:
  - Resumo do executado.
  - Confirmação de cada verificação.
  - **§análise de risco** (padrão N=13 → 14; primeira quebra
    de padrão "estabilidade hash content.rs" — primeiro M+
    par acoplado; primeira matérialização de tipo entity
    novo desde série P156).
  - Slope cumulativo Model (mesa P155-P159A) com avanço
    quantitativo registado.
  - ADR-0061 §"Aplicações cumulativas" anotada com P159A
    (slope Layout "—"; nota cross-domínio).
  - **Confirmação**: ADR-0064 Caso A patamar N=5; ADR-0065
    critério #2 (escolha de tipo) primeira aplicação
    isolada concreta.
  - **Quebra de padrões registada**: hash content.rs (8
    passos terminam) + granularidade (N=13 → M+).

---

## O que pode sair errado

**Cenários gerais**:
- Inventário .1 revela que vanilla `BibEntry` exige fields
  além dos 4 minimais (e.g. `type` field obrigatório) →
  expandir struct ou registar como ADR-0054 graded explícita;
  documentar.
- Inventário .1 revela que walk single-pass de Cite tem
  ambiguidade (e.g. cite cross-document depende de validação
  cross-pass) → simplificar para "sem validação" e documentar
  como diferida per ADR-0017.

**Cenários específicos**:
- Helper `extract_bib_entries` ter complexidade superior
  (e.g. parse de Dict aninhado para BibEntry profundo) →
  fallback para parse linear; documentar.
- Pattern-match exhaustive falhar em variant existente fora
  de `content.rs` (paridade P157A/B/C) → grep por `Content::`
  em todo o crate.
- Layout E2E "bibliography com entries" produzir output
  divergente de esperado → ajustar formatting de string em
  `layout/mod.rs`; tests updated.
- Tipo entity `BibEntry` colidir com tipo cristalino existente
  (improvável; verificar em .1) → renomear se conflito.
- `extract_bib_entries` exigir conversão Dict→Struct via
  trait que cristalino não tem → implementar inline; sem
  trait genérico.

---

## Notas operacionais

- **Décima quarta aplicação de materialização**. Patamar
  empírico forte. **Primeiro M+ par acoplado** desde P156C
  (que era passo aditivo, não par funcional inseparável).
- **§análise de risco no relatório** com peso real (décima
  quarta aplicação consecutiva). Primeiro M+ par acoplado
  Model — registar como precedente.
- **Helper `extract_bib_entries` N=1**. Sem candidato a reuso
  até agora; promoção diferida.
- **Quebra de padrão "estabilidade hash content.rs"**: 8 passos
  consecutivos terminam em P159A. Reconhecer explicitamente
  como inevitável (variants novos no enum). Padrão emergente
  de "passos aditivos preservam hash" preserva-se conceptualmente
  mas P159A é correctamente classificado como passo aditivo
  apesar de quebrar contagem (variants novos vs refinos puros).
- **ADR-0064 Caso A patamar cresce N=5**: P156G/H/I (Layout)
  + P157B (Model TableCell.x/y) + **P159A (Model
  Bibliography.title + Cite.supplement)**. Diversidade
  cross-domínio reforçada.
- **ADR-0065 critério #2 (escolha de tipo) primeira aplicação
  isolada concreta**: decisão de `BibEntry` campos minimais
  (4 fields) é decisão arquitectural-chave registada em .1.
  Patamar inventariar-primeiro N=13 → 14 com diversidade
  crescente.
- **Política "sem novas reservas" preservada** — refinos
  pós-P159A (hayagriva, CSL, form, numbering, cross-document)
  permanecem candidatos NÃO-reservados.

---

## Pós-passo

Após conclusão de P159A:

**Layout fica em 78% inalterado**. **Model agregado avança
quantitativamente** (estimado +2-3pp; cálculo exacto em .1
e relatório). **Hash `entities/content.rs` quebra padrão de
8 passos** — novo hash a propagar.

**Próxima decisão** (sem candidata pré-acordada — política
"sem novas reservas"):

- Continuar refino Bibliography/Cite (hayagriva integration;
  form variants; CSL — todos NÃO reservados).
- Continuar refino figure-kinds (supplement por lang —
  NÃO reservado).
- Atacar Introspection (17%; mais fraco).
- Continuar Fase 3 Layout (columns/colbreak — DEBT-56).
- Footnote area.
- Promover ADR-0061 a IMPLEMENTADO.
- Promover `extract_length` a helper público (N=7 patamar
  forte).
- Fechar DEBT-34e + DEBT-56 (refactor multi-region L+).
- Promover ADR-0060 a R1 com confirmação Fase 2 fechada.
- ADR meta XS de "ADR-0064 caso completion" (saturação).
- Promover ADR-0062 a IMPLEMENTADO (precondição hayagriva).
- Criar ADR-0062 como ficheiro PROPOSTO (passo administrativo
  XS).

ADR-0060 mantém-se `IMPLEMENTADO`. ADR-0061 mantém-se
`PROPOSTO`. ADR-0062 mantém-se reserva sem ficheiro.

Padrão granularidade 1-2 features/passo (N=13) **quebrado em
P159A para M+** honestamente registada. Não é primeira quebra
da série (precedente P156C par lógico) mas é primeira no
domínio Model.

**Pausa natural após P159A — Bibliography + Cite minimal
materializado; ADR-0060 §"Decisão" Bibliography subset minimal
fechado; padrões de quebra honestamente registados; política
"sem novas reservas" preservada. Decisão humana sobre próxima
direcção (12 candidatas documentadas) tem máxima informação
acumulada.**
