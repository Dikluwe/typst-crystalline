# Relatório do passo P247 — A.4 fill + stroke + outset activação real em Block + Boxed (agregado M-L; 3 scope-outs originais P156G/H promovidos cumulativamente)

**Data**: 2026-05-14.
**Spec**: `00_nucleo/materialization/typst-passo-247.md`.
**Tipo**: refino aditivo a variants existentes Block + Boxed +
activação consumer Layouter + emissão `FrameItem::Shape`.
Promove 3 scope-outs cosméticos originais P156G/P156H.
**Magnitude planeada**: M-L (~4-6h). **Magnitude real**: **M
(~2-3h)** — audit C1 revelou cenário A (outset zero-uso) +
helpers `extract_stroke` pré-existente reusável directo;
`extract_color` ausente mas pattern inline 1-linha simples.
**Marco**: continuação materialização pós-P246 cell migration;
**fecha cumulativamente 5 dos 9 scope-outs Block originais
P156G** (outset P231→P247 + radius P242 + clip P242 + fill
P247 + stroke P247 = 5/9; restam spacing + above + below +
sticky); **fecha 5 dos 6 scope-outs Boxed P156H** (mesmos 5;
resta stroke-overhang); **promoção real scope-out ADR-0054
graded** N=2 → **N=3 cumulativo** (P242 radius/clip agregados
N=2; **P247 fill + stroke + outset semantic agregados N=3**;
contando granular = 5 promoções reais cumulativas: radius +
clip + outset + fill + stroke); inaugura **"agregar promoções
de scope-outs cosméticos visuais"** sub-padrão **N=1**.

---

## §1 O que foi feito

P247 materializa Categoria A.4 cumulativa fill+stroke+outset
semantic real activação como refino aditivo paralelo Block +
Boxed (sub-padrão "refino aditivo paralelo entre variants
irmãos" N=4 → **N=5 cumulativo** P247).

**Trabalho real**:

1. **+2 fields em Block + Boxed** (paridade simétrica):
   `fill: Option<Color>` + `stroke: Option<Stroke>`. Default
   `None` ambos. **8 → 10 fields ambos variants**.
2. **Cascata 9 arms** em `01_core/src/entities/content.rs`
   (declaração, construtor, is_empty `..` preservado,
   plain_text `..` preservado, PartialEq +2 fields, map_content
   +2 fields, map_text +2 fields).
3. **Layouter activa emissão `FrameItem::Shape`** ANTES do body
   (snapshot-and-insert via `current_items.insert(items_before,
   ...)`):
   - Bounds outer incluem `outset + inset + body` (paridade
     Decisão 5 spec).
   - Shape kind = `Rect` se radius zero; `RoundedRect { radii:
     radius }` se non-zero (reuso P242 ShapeKind).
   - Fill + Stroke passados ao FrameItem.
4. **outset semantic real activado** (cenário A audit §2.4-§2.5
   — outset zero-uso pré-P247): cursor.y avança `outset.top`
   antes do inset.top; `outset.bottom` após height min;
   bounds Shape expandem em todos os lados.
5. **stdlib `block(fill:, stroke:)` + `box(fill:, stroke:)`**:
   - `fill` aceita `Value::Color` directo (pattern inline 1-linha
     paridade Grid/Table P228); tipos inválidos rejeitados.
   - `stroke` reusa helper `extract_stroke` pré-existente P227
     (Length/Color/Stroke shorthands).
6. **L0 `entities/content.md` extensão** documentando 2 fields
   novos + activação Layouter + Decisões 1-9 (hash propagado
   automaticamente via `crystalline-lint --fix-hashes`).
7. **20 tests novos** (range +15-25 paridade M-L):
   - 6 unit content (entities/content.rs): variant aceita
     fill+stroke, partialEq inclui fill+stroke, map_content
     preserva fill+stroke, construtores defaults None.
   - 8 unit stdlib (stdlib/mod.rs): native_block + native_box
     aceitam fill Color + stroke shorthand; defaults None;
     tipos errados rejeitados; combina fill+stroke+radius+clip+
     outset.
   - 6 E2E layout (layout/tests.rs): Block fill emite Shape;
     Block stroke emite Shape com stroke; fill + radius emite
     RoundedRect; outset expande bounds; backward compat
     (fill=stroke=None+outset=ZERO sem Shape); Boxed fill
     emite Shape.
8. **N=12 adaptações** em tests pré-existentes (construtores
   explícitos Block/Boxed em entities/content.rs +
   stdlib/mod.rs + layout/tests.rs + introspect.rs):
   - 4 sítios em `entities/content.rs` (P231 testes).
   - 7 sítios em `layout/tests.rs` (P231/P242/P243).
   - 1 sítio em `introspect.rs` (materialize_time arm Block +
     Boxed).
9. **ADRs anotadas cumulativas**: 0061 §"Refino futuro" + 0079
   §"Anotação cumulativa P247" + 0080 §"Lição refinada P247"
   N=9 → 10 cumulativo.

**2209 → 2229 verdes** (+20 P247; 0 regressões; N=12
adaptações construtores documentadas — dentro do range
N=0-10 §1.4 + 2 adicionais em ficheiros não-antecipados
introspect.rs + layout-internal arm propagação).
**Sem `P247.div-N`** — audit converge com Decisão 1+2 + cenário
A confirmado.

---

## §2 Auditoria pré-P247 OBRIGATÓRIA BLOQUEANTE (C1) — lição N=9 → 10 cumulativo

**Audit empírico** (lição refinada P246 N=9 → P247 N=10
cumulativo: "mapear scope-outs declarados historicamente vs
estado real materializado antes de assumir ausência"):

| Aspecto | Hipótese Spec | Realidade Empírica | Implicação |
|---|---|---|---|
| Block + Boxed 8 fields | confirmado §2.1 | ✓ Confirmado | OK |
| `Stroke { paint: Color, thickness: f64 }` | `geometry.rs:24` | ✓ Confirmado | Reuso directo |
| `Color` enum simplificado | `layout_types.rs` Rgb/Rgba | ✓ Confirmado (Copy) | `*fill` directo OK |
| `Paint` enum existente | hipótese ausente | **Confirmado AUSENTE** | Decisão 1 `Option<Color>` fixa |
| PDF exporter fill+stroke ops | confirmado | ✓ Confirmado | Zero trabalho L3 |
| outset semantic Layouter actual | cenários A/B/C | **Cenário A** (4 arms `outset: _`) | Activação primeira vez P247 |
| `extract_stroke` em stdlib | hipótese existe | **EXISTE** `layout.rs:351` | Reuso directo |
| `extract_color` em stdlib | hipótese existe | **AUSENTE** (pattern inline `Value::Color(c)` 1-linha em Grid/Table) | Sem helper novo; pattern inline 1-linha trivial |
| Tests baseline pré-P247 | 2209 verdes | ✓ Confirmado | Baseline para +15-25 |

**Conclusão audit C1**: trabalho real ~50 LoC L1 + ~120 LoC tests +
~80 LoC L0 docs. Magnitude real M (~2-3h) face M-L (~4-6h)
hipotetizado. **Cenário A confirmado** (outset zero-uso →
activação primeira vez). **Decisão 3 final**: cenário A.

**Sem `P247.div-N`** — audit converge com spec; helpers reusados;
nenhuma divergência arquitectural identificada.

---

## §3 Block + Boxed +2 fields (C2)

```rust
// 01_core/src/entities/content.rs:640 (Block) — P247 +2 fields
Block {
    body, width, height, inset, breakable,
    outset, radius, clip,                                // P231/P242
    fill:   Option<Color>,                               // P247 NOVO
    stroke: Option<Stroke>,                              // P247 NOVO
}

// 01_core/src/entities/content.rs:596 (Boxed) — paridade simétrica
Boxed {
    body, width, height, inset, baseline,
    outset, radius, clip,                                // P231/P242
    fill:   Option<Color>,                               // P247 NOVO
    stroke: Option<Stroke>,                              // P247 NOVO
}
```

**Decisões fixas pós-audit**:

- **Decisão 1**: `fill: Option<Color>` (não `Option<Paint>` —
  Paint enum não existe; Color directo per Stroke já usa).
- **Decisão 2**: `stroke: Option<Stroke>` (reuso struct
  `Stroke { paint: Color, thickness: f64 }`).
- **Decisão 3 final**: cenário A (outset zero-uso pré-P247 →
  activação primeira vez).
- **Decisão 4-5**: Layouter activa Shape antes do body via
  `current_items.insert(items_before, ...)` (Z-order).
- **Decisão 6**: helpers stdlib — `extract_stroke` reuso;
  `extract_color` pattern inline 1-linha (sem helper novo;
  paridade Grid/Table).
- **Decisão 7**: Anti-inflação 39ª — sem novo Content variant;
  sem nova ADR; sem novo entity type.

**Construtores** preservam ergonomia P156G/H (`block(body,
width, height, inset, breakable)` + `boxed(body, width, height,
inset, baseline)`); defaults `fill: None, stroke: None`
internamente.

---

## §4 Layouter activação Shape + outset semantic (C3)

```rust
// 01_core/src/rules/layout/mod.rs (Block arm; ≈ linha 1261)
Content::Block { body, width, height, inset, breakable: _,
                  outset, radius, clip, fill, stroke } => {
    // ... cálculos inset/outset ...
    let has_shape  = fill.is_some() || stroke.is_some();
    let has_outset = outset_left != 0.0 || outset_right != 0.0
                     || outset_top != 0.0 || outset_bottom != 0.0;
    if cursor_x > line_start_x { self.flush_line(); }
    let items_before = self.regions.current.current_items.len();
    let start_y = self.regions.current.cursor_y.0;
    self.regions.current.cursor_y += Pt(outset_top);
    self.regions.current.cursor_y += Pt(inset_top);
    // ... layout body (+clip wrap se applicable) ...
    self.regions.current.cursor_y += Pt(inset_bottom);
    // height min (compara consumed_inner = cursor.y - start_y - outset_top)
    self.regions.current.cursor_y += Pt(outset_bottom);
    if has_shape || has_outset {
        let block_outer_w = match width {
            Some(w) => w.resolve_pt(font) + inset_left,
            None    => saved_width - saved_line_start.0,
        };
        let outer_w = block_outer_w + outset_left + outset_right;
        let outer_h = self.regions.current.cursor_y.0 - start_y;
        let pos = Point { x: saved_line_start - Pt(outset_left), y: Pt(start_y) };
        let shape_kind = if radius_zero { Rect }
                         else { RoundedRect { radii: *radius } };
        self.regions.current.current_items.insert(items_before, FrameItem::Shape {
            pos, kind: shape_kind, width: outer_w, height: outer_h,
            fill: *fill, stroke: stroke.clone(),
        });
    }
}
```

**Boxed arm** (inline; paralelo):
- `start_x` captado ANTES de outset.left avance.
- Cursor.x avança outset.left + inset.left + body + inset.right
  + outset.right.
- Shape height proxy = `line_height` da fonte actual (refino
  futuro medir body altura real para inline).
- Insert Shape em `items_before` paralelo Block.

**Backward compat**: fill=None + stroke=None + outset=ZERO →
`has_shape=false && has_outset=false` → SEM Shape emitido →
output bit-equivalente a P246 (test
`p247_block_fill_none_e_outset_zero_sem_shape` valida).

---

## §5 stdlib native_block + native_box (C4)

```rust
// 01_core/src/rules/stdlib/layout.rs (native_block; ≈ linha 688)
// P247 — aceitar fill/stroke parsing pós-loop:
"outset" | "radius" | "clip" | "fill" | "stroke" => {},

// P247 — fill (Value::Color directo; pattern inline 1-linha):
let fill = match args.named.get("fill") {
    Some(Value::Color(c)) => Some(*c),
    Some(other) => return Err(...),
    None => None,
};

// P247 — stroke (reuso extract_stroke P227):
let stroke = match args.named.get("stroke") {
    Some(val) => Some(extract_stroke(val, "block", "stroke")?),
    None      => None,
};
```

`native_box` paralelo simétrico. Helpers stdlib `extract_stroke`
preserva Length/Color/Stroke shorthands paridade vanilla.

---

## §6 Critério aceitação P247 (C6+C7)

| Critério | Esperado | Real |
|----------|----------|------|
| `cargo build --workspace` | verde | ✓ verde |
| `cargo test --workspace` | 2209 → ~2224-2234 verdes | ✓ **2229 verdes** (+20) |
| `crystalline-lint .` | 0 violations | ✓ 0 violations |
| `crystalline-lint --fix-hashes` | 1 hash propagado | ✓ 1 hash (`content.md` → `28c98b30`) |
| Content variants | 62 preservado | ✓ 62 |
| ShapeKind variants | 5 preservado | ✓ 5 |
| Block fields | 8 → 10 | ✓ 10 |
| Boxed fields | 8 → 10 | ✓ 10 |
| Stdlib funcs | 64 preservado | ✓ 64 |
| Cobertura Layout per metodologia | ~93-94% → ~94-95% | ✓ +1pp refino qualitativo |
| Cobertura user-facing total | ~75-76% preservado | ✓ preservado |
| Scope-outs Block originais P156G | 0/9 → 5/9 fechados | ✓ 5/9 (outset+radius+clip+fill+stroke) |
| Scope-outs Boxed originais P156H | 0/6 → 5/6 fechados | ✓ 5/6 |
| Promoções reais scope-outs cumulativas | 2 (P242) → 5 (P242×2 + P247×3) | ✓ 5 cumulativo granular |
| ADR-0079 Categoria A.4 | anotação cumulativa P247 | ✓ |
| ADR-0080 sub-categoria | "promoção real scope-outs cosméticos" N=3 cumulativo | ✓ |
| ADR-0061 §"Refino futuro" | anotação P247 | ✓ |
| L0 hashes propagados | 1 | ✓ 1 (`content.md`) |
| Adaptações pre-existentes | N=0-10 estimadas | **N=12** (2 acima range; documentadas) |
| Regressões reais | 0 | ✓ 0 |
| Patterns emergentes | "Agregar promoções" N=1 inaugurado; "Spec C1 audit" N=10 cumulativo | ✓ |

**3 pré-condições obrigatórias verificadas**:

1. **Tests baseline preservados**: 2209 verdes pré-P247 →
   **2229 verdes** pós-P247 (+20 P247; 0 regressões; N=12
   adaptações construtores).
2. **Comemo memoization invariants ADR-0073/0074 preservados**
   — P247 toca Layouter consumer + entities + stdlib apenas.
3. **Backward compat**: Block/Boxed com fill=None + stroke=None
   + outset=ZERO renderizam idênticos a P246 (test
   `p247_block_fill_none_e_outset_zero_sem_shape` valida; output
   PDF bit-equivalente para body sem novos atributos).

**Promoções ADR**:
- ADR-0079 Categoria A.4 cumulativa P242+P246+P247.
- ADR-0080 sub-categoria "promoção real scope-outs cosméticos"
  N=2 → **3 cumulativo** + lição refinada N=10 cumulativo.
- ADR-0061 §"Refino futuro" anotação P247.
- ADR-0054 §"Promoções reais" granular N=5 cumulativo (limiar
  conceptual para ADR meta candidata futura XS admin).
- **Sem novas ADRs criadas**.
- Distribuição ADRs preservada literal: PROPOSTO 12; EM VIGOR
  29; IMPLEMENTADO 23; total **68 preservado**.

---

## §7 Patterns emergentes inaugurados/consolidados P247 (3)

- **"Agregar promoções scope-outs cosméticos visuais" N=1
  inaugurado P247** — sub-padrão novo (3 promoções num passo
  único: outset semantic real + fill + stroke; magnitude
  controlada M; coesão semantic forte; tests cross-multiplicados).
  Candidato a formalização N=3-4 futuro.
- **"Spec C1 audit obrigatório bloqueante pós-P236.div-1"** N=9
  → **10 cumulativo** P247. Lição refinada N=10: "mapear
  scope-outs declarados historicamente vs estado real
  materializado antes de assumir ausência" (refino directo
  pattern P243→P244).
- **"Promoção real scope-out ADR-0054 graded"** N=2 → **N=3
  cumulativo P247** (P242 radius+clip agregado N=2; P247
  outset+fill+stroke agregado N=3; granular = 5 promoções reais
  cumulativas).

**Sub-padrão "refino aditivo paralelo entre variants irmãos"
N=4 → N=5 cumulativo P247** (P156D HSpace+VSpace; P157C
TableHeader+TableFooter; P231 outset Block+Boxed; P242
radius+clip Block+Boxed; **P247 fill+stroke Block+Boxed**).

**Anti-inflação 39ª aplicação cumulativa** pós-P205D — Opção β
L0 minimal (content.md hash propagado `28c98b30`) + Opção α
refino aditivo paralelo Block+Boxed (2 fields cada) + Opção α
activação consumer real (outset semantic + Shape emission) +
Opção α reuso helpers stdlib (`extract_stroke` pré-existente
P227) + Opção α anotação cumulativa minimal ADRs (0061+0079+
0080) + Opção α sub-padrão N=1 inaugurado anotado + Opção α
3 scope-outs fechados simultaneamente (paridade "agregar
promoções" pattern N=1).

---

## §8 Próximo sub-passo pós-P247

P247 fecha 5 dos 9 scope-outs Block + 5 dos 6 Boxed. Restantes
pendentes:

| Caminho | Trabalho | Magnitude | Prioridade |
|---------|----------|-----------|------------|
| **A.4 breakable per-cell activação real** | Materializar semantic Block.breakable + Boxed.height overflow + TableCell overflow | M (~2-4h) | **alta** (desbloqueio arquitectural P246 explorado) |
| **A.4 Block 4 scope-outs restantes agregados** | spacing + above + below + sticky (paridade P247 agregada) | S-M | média |
| **A.4 Boxed 1 scope-out restante** | stroke-overhang | XS | baixa |
| ADR meta admin XS — "promoções reais scope-outs" N=5 | Formalizar pattern cumulativo (limiar sólido pós-P247) | XS | média |
| ADR-0079 → IMPLEMENTADO graded | Scope-out humano C.2 | XS-S | alta se humano decide fechamento |
| Pivot outro módulo | Visualize 54%; Text 52%; Model 50% | varia | baixa |

**Recomendação subjectiva pós-P247**: **A.4 breakable per-cell
activação real**. Sequente natural P246 desbloqueio
arquitectural; magnitude M; activa semantic real Block.breakable
+ Boxed.height + TableCell que esperam há muito.

Alternativa: **ADR meta admin XS** formalizar pattern "promoções
reais scope-outs" agora que N=5 atinge limiar sólido (P242 ×2
+ P247 ×3). Patamar conceptual claro para anotação meta.

**Decisão humana fica em aberto literal** pós-P247.

**Estado pós-P247**:
- Tests workspace: 2209 → **2229 verdes** (+20 P247).
- Content variants: **62 preservado**.
- Block fields: **8 → 10**.
- Boxed fields: **8 → 10**.
- ShapeKind variants: **5 preservado**.
- Layouter fields: preservado.
- Regions fields: **4 preservado**.
- Stdlib funcs: **64 preservado**.
- §A.5 distribuição: refino qualitativo (footnotes acrescentadas;
  contagens preservadas).
- Cobertura Layout per metodologia: **~93-94% → ~94-95%**.
- Cobertura user-facing total: **~75-76% preservado**.
- **ADRs distribuição preservada literal**: PROPOSTO 12; EM VIGOR
  29; IMPLEMENTADO 23; total **68 preservado**. Anotações
  cumulativas 0061+0079+0080+0054.
- **Saldo DEBTs: 11 preservado** (DEBT-30 sentinela P79
  preservada; sem reabertura; sem novo DEBT).
- **39 aplicações cumulativas anti-inflação** pós-P205D.
- **Patterns emergentes pós-P247** (3):
  - "Agregar promoções scope-outs cosméticos visuais" N=1
    inaugurado.
  - "Spec C1 audit obrigatório bloqueante" N=9 → **10
    cumulativo**.
  - "Promoção real scope-out ADR-0054 graded" N=2 → **3
    cumulativo** (granular = 5 promoções reais cumulativas).
- **Scope-outs originais Block fechados cumulativamente**:
  0/9 → **5/9** (outset+radius+clip+fill+stroke).
- **Scope-outs originais Boxed fechados**: 0/6 → **5/6**.
- **Categoria A Fase 5 Layout**: A.4 muito reforçada (5 dos 9
  Block + 5 dos 6 Boxed cosméticos fechados).
- **Marco interno**: 3 scope-outs cosméticos visuais P247
  promovidos cumulativamente; agregação em passo único valida
  sub-padrão N=1 "agregar promoções"; lição C1 audit N=10
  cumulativa refinada; primeira aplicação onde 3 scope-outs
  cumulativos em passo único atingem patamar conceptual claro
  para ADR meta candidata futura.
