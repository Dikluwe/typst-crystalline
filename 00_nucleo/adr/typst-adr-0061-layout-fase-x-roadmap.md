# ⚖️ ADR-0061: Layout Fase X — page model + multi-column + footnote area roadmap

**Status**: `PROPOSTO`
**Data**: 2026-04-25
**Autor**: Humano + IA
**Validado**: Passo 156B — diagnóstico.
**Diagnóstico prévio**:
[`00_nucleo/diagnosticos/diagnostico-layout-passo-156b.md`](../diagnosticos/diagnostico-layout-passo-156b.md)

**Nota de reocupação**: ADR-0061 estava reservada para autorização da
crate `hayagriva` (per blueprint pré-P156A + relatório 154A); **reocupada
por este passo P156B** para roadmap Layout. **`hayagriva` passa para
ADR-0062** (próxima reserva). DEBT-55 (bibliography + cite) actualizada
com a nova referência.

---

## Contexto

Inventário 148 §A.5 declara categoria Layout com **17 entradas** e
cobertura 38% (impl + impl⁺ = 6/0/2/8/0=16 — 1 entrada duplicada
contada uma vez). Diagnóstico P156B confirmou empiricamente:

- Cobertura empírica recalculada: **22% implementado puro** (4/18);
  **39% incluindo `parcial`** (próximo do 38% declarado mas com
  redistribuição).
- 4 entradas implementadas: `align`, `move`, `rotate`, `scale`.
- 3 entradas parciais: `place` (sem float/clearance), `grid`
  (sem gutter/header/footer/colspan), `measure` (heurística privada).
- 11 entradas ausentes: `pad`, `box`, `block`, `stack`, `hide`,
  `repeat`, `columns`, `colbreak`, `pagebreak` (manual), `h`/`v`
  (combinada), `skew`.
- **2 entradas adicionais não listadas em §A.5**: `h()`/`v()`
  spacing primitives (vanilla `HElem`/`VElem`) e `skew`. Adicionadas
  por P156B.
- **3 reclassificações face ao inventário declarado**: `pad`
  (parcial → ausente); `pagebreak` (parcial → ausente para manual);
  `grid` (impl⁺ → parcial). Padrão análogo ao recálculo Model 154A.

**Bloqueante crítico identificado**: footnote area no page model.
`Page` actual (`01_core/src/entities/layout_types.rs:359`) é
`{ width, height, items: Vec<FrameItem> }` — sem campo
`footnote_area`, sem header/footer/background/foreground. Vanilla
gere footnotes em `FlowState` separado, mas para paridade
mínima cristalino precisa de extensão de `Page` para reservar
espaço.

**`Content::Styled`** (perfil ADR-0026) é **inadequado** para
features Layout — todas exigem semantic estrutural (composição,
overflow, dimensões) que excede styling visual.

ADR-0017 (estratégia gradual typst-library) declarou progressão
incremental; este roadmap operacionaliza a continuação para Layout.

## Decisão

ADR-0061 propõe **3 fases** com prioridades explícitas e relação
clara com Model Fase 2 (ADR-0060 renumerada).

### Decisão 1 — Fase 1 Layout (M+ agregado; sem novas crates)

**Sub-fase mínima que desbloqueia footnote em Model Fase 2.**

5 features num único passo (modelo P154B agregado):

- **Passo 156C** — Fase 1 Layout (M+ agregado):
  1. **Page model com footnote area** (M, alto valor — crítico):
     extensão `Page::footnote_area: Vec<FrameItem>` (ou
     `Option<Frame>`); Layouter reserva espaço.
  2. **`pad`** (S, alto valor — trivial): `Content::Pad { left,
     top, right, bottom, body }` + stdlib `pad()` + consumer
     no Layouter.
  3. **`hide`** (S, médio valor — trivial): `Content::Hide { body }`
     + stdlib `hide()` + consumer (skip emit, manter cursor advance).
  4. **`pagebreak` manual** (S, alto valor): `Content::Pagebreak
     { weak: bool }` + stdlib `pagebreak()` + consumer
     (`Layouter::new_page()`).
  5. **`h()` / `v()` spacing** (S, alto valor): `Content::HSpace
     { amount, weak }` + `Content::VSpace { amount, weak, attach }`
     + stdlib `h()`/`v()` + consumer (avance cursor sem emit).

Cobertura post-Fase 1: **22% → ~50%** (4/18 → 9/18 implementadas).

**Footnote desbloqueado** após esta sub-fase (Model Fase 2 pode
abrir passo de footnote sem aguardar mais Layout).

### Decisão 2 — Fase 2 Layout (M+ agregado)

**Containers e composição.**

3 features:

- **Passo dedicado Fase 2 Layout** (M+; numeração a decidir
  pós-P156C):
  6. **`block`** (M+, alto valor): `Content::Block { width,
     height, breakable, inset, fill, stroke, body }`. Variant novo.
  7. **`box`** (S-M, médio valor): `Content::Box { width, height,
     baseline, body }` (inline). Variant novo.
  8. **`stack`** (S-M, médio valor): `Content::Stack { dir,
     spacing, children }`. Variant novo.

Cobertura post-Fase 2: **~50% → ~67%** (9/18 → 12/18).

### Decisão 3 — Fase 3 Layout (condicional / com DEBT)

**Trabalho L+ ou de baixo valor; condicional a priorização humana.**

5 features:

- **`columns`** + **`colbreak`** (L+, alto valor — complexo):
  `Content::Columns { count, gutter, body }` + `Content::Colbreak`.
  Exige refactor multi-region do Layouter. **DEBT-56 aberto** por
  este passo. Requererá ADR dedicada quando materializado (column
  flow algorithm).
- **`repeat`** (M, baixo valor): `Content::Repeat { body, gap,
  justify }` com lazy semantic. Variant novo + consumer dedicado.
- **`skew`** (S, baixo valor): `Content::Skew { ax, ay, body }`
  ou via `Content::Transform` existente.
- **Refino Page rico** (M+, médio valor): PageConfig com
  `Sides<Length>` margens; header/footer/background/foreground.
  Aproxima paridade vanilla.

Cobertura post-Fase 3: **~67% → 94-100%** (12/18 → 17-18/18).

`measure(body)` e `layout(callback)`: **condicionais ao desbloqueio
de ADR-0017** (Introspection runtime ainda adiada). Não incluídos
em qualquer fase desta ADR.

### Decisão 4 — `Content::Styled` vs variant novo

Para cada feature Fase 1/2/3: **variant novo** no `Content` enum
(modelo ADR-0060 Decisão 4).

Razão: features Layout têm semantic estrutural (composição,
overflow, dimensões) que excede styling visual. `Content::Styled`
(ADR-0026 perfil) cobre apenas estilos. Excepções consideradas
por feature (e.g. `pagebreak` poderia ser `Style::PageBreak`),
mas decisão default é variant novo para uniformidade.

Consequência: `Content` enum cresce de **43 variants pós-P155**
para **~52 variants pós-Fase 2** (+5 Fase 1 + 3 Fase 2 + variants
condicionais Fase 3). ADR-0026-R1 (`Arc<[T]>` em `Sequence`)
cobre performance de clone.

### Decisão 5 — Footnote area como sub-fase prioritária explícita

Page model com footnote area é **a sub-fase mínima** do roadmap.
Materializável independentemente do resto. Permite que **Model
Fase 2 footnote** seja atacado imediatamente após Fase 1 Layout
(sem aguardar block/box/columns).

Forma proposta: `Page::footnote_area: Vec<FrameItem>` (extensão
minimalista; reserva espaço; populated por consumer Model
posterior). Alternativas (PageWithFootnotes separado) rejeitadas
por backward-compatibility com Frame existente.

### Decisão 6 — Relação com ADR-0060 Model Fase 2

ADR-0060 anotada com **renumeração** (P156A foi historiograma;
P156B é este diagnóstico):

| Antes (ADR-0060 original) | Depois (pós-P156B) |
|---------------------------|---------------------|
| P156 = Model table foundations | **P157** |
| P157 = Model figure-kinds | **P158** |
| P158 = Model bibliography (XL) | **P159** |
| ADR-0061 = autorização hayagriva | **ADR-0062** |

DEBT-55 (Bibliography + Cite) actualizada para reflectir a
nova reserva. Modelo de roadmap independente: Layout (esta ADR)
e Model (ADR-0060) progridem em paralelo conforme decisão
humana, sem dependência cruzada (excepto footnote, que aguarda
P156C Fase 1 Layout).

### Decisão 7 — Sem novas crates externas

Confirmado empiricamente em §5 do diagnóstico: o módulo Layout
vanilla não usa crates externas específicas de layout. Toda a
Fase 1, 2 e 3 (incluindo column flow) é trabalho L1 puro com
crates já autorizadas (`comemo`, `smallvec`, `ecow`,
`unicode-bidi`, `typst_utils`).

**Nenhuma ADR de autorização de crate é necessária** para
Layout Fase X.

## Alternativas consideradas

| Alternativa | Prós | Contras |
|-------------|------|---------|
| **3 fases ranqueadas** ✓ | Roadmap explícito; cobertura predictível; trabalho mecânico decomposto | Trabalho longo (3-5+ passos) |
| Atacar tudo num passo XL | Único output | Risco alto; mistura concerns; revisão difícil; viola ADR-0036 atomização |
| Adiar Layout completo até Model Fase 2 fechar | Foco em features Model | Footnote bloqueado indefinidamente; cobertura Layout estagna em 22% |
| Apenas Fase 1 (sem 2/3) | Mínimo risco; desbloqueia footnote | Não responde a "trabalho real necessário"; deixa block/box/stack/columns como dívida implícita |
| Atacar columns primeiro (refactor multi-region) | Cobertura imediata para colunas | Custo L+; alto risco; sem ADR validação prévia; bloquearia outras features |
| ADR única para Layout + Bibliography | Menos ADRs | Mistura roadmap; bibliography exige autorização crate (ADR-0062 dedicada) |

**Escolha**: 3 fases com Fase 1 mínima (P156C) que desbloqueia
footnote sem comprometer-se a Fase 2 e 3. Fase 3 condicional
com DEBT-56 explícito para columns.

## Consequências

### Positivas

- **Roadmap explícito** para sair de cobertura Layout 22% para
  ~67% (post-Fase 2) ou 94-100% (post-Fase 3 completa).
- **Footnote desbloqueado em Model Fase 2** após Fase 1 Layout
  (P156C); permite ataque paralelo aos dois roadmaps.
- **Cada sub-passo tem escopo S/M/M+ definido** (excepto columns
  Fase 3 = L+ com DEBT-56 + ADR dedicada futura).
- **Sem dependência de crates novas** — risco de fetch reduzido a
  zero.
- **Compatível com ADR-0060** Model Fase 2 (renumeração documentada).
- **Coexiste com ADR-0017** adiada (Introspection): `measure`/
  `layout(callback)` ficam fora de fases obrigatórias.

### Negativas

- **3-5 sub-passos no caminho crítico** — investimento significativo
  de tempo se Fase 2+3 priorizadas.
- **Renumeração da ADR-0060** introduz risco de confusão com
  numeração antiga em documentos históricos. Mitigação: anotação
  explícita em ADR-0060 + DEBT-55 + README ADRs.
- **DEBT-56 column flow** fica em aberto por tempo indeterminado
  se Fase 3 não for priorizada.
- **`Content` enum cresce** para ~52 variants pós-Fase 2 (já
  esperado por ADR-0026-R1).
- **Reocupação de ADR-0061** (era hayagriva) requer actualização
  cruzada de blueprint, README ADRs, DEBT-55, ADR-0060.

### Neutras

- Inventário 148 ganha referências cruzadas para ADR-0061
  (per padrão P154A).
- ADR-0062 reservada para `hayagriva` (sem trabalho imediato
  até P159 ser atacado).
- Potencial passo dedicado para refino Page rico
  (Sides<Length>, header/footer) registado como Fase 3 mas
  pode antecipar-se se prioritário.

## Plano de materialização

5+ passos no caminho crítico (Layout Fase 1 + Fase 2 + Fase 3
+ refino):

| Passo | Escopo | Features | ADR adicional? |
|-------|--------|----------|----------------|
| **156C** | M+ | Fase 1 Layout (page model footnote area + pad + hide + pagebreak + h + v) | — (aplica esta ADR) |
| (Layout F2) | M+ | block + box + stack | — |
| (Layout F3 columns) | L+ | columns + colbreak; fecha DEBT-56 | **ADR dedicada** column flow algorithm |
| (Layout F3 visuais) | S+M | repeat + skew | — |
| (Layout refino Page) | M+ | Sides<Length> margens; header/footer/background/foreground | — |

Numeração final dos passos pós-156C decidida humanamente,
podendo intercalar com Model Fase 2 (P157 table; P158 figure-kinds;
P159 bibliography).

**Sub-passo crítico declarado**: P156C **desbloqueia
`footnote()`** em Model Fase 2; passo dedicado de footnote pode
seguir-se imediatamente.

ADR-0061 transitará `PROPOSTO → IMPLEMENTADO` quando Fase 1 for
materializada (por analogia com ADR-0060 que transitou no fim
da Fase 1 Model em P155).

## Referências

- **ADR-0017** — estratégia typst-library gradual.
- **ADR-0026** + **ADR-0026-R1** — `Content` enum fechado com
  `Arc<[T]>` para sequences.
- **ADR-0033** — paridade funcional para cada feature Layout
  materializada.
- **ADR-0034** — diagnóstico obrigatório (cumprido por P156B).
- **ADR-0036** — atomização progressiva — cada feature consumer
  explícito.
- **ADR-0037** — coesão por domínio — Layout permanece em
  `01_core/src/rules/layout/`; modulação se necessário.
- **ADR-0038** — `Content::Styled` para styling estrutural
  (rejeitado para Layout features).
- **ADR-0054** — perfil observacional graded — Fase 1 cumpre
  com aproximações aceites.
- **ADR-0060** (anotada por P156B) — Model Fase 2 roadmap;
  renumeração documentada.
- **ADR-0062** (reservada) — autorização `hayagriva` (era
  ADR-0061 antes desta reocupação).
- **DEBT-55** (P154A; actualizada por P156B) — Bibliography
  + Cite XL com plano ADR-0062.
- **DEBT-56** (P156B) — Column flow Fase 3 Layout L+; aberto
  por este passo.
- **DEBT-34d** / **DEBT-34e** (P80) — grid cell layouting; trabalho
  similar mas distinto de Layout Fase X.
- **Inventário 148** (`typst-cobertura-vanilla-vs-cristalino.md`)
  — Tabela A linha "Layout"; reclassificação P156B.
- **Diagnóstico P156B** (`diagnostico-layout-passo-156b.md`) —
  Tabelas §1, §2, §3, §6, §7 com plano detalhado.
- **Historiograma P156A** (`historiograma-passos.md`) — §4.1
  evidência 6/6 do padrão diagnóstico-primeiro que motivou
  esta aplicação a Layout.

---

## Aplicações cumulativas (pós-ADR-0062-create)

ADR-0061 PROPOSTO em P156B (2026-04-25). **Fase 1+2
materializadas em sequência granular P156C-I** (7 passos
consecutivos, 2026-04-25 a 2026-04-26). **Fase 3 iniciada em
P156J** (caminho 1 dos 3 documentados — activado em
2026-04-26). **Refino Fase 3 em P156L** (sub-passo 2; primeiro
refactor real após série aditiva, primeira aplicação concreta
de ADR-0065 critério #3):

| Passo | Feature(s) | Slope | Cobertura cumulativa | Tests Δ |
|-------|-----------|------:|---------------------:|--------:|
| P156C | pad + hide | +11% | 22% → 33% | +27 |
| P156D | h + v | +11% | 33% → 44% | +20 |
| P156E | pagebreak | +6%  | 44% → 50% | +22 |
| P156F | skew | +6%  | 50% → 56% | +16 |
| P156G | block | +5%  | 56% → 61% | +20 |
| P156H | box | +6%  | 61% → 67% | +21 |
| P156I | stack | +5%  | 67% → 72% (target Fase 1+2) | +25 |
| P156J | repeat | +6%  | 72% → 78% (Fase 3 sub-passo 1) | +19 |
| P156K | (meta — ADRs 0064+0065) | — | — (sem código) | 0 |
| P156L | pad refino sides | 0% | 78% (refino qualitativo) | +4 |
| P157 | (diagnóstico Model Fase 2 — table foundations) | — | — (sem código; passo documental) | 0 |
| P157A | table minimal (Model Fase 2 sub-passo 1) | +5% Model | Layout 78% inalterado; Model 45% → 50% | +16 |
| P157B | table cell (Model Fase 2 sub-passo 2) | 0% agregado | Layout 78% inalterado; Model 50% inalterado (sub-entrada) | +18 |
| P157C | table header + footer (Model Fase 2 sub-passo 3 — fecha table foundations) | 0% agregado | Layout 78% inalterado; Model 50% inalterado (par sub-entradas); arquitectural 78% → 80% | +26 |
| P158 | (diagnóstico Model figure-kinds) | — | — (sem código; passo documental) | 0 |
| P158A | figure auto-detect (Model figure-kinds sub-passo 1) | 0% agregado | Layout 78%; Model 50% inalterado (refino qualitativo) | +6 |
| P159 | (diagnóstico Bibliography + Cite) | — | — (sem código; passo documental) | 0 |
| P159A | Bibliography + Cite par acoplado minimal (Model Fase 2) | +arq 80% → 82% | Layout 78%; Model agregada 50% inalterada; cite/bib `ausente → parcial` | +27 |
| P159B | (diagnóstico amplo expansão série 159 + tecto Model) | — | — (sem código; passo documental amplo M-) | 0 |
| **ADR-0062-create** | **(administrativo XS — formaliza reserva ADR-0062 PROPOSTO)** | — | — (sem código; ADRs total 63 → 64) | **0** |
| P158B | figure supplement por lang (Model figure-kinds sub-passo 2) | 0% agregado | Layout 78%; Model 50% inalterado (refino qualitativo) | +15 |
| P159C | cite.form variants (Model bibliography+cite sub-passo 2) | 0% agregado | Layout 78%; Model 50% inalterado (refino estrutural); ADR-0064 Caso A N=5→6 | +15 |
| P159D | BibEntry fields adicionais (Model bibliography+cite sub-passo 3) | 0% agregado | Layout 78%; Model 50% inalterado (refino tipo entity); ADR-0065 #2 N=2→3 | +8 |

**Total**: +56 pontos percentuais Layout em 9 passos consecutivos
de materialização Layout (22% → 78%); **+5pp Model** em P157A
(primeiro sub-passo Model Fase 2; cross-domínio). P156K, P157,
**P158** não contam para o slope (meta/diagnóstico). Target
Fase 1+2 Layout atingido em P156I; P156J ultrapassa target ao
iniciar Fase 3 Layout; P157A inicia série Model Fase 2 análoga
a Layout P156C-J/L; P157B continua série Model com sub-entrada
qualitativa (TableCell); P157C fecha "table foundations"
declarado em ADR-0060 com par simétrico TableHeader/TableFooter
(ADR-0064 Caso D primeira aplicação Model — atinge saturação
cross-domínio cross-caso A/B/C/D). P156L é refino qualitativo
de variant existente — primeira aplicação concreta de ADR-0065
critério #3. P157 é diagnóstico precedendo materialização Model
Fase 2 (table foundations) — primeira aplicação concreta de
ADR-0065 critério #5. **P158 é diagnóstico precedendo
materialização Model figure-kinds — segunda aplicação concreta
do critério #5**. **P158B é segundo refino qualitativo
consecutivo de `figure` (supplement por lang) — sub-passo 2 de
Model figure-kinds; primeiro reuso explícito cross-feature do
pattern P155 `localize_quotes`**. **P159C é segundo sub-passo
substantivo de Bibliography + Cite — adiciona enum
`CitationForm` + field `form` em `Content::Cite`; ADR-0064
Caso A patamar N=5 → 6 atinge equilíbrio cross-domínio 50/50
Layout/Model**. **P159D é terceiro sub-passo substantivo de
Bibliography + Cite — refino de tipo entity `BibEntry` com
4 fields opcionais universais (volume/pages/journal/publisher)
+ builder pattern; ADR-0065 critério #2 patamar N=2→3 (terceira
aplicação isolada concreta — selecção de fields universais);
subpadrão emergente N=1 "refino de tipo entity sem alteração
ao variant Content" (precedente novo)**. **+305 tests**
acumulados (1145 → 1450 lib+integ+diagnostic — +8 em P159D).
**Zero reformulações mid-passo** em N=17 aplicações de
materialização (9 Layout + 8 Model). Padrão granular universal
cross-domínio confirmado e estendido. Cobertura arquitectural
mantém **82%** após P159D (refino tipo entity ortogonal ao
enum Content).

### Tipos novos infraestruturais

- `Sides<T>` (P156C) — para padding/inset/margem.
- `Parity` (P156E) — para pagebreak `to:`.
- `TransformMatrix::skew` (P156F) — método novo em tipo
  existente.
- `Dir` (P156I) — para stack direcção.

### Variants `Content` adicionados ou refinados

- `Pad`, `Hide` (P156C). **`Pad` refinado P156L**:
  `padding: Sides<Length>` → `sides: Sides<Option<Length>>`
  per ADR-0064 Caso C (None ↔ default vanilla zero).
- `HSpace`, `VSpace` (P156D).
- `Pagebreak` (P156E).
- (P156F: zero — método em TransformMatrix existente).
- `Block` (P156G).
- `Boxed` (P156H — naming evita conflito std::Box).
- `Stack` (P156I).
- `Repeat` (P156J — primeira Fase 3).
- (P156L: zero variants novos — refactor de variant existente).

**Total: 9 variants novos + 1 método novo em tipo existente
+ 1 refino de variant (P156L).** Variant count Content: 43 →
52 (+9; inalterado em P156L).

### Stdlib funcs adicionadas ou refinadas

- `pad`, `hide`, `h`, `v`, `pagebreak`, `skew`, `block`, `box`,
  `stack`, `repeat` = 10 funcs novas (32 → 42).
- **P156L**: `pad` ganha helper privado `extract_sides_lengths`;
  contagem stdlib funcs **inalterada (42)**.

### Padrões metodológicos consolidados

1. **Granularidade 1-2 features/passo**: **N=17** aplicações
   consecutivas sem reformulação (8 materialização Layout
   + 1 refino Layout P156L + 3 materialização Model
   P157A/B/C + 1 refino Model P158A + 1 par acoplado Model
   P159A + 1 segundo refino Model P158B + 1 refino
   estrutural Model P159C + **1 refino tipo entity Model
   P159D**). **Padrão cross-domínio reforçado**
   mas **com primeira quebra honestamente registada**: P159A é
   M+ par acoplado (granularidade quebrada N=13 → M+ com
   precedente P156C par lógico pad+hide). Hipótese da decisão
   humana 2026-04-25 empiricamente confirmada e estendida a
   refino Layout + Model Fase 2 multi-passo + refino Model +
   par acoplado + segundo refino consecutivo de mesmo Model
   feature (figure P158A→P158B) + refino estrutural de variant
   existente com tipo entity novo (P159C cite.form). **Formalizada
   parcialmente em ADR-0065** (que cita N=5 com diversidade de
   critérios; P156L é primeira aplicação concreta do critério #3;
   P157 é primeira aplicação do critério #5).

2. **"Inventariar primeiro" pré-decisão arquitectural**:
   **N=19** aplicações (P156F defensivo; P156G deliberado;
   P156H curto; P156I curto focado; P156J curto focado;
   P156L expansão variant existente — primeira aplicação
   concreta do critério #3 de ADR-0065; P157 scope determinado
   por inventário — primeira aplicação concreta do critério #5
   de ADR-0065; P157A inventário completo Model Fase 2 com
   decisão de módulo `stdlib/structural.rs` continuação vs
   `stdlib/model.rs` novo — implícito critério #1 naming + #5
   scope; P157B inventário completo TableCell com decisão de
   naming `table_cell` flat vs vanilla `table.cell` por
   limitação cristalina FieldAccess — explícito critério #1
   naming + #6 divergência da spec da feature vanilla; P157C
   inventário completo par simétrico TableHeader/TableFooter
   com decisão de divergência `body: Box<Content>` vs vanilla
   `Vec<TableItem>` per ADR-0033 — explícito critério #6
   reforçado; P158 inventário Model figure-kinds com decisão
   de scope subset minimal (auto-detecção) vs subset máximo
   (auto-detecção + supplement) — segunda aplicação concreta do
   critério #5 após P157; **P158A inventário figure auto-detect
   com decisão Sequence handling — recursão limitada a Sequence
   per ADR-0033 (paridade vanilla parcial); critério #5 scope
   reforçado; **P159 inventário Bibliography + Cite com avaliação
   de 3 estruturas (multi-passo / minimal / diferimento) e
   recomendação Estrutura A adaptada — terceira aplicação concreta
   do critério #5 com diversidade cross-feature confirmada;
   P159A inventário par acoplado com decisão de tipo entity
   `BibEntry` 4 fields minimais — primeira aplicação isolada
   concreta de ADR-0065 critério #2 (escolha de tipo);
   P159B inventário amplo expansão série 159 + tecto Model
   — quarta aplicação concreta critério #5 com diversidade
   ampliada multi-feature; **ADR-0062-create inventário
   precedentes ADR autorização crate (0023/0024/0057) +
   convenção naming + estrutura canónica — primeira aplicação
   isolada concreta de ADR-0065 critério #1 (naming) num
   passo administrativo XS**; **P158B inventário figure
   supplement com decisão fallback PT vs EN — quinta aplicação
   concreta critério #5 com diversidade reforçada (lang-aware
   refino) + reuso pattern P155 cross-feature como precedente
   metodológico**; **P159C inventário cite.form com decisão
   lookup via CounterState (Opção C) + reuso pattern P158B
   `state.lang` para infraestrutura `state.bib_entries` —
   sexta aplicação concreta critério #5 + segunda aplicação
   isolada concreta critério #2 (escolha de tipo enum
   `CitationForm` vs `Option<String>`)**; **P159D inventário
   BibEntry fields com decisão constructor pattern (Opção C
   builder pattern fluente vs new_full vs field assignment) +
   selecção de 4 fields universais (volume/pages/journal/
   publisher) vs alternativas (url/doi/editor) — sétima
   aplicação concreta critério #5 + terceira aplicação isolada
   concreta critério #2 (selecção de fields universais)**).
   **Formalizado em ADR-0065** (P156K); **agora N=19 com 4
   critérios formalmente validados** (#1 P157A/B + ADR-0062-create
   administrativo XS; #2 P159A + P159C + **P159D** patamar N=3;
   #3 P156L; #5 P157 + P157A + P158 + P158A + P158B + P159 +
   P159B + P159C + **P159D** multi-feature; #6 P157B/C).

3. **"Smart<T> → Option<T> ou default"**: **N=11** aplicações
   (P156E Parity; P156F angles; P156G Block.width; P156H
   Box.width; P156I Stack.spacing + Dir.default; P156J
   Repeat.gap; P156L Pad sides — segunda aplicação concreta
   do Caso C; P157B TableCell.x/y/colspan/rowspan — primeira
   aplicação Caso A em Model + terceira global Caso C com
   primeira variação `usize`; **P157C TableHeader/TableFooter.repeat
   — primeira aplicação Caso D em Model; **P159A
   Bibliography.title + Cite.supplement — Caso A patamar
   cresce N=4 → 5; reforça diversidade cross-domínio**;
   **P159C Cite.form — Caso A patamar cresce N=5 → 6;
   atinge equilíbrio cross-domínio 50/50 Layout/Model**).
   **Formalizado em ADR-0064** (P156K) com 4 casos canónicos
   A/B/C/D.

   **Patamares por caso pós-P159C**:
   - Caso A: **N=6** (P156G/H/I + P157B + P159A + **P159C**);
     50% Layout (3) + 50% Model (3) — **equilíbrio
     cross-domínio atingido**.
   - Caso B: N=1 (P156I Dir); 100% Layout (Caso B só Layout
     — candidato futuro Model).
   - Caso C: N=3 (P156I/J + P157B); primeira variação `usize`
     em P157B; 66% Layout + 33% Model.
   - Caso D: N=4 (P156D/G/J + P157C); 75% Layout + 25% Model.
   - **Todos os 4 casos canónicos validados em Layout** ✓.
   - **3/4 casos canónicos validados em Model** (A, C, D);
     Caso B só Layout — candidato futuro.
   - **Caso A é o caso mais aplicado** (N=6; **equilíbrio
     cross-domínio**).

4. **"§análise de risco no relatório"**: **N=19** aplicações
   (P156F/G/H/I/J/K + L com peso real — primeiro refactor
   real após série aditiva; P157 com risco baixo diagnóstico;
   P157A com risco baixo-médio — primeiro Model Fase 2 com
   decisão arquitectural de módulo stdlib; P157B com risco
   baixo-médio — primeira aplicação ADR-0064 Caso A em Model
   + decisão de naming `table_cell` flat; P157C com risco
   baixo — par simétrico aditivo; primeira aplicação ADR-0064
   Caso D em Model + saturação cross-domínio cross-caso
   atingida; **P158 com risco baixo — diagnóstico Model
   figure-kinds; segunda aplicação concreta de ADR-0065
   critério #5; estabelece precedente "sem novas reservas";
   **P158A com risco muito baixo — refino comportamental sem
   alteração estrutural; primeiro passo Model com refino
   qualitativo; **P159 com risco baixo — diagnóstico
   Bibliography+Cite XL declarado; terceira aplicação concreta
   ADR-0065 critério #5 com avaliação de 3 estruturas
   (multi-passo/minimal/diferimento); **P159A com risco médio
   — primeiro M+ par acoplado pós-P156C; tipo entity novo
   `BibEntry` em ficheiro novo; ADR-0065 critério #2 primeira
   aplicação isolada concreta; **P159B com risco baixo —
   diagnóstico amplo M-; tecto Model puro estimado +5-10pp
   alcançável com 5 sub-passos Bloco A; recomendação concreta
   P158B supplement por lang figure; **ADR-0062-create com
   risco muito baixo — passo administrativo XS; primeiro do
   tipo "criar ADR a partir de reserva pré-existente"; sem
   código alterado**; **P158B com risco muito baixo — segundo
   refino comportamental consecutivo de figure (P158A→P158B);
   reuso explícito do padrão P155 `localize_quotes` cross-feature;
   helper novo `figure_supplement_for_lang` em
   `rules/lang/figure_supplement.rs` paralelo a `quotes.rs`;
   field novo `lang: Option<Lang>` em `CounterState` para lang
   resolution; fallback PT preserva backwards compat**;
   **P159C com risco baixo-médio — refino estrutural de variant
   existente (Cite); enum entity novo CitationForm em ficheiro
   próprio (5ª aplicação consecutiva do padrão); ADR-0064 Caso A
   patamar N=5→6 atingindo equilíbrio cross-domínio 50/50;
   field novo `bib_entries: Vec<BibEntry>` em CounterState
   (paridade infraestrutural P158B `state.lang`); 13 sítios
   pattern-match Content actualizados; **decisão Opção C**
   lookup via state em vez de Layouter field ou second-pass;
   hash content.rs preservado (12º consecutivo via L0-baseline)**;
   **P159D com risco baixo — refino de tipo entity sem alteração
   ao variant Content (precedente novo); 4 fields opcionais
   `Option<String>` directos (sem ADR-0064 aplicável); builder
   pattern Opção C escolhido por legibilidade; helper
   `format_bib_entry` privado em layout para concatenação
   condicional; backwards compat trivial via fields default
   None; hashes content.rs e bib_entry.rs ambos preservados
   via L0-baseline (13º consecutivo content.rs)**).
   Cobertura sistemática do risco.

5. **"Reuso de template containers"**: **N=4** aplicações
   (Block → Boxed → Stack → Repeat). Padrão "variant rico
   para containers cujos atributos não são propriedades de
   texto" estabelecido em P156G; **P156L não acrescenta** (é
   refactor não criação).

6. **"Antecipar especificidades técnicas"**: N=2-3
   aplicações (Boxed naming P156H; Vec/Arc<[T]> arms P156I).

7. **"Helper `extract_length` reuso"** (subpadrão dentro de §3
   ADR-0064 §Implicações): **N=7** aplicações consecutivas
   (P156C/D/G/H/I/J/L). Emergiu como vocabulário canónico
   para coerção Length em named args — promoção a helper público
   `pub fn extract_length(...)` é candidato a refactor escopo XS.

8. **"Reuso de infraestrutura `Sides<T>`"** (novo subpadrão
   P156L): **N=2** aplicações concretas (P156C origin —
   `Sides<Length>`; P156L refino — `Sides<Option<Length>>`).
   Tipo genérico paga investimento de design ao segundo uso.

9. **"Helper `extract_tracks` reuso"** (subpadrão P157A):
   **N=2** aplicações concretas (P82-83 origin em `native_grid`;
   P157A reuso em `native_table` com promoção a `pub(super)`
   para acesso cross-módulo `stdlib/layout.rs` → `stdlib/structural.rs`).
   Análogo ao subpadrão `extract_length` em fase inicial; promoção
   formal diferida até atingir N=3-4 (mesma política).

10. **"Helper `extract_usize_or_none_min` privado"** (subpadrão
    P157B): **N=4** usos no mesmo passo (`x`, `y` com min=0;
    `colspan`, `rowspan` com min=1) num único helper
    parametrizado. Padrão de combinação via param em vez de
    helpers separados — evita duplicação e reduz superfície
    pública. Promoção a `pub(super)` ou helper público diferida
    até reuso noutro passo (e.g. P157C `Header.level`); política
    consistente N=2-3 mínima.

11. **"Helper `extract_bool_with_default` privado"** (novo
    subpadrão P157C): **N=2** usos no mesmo passo (`repeat` em
    TableHeader e TableFooter, ambos com default=true). Padrão
    parametrizado análogo a `extract_usize_or_none_min` (P157B)
    e `extract_length` (N=7 reuso). Distinção vs `extract_weak`
    em `stdlib/layout.rs` (específico para key="weak"
    default=false): novo helper é genérico no key e no default,
    preservando separação de domínios per ADR-0037. Promoção
    a `pub(super)` diferida até N=3-4 reuso noutros passos
    (e.g. P158 figure-kinds).

12. **"Par simétrico em pattern-match"** (subpadrão emergente
    P157C): **N=2** aplicações concretas (P156D HSpace+VSpace +
    **P157C TableHeader+TableFooter**). Padrão "tratamento
    simétrico em todos os arms com entradas adjacentes" torna
    paridade visualmente óbvia em pattern-match. Candidato a
    formalização se P158/P159 também usarem pares simétricos
    (e.g. `figure.caption`/`figure.numbering` se aplicável).

13. **"Padrão P155 i18n reusado cross-feature"** (subpadrão
    emergente P158B): **N=1** aplicação concreta (P155
    `localize_quotes(lang)` em `rules/lang/quotes.rs` →
    P158B `figure_supplement_for_lang(kind, lang)` em
    `rules/lang/figure_supplement.rs`). Estrutura paralela:
    tabela estática `&[((key,...), value)]` + lookup linear
    por exact match + fallback constante. Primeiro reuso
    explícito cross-feature do pattern P155 (quotes → figure
    supplement). Candidato a formalização (helper genérico ou
    macro de tabela i18n) se outro feature reusar — e.g. table
    caption supplement futuro; bibliography lang per P159B §4.
    Política consistente N=3-4 mínima para promoção.

14. **"Tipo entity em ficheiro próprio"** (subpadrão emergente
    P159C — formalmente N=5): aplicações concretas Sides P156C →
    Parity P156E → Dir P156I → BibEntry P159A → **CitationForm
    P159C**. Padrão "enum/struct dedicado em ficheiro próprio
    `entities/<nome>.rs` em vez de inline em content.rs ou
    bib_entry.rs". 5 aplicações consecutivas sem reformulação.
    Promoção a subpadrão consolidado per política N=4-5; pode ser
    formalizado em ADR futura.

15. **"Infraestrutura state lookup"** (subpadrão emergente
    P159C N=2): aplicações concretas P158B `state.lang:
    Option<Lang>` (lang resolution) + **P159C `state.bib_entries:
    Vec<BibEntry>` (cross-reference lookup)**. Padrão "field
    novo opcional em `CounterState` populado por introspect
    walk; consumido por layouter via state borrow". Reuso de
    infraestrutura existente em vez de modificação de signature
    ou novo Layouter field. Candidato a formalização N=3-4.

16. **"Refino de tipo entity sem alteração ao variant Content"**
    (subpadrão emergente P159D N=1): aplicação concreta P159D
    expansão de `BibEntry` struct (4 fields opcionais novos +
    builder pattern) sem afectar `Content::Bibliography` nem
    qualquer outro variant Content. Distinção vs P156L (refino
    de variant Content `Pad`) e P159C (refino de variant Content
    `Cite`). Subpadrão captura "tipo entity em ficheiro próprio
    (padrão #14) é refinável independentemente do enum Content
    — preserva hash content.rs sem necessidade de actualizar L0
    do enum". Candidato a formalização N=3-4 mínima.

### Estado pós-P159D

- **Cobertura Layout**: **78%** (inalterada por P157A/B/C +
  P158/P158A/P159/P159A/P159B + ADR-0062-create + P158B + P159C +
  **P159D** — escopo Model + refino qualitativo + par acoplado +
  diagnóstico amplo + administrativo XS + segundo refino
  qualitativo figure + refino estrutural cite + refino tipo
  entity bib_entry). Target ADR-0061 (72%) **continua
  ultrapassado**.
- **Cobertura arquitectural**: **82%** (inalterada por P159B +
  P158B + P159C + **P159D** — refinos qualitativos/estruturais
  de variants Content existentes ou tipos entity ortogonais;
  sem variants novos).
- **Tecto Model puro estimado** (P159B §4): cobertura agregada
  ~50% → **~55-60% alcançável** com 5 sub-passos Bloco A
  (supplement figure / cite.form / BibEntry fields / kind
  refactor / bib numbering simples). Tecto Model + hayagriva:
  **~68%** (paridade ADR-0060 declarado).
- **Cobertura Model agregada**: ~50% (inalterada vs P157A/B —
  TableCell/TableHeader/TableFooter são sub-entradas de table
  que não contam separadamente na agregação). Ganho qualitativo
  via **expansão estrutural completa de "table foundations"**.
- **Cobertura arquitectural total**: **80%** (era 78% pós-P157B;
  +2pp via fechamento de "table foundations" — variants Content
  vanilla extra ausentes desce de ~1 a 0). Patamar 80% atingido.
- **"Table foundations" declarado em ADR-0060 §"Decisão 1"
  sub-passo 3 fica integralmente fechado** com P157A + P157B +
  P157C (3 sub-passos M cada; granularidade N=10/11/12 sem
  reformulação). Marca conceptual importante.
- **P158 (diagnóstico figure-kinds)**: scope decidido em
  diagnóstico §3 — subset minimal auto-detecção recomendado.
- **P158A (figure auto-detect)**: materializado. Helper privado
  `infer_kind_from_body` em `stdlib/figure_image.rs` cobrindo
  Image/Table/Raw + recursão limitada a Sequence (paridade
  vanilla parcial per ADR-0033). Sem alteração a variant
  `Content::Figure` ou layout. Hash `entities/content.rs`
  preservado (sétimo passo consecutivo).
- **P159 (diagnóstico Bibliography + Cite)**: scope decidido em
  diagnóstico §3 — Estrutura A adaptada.
- **P159A (par acoplado Bibliography + Cite minimal)**:
  materializado. Tipo entity novo `BibEntry { key, author,
  title, year }` em `entities/bib_entry.rs`. Variants
  `Content::Bibliography { entries, title }` + `Content::Cite
  { key, supplement }`. Stdlib `native_bibliography` +
  `native_cite` em `structural.rs`. Layouter renderiza
  placeholder per ADR-0033 + ADR-0054 graded. **Sem hayagriva,
  sem CSL** — input cristalino literal. **Sem validação
  cross-reference** — ADR-0017 adiada. ADR-0062 mantém-se
  reserva sem ficheiro. **DEBT-55 contribuído mas NÃO fechado**
  — refinos futuros pendem de hayagriva integration.
- **P158B (figure supplement por lang)**: materializado.
  Helper novo `figure_supplement_for_lang(kind, lang) ->
  String` em `rules/lang/figure_supplement.rs` cobrindo
  6 langs × 3 kinds = 18 entradas + fallback PT por kind +
  capitalização para kind desconhecido. Field novo `pub
  lang: Option<Lang>` em `CounterState` para lang resolution
  (default `None` → fallback PT, paridade backwards compat).
  Modificação trivial em `introspect.rs` linha 334. Sem
  alteração ao variant `Content::Figure`. Hash
  `entities/content.rs` preservado **`ec58d849` — décimo
  primeiro passo consecutivo** (P156L → P158B). **Reuso
  explícito do padrão P155** `localize_quotes(lang)` —
  primeiro reuso cross-feature (subpadrão emergente N=1).
- **P159C (cite.form variants)**: materializado. Enum novo
  `CitationForm { Normal, Prose, Author, Year }` em
  `entities/citation_form.rs` (5ª aplicação consecutiva do
  padrão "tipo entity em ficheiro próprio"). Field
  `form: Option<CitationForm>` em `Content::Cite` per ADR-0064
  Caso A (patamar N=5→6 atinge equilíbrio cross-domínio
  50/50). Helper privado `extract_citation_form` em stdlib
  (strict matching case-sensitive). Layout placeholder
  melhorado por form com lookup Bibliography via novo field
  `pub bib_entries: Vec<BibEntry>` em `CounterState` (paridade
  infraestrutural P158B `state.lang`); populado por introspect
  walk. Hash `entities/content.rs` preservado `ec58d849`
  (12º consecutivo via L0-baseline interpretation — content.md
  prompt não modificado).
- **P159D (BibEntry fields adicionais)**: materializado.
  Struct entity `BibEntry` extendido com 4 fields opcionais
  universais (`volume`/`pages`/`journal`/`publisher`) per
  ADR-0065 critério #2 (terceira aplicação isolada concreta —
  selecção de fields universais). Builder pattern fluente
  `with_volume()`/etc. (Opção C). Helper `extract_bib_entries`
  extendido para parsing dos 4 fields opcionais. Helper privado
  novo `format_bib_entry` em `layout/mod.rs` para concatenação
  condicional APA-like. Backwards compat trivial — fields novos
  default `None` preservam output P159A. **Sem alteração ao
  variant `Content::Bibliography` ou `Content::Cite`**.
  Hashes `entities/content.rs` e `entities/bib_entry.rs` ambos
  preservados via L0-baseline interpretation (13º consecutivo
  content.rs).
- **Restantes Fase 2 Model** (per P159B §5 Bloco A — 2
  candidatos restantes pós-P159D):
  - Refactor `kind: String → Option<String>` per ADR-0064 Caso A
    (P158C — quebra hash content.rs **inevitável**).
  - Numbering numérico simples Bibliography (P159F — counter local).
- **Bloco B (hayagriva, NÃO reservados)** P159B §5: ADR-0062
  promovida → CSL parsing + styles APA/IEEE/etc.
- **Bloco C (cross-módulo, NÃO materializáveis em Model puro)**
  P159B §5: DEBT-34e placement Grid; DEBT-56 multi-region;
  ADR-0017 cite cross-document forward refs.
- **Restantes 3 entradas Layout** pendentes (mesmo subset
  pós-P156L; P157A/B/C não tocam):
  - `columns`/`colbreak` (Fase 3 condicional — DEBT-56).
  - `place` parcial — refino column scope.
  - `measure` parcial — depende ADR-0017 Introspection.
- **Total user-facing**: ~61.0% (inalterada; +0.7pp ganho
  P157A mantém-se; ganhos qualitativos cumulativos P157B/C).
- **DEBT-34e e DEBT-56 permanecem abertos**: P157B contribui
  ao DEBT-34e (storage de x/y/colspan/rowspan); P157C
  contribui ao DEBT-56 (storage de repeat). Fechamento de
  ambos fica para refactor dedicado.
- **Zero novos DEBTs** em toda a série P156C-L + P157 +
  P157A/B/C + P158 + P158A + P159 + P159A + P159B +
  **ADR-0062-create** (20 passos total: 14 materialização +
  5 meta/diagnóstico + 1 administrativo XS).
- **Footnote area** scope-out per decisão humana
  2026-04-25 (não incluído na Fase 1+2 Layout nem em P156J/L/
  P157/P157A/B/C/P158/P158A).
- **Hash `entities/content.rs` preservado**: `ec58d849`
  desde P156L — **décimo primeiro passo consecutivo** (P156L
  → P157 → P157A → P157B → P157C → P158 → P158A → P159 →
  P159A → P159B → **ADR-0062-create**) sem alteração ao
  prompt L0 do content. Padrão "estabilidade contrato L0 do
  content" continua a fortalecer-se em ADR-0062-create (passo
  administrativo XS). Refino futuro pode actualizar prompt
  L0 com documentação dos novos variants Bibliography/Cite
  (passo administrativo XS NÃO reservado per política P158).
- **Política nova "sem novas reservas"** (P158): reservas
  pré-existentes (P159 + ADR-0062) respeitadas mas não
  reforçadas; passos seguintes a decidir sequencialmente per
  evidência empírica em vez de pré-comprometimento.
- **Cadência cross-domínio fortalecida**: padrão granular
  Layout (P156C-L) replicado a Model (P157A/B/C; **3 sub-passos
  Model consecutivos** fecham conjunto coerente). Granularidade
  N=12 sem reformulação. **Saturação cross-domínio cross-caso
  ADR-0064**: todos os 4 casos canónicos (A/B/C/D) validados em
  Layout; 3/4 (A, C, D) validados em Model. Patamar empírico
  ADR meta atinge maturidade.

### Status

**`PROPOSTO`** mantido. Promoção a `IMPLEMENTADO` requer
**uma** das seguintes:
1. ~~Fase 3 materializada~~ → **parcialmente activado em
   P156J** (repeat ✓; columns/colbreak pendentes — DEBT-56).
2. Decisão humana de scope-out formal de columns/colbreak
   (com anotação que ADR-0061 fica "fase mínima + repeat
   cumprida; columns/colbreak adiadas por DEBT-56").
3. Inclusão de footnote area + actualização do scope da ADR.

Caminho 1 actualizado: 50% concluído (1/2 features Fase 3
materializadas em P156J). **Promoção a IMPLEMENTADO continua
diferida** — decisão humana sobre se columns/colbreak são
suficientes para fechamento, ou se DEBT-56 column flow L+
justifica scope-out formal.

Anotação cumulativa acima preserva o contexto histórico para
retomada futura.
