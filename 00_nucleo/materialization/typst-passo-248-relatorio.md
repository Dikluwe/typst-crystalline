# Relatório do passo P248 — A.4 breakable + Boxed.height overflow + TableCell overflow activação real (agregado L; 3 promoções graded → real cumulativas)

**Data**: 2026-05-14.
**Spec**: `00_nucleo/materialization/typst-passo-248.md`.
**Tipo**: refactor consumer Layouter — activação semantic real
de 3 fields/comportamentos graded armazenados em passos prévios
(P156G `breakable`; P156H `height` overflow; P157B
`TableCell.body` overflow) via mecanismo comum medição
antecipada (`measure_content_constrained` puro pré-existente).
**Magnitude planeada**: L (~5-8h). **Magnitude real**: **M
(~2-3h)** — audit C1 revelou helper `measure_content_constrained`
já puro (zero refactor) + mecanismo page break existente
reusável sem alterações + algoritmos preliminares §3 directos
(sem necessidade de paridade vanilla detalhada §2.5 — caminho
empírico suficiente).
**Marco**: continuação materialização pós-P247 atributos
visuais; **inaugura sub-padrão "Activação semantic real
multi-consumer via mecanismo comum"** N=1 (P247 inaugurou
"agregar promoções cosméticos visuais" N=1 — relacionado mas
distinto: P247 = cosméticos visuais ortogonais aditivos; P248 =
semantic real multi-consumer com interacção via medição
antecipada partilhada); **promoção graded → real semantic
activação consumer** N=1 → **N=2 cumulativo** (P245 Place float
real N=1; P248 N=2 agregado granular = N=4); 11ª aplicação
cumulativa pattern "spec C1 audit obrigatório bloqueante pós-
P236.div-1" N=10 → 11 cumulativo.

---

## §1 O que foi feito

P248 materializa Categoria A.4 cumulativa via 3 activações
graded → real semantic em agregação. Mecanismo comum:
`measure_content_constrained` puro pré-existente (audit C1
§2.4 confirmado).

**Trabalho real**:

1. **Activação A — Block.breakable** (Layouter `mod.rs` Block
   arm ≈ linha 1330):
   - `breakable: _` → `breakable: *breakable` (consumer real).
   - Medição antecipada via `measure_content_constrained`
     quando `!breakable`.
   - `new_page()` antecipado se `block_total_h > remaining_h`
     E `block_total_h <= page_usable_h`.
   - Overlong (`> page_usable_h`) emit normal (paridade vanilla).
2. **Activação B — Boxed.height** overflow (Layouter Boxed arm
   ≈ linha 1215):
   - `clip: _` → `clip` real lido.
   - Quando `height: Some(h)` + `clip: true`: medição body via
     `measure_content_constrained`; se `body_h_real > h_pt`,
     wrap items emitidos em FrameItem::Group com clip_mask Rect
     altura `h_pt` (reuso mecanismo P242).
   - Quando `clip: false`: emit normal (overflow visível
     paridade vanilla).
   - `height: None` preservado P156H literal.
3. **Activação C — TableCell.body** overflow clip implícito
   (Layouter `grid.rs` cell arm ≈ linha 376):
   - `_cell_h_measured` → `cell_h_measured` (consumer real).
   - Detecção overflow: `cell_h_measured > body_h`.
   - Quando overflow: wrap translated_items em FrameItem::Group
     com clip_mask Rect, pos=(body_x, body_y), inner_(w,h)=
     (body_w, body_h).
   - Quando sem overflow: push directo preservado P157B.
4. **L0 `entities/content.md` extensão** documentando 3
   activações + sub-padrões emergentes; hash propagado
   automaticamente via `crystalline-lint --fix-hashes` →
   `9f03e1a8`.
5. **26 tests novos** (range +25-35 paridade L):
   - 16 unit/E2E em `layout/tests.rs` (Activação A + B + C +
     cross-attribute).
   - 4 unit stdlib `native_block`/`native_box` propagação
     breakable/height/clip a variant.
   - 6 cross-attribute (breakable + outset; height + radius;
     cell overflow + radius+clip; clip diff true vs false; etc).
6. **N=0 adaptações** em tests pré-existentes — defaults
   preservados literais (backward compat estrita).
7. **ADRs anotadas cumulativas**: 0061 §"Refino futuro" + 0079
   §"Anotação cumulativa P248" + 0080 §"Lição refinada P248"
   N=10 → 11 cumulativo.

**2229 → 2255 verdes** (+26 P248; 0 regressões; **0 adaptações**).
**Sem `P248.div-N`** — audit converge com Decisões 1+2+3
+ measure_content_constrained puro + cell layout sem mecanismo
embrionário.

---

## §2 Auditoria pré-P248 OBRIGATÓRIA BLOQUEANTE (C1) — lição N=10 → 11 cumulativo

**Audit empírico** (lição refinada P247 N=10 → P248 N=11
cumulativo: "mapear pontos de check overflow existentes antes
de adicionar novos checks duplicados"):

| Aspecto | Hipótese Spec | Realidade Empírica | Implicação |
|---|---|---|---|
| Mecanismo page break actual | `cursor.rs:127` + 9 sítios | ✓ Confirmado | P248 acrescenta checks antecipados sem substituir |
| `breakable` leituras em Layouter | zero (`breakable: _`) | ✓ Confirmado | Trabalho material 100% pendente Activação A |
| `regions.cell.height` consumers | `placement.rs` (Place anchoring) | ✓ Confirmado | Activação C precisa adicionar consumer cell layout |
| `measure_content_constrained` puridade | hipotetizou puro | ✓ Confirmado `&self`, sem side-effects | Reuso directo sem refactor |
| Cell layout mecanismo embrionário overflow | hipotetizou ausente | ✓ Confirmado ausente | Activação C primeira vez |
| Tests baseline pré-P248 | 2229 verdes | ✓ Confirmado | Baseline para +25-35 |

**Conclusão audit C1**: trabalho real ~30 LoC L1 + ~700 LoC tests +
~140 LoC L0 docs. Magnitude real **M (~2-3h)** face L (~5-8h)
hipotetizado. **Cenário "tudo limpo"**: mecanismo comum reusável
directo, sem refactor, sem divergências formais.

**Sem `P248.div-N`** — audit converge com spec completo; helpers
puros; nenhuma divergência arquitectural identificada.

---

## §3 Activação A — Block.breakable (C2)

```rust
// 01_core/src/rules/layout/mod.rs (Block arm; ≈ linha 1330)
Content::Block { body, width, height, inset, breakable,
                  outset, radius, clip, fill, stroke } => {
    // ... existing setup ...
    if self.regions.current.cursor_x.0 > self.regions.current.line_start_x.0 {
        self.flush_line();
    }

    // P248 — medição antecipada quando !breakable.
    if !*breakable {
        let avail_w = match width {
            Some(w) => w.resolve_pt(font),
            None    => self.available_width(),
        };
        let (_, body_h) = self.measure_content_constrained(body, avail_w);
        let height_min = height.map(|h| h.resolve_pt(font)).unwrap_or(0.0);
        let inner_h = body_h.max(height_min);
        let block_total_h = outset_top + inset_top + inner_h
                           + inset_bottom + outset_bottom;
        let page_usable_h = self.available_height();
        let remaining_h = self.page_bottom_limit()
                        - self.regions.current.cursor_y.0;
        if block_total_h <= page_usable_h && block_total_h > remaining_h {
            self.new_page();
        }
        // else: cabe na actual OU overlong (emit normal — paridade vanilla).
    }
    // ... rest of arm: outset + inset + body + height min + outset_bottom + Shape ...
}
```

3 cenários:
- `block_total_h <= remaining_h` → cabe na actual; emit normal.
- `block_total_h > remaining_h && <= page_usable_h` → break
  antecipado; emit na nova página.
- `block_total_h > page_usable_h` → overlong; emit normal
  (paridade vanilla "overlong atómico não quebra").

---

## §4 Activação B — Boxed.height overflow (C3)

```rust
// 01_core/src/rules/layout/mod.rs (Boxed arm; ≈ linha 1215)
Content::Boxed { body, width, height, inset, baseline, outset,
                  radius, clip, fill, stroke } => {
    // ... existing setup + outset_left advance ...
    let body_items_before = self.regions.current.current_items.len();
    self.layout_content(body);
    // ... restore width + inset_right + outset_right ...

    // P248 — height overflow clip.
    if let Some(h) = height {
        if *clip {
            let h_pt = h.resolve_pt(font);
            let avail_w_box = match width {
                Some(w) => w.resolve_pt(font),
                None    => self.available_width(),
            };
            let (body_w_real, body_h_real) =
                self.measure_content_constrained(body, avail_w_box);
            if body_h_real > h_pt {
                let body_items: Vec<FrameItem> = self.regions.current
                    .current_items.drain(body_items_before..).collect();
                let pos_box = Point { x: ..., y: ... };
                self.regions.current.current_items.push(FrameItem::Group {
                    pos: pos_box,
                    matrix: TransformMatrix::identity(),
                    clip_mask: Some(ShapeKind::Rect),
                    inner_width: body_w_real,
                    inner_height: h_pt,
                    items: body_items,
                });
            }
        }
    }
    // ... rest of arm: Shape inline ...
}
```

3 cenários:
- `height: None` → preservado P156H literal.
- `height: Some(h)` + body cabe → preservado literal.
- `height: Some(h)` + body excede + `clip: true` → Group com
  clip_mask Rect altura h.
- `height: Some(h)` + body excede + `clip: false` → emit normal
  (overflow visível).

---

## §5 Activação C — TableCell overflow (C4)

```rust
// 01_core/src/rules/layout/grid.rs (cell arm; ≈ linha 376)
let (cell_h_measured, cell_items) =
    self.layout_sub_frame_with_width(cell, body_x, body_w);
// ...
let cell_overflow = cell_h_measured > body_h;
// ...
let translated_items: Vec<FrameItem> = cell_items.into_iter()
    .map(|item| {
        let (lx, ly) = item_pos(&item);
        let abs_pos = Point { x: Pt(lx), y: Pt(body_y + (ly - local_start_y)) };
        translate_frame_item(item, abs_pos.x, abs_pos.y)
    })
    .collect();
if cell_overflow {
    self.regions.current.current_items.push(FrameItem::Group {
        pos: Point { x: Pt(body_x), y: Pt(body_y) },
        matrix: TransformMatrix::identity(),
        clip_mask: Some(ShapeKind::Rect),
        inner_width: body_w,
        inner_height: body_h,
        items: translated_items,
    });
} else {
    for item in translated_items { self.regions.current.current_items.push(item); }
}
```

- Cell body cabe em `body_h = cell_h - inset_t - inset_b`
  (populado via `regions.cell.height` P246) → preservado P157B
  literal.
- Cell body excede → clip implícito Group + clip_mask Rect.
- Row break real diferido per Decisão 3 (refino futuro; DEBT-34e
  preservado aberto distinto).

---

## §6 Critério aceitação P248 (C6+C7)

| Critério | Esperado | Real |
|----------|----------|------|
| `cargo build --workspace` | verde | ✓ verde |
| `cargo test --workspace` | 2229 → ~2254-2264 verdes | ✓ **2255 verdes** (+26) |
| `crystalline-lint .` | 0 violations | ✓ 0 violations |
| `crystalline-lint --fix-hashes` | 0-1 hash propagado | ✓ 1 hash (`content.md` → `9f03e1a8`) |
| Content variants | 62 preservado | ✓ 62 |
| ShapeKind variants | 5 preservado | ✓ 5 |
| Block fields | 10 preservado (P247 final) | ✓ 10 |
| Boxed fields | 10 preservado | ✓ 10 |
| TableCell fields | 5 preservado (P157B final) | ✓ 5 |
| Layouter fields | preservado | ✓ |
| Regions fields | 4 preservado | ✓ 4 |
| Stdlib funcs | 64 preservado | ✓ 64 |
| Cobertura Layout per metodologia | ~94-95% → ~95-96% | ✓ +1pp refino qualitativo |
| Cobertura user-facing total | ~75-76% preservado | ✓ preservado |
| Promoções graded → real cumulativas (P245+P248) | N=1 → N=2 | ✓ N=2 agregado; granular N=4 |
| Promoções reais scope-outs ADR-0054 cumulativas granular | 5 → ~8-9 | ✓ **8** (P242×2 + P247×3 + P248×3) |
| ADR-0079 Categoria A.4 | anotação cumulativa P248 | ✓ |
| ADR-0080 sub-categoria | "Activação semantic real multi-consumer via mecanismo comum" N=1 inaugurada | ✓ |
| ADR-0061 §"Refino futuro" | anotação P248 | ✓ |
| DEBT-34c | ENCERRADO preservado P83 | ✓ preservado |
| DEBT-34e | EM ABERTO preservado | ✓ preservado aberto |
| DEBT-30 | ENCERRADO preservado P79 | ✓ preservado |
| L0 hashes propagados | 0-1 | ✓ 1 (`content.md`) |
| Adaptações pre-existentes | N=0-5 estimadas | ✓ **N=0** (defaults preservados literal) |
| Regressões reais | 0 mandatório | ✓ 0 |
| Patterns emergentes | "Agregar multi-consumer mecanismo comum" N=1; "Promoção graded → real" N=1 → 2 cumulativo; "Spec C1 audit" N=10 → 11 cumulativo | ✓ todos |

**3 pré-condições obrigatórias verificadas**:

1. **Tests baseline preservados**: 2229 verdes pré-P248 →
   **2255 verdes** pós-P248 (+26 P248; 0 regressões; **0
   adaptações** — defaults literais preservados).
2. **Comemo memoization invariants ADR-0073/0074 preservados** —
   P248 toca Layouter consumer apenas; entities + trait
   Introspector intocadas.
3. **Backward compat literal**: Block com `breakable: true`
   (default) + Boxed com `height: None` + TableCell sem
   overflow renderizam idênticos a P247 (tests
   `p248_block_breakable_true_preserva_emit_normal`,
   `p248_boxed_height_none_preserva_p156h`,
   `p248_table_cell_sem_overflow_preserva_p157b` validam).

**Promoções ADR**:
- ADR-0079 Categoria A.4 cumulativa P242+P246+P247+P248.
- ADR-0080 sub-categoria nova "Activação semantic real
  multi-consumer via mecanismo comum" N=1 inaugurada + lição
  refinada N=11 cumulativo + sub-padrão "promoção graded → real"
  N=2 cumulativo.
- ADR-0061 §"Refino futuro" anotação P248.
- ADR-0054 §"Promoções reais" granular cumulativo N=8 (limiar
  ADR meta reforçado N≥6 patamar sólido).
- **Sem novas ADRs criadas**.
- Distribuição ADRs preservada literal: PROPOSTO 12; EM VIGOR
  29; IMPLEMENTADO 23; total **68 preservado**.

---

## §7 Patterns emergentes inaugurados/consolidados P248 (4)

- **"Agregar promoções graded → real multi-consumer via mecanismo
  comum" N=1 inaugurado P248** — sub-padrão novo (3
  sub-activações com mecanismo partilhado
  `measure_content_constrained`; magnitude L controlada não L+
  porque mecanismo comum reduz custo per-activação; distinto
  N=1 P247 "agregar cosméticos visuais ortogonais"). Candidato a
  formalização N=3-4 futuro.
- **"Promoção graded → real semantic activação consumer" N=1
  → N=2 cumulativo P248** (P245 Place float = N=1; **P248
  agregado = N=2**; granular = N=4 contando 3 sub-activações
  P248 + 1 P245).
- **"Spec C1 audit obrigatório bloqueante pós-P236.div-1"** N=10
  → **N=11 cumulativo** P248. Lição refinada N=11: "mapear
  pontos de check overflow existentes antes de adicionar novos
  checks duplicados".
- **"Promoção real scope-out ADR-0054 graded"** granular N=5 →
  **N=8 cumulativo P248** (radius + clip + outset + fill + stroke
  + breakable + height + cell_overflow). Limiar conceptual sólido
  para ADR meta candidata futura XS admin (N≥6 patamar atingido
  P248).

**Anti-inflação 40ª aplicação cumulativa** pós-P205D — Opção β
L0 minimal (content.md hash propagado `9f03e1a8`) + Opção α
activação consumer real (3 sub-activações cumulativas) + Opção
α reuso `measure_content_constrained` (helper existente puro)
+ Opção α reuso `FrameItem::Group + clip_mask` (P242 mecanismo)
+ Opção α anotação cumulativa minimal ADRs (0061+0079+0080+0054)
+ Opção α sub-padrão N=1 inaugurado anotado + Opção α DEBT-34e
preservado aberto sem reabertura (relação cumulativa anotada).

---

## §8 Próximo sub-passo pós-P248

P248 fecha 3 promoções graded → real semantic. Restantes
pendentes:

| Caminho | Trabalho | Magnitude | Prioridade |
|---------|----------|-----------|------------|
| **ADR meta admin XS** — "promoções reais scope-outs" N≥8 | Formalizar pattern cumulativo (limiar sólido pós-P248) | XS | **alta** (patamar conceptual sólido atingido) |
| **A.4 Block 4 scope-outs restantes** | spacing + above + below + sticky (paridade P247 agregada) | S-M | média |
| **A.4 Boxed 1 scope-out restante** | stroke-overhang | XS | baixa |
| **A.4 TableCell row break real** | Activação row break (refino P248 clip implícito) | M-L | baixa-média |
| **ADR-0079 → IMPLEMENTADO graded** | Scope-out humano C.2 | XS-S | alta se humano decide fechamento |
| **DEBT-34e abrir P-passo** | Refactor placement Grid completo (colspan/rowspan real) | L+ | baixa (não-reservado P158) |
| **Pivot outro módulo** | Visualize 54%; Text 52%; Model 50% | varia | baixa |

**Recomendação subjectiva pós-P248**: **ADR meta admin XS**
"promoções reais scope-outs" — N=8 cumulativo granular atinge
limiar sólido para formalização. Patamar conceptual claro;
paridade "passo administrativo XS" N=6 (limiar P244). Magnitude
XS controlada.

Alternativa: **A.4 Block 4 scope-outs restantes** (spacing +
above + below + sticky, S-M agregado) — paridade P247 agregação
fecharia Block.A.4 completo (10/10 scope-outs originais P156G).

**Decisão humana fica em aberto literal** pós-P248.

**Estado pós-P248**:
- Tests workspace: 2229 → **2255 verdes** (+26 P248).
- Content variants: **62 preservado**.
- Block fields: **10 preservado**.
- Boxed fields: **10 preservado**.
- TableCell fields: **5 preservado**.
- ShapeKind variants: **5 preservado**.
- Layouter fields: preservado.
- Regions fields: **4 preservado**.
- Stdlib funcs: **64 preservado**.
- §A.5 distribuição: refino qualitativo (footnote ⁶⁶ acrescentada).
- Cobertura Layout per metodologia: **~94-95% → ~95-96%**
  (+1pp refino qualitativo).
- Cobertura user-facing total: **~75-76% preservado**.
- **ADRs distribuição preservada literal**: PROPOSTO 12; EM VIGOR
  29; IMPLEMENTADO 23; total **68 preservado**. Anotações
  cumulativas 0061+0079+0080+0054.
- **Saldo DEBTs: 11 preservado** (DEBT-34c+DEBT-34e+DEBT-30
  sentinelas preservadas; sem reabertura; sem novo DEBT).
- **40 aplicações cumulativas anti-inflação** pós-P205D.
- **Patterns emergentes pós-P248** (4):
  - "Agregar promoções graded → real multi-consumer via
    mecanismo comum" N=1 inaugurado P248.
  - "Promoção graded → real semantic activação consumer" N=1
    → **N=2 cumulativo** (P245 + P248 agregado).
  - "Spec C1 audit obrigatório bloqueante" N=10 → **11
    cumulativo**.
  - "Promoção real scope-out ADR-0054 graded" granular **N=8
    cumulativo** (P242 ×2 + P247 ×3 + P248 ×3).
- **Scope-outs originais Block fechados cumulativamente**:
  5/9 → **6/9** (+breakable real); restam 3 (spacing+above+
  below+sticky agrupados).
- **Boxed.height** semantic real activada cumulativamente.
- **TableCell** overflow Y clip implícito activado (row break
  é refino futuro).
- **Categoria A Fase 5 Layout**: A.4 muito reforçada cumulativa.
- **Categoria C.1 Fase 5 Layout**: cumprida P245.
- **Categoria C.2 Fase 5 Layout**: parcial (cell overflow clip
  implícito P248; row break real pendente).
- **Marco interno**: 3 semantic real activações cumulativas
  num passo único; sub-padrão N=1 "Activação semantic real
  multi-consumer via mecanismo comum" inaugurado; promoção
  graded → real atinge N=2 cumulativo; lição C1 audit N=11
  cumulativa refinada; mecanismo comum medição antecipada
  validado primeira vez em escala multi-consumer; **0
  adaptações em tests pré-existentes** — primeira sub-padrão
  cumulativo onde defaults literais preservados estritamente
  (cumprimento pleno backward compat literal).
