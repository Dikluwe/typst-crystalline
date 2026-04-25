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
