# ⚖️ ADR-0060: Model (structural) roadmap — Fase 1 + Fase 2 + Fase 3

**Status**: `IMPLEMENTADO` (Fase 1 fechada; Fase 2 e Fase 3 prosseguem
como roadmap planeado e aplicam-se em passos subsequentes —
**P157/158/159** após renumeração registada em P156B; ver anotação
abaixo).
**Validado**: Passo 154A — diagnóstico; Passo 154B — sub-passo 1
(terms + divider); **Passo 155 — sub-passo 2 (quote); Fase 1 fechada**.
**Data**: 2026-04-25
**Autor**: Humano + IA
**Diagnóstico prévio**:
[`00_nucleo/diagnosticos/diagnostico-model-passo-154a.md`](../diagnosticos/diagnostico-model-passo-154a.md)

**Anotação Passo 154B (2026-04-24)**: primeiro sub-passo da Fase 1
materializado — `Content::Divider`, `Content::Terms`,
`Content::TermItem` adicionados ao enum `Content`; `native_terms`
e `native_divider` registadas em `make_stdlib`. Sem ADR nova.
Status permaneceu `PROPOSTO` aguardando Passo 155.

**Anotação Passo 155 (2026-04-25)**: segundo sub-passo da Fase 1
materializado — `Content::Quote { body, attribution, block, quotes }`
adicionado ao enum; `native_quote` registada em `make_stdlib`;
módulo novo `01_core/src/rules/lang/quotes.rs` com
`localize_quotes(lang)` cobrindo 6 idiomas (`pt`/`en`/`de`/`fr`/`es`/`it`)
+ default ASCII; `eval_markup` actualizado para tratar
`SyntaxKind::SmartQuote` (alternância open/close por sequência markup
emitindo glyph localizado). Regression test garante que `"..."` em
contexto de código (ex: `#let s = "..."`) continua a ser
`Value::Str`. **Fase 1 fechada**. Status `PROPOSTO → IMPLEMENTADO`.
Cobertura Model 41% → ~45%; arquitectural Content 75% → ~77%.
Plano Fase 2 (P156/157/158 — table foundations, figure kinds,
bibliography+cite com ADR-0061) inalterado.

**Anotação Passo 159C (2026-04-27)**: **segundo sub-passo
substantivo Bibliography + Cite materializado** (Fase 2
continuação após P159A par acoplado). Refino estrutural-
comportamental de `cite` adicionando enum
`CitationForm { Normal, Prose, Author, Year }` em
`entities/citation_form.rs` (ficheiro novo; **5ª aplicação
consecutiva** do padrão "tipo entity em ficheiro próprio" —
Sides P156C → Parity P156E → Dir P156I → BibEntry P159A →
CitationForm P159C) + field `form: Option<CitationForm>` em
`Content::Cite` per **ADR-0064 Caso A** (patamar Caso A cresce
**N=5 → 6 atingindo equilíbrio cross-domínio 50/50 Layout/Model**
— terceira aplicação Model após P157B/P159A). 13 sítios
pattern-match Content actualizados (paridade P157A/P159A).
Helper privado novo `extract_citation_form` em
`stdlib/structural.rs` (strict matching case-sensitive; 4
forms válidos; mensagem de erro lista forms aceites). Layout
placeholder melhorado por form com lookup Bibliography via
novo field `pub bib_entries: Vec<BibEntry>` em `CounterState`
(paridade infraestrutural P158B `state.lang`); populado por
introspect walk; multi-Bibliography concatena na ordem de
aparecimento. Render: `Normal/None ↔ [key]`; `Prose ↔ Author
(Year)`; `Author ↔ Author`; `Year ↔ Year`; fallback `[key]`
quando key não encontrada (paridade Normal sem entry). **Sem
alteração ao número de variants Content** (estrutura
inalterada; expansão de field). **Subpadrão emergente N=2**
"infraestrutura state lookup" (P158B `state.lang` + P159C
`state.bib_entries`). Tests +15 (1189 → 1204; 8 unit +
2 cite com form + 6 stdlib parse + 4 layout E2E forms; range
esperado +12-17). Cobertura Model agregada **inalterada**
(~50%). Cobertura arquitectural **inalterada** 82%. Hash
`entities/content.rs` preservado `ec58d849` (**décimo segundo
passo consecutivo** via L0-baseline interpretation —
content.md prompt não modificado; refino arquitectural
documentado via doc-comments + referência cruzada
citation_form.md). Status `IMPLEMENTADO` mantido. **Política
"sem novas reservas" preservada** — forms vanilla adicionais
(Full, CSL-specific), CSL render real (depende hayagriva
ADR-0062), `style: Str` per-Cite, cross-document refs
(bloqueado ADR-0017), promoção `extract_citation_form` a
helper público permanecem candidatos NÃO-reservados.

**Anotação Passo 158B (2026-04-27)**: **segundo sub-passo
Model figure-kinds materializado** (Fase 2 continuação após
P158A). Refino qualitativo de `figure` — supplement
automático localizado por lang. Helper novo
`figure_supplement_for_lang(kind: &str, lang: Option<&Lang>)
-> String` em `rules/lang/figure_supplement.rs` cobrindo
6 langs (pt/en/de/fr/es/it) × 3 kinds (image/table/raw) =
18 entradas + fallback PT por kind + capitalização para kind
desconhecido. Field novo `pub lang: Option<Lang>` em
`CounterState` para lang resolution (default `None` →
fallback PT, paridade backwards compat com tests
pré-existentes que esperam "Figura"). Modificação trivial
em `introspect.rs` linha 334: `Some(format!("Figura {}", n))`
→ `Some(format!("{} {}", figure_supplement_for_lang(kind,
lang), n))`. **Sem alteração ao variant `Content::Figure`**
(estrutura inalterada). **Reuso explícito do padrão P155**
`localize_quotes(lang)` em `rules/lang/quotes.rs` —
**primeiro reuso cross-feature** (quotes → figure supplement);
estrutura paralela: tabela estática + lookup linear + fallback;
**subpadrão emergente N=1** "padrão P155 i18n reusado
cross-feature" (candidato a formalização N=3-4 mínima).
Tests +15 (1174 → 1189; 8 unit em figure_supplement.rs +
7 integration em introspect.rs; range esperado +12-15).
Cobertura Model agregada **inalterada** (~50%) — segundo
refino qualitativo consecutivo de `figure`. Cobertura
arquitectural **inalterada** 82% (refino de variant existente).
Hash `entities/content.rs` preservado `ec58d849` (**décimo
primeiro passo consecutivo**). Status `IMPLEMENTADO` mantido.
**Política "sem novas reservas" preservada** (estabelecida
em P158; respeitada em P158A/B): `supplement: Option<Content>`
field user-facing, mais langs além de 6 minimais, CSL-aware
format (depende hayagriva), region-specific supplements
permanecem candidatos NÃO-reservados.

**Anotação Passo 159A (2026-04-27)**: **par acoplado
Bibliography + Cite minimal materializado** (Fase 2 continuação
após figure-kinds P158A). **Estrutura A adaptada** per diagnóstico
P159 §3.5 — par num único passo M+ sem hayagriva. Tipo entity
novo `BibEntry { key, author, title, year }` em
`entities/bib_entry.rs` (4 fields universais; ADR-0065 critério
#2 escolha de tipo primeira aplicação isolada concreta).
Variants `Content::Bibliography { entries: Vec<BibEntry>,
title: Option<Box<Content>> }` + `Content::Cite { key: String,
supplement: Option<Box<Content>> }` adicionados ao enum (56 →
58 variants). Stdlib `native_bibliography` + `native_cite`
em `stdlib/structural.rs` (continuação Model). **Naming
`bibliography` e `cite` flat** (paridade decisão P157B
naming flat). **Helper privado novo `extract_bib_entries`**
parseia `Value::Array<Value::Dict>` para `Vec<BibEntry>` com
validação hard de 4 fields obrigatórios. **ADR-0064 Caso A**
aplicado em title (Bibliography) e supplement (Cite) — patamar
Caso A cresce **N=4 → 5** (P156G/H/I + P157B + P159A; 60% Layout
+ 40% Model). Layouter renderiza placeholder per ADR-0033 +
ADR-0054 graded — Bibliography como lista
`"[{key}] {author}. {title} ({year})."` per linha; Cite como
`"[{key}]"` + supplement. **Sem validação cross-reference**
`Cite.key ∈ Bibliography.keys` per ADR-0017 Introspection
runtime adiada. **Sem hayagriva** — input cristalino literal
`Vec<BibEntry>`; ADR-0062 mantém-se reserva sem ficheiro.
Tests +27 (1147 → 1174). Cobertura Model agregada **inalterada**
(50%); cobertura ampla impl+impl⁺+parcial cresce
(22 → 24 entradas parciais — `cite` e `bibliography` movem
`ausente → parcial`); cobertura arquitectural **80% → 82%**
(2 variants novos). Status `IMPLEMENTADO` mantido. **Granularidade
quebrada honestamente** N=13 → M+ com precedente P156C par
lógico pad+hide. **DEBT-55 contribuído mas NÃO fechado** —
refinos futuros (hayagriva integration, CSL, form variants
Normal/Prose, numbering schemes, cross-document forward refs)
**NÃO reservados** per política P158. Hash `entities/content.rs`
mantém-se `ec58d849` (nono passo consecutivo — hash refere-se
ao prompt L0 que permanece inalterado; refino futuro pode
actualizar L0 com documentação dos novos variants).

**Anotação Passo 158A (2026-04-27)**: **primeiro sub-passo
Model figure-kinds materializado** (Fase 2 continuação após
table foundations fechado P157C). Refino qualitativo de
`native_figure`: helper privado novo `infer_kind_from_body(body:
&Content) -> Option<String>` em `stdlib/figure_image.rs` cobrindo
Image/Table/Raw + recursão limitada a `Content::Sequence`
(paridade vanilla parcial per ADR-0033 — vanilla usa
`query_first_naive` recursivo profundo; cristalino limita a
Sequence per decisão P158A §8). Fallback chain 3 níveis em
`native_figure`: `kind:` explícito > inferência > default
`"image"` (precedência absoluta para `kind:` explícito preserva
tests pré-existentes). **Sem alteração ao variant `Content::Figure`**
(estrutura inalterada; `kind: String` continua directo).
**Sem alteração a `introspect.rs` ou layout** (counters por
kind continuam funcionar inalterados — refino vive só na
origem do valor `kind`). Tests +6 (1141 → 1147; range esperado
+6-8). Cobertura Model agregada **inalterada** (~50%) — refino
qualitativo. Hash `entities/content.rs` preservado `ec58d849`
(sétimo passo consecutivo). Status `IMPLEMENTADO` mantido.
**Política "sem novas reservas" preservada** (estabelecida em
P158): supplement automático por lang, show selectors
`figure.where(kind:)`, refactor `kind: String → Option<String>`
permanecem candidatos NÃO-reservados.

**Anotação Passo 157C (2026-04-26)**: **terceiro e último
sub-passo Fase 2 Model — "table foundations" fechado**.
Par simétrico `Content::TableHeader { body, repeat: bool }` +
`Content::TableFooter { body, repeat: bool }` adicionados ao
enum (54 → 56 variants); `native_table_header` e
`native_table_footer` registadas em `make_stdlib` em
`stdlib/structural.rs` (continuação P157A/B). **Naming
`table_header`/`table_footer` flat** (paridade decisão P157B).
**Primeira aplicação concreta de ADR-0064 Caso D em domínio
Model** (`bool` directo com default `true` paridade vanilla;
P156D weak / P156G breakable / P156J justify aplicaram-no em
Layout). **Saturação cross-domínio cross-caso ADR-0064**: após
P157C, todos os 4 casos canónicos A/B/C/D validados em Layout;
3/4 (A, C, D) validados em Model. Layouter renderiza body
no contexto actual; **`repeat` armazenado mas ignorado em layout**
per ADR-0054 graded — **DEBT-56 permanece aberto** (algoritmo
de repetição em page breaks fica para refactor multi-region).
**Divergência aceite per ADR-0033**: `body: Box<Content>` em
vez de vanilla `#[variadic] children: Vec<TableItem>` para
uniformidade com containers cristalinos existentes; `level`
(Header) e `repeat-rows` scope-out per ADR-0054 graded. Helper
privado novo `extract_bool_with_default(args, fn, field, default)`
parametrizado (genérico no key e no default; distinto de
`extract_weak` específico por field/default — separação de
domínios per ADR-0037). Tests +26 (1353 → 1379). Cobertura
Model agregada **inalterada** (50% mantém-se — sub-entradas
qualitativas paridade P157B); cobertura arquitectural **78%
→ 80%** (variants Content vanilla extra ausentes desce de ~1
a 0). Status `IMPLEMENTADO` mantido. **Padrão cross-domínio
fortalecido**: 3 sub-passos Model consecutivos sem reformulação;
**N=12 materialização**. **"Table foundations" integralmente
materializado** com P157A + P157B + P157C (3 sub-passos M cada;
granularidade preservada N=10/11/12). Promoção a revisão R1
da ADR-0060 candidata (passo administrativo XS) se decisão
humana for prioritária.

**Anotação Passo 157B (2026-04-26)**: **segundo sub-passo
Fase 2 Model materializado** — `Content::TableCell { body,
x: Option<usize>, y: Option<usize>, colspan: Option<usize>,
rowspan: Option<usize> }` adicionado ao enum (53 → 54 variants);
`native_table_cell` registada em `make_stdlib` em
`stdlib/structural.rs` (continuação P157A). **Naming `table_cell`
flat** (não vanilla `table.cell` — FieldAccess actual não suporta
namespacing de funcs `Value::Func.subname`; divergência intencional
documentada per ADR-0033 em diagnóstico P157B §8). **Primeira
aplicação concreta de ADR-0064 Caso A em domínio Model**
(`Smart<usize>` → `Option<usize>` para x/y); **terceira aplicação
global de Caso C** com **primeira variação `usize`**
(NonZeroUsize default 1 → Option<usize> com None ↔ default 1
para colspan/rowspan; zero rejeitado em stdlib paridade vanilla).
Layouter renderiza body no contexto actual; **x/y/colspan/rowspan
armazenados mas ignorados em layout** per ADR-0054 graded —
**DEBT-34e permanece aberto** (placement algorítmico Grid
completo fica para refactor dedicado). Helper privado novo
`extract_usize_or_none_min(val, fn, field, min)` parametrizado
(min=0 para x/y; min=1 para colspan/rowspan). Tests +18
(1335 → 1353). Cobertura Model agregada **inalterada** (50%
mantém-se — `table.cell` é sub-entrada de `table` per padrão
P154A; ganho qualitativo via expansão estrutural). Status
`IMPLEMENTADO` mantido. **Padrão cross-domínio reforçado**:
2 sub-passos Model consecutivos (P157A/B) sem reformulação;
N=11 materialização. TableHeader/Footer ficam para **P157C**.

**Anotação Passo 157A (2026-04-26)**: **primeiro sub-passo
Fase 2 Model materializado** — `Content::Table { columns:
Vec<TrackSizing>, rows: Vec<TrackSizing>, children: Vec<Content> }`
adicionado ao enum (52 → 53 variants); `native_table` registada
em `make_stdlib` em **`stdlib/structural.rs`** (decisão de
módulo Model existente, não novo `stdlib/model.rs` — per
diagnóstico P157A §8). Subset minimal per ADR-0054 graded:
3 fields críticos; ~9 atributos vanilla scope-out; TableCell
estruturado diferido para **P157B**; TableHeader/Footer
diferidos para **P157C**. Layouter delega a `layout_grid`
clone simples per Decisão 4 (sem modificação de `grid.rs`).
Helper `extract_tracks` promovido a `pub(super)` para reuso
cross-módulo (N=2; subpadrão emergente). Tests +16
(1319 → 1335). Cobertura Model 45% → 50% (entrada `table`
transita `ausente → implementado`). Status `IMPLEMENTADO`
mantido (Fase 1 fechada P155 não muda; Fase 2 prossegue per
roadmap). **Padrão cross-domínio confirmado**: granularidade
N=10 estendida de Layout (P156C-L) a Model (P157A) sem
reformulação.

**Anotação Passo 156B (2026-04-25)** — **renumeração de Fase 2**:
P156A foi consumido pelo historiograma (passo administrativo);
P156B é o diagnóstico Layout (este passo de origem da anotação).
Consequentemente Fase 2 Model desloca-se uma posição:

| Antes (ADR-0060 original) | Depois (pós-P156B) |
|---------------------------|---------------------|
| P156 = Model table foundations | **P157** |
| P157 = Model figure-kinds | **P158** |
| P158 = Model bibliography (XL) | **P159** |
| ADR-0061 = autorização hayagriva | **ADR-0062** |

ADR-0061 foi **reocupada** por P156B para roadmap Layout
(`typst-adr-0061-layout-fase-x-roadmap.md`, status `PROPOSTO`).
`hayagriva` passa a reserva ADR-0062 (sem ficheiro criado;
documentado em README ADRs e DEBT-55). Decisão 2 desta ADR-0060
(Fase 2 — `Content::Bibliography` + `Content::Cite` com
autorização `hayagriva`) lê-se agora "ADR-0062 + Passo 159"
em vez de "ADR-0061 + Passo 158". DEBT-55 actualizada em P156B.

Bloqueio adicional documentado em P156B: **`footnote()` (Decisão 2
desta ADR-0060) requer page model com footnote area** — desbloqueado
pela Fase 1 do roadmap Layout (ADR-0061 nova, Decisão 1 + Decisão 5;
Passo 156C).

---

## Contexto

Inventário 148 §A.6 declara categoria Model (structural) com
**21 entradas** e cobertura 38% (impl + impl⁺). P154A
investigou empiricamente: contagem real = **22 entradas**;
cobertura empírica = **32-36%** (revisão para baixo).

Decomposição empírica (P154A §2):

- 3-4 `implementado` (heading, emph, strong, outline).
- 4 `implementado⁺` (figure, ref, numbering, heading com
  ressalva).
- 5 `parcial` (link, list, enum, par, caption inline).
- **10 `ausente`** (bibliography, cite, footnote, quote,
  terms, table, document, divider, asset, title).

Top divergência 7 do inventário 148 ("~14 elementos `Content::*`
vanilla ausentes") agrega Model + Layout + Visualize. Para
Model especificamente, 6 dessas entradas são alto valor:
`bibliography`, `cite`, `footnote`, `quote`, `terms`,
`table`. Restantes (`document`, `divider`, `asset`, `title`)
são baixo valor ou divergência intencional.

`Content::Styled` (ADR-0026 perfil) é **inadequado** para
Model structural — estas features têm semântica que excede
styling.

ADR-0017 (estratégia typst-library) declarou progressão
gradual; este roadmap operacionaliza a continuação.

## Decisão

ADR-0060 propõe **3 fases** com prioridades explícitas:

### Decisão 1 — Fase 1 (S+M; sem novas crates)

3 sub-passos:

- **Passo 154B** — `Content::Terms` + `Content::TermItem` +
  `Content::Divider` (S agregado).
- **Passo 155** — `Content::Quote` com atributos
  `attribution`, `block` (M).
- **Passo 157** (renumerado de P156 em P156B) —
  `Content::Table` foundations: variant nova + sub-elementos
  `TableCell`, `TableHeader`, `TableFooter` (M+; reaproveita
  `Content::Grid` parcial para layout).

Cobertura post-Fase 1: ~50% (8/22 → 11-12/22).

### Decisão 2 — Fase 2 (com ADR de autorização)

3 sub-passos:

- **Passo 158** (renumerado de P157 em P156B) — `figure` kinds
  extension (depende de Passo 157 para figure-table; M).
- **ADR-0062 + Passo 159** (renumerados de ADR-0061+P158 em P156B)
  — `Content::Bibliography` + `Content::Cite` com autorização
  `hayagriva`. ADR-0062 documenta autorização (precedente
  ADR-0024 ecow, ADR-0023 indexmap, ADR-0057 hypher). Crate
  `hayagriva 0.9.1` já em cache local (per P152). **Nota**:
  ADR-0061 foi reocupada por P156B para roadmap Layout; reserva
  hayagriva passou para ADR-0062.
- **Passo dedicado footnote** — `Content::Footnote` desbloqueado
  por Fase 1 Layout (ADR-0061 nova; Passo 156C). Page model
  ganha `footnote_area` minimalista.

Cobertura post-Fase 2: ~68% (11-12/22 → 15-16/22).

### Decisão 3 — Fase 3 (condicional / divergência intencional)

- **`asset`**: alt-text + scaling sobre `Image`. Acessibilidade.
- **`document`**: divergência intencional cristalino emite
  metadata em export PDF directamente; sem wrapper Content.
- **`title`**: depende de `document`; mesma divergência.

Cobertura potencial: ~77-82% (com restantes em scope-out
declarado).

### Decisão 4 — `Content::Styled` vs variant novo

Para cada feature Fase 1/2: **variant novo** no `Content`
enum.

Razão: Model structural tem semântica que excede styling
(numbering, attribution, cells, citations). `Content::Styled`
(ADR-0026 perfil) cobre apenas estilos visuais simples.
Todas as Fase 1/2 features exigem variants dedicados.

### Decisão 5 — Relação com `lab/parity` corpus

Cada sub-passo Fase 1/2 deve **adicionar 1-3 ficheiros** ao
corpus `lab/parity/corpus/visual/` ou `corpus/markup/`
exercitando a feature nova. Suite layout_parity (P150)
detecta automaticamente; matriz P3 cresce.

Quando vanilla integration fechar (DEBT-53 + DEBT-54), as
mesmas features ganham comparação real.

## Alternativas consideradas

| Alternativa | Prós | Contras |
|-------------|------|---------|
| **Fases 1+2+3 ranqueadas** ✓ | Materialização gradual; cobertura predictível; ADRs por trabalho específico | Trabalho longo (5+ passos) |
| Atacar tudo num passo XL | Único output | Risco alto; mistura concerns; dificil revisão |
| Adiar Model completo até DEBT-53 + DEBT-54 fechar | Foco na série paridade primeiro | Cobertura observacional cristalino-only fica fraca; impede eval real do gap |
| Apenas Fase 1 com ADR limitada | Mínimo risco | Não responde a "trabalho real necessário" |
| ADR única para todas as fases (sem 0061) | Menos ADRs | `hayagriva` exige autorização explícita conforme precedente |

**Escolha**: 3 fases com Fase 2 ganhando ADR-0061 dedicada
para `hayagriva`. Fase 3 condicional sem ADR (decisão
humana posterior).

## Consequências

### Positivas

- **Roadmap explícito** para sair de cobertura Model 32%
  para ~68% sem comprometer ADR-0017 (estratégia gradual).
- **Cada sub-passo tem escopo S/M definido** (excepto
  bibliography Fase 2 = XL com ADR-0061).
- **Corpus paridade cresce automaticamente** com cada
  sub-passo (Decisão 5).
- **Footnote desacoplado** da Fase 1 — não bloqueia features
  simples.
- **`hayagriva` em cache** (probe P152): risco de fetch
  reduzido.

### Negativas

- **5-7 sub-passos entre P154B e P158+** — investimento
  significativo de tempo.
- **Fase 3 condicional**: documentos com `#document(...)`
  ou `#title(...)` continuam não-suportados. Aceitável
  conforme inventário 148 e ADR-0033 perfil graded.
- **`hayagriva` em L1**: precedente ADR-0024 (ecow) +
  ADR-0057 (hypher) cobrem; ADR-0061 invocará.

### Neutras

- Inventário 148 ganha referências cruzadas para ADR-0060
  (per Decisão 5 + actualização P154A).
- `Content` enum cresce: 38 variants → ~46 variants
  pós-Fase 2. ADR-0026-R1 (`Arc<[T]>` em `Sequence`) cobre
  performance de clone.

## Plano de materialização

5 passos no caminho crítico (Fase 1 + Fase 2):

| Passo | Escopo | Features | ADR adicional? |
|-------|--------|----------|-----------------|
| 154B | S | terms, divider | — |
| 155 | M | quote | — |
| 157 (renumerado de 156 em P156B) | M+ | table foundations | — |
| 158 (renumerado de 157 em P156B) | M | figure kinds | — |
| ADR-0062 + 159 (renumerados em P156B) | XL | bibliography + cite | ADR-0062 (era ADR-0061 antes da reocupação por Layout em P156B) |
| (futuro pós-156C) | M-L | footnote | — (Layout Fase 1 ADR-0061 desbloqueia) |
| (Fase 3) | S | asset | — |
| (Fase 3) | divergência | document, title | — |

**ADR-0060 transitou `PROPOSTO → IMPLEMENTADO`** em Passo 155
ao fechar a Fase 1 (terms + divider em P154B; quote em P155).
A Fase 2 (table/figure-kinds/bibliography) e a Fase 3
(asset/document/title) prosseguem como planeado em P156–P158+
sem necessidade de re-abertura desta ADR.

## Referências

- **ADR-0017** — estratégia typst-library gradual.
- **ADR-0026** + **ADR-0026-R1** — `Content` enum fechado
  com `Arc<[T]>` para sequences.
- **ADR-0033** — paridade funcional para cada feature
  materializada.
- **ADR-0034** — diagnóstico obrigatório (cumprido por
  P154A).
- **ADR-0036** — atomização progressiva.
- **ADR-0037** — coesão por domínio.
- **ADR-0038** — `Content::Styled` para styling estrutural.
- **ADR-0054** — perfil observacional graded.
- **DEBT-55** (P154A; actualizada por P156B) — bibliography
  + cite XL com plano **ADR-0062 + Passo 159** (era ADR-0061
  + Passo 158 antes da renumeração).
- **ADR-0061** (P156B) — Layout Fase X roadmap; reocupou o
  número antes reservado para hayagriva.
- **DEBT-56** (P156B) — Column flow Fase 3 Layout L+; aberto
  por P156B.
- **DEBT-34d / DEBT-34e** — grid cell layouting (Passo 80);
  trabalho similar mas distinto de `Content::Table`.
- **Inventário 148** (`typst-cobertura-vanilla-vs-cristalino.md`)
  — Tabela A linha "Model"; §7 entrada 7.
- **Diagnóstico 154A** (`diagnostico-model-passo-154a.md`)
  — Tabelas §2, §3, §6, §7 com plano detalhado.
