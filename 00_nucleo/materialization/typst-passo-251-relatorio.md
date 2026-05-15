# Relatório do passo P251 — A.4 TableCell row break real via slice frame items at height (γ-Items); segunda aplicação citante ADR-0082 PROPOSTO N=2; activa Categoria C.2 Fase 5 Layout parcial cell-level

**Data**: 2026-05-14.
**Spec**: `00_nucleo/materialization/typst-passo-251.md`.
**Tipo**: refino consumer Layouter — γ-Items slice + buffer
pending + flush em new_page chain. Promove scope-out P157B
TableCell.body overflow de "clip implícito P248" para "row break
vertical real cell-level".
**Magnitude planeada**: L (~6-8h). **Magnitude real**: **M-L
(~3-4h)** — audit C1 revelou `layout_sub_frame_with_width`
retorna items com `pos.y` local; γ-Items reuso directo de
helpers existentes (`item_pos`, pattern `DeferredFloat` P245);
modulo `slicing.rs` puro com 6 variants exhaustive.
**Marco**: **segunda aplicação cumulativa citante ADR-0082
PROPOSTO** N=1 → 2 (paridade pattern ADR-0065 P156K validado
pós-P156J N=1 → P157A N=2 → P157B N=3 EM VIGOR); **promoção
real scope-out TableCell P157B graded "clip implícito P248"
→ row break vertical real**; **activa Categoria C.2 Fase 5
Layout parcialmente** (cell-level apenas); **primeira aplicação
cumulativa do padrão "Slice frame items at height via filter
+ rebase pos.y"** N=1 inaugurado; **subpadrão "DeferredX buffer
+ flush em new_page"** N=1 → N=2 cumulativo (P245 floats + P251
cell tails); décima quarta aplicação cumulativa pattern "spec
C1 audit obrigatório bloqueante pós-P236.div-1" N=13 → 14
cumulativo (lição refinada P251: "audit C1 deve confirmar
localidade pos.y antes de fixar abordagem γ-Items vs γ-Content").

---

## §1 O que foi feito

P251 materializa Categoria C.2 Fase 5 Layout **parcialmente
cell-level** via γ-Items (slice frame items por threshold +
buffer pending + flush em new_page chain). **Multi-region
completo** (column flow DEBT-56) continua diferido NÃO-reservado
per política P158.

**Trabalho real**:

1. **Novo módulo `01_core/src/rules/layout/slicing.rs`** (~270 LoC
   incluindo 10 unit tests):
   - `pub(super) fn slice_frame_items_at_height(items, threshold)
     -> (head, tail)` — função pura.
   - `pub(super) fn rebase_item_y(item, delta) -> FrameItem` —
     match exhaustive sobre 6 variants
     (Text/Line/Glyph/Image/Shape/Group); `Line` rebase
     simétricamente `start.y` + `end.y` para preservar geometria.
   - `Group.items` **não** recursivamente rebased (espaço local
     relativo a Group.pos per comentário `layout_types.rs`).
2. **Layouter +1 field** `pending_cell_tails:
   Vec<DeferredCellTail>` (paridade arquitectural P245
   `floats_pending`).
3. **+1 struct local** `DeferredCellTail` (items + origin_x +
   width + fill + stroke + `forwarded_count: u32`).
4. **+1 método** `flush_pending_cell_tails()` em `cursor.rs`
   chamado **no fim de `new_page()`** após cursor reset (items
   emit no topo da nova página; Z-order paridade P248: fill
   atrás → items rebased por cursor_top → stroke à frente).
   Limit 3 forwardings (paridade vanilla heurística).
5. **Refactor `grid.rs:393-449`** cell overflow:
   - Rows `TrackSizing::Fixed` preservam P248 clip implícito
     (paridade vanilla "Fixed rows clip overflow").
   - Rows Auto/Fraction usam P251: `slice_frame_items_at_height`
     com `threshold = body_y + body_h`; head emit directo; tail
     push ao `pending_cell_tails`.
6. **L0 `entities/region.md` extensão** documentando
   `pending_cell_tails` field + `DeferredCellTail` struct +
   `flush_pending_cell_tails` method + sub-padrões emergentes;
   hash propagado `6eec928d`.
7. **18 tests novos** (range +15-25 paridade L):
   - 10 unit em `layout/slicing.rs` (slice vazio, todos head,
     todos tail rebased, mistos, atomic Shape, threshold zero,
     rebase 4 variants Text/Line/Shape/Group + Group items
     locais preservados).
   - 8 unit/E2E em `layout/tests.rs` (row Fixed preserva P248;
     row Auto P251 slice; cell sem overflow preserva P248;
     pending_cell_tails inicial vazio; tail flushed em
     new_page via pagebreak; cell overflow + fill re-emit;
     flush vazio no-op; 2 rows independentes).
8. **N=0 adaptações** em tests pré-existentes — sentinelas P248
   usam `TrackSizing::Fixed` rows que preservam clip implícito
   paridade vanilla; backward compat literal estrita.
9. **ADRs anotadas cumulativas**: 0061 §"Refino futuro" + 0079
   §"Anotação cumulativa P251 — Categoria C.2 parcial activada"
   + 0080 §"Lição refinada P251" N=13 → 14 cumulativo + 0054
   §"Promoções reais cumulativas" tabela N=13 + **0082
   §"Aplicações citantes" N=1 → N=2** (segunda aplicação citante
   explícita).

**2276 → 2294 verdes** (+18 P251; **0 regressões**; **0
adaptações** em tests pré-existentes — primeira sub-padrão
cumulativo de backward compat literal estrita pós-P248).
**Sem `P251.div-N`** — audit converge com Decisões 1-11 + helpers
puros reusáveis.

---

## §2 Auditoria pré-P251 OBRIGATÓRIA BLOQUEANTE (C1) — lição N=13 → 14 cumulativo

**Audit empírico** (lição refinada P250 N=13 → P251 N=14
cumulativo: "audit C1 deve confirmar localidade pos.y antes de
fixar abordagem γ-Items vs γ-Content para slicing"):

| Aspecto | Hipótese Spec | Realidade Empírica | Implicação |
|---|---|---|---|
| `layout_sub_frame_with_width` signature | hipotetizou pos.y local | ✓ Confirmado mod.rs:2046 + comentário literal | Decisão 1 fixa γ-Items |
| P248 clip implícito exacto | grid.rs:393-433 | ✓ Confirmado | Refactor target identificado |
| Slicing patterns pré-existentes | hipotetizou zero | ✓ Confirmado zero | P251 inaugura pattern |
| FrameItem variants | hipotetizou 4 | ✓ Confirmado **6** (Text/Line/Glyph/Image/Shape/Group) | rebase_item_y exhaustive sobre 6 |
| Vanilla algorithm row break | hipotetizou via lab/typst-original | Disponível em `crates/typst-layout/src/grid/` | Decisões 6-8 (atomic, recursive, re-emit) preliminares confirmados |
| DeferredFloat pattern P245 | hipotetizou reusável | ✓ Confirmado paridade directa | DeferredCellTail paralelo |
| Tests baseline pré-P251 | 2276 verdes | ✓ Confirmado | Baseline preservado |

**Conclusão audit C1**: trabalho real ~330 LoC L1 + ~370 LoC tests
+ ~100 LoC L0 docs + ~200 LoC ADRs. Magnitude real **M-L (~3-4h)**
face L (~6-8h) hipotetizado — γ-Items facilitado por helpers
existentes (`item_pos`, `translate_frame_item`).

**Sem `P251.div-N`** — audit converge com spec; helpers reusados;
nenhuma divergência arquitectural identificada.

---

## §3 Módulo `slicing.rs` + struct + field + método (C2+C3)

```rust
// 01_core/src/rules/layout/slicing.rs (~270 LoC)
pub(super) fn slice_frame_items_at_height(
    items: Vec<FrameItem>,
    threshold: f64,
) -> (Vec<FrameItem>, Vec<FrameItem>) {
    let mut head = Vec::with_capacity(items.len());
    let mut tail = Vec::new();
    for item in items {
        let y = item_y_start(&item);
        if y >= threshold {
            tail.push(rebase_item_y(item, -threshold));
        } else {
            head.push(item);
        }
    }
    (head, tail)
}

pub(super) fn rebase_item_y(item: FrameItem, delta: f64) -> FrameItem {
    match item {
        FrameItem::Text { pos, .. } => /* rebase pos.y */,
        FrameItem::Line { start, end, .. } => /* rebase start.y + end.y */,
        FrameItem::Glyph { pos, .. } => /* idem */,
        FrameItem::Image { pos, .. } => /* idem */,
        FrameItem::Shape { pos, .. } => /* idem */,
        FrameItem::Group { pos, items, .. } =>
            /* rebase Group.pos; Group.items NÃO recursivamente rebased
               (espaço local relativo a Group.pos) */,
    }
}
```

```rust
// 01_core/src/rules/layout/mod.rs
pub(super) struct DeferredCellTail {
    pub items:           Vec<FrameItem>,  // rebased pos.y
    pub origin_x:        f64,             // cell column-aligned
    pub width:           f64,             // body_w preservado
    pub fill:            Option<Color>,   // re-emit Z-order 1
    pub stroke:          Option<Stroke>,  // re-emit Z-order 3
    pub forwarded_count: u32,             // max 3 iter
}

pub struct Layouter {
    // ... fields existentes ...
    pub(super) pending_cell_tails: Vec<DeferredCellTail>,  // P251
}
```

```rust
// 01_core/src/rules/layout/cursor.rs
pub(super) fn new_page(&mut self) {
    self.flush_pending_floats();      // P245 — old page
    // close + setup new page ...
    self.flush_pending_cell_tails();  // P251 — new page top
}

pub(super) fn flush_pending_cell_tails(&mut self) {
    // Drain buffer; emit Z-order fill atrás → items rebased
    // por cursor_top → stroke à frente; cursor_y avança.
}
```

```rust
// 01_core/src/rules/layout/grid.rs (cell overflow arm)
if cell_overflow {
    let row_track = &row_tracks[placed.row % row_tracks.len()];
    let is_fixed_row = matches!(row_track, TrackSizing::Fixed(_));
    if is_fixed_row {
        // P248 preservado para Fixed rows.
        push FrameItem::Group { clip_mask: Some(Rect), .. };
    } else {
        // P251 — γ-Items.
        let threshold = body_y + body_h;
        let (head, tail) = slice_frame_items_at_height(
            translated_items, threshold,
        );
        for item in head { push directo; }
        if !tail.is_empty() {
            self.pending_cell_tails.push(DeferredCellTail {
                items: tail, origin_x: body_x, width: body_w,
                fill: effective_fill.copied(),
                stroke: effective_stroke.cloned(),
                forwarded_count: 0,
            });
        }
    }
}
```

---

## §4 Citação ADR-0082 PROPOSTO N=1 → N=2 (segunda aplicação citante)

P251 é **segunda aplicação concreta citante** ADR-0082 PROPOSTO
(criada P249; P250 N=1 primeira). Os 4 critérios operacionais
verificados:

1. **Storage prévio** ✓ — TableCell.body já armazenado P157B
   (scope-out original "ignorados em layout" graded); semantic
   actual P248 "clip implícito" não é variant novo.
2. **Consumer Layouter pre-promoção graded** ✓ — P248 "clip
   implícito" é graded (`FrameItem::Group { clip_mask: Some(Rect),
   .. }`); não é semantic real "row break vertical cross-page".
3. **Paridade vanilla referência empírica** ✓ — audit C1 §2.1
   P251 confirmou pos.y locality; γ-Items viável magnitude L
   face γ-Content L+.
4. **Backward compat literal** ✓ — cells sem overflow + cells em
   rows `TrackSizing::Fixed` preservam P248 clip implícito
   bit-equivalente (sentinelas
   `p251_cell_sem_overflow_preserva_p248_output_literal` +
   `p251_table_cell_overflow_row_fixed_preserva_p248_clip`).

**Validação ADR-0082 N=2 citante** — segundo passo dum sequente
candidato N=3 para promoção EM VIGOR. **P252 candidato**: A.4
Boxed stroke-overhang (XS isolado) → N=3 citante → **promoção
ADR-0082 → EM VIGOR humana possível**.

---

## §5 Limitações conscientes γ-Items (per ADR-0054 graded)

Documentadas formalmente para clarificar divergências γ-Items vs
γ-Content:

1. **Items atómicos não dividem mid-item**: Group/Shape com
   bounds grandes que começam abaixo de threshold vão completos
   para tail (paridade vanilla "atomic block can't split
   mid-paragraph"). Refino futuro γ-Content (re-layout body)
   permitiria slice mid-paragraph — fora de scope P251.
2. **Tail cell ocupa toda a largura da row** na nova página
   (cell_x + width preservados). Outras cells da row original
   **não continuam** (paridade vanilla cell-level mas não
   row-level perfeito).
3. **row_heights da próxima página são só do tail cell** —
   não há tracking de "row N continua". Próxima página começa
   fresh com cell tail emit + flow continua.
4. **Recursive overflow limitar 3 iterações** (tail forwarding
   `forwarded_count >= 3` descartado silenciosamente; mitigação
   loop infinito; paridade vanilla heurística).
5. **Stroke + fill re-emit per fragment** com bounds = tail extent
   (visualmente "duas células separadas"; paridade vanilla "split
   block draws two borders").

**Marco C.2 parcial cell-level activada P251**. Multi-region
completo (column flow DEBT-56) continua diferido NÃO-reservado
per política P158.

---

## §6 Critério aceitação P251 (C6+C7)

| Critério | Esperado | Real |
|----------|----------|------|
| `cargo build --workspace` | verde | ✓ verde |
| `cargo test --workspace` | 2276 → ~2291-2301 verdes | ✓ **2294 verdes** (+18) |
| `crystalline-lint .` | 0 violations | ✓ 0 violations |
| `crystalline-lint --fix-hashes` | 1 hash propagado | ✓ 1 hash (`region.md` → `6eec928d`) |
| Content variants | 62 preservado | ✓ 62 |
| ShapeKind variants | 5 preservado | ✓ 5 |
| Block / Boxed / TableCell fields | preservados | ✓ preservados |
| Layouter fields | preservado ou +1 | ✓ **+1** (`pending_cell_tails`) |
| Layouter methods | preservado ou +1 | ✓ **+1** (`flush_pending_cell_tails`) |
| Layouter struct local | +1 | ✓ **+1** (`DeferredCellTail`) |
| Layouter helper module | +1 | ✓ **+1** (`layout/slicing.rs` ~270 LoC com 10 unit tests) |
| Regions fields | 4 preservado | ✓ 4 |
| Stdlib funcs | 64 preservado | ✓ 64 |
| Cobertura Layout per metodologia | ~96-97% → ~97-98% | ✓ +1pp refino qualitativo |
| Cobertura user-facing total | ~75-76% preservado | ✓ preservado |
| Scope-outs Block originais P156G fechados | 10/10 preservado | ✓ preservado (Block A.4 COMPLETO) |
| Scope-outs Boxed originais P156H fechados | 5/6 preservado | ✓ 5/6 |
| Promoções reais scope-outs ADR-0054 cumulativas granular | 12 → 13 | ✓ **13** (P251 ×1) |
| ADR-0079 Categoria C.2 | anotação P251 parcial cell-level | ✓ |
| ADR-0080 sub-categoria | "Slice frame items at height" N=1 inaugurada | ✓ |
| ADR-0061 §"Refino futuro" | anotação P251 | ✓ |
| ADR-0054 §"Promoções reais" | cumulativo granular N=13 | ✓ |
| **ADR-0082** | §"Aplicações citantes" N=1 → **N=2** (segunda citante) | ✓ |
| DEBT-30/34c/34e/56 | sentinelas preservadas | ✓ |
| L0 hashes propagados | 1 | ✓ 1 (`region.md` → `6eec928d`) |
| Adaptações pre-existentes | N=2-8 estimadas | ✓ **N=0** (backward compat literal estrita) |
| Regressões reais | 0 mandatório | ✓ 0 |
| Patterns emergentes | 4 cumulativos esperados | ✓ todos |
| `P251.div-N` | possíveis 4 cenários | ✓ nenhum activado |

**3 pré-condições obrigatórias verificadas**:

1. **Tests baseline preservados**: 2276 verdes pré-P251 →
   **2294 verdes** pós-P251 (+18 P251; 0 regressões; **0
   adaptações** — sentinelas P248 com Fixed rows preservadas).
2. **Comemo memoization invariants ADR-0073/0074 preservados** —
   P251 toca Layouter consumer apenas; Introspector trait
   intocada.
3. **Backward compat literal**: cells sem overflow + cells em
   rows Fixed preservam output P248 bit-equivalente; só cells
   Auto/Fraction com overflow ganham semantic nova.

**Promoções ADR**:
- ADR-0079 Categoria C.2 anotação **parcial cell-level activada**
  P251 + sub-passo 11 cumulativo P227-P251.
- ADR-0080 sub-categoria nova "Slice frame items at height" N=1
  inaugurada + sub-categoria "DeferredX buffer + flush" N=1 → 2
  cumulativo + lição refinada N=14 cumulativo.
- ADR-0061 §"Refino futuro" anotação P251.
- ADR-0054 §"Promoções reais" cumulativo granular N=13 (P251 ×1).
- **ADR-0082 §"Aplicações citantes" N=1 → N=2** (segunda citante
  explícita; status PROPOSTO preservado).
- **Sem novas ADRs criadas**.

---

## §7 Patterns emergentes inaugurados/consolidados P251 (4)

- **"Slice frame items at height via filter + rebase pos.y"** N=1
  inaugurado P251 — pattern novo (γ-Items split via threshold;
  função pura reusável). Candidato a formalização N=3-4 futuro
  (hipóteses: column flow DEBT-56 multi-region; pagination
  overflow generic).
- **"DeferredX buffer + flush em new_page"** N=1 → **N=2
  cumulativo P251** (P245 floats + P251 cell tails). Paridade
  arquitectural directa; pattern emergente consolidado.
- **"Aplicação citante ADR-0082 PROPOSTO"** N=1 → **N=2
  cumulativo P251** (P250 N=1; P251 N=2). N=3 candidato P252.
- **"Spec C1 audit obrigatório bloqueante"** N=13 → **N=14
  cumulativo** P251 (lição refinada: "audit C1 deve confirmar
  localidade pos.y antes de fixar abordagem γ-Items vs γ-Content").

**Anti-inflação 43ª aplicação cumulativa** pós-P205D — Opção β
L0 minimal (`region.md` hash propagado `6eec928d`) + Opção α
helper novo em ficheiro próprio (`layout/slicing.rs`; paridade
subpadrão "tipo entity em ficheiro próprio") + Opção α activação
consumer real (refino directo P248 clip implícito → row break
real) + Opção α reuso DeferredFloat pattern P245 (paridade
arquitectural) + Opção α anotação cumulativa minimal ADRs (0061
+ 0079 + 0080 + 0054 + **0082 citação segunda**) + Opção α
sub-padrão N=1 inaugurado "slice frame items at height" + Opção
α DEBT-34e preservado aberto distinção explícita.

---

## §8 Próximo sub-passo pós-P251 — Categoria C.2 parcial activada

P251 fecha TableCell row break real cell-level via γ-Items.
Restantes pendentes:

| Caminho | Trabalho | Magnitude | Prioridade |
|---------|----------|-----------|------------|
| **A.4 Boxed stroke-overhang** | Único scope-out P156H restante (cita ADR-0082 N=2 → 3 → **promoção EM VIGOR humana possível**) | XS | **alta** (Boxed A.4 completo 6/6; triggera promoção ADR-0082 → EM VIGOR) |
| **ADR-0079 → IMPLEMENTADO graded** | Categoria C.2 cumprida parcialmente P251; Categoria A.4 quase completa | XS-S | alta se humano decide fechamento |
| **ADR-0082 → EM VIGOR** | Decisão humana pós-N=3 citantes (P250+P251+P252 candidato) | XS | **alta** após P252 |
| **DEBT-34e abrir P-passo** | Refactor placement Grid completo (colspan/rowspan real) | L+ | baixa (não-reservado P158) |
| **Pivot outro módulo** | Visualize 54%; Text 52%; Model 50% | varia | baixa-média (Layout muito reforçado) |
| **Cell tail forwarding limit refino** | Limit `max 3 iter` para configurable / robust | XS | baixa |

**Recomendação subjectiva pós-P251**: **A.4 Boxed stroke-overhang**
(XS isolado) — **terceira aplicação citante ADR-0082** N=2 → 3;
fecha Boxed A.4 completo 6/6; **triggera promoção ADR-0082 → EM
VIGOR humana possível** (paridade ADR-0065 P156K validada via
P156J/P157A/P157B sequente). Magnitude controlada XS.

Alternativa: **ADR-0079 → IMPLEMENTADO graded** (XS-S) —
fechamento administrativo Fase 5 Layout agora que A.4 Block
COMPLETO + A.4 Boxed 5/6 + A.4 TableCell row break + C.2
parcial. Patamar conceptual sólido para fechamento administrativo.

**Decisão humana fica em aberto literal** pós-P251.

**Estado pós-P251**:
- Tests workspace: 2276 → **2294 verdes** (+18 P251).
- Content variants: **62 preservado**.
- Block fields: **14 preservado** (P250 final).
- Boxed fields: **10 preservado**.
- TableCell fields: **5 preservado** (P157B final).
- ShapeKind variants: **5 preservado**.
- Layouter fields: **+1** (`pending_cell_tails`).
- Layouter methods: **+1** (`flush_pending_cell_tails`).
- Layouter struct local: **+1** (`DeferredCellTail`).
- Layouter modules: **+1** (`layout/slicing.rs`).
- Regions fields: **4 preservado**.
- Stdlib funcs: **64 preservado**.
- §A.5 distribuição: refino qualitativo (footnote ⁶⁸ P251 —
  `table_cell` reclassificado parcial⁺).
- Cobertura Layout per metodologia: **~96-97% → ~97-98%**.
- Cobertura user-facing total: **~75-76% preservado**.
- **ADRs distribuição preservada literal**: PROPOSTO 13; EM VIGOR
  29; IMPLEMENTADO 23; total **69 preservado**. Anotações
  cumulativas 0061+0079+0080+0054+**0082 §"Aplicações citantes"
  N=2**.
- **Saldo DEBTs: 11 preservado** (DEBT-30/34c/34e/56 sentinelas
  preservadas; sem reabertura; sem novo DEBT).
- **43 aplicações cumulativas anti-inflação** pós-P205D.
- **Patterns emergentes pós-P251** (4):
  - "Slice frame items at height" N=1 inaugurado.
  - "DeferredX buffer + flush em new_page" N=1 → **N=2
    cumulativo** (P245 floats + P251 cell tails).
  - "Aplicação citante ADR-0082 PROPOSTO" N=1 → **N=2
    cumulativo**.
  - "Spec C1 audit obrigatório bloqueante" N=13 → **N=14
    cumulativo**.
- "Promoção real scope-out ADR-0054 graded" granular N=12 →
  **N=13 cumulativo** (P251 ×1).
- **Scope-outs originais Block fechados**: 10/10 preservado
  (Block A.4 COMPLETO).
- **Scope-outs originais Boxed fechados**: 5/6 preservado
  (P252 candidato XS fecha 6/6).
- **Categoria A.4 Fase 5 Layout**: muito reforçada cumulativa.
- **Categoria C.2 Fase 5 Layout**: **parcialmente cumprida P251**
  (cell-level row break real); multi-region completo continua
  diferido.
- **Marco interno**: TableCell row break real cell-level γ-Items
  materializado; segunda aplicação citante ADR-0082; sub-padrão
  N=1 "slice frame items at height" inaugurado; subpadrão
  "DeferredX buffer + flush em new_page" N=2 cumulativo;
  Categoria C.2 Fase 5 Layout parcialmente activada; lição C1
  audit N=14 cumulativa refinada procedimentalmente; primeiro
  passo onde audit C1 fixa abordagem γ-Items vs γ-Content
  empíricamente baseado em pos.y locality.
