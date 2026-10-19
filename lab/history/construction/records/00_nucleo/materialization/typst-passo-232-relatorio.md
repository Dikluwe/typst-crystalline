# Relatório do passo P232 — A.5 Place per-cell alignment override (Fase 5 Categoria A 5/5; **FECHA Categoria A**; terceira aplicação automática ADR-0080 EM VIGOR)

**Data**: 2026-05-13.
**Spec**: `00_nucleo/materialization/typst-passo-232.md`.
**Tipo**: refino algorítmico puro — **zero fields novos** em
Place/Grid/Table/Cell; +1 field `cell_align` no Layouter;
lógica precedência per-eixo via `.or()` no arm Place.
**Magnitude planeada**: S+ (~1h). **Magnitude real**: S (~45min).
**Marco**: **fecho Categoria A 5/5 estructuralmente** (sem
transição ADR-0079 status); **3 patterns emergentes
novos/consolidados** (N=1 → 2 precedência `.or()`; N=1
inaugurado fecho categoria sem transição; N=1 inaugurado
sub-passo sem novos fields); **terceira aplicação automática
ADR-0080 EM VIGOR**.

---

## §1 O que foi feito

P232 materializa A.5 Place per-cell alignment override:
- **Zero fields novos** Place/Grid/Table/Cell.
- **+1 field `cell_align: Option<Align2D>`** no `Layouter`
  struct (paridade `cell_origin_*` baseline P84.6;
  save/restore ao entrar/sair Grid context em layout_grid).
- **Lógica precedência per-eixo via `.or()`** no arm
  `Content::Place`: `effective_h = alignment.h.or(cell_align.h)`;
  `effective_v` idem.
- **Stdlib `native_place` NÃO modificado** (sintaxe utilizador
  preservada literal).
- **L0 NÃO tocado** — terceira aplicação automática ADR-0080
  EM VIGOR pós-promoção P229.
- 5 tests novos (5 E2E layout precedência); workspace **2101 →
  2106 verdes** (+5); 0 adaptações intencionais; 0
  regressões reais; 0 violations.
- **Categoria A Fase 5 Layout 5/5 ✓ FECHADA estructuralmente**.

---

## §2 Inventário pré-P232 + audit Table.align + arm Place (C1)

**Audit empírico**:
- `Content::Place` 7 fields baseline P84.5/P84.6+P223 ✓.
- `Align2D { h: Option<HAlign>, v: Option<VAlign> }` struct +
  Option vazio baseline P84.5 ✓.
- `Grid.align: Option<Align2D>` baseline P224.A ✓.
- **`Content::Table.align` NÃO existe baseline** — Table 5
  fields (columns/rows/children/stroke/fill); sem align
  field. **P232 escopo limitado a Grid context**; Table align
  paralelo refino XS futuro candidato (sem `P232.div-N`
  formal — decisão pragmática aceite).
- Arm `Content::Place` em `layout/mod.rs:871` baseline com
  destructure 7 fields; chama `self.layout_place(...)`.
- `cell_origin_x/y/w` baseline em `Layouter` linhas 158-160;
  `cell_align` **NÃO existe** → P232 adiciona.
- `cell_origin_*` save/restore pattern confirmado em
  `grid.rs:232+`.

Sem `P232.div-N` formal — divergências aceites como decisões
pragmáticas.

---

## §3 Refactor arm Place + Layouter +cell_align (C2+C3)

**Layouter struct** (`layout/mod.rs:160`):
```rust
pub(super) cell_origin_x: Option<f64>,
pub(super) cell_origin_y: Option<f64>,
pub(super) cell_origin_w: Option<f64>,
pub(super) cell_align: Option<Align2D>,  // P232 — NEW
```

`Default` impl (~linha 235):
```rust
cell_align: None,  // P232
```

**`layout_grid` save/restore** (`grid.rs:32+`):
- Signature: `_align: Option<Align2D>` → `align: Option<Align2D>`
  (não-ignorado).
- Entrada do método: `let saved_cell_align = self.cell_align;
  self.cell_align = align;` (linha ~38).
- Saída do método: `self.cell_align = saved_cell_align;`
  (linha ~360, antes do `}` final).
- Scope **Grid-level** (não per-cell — align uniforme
  aplica-se a todas cells do Grid).

**Arm `Content::Place`** em `layout/mod.rs:871`:
```rust
Content::Place { alignment, dx, dy, scope, float: _, clearance: _, body } => {
    // P232 — effective alignment per eixo via `.or()`.
    let effective_alignment = match self.cell_align {
        Some(grid_a) => Align2D {
            h: alignment.h.or(grid_a.h),
            v: alignment.v.or(grid_a.v),
        },
        None => *alignment,  // Place fora Grid: baseline preservado.
    };
    self.layout_place(effective_alignment, *dx, *dy, *scope, body);
}
```

---

## §4 Resolução effective alignment via `.or()` per axis (C2 detalhe)

**Semantic `.or()` per eixo independente**:

| Sintaxe Place | `alignment` valor | Comportamento P232 (com Grid align center+top) |
|---------------|-------------------|------------------------------------------------|
| `place(body)` | `{ h: None, v: None }` | Ambos eixos herdam Grid → center+top |
| `place(top, body)` | `{ h: None, v: Some(Top) }` | V Place override; H herda Grid → center+top |
| `place(center, body)` | `{ h: Some(Center), v: None }` | H Place override (mesmo); V herda Grid → center+top |
| `place(left + bottom, body)` | `{ h: Some(Left), v: Some(Bottom) }` | Ambos override → left+bottom |

**Place fora Grid** (cell_align None): `*alignment` direct;
baseline P84.5 preservado literal.

**Pattern "precedência per-X via `.or()` resolution" N=1 →
2 cumulativo** (P230 GridCell over Grid stroke/fill; **P232
Place per-axis over Grid align**).

---

## §5 Decisões substantivas + terceira aplicação automática ADR-0080 EM VIGOR

**8 decisões fixadas**:
- **Decisão 1** — Opção α lógica precedência (zero fields
  novos; paridade pattern P230 GridCell).
- **Decisão 2** — Opção α precedência por eixo independente
  via `.or()`.
- **Decisão 3** — Opção α inline no arm Place existente
  (refactor mínimo).
- **Decisão 4** — Opção α stdlib `native_place` preservado.
- **Decisão 5** — 5 tests E2E precedência explícitos.
- **Decisão 6** — Audit C1: Table.align ausente → escopo
  Grid only.
- **Decisão 7** — Anotação ADR-0079 Categoria A 5/5 ✓
  fechada sem transição status (pattern "fecho categoria
  completa dentro de ADR PROPOSTO sem transição" N=1
  inaugurado P232).
- **Decisão 8** — Opção γ L0 NÃO tocado automaticamente
  (**terceira aplicação automática ADR-0080 EM VIGOR**).

**ADR-0080 EM VIGOR aplicação automática N=2 → 3 cumulativo**:
- L0 prompts NÃO tocados em P232.
- `crystalline-lint --fix-hashes`: "Nothing to fix" em L0.
- **Terceira aplicação automática pós-promoção P229** (P230
  primeira; P231 segunda; **P232 terceira**).

**Anti-inflação 24ª aplicação cumulativa** pós-P205D.

---

## §6 Resultados verificação (C5+C7)

| Critério | Esperado | Real |
|----------|----------|------|
| `cargo build --workspace` | verde | ✓ verde |
| `cargo test --workspace` | ~2110 verdes | **2106 verdes** (1817+242+24+2+21) ✓ (5 novos vs ~9 spec; subset pragmático — 5 E2E precedência cobre todos cenários canónicos) |
| `crystalline-lint .` | 0 violations | **0 violations** ✓ |
| `crystalline-lint --fix-hashes` | "Nothing to fix" | **"Nothing to fix"** ✓ (L0 não tocado automático N=3) |
| Adaptações pre-existentes | N=0-2 | **N=0** (Place baseline preservado fora Grid; mudança aplica-se só em Grid context com align Some — caso novo) |
| Place fields | 7 preservado | ✓ (zero fields novos) |
| Layouter fields | +1 (cell_align) | ✓ |
| Regressões reais | 0 | **0** |

---

## §7 Inventário 148 footnote ⁵¹ + ADR-0079 Categoria A 5/5 ✓ fechada (C8+C9)

**Inventário 148**:
- §A.5 Layout entradas `place(...)` + `grid(...)`: footnotes
  existing pre-P232 + ⁵¹.
- Footnote ⁵¹ adicionada (~110 linhas) documentando A.5
  materializado + 8 decisões + audit C1 Table.align
  ausente + 4 patterns emergentes consolidados + **Categoria
  A Fase 5 FECHADA ESTRUCTURALMENTE 5/5**.

**ADR-0079**:
- §"Aplicações cumulativas" anotado com bloco
  `### P232 anotação — Categoria A sub-passo 5 (Place
  per-cell alignment override); Categoria A 5/5 ✓ FECHADA
  ESTRUCTURALMENTE`.
- Status ADR-0079 mantido PROPOSTO (5/13-15 sub-passos
  cumulativos; Categoria A 5/5 ✓; B/C/D pendentes).
- **Marco interno implícito Categoria A fechada
  estructuralmente** documentado.

**Sem marco cirúrgico blueprint** — pattern §3.0... para
fechos/aberturas de Fase, não categorias internas dentro
de Fase (anti-inflação preservada).

---

## §8 Próximo sub-passo

P232 fecha Categoria A 5/5 estructuralmente. **Categoria A
não tem mais sub-passos identificados em ADR-0079
§"Próximos passos"**. Decisão humana sobre próxima sessão
(reset decisão pós-fecho categoria):

| Caminho | Trabalho | Magnitude | Prioridade |
|---------|----------|-----------|------------|
| **B.1 DEBT-34d Auto track sizing** | Algorítmico isolado; fecha DEBT-34d preservado per `P224.div-1` | M (~2-3h) | alta (fecha DEBT preservado; algorítmico standalone) |
| **B.2 Consumer geometric** | `place_cells` algorítmico → Layouter geometric integration | M (~2-3h) | média (consolida P224.C algorítmico) |
| **B.3 GridCell align/inset/breakable** | Per-cell algorítmico (precedência paridade P230+P232) | M (~2-3h) | média (cohesão Categoria B) |
| **D.1 state runtime** | Runtime mutable; **desbloqueia ADR-0066 PROPOSTO → IMPLEMENTADO** + Introspection +33pp | M (~2-3h) | **alta** (transição arquitectural maior; promoção ADR significativa) |
| **C.1 Place float real** | Flow contorna (reabre Opção B P219 graded) | L+ (~5-8h) | baixa (reabertura arquitectural maior) |
| **C.2 Multi-region completa** | Reabre P216B + DEBT-56b novo | L+ a XL (~10-20h) | baixa |
| Pivot outro módulo | Visualize 54%; Text 52%; Model 50% | varia | baixa-média |
| ADR meta admin | Promoção patterns N=4 paralelo / N=10 Smart→Option / N=7 semantic adiada | XS (~30min cada) | baixa |

**Recomendação subjectiva**: **B.1 DEBT-34d** (M ~2-3h) —
algorítmico isolado; **fecha DEBT-34d preservado per
`P224.div-1` há ~tempo** (continuidade de honestidade
arquitectural); sem reabrir decisões maiores. Alternativa:
**D.1 state** se humano priorizar promoção ADR-0066
IMPLEMENTADO + bonus Introspection +33pp.

**Decisão humana fica em aberto literal** pós-P232.

**Estado pós-P232**:
- Tests workspace: 2101 → **2106 verdes** (+5 P232).
- Content variants: 59 preservado.
- Value variants: 55 preservado.
- Stdlib funcs: 60 preservado.
- Place fields: 7 preservado (zero novos).
- Layouter fields: +1 `cell_align`.
- §A.5 distribuição: `12/4/2/0/0 = 18` preservada.
- Cobertura Layout per metodologia: **89% preservada**.
- Cobertura user-facing total: 67% preservada.
- ADRs: PROPOSTO 12; EM VIGOR 29 (ADR-0080); IMPLEMENTADO
  21; total 67.
- Saldo DEBTs: 12 preservado.
- **24 aplicações cumulativas anti-inflação** pós-P205D.
- **Pattern "L0 minimal para refactors" aplicação
  automática N=2 → 3 cumulativo** (P230+P231+P232).
- **Pattern "precedência per-X via `.or()` resolution"
  N=1 → 2 cumulativo** (P230 GridCell; P232 Place
  per-axis).
- **Pattern "fecho categoria completa dentro de ADR
  PROPOSTO sem transição" N=1 inaugurado P232**.
- **Pattern "sub-passo sem novos fields; só lógica
  precedence" N=1 inaugurado P232**.
- **Categoria A Fase 5 Layout: 5/5 ✓ FECHADA
  estructuralmente** — próximo sub-passo pivot Categoria
  B/C/D ou outro módulo.
- **Fase 5 Layout candidata: 5/13-15 sub-passos
  materializados** (~33-38% cumulativo; **Categoria A
  100% interna**).
