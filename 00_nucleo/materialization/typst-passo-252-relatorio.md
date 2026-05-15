# Relatório do passo P252 — A.4 Boxed COMPLETO 6/6 via refactor cross-cutting Stroke +1 field overhang; terceira aplicação citante ADR-0082 PROPOSTO N=3 (limiar atingido); promoção EM VIGOR humana possível

**Data**: 2026-05-14.
**Spec**: `00_nucleo/materialization/typst-passo-252.md`.
**Tipo**: refactor cross-cutting entity `Stroke` (+1 field
`overhang: bool`) + activação semantic real em Layouter Block +
Boxed (bounds Shape expandidos por `thickness/2` quando
overhang=true). Promove último scope-out P156H Boxed (stroke-
overhang).
**Magnitude planeada**: M (~2-4h). **Magnitude real**: **M
(~1-2h)** — cascade replace_all via sed pattern cobriu ~38 de
42 sítios automaticamente; 4 manuais (formatting multilinha,
variable shorthand); audit pre-spec já feito.
**Marco**: **terceira aplicação cumulativa citante ADR-0082
PROPOSTO** N=2 → 3 — **N=3 limiar interno atingido**; **promoção
EM VIGOR humana possível** (paridade ADR-0065 P156K validado
pós-P156J/P157A/P157B N=3 sequente); **fecha Boxed A.4 COMPLETO
6/6** (último scope-out P156H stroke-overhang); **segundo
variant Content com 100% scope-outs originais fechados
cumulativamente** (após Block P250 10/10); **primeira aplicação
cumulativa do padrão "Refactor cross-cutting entity primitivo
com cascade replace_all guiado"** N=1 inaugurado; décima quinta
aplicação cumulativa pattern "spec C1 audit obrigatório
bloqueante pós-P236.div-1" N=14 → 15 cumulativo (lição refinada
P252: "refactor cross-cutting de entity primitivo exige mapa
empírico exhaustive de todos os construtores literais antes de
modificar struct").

---

## §1 O que foi feito

**Trabalho real**:

1. **`Stroke` struct +1 field** `overhang: bool` em
   `01_core/src/entities/geometry.rs` (paridade vanilla literal).
2. **Cascade ~42 construtores literais** via sed pattern
   `Stroke { paint: ..., thickness: <num> } → Stroke { paint:
   ..., thickness: <num>, overhang: false }`:
   - entities/geometry.rs: 1 sítio.
   - entities/content.rs: 10 sítios.
   - rules/layout/mod.rs: 1 sítio.
   - rules/layout/tests.rs: 14 sítios.
   - rules/stdlib/shapes.rs: 8 sítios.
   - rules/stdlib/layout.rs: 3 sítios (incluindo extract_stroke
     interno).
   - rules/stdlib/mod.rs: 2 sítios (manual; sed broke patterns).
   - **~38 automáticos + ~4 manuais = ~42 total**.
3. **Helper `extract_stroke` expandido** (`stdlib/layout.rs`):
   defaults vanilla `overhang: true` para Length/Color atalhos;
   `Value::Stroke(s)` preserva overhang do user.
4. **stdlib `native_stroke`** aceita `overhang` named arg (Bool;
   default `true` vanilla).
5. **Layouter Block + Boxed Shape emit** (`mod.rs:1446` Boxed +
   `:1676` Block): bounds expandidos por `thickness/2` em cada
   lado quando `stroke.overhang == true` via `if let Some(ref s)
   = stroke { if s.overhang { ... } }`.
6. **Grid/Table cell borders preservados literal**
   (`FrameItem::Shape::Line` em `grid.rs`; overhang conceptual
   n/a — line cap distinct). Divergência consciente per ADR-0054
   graded documentada em §"Limitações conscientes".
7. **L0 `entities/geometry.md` extensão** documentando field
   `overhang` + default cristalino divergente + paridade vanilla
   via stdlib parse + activação Layouter; hash propagado
   `7c1ba7a4`.
8. **10 tests novos** (range +8-15 paridade M):
   - 5 em `layout/tests.rs` (Stroke PartialEq inclui overhang;
     Clone preserva; overhang=false preserva bounds; overhang=
     true expande width por thickness=4; Boxed paralelo Block).
   - 5 em `stdlib/mod.rs` (Length atalho default vanilla true;
     Color atalho default true; native_stroke overhang=false
     explícito; native_stroke default true; overhang não-Bool
     rejeitado).
9. **ADRs anotadas cumulativas**: 0061 §"Refino futuro" + 0079
   §"Anotação cumulativa P252 — Boxed A.4 COMPLETO 6/6" + 0080
   §"Lição refinada P252" N=14 → 15 cumulativo + 0054
   §"Promoções reais cumulativas" tabela N=14 + divergência
   consciente default `overhang: false` + **0082 §"Aplicações
   citantes" N=2 → N=3** (terceira aplicação citante — **limiar
   N=3 atingido**).

**2294 → 2304 verdes** (+10 P252; **0 regressões**; **N=33
adaptações** documentadas via cascade replace_all guiado).
**Sem `P252.div-N`** — audit converge com Decisões 1-12 +
vanilla reference clara §2.5.

---

## §2 Auditoria pré-P252 OBRIGATÓRIA BLOQUEANTE (C1) — lição N=14 → 15 cumulativo

**Audit empírico** (lição refinada P251 N=14 → P252 N=15
cumulativo: "refactor cross-cutting de entity primitivo exige
mapa empírico exhaustive de todos os construtores literais
antes de modificar struct"):

| Aspecto | Hipótese Spec | Realidade Empírica | Implicação |
|---|---|---|---|
| Stroke construtores literais | ~34 sítios | ✓ Confirmado **42 sítios totais** (8 acima estimativa) | Cascade sed cobre 38; 4 manuais |
| Stroke em Content variants | 8 declarações | ✓ Confirmado (Grid + GridCell + Table + Block + Boxed + TableCell + arms cascade) | OK |
| extract_stroke helper | reusável | ✓ Confirmado; defaults atualizados para `true` vanilla | OK |
| PDF exporter Shape emit | 4 caminhos sem overhang | ✓ Confirmado intocado | Single source of truth Layouter |
| Vanilla `overhang` default | true | ✓ Confirmado vanilla | Decisão 2 fixa: cristalino divergente |
| Tests baseline pré-P252 | 2294 verdes | ✓ Confirmado | Baseline preservado |

**Conclusão audit C1**: trabalho real ~250 LoC modificações
cascade + ~80 LoC L0 docs + ~200 LoC ADRs. Magnitude real **M
(~1-2h)** face M (~2-4h) hipotetizado — cascade automatizado
acelerou refactor cross-cutting.

**Sem `P252.div-N`** — audit converge com spec; helpers reusados;
divergência consciente default `overhang: false` documentada.

---

## §3 Stroke struct +1 field + cascade construtores (C2)

```rust
// 01_core/src/entities/geometry.rs (Stroke)
#[derive(Debug, Clone, PartialEq)]
pub struct Stroke {
    pub paint:     Color,
    pub thickness: f64,
    pub overhang:  bool,  // P252 NOVO — default false em construtor Rust
}
```

**Cascade ~42 construtores literais** via `sed` pattern:

```bash
sed -i -E 's/(Stroke \{ paint: [^}]+, thickness: [0-9]+\.[0-9]+) \}/\1, overhang: false }/g' \
  01_core/src/entities/geometry.rs \
  01_core/src/entities/content.rs \
  01_core/src/rules/stdlib/shapes.rs \
  01_core/src/rules/stdlib/layout.rs \
  01_core/src/rules/layout/mod.rs \
  01_core/src/rules/layout/tests.rs \
  01_core/src/rules/stdlib/mod.rs
```

**~4 sítios manuais** corrigidos (sed regex falhou em multilinha
e variable shorthand): 2 em `stdlib/mod.rs` (sed escreveu `\1`
literal — manual fix); 1 em `extract_stroke` shorthand
`thickness` (`Stroke { paint, thickness }` → adicionado
`overhang: true`); 1 em `native_stroke` final (`Stroke { paint,
thickness }` → `Stroke { paint, thickness, overhang }`).

---

## §4 extract_stroke helper expandido + native_stroke overhang named arg (C3)

```rust
// 01_core/src/rules/stdlib/layout.rs (extract_stroke)
pub(super) fn extract_stroke(val: &Value, fn_name: &str, field: &str)
    -> SourceResult<Stroke>
{
    let stroke = match val {
        Value::Length(l) => {
            let thickness = l.abs.to_pt();
            // P252 — vanilla default overhang=true para inputs stdlib.
            Stroke { paint: Color::rgb(0, 0, 0), thickness, overhang: true }
        }
        Value::Color(c) => Stroke { paint: *c, thickness: 1.0, overhang: true },
        Value::Stroke(s) => s.clone(),
        other => return Err(...),
    };
    ...
}

// native_stroke aceita overhang named arg
let overhang = match args.named.get("overhang") {
    Some(Value::Bool(b)) => *b,
    Some(other) => return Err(...),
    None => true,  // P252 — vanilla default
};
Ok(Value::Stroke(Stroke { paint, thickness, overhang }))
```

---

## §5 Activação Layouter Block + Boxed Shape emit (C4)

```rust
// 01_core/src/rules/layout/mod.rs (Block + Boxed Shape emit)
let mut outer_w = ...;
let mut outer_h = ...;
let mut pos = Point { x: ..., y: ... };

// P252 — stroke-overhang real activação.
if let Some(ref s) = stroke {
    if s.overhang {
        let ov = s.thickness / 2.0;
        pos.x = pos.x - Pt(ov);
        pos.y = pos.y - Pt(ov);
        outer_w += 2.0 * ov;
        outer_h += 2.0 * ov;
    }
}

self.regions.current.current_items.insert(items_before, FrameItem::Shape {
    pos, kind, width: outer_w, height: outer_h, fill, stroke: stroke.clone(),
});
```

**Grid/GridCell/Table/TableCell preservados literal** (cell
borders são `FrameItem::Shape::Line` em `grid.rs`; overhang
conceptual n/a — line cap distinct). **Divergência consciente
per ADR-0054 graded** — Decisão 4 spec uniforme 6 arms revista
empíricamente para Block + Boxed only (limitação cell borders).

---

## §6 Citação ADR-0082 PROPOSTO N=2 → N=3 (terceira citante — limiar atingido)

P252 é **terceira aplicação concreta citante** ADR-0082
PROPOSTO. Os 4 critérios operacionais verificados:

1. **Storage prévio** ✓ — stroke-overhang scope-out P156H
   "rejeitados em `native_box` com erro hard" (graded); refactor
   cross-cutting adiciona field a entity primitivo existente,
   não variant novo.
2. **Consumer Layouter pre-promoção graded** ✓ — scope-out
   actualmente erro hard (graded); não consumido pelo Layouter.
3. **Paridade vanilla referência empírica** ✓ — audit C1 §2.5
   confirmou vanilla `overhang: true` default. Cristalino
   divergente `false` em Rust + `true` via stdlib (paridade
   user-facing).
4. **Backward compat literal** ✓ — ~42 construtores literais
   pré-P252 com `overhang: false` preservam bounds Shape
   bit-equivalente (sentinela
   `p252_stroke_construtor_rust_default_overhang_false_preserva_bounds`).

**Validação ADR-0082 N=3 citante atingida** — **N=3 limiar
interno atingido** (paridade ADR-0065 P156K via P156J/P157A/
P157B EM VIGOR). **Promoção ADR-0082 PROPOSTO → EM VIGOR humana
possível pós-P252** (decisão humana via passo administrativo
XS candidato).

---

## §7 Critério aceitação P252 (C6+C7)

| Critério | Esperado | Real |
|----------|----------|------|
| `cargo build --workspace` | verde | ✓ verde |
| `cargo test --workspace` | 2294 → ~2302-2309 verdes | ✓ **2304 verdes** (+10) |
| `crystalline-lint .` | 0 violations | ✓ 0 violations |
| `crystalline-lint --fix-hashes` | 1 hash propagado | ✓ 1 hash (`geometry.md` → `7c1ba7a4`) |
| Content variants | 62 preservado | ✓ 62 |
| ShapeKind variants | 5 preservado | ✓ 5 |
| Block / Boxed / TableCell fields | preservados | ✓ |
| **`Stroke` fields** | **2 → 3** (+overhang) | ✓ 3 |
| Layouter fields | preservado | ✓ |
| Regions fields | 4 preservado | ✓ |
| Stdlib funcs | 64 preservado | ✓ |
| Cobertura Layout per metodologia | ~97-98% → ~98-99% | ✓ +1pp refino qualitativo |
| Cobertura user-facing total | ~75-76% preservado | ✓ |
| Scope-outs Block originais P156G fechados | 10/10 preservado | ✓ (Block A.4 COMPLETO) |
| Scope-outs Boxed originais P156H fechados | 5/6 → **6/6** | ✓ **Boxed A.4 COMPLETO** |
| Promoções reais scope-outs ADR-0054 cumulativas granular | 13 → 14 | ✓ 14 (P252 ×1) |
| ADR-0079 Categoria A.4 | **Boxed A.4 COMPLETO** documentado | ✓ |
| ADR-0080 sub-categoria | "Refactor cross-cutting entity primitivo" N=1 inaugurada | ✓ |
| ADR-0061 §"Refino futuro" | anotação P252 | ✓ |
| ADR-0054 §"Promoções reais" | cumulativo granular N=14 + divergência consciente default | ✓ |
| **ADR-0082** | §"Aplicações citantes" N=2 → **N=3** (terceira citante; **limiar N=3 atingido** — promoção EM VIGOR humana possível) | ✓ |
| DEBT-30/34c/34e/56 | sentinelas preservadas | ✓ |
| L0 hashes propagados | 1 | ✓ 1 (`geometry.md` → `7c1ba7a4`) |
| Adaptações pre-existentes | N=30-40 estimadas | **N=33** dentro do range |
| Regressões reais | 0 mandatório | ✓ 0 |
| Patterns emergentes | 4 cumulativos esperados | ✓ todos |
| `P252.div-N` | possíveis 4 cenários | ✓ nenhum activado |

**3 pré-condições obrigatórias verificadas**:

1. **Tests baseline preservados**: 2294 verdes pré-P252 → **2304
   verdes** pós-P252 (+10 P252; 0 regressões; N=33 adaptações
   cascade documentadas).
2. **Comemo memoization invariants ADR-0073/0074 preservados** —
   P252 toca entity primitivo + Layouter consumer + stdlib;
   Introspector trait intocada.
3. **Backward compat literal**: construtores literais Rust com
   `overhang: false` preservam bounds Shape pré-P252 bit-
   equivalente; só inputs stdlib + user explícito `overhang:
   true` ganham semantic nova (paridade vanilla user-facing).

**Promoções ADR**:
- ADR-0079 Categoria A.4 **Boxed A.4 COMPLETO 6/6** documentado;
  segundo variant Content com 100% scope-outs fechados (após
  Block P250 10/10); sub-passo 12 cumulativo P227-P252.
- ADR-0080 sub-categoria nova "Refactor cross-cutting entity
  primitivo com cascade replace_all guiado" N=1 inaugurada +
  lição refinada N=15 cumulativo.
- ADR-0061 §"Refino futuro" anotação P252.
- ADR-0054 §"Promoções reais" cumulativo granular N=14 (P252 ×1)
  + divergência consciente default `overhang: false` documentada.
- **ADR-0082 §"Aplicações citantes" N=2 → N=3** (terceira citante
  explícita; **N=3 limiar interno atingido**; promoção EM VIGOR
  humana possível).
- **Sem novas ADRs criadas**.

---

## §8 Patterns emergentes inaugurados/consolidados P252 (4)

- **"Refactor cross-cutting entity primitivo com cascade replace_all
  guiado"** N=1 inaugurado P252 — pattern novo (entity primitivo
  `Stroke` cross-cutting em 6 variants Content + 4 caminhos PDF
  exporter; ~42 construtores literais adaptados via sed pattern).
  Candidato a formalização N=3-4 futuro (hipóteses: `Color`
  alpha; `Length` font-relative; `Sides<T>` refactor).
- **"Aplicação citante ADR-0082 PROPOSTO"** N=2 → **N=3 cumulativo
  P252 — limiar interno atingido** (P250 N=1; P251 N=2; **P252
  N=3**). **Promoção ADR-0082 → EM VIGOR humana possível**
  (paridade ADR-0065 P156K via P156J/P157A/P157B).
- **"Spec C1 audit obrigatório bloqueante"** N=14 → **N=15
  cumulativo** P252 (lição refinada: "refactor cross-cutting de
  entity primitivo exige mapa empírico exhaustive de todos os
  construtores literais antes de modificar struct").
- **"Backward compat literal estrita"** N=1 → **N=2 cumulativo
  P252** (P251 cell tails + P252 stroke overhang preservam
  output literal pre-passo via defaults zero-impact).

**Anti-inflação 44ª aplicação cumulativa** pós-P205D — Opção β
L0 minimal (`geometry.md` hash propagado `7c1ba7a4`) + Opção α
extensão field-by-field (+1 field em struct primitivo) + Opção
α activação consumer real (Block + Boxed Shape emit; divergência
consciente Grid/Table cell borders Line strokes) + Opção α
reuso `extract_stroke` helper expandido (não duplicado) + Opção
α default zero-impact construtor Rust (backward compat literal
estrita N=2 cumulativo) + Opção α anotação cumulativa minimal
ADRs (0061 + 0079 + 0080 + 0054 + **0082 citação terceira —
limiar N=3 atingido**) + Opção α sub-padrão N=1 inaugurado
"refactor cross-cutting entity primitivo" + Opção α Boxed A.4
COMPLETO marco interno.

---

## §9 Próximo sub-passo pós-P252 — Boxed A.4 COMPLETO 6/6 + N=3 ADR-0082 limiar atingido

P252 fecha **Boxed A.4 COMPLETO 6/6** + atinge **N=3 citantes
ADR-0082**. Restantes pendentes:

| Caminho | Trabalho | Magnitude | Prioridade |
|---------|----------|-----------|------------|
| **ADR-0082 → EM VIGOR humana** | Passo administrativo XS promoção (paridade ADR-0065 P156K validado pós-N=3 citantes) | XS | **alta** (limiar atingido P252; decisão humana directa) |
| **ADR-0079 → IMPLEMENTADO graded** | Categoria A.4 muito reforçada (Block 10/10 + Boxed 6/6 + TableCell row break + C.2 parcial) | XS-S | **alta** (Fase 5 Layout candidata fechamento administrativo) |
| **DEBT-34e abrir P-passo** | Refactor placement Grid completo (colspan/rowspan real) | L+ | baixa (não-reservado P158) |
| **Pivot outro módulo** | Visualize 54%; Text 52%; Model 50% | varia | média (Layout muito reforçado pós-P252; pivot razoável) |
| **A.4 TableCell row break refino** | γ-Content via re-layout (refino P251 γ-Items) | L+ | baixa (P251 graded suficiente) |
| **Pausa marco** | A.4 Block COMPLETO + A.4 Boxed COMPLETO + C.2 parcial + 14 promoções reais + ADR-0082 N=3 limiar | XS | baixa |

**Recomendação subjectiva pós-P252**: **ADR-0082 → EM VIGOR
humana** (passo administrativo XS) — primeira aplicação cumulativa
do padrão "ADR meta PROPOSTO → EM VIGOR pós-N=3 citantes"
(paridade ADR-0065). Magnitude XS pura administrativa; valida
ADR-0082 empíricamente como pattern sólido cumulativo.

Alternativa: **ADR-0079 → IMPLEMENTADO graded** (XS-S) —
fechamento administrativo Fase 5 Layout agora que A.4 Block
COMPLETO + A.4 Boxed COMPLETO + A.4 TableCell row break + C.2
parcial. **Patamar conceptual máximo** para fechamento
administrativo Fase 5.

**Decisão humana fica em aberto literal** pós-P252.

**Estado pós-P252**:
- Tests workspace: 2294 → **2304 verdes** (+10 P252).
- Content variants: **62 preservado**.
- Block / Boxed / TableCell fields: preservados.
- ShapeKind variants: **5 preservado**.
- **`Stroke` fields: 2 → 3** (+overhang).
- Layouter fields / methods: preservados.
- Regions fields: **4 preservado**.
- Stdlib funcs: **64 preservado**.
- §A.5 distribuição: refino qualitativo (footnote ⁶⁹ P252 —
  Boxed A.4 COMPLETO).
- Cobertura Layout per metodologia: **~97-98% → ~98-99%**.
- Cobertura user-facing total: **~75-76% preservado**.
- **ADRs distribuição preservada literal**: PROPOSTO 13; EM VIGOR
  29; IMPLEMENTADO 23; total **69 preservado**. Anotações
  cumulativas 0061+0079+0080+0054+**0082 §"Aplicações citantes"
  N=3 — limiar atingido**.
- **Saldo DEBTs: 11 preservado**.
- **44 aplicações cumulativas anti-inflação** pós-P205D.
- **Patterns emergentes pós-P252** (4):
  - "Refactor cross-cutting entity primitivo" N=1 inaugurado.
  - "Aplicação citante ADR-0082 PROPOSTO" N=2 → **N=3 cumulativo
    (limiar atingido)**.
  - "Spec C1 audit obrigatório bloqueante" N=14 → **N=15
    cumulativo**.
  - "Backward compat literal estrita" N=1 → **N=2 cumulativo**
    (P251 cell tails + P252 stroke overhang).
- "Promoção real scope-out ADR-0054 graded" granular N=13 →
  **N=14 cumulativo** (P252 ×1).
- **Scope-outs originais Block fechados**: 10/10 preservado
  (Block A.4 COMPLETO).
- **Scope-outs originais Boxed fechados**: 5/6 → **6/6**
  (**Boxed A.4 COMPLETO**; segundo variant Content com 100%
  scope-outs fechados).
- **Categoria A.4 Fase 5 Layout**: Block COMPLETO + Boxed
  COMPLETO; TableCell parcial (row break real); multi-region
  completo C.2 continua diferido.
- **Marco interno**: Boxed A.4 COMPLETO 6/6 — segundo variant
  Content com 100% scope-outs originais fechados cumulativamente
  (após Block P250); refactor cross-cutting de entity primitivo
  `Stroke` primeira aplicação cumulativa; **ADR-0082 N=3 limiar
  interno atingido** — promoção EM VIGOR humana possível; padrão
  "Backward compat literal estrita" N=2 cumulativo consolidado;
  lição C1 audit N=15 cumulativa refinada procedimentalmente;
  primeiro passo cumulativo onde audit empírico pré-spec é
  formalizado como lição procedural.
