# P205A — Auditoria empírica de F3

**Data**: 2026-05-07.
**Cláusulas**: A1–A14.
**Pré-condição confirmada**: M8 estruturalmente fechado em
P204H; tests 1852 verdes; 0 violations; ADR-0073 ACEITE.

---

## §1 Bloco 1 — Layouter cristalino (inventário 22 fields)

### A1 — Listagem completa

**Fonte**: `01_core/src/rules/layout/mod.rs:69`.

```text
pub struct Layouter<'a, M: FontMetrics, S: ImageSizer = NullImageSizer>
```

| # | Field | Tipo | Visibilidade |
|---|-------|------|--------------|
| 1 | `metrics` | `M` (generic FontMetrics) | `pub(super)` |
| 2 | `sizer` | `S` (generic ImageSizer) | private |
| 3 | `font_size_pt` | `Pt` | `pub(super)` |
| 4 | `style` | `TextStyle` | `pub(super)` |
| 5 | `chain` | `StyleChain` | `pub(super)` |
| 6 | `page_config` | `PageConfig` | `pub` |
| 7 | `pages` | `Vec<Page>` | `pub(super)` |
| 8 | `current_items` | `Vec<FrameItem>` | `pub(super)` |
| 9 | `cursor_x` | `Pt` | `pub(super)` |
| 10 | `cursor_y` | `Pt` | `pub(super)` |
| 11 | `line_start_x` | `Pt` | `pub(super)` |
| 12 | `current_line` | `Vec<FrameItem>` | `pub(super)` |
| 13 | `introspector` | `Tracked<'a, dyn Introspector + 'a>` | `pub(super)` |
| 14 | `figure_progress` | `HashMap<String, usize>` | private |
| 15 | `is_height_unconstrained` | `bool` | `pub(super)` |
| 16 | `cell_available_h` | `Option<f64>` | `pub(super)` |
| 17 | `cell_origin_x` | `Option<f64>` | `pub(super)` |
| 18 | `cell_origin_y` | `Option<f64>` | `pub(super)` |
| 19 | `cell_origin_w` | `Option<f64>` | `pub(super)` |
| 20 | `locator` | `Locator` | `pub(super)` |
| 21 | `current_location` | `Option<Location>` | `pub(super)` |
| 22 | `runtime` | `LayouterRuntimeState` | `pub` |

**Status A1**: ✅ **CONFIRMADO** — 22 fields totais. Field
#13 é o `Tracked<dyn Introspector>` introduzido por P204C
(M8). Restantes 21 são "ortogonais" para efeitos de F3.

### A1.1 — Sub-fields de `LayouterRuntimeState`

**Fonte**: `01_core/src/entities/layouter_runtime_state.rs`.

| # | Sub-field | Tipo | Semântica |
|---|-----------|------|-----------|
| 1 | `label_pages` | `HashMap<Label, usize>` | Write durante layout (`references.rs`); read no fim para `PagedDocument.extracted_label_pages`. |
| 2 | `known_page_numbers` | `HashMap<Label, usize>` | Read no início de cada iteração fixpoint; set entre iterações. |
| 3 | `is_readonly` | `bool` | Flag interno controlando branch em `layout_counter_update`. |
| 4 | `positions` | `HashMap<Location, Position>` | Write single-pass por locatable (`Layouter::advance_locator_if_locatable`); read futuramente por queries `position_of`. |

### A2 — Classificação dos 21 ortogonais

Categorias preliminares (per spec §3 A2):

- **A** — Runtime puro de layout (cursor, chain, items, cells).
- **B** — Runtime de introspecção (consumido por queries).
- **C** — Config (set na construção).
- **D** — Fronteira ambígua.

| # | Field | Categoria | Justificação |
|---|-------|-----------|--------------|
| 1 | `metrics` | **C** | Set na construção; imutável durante layout. |
| 2 | `sizer` | **C** | Set na construção; imutável durante layout. |
| 3 | `font_size_pt` | **A** | Mutado via push/pop de chain; cache local. |
| 4 | `style` | **A** | Cache flat de `chain`; resolved em cada push/pop. |
| 5 | `chain` | **A** | StyleChain mutável (push/pop por Content::Styled). |
| 6 | `page_config` | **D** | Inicialmente config; mutável via `Content::SetPage`. |
| 7 | `pages` | **A** | Output acumulado; runtime puro. |
| 8 | `current_items` | **A** | Página actual em construção. |
| 9 | `cursor_x` | **A** | Runtime puro. |
| 10 | `cursor_y` | **A** | Runtime puro. |
| 11 | `line_start_x` | **A** | Runtime puro (sub-layouts de cell). |
| 12 | `current_line` | **A** | Linha actual em construção. |
| 14 | `figure_progress` | **A** | Counter local por kind de figura. |
| 15 | `is_height_unconstrained` | **A** | Flag interno por sub-layout. |
| 16 | `cell_available_h` | **A** | Runtime cell. |
| 17 | `cell_origin_x` | **A** | Runtime cell. |
| 18 | `cell_origin_y` | **A** | Runtime cell. |
| 19 | `cell_origin_w` | **A** | Runtime cell. |
| 20 | `locator` | **D** | Gerador determinístico mutável; mas é primário de `current_location` que é semi-tracked-relevant. |
| 21 | `current_location` | **D** | Lido por consumers location-aware (sub-trait Introspector); ambíguo. |
| 22 | `runtime` | **B** (composto) | Contém 4 sub-fields, dos quais 3 são B (label_pages, known_page_numbers, positions) e 1 é A (is_readonly). |

**Sumário por categoria**:

- **Categoria A** (runtime puro layout): **15 fields** (3, 4, 5, 7, 8, 9, 10, 11, 12, 14, 15, 16, 17, 18, 19) + 1 sub-field (`runtime.is_readonly`). 16 totais.
- **Categoria B** (runtime introspecção): **3 sub-fields** de `runtime` (`label_pages`, `known_page_numbers`, `positions`).
- **Categoria C** (config): **2 fields** (1, 2).
- **Categoria D** (ambígua): **3 fields** (6, 20, 21).

**Status A2**: ✅ **CONFIRMADO**. Categoria B é restrita
ao `runtime` field (3 sub-fields).

### A3 — Mutabilidade actual

| Field | Set em | Mutado em | Lido em | Hot path? |
|-------|--------|-----------|---------|-----------|
| `metrics`, `sizer` | `Layouter::new` | nunca | em todo o layout (medição) | sim |
| `font_size_pt`, `style`, `chain` | `new` | push/pop styled/heading | hot path text | sim |
| `page_config` | `new` (default) | `Content::SetPage` | hot path each item | sim |
| `pages` | `new` (vazio) | finalize_page | finish | morno (1× por page) |
| `current_items` | `new` | append por item | finalize_page | sim |
| `cursor_x`, `cursor_y`, `line_start_x`, `current_line` | `new` | hot path | hot path | sim |
| `introspector` | `new` (Tracked) | nunca (Tracked) | queries | morno |
| `figure_progress` | `new` | `Content::Figure` arm | `Content::Figure` arm | morno |
| `is_height_unconstrained`, `cell_*` | `new` (default) | `layout_sub_frame_with_width`, `Content::Grid` | resolve_alignment, place | morno |
| `locator` | `new` | `advance_locator_if_locatable` | gating | morno |
| `current_location` | `new` (None) | `advance_locator_if_locatable` | consumers location-aware | morno |
| `runtime.label_pages` | `new` (vazio) | `references.rs` | `finish` | morno |
| `runtime.known_page_numbers` | `new` (vazio); inj. entre iters fixpoint | `outline.rs` resolver | `finish` | morno |
| `runtime.is_readonly` | `new` (false) | `outline.rs` em volta de `layout_content` | `layout_counter_update` | morno |
| `runtime.positions` | `new` (vazio) | `advance_locator_if_locatable` (P204D) | consumers `position_of` (futuro) | morno (1× por locatable) |

**Padrão write-once vs mutate-frequently**:

- **Mutate-frequently**: cursor, chain, items, page_config, style.
- **Mutate-moderate**: figure_progress, locator, current_location, runtime.* (1× por locatable / por reference).
- **Write-once**: metrics, sizer, introspector (Tracked).

**Status A3**: ✅ **CONFIRMADO**. Categoria B (runtime
sub-fields) tem padrão write-during-layout + read-after.

### A4 — Aliasing entre fields

**Pares identificados**:

1. `cursor_x, cursor_y` — agrupáveis em `Point { x, y }`.
2. `line_start_x` ↔ `cursor_x` — `line_start_x` é "cursor_x at line start".
3. `cell_origin_x, cell_origin_y, cell_origin_w` —
   agrupáveis em `Option<CellRect { x, y, w }>`.
4. `cell_available_h` ↔ `cell_origin_*` — co-existência
   condicional (todos `Some` em conjunto).
5. `style` ↔ `chain` — `style` é cache flat de `chain`.
   Sincronizados por push/pop.
6. `pages.len() + 1` ↔ "current_page" implícito — não
   há field explícito, mas derivável.
7. `runtime.label_pages` ↔ `runtime.known_page_numbers` —
   semantica de "este label → esta página"; difere por
   *quem* preenche (label_pages = layout actual;
   known_page_numbers = layout anterior do fixpoint).
8. `current_location` ↔ `locator` — `current_location` é
   "última `Location` produzida por `locator`".
9. `runtime.positions[loc]` ↔ `current_location` quando
   loc == current_location — correlação no momento da
   inserção.

**Sugestões de consolidação**:

- Grupo `cell_*` → `Option<CellRect>` (P210 candidate).
- Grupo `cursor` → `Point` (P211 candidate).
- `style` cache pode permanecer (sync invariant explícito
  via comentário ADR-0039).

F3 **não consolida** (esses são refactors ortogonais);
foco fica em sub-stores trackable (Categoria B).

**Status A4**: ✅ **CONFIRMADO** com 9 pares + 3 grupos
sugeridos.

---

## §2 Bloco 2 — Vanilla typst Layouter (referência)

### A5 — Forma do Layouter vanilla

**Achado central**: vanilla **não tem um Layouter
monolítico** análogo ao cristalino.

**Fonte**: `lab/typst-original/crates/typst-layout/src/`
estrutura.

Em vez disso, vanilla divide-se em:

| Struct | Localização | Função |
|--------|-------------|--------|
| `Engine<'a>` | `typst-library/src/engine.rs:19` | Cross-modular context (6 fields, todos Tracked). |
| `Composer<'a, 'b, 'x, 'y>` | `typst-layout/src/flow/compose.rs:63` | Compõe flow de blocks. |
| `Distributor<'a, 'b, 'x, 'y, 'z>` | `flow/distribute.rs:35` | Distribui blocks por page. |
| `Work<'a, 'b>` | `flow/mod.rs:294` | State trabalho actual. |
| `Config<'x>` | `flow/mod.rs:356` | Config do flow (FootnoteConfig, ColumnConfig, LineNumberConfig). |
| `Collector<'a, 'x, 'y>` | `flow/collect.rs:56` | Colecta children para flow. |
| `StackLayouter<'a>` | `stack.rs:65` | Layouter de stack. |
| `GridLayouter<'a>` | `grid/layouter.rs:27` | Layouter de grid. |

Cada Layouter especializado é construído localmente para
uma fase de layout, recebe `Engine` (ou parts dele) por
parameter, e tem fields locais ao seu contexto.

**`Engine` fields**:

```text
pub routines: &'a Routines,                                     // C — config
pub world: Tracked<'a, dyn World + 'a>,                         // TRACKED
pub introspector: Protected<Tracked<'a, dyn Introspector + 'a>>, // TRACKED + Protected
pub traced: Tracked<'a, Traced>,                                // TRACKED
pub sink: TrackedMut<'a, Sink>,                                 // TRACKED MUT (push-only)
pub route: Route<'a>,                                            // TRACKED via .track()
```

6 fields no Engine; **5 são Tracked / TrackedMut**.

**Status A5**: ✅ **CONFIRMADO**. Vanilla tem
arquitectura **não-monolítica**; nenhum struct vanilla é
análogo ao Layouter cristalino. **Esta é DIVERGÊNCIA
arquitectónica intencional** entre cristalino (1
Layouter) e vanilla (Engine + N Layouters).

### A6 — Sub-stores trackable em vanilla

**Tipos com `#[comemo::track]`** em vanilla
typst-library:

| Tipo | Localização | Mutabilidade |
|------|-------------|--------------|
| `World` (trait) | `typst-library/src/lib.rs:59` | read-only |
| `Traced` | `engine.rs:131` | read-only |
| `Sink` | `engine.rs:202` | **mutable** (push-only) |
| `Route<'a>` | `engine.rs:389` | mostly read, some increase/decrease |
| `Context<'a>` | `foundations/context.rs:35` | read-only |
| `Introspector` (trait) | `introspection/introspector.rs:28` | read-only (matched cristalino M8) |
| `Locator<'a>` | `introspection/locator.rs:208` | read-only |
| `LateLinkResolver<'a>` | `model/link.rs:678` | read-only |

**Tipos com `Tracked<...>` em fields** (vistos em
`typst-layout/`):

- `Composer/Distributor/Work` recebem todos os 5 Tracked
  do Engine como parameters individuais.
- Nenhum struct vanilla em `typst-layout` é
  `#[comemo::track]` directamente.

**Padrão**: vanilla usa **Padrão A literal** para tipos
infraestruturais (Sink, Route, Traced, Locator) e para
traits cross-modular (World, Introspector). **Não usa
Padrão B3** (trait + blanket impl) para layout state.

**Vanilla NÃO trackeia fields internos do Layouter** —
porque não tem Layouter monolítico. Os "fields" análogos
são `Engine.{world, introspector, traced, sink, route}` e
esses **já são Tracked cross-modular**.

**Status A6**: ✅ **CONFIRMADO**. Vanilla aplica Padrão A
literal em 8 sub-stores tracked, todos cross-modular
(via Engine ou parameters). Nenhum field interno de
Layouter é tracked.

### A7 — Mapeamento cristalino ↔ vanilla

| Field cristalino | Vanilla equivalente | Tipo |
|------------------|---------------------|------|
| `metrics: M` | (em layouter especializado, ex `GridLayouter.config`) | 1:1 ausente cross-modular |
| `sizer: S` | similar | 1:1 ausente cross-modular |
| `font_size_pt`, `style`, `chain` | `StyleChain<'a>` em parameter | 1:N (cristalino agrupa) |
| `page_config` | `pages::Config<'x>` (FootnoteConfig + ColumnConfig + ...) | 1:N (vanilla decompõe) |
| `pages` | `LayoutedPage` em pipeline | 1:1 estrutural; vanilla acumula em outro local |
| `current_items, current_line, cursor_*` | `Distributor` state | 1:N |
| `introspector` (Tracked) | `Engine.introspector` (Tracked) | **1:1 — paridade M8 literal** ✅ |
| `figure_progress` | counter via Sink+introspector | sem field equivalente; comportamento via Sink |
| `is_height_unconstrained`, `cell_*` | `GridLayouter` fields | 1:N |
| `locator` | `Locator<'a>` (`#[comemo::track]`) passed via parameter | 1:1 mas tracked em vanilla |
| `current_location` | computado on-demand | sem field; on-demand |
| `runtime.label_pages` | sub-store de `PagedIntrospector` post-layout | divergente — vanilla cria post-layout |
| `runtime.known_page_numbers` | sem equivalente directo (vanilla converge via comemo, não loops) | divergente |
| `runtime.is_readonly` | sem equivalente directo (vanilla controla via Sink/Engine) | divergente |
| `runtime.positions` | sub-store de `PagedIntrospector` post-layout | divergente — vanilla cria post-layout |

**Estatísticas**:

- **1:1 paridade**: 1 field (`introspector`).
- **1:N**: 9 fields cristalino → múltiplos vanilla.
- **divergente**: 4 fields runtime cristalino sem
  equivalente directo (vanilla resolve via PagedIntrospector
  post-layout).

**Status A7**: ✅ **CONFIRMADO**. Mapeamento é
**fundamentalmente assimétrico** — cristalino e vanilla
têm arquitecturas diferentes; F3 não pode ser "paridade
literal" porque não há vanilla literal.

---

## §3 Bloco 3 — Compatibilidade com comemo

### A8 — Bounds satisfeitos por categoria B

Para fields da Categoria B (`runtime.label_pages`,
`runtime.known_page_numbers`, `runtime.positions`):

| Sub-field | `Send + Sync`? | Returns `Hash`? | Mutação tracking-compatível? |
|-----------|----------------|------------------|-------------------------------|
| `label_pages: HashMap<Label, usize>` | ✅ (HashMap é Send+Sync se K, V são) | ❌ HashMap não é Hash; lookup retorna `Option<&usize>` (Hash) | ⚠️ mutado durante layout — paradox: tracking exige imutabilidade durante uso |
| `known_page_numbers: HashMap<Label, usize>` | ✅ | ❌ idem | ⚠️ idem |
| `positions: HashMap<Location, Position>` | ✅ | ❌ idem (lookup retorna `Option<Position>` que é Hash via P204D) | ⚠️ idem |

**Hipótese específica confirmada**: fields populated
single-pass durante layout não podem ser tracked durante
o mesmo layout. **Paradox**: tracking exige imutabilidade
durante o uso; population exige mutabilidade.

**Resolução possível**:

- **Sealing post-layout**: emular Vanilla `PagedIntrospector::new`
  — sub-stores construídos **depois** do layout terminar e
  tracked na fase de queries seguinte (ou iteração
  fixpoint seguinte).
- **Tracked apenas para reads cross-iteration**: durante
  iteração N, sub-stores são populated mutably; entre
  iteração N e N+1, snapshot é sealed e exposto Tracked
  para queries da iteração N+1.

**Status A8**: ⚠️ **DIVERGÊNCIA REGISTADA**. Categoria B
**não pode ser tracked single-pass durante layout**.
F3 exige sealing point ou divergência intencional.

### A9 — Modelo de tracking pós-layout vanilla

**Fonte**: `lab/typst-original/crates/typst-layout/src/document.rs:13-43`
+ `introspect.rs:35-58`.

```text
PagedIntrospector::new(pages: &[Page]) -> PagedIntrospector
```

Vanilla **constrói o introspector post-layout** com walk
sobre `pages` finalizadas. `PagedIntrospector` contém
`elements: ElementIntrospector<PagedPosition>` +
`frame_link_targets: FxHashSet<Location>` +
`page_numberings` + `page_supplements`.

`PagedDocument.introspector: Arc<PagedIntrospector>` é o
field que consumers lêem. Tracking aplica-se ao trait
`Introspector` (impl `for PagedIntrospector`).

**Hipótese A9 confirmada**: vanilla **trackeia
apenas pós-sealing**. Sub-stores populated durante
layout (ainda mutáveis) não são tracked.

**Cristalino diverge intencionalmente** (per P204D + P203A
C5): popula `runtime.positions` durante o layout
(single-pass), sem post-layout walk separado.

**Status A9**: ⚠️ **DIVERGÊNCIA INTENCIONAL CONFIRMADA**.
Cristalino single-pass; vanilla post-layout sealing.

---

## §4 Bloco 4 — Loops fixpoint e F3

### A10 — Loops fixpoint cristalinos vs F3

**Fonte**: `01_core/src/rules/layout/mod.rs:1535-1581` +
`01_core/src/rules/introspect/fixpoint.rs`.

Loop TOC (per `mod.rs:1545`):

```text
const MAX_ITERATIONS: usize = 5;
for _ in 0..MAX_ITERATIONS {
    let mut l = Layouter::new(..., intr_tracked);
    l.runtime.known_page_numbers = known_page_numbers.clone();
    l.layout_content(content);
    let doc = l.finish();
    if doc.extracted_label_pages == known_page_numbers {
        return doc;
    }
    known_page_numbers = doc.extracted_label_pages.clone();
}
```

**Convergência**: hash-based (comparação directa de
HashMaps). Não usa `comemo::Constraint::validate`.

**Impacto F3**:

- F3 sub-stores trackable **não substituem** convergência
  hash directamente.
- Tracking comemo + hash convergence **podem coexistir**:
  - Tracking acelera leituras dentro de cada iteração
    (cache de queries repetidas).
  - Hash convergence detecta fim do fixpoint.
- Substituição (F3 substitui hash convergence por
  tracking-based) seria refactor profundo que muda a
  semântica do fixpoint — fora-de-escopo de F3 minimal.

**Hash actual continua válido**: `extracted_label_pages`
é populated por `references.rs` durante layout (write em
`runtime.label_pages`); comparação no fim da iteração.
F3 não muda esse fluxo.

**F3 reduz iterações esperadas?** Não directamente. F3
melhora cache de queries dentro da iteração; iterações
totais dependem de convergência de page numbers (lógica
de outline/references, não de cache).

**Status A10**: ✅ **CONFIRMADO**. F3 + hash convergence
coexistem (preferido). Substituição seria refactor
ortogonal.

### A11 — Position concrete (P204D) e F3

**Fonte**: `01_core/src/rules/layout/mod.rs:272-287`.

```text
self.runtime.positions.insert(
    location,
    Position { page: NonZero..., point: ... },
);
```

`positions` é populated **single-pass** durante layout
(per P204D + ADR-0073 §C6a). Read futura via
`position_of` consumers (não exercida em produção
actualmente — TagIntrospector retorna `None` per
P204D §C6a; consumers acedem `layouter.runtime.positions`
directamente).

**Hipótese A11 confirmada**: positions single-pass
populated; tracking exige sealing pós-layout.

**Padrão para F3**:

- **Positions trackable post-layout**: sealing point após
  fim de `pub fn layout` (ou após cada iteração fixpoint);
  Position fica tracked em sub-store sealed.
- **Positions permanece em runtime (Padrão C6a)**:
  consumers continuam a aceder via
  `layouter.runtime.positions.get(loc)`; F3 não toca
  Position. Divergência intencional cristalino vs vanilla
  preservada.

**Status A11**: ✅ **CONFIRMADO**. Position é trackable
**apenas se sealing point existir**.

---

## §5 Bloco 5 — Estado pós-M8 e oportunidades

### A12 — Sub-stores cristalinos elegíveis

Critérios:

1. Categoria B (runtime introspecção).
2. Sealable pós-layout (ou pós-iteração fixpoint).
3. Tipos com `Hash` impl (ou facilmente adicionáveis).

**Candidatos**:

| Sub-field | Sealable? | Hash? | Magnitude migração |
|-----------|-----------|-------|---------------------|
| `runtime.label_pages` | Sim — após cada iteração | Lookup retorna `Option<&usize>` (Hash) | **S** — sealing após `l.finish()` |
| `runtime.known_page_numbers` | Sim — sealable entre iterações | idem | **S** — já é "sealed" entre iterações (set externamente) |
| `runtime.positions` | Sim — após cada iteração | Lookup retorna `Option<Position>` (Hash; P204D) | **S-M** — sealing + trait queries |

**Oportunidades estruturais**:

- **`label_pages` + `known_page_numbers` poderiam unir-se**:
  ambos são `HashMap<Label, usize>` representando "label →
  page". Diferença é write-during-layout vs
  read-from-previous-iteration. Sealing point natural
  entre iterações poderia eliminar a duplicação.
- **`positions` ganha trait queries reais**: P204D deixou
  `position_of` retornando `None` em `TagIntrospector`. F3
  poderia popular sub-store sealed pós-layout que retorna
  `Some(Position)`.

### A13 — Sub-stores ineligíveis

| Sub-field | Razão |
|-----------|-------|
| `runtime.is_readonly` | Categoria A (flag interno); sem semântica de query. |
| Categoria A 15 fields (cursor, chain, items, cell_*) | Runtime puro layout; não consumidos por queries; tracking irrelevante. |
| `metrics`, `sizer` | Categoria C (config); imutáveis; tracking redundante (sem cache hits). |
| `page_config` | Categoria D mutável via `Content::SetPage`; mas consumido apenas internamente pelo Layouter; sem semântica de query externa. |
| `locator`, `current_location` | Categoria D; semântica interna ao Layouter; consumers location-aware acedem via Layouter directamente, não via tracking. |

### A14 — Performance e benefício

**Benefício esperado de F3 minimal** (label_pages +
known_page_numbers + positions trackable):

- Cache hits em queries `position_of(loc)` repetidas após
  sealing — proveitoso para documentos grandes com muitas
  references.
- Cache hits em lookups `label_pages[label]` durante
  finalização de cada iteração — moderado (já é HashMap;
  comemo adiciona overhead de validation).

**Benefício real depende de**:

- Frequência de queries pós-sealing por documento.
- Custo de validation comemo vs custo de HashMap lookup
  directo.

**Vanilla mostra ganho mensurável?** `PagedIntrospector`
em vanilla é `Arc<PagedIntrospector>` (clone barato); o
ganho vem do **Arc + Tracked** combinados, não só do
tracking. Cristalino sem Arc não recolhe esse benefício
directamente.

**Sem benchmarks** (per spec; benchmarks ficam para
sub-passo dedicado se F3 prosseguir). Estimativa
qualitativa: F3 minimal **melhora arquitectura sem
ganho mensurável claro**; F3 completo (post-layout
sealing + Arc) tem potencial real mas magnitude XL.

**Status A14**: ⚠️ **NEUTRO**. Benefício de F3 minimal
**arquitectural**, não claramente performance.
Benefício de F3 completo é potencial mas exige refactor
profundo.

---

## §6 Resumo

**Achados centrais**:

1. **22 fields Layouter cristalino confirmados**; 21
   ortogonais ao `introspector` (já Tracked P204C).
2. **Categoria B é restrita ao `runtime` field** — 3
   sub-fields candidatos (`label_pages`,
   `known_page_numbers`, `positions`); 1 sub-field é A
   (`is_readonly`).
3. **Vanilla não tem Layouter monolítico** —
   arquitectura assimétrica cristalino vs vanilla. F3
   não tem paridade literal.
4. **Vanilla trackeia apenas post-sealing** —
   `PagedIntrospector::new(&pages)` constrói
   sub-stores immutáveis depois do layout terminar.
5. **Cristalino single-pass populates `positions`
   durante layout** — tracking single-pass impossível
   (paradox mutability/imutabilidade).
6. **F3 exige sealing point ou divergência
   intencional**; ambos viáveis arquitecturalmente.
7. **Loop fixpoint hash-based mantém-se** — F3
   coexiste, não substitui.
8. **Benefício performance F3 minimal incerto**; F3
   completo magnitude XL com benefício potencial real.

**Divergências registadas**:

- `P205A.div-1` — Vanilla não tem Layouter monolítico;
  arquitectura cristalino vs vanilla é
  fundamentalmente assimétrica. F3 não pode ser
  paridade literal — é solução cristalina específica.
- `P205A.div-2` — Categoria B fields são populated
  single-pass durante layout; tracking exige sealing
  point que cristalino não tem. F3 exige decisão
  arquitectural sobre sealing.

**Pré-condições para C1+**:

- A1–A14 todos com etiqueta (CONFIRMADO ou DIVERGÊNCIA).
- Material empírico suficiente para fixar C1–C11.
- ADR-0074 PROPOSTO ou não — decisão fica para C9 com
  base em A5/A6 (vanilla não tem Padrão literal para
  Layouter sub-stores).
