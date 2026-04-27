# Diagnóstico expansão série 159 e tecto realista Model — Passo P159B

Inventário diagnóstico amplo precedendo materialização per
**ADR-0034 + ADR-0065** — análogo estrutural a **P156B**
(diagnóstico Layout amplo). **Décima sexta aplicação consecutiva**
do padrão diagnóstico-primeiro; **quarta aplicação concreta** do
critério #5 ADR-0065 (scope determinado por inventário) com
**diversidade ampliada para multi-feature** (P157/P158/P159
inventariaram uma feature cada; P159B inventaria **todas as
expansões pendentes da família 159 + outros refinos Model**).

Análoga à pergunta "fechamos Layout só mantendo o ritmo?" feita
anteriormente, mas para Model com base factual acumulada
P157/P158/P159A.

---

## §1 — Inventário ADRs/DEBTs por família

### §1.1 Família 159 (Bibliography + Cite)

#### DEBT-55 (P154A; renumerada P156B)

**Conteúdo completo** (re-confirmado pós-P159A):
- **Plano declarado** (10 itens):
  - [ ] ADR-0062 criada
  - [ ] Cargo.toml + crystalline.toml configurados
  - [ ] `Content::Bibliography` + `Content::Cite` variants ✓ **(P159A)**
  - [ ] `native_bibliography` + `native_cite` em stdlib ✓ **(P159A)**
  - [ ] Pipeline introspect com resolução cruzada
  - [ ] Render layout para ambos ✓ **(P159A placeholder)**
  - [ ] 5-10 testes ✓ **(P159A: 27 tests)**
  - [ ] Inventário 148 reclassifica de `ausente` para `implementado⁺` ✓
        **(P159A: cite/bib `ausente → parcial`)**
- **Critério de fecho**: ADR-0062 IMPLEMENTADO + bibliography
  + cite materializados + tests + inventário actualizado.

**Estado pós-P159A**: 4/10 itens cumpridos (variants + stdlib +
layout placeholder + tests + reclassificação). **6/10 pendentes**:
- ADR-0062 criar.
- Cargo.toml + crystalline.toml.
- Pipeline introspect com resolução cruzada (depende ADR-0017).
- Render layout completo (CSL).

DEBT-55 **NÃO fecha** com refinos Model puro — exige hayagriva
+ ADR-0062 promovida + ADR-0017 resolvida.

#### ADR-0062 — reserva sem ficheiro (confirmado)

Confirmado em P159 §1.2 e P159A: ADR-0062 **NÃO existe como
ficheiro** — apenas reserva em README ADRs e mencionada em
ADR-0060.

Promoção exigida se hayagriva for integrada:
1. Criar ficheiro `00_nucleo/adr/typst-adr-0062-hayagriva-autorizacao.md`.
2. Status inicial PROPOSTO.
3. Justificação per precedentes ADR-0024 (ecow), ADR-0023
   (indexmap), ADR-0057 (hypher).
4. Crate `hayagriva 0.9.1` já em cache local (probe P152).

#### ADR-0017 — Introspection runtime adiada

Status `IMPLEMENTADO`. Decisão: `eval()` não migra; tipos
`Engine`/`Route`/`Sink`/`Traced`/`Introspector`/etc. ficam
em vanilla por dependerem de I/O e rendering.

**Impacto em cite() cross-document**:
- Forward refs (`cite(<x>)` antes de `bibliography([x: ...])`)
  exigem Introspector cross-pass — bloqueado.
- Walk single-pass (cite após bibliography no mesmo documento) —
  viável e implementado em P159A subset minimal.

#### ADRs autorização crate externa (precedentes)

- ADR-0024 (ecow): autorização L1.
- ADR-0023 (indexmap): autorização L1.
- ADR-0057 (hypher): autorização L1.

Padrão claro de autorização per ADR. Hayagriva seguirá mesmo
padrão.

### §1.2 Família 158 (figure-kinds)

#### Diagnóstico P158 §3.3 (subset máximo)

Subset máximo declarado mas NÃO materializado em P158A:
- **Auto-detecção + supplement automático** ("Figure"/"Table"/
  "Listing" prefix por lang).
- Refactor moderado em `introspect.rs` para mapeamento
  `kind → prefix_localizado(lang)`.
- Tests ~12-18 esperados.

Refino futuro candidato a passo dedicado P158B (NÃO reservado
per política P158).

#### ADR-0041 sobre show rules

Show rules para `figure.where(kind: ...)` selectors estão
**scope-out per ADR-0041 + ADR-0054 graded**. Show rules em
cristalino actual cobrem `figure: ...` (selector simples)
mas não `figure.where(...)` (selector complex).

Refino futuro candidato a refactor de show rules — **fora de
scope refinos Model** (toca em rules/show ou similar).

#### DEBTs i18n

Verificação: nenhum DEBT específico de i18n encontrado em
DEBT.md. P155 quote já implementou localização parcial via
`rules/lang/quotes.rs` (6 idiomas + ASCII fallback) — padrão
reusável para supplement.

### §1.3 Família 157 (table foundations)

#### DEBT-34e (Passo 80) — colspan e rowspan

**Conteúdo completo** (re-confirmado):
> "Células que ocupam múltiplas colunas ou linhas requerem um
> algoritmo de placement diferente. Resolução: passo futuro."

DEBT muito sucinto. **Implicação**: refactor profundo de
`layout/grid.rs` (272 linhas) para suportar placement com
colspan/rowspan. Não documentado se afecta outros containers
(provavelmente isolado a Grid).

#### ADR-0054 graded — refinos diferidos

ADR-0054 cobre todos os scope-outs em P157A/B/C:
- TableCell: align/stroke/fill/inset/breakable diferidos.
- TableHeader/Footer: level/repeat-rows + algoritmo repetição
  diferidos.
- Refinos individuais são extensíveis sem breaking change.

### §1.4 Outros Model

#### Promoção ADR-0060 a R1 (administrativo XS)

ADR-0060 status `IMPLEMENTADO` (Fase 1 fechada P155; Fase 2
sub-passo 3 fechado P157C; P158A + P159A continuam materializados).
Promoção R1 candidata para **anotar fechamento formal** de
table foundations + refinos figure + bibliography minimal.

#### Actualizar L0 prompt content.md (administrativo XS)

Variants Bibliography/Cite (P159A) adicionados ao código mas
prompt L0 `entities/content.md` permanece inalterado. Refino
candidato a actualizar L0 com documentação dos novos variants
(quebra hash inevitável).

#### ADR meta XS de "ADR-0064 caso completion"

P157C atingiu saturação cross-domínio cross-caso ADR-0064.
ADR meta candidato a **registar formalmente** a saturação como
patamar empírico válido — nenhum item pendente nos 4 casos
canónicos requer aplicação concreta nova.

---

## §2 — Inventário código pendente por família

### §2.1 Família 159 (Bibliography + Cite)

**Estado actual pós-P159A**:
- Variants `Content::Bibliography { entries: Vec<BibEntry>,
  title: Option<Box<Content>> }` + `Content::Cite { key: String,
  supplement: Option<Box<Content>> }`.
- Tipo `BibEntry { key, author, title, year }` em
  `entities/bib_entry.rs`.
- Stdlib `native_bibliography` + `native_cite` em
  `stdlib/structural.rs`.
- Helper `extract_bib_entries` privado.
- Layout placeholder render.
- Walk single-pass.

**Refinos pendentes** (campos diferidos vanilla):
- `BibEntry` adicional fields (volume/pages/journal/publisher/
  url/doi/isbn): extensível via `Option<String>` fields.
- `Content::Cite.form: Option<CitationForm>` (Normal/Prose/
  Author/Year/etc.): variant adicional ou enum.
- `Content::Cite.style`: CSL override (depende hayagriva).
- `Content::Bibliography.style`: CSL style (depende hayagriva).
- `Content::Bibliography.full: bool`: full vs cited only
  (depende validação cross-reference — ADR-0017).
- `Content::Bibliography.lang/region`: i18n.

### §2.2 Família 158 (figure-kinds)

**Estado actual pós-P158A**:
- Auto-detecção de kind via `infer_kind_from_body` em
  `stdlib/figure_image.rs`.
- Recursão limitada a `Content::Sequence`.
- Fallback chain `kind explícito > infer > "image"`.

**Refinos pendentes**:
- **Supplement automático por lang** (M; análogo a P155 quotes
  com `rules/lang/`):
  - Mapeamento `kind → prefix_localizado(lang)`.
  - Modificação em `introspect.rs` para emitir prefix correcto.
  - Reuso de padrão `localize_quotes(lang)` (P155).
- Refactor `kind: String → Option<String>` per ADR-0064 Caso A
  (XS): cosmetic; benefício marginal.
- Show selectors `figure.where(kind:)`: refactor de show rules
  (fora de Model puro).

### §2.3 Família 157 (table foundations)

**Estado actual pós-P157C**:
- Variants Table/TableCell/TableHeader/TableFooter completos.
- Layouter delega a `layout_grid` linear; sem placement
  estruturado.
- DEBT-34e armazenado mas não usado.
- DEBT-56 armazenado mas não usado.

**Refinos pendentes**:
- Cells refinos (align/stroke/fill/inset/breakable) — refino
  individual sem dependência cruzada hard, mas requer modelagem
  de `Stroke`, `Paint`, etc. em entities.
- Algoritmo placement Grid (DEBT-34e) — refactor extenso de
  `layout/grid.rs`.
- Repeat algorithm (DEBT-56) — refactor multi-region.

### §2.4 Outros Model

**Footnote** (`ausente` em A.6 Model): scope-out per decisão
humana 2026-04-25 (não incluído na Fase 1+2 Layout nem em
P156J/L/P157/P157A/B/C/P158/P158A/P159/P159A).

**Document/Title/Asset** (Fase 3 ADR-0060): divergência
intencional cristalino (export PDF directo; sem wrapper Content).

**List/Enum function form** (`parcial` em A.6 Model): sintaxe
parcial; sem function form completa. Refino M candidato.

**Caption** (`parcial` em A.6 Model): dentro de figure; sem
element dedicado. Refino candidato a element separado.

**Link** (`parcial`): `Content::Link` capturado; sem render
visual. Refino candidato.

**Hashes actuais relevantes**:
- `entities/content.rs`: `ec58d849` (preservado 9 passos).
- `entities/bib_entry.rs`: `5a2c0ebd` (P159A).
- `rules/stdlib/structural.rs`: hash via lineage.
- `rules/stdlib/figure_image.rs`: hash via lineage.

---

## §3 — Matriz de dependências cruzadas

### §3.1 Categoria A — Introspection runtime (ADR-0017)

| Refino candidato | Depende ADR-0017? | Notas |
|------------------|:------------------:|-------|
| Cite cross-document forward refs | **SIM hard** | Cite após bibliography mesmo doc OK; forward refs bloqueados |
| Bibliography validação `key ∈ entries` | **SIM soft** | Validação warning em walk single-pass possível sem ADR-0017 |
| Numbering autor-ano cross-references | **SIM hard** | Autor-ano exige resolver entries antes da render |
| Numbering numérico simples | NÃO | Counter local viável |
| Cite.form (Normal/Prose/etc.) | NÃO | Form é estilístico, não cross-document |
| Cite.supplement | NÃO | Já implementado P159A |
| Supplement por lang figure | NÃO | Lang é local |
| Show selectors `figure.where(kind:)` | NÃO directamente | Mas depende refactor show rules |

### §3.2 Categoria B — Refactor multi-region (DEBT-56)

| Refino candidato | Depende DEBT-56? | Notas |
|------------------|:----------------:|-------|
| TableHeader/Footer.repeat real | **SIM** | Confirmado P157C — algoritmo repetição em page breaks |
| Bibliography paginada | **SIM** | Bib longa requer flow cross-page |
| Bib `full: bool` (full vs cited) | NÃO directamente | Depende validação cross-reference |

### §3.3 Categoria C — Crate externa (hayagriva)

| Refino candidato | Depende hayagriva? | Notas |
|------------------|:------------------:|-------|
| CSL parsing/styles | **SIM hard** | hayagriva é o parser CSL |
| Bibliography parsing `.bib`/`.yaml` | **SIM hard** | hayagriva consume DataSource |
| Bib styles formatadas (APA, IEEE, MLA, etc.) | **SIM hard** | Cada style tem CSL definition |
| Cite.style override | **SIM hard** | Idem |
| Bib entries adicionais (Vec literal expandido) | NÃO | Extensão directa do BibEntry struct |
| Cite.form variants | NÃO | Form é cristalino-side |

### §3.4 Categoria D — ADR pendente de promoção

| Refino candidato | ADR pendente | Notas |
|------------------|:------------:|-------|
| hayagriva integration (qualquer) | ADR-0062 | Reserva sem ficheiro; promoção XS administrativa |
| Cite cross-document forward refs | ADR-0017 efectivamente promovida | Implica refactor extenso |
| Numbering autor-ano cross-refs | ADR-0017 | Idem |

### §3.5 Categoria E — Outro módulo (Layout/Introspection/Eval)

| Refino candidato | Toca outro módulo? | Notas |
|------------------|:------------------:|-------|
| Algoritmo placement Grid (DEBT-34e) | **Layout** | Refactor de `layout/grid.rs` |
| Refinos table cells (align/fill/stroke/etc.) | **Layout** parcial | Necessita modelagem `Stroke`, `Paint` em entities + render em layout |
| Cells colspan/rowspan visual | **Layout** (DEBT-34e) | Idem |
| Show selectors `figure.where(kind:)` | **Rules/show** | Refactor show rules |
| Cite cross-document forward refs | **Introspection** runtime | ADR-0017 |
| Bibliography paginada | **Layout** (DEBT-56) | Refactor multi-region |
| Supplement automático por lang | NÃO directamente | Modificação em `introspect.rs` é cross-módulo trivial (já fazemos isso para counters) |

### §3.6 Síntese

**Refinos puramente Model (sem dependência cruzada hard)**:
1. Supplement automático por lang em figure (Família 158).
2. Cite.form variants (Família 159) — Normal/Prose/Author/Year.
3. BibEntry fields adicionais (Família 159) — volume/pages/etc.
4. Refactor `kind: String → Option<String>` (Família 158) — XS cosmetic.
5. Numbering numérico simples para Bibliography (Família 159).

**Refinos com dependência ADR-0062 (hayagriva)**:
1. CSL parsing/styles (Família 159).
2. Bibliography `.bib`/`.yaml` parsing.
3. Cite.style override.

**Refinos com dependência cross-módulo hard**:
1. Algoritmo placement Grid (DEBT-34e; Layout).
2. Repeat header/footer real (DEBT-56; Layout).
3. Cite cross-document forward refs (ADR-0017; Introspection).
4. Show selectors `figure.where(kind:)` (Rules/show).
5. Cells refinos com Stroke/Paint (Layout + entities).

---

## §4 — Tecto realista de Model — sem entrar noutro módulo

### §4.1 Estado actual factual

**Cobertura Model agregada (impl + impl⁺)**: **50%** (11/22).
- 7 implementado: heading, terms, quote, divider, outline +
  table, ?
- 4 implementado⁺: figure, ref, numbering, heading com ressalva.

**Cobertura ampla (impl + impl⁺ + parcial)**: **24/22 entradas
considerando sub-entradas + parcial = 11+13 = 24** (parcial:
caption, list, enum, link, par, table.cell, table.header,
table.footer, cite, bibliography + outros).

**22 entradas totais Model A.6**.

### §4.2 Refinos puramente Model — estimativa

**Bloco A** (5 refinos sem dependência cruzada hard):

| Refino | Tamanho | Move agregada? | Move ampla? | Tests Δ |
|--------|---------|:--------------:|:-----------:|--------:|
| Supplement por lang figure (P158B?) | M | Não (figure já impl⁺) | Não (refino qualitativo) | +12-15 |
| Cite.form variants (P159C?) | M | Sim (cite parcial → impl?) | Sim | +10-15 |
| BibEntry fields adicionais (P159D?) | S+ | Não (refino struct) | Não | +5-8 |
| Refactor kind: String → Option<String> | XS | Não | Não | +2-4 |
| Numbering numérico simples Bibliography (P159E?) | M | Sim (bib parcial → impl?) | Sim | +10-15 |

**Limite teórico Model puro**: aproximadamente +2-3 entradas
podem mover de `parcial → implementado` (cite + bibliography
se completarem com form + numbering); supplement por lang
mantém figure em implementado⁺ mas refina; outras entradas
parciais (caption/list/enum/link/par) requerem trabalho dedicado
fora do scope família 159.

**Tecto Model puro estimado**: cobertura agregada **50% → ~55-60%**
(alcançável com 3-4 sub-passos M/M+ aplicados a Bloco A).
Cobertura ampla cresce ligeiramente; cobertura arquitectural
mantém-se ~82% (sem novos variants Content).

### §4.3 Refinos pós-resolver dependências (informativo)

**Bloco B** (5 refinos com hayagriva ADR-0062):
- CSL parsing (XL).
- Bibliography paginada (XL).
- Styles APA/IEEE/MLA/etc. (XL agregado).
- Cite.style override (M se hayagriva integrada).
- Cite cross-document forward refs (depende ADR-0017 também).

**Tecto Model + hayagriva estimado**: cobertura agregada
**~55-60% → ~68%** (paridade ADR-0060 declarado).

**Bloco C** (cross-módulo): cobertura adicional difícil de
estimar — depende prioridade humana entre Model/Layout/
Introspection.

### §4.4 Distinção operacional confirmada

- **Refino qualitativo**: Bloco A items que não movem agregada
  (supplement, refactor kind).
- **Materialização nova**: Bloco A items que movem agregada
  (cite.form, bib.numbering — entradas movem `parcial → impl`).
- **Diferimento**: Bloco B (hayagriva) e Bloco C (cross-módulo)
  ficam scope-out per ADR-0054 graded; documentar honestamente.

---

## §5 — Sequência candidata sub-passos

### §5.1 Bloco A — Refinos sem dependência cruzada hard (ordenado)

| Ord | Identificador sugerido | Refino | Tamanho | Hash content.rs impacto | ADR-0064 esperada | Tests Δ |
|---:|:---------------------|--------|---------|:-----------------------:|:------------------:|--------:|
| 1 | **P158B** | Supplement automático por lang em figure | M | preservado (refino L0+stdlib+introspect) | NÃO directamente | +12-15 |
| 2 | **P159C** | Cite.form variants (Normal/Prose/etc.) | M | quebrado (variant Cite expande field; ou enum novo) | Caso A para form | +10-15 |
| 3 | **P159D** | BibEntry fields adicionais (volume/pages/etc.) | S+ | preservado (struct extensão; sem variant Content) | (não aplicável) | +5-8 |
| 4 | **P158C** ou **P159E** | Refactor `kind: String → Option<String>` | XS | quebrado (Figure variant refino) | Caso A | +2-4 |
| 5 | **P159F** | Numbering numérico simples Bibliography | M | preservado (refino layout + entities apenas) | NÃO directamente | +10-15 |

### §5.2 Bloco B — Refinos com ADR-0062 (hayagriva)

Pré-requisito: **ADR-0062 promovida** (passo administrativo XS;
criar ficheiro `typst-adr-0062-hayagriva-autorizacao.md`).

| Ord | Identificador | Refino | Tamanho | Pré-requisitos |
|---:|:-------------|--------|---------|----------------|
| 1 | **ADR-0062 XS** | Criar ADR-0062 PROPOSTO | XS | — |
| 2 | **P159G** | Cargo.toml + crystalline.toml hayagriva | XS | ADR-0062 |
| 3 | **P159H** | hayagriva integration minimal (parsing entries) | M+ | P159G |
| 4 | **P159I** | CSL styles APA simples | M | P159H |
| 5 | **P159J** | CSL styles adicionais (IEEE/MLA/Chicago) | M cada | P159I |

### §5.3 Bloco C — Refinos com dependência cross-módulo

NÃO materializáveis em Model puro:
- Algoritmo placement Grid (DEBT-34e; Layout).
- Repeat header/footer real (DEBT-56; Layout).
- Cite cross-document forward refs (ADR-0017; Introspection).
- Show selectors `figure.where(kind:)` (Rules/show).
- Cells refinos com Stroke/Paint (Layout + entities).
- Bibliography paginada (DEBT-56; Layout).

---

## §6 — Recomendação concreta para passo seguinte

### §6.1 Ordem recomendada

**Recomendação primária**: **P158B — Supplement automático por
lang em figure**.

Justificação:
1. **Sem dependência cruzada hard** (refino comportamental
   localizado).
2. **Reuso de padrão consolidado**: `localize_quotes(lang)` em
   `rules/lang/quotes.rs` (P155) é precedente directo.
3. **Hash content.rs preservado** (modificação só em
   stdlib/figure_image.rs + introspect.rs).
4. **Tamanho M** preserva cadência granular.
5. **ADR-0064 NÃO directamente aplicável** mas continua estável.
6. **Funcionalidade visível** — `figure(image(...))` automaticamente
   prefixado com "Figura"/"Figure"/"Abbildung" baseado em lang
   activo. Melhoria UX significativa.

**Recomendação secundária** (se P158B for redirigido):
**P159D — BibEntry fields adicionais** (S+; refino struct sem
variant; menor complexidade).

**Recomendação terciária** (passo administrativo XS):
**Criar ADR-0062 PROPOSTO** + actualizar L0 prompt content.md.

### §6.2 Estimativa de saturação Bloco A

Com Bloco A completo (5 sub-passos), Model atinge tecto
**~55-60% agregada**; cobertura ampla cresce continuamente.
Após Bloco A, dependências cruzadas ficam evidentes —
**decisão humana sobre próxima direcção** (hayagriva, Layout,
Introspection) tem máxima informação acumulada.

### §6.3 Decisão crítica registada

**Tecto Model puro vs pós-resolver dependências**:

| Cenário | Cobertura agregada | Esforço | Próxima direcção |
|---------|:------------------:|:-------:|------------------|
| Pós-Bloco A apenas | ~55-60% | 3-4 sub-passos M | Decidir hayagriva ou cross-módulo |
| Pós-Bloco A + Bloco B (hayagriva) | ~68% | +5-8 sub-passos M-XL | Refinos restantes Layout/Introspection |
| Pós-Bloco C (cross-módulo) | difícil estimar | refactor extenso | Saturação |

**Recomendação operacional**: **executar Bloco A primeiro** —
informação acumulada após cada sub-passo melhora decisão sobre
Bloco B/C.

---

## Resumo executivo

P159B é **diagnóstico amplo** (M-) análogo a P156B (Layout)
para Model. Confirma factualmente:

1. **Família 159 (Bibliography + Cite)**: DEBT-55 plano ainda
   tem 6/10 itens pendentes; ADR-0062 reserva sem ficheiro;
   refinos sem hayagriva possíveis (form, fields adicionais,
   numbering simples).
2. **Família 158 (figure-kinds)**: subset máximo (supplement
   por lang) materializável sem dependência cruzada — paridade
   P155 quotes lang.
3. **Família 157 (table foundations)**: refinos requerem cross-
   módulo (DEBT-34e Layout; cells Stroke/Paint Layout+entities).
4. **5 refinos no Bloco A** (sem dependência cruzada hard):
   supplement figure (M); cite.form (M); BibEntry fields (S+);
   kind refactor (XS); bib numbering simples (M).
5. **Tecto Model puro estimado**: ~55-60% agregada (alcançável
   com 3-4 sub-passos M Bloco A).
6. **Tecto Model + hayagriva**: ~68% (paridade ADR-0060
   declarado).
7. **Recomendação primária**: **P158B — Supplement automático
   por lang em figure** (M; reuso padrão lang P155; hash
   content.rs preservado; funcionalidade visível).

**Auto-validação ADR-0065 critério #5**: quarta aplicação concreta
após P157/P158/P159 com **diversidade ampliada multi-feature**.
P159B inventaria múltiplas famílias simultaneamente — primeira
aplicação multi-feature do critério #5. Patamar empírico cresce.

**Política "sem novas reservas" preservada** — recomendações
em §5/§6 são para validação humana, não compromissos.
