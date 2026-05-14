# Spec do passo P243 — M7+3 multi-region infrastructure fase (a): introduzir `Regions { current, backlog, last }` no Layouter sem activar consumers (M9d quarta sub-passo; refactor profundo Layouter; fase (a) das duas-fases DEBT-56 §"Notas"; sub-passo M7+ não-pipeline #2)

**Data**: 2026-05-14.
**Tipo**: refactor profundo Layouter (L1) — introdução de abstracção
`Regions`-like mantendo comportamento single-region observable
inalterado. **NÃO materializa** `Content::Columns` ou
`Content::Colbreak` (esses ficam para passo subsequente — fase (b)
DEBT-56). **NÃO toca pipeline walk-time** (distinto P240/P241).
**NÃO toca geometry/exporter** (distinto P242).
**Magnitude planeada**: L+ (~8-12h). Inferida pelo §8 dos
relatórios P240/P241/P242 e DEBT-56 §"Plano". Maior magnitude
absoluta da série M7+ desde inauguração P240.
**Marco**: **quarta sub-passo materialização M9d / M7+ pós-P242**;
**primeira aplicação cumulativa "refactor profundo Layouter
internal"** N=1 inaugurado P243 (distinta de refactor field-add
externo P242, refactor pipeline P240/P241, refactor entity P156L);
**primeira fase (a)** das duas-fases DEBT-56 explícitas §"Notas"
("introduzir `Region`/`Regions` mantendo comportamento single-column"
+ "consumir multi-column" cada fase candidata a passo independente);
sexta aplicação cumulativa pattern "spec C1 audit obrigatório
bloqueante pós-P236.div-1" N=5 → 6 cumulativo.

---

## §1 O que será feito

P243 materializa **fase (a) do plano duas-fases DEBT-56** explícito
em §"Notas" da DEBT: introduzir abstracção `Regions`-like no
Layouter mantendo comportamento single-region observable
inalterado. Três adições estruturais ortogonais ao Content enum
(zero variants novas):

1. **Novo tipo `Regions`** em `01_core/src/entities/regions.rs`
   contendo:
   - `current: RegionState` — região activa (Page actual a
     receber items + dimensões + cursor).
   - `backlog: Vec<RegionState>` — regions ainda não consumidas
     (vazias na fase (a); populadas só na fase (b) por columns).
   - `last: Option<RegionState>` — última region preservada para
     overflow/fallback (vazia na fase (a)).

2. **Refactor `Layouter` para encapsular state em `Regions`** —
   campos `current_items`, `current_line`, `cursor_x`, `cursor_y`,
   `line_start_x`, `page_config` (parcialmente) migram para
   `current: RegionState`. Métodos `flush_line`, `new_page`,
   `layout_word` actualizam via `self.regions.current.*`.

3. **Promoção real graded ADR-0054 de scope-outs ligados a
   "refactor multi-region"** — pelo menos 3 dos seguintes
   transitam de "armazenado mas semantic real adiada" para
   "implementado":
   - `Pad.right` scope-out P156C (§"Renderização" Pad: "Layouter
     actual não tem mecânica de largura útil por arm").
   - `Block.width` semantic real (P156G "armazenado mas não impõe
     limite real").
   - `Boxed.width/height` semantic real (P156H idem).
   - `Block.breakable: false` semantic real (P156G "armazenado
     mas layouter não impede quebra ainda").
   - Possivelmente outros — audit C1 inventaria todos os
     scope-outs que mencionam "DEBT-56" ou "refactor multi-region"
     em §"Limitações conscientes" L0.

**Tests esperados**: 2190 → ~2206 verdes (+10-18 baseline;
range alinhado P242 +15). Zero regressões esperadas em
comportamento single-region observable (paridade output PDF
pré-P243 preservada literal). **Adaptações pre-existentes:
N=~30-50** estimado — significativamente maior que P242 (N=7)
por afectar ~30+ sítios do Layouter que referenciam `cursor_x`/
`cursor_y`/`current_items`/`current_line`/`line_start_x` em
`mod.rs` + `cursor.rs` + `helpers.rs` + `placement.rs` +
`grid.rs` + `equation.rs`.

**Audit C1 P243** deve refinar 4-6 hipóteses (lição N=5 cumulativo
preservada): forma exacta `RegionState`, scope-outs a promover,
estratégia de migração field-by-field, naming `current`/`backlog`/`last`
vs alternativos.

---

## §2 Auditoria pré-P243 OBRIGATÓRIA BLOQUEANTE (C1) — lição N=6 cumulativo

**Audit empírico obrigatório** antes de qualquer código tocado
(paralelo lição refinada `P236.div-1 → P238.div-1 → P239 audit →
P240 audit → P241 audit → P242 audit` N=6 cumulativo). Aspectos
críticos:

| Aspecto a auditar | Hipótese a confirmar | Implicação se falhar |
|-------------------|---------------------|----------------------|
| `Layouter` fields actuais | ≥21 fields ortogonais per snapshot 2026-05-05 §5 (P205A); inclui `cursor_x`, `cursor_y`, `line_start_x`, `current_items`, `current_line`, `pages`, `page_config`, `style`, `chain`, `counter`, `runtime`, `introspector`, `locator`, `current_location`, etc. | Migração field-by-field é viável; audit deve produzir tabela 21 fields × {migra/preserva/refina} |
| `Layouter::new_page` em `cursor.rs:128` | Commits `current_items` numa nova `Page`, push `pages`, reset cursor (per P156E spec) | Função alvo para refactor — torna-se `Regions::advance_to_next_region` ou `flush_current_region` |
| `Layouter::flush_line` em `cursor.rs` | Existe; consultado por Pad, Block, Pagebreak, etc. | Refactor para actualizar via `self.regions.current.*` em vez de fields directos |
| `Layouter::layout_word` em `cursor.rs` | Consulta `page_config.width` para width-aware wrap | **Hipótese-chave**: width pode passar a `regions.current.width` (que na fase (a) == `page_config.width`; na fase (b) reduce para columns) |
| ADR-0074 `SealedPositions` + `LayouterRuntimeState` | P205B-E completos; pattern "Layouter-runtime → struct dedicada" estabelecido | P243 reusa pattern conceptualmente; `Regions` é nova struct dedicada análoga estruturalmente a `LayouterRuntimeState` |
| ADR-0061 §"Aplicações cumulativas" | 10 entradas pós-P242; Fase 3 Layout "parcialmente activado P156J" (repeat ✓; columns/colbreak pendentes) | P243 anota 11ª entrada; Fase 3 transita "50% → fase (a) infrastructure ✓; fase (b) consumers pendentes" |
| Scope-outs P156C-J `DEBT-56` / `multi-region` | grep empírico nos L0 prompts em `00_nucleo/prompts/entities/` + `00_nucleo/prompts/rules/layout.md` | Tabela de N scope-outs candidatos a promover real em P243 — audit confirma quais migram em P243 vs ficam diferidos |
| DEBT-56 status pós-P243 | Spec assume permanece aberto (fase (b) columns/colbreak pendente) | Confirmar; eventual nomenclatura `DEBT-56a` (fase a fechada P243) + `DEBT-56b` (fase b pendente) — alternativa: deixar DEBT-56 aberto inteiro até columns/colbreak materializem |
| Comemo memoization invariants ADR-0073/0074 | Trait `Introspector` + `SealedPositions` não tocados | Refactor `Layouter` é cross-module L1 mas isolado de trait — invariants preservados |
| `measure_content_constrained` | Função que retorna dimensões dado constraint width | Função a refactorar: `width` passa a vir de `regions.current.width` (paramétrico) |
| `cell_available_h: Option<f64>` field (P83) | Existe; alimentado pelo Grid arm para passar altura disponível a Align dentro de células | **Candidato natural para refactor**: cell layout pode usar `Regions` com region única de altura limitada — primeira validação do pattern em estrutura existente |
| Tests baseline pré-P243 | **2190 verdes** confirmado pós-P242 | Baseline para +10-18 |

**Sem `P243.div-N` formal antecipado**, mas dada a magnitude L+
o risco de bloqueador material é maior que precedentes:

- Se audit revelar que `cursor_x`/`cursor_y` são lidos directamente
  por ≥50 sítios cross-module (não apenas dentro do submódulo
  `layout/`), criar `P243.div-1` formal documentando estratégia
  de migração via deprecated proxies (`Layouter::cursor_x()` etc.)
  antes de remover fields.
- Se audit revelar que `cell_available_h` está acoplado a Grid
  em forma que não permite generalização, ajustar spec para
  Decisão 4 abaixo (cell layout migration parcial).

---

## §3 Decisões fixadas P243 — 10 decisões

### Decisão 1 — Tipo `Regions` paralelo conceptual a `LayouterRuntimeState`

**Forma esperada** (`01_core/src/entities/regions.rs` — ficheiro
novo):

```rust
#[derive(Debug, Clone)]
pub struct Regions {
    pub current: RegionState,
    pub backlog: Vec<RegionState>,
    pub last:    Option<RegionState>,
}

#[derive(Debug, Clone)]
pub struct RegionState {
    /// Dimensões úteis desta region.
    pub width:  f64,    // Pt
    pub height: f64,    // Pt
    /// Items acumulados nesta region (anteriormente `current_items`
    /// no Layouter directo).
    pub items:  Vec<FrameItem>,
    /// Linha em construção (anteriormente `current_line`).
    pub line:   Vec<FrameItem>,
    /// Cursor X (anteriormente `cursor_x`).
    pub cursor_x: f64,
    /// Cursor Y (anteriormente `cursor_y`).
    pub cursor_y: f64,
    /// Start X da linha actual (anteriormente `line_start_x`).
    pub line_start_x: f64,
}

impl Regions {
    /// Construtor para single-region (caso fase (a)). Backlog
    /// vazio; `last` None.
    pub fn single(width: f64, height: f64) -> Self;

    /// Avança para próxima region — commit current → Page, copy
    /// from backlog se houver, senão criar nova via callback do
    /// caller.
    pub fn advance(&mut self) -> Option<RegionState>;
}
```

**Naming `current`/`backlog`/`last`** preserva nomenclatura
vanilla (per DEBT-56 §"Contexto" referência a `Regions` vanilla);
audit C1 confirma. Alternativa rejeitada: `active`/`pending`/
`previous` — paridade vanilla preferida per ADR-0033.

L0 novo: `00_nucleo/prompts/entities/regions.md` paralelo
conceptual `layouter_runtime_state.md`. **Sub-padrão #14 "Tipo
entity em ficheiro próprio" N=6 → 7 cumulativo** (Sides P156C →
Parity P156E → Dir P156I → BibEntry P159A → CitationForm P159C →
Corners P242 → **Regions P243**).

### Decisão 2 — Migração field-by-field do Layouter

Os 7 fields `cursor_x`, `cursor_y`, `line_start_x`, `current_items`,
`current_line`, e potencialmente `page_config.width`/`height`
(parcial) migram para `regions.current.*`. Layouter ganha
**novo field `regions: Regions`** substituindo os 5-7 fields
directos. Métodos `flush_line`, `new_page`, `layout_word` actualizam
via deref `self.regions.current.*`.

**Restantes 14+ fields preservados inalterados**: `pages`,
`page_config` (porção structural; height/width podem migrar para
`regions.current`), `style`, `chain`, `counter`, `runtime`,
`introspector`, `locator`, `current_location`, `cell_available_h`,
`tags`, etc.

**Migração técnica via "Opção β"** (sem proxies deprecados):
substituir todos os ~30-50 sítios `self.cursor_x` →
`self.regions.current.cursor_x` em commit único. Audit C1
confirma viabilidade vs "Opção α" via proxies graduais
(rejeitada — adicionaria deuda técnica imediata).

### Decisão 3 — Fase (a) preserva single-region observable literal

P243 **NÃO altera output PDF** para qualquer test pré-existente.
Comportamento observable preservado:
- `Regions::single(page_width, page_height)` em `Layouter::new`.
- `Regions::advance` na fase (a) sempre cria nova region com
  `single(page_width, page_height)` (porque backlog é vazio).
- `new_page` torna-se wrapper para `regions.advance` + push
  `Page` to `pages`.

**Activação de fase (b)** (columns/colbreak) fica para passo
subsequente — populates `backlog` com regions reduzidas.

### Decisão 4 — Promoção real ≥3 scope-outs ligados a multi-region

**Lista candidata** (audit C1 confirma exact set):
- ✓ `Pad.right` scope-out P156C (mecânica "largura útil por arm").
- ✓ `Block.width` semantic real (limite de largura no arm).
- ✓ `Boxed.width` semantic real (paralelo Block).
- ? `Boxed.height` semantic real (depende cell_available_h
  generalization).
- ? `Block.breakable: false` semantic real (depende de
  flush/no-flush decision per region).

**Mecanismo de promoção**: ao chamar `flush_line` ou
`layout_word`, consultar `self.regions.current.width` em vez
de `self.page_config.width`. Para arms ricos (Pad/Block/Boxed),
salvar/restaurar largura efectiva da region via
`regions.current.width = original - left - right` (paridade
mecânica vanilla).

**Sub-padrão "promoção real scope-out ADR-0054 graded"** N=1
(P242) → **2 cumulativo** (P242 radius/clip + **P243 multi-region
scope-outs**). Atinge limiar formalização N=2-3 (candidato a ADR
meta em passo administrativo XS futuro).

### Decisão 5 — Sem `Content::Columns`/`Colbreak` em P243

Variants novas explicitamente fora do escopo P243 — ficam para
fase (b) DEBT-56. Razões:
- Magnitude P243 já L+ (~8-12h); adicionar 2 variants + stdlib
  + arms cresce para XL.
- DEBT-56 §"Notas" recomenda explicitamente split em duas fases
  (a) infra + (b) consumers — paridade literal.
- Granularidade preservada (P156K-meta ADR-0064 padrão N=10+
  passos sem reformulação).

DEBT-56 §"Critério de fecho" lista 4 sub-itens — P243 cumpre
**1 dos 4** ("Refactor minimal `Layouter` para multi-region"
↔ "Regions infrastructure" P243); os outros 3 (`Content::Columns`
+ `native_columns` + Layouter consumer multi-column + 5-10 tests)
ficam para fase (b).

### Decisão 6 — Sem ADR dedicada column flow algorithm em P243

DEBT-56 §"Pré-requisitos" #1 lista "ADR dedicada column flow
algorithm" como pré-requisito. **P243 difere** — criação da ADR
fica para fase (b) (quando o algoritmo for materializado).

Razão: ADR dedicada precisa de decisões sobre balanceamento
(scope-out vanilla), interaction with pagebreak, gutter
calculation, weak colbreak collapse — todas relevantes só
quando `Content::Columns` materializar. Em P243 (fase (a))
nenhuma destas decisões é tomada — apenas o transporte
abstracto `Regions { current, backlog, last }`.

**Excepção ADR-0080 NÃO necessária** — P243 toca L0 partial
mas a sub-categoria continua a ser "Layouter internal refactor"
(nova subcategoria; ver Decisão 8).

### Decisão 7 — `cell_available_h` (P83) integration deferida

P83 introduziu `cell_available_h: Option<f64>` no Layouter para
passar altura disponível a Align dentro de cells. Field existente
NÃO migra para `Regions` em P243 — fica como field directo.

**Razão**: cell layout actual usa `cell_available_h` como
constraint pontual passada ao arm Align; generalização para
`Regions { current: RegionState { height: cell_height } }` seria
elegante mas exigiria refactor cross-module Grid + Align +
helpers. Defer para passo futuro NÃO reservado (e.g. P244 ou
posterior).

Anotação: A.4 breakable per-cell (mencionado em §8 P242 como
desbloqueio M7+3) é **parcialmente desbloqueado** por P243
(infra disponível) mas **não materializado** (depende de cell
layout migration).

### Decisão 8 — Nova sub-categoria ADR-0080 "Layouter internal refactor"

P240/P241 abriram sub-categoria "L0 tocado runtime + walk
integration" (N=2 cumulativo). P242 abriu sub-categoria
"geometry/exporter infrastructure" (N=1). P243 abre **terceira
sub-categoria** "Layouter internal refactor" — distinta das
anteriores porque:
- Não toca pipeline walk (vs sub-cat 1).
- Não toca geometry/exporter (vs sub-cat 2).
- Toca **estrutura interna** do Layouter (L1) que afecta arms
  cascata cross-feature.

L0 a tocar (estimado 5-7 ficheiros):
- `entities/regions.md` (**ficheiro novo**).
- `entities/layouter_runtime_state.md` (cross-reference para
  Regions; pattern paralelo).
- `rules/layout.md` (§"Layouter struct" ou similar — referência
  Regions).
- `entities/content.md` (actualizar §"Limitações conscientes" P156C
  Pad.right + P156G Block.width + P156H Boxed.width — remover
  ou marcar como "fechado P243").
- Possivelmente outros conforme audit C1.

**Quarta excepção justificada ADR-0080 EM VIGOR pós-P229** —
N=3 (P241+P242 em sub-categorias diferentes) → **4 cumulativo
em 3 sub-categorias distintas**. Anotação ADR-0080 §"Excepções"
entrada P243 sub-categoria nova.

### Decisão 9 — Tests E2E focam preservação observable

A maioria dos tests P243 (estimado ~12-15 de 16-18 total) são
**regression tests** confirmando que comportamento single-region
pré-P243 é preservado literal:
- Texto multi-página continua a quebrar entre páginas no mesmo
  ponto.
- Pad/Block/Boxed renderizam dimensões idênticas (modulo
  scope-outs promovidos em Decisão 4).
- Cell layout em Grid preserva comportamento P83.

Tests **novos** (estimado 4-6) focam:
- `Regions::single` cria region com dimensões correctas.
- `Regions::advance` em fase (a) preserva semantic new_page.
- Promoções scope-out Decisão 4 funcionam (e.g. Pad.right
  efectivo; Block.width limita real).

### Decisão 10 — Sem fechamento Fase 5 / ADR-0061 / DEBT-56

P243 completa M7+3 fase (a) (4/5 M7+ sub-passos). Cobertura
Layout per metodologia transita ~91-92% (pós-P242) → **~93-94%**
(refino qualitativo + parcial quantitativo via scope-outs
promovidos). Fase 5 graded transita 13/13-15 → **14/13-15**.

**Sem promoção formal ADR-0061 → IMPLEMENTADO** — depende de
fase (b) DEBT-56 (`columns`/`colbreak`) ou decisão humana
scope-out formal.

**Sem fechamento DEBT-56** — fase (b) pendente. Mas anotação
DEBT-56 §"Plano" actualiza checklist:
- ✓ "Refactor minimal `Layouter` para multi-region" — P243.
- ✗ ADR dedicada column flow — fase (b).
- ✗ `Content::Columns` + `Content::Colbreak` — fase (b).
- ✗ `native_columns` + `native_colbreak` — fase (b).
- ✗ Layouter consumer multi-column — fase (b).
- ✗ Tests + inventário 148 + DEBT fecho — fase (b).

---

## §4 `Regions` + `RegionState` (C2+C3)

Forma detalhada em §3 Decisão 1.

**Tests dedicados** (4 tests em `entities/regions.rs`):
- `p243_regions_single_constroi_uma_region_backlog_vazio_last_none`.
- `p243_region_state_default_zero_dimensions_zero_cursor`.
- `p243_regions_advance_fase_a_preserva_dimensoes_via_callback`.
- `p243_regions_clone_eq_funcionam`.

---

## §5 Refactor Layouter para `regions: Regions` (C4)

**Sítios cross-module afectados** (estimado 30-50, audit C1
inventaria exact list):

Em `01_core/src/rules/layout/mod.rs`:
- `Layouter` struct definição — substituir 5-7 fields directos
  por `pub(super) regions: Regions`.
- `Layouter::new` — inicializar com `Regions::single(width, height)`.
- Layout arms que lêem `self.cursor_*` (estimado 15-20 sítios).
- Layout arms que escrevem `self.current_items` ou
  `self.current_line` (estimado 10-15 sítios).

Em `01_core/src/rules/layout/cursor.rs`:
- `flush_line`, `new_page`, `layout_word` — actualizar para
  via `self.regions.current.*`.
- `new_page` torna-se wrapper: `self.regions.advance()` +
  `pages.push(Page::new(items_drained_from_advance))`.

Em `01_core/src/rules/layout/helpers.rs`:
- `item_pos`, `translate_frame_item`, `measure_content`,
  `collect_sub_items` — actualizar consultas.

Em `01_core/src/rules/layout/placement.rs`, `grid.rs`,
`equation.rs`, `metrics.rs`:
- Pattern-match cascata onde aplicável.

**Migração técnica**: usar `cargo check` iterativamente para
identificar todos os sítios; substituir em commit único após
audit C1 confirmar inventário completo.

---

## §6 Promoções scope-out Decisão 4 (C5)

`Pad.right` arm em `layout/mod.rs`: actualizar comportamento
para usar `self.regions.current.width -= sides.right`
temporariamente durante body layout; restaurar após body.

`Block.width` / `Boxed.width` arms: similar — clamp
`self.regions.current.width` ao valor user-provided durante
body layout; restaurar.

**L0 actualizado**: `entities/content.md` §"Limitações conscientes"
P156C/G/H — secções relevantes marcadas como "fechado P243"
ou anotação cruzada.

---

## §7 Critério aceitação P243 (C8+C9+C10)

| Critério | Esperado |
|----------|----------|
| `cargo build --workspace` | verde |
| `cargo test --workspace` | **~2206 verdes** (range 2200-2208; +10-18 vs 2190 baseline P242) |
| `crystalline-lint .` | 0 violations |
| `crystalline-lint --fix-hashes` | 5-7 L0 hashes + ~15-25 ficheiros L1 actualizados (estimado superior a P242 por refactor cross-module Layouter) |
| Adaptações pre-existentes | **N=~30-50** (significativamente maior que P242 N=7; paridade conceptual P156L refactor field-rename mas em escala maior) |
| Content variants | 62 preservado (zero variants novas) |
| Layouter fields directos | ~21 → ~14-16 (5-7 fields migram para regions.current) |
| Tipos entity novos | **+1 Regions** (paralelo `LayouterRuntimeState` P190C) |
| Tipos entity ortogonais | **+1 RegionState** (sub-tipo de Regions) |
| Helpers stdlib novos | 0 (sem stdlib touched) |
| Scope-outs promovidos | **≥3** (Pad.right + Block.width + Boxed.width mínimo) |
| ADR-0081 status | IMPLEMENTADO parcial 3/5 → 4/5 |
| ADR-0061 §"Aplicações cumulativas" | 11ª entrada Layout adicionada |
| ADR-0080 §"Excepção P243" | anotada N=4 cumulativo, sub-categoria nova "Layouter internal refactor" |
| DEBT-56 status | EM ABERTO preservado; checklist ✓ "Refactor minimal Layouter" anotado |
| L0 partial tocado | 5-7 ficheiros |
| Regressões reais | **0** (preservação literal single-region observable) |

**Tests P243** (estimativa ~16-18 unit + cenários canónicos):

**Unit regions** (4 tests em `entities/regions.rs`) — ver §4.

**Unit layouter regression** (4-5 tests em `rules/layout/tests.rs`):
- `p243_layouter_new_inicializa_regions_single_dimensions_corretas`.
- `p243_layouter_new_page_usa_regions_advance_preserva_observable`.
- `p243_layouter_flush_line_via_regions_current_preserva_observable`.
- `p243_layouter_multi_page_text_quebras_idênticas_pre_p243`
  (regression literal).
- `p243_layouter_cursor_advance_via_regions_current`.

**Unit scope-outs promovidos** (4-6 tests):
- `p243_pad_right_efetivo_limita_largura_word_wrap`.
- `p243_block_width_efetivo_clampa_largura_real`.
- `p243_boxed_width_efetivo_clampa_largura_real`.
- `p243_block_breakable_false_atomic_no_flush_inside`
  (condicional na Decisão 4).
- `p243_pad_block_aninhado_largura_cumulativa_correta`.
- `p243_largura_negativa_apos_pad_extremo_rejeita_ou_clampa_zero`.

**Unit/E2E regression preservation** (3-4 tests):
- `p243_grid_celula_layout_preservado` (cell_available_h
  preservado per Decisão 7).
- `p243_pagebreak_funciona_via_regions` (refactor não-disruptive).
- `p243_layout_unicode_multipagina_identico_pre_p243`.

**3 pré-condições obrigatórias verificadas** (per ADR-0081
§"Pré-condições obrigatórias"):
1. **Tests baseline preservados**: 2190 verdes pré-P243 →
   ~2206 verdes pós-P243 (+10-18; 0 regressões observable
   esperadas). **N=30-50 adaptações triviais field-rename**
   (~`self.cursor_x` → `self.regions.current.cursor_x`).
2. **Comemo memoization invariants ADR-0073/0074 preservados**:
   P243 não toca trait `Introspector`, `SealedPositions`, nem
   `LayouterRuntimeState`. Refactor isolado a fields Layouter
   internos; invariants preservados literal.
3. **Backward compat observable**: output PDF de tests
   pré-P243 idêntico literal (modulo scope-outs promovidos
   Decisão 4 — comportamento melhorado, não pior).

**Promoções ADR esperadas**:
- ADR-0081 IMPLEMENTADO parcial **3/5 → 4/5** (M7+3 fase (a) ✓;
  M7+4 pendente; **M7+3 fase (b) tratada como sub-passo
  independente fora M7+ count** — ver §8). Distribuição
  preservada literal.
- ADR-0061 §"Aplicações cumulativas" **11ª entrada** Layout
  adicionada — primeira aplicação "refactor profundo Layouter
  internal" N=1 inaugurada.
- ADR-0080 §"Excepções" entrada P243 anotada N=4 cumulativo;
  **terceira sub-categoria** "Layouter internal refactor".
- DEBT-56 §"Plano" checklist ✓ item 1 ("Refactor minimal
  Layouter") sem fechamento da DEBT global.
- Sub-padrão #14 "Tipo entity em ficheiro próprio" N=6 →
  **7 cumulativo**.
- Sub-padrão "promoção real scope-out ADR-0054 graded" N=1 →
  **2 cumulativo** (P242 radius/clip + P243 multi-region attrs).

**Inventário 148 footnote ⁶²** adicionada (~300 linhas estimadas
— maior que P242 por reflectir magnitude L+) documentando:
M7+3 fase (a) infrastructure materializada; lição N=6 cumulativo
C1 audit validada; sub-padrão "refactor profundo Layouter
internal" N=1 inaugurado; sub-padrão #14 N=6 → 7; sub-padrão
"promoção real scope-out" N=1 → 2; quarta excepção justificada
ADR-0080 sub-categoria nova; 10 decisões fixadas; A.4 breakable
per-cell parcialmente desbloqueado (infra ✓; activação em passo
futuro).

---

## §8 Próximo sub-passo pós-P243

P243 completa M7+3 fase (a). Restantes pendentes (magnitude
cumulativa restante ~13-20h):

| Caminho | Trabalho | Magnitude | Prioridade |
|---------|----------|-----------|------------|
| **M7+3 fase (b)** | `Content::Columns` + `Content::Colbreak` + `native_columns` + Layouter consumer multi-column + ADR dedicada column flow + tests | L (~5-8h) | **alta** (fecha DEBT-56 + completa M7+3 + activa fase (b) após infra P243; promoção ADR-0061 → IMPLEMENTADO possível) |
| M7+4 Place float real | Reabertura Opção B P219 graded | L (~5-8h) | média (desbloqueia C.1; isolada de fase (b)) |
| Cell layout migration para Regions | `cell_available_h` → `regions.current.height` (per Decisão 7 diferida); activa A.4 breakable per-cell | M (~2-4h) | média (refino sequente P243 natural; passo XS dedicado) |
| Refino A.4 — `outset`+`fill`+`stroke` Block+Boxed | 3 dos 4 scope-outs restantes pós-P242 | S-M por attr | baixa-média |
| ADR meta admin XS | Promoção formal patterns cumulativos: refino paralelo callers fixpoint N=2; tipo entity #14 N=7; promoção real scope-out N=2; refactor profundo Layouter N=1 | XS por pattern | média (3 patterns atingem limiar N=2+) |
| Pivot outro módulo | Visualize 54%; Text 52%; Model 50% | varia | baixa |
| Pausa M-fase | Fase 5 graded ~93-94% (14/13-15 sub-passos) | XS | baixa |

**Recomendação subjectiva pós-P243**: **M7+3 fase (b)**. Sequência
natural: infra (a) P243 prepara consumers (b) seguinte; magnitude
L isolada (~5-8h); fecha DEBT-56 e potencialmente promove ADR-0061
→ IMPLEMENTADO. Alternativa: M7+4 Place float (magnitude L
isolada; desbloqueia C.1; sem dependência fase (b)).

**Decisão humana fica em aberto literal** pós-P243.

**Estado esperado pós-P243**:
- Tests workspace: 2190 → **~2206 verdes** (+16 P243 estimado).
- Content variants: 62 preservado.
- ShapeKind variants: 5 preservado (sem alteração).
- Layouter fields directos: ~21 → **~14-16** (5-7 migram para
  regions.current).
- **Tipos entity novos: +1 Regions + 1 RegionState** (paralelo
  conceptual LayouterRuntimeState P190C).
- Stdlib funcs: 64 preservado.
- §A.5 distribuição: preservada.
- Cobertura Layout per metodologia: ~91-92% → **~93-94%** (refino
  qualitativo + parcial quantitativo via scope-outs promovidos
  Decisão 4).
- Cobertura user-facing total: ~73-74% → **~74-75%** (scope-outs
  promovidos bonus marginal).
- **ADRs distribuição preservada literal**: PROPOSTO 12; EM VIGOR
  29; IMPLEMENTADO 22; total **68 preservado**. ADR-0081 transita
  3/5 → **4/5** internamente. ADR-0061 §"Aplicações cumulativas"
  11ª entrada. ADR-0080 §"Excepção P243" N=4 cumulativo sub-
  categoria 3.
- **Saldo DEBTs: 11 preservado** (DEBT-56 anotada mas não
  fechada).
- **35 aplicações cumulativas anti-inflação** pós-P205D.
- **Patterns emergentes pós-P243** (4 inaugurados/consolidados):
  - "Refactor profundo Layouter internal" N=1 inaugurado P243.
  - "Tipo entity em ficheiro próprio" (sub-padrão #14) N=6 →
    **7 cumulativo** (+Regions).
  - "Promoção real scope-out ADR-0054 graded" N=1 → **2
    cumulativo** (P242 radius/clip + **P243 multi-region scope-outs**).
  - "Spec C1 audit obrigatório bloqueante" N=5 → **6 cumulativo**.
- **Categoria D Fase 5 Layout: 3/? sub-passos materializados**
  preservado (D.1+D.2+D.3 pós-P241; P242+P243 são Categoria A).
- **Categoria A.4 Fase 5 Layout**: parcial P242 → **parcial+
  P243** (radius+clip ✓ P242; breakable per-cell parcialmente
  desbloqueado P243 — infra ✓; activação pendente cell layout
  migration).
- **Fase 5 Layout candidata: 13/13-15 → 14/13-15 sub-passos
  materializados** (~93-100% cumulativo dependendo de granularidade).
- **M9d / M7+ progresso**: **4/5 sub-passos materializados**
  (M7+1 ✓; M7+2 ✓; **M7+3 fase (a) ✓**; M7+5 ✓; M7+3 fase (b) +
  M7+4 pendentes; cumulativa restante ~13-20h se M7+3 fase (b)
  contar como sub-passo independente, ou ~10-16h se considerado
  continuação P243).

---

## §9 Notas operacionais para o executor

1. **Audit C1 PRIMEIRO** — não tocar código antes de validar
   empíricamente os 12 aspectos da tabela §2. Lição N=6
   cumulativo. Magnitude L+ aumenta criticidade do audit.

2. **Migração field-by-field — `cargo check` é amigo**: substituir
   `self.cursor_x` → `self.regions.current.cursor_x` em commit
   único após audit confirmar set completo de sítios. Cargo
   reportará todos os sítios remanescentes via erro de compilação.
   Não fazer migração parcial com fallback.

3. **Preservação observable é critério bloqueante** — qualquer
   test pré-P243 que falhe em ≥1 byte diferente do PDF original
   é regressão real e deve parar P243 imediatamente para
   investigação. Modulo scope-outs promovidos Decisão 4
   (comportamento melhorado expected).

4. **Sem ADR dedicada column flow em P243** — fica para fase (b).
   P243 é puramente refactor infraestrutural; decisões
   algorítmicas (balanceamento, gutter, colbreak weak collapse)
   ficam para quando `Content::Columns` materializar.

5. **`cell_available_h` NÃO migra em P243** (Decisão 7). Cell
   layout preservado literal via field directo no Layouter.
   Refino futuro em passo dedicado XS (NÃO reservado).

6. **L0 partial tocado é quarta excepção justificada ADR-0080**
   — anotar como **sub-categoria 3** "Layouter internal refactor"
   (distinta P240/P241 "features runtime walk integration" e
   P242 "geometry/exporter infrastructure"). Permite N=4 cumulativo
   em 3 sub-categorias distintas.

7. **`P243.div-N` é provável** dada magnitude L+ — não-emergir
   `div` seria surpreendente. Cenários candidatos:
   - Audit revela ≥50 sítios cross-module → `P243.div-1`
     estratégia proxies deprecated.
   - `cell_available_h` revelar acoplamento com Grid maior que
     esperado → `P243.div-2` cell migration parcial.
   - Width-aware wrap em `layout_word` revelar dependência
     `page_config.width` directa em sítios profundos → `P243.div-3`
     refactor staged.

   **Em qualquer cenário**: criar `P243.div-N` formal, documentar
   estratégia, e prosseguir. Lição N=6 cumulativo ainda preserva
   "audit C1 ajustes triviais sem div-N" para casos comuns;
   div-N obrigatório só para bloqueador material.

8. **DEBT-56 anotação** — actualizar §"Plano" checklist ✓ item
   1 ("Refactor minimal Layouter") + adicionar nota P243 fase (a)
   completa. **Sem fechamento global** — fase (b) preserva DEBT-56
   aberta.

9. **Cobertura quantitativa Layout pequena** — refactor refino
   qualitativo (+2pp) + parcial quantitativo via scope-outs
   promovidos. Sem novos variants Content explica delta moderado
   (vs P242 que adicionou ShapeKind variant).
