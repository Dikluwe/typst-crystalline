# Passo 231 — A.4 `outset`/`radius`/`clip` Block + Boxed (Fase 5 Layout candidata Categoria A 4/5; segunda aplicação automática ADR-0080 EM VIGOR)

**Série**: 231 (décimo-sétimo sub-passo Layout pós-M9c;
**quarto sub-passo materialização Fase 5 Layout candidata**
per ADR-0079 PROPOSTO; quarto sub-passo Categoria A
"cosméticos sem reabrir decisões"; **segunda aplicação
automática ADR-0080 EM VIGOR** pós-P230).
**Marco**: nenhum (décimo-nono passo pós-M9c; **valida
pattern "refino aditivo paralelo entre variants irmãos"
N=3 → 4 cumulativo** Block+Boxed P231; pattern "aplicação
automática ADR EM VIGOR" N=1 → 2; **reabre P156G+H
scope-outs documentados há tempo**).
**Tipo**: refino aditivo a 2 variants existentes
(`Content::Block` + `Content::Boxed`); 3 fields novos a
cada variant + renderização parcial graded (Opção β:
outset real + radius/clip armazenado semantic adiada se
primitivos não existem baseline; audit C1 crítico).
**Magnitude**: M (~2-3h).
**Pré-condição**: P230 concluído (A.3 stroke/fill per-cell
GridCell + TableCell + precedência via `.or()`; 2086
verdes; 0 violations; saldo DEBTs 12; ADR-0079 Categoria
A 3/5; primeira aplicação automática ADR-0080 EM VIGOR);
humano fixou A.4 (decisão literal pós-P230 §8);
`Content::Block { body, width, height, inset, breakable }`
baseline P156G; `Content::Boxed { body, width, height,
inset, baseline }` baseline P156H; `Sides<Length>` baseline
P156C; `FrameItem::Shape::Rect` baseline P76; `FrameItem::Group`
baseline P77/P78 (com `clip_mask: Option<ShapeKind>` per
DEBT-30 P78); `Color::rgb(0,0,0)` baseline P25; ADR-0080
EM VIGOR P229 + Categoria A pattern paralelo variants
irmãos N=3 P227+P228+P230.
**Output**: 1 ficheiro relatório curto + código alterado em
~5-7 ficheiros L1 + **L0 NÃO tocado automaticamente** per
ADR-0080 EM VIGOR + inventário 148 anotação cumulativa
(footnote ⁵⁰) + ADR-0079 anotação Categoria A 4/5.

---

## §1 Trabalho

`Content::Block` baseline P156G tem 5 fields (body/width/
height/inset/breakable); §"Atributos scope-out" listou
`outset/fill/stroke/radius/clip/spacing/above/below/sticky`.
`Content::Boxed` baseline P156H tem 5 fields (body/width/
height/inset/baseline); §"Atributos scope-out" listou
`outset/fill/stroke/radius/clip/stroke-overhang`.

P226 diagnóstico Categoria A.4 marcou "Block/Boxed
outset/radius/clip" como sub-passo Fase 5 cosmético
cumulativo a A.1+A.2+A.3.

**P231 materializa A.4**:
- **Block +3 fields**: `outset: Sides<Length>` + `radius:
  Option<Length>` + `clip: bool`.
- **Boxed +3 fields**: `outset: Sides<Length>` + `radius:
  Option<Length>` + `clip: bool` (paralelo Block; pattern
  refino aditivo paralelo entre variants irmãos N=3 → 4
  cumulativo).
- **`native_block` + `native_box` accept 3 named args**
  (parsing `outset` via helper `extract_sides_lengths` se
  já existir P156C-N OU criação se necessário; `radius`
  Length inline; `clip` Bool inline).
- **Renderização Opção β parcial graded** (audit C1):
  - **`outset` real** (bounds visual expandidos; renderização
    trivial).
  - **`radius` armazenado semantic adiada** se primitivo
    `RoundedRect` não existir baseline (pattern N=5 → 6
    cumulativo).
  - **`clip` armazenado semantic adiada** se mask
    `FrameItem::Group::clip_mask` não estender ao body
    automaticamente.

**Decisão arquitectural central — 7 decisões fixadas**:

### Decisão 1 — Escopo P231 Opção α (outset+radius+clip apenas)

P226 diagnóstico Categoria A.4 literal: "Block/Boxed
outset/radius/clip". P231 segue escopo literal:

| Opção | Atributos cobertos | Trade-off |
|-------|---------------------|-----------|
| **α** | outset + radius + clip apenas | Subset minimal; coerente diagnóstico P226 |
| β | + fill + stroke Block/Boxed | Categoria A separada (não cobertos em P226 A.4) |
| γ | Apenas outset | Mais minimal mas viola P226 escopo |

**Decisão fixada — Opção α** (escopo P226 literal).

fill/stroke Block/Boxed ficam para Categoria A separada
futura (não-reservada). `spacing/above/below/sticky` Block
são atributos de fluxo distintos (Categoria B candidato).

### Decisão 2 — Tipo do field `outset` Opção α (Sides<Length>)

3 opções consideradas:

| Opção | Tipo | Trade-off |
|-------|------|-----------|
| **α** | `Sides<Length>` | Paridade `inset` P156G/H baseline; coerente structurally |
| β | `Option<Length>` uniforme | Menos expressivo; ortogonal `inset` baseline |
| γ | `Option<Sides<Length>>` | Inflacionário; `Sides<Length>` já tem default zero |

**Decisão fixada — Opção α** (paridade literal `inset`).

Default `Sides<Length>::default()` (zero). Validação:
negativos rejeitados (paridade `inset` validação P156G/H).

### Decisão 3 — Tipo do field `radius` Opção β (Option<Length> uniforme)

Vanilla `radius` aceita Length (uniforme), Corners<Length>
(per-corner), ou Rel.

3 opções consideradas:

| Opção | Tipo | Trade-off |
|-------|------|-----------|
| α | `Corners<Length>` paridade vanilla | **Corners<T> NÃO existe cristalino** per ADR-0029; criar é trabalho separado |
| **β** | `Option<Length>` uniforme | Subset minimal; per-corner refino futuro |
| γ | `Sides<Length>` (reuso Sides) | Conceptualmente errado (corners ≠ sides) |

**Decisão fixada — Opção β** (`radius: Option<Length>`
uniforme):
- `Corners<T>` cristalino NÃO existe (audit ADR-0029
  confirma).
- Criar `Corners<T>` é trabalho substantivo separado fora
  escopo P231.
- Per-corner é refino futuro candidato (criar `Corners<T>`
  primeiro como pré-requisito).
- Pattern Smart→Option N=9 → **10 cumulativo**.

### Decisão 4 — Tipo do field `clip` Opção α (bool)

Vanilla `clip: bool` (default `false`). Trivial.

**Decisão fixada — Opção α**: `clip: bool` (default
`false`). Paridade vanilla literal. Sem Option (paridade
`breakable: bool` P156G + `repeat: bool` P224.B).

Pattern emergente "Field bool simples paridade vanilla" N=1
→ 2 → 3 cumulativo (`breakable` P156G; `repeat` P224.B;
**clip** P231).

### Decisão 5 — Renderização Opção β parcial graded

3 opções consideradas:

| Opção | Mecânica | Trade-off |
|-------|----------|-----------|
| α | Renderização completa (outset real + radius cantos arredondados + clip mask) | L se primitivos baseline não existem |
| **β** | Parcial graded: outset real; radius/clip armazenado adiada se primitivos ausentes | M; paridade pattern N=5 cumulativo |
| γ | Tudo adiada graded (paridade N=5 puro) | Viola intent A.4 cosméticos visíveis (outset trivial) |

**Decisão fixada — Opção β** (parcial graded condicional
a audit C1):

**Audit C1 crítico**:
1. **`outset`**: renderização real trivial (adiciona ao
   bounds visual antes de emit). Sempre real.
2. **`radius`**: requer `FrameItem::Shape::RoundedRect` ou
   primitivo equivalente. Audit baseline P76 geometry.rs:
   - Se primitivo existe → renderização real.
   - Se NÃO existe → **armazenado semantic adiada**;
     pattern N=5 → 6 cumulativo.
3. **`clip`**: requer `FrameItem::Group` com `clip_mask:
   Some(...)` extensível ao body. Audit baseline P77/P78
   + DEBT-30:
   - Se `Group::clip_mask` aceita extensão automática →
     renderização real.
   - Se NÃO → **armazenado semantic adiada**; pattern N=5
     → 6 ou 7 cumulativo.

**Pattern "Field armazenado semantic adiada" N=5 → ?**:
N=5 baseline (weak P156D + weak P156E + breakable P156G +
float P223 + repeat P224.B). P231 pode adicionar +1 ou +2
(radius e/ou clip se primitivos ausentes).

### Decisão 6 — Refino paralelo Block + Boxed Opção α

Pattern "refino aditivo paralelo entre variants irmãos"
N=3 baseline (Grid+Table P227/P228; GridCell+TableCell
P230). P231 estende a Block+Boxed:

| Opção | Acção | Trade-off |
|-------|-------|-----------|
| **α** | Block + Boxed ambos +3 fields paralelo | Pattern N=3 → 4 cumulativo consolidado |
| β | Apenas Block (escopo "structural"); Boxed adiada | Quebra pattern paralelo |
| γ | Separar P231 + P232 (Block primeiro; Boxed segundo) | Atomização excessiva |

**Decisão fixada — Opção α**:
- Pattern paralelo variants irmãos N=3 → **4 cumulativo**.
- Block (structural) + Boxed (inline) ambos têm os 3
  atributos vanilla (outset/radius/clip são propriedades
  visuais aplicáveis a ambos).
- Magnitude adicional mínima (Boxed paridade Block trivial).

### Decisão 7 — L0 NÃO tocado automaticamente (ADR-0080 EM VIGOR aplicação automática N=2)

**Decisão fixada — aplicação automática ADR-0080 EM VIGOR**:

ADR-0080 EM VIGOR P229 §"Decisão" aplica-se por defeito.
P231 é **segunda aplicação automática pós-promoção P229**
(primeira foi P230). Pattern "aplicação automática ADR EM
VIGOR sem decisão explícita por sub-passo" N=1 → **2
cumulativo**.

L0 prompts NÃO tocados.

Reuso de dados (sem recolha nova):

- `Content::Block { body, width, height, inset, breakable }`
  baseline P156G.
- `Content::Boxed { body, width, height, inset, baseline }`
  baseline P156H.
- `Sides<Length>` baseline P156C com `default()` zero.
- Helper `extract_sides_lengths` provável baseline P156G/H
  (audit C1).
- `extract_length` helper N=10 baseline.
- `FrameItem::Shape::Rect` baseline P76.
- `FrameItem::Group { clip_mask: Option<ShapeKind> }`
  baseline P77/P78 + DEBT-30.
- `native_block` + `native_box` em `stdlib/layout.rs`
  baseline P156G/H.
- Pattern "refino aditivo paralelo entre variants irmãos"
  N=3 cumulativo P227+P228+P230.
- Pattern "Field armazenado semantic adiada" N=5 cumulativo
  (weak/breakable/float/repeat).
- Pattern Smart→Option N=9 cumulativo.
- Pattern "Field bool simples" N=2 baseline.
- ADR-0080 EM VIGOR aplicação automática N=1 baseline P230.

---

## §2 Cláusulas (12 — atomização paridade P230)

### C1 — Inventário pré-P231: confirmar Block/Boxed + Sides helper + audit primitivos visuais

Auditoria empírica crítica:

```
grep -n "Block {" 01_core/src/entities/content.rs
grep -n "Boxed {" 01_core/src/entities/content.rs
grep -n "pub(super) fn extract_sides_lengths\|fn extract_sides" 01_core/src/rules/stdlib/
grep -n "FrameItem::Group\|RoundedRect" 01_core/src/entities/layout_types.rs
grep -n "clip_mask" 01_core/src/entities/layout_types.rs
grep -A 10 "ShapeKind" 01_core/src/entities/geometry.rs
```

Hipótese:
- `Block { body, width, height, inset, breakable }` 5 fields
  baseline P156G.
- `Boxed { body, width, height, inset, baseline }` 5 fields
  baseline P156H.
- `extract_sides_lengths` helper provável baseline (audit
  C1; criado em P156G/H).
- **`ShapeKind::RoundedRect` NÃO existe** (hipótese
  provável; geometry.rs P76 enumera apenas Rect/Ellipse/
  Line).
- **`FrameItem::Group::clip_mask`** existe baseline P78
  per DEBT-30 ENCERRADO.

**Decisões críticas C1**:
1. Se `extract_sides_lengths` NÃO existe: criar inline ou
   `extract_length` aplicado per-side.
2. Se `RoundedRect` NÃO existe: **radius armazenado
   semantic adiada** (Opção β graded). Pattern N=5 → 6.
3. Se `clip_mask` extensível ao body: clip real. Se NÃO:
   **clip armazenado semantic adiada** (Opção β graded).
   Pattern N=5 → 7 se ambos adiadas.

Se signature ou estado divergir significativamente: registar
`P231.div-N`.

### C2 — Adicionar `outset` + `radius` + `clip` a `Content::Block`

Editar `01_core/src/entities/content.rs` variant Block:

```rust
Block {
    body, width, height, inset, breakable,   // P156G baseline
    /// P231 — outset uniforme (margem externa visual).
    /// Default zero. Paridade `inset` baseline.
    outset: Sides<Length>,
    /// P231 — radius uniforme cantos arredondados.
    /// Default None. Semantic real depende `RoundedRect`
    /// primitivo (Opção β graded; audit C1 determina real
    /// ou adiada).
    radius: Option<Length>,
    /// P231 — clip overflow. Default false. Semantic real
    /// depende `FrameItem::Group::clip_mask` extensível
    /// (Opção β graded).
    clip: bool,
},
```

Block fields: **5 → 8** (+outset + radius + clip).

### C3 — Adicionar `outset` + `radius` + `clip` a `Content::Boxed`

Editar variant Boxed paridade:

```rust
Boxed {
    body, width, height, inset, baseline,    // P156H baseline
    /// P231 — outset paralelo Block.
    outset: Sides<Length>,
    /// P231 — radius paralelo Block (semantic adiada se
    /// RoundedRect ausente).
    radius: Option<Length>,
    /// P231 — clip paralelo Block (semantic adiada se
    /// clip_mask não extensível).
    clip: bool,
},
```

Boxed fields: **5 → 8** (+outset + radius + clip).

### C4 — Arms cascata exhaustivos (compiler-driven)

Total arms refino Block + Boxed P231:

**`entities/content.rs`** (5 arms × 2 variants = 10 arms):
- `is_empty` — proxy body (preservado).
- `plain_text` — recurse body (preservado).
- `PartialEq::eq` — comparação +3 fields cada (Block 8;
  Boxed 8).
- `map_content` — preserva 3 fields (Sides Copy via Length
  Copy; Option<Length> Copy; bool Copy).
- `map_text` — idem.

**`rules/introspect.rs`** (2 arms × 2 = 4 arms):
- `materialize_time` — preserva.
- `walk` — preserva.

**`rules/layout/mod.rs::layout_content`** (1 arm Block +
1 arm Boxed; refino renderização parcial graded).

**`rules/layout/mod.rs::measure_content_constrained`** (2
arms — outset afecta dimensions; radius/clip não).

**`rules/introspect/locatable.rs`** (catch-all preserva).

Total: **~18 arms cumulativos** (compiler-driven; iterar
até zero errors; possíveis 8-15 errors E0027/E0063).

### C5 — Refino `native_block` + `native_box` accept 3 named args

Editar `stdlib/layout.rs::native_block`:

```rust
// Accept named args expandido: [..., "outset", "radius", "clip"].
let outset = match args.named.get("outset") {
    Some(val) => extract_sides_lengths(val, "block", "outset")?,
    None => Sides::default(),
};
let radius = match args.named.get("radius") {
    Some(val) => Some(extract_length(val, "block", "radius")?),
    None => None,
};
let clip = match args.named.get("clip") {
    Some(Value::Bool(b)) => *b,
    Some(other) => return Err(/* "clip: espera Bool" */),
    None => false,
};
// Validar radius >= 0.
if let Some(r) = &radius {
    if r.is_negative() { return Err(/* "radius: negativo rejeitado" */); }
}
// ... existing ...
Ok(Value::Content(Content::Block {
    body, width, height, inset, breakable,
    outset, radius, clip,  // P231 +3
}))
```

Editar `native_box` paridade literal.

**Audit C1**: se `extract_sides_lengths` não existe, criar
inline ou usar `extract_length` per-side. Reuso N=N+1 se
existe.

Magnitude C5: **S+ (~45min)**.

### C6 — Renderização Opção β parcial graded

Editar arms `Content::Block` e `Content::Boxed` em
`layout/mod.rs::layout_content`:

**`outset` renderização real** (sempre):
```rust
// P231 — outset expande bounds visual antes de emit.
let visual_x0 = block_x0 - outset.left.to_pt();
let visual_y0 = block_y0 - outset.top.to_pt();
let visual_w = block_w + outset.left.to_pt() + outset.right.to_pt();
let visual_h = block_h + outset.top.to_pt() + outset.bottom.to_pt();
// (Note: outset apenas afecta bounds visual; não afecta layout flow.)
```

**`radius` renderização condicional**:
- **Se `RoundedRect` existe**: emit `FrameItem::Shape::RoundedRect`
  com radius.
- **Se NÃO existe (hipótese provável)**: **armazenado mas
  semantic adiada** per ADR-0054 graded; pattern N=5 → 6.
  Footnote ⁵⁰ inventário 148 documenta.

**`clip` renderização condicional**:
- **Se `Group::clip_mask` extensível ao body**: wrap body
  em `FrameItem::Group { clip_mask: Some(ShapeKind::Rect) }`.
- **Se NÃO**: armazenado semantic adiada; pattern N=5 → 7.

**Decisão pragmática per audit C1**: implementar parts
reais possíveis (outset garantido; radius/clip dependentes);
documentar parts adiadas explicitamente em footnote ⁵⁰.

Magnitude C6: **S+ (~1-1.5h)** dependendo de audit C1.

### C7 — Sentinelas P231

Tests P231 (paridade P230 estrutura; ~17 tests):

**Unit content** (~4 tests):
- `p231_block_variant_aceita_outset_radius_clip`.
- `p231_boxed_variant_aceita_outset_radius_clip`.
- `p231_block_partial_eq_inclui_3_fields` — comparação 8
  fields.
- `p231_block_map_content_preserva_3_fields`.

**Unit stdlib** (~9 tests):
- `p231_native_block_outset_sides_aceita`.
- `p231_native_block_outset_length_uniforme_aceita`.
- `p231_native_block_outset_negativo_rejeita`.
- `p231_native_block_radius_length_aceita`.
- `p231_native_block_radius_negativo_rejeita`.
- `p231_native_block_clip_bool_aceita`.
- `p231_native_block_clip_tipo_errado_rejeita`.
- `p231_native_box_paridade_block` — 3 fields paralelo.
- `p231_native_block_3_fields_simultaneos`.

**Layout E2E** (~4 tests):
- `p231_block_outset_expande_bounds_visual`.
- `p231_block_radius_armazenado_layout_preservado`
  (semantic adiada se RoundedRect ausente).
- `p231_block_clip_armazenado_layout_preservado`
  (semantic adiada se mask não extensível).
- `p231_box_outset_paridade_block`.

Total tests P231: **~17 tests** (4+9+4). Esperado pós-P231:
**2086 + 17 = 2103 verdes**.

### C8 — L0 NÃO tocado (ADR-0080 EM VIGOR aplicação
automática N=2)

**Decisão fixada — aplicação automática**:

P231 é refactor aditivo a variants Content existentes
(Block + Boxed). ADR-0080 §"Decisão" aplica-se por defeito.
**Segunda aplicação automática pós-promoção P229** (N=1 →
2 cumulativo).

L0 prompts NÃO tocados.

### C9 — Verificação tests workspace + lint

```
cargo test --workspace 2>&1 | tail -3
crystalline-lint .
crystalline-lint --fix-hashes .
```

Critério:
- 2086 verdes pré-P231 + ~17 novos = **~2103 verdes**.
- 0 violations preservadas.
- Hashes propagados em ~5-7 ficheiros L1.
- L0 prompts não tocados — "Nothing to fix".

**Risco regressão**: P156G + P156H tests pre-existentes
com construtor directo `Block {...}` ou `Boxed {...}`
podem precisar adaptação (+3 fields defaults). Hipótese
N=3-7 adaptações intencionais (paridade P228 N=6 +
escalação para 3 fields em vez de 1).

### C10 — Inventário 148 footnote ⁵⁰

**§A.5 Layout entrada `block(...)` + `box(...)`**: footnotes
existing pós-P156G/H. Adicionar **footnote ⁵⁰** documentando:
- A.4 materializado (quarto Categoria A Fase 5).
- 7 decisões fixadas.
- 3 atributos cumulativos (outset real; radius semantic
  graded; clip semantic graded — audit C1 determinará
  estado real).
- Pattern "refino aditivo paralelo entre variants irmãos"
  N=3 → **4 cumulativo** (Grid+Table P227/P228;
  GridCell+TableCell P230; **Block+Boxed P231**).
- Pattern "Field armazenado semantic adiada" N=5 → ? (5
  preservado, 6, ou 7 dependendo audit C1).
- Pattern Smart→Option N=9 → 10 cumulativo.
- Pattern "Field bool simples paridade vanilla" N=2 → 3
  cumulativo (`breakable`/`repeat`/**`clip`**).
- Pattern "aplicação automática ADR EM VIGOR" N=1 → 2.
- Reabertura P156G+H scope-outs documentados (parcial;
  fill/stroke/spacing/above/below/sticky preservados
  scope-out).

### C11 — ADR-0079 anotação Categoria A 4/5

Editar ADR-0079 com bloco P231:

```markdown
### P231 anotação — Categoria A sub-passo 4 (outset/radius/clip
Block + Boxed)

**Categoria A**: 4/5 sub-passos materializados ✓.
- A.1 stroke (P227) ✓.
- A.2 fill (P228) ✓.
- A.3 per-cell (P230) ✓.
- **A.4 Block/Boxed cosméticos (P231) ✓**.
- A.5 Place per-cell alignment override — pendente.

Trabalho P231:
- Block +3 fields outset/radius/clip (5 → 8 fields).
- Boxed +3 fields paralelo (5 → 8 fields).
- Renderização Opção β parcial graded (outset real;
  radius/clip semantic adiada se primitivos baseline
  ausentes; audit C1 determina).
- ~17 tests novos.
- **Segunda aplicação automática ADR-0080 EM VIGOR**.

Patterns consolidados:
- "Refino aditivo paralelo entre variants irmãos" N=3 →
  **4 cumulativo** (Grid+Table; GridCell+TableCell;
  Block+Boxed).
- "Aplicação automática ADR EM VIGOR" N=1 → 2.
- "Field armazenado semantic adiada" N=5 → ? (audit C1).
- "Field bool simples paridade vanilla" N=2 → 3 cumulativo.
- Smart→Option N=9 → 10 cumulativo.

Status ADR-0079 mantido PROPOSTO (4/13-15 sub-passos).
```

### C12 — Critério aceitação P231

- ~17 tests novos verdes.
- 2086 tests pre-existentes preservados (após N=3-7
  adaptações intencionais).
- 0 violations.
- Block +3 fields (5 → 8); Boxed +3 fields (5 → 8;
  paralelo).
- Renderização parcial graded funcional.
- ADR-0079 Categoria A 4/5 anotado.
- ADR-0080 EM VIGOR aplicação automática N=1 → 2.
- Cobertura Layout 89% preservada (refino qualitativo).

---

## §3 Output

1 ficheiro relatório curto:
`00_nucleo/materialization/typst-passo-231-relatorio.md`.

Estrutura (~6-8 KB) com 8 §s:

- §1 O que foi feito (sumário 3-5 linhas).
- §2 Inventário pré-P231 + audit primitivos visuais (C1).
- §3 Block/Boxed refino +3 fields cada (C2+C3).
- §4 `native_block`/`native_box` accept 3 named args (C5).
- §5 Renderização Opção β parcial graded (C6;
  outset real + radius/clip semantic estado real per audit).
- §6 Decisões substantivas (7 decisões fixadas) +
  segunda aplicação automática ADR-0080 EM VIGOR.
- §7 Resultados verificação + inventário 148 footnote ⁵⁰
  + ADR-0079 anotação Categoria A 4/5 (C7+C9+C10+C11).
- §8 Próximo sub-passo (P232 candidatos: A.5 Place
  per-cell; B.1 DEBT-34d; D.1 state; pivot).

Código alterado:
- **Editado**: `01_core/src/entities/content.rs` (Block
  + Boxed refino +3 fields cada + arms cascata + ~4 unit
  tests).
- **Editado**: `01_core/src/rules/introspect.rs` (arms
  preservados).
- **Editado**: `01_core/src/rules/layout/mod.rs` (arms
  Block + Boxed refino renderização + measure refino
  outset).
- **Editado**: `01_core/src/rules/stdlib/layout.rs`
  (`native_block` + `native_box` accept 3 named args;
  +~9 unit tests; possível helper `extract_sides_lengths`
  reuso).
- **Editado**: `01_core/src/rules/layout/tests.rs` (+~4
  E2E tests).
- **Editado**: `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`
  (footnote ⁵⁰ P231).
- **Editado**: `00_nucleo/adr/typst-adr-0079-layout-fase-5-roadmap.md`
  (+ anotação Categoria A 4/5 P231).

**Sem novos ficheiros**.

---

## §4 Não-objectivos

- `fill`/`stroke` Block/Boxed — não estavam em diagnóstico
  P226 Categoria A.4; Categoria A separada futura.
- `spacing`/`above`/`below`/`sticky` Block — atributos de
  fluxo distintos (Categoria B candidato algorítmico).
- `stroke-overhang` Boxed — refino futuro candidato.
- Per-corner `radius` via `Corners<T>` — requer criar tipo
  cristalino `Corners<T>` (trabalho separado).
- Implementar `RoundedRect` primitivo — fora escopo P231
  se ausente baseline; semantic adiada graded preserva
  Opção β.
- Refactor `FrameItem::Group::clip_mask` para extensão
  body — fora escopo P231 se necessário; semantic adiada
  graded.
- Promover ADR-0079 PROPOSTO → IMPLEMENTADO — só pós
  Categoria A 5/5 + B + C + D completas.
- Promover ADR-0066 PROPOSTO → IMPLEMENTADO — só pós-D.1
  state materializa.
- Tocar em L0 prompts — ADR-0080 EM VIGOR aplicação
  automática.
- Show rules `#show block: ...` — fora escopo Fase 5.
- Reabrir decisões arquiteturais — A.4 é Categoria A.
- Renderização completa cantos arredondados se primitivo
  baseline ausente — semantic adiada graded é decisão
  arquitectural aceite per ADR-0054.

---

## §5 Riscos a evitar

1. **`RoundedRect` primitivo ausente forçar Opção α
   inflada**: audit C1 crítico. Se ausente, **manter
   Opção β graded** (radius semantic adiada); NÃO criar
   primitivo em P231 (escopo separado).
2. **`Group::clip_mask` não-extensível ao body forçar
   refactor**: idem. **Manter Opção β graded** (clip
   semantic adiada); refactor mask separado.
3. **`extract_sides_lengths` helper ausente forçar inline
   parsing**: audit C1; alternativa inline trivial
   aceitável.
4. **Tests pre-existentes Block/Boxed**: hipótese N=3-7
   testes com construtor directo precisam +3 fields
   defaults. Adaptação intencional documentada.
5. **L0 tocado por engano**: violar ADR-0080 EM VIGOR
   exactamente em segunda aplicação automática. Mitigação:
   §5 risco 5 explícito + §C8 fixa não tocar.
6. **`Corners<T>` criação prematura**: tentação por
   "paridade vanilla literal radius per-corner". Rejeitada
   — escopo separado per Decisão 3 Opção β.
7. **`outset` afectar layout flow**: outset apenas afecta
   bounds visual; NÃO afecta layout flow (paridade vanilla
   literal). Mitigação: documentar inline-doc.
8. **Magnitude exceder M (~2-3h)**: P230 chegou em ~1.5h.
   P231 mais complexo (3 fields × 2 variants + renderização
   condicional). Hipótese real M (~2h) com audit C1
   convergindo Opção β graded.
9. **Pattern "Field semantic adiada" N=5 → 7 inflar**:
   se ambos radius e clip semantic adiadas, pattern N=5
   → 7 cumulativo (+2 em P231). Aceitável; pattern já
   sólido N=5 patamar.
10. **Validação `outset` negativo confuso**: outset positivo
    expande visualmente; negativo seria "outset interior"
    paridade conceito. Vanilla rejeita negativos.
    Cristalino paridade — rejeitar negativos.
11. **Refactor `Sides<Length>::default()` para outset**:
    `Sides<Length>::default()` retorna `Sides { top:
    Length::zero(), right: 0, bottom: 0, left: 0 }`.
    Audit C1 confirma comportamento.
12. **Documentar precedência outset/inset em L0**: tentação
    por "regra nova". Rejeitada — ADR-0080 EM VIGOR
    aplicação automática; inline-doc + footnote ⁵⁰
    suficiente.

---

## §6 Hipótese provável

C1 confirmará Block + Boxed baseline 5 fields cada;
`extract_sides_lengths` helper provável existente; **`RoundedRect`
provável NÃO existir** (geometry.rs P76 enumera Rect/Ellipse/
Line apenas); `Group::clip_mask` existe baseline P78.

C2+C3 adicionarão 3 fields paralelo a Block e Boxed.

C4 cobrirá ~18 arms cumulativos (compiler-driven).

C5 refinará `native_block` + `native_box` accept 3 named
args.

C6 implementará outset real + **radius semantic adiada**
(RoundedRect ausente) + clip avaliar conforme audit C1.

C7 criará ~17 tests novos.

C8 NÃO tocará L0 (aplicação automática ADR-0080 EM VIGOR
N=1 → 2).

C9 reportará ~2103 verdes; 0 violations; possíveis N=3-7
adaptações Block/Boxed baseline.

C10 reclassificará footnote ⁵⁰.

C11 anotará ADR-0079 Categoria A 4/5.

C12 verifica critério aceitação.

Custo real: **M (~2h)** — paridade P228 estructura mas com
3 fields × 2 variants + renderização condicional.

Mas é hipótese, não decisão. C1-C12 fixam-se empíricamente.

---

## §7 Particularidade P231

P231 é estruturalmente distinto na trajectória pós-M9c:

- **Quarto sub-passo materialização Fase 5 Layout
  candidata** — Categoria A 4/5 sub-passos. Após P231,
  apenas A.5 (Place per-cell) pendente para fechar
  Categoria A 5/5.
- **Segunda aplicação automática ADR-0080 EM VIGOR
  pós-promoção P229** — pattern N=1 → 2 cumulativo.
  Validação contínua de regra metodológica em prática.
- **Pattern "refino aditivo paralelo entre variants
  irmãos" N=3 → 4 cumulativo** (Grid+Table P227/P228;
  GridCell+TableCell P230; **Block+Boxed P231**). Pattern
  muito sólido empiricamente.
- **Reabertura formal de P156G+H scope-outs documentados
  há tempo** — primeira reabertura cosmética sub-passo
  Fase 5 (P156G+H criados 2026-04-25; P231 reabre 18 dias
  depois). Pattern de continuidade arquitectural cumulativa.
- **Pattern "Field bool simples paridade vanilla" N=2 →
  3 cumulativo** (`breakable` P156G; `repeat` P224.B;
  **`clip` P231**). Patamar N=3 atinge limiar
  formalização (N=3-4); promoção formal candidato refino
  XS futuro.
- **Pattern Smart→Option N=9 → 10 cumulativo** (radius
  Option<Length>). N=10 patamar empírico **muito sólido**;
  promoção formal candidato (análogo a `extract_length`
  N=10 promoção pública candidato).
- **Pattern "Field armazenado semantic adiada" N=5 → ? +
  1 ou 2** (radius e/ou clip se primitivos ausentes; audit
  C1 determina).
- **Decisão 5 Opção β graded inaugura segunda aplicação
  graded estructural pós-M9c** — distinto de Decisões
  graded P156D/E (Weak collapse) ou P224.B (repeat
  layout). Graded P231 é **partial real + adiada por field**
  em vez de uniforme.
- **Cobertura Layout per metodologia preservada 89% real**
  — A.4 é refino qualitativo cosmético.
- **Anti-inflação 23ª aplicação cumulativa** pós-P205D —
  Opção α escopo restrito + Opção β graded parcial +
  Opção α refino paralelo + Opção γ L0 automático + sem
  promoção helper público + sem criar `Corners<T>` + sem
  criar `RoundedRect` primitivo + ADR-0079 sem promoção.

Por isso §5 risco 6 (criar `Corners<T>` prematuro) é o
mais provável simbolicamente. Tentação: "vanilla radius
aceita Corners; paridade observable; criar agora".
**Defesa**: ADR-0029 enumera `Corners<T>` como tipo
vanilla scope-out cristalino; criar é trabalho separado.
Decisão 3 Opção β preserva escopo.

**Critério de aceitação P231**:
- ~17 tests novos verdes.
- 2086 tests pre-existentes preservados (após N=3-7
  adaptações intencionais).
- 0 violations.
- Block +3 fields (5 → 8); Boxed +3 fields (5 → 8;
  paralelo).
- Renderização parcial graded funcional (outset real;
  radius/clip dependentes audit C1).
- ADR-0080 EM VIGOR aplicação automática N=1 → 2.
- ADR-0079 Categoria A 4/5 anotado.
- Cobertura Layout 89% preservada.

**Estado pós-P231 esperado**:
- Tests workspace: 2086 → **~2103 verdes** (+17).
- Stdlib funcs: 60 preservado.
- Content variants: 59 preservado.
- Value variants: 55 preservado.
- **Block fields: 5 → 8** (+outset + radius + clip).
- **Boxed fields: 5 → 8** (+outset + radius + clip
  paralelo).
- §A.5 distribuição: `12/4/2/0/0 = 18` preservada (refino
  qualitativo).
- Cobertura Layout per metodologia: **89% preservada**.
- Cobertura user-facing total: 67% preservada.
- ADR-0066 PROPOSTO; ADR-0061 IMPLEMENTADO; ADR-0078
  IMPLEMENTADO; ADR-0079 PROPOSTO; ADR-0080 EM VIGOR.
- Saldo DEBTs: 12 preservado.
- **23 aplicações cumulativas anti-inflação** pós-P205D.
- **Pattern "refino aditivo paralelo entre variants
  irmãos" N=4 cumulativo** consolidado.
- **Pattern "aplicação automática ADR EM VIGOR" N=2
  cumulativo**.
- **Pattern "Field bool simples paridade vanilla" N=3
  cumulativo** (limiar formalização atingido; candidato
  XS).
- **Pattern Smart→Option N=10 cumulativo** (patamar muito
  sólido; candidato promoção formal).
- **Pattern "Field armazenado semantic adiada" N=5 → ? +
  1 ou 2** (audit C1 determina).
- **Categoria A Fase 5 Layout**: 4/5 → próximo A.5 Place
  per-cell fecha categoria 5/5.
- **Fase 5 Layout candidata**: 4/13-15 sub-passos
  materializados (P227 A.1 ✓; P228 A.2 ✓; P230 A.3 ✓;
  **P231 A.4 ✓**; A.5 + B + C + D pendentes).
