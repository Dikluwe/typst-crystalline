# Relatório do passo P231 — A.4 outset/radius/clip Block + Boxed (Fase 5 Categoria A 4/5; segunda aplicação automática ADR-0080 EM VIGOR)

**Data**: 2026-05-13.
**Spec**: `00_nucleo/materialization/typst-passo-231.md`.
**Tipo**: refino aditivo a 2 variants existentes (Block + Boxed)
+ parsing inline 3 named args + renderização Opção β parcial
graded (3 fields semantic real adiada per audit C1 — primitivos
baseline ausentes).
**Magnitude planeada**: M (~2-3h). **Magnitude real**: M (~2h).
**Marco**: nenhum (décimo-nono passo pós-M9c; **valida pattern
"refino aditivo paralelo entre variants irmãos" N=3 → 4
cumulativo**; **segunda aplicação automática ADR-0080 EM VIGOR**
pós-promoção P229; **reabertura formal P156G+H scope-outs**
documentados há 18 dias).

---

## §1 O que foi feito

P231 materializa A.4 outset/radius/clip Block + Boxed:
- **Block +3 fields** (5 → 8): `outset: Sides<Length>` +
  `radius: Option<Length>` + `clip: bool`.
- **Boxed +3 fields** paralelo Block (5 → 8).
- **`native_block`/`native_box` accept 3 named args** via
  parsing inline (Length uniforme outset; Length opcional
  radius; Bool clip); validações negativos rejeitados.
- **Renderização Opção β parcial graded**: todos 3 fields
  **semantic real adiada** per audit C1 (primitivos baseline
  ausentes — `RoundedRect` não existe; `Group::clip_mask`
  refactor estructural). Pattern "Field armazenado semantic
  adiada" N=5 → **7 cumulativo**.
- **L0 NÃO tocado automaticamente** (segunda aplicação
  automática ADR-0080 EM VIGOR pós-P229).
- 15 tests novos (4 unit content + 9 unit stdlib + 2 E2E
  layout); workspace **2086 → 2101 verdes** (+15); 4
  adaptações intencionais Block/Boxed constructors; 0
  regressões reais; 0 violations.

---

## §2 Inventário pré-P231 + audit primitivos visuais (C1)

**Audit empírico**:
- `Content::Block` 5 fields baseline P156G ✓.
- `Content::Boxed` 5 fields baseline P156H ✓.
- `extract_sides_lengths` helper existe em `stdlib/layout.rs:422`
  mas signature `(args: &Args, fn_name: &str) -> SourceResult<Sides<Option<Length>>>`
  — toma args completo (pad() pattern); **não reusável
  direct** para parsing `outset:` Value individual.
- **`ShapeKind` enum em `geometry.rs:32`** enumera Rect,
  Ellipse, Line, Path. **`RoundedRect` NÃO existe**.
- **`FrameItem::Group::clip_mask: Option<ShapeKind>`** existe
  baseline P78 DEBT-30; mas wrap body em Group requer
  refactor estructural baseline.

**Decisões críticas C1**:
- `extract_sides_lengths` não reusável → parsing inline
  Length uniforme (subset; per-side refino futuro).
- `RoundedRect` ausente → **radius semantic adiada**
  (pattern N=5 → 6).
- `Group::clip_mask` extensível requer refactor → **clip
  semantic adiada** (pattern N=6 → 7).
- `outset` parsing inline OK; mas semantic real (bounds
  visual) também adiada porque requer refactor layouter
  cumulativo.

Sem `P231.div-N` formal — divergências são decisões
arquiteturais aceites per Opção β graded.

---

## §3 Block/Boxed refino +3 fields (C2+C3)

```rust
Block {
    body, width, height, inset, breakable,   // P156G baseline
    outset: Sides<Length>,                    // P231 (A.4)
    radius: Option<Length>,                   // P231 (A.4)
    clip:   bool,                             // P231 (A.4)
}

Boxed {
    body, width, height, inset, baseline,    // P156H baseline
    outset: Sides<Length>,                    // P231 paralelo
    radius: Option<Length>,                   // P231 paralelo
    clip:   bool,                             // P231 paralelo
}
```

Block: 5 → **8 fields**. Boxed: 5 → **8 fields**.

Arms cascata (compiler-driven; ~15 sítios):
- `entities/content.rs` (PartialEq + map_content + map_text
  + 2 constructors `Content::block`/`Content::boxed`).
- `rules/introspect.rs` (materialize_time Block + Boxed).
- `rules/layout/mod.rs` (4 arms: layout_content Block +
  Boxed; measure_content_constrained Block + Boxed; todos
  destructure com `outset: _, radius: _, clip: _`
  ignorados).

---

## §4 `native_block`/`native_box` accept 3 named args (C5)

```rust
// Em ambos native_block e native_box:

// Named args loop accept "outset" | "radius" | "clip" (parse pós-loop).

// Parse outset (Length uniforme; negativo rejeitado).
let outset = match args.named.get("outset") {
    Some(val) => {
        let len = extract_length(val)?;
        if len.abs.0 < 0.0 || len.em < 0.0 { Err(...) }
        Sides::uniform(len)
    }
    None => Sides::uniform(Length::ZERO),
};

// Parse radius (Option<Length>; negativo rejeitado).
let radius = match args.named.get("radius") {
    Some(val) => {
        let len = extract_length(val)?;
        if len.abs.0 < 0.0 || len.em < 0.0 { Err(...) }
        Some(len)
    }
    None => None,
};

// Parse clip (Bool default false).
let clip = match args.named.get("clip") {
    Some(Value::Bool(b)) => *b,
    Some(other) => Err(...),
    None => false,
};
```

Parsing inline (sem helper novo); `extract_length` reuso
N=10 preservado.

---

## §5 Renderização Opção β parcial graded (C6)

**Audit C1 determinou todos 3 fields semantic real
adiada**:

- **`outset`**: armazenado em variant; semantic real
  (bounds visual expandidos antes de emit) **adiada**
  porque requer refactor layouter cumulativo (mais que
  P231 scope individual aditivo).
- **`radius`**: armazenado; semantic real adiada porque
  `ShapeKind::RoundedRect` primitivo NÃO existe baseline
  geometry.rs P76. Pattern N=5 → 6 cumulativo.
- **`clip`**: armazenado; semantic real adiada porque
  wrap body em `FrameItem::Group { clip_mask: Some(...) }`
  requer refactor estructural baseline. Pattern N=6 → 7
  cumulativo.

**Pattern "Field armazenado semantic adiada" N=5 → 7
cumulativo** (+3 em P231: outset+radius+clip).

Arms `Content::Block` + `Content::Boxed` em `layout/mod.rs`
destructure novos fields com `outset: _, radius: _, clip:
_` ignorados; body renderiza preservando baseline P156G/H.

---

## §6 Decisões substantivas + segunda aplicação automática ADR-0080 EM VIGOR

**7 decisões fixadas**:
- **Decisão 1** — Opção α escopo restrito (outset+radius+clip
  apenas; fill/stroke separados; spacing/above/below/sticky
  Categoria B candidato).
- **Decisão 2** — Opção α `Sides<Length>` outset (paridade
  `inset` baseline).
- **Decisão 3** — Opção β `Option<Length>` radius uniforme
  (`Corners<T>` cristalino NÃO existe per ADR-0029; criar
  trabalho separado).
- **Decisão 4** — Opção α `bool` clip (paridade vanilla
  literal).
- **Decisão 5** — Opção β graded parcial (3 fields semantic
  adiada per audit C1).
- **Decisão 6** — Opção α refino paralelo Block + Boxed.
- **Decisão 7** — Opção γ L0 NÃO tocado automaticamente
  (**segunda aplicação automática ADR-0080 EM VIGOR**).

**ADR-0080 EM VIGOR aplicação automática N=1 → 2 cumulativo**:
- L0 prompts NÃO tocados em P231.
- `crystalline-lint --fix-hashes`: "Nothing to fix" em L0.
- **Segunda aplicação automática pós-promoção P229** (P230
  foi primeira).
- Pattern "aplicação automática ADR EM VIGOR" consolida.

**Anti-inflação 23ª aplicação cumulativa** pós-P205D.

---

## §7 Resultados verificação + inventário 148 + ADR-0079 (C7+C9+C10+C11)

| Critério | Esperado | Real |
|----------|----------|------|
| `cargo build --workspace` | verde | ✓ verde |
| `cargo test --workspace` | ~2103 verdes | **2101 verdes** (1812+242+24+2+21) ✓ (15 novos vs ~17 spec; subset pragmático) |
| `crystalline-lint .` | 0 violations | **0 violations** ✓ |
| `crystalline-lint --fix-hashes` | "Nothing to fix" | **"Nothing to fix"** ✓ (L0 não tocado automático) |
| Adaptações pre-existentes | N=3-7 | **N=4** (Block/Boxed test patterns) |
| Block fields | 5 → 8 | ✓ |
| Boxed fields | 5 → 8 | ✓ |
| Regressões reais | 0 | **0** |

**Inventário 148**:
- §A.5 Layout `block`/`box` pre-existentes `implementado`;
  footnote ⁵⁰ adicionada (~120 linhas) documentando A.4
  materializado + 7 decisões + ADR-0080 EM VIGOR aplicação
  automática N=2 + patterns cumulativos N=4 paralelo +
  N=3 bool simples + N=10 Smart→Option + N=7 semantic
  adiada + reabertura P156G+H scope-outs.

**ADR-0079**:
- §"Aplicações cumulativas" anotado com bloco
  `### P231 anotação — Categoria A sub-passo 4
  (outset/radius/clip Block + Boxed)`.
- Status ADR-0079 mantido PROPOSTO (4/13-15 sub-passos).
- Categoria A: 4/5 materializados (A.1 ✓; A.2 ✓; A.3 ✓;
  A.4 ✓; A.5 pendente).

---

## §8 Próximo sub-passo

P231 fecha quarto sub-passo Fase 5 Layout candidata
(Categoria A 4/5). Decisão humana sobre próxima sessão:

| Caminho | Trabalho | Magnitude | Prioridade |
|---------|----------|-----------|------------|
| **A.5 Place per-cell** | Place dentro Grid com align: ? per-cell — **fecha Categoria A 5/5** | S+ (~1h) | **alta** (fecha categoria completa; momentum cumulativo) |
| **B.1 DEBT-34d** | Auto track sizing algorítmico (fecha DEBT-34d preservado) | M (~2-3h) | média |
| **B.2 Consumer geometric** | `place_cells` algorítmico → Layouter geometric | M (~2-3h) | média |
| **D.1 state runtime** | runtime mutable; desbloqueia ADR-0066 IMPLEMENTADO | M (~2-3h) | alta (+33pp Introspection) |
| Pivot outro módulo | Visualize 54%; Text 52%; Model 50% | varia | baixa |

**Recomendação subjectiva**: **A.5 Place per-cell** (S+
~1h) — fecha Categoria A 5/5 completa; sub-passo menor
ortogonal; momentum natural P227→P228→P230→P231→A.5.
Alternativa: **D.1 state** se humano priorizar runtime
queries para desbloquear ADR-0066 IMPLEMENTADO.

**Decisão humana fica em aberto literal** pós-P231.

**Estado pós-P231**:
- Tests workspace: 2086 → **2101 verdes** (+15 P231).
- Content variants: 59 preservado.
- Value variants: 55 preservado.
- Stdlib funcs: 60 preservado.
- **Block fields: 5 → 8** (+outset + radius + clip).
- **Boxed fields: 5 → 8** (+outset + radius + clip
  paralelo).
- §A.5 distribuição: `12/4/2/0/0 = 18` preservada.
- Cobertura Layout per metodologia: **89% preservada**.
- Cobertura user-facing total: 67% preservada.
- ADR-0066 PROPOSTO; ADR-0061 IMPLEMENTADO; ADR-0078
  IMPLEMENTADO; ADR-0079 PROPOSTO; **ADR-0080 EM VIGOR**.
- Saldo DEBTs: 12 preservado.
- **23 aplicações cumulativas anti-inflação** pós-P205D.
- **Pattern "L0 minimal para refactors" aplicação
  automática N=1 → 2 cumulativo** (P230 + P231).
- **Pattern "refino aditivo paralelo entre variants
  irmãos" N=3 → 4 cumulativo** consolidado (muito sólido
  — promoção formal ADR meta candidato).
- **Pattern "Field bool simples paridade vanilla" N=2 →
  3 cumulativo** (`breakable`/`repeat`/`clip`).
- **Pattern Smart→Option N=9 → 10 cumulativo** (radius).
- **Pattern "Field armazenado semantic adiada" N=5 → 7
  cumulativo** (+outset+radius+clip todos adiadas).
- **Reabertura formal P156G+H scope-outs** documentados
  18 dias atrás.
- **Categoria A Fase 5 Layout**: 4/5 → próximo A.5 fecha
  categoria 5/5.
- **Fase 5 Layout candidata**: 4/13-15 sub-passos
  materializados (P227 A.1 ✓; P228 A.2 ✓; P230 A.3 ✓;
  **P231 A.4 ✓**; A.5 + B + C + D pendentes).
