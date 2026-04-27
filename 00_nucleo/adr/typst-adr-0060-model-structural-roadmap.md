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
