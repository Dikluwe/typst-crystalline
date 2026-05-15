# Spec do passo P247 — A.4 fill + stroke + outset activação real em Block + Boxed (agregado M-L; 3 scope-outs originais P156G/H promovidos cumulativamente; fecha 6 dos 9 scope-outs originais Block ÷ 6 dos 6 scope-outs Boxed)

**Data**: 2026-05-14.
**Tipo**: refino aditivo a variants existentes Block + Boxed
+ activação consumer Layouter + emissão `FrameItem::Shape`.
Promove 3 scope-outs cosméticos originais P156G/P156H. **2
fields novos** em ambos variants (`fill: Option<Color>` +
`stroke: Option<Stroke>`); **outset semantic real activado**.
**Magnitude planeada**: **M-L (~4-6h)** — paridade conservadora.
Audit C1 §2 pode revelar magnitude inferior se outset já tiver
trabalho parcial pré-existente (paridade pattern P243→P244
detectado). `Stroke` + `Color` tipos já existentes; PDF exporter
fill+stroke já operacional (confirmado pelos achados empíricos
2026-05-14).
**Marco**: continuação materialização pós-P246 cell migration;
**fecha cumulativamente 5 dos 9 scope-outs Block originais
P156G** (outset P231 ✓ + radius P242 ✓ + clip P242 ✓ + fill P247
+ stroke P247 = 5/9; restam spacing + above + below + sticky);
**fecha 5 dos 6 scope-outs Boxed P156H** (outset P231 + radius
P242 + clip P242 + fill P247 + stroke P247 = 5/6; resta
stroke-overhang); **promoção real scope-out ADR-0054 graded**
N=2 → **N=3 cumulativo** (P242 radius/clip; P246 cell migration
foi refactor não-promoção; **P247 fill + stroke + outset
semantic = N=3 promoções reais**); inaugura **"agregar promoções
de scope-outs cosméticos visuais"** sub-padrão N=1.

---

## §1 O que será feito

### §1.1 Estado pré-P247 confirmado empíricamente (2026-05-14)

`Content::Block` (8 fields):
```rust
Block {
    body:      Box<Content>,
    width:     Option<Length>,
    height:    Option<Length>,
    inset:     Sides<Length>,
    breakable: bool,
    outset:    Sides<Length>,                      // P231 armazenado; semantic parcial
    radius:    Corners<Length>,                    // P242 semantic real
    clip:      bool,                               // P242 semantic real
}
```

`Content::Boxed` (8 fields):
```rust
Boxed {
    body:     Box<Content>,
    width:    Option<Length>,
    height:   Option<Length>,
    inset:    Sides<Length>,
    baseline: Length,
    outset:   Sides<Length>,                       // P231 armazenado; semantic incerta
    radius:   Corners<Length>,                     // P242 semantic real
    clip:     bool,                                // P242 semantic real
}
```

`Stroke` em `01_core/src/entities/geometry.rs:24`:
```rust
pub struct Stroke {
    pub paint:     Color,
    pub thickness: f64,
}
```

`Color` em `layout_types.rs` (ADR-0028 simplificado; preservado
ADR-0029): enum `{ Rgb { r, g, b: u8 }, Rgba { r, g, b, a: u8 } }`.

`FrameItem::Shape { pos, kind, width, height, fill, stroke }`
confirmado em 03_infra/src/export.rs:845 + 1117 + 1361 + 1543
(4 caminhos PDF: Helvetica + CIDFont + 2 variantes). Operadores
`rg` (fill RGB), `RG` (stroke RGB), `w` (thickness), paint
operator `b`/`B`/`f`/`s` selecionado conforme `(fill.is_some(),
stroke.is_some())`.

`outset` em Block linha 660-663 — comentário declara
"Renderização real em layout_grid expande bounds visual" mas
ambiguidade: pode estar activo só em contexto Grid (parent
cell) ou armazenado mas inerte. **Audit C1 §2.5 verifica
empíricamente**.

### §1.2 Trabalho a fazer P247

1. **+2 fields em Block + Boxed** (paridade simétrica):
   - `fill: Option<Color>` (default `None` == sem fill).
   - `stroke: Option<Stroke>` (default `None` == sem stroke).
2. **8 → 10 fields** ambos variants.
3. **Layouter activa emissão `FrameItem::Shape`** antes do body
   quando `fill.is_some() || stroke.is_some() || outset != Sides::ZERO`:
   - Shape kind = `Rect` (ou `RoundedRect { radii: radius }`
     reusando P242 se radius != zero).
   - Bounds expandidos por outset (margem externa visual).
   - `fill` + `stroke` passados para Shape.
4. **outset semantic real**: bounds visual do Shape expande por
   outset; cursor.y avança outset.top antes do body, outset.bottom
   depois (paralelo a inset mas margem externa). Audit C1 §2.5
   confirma se já existia em Grid context — se sim, preserva +
   estende a Block isolado.
5. **stdlib `native_block` + `native_box` aceitam** `fill` +
   `stroke` named args. Helpers de extracção: `extract_color`
   (já pode existir; verificar §2.6) + `extract_stroke` (novo;
   N=1 ou reuso se existir em Table/Grid).
6. **`P156G` + `P156H` "limitações conscientes"** anotadas em
   `entities/content.md` L0 prompt: 3 scope-outs cosméticos
   fechados em P247.
7. **Tests** (~15-25 novos): unit content + unit stdlib + E2E
   layout.

### §1.3 Tests esperados

Tests P247 novos estimados: **15-25** (range M-L magnitude;
2 variants × atributos múltiplos):

- 4-6 unit content (Block + Boxed fields cascata + PartialEq +
  map_content + map_text + construtores).
- 6-10 unit stdlib (`native_block` + `native_box`: fill aceita
  Color; stroke aceita Stroke; fill/stroke ausentes default
  None; named arg desconhecido rejeitado; tipos errados
  rejeitados).
- 3-5 E2E layout (Block com fill renderiza shape; Block com
  stroke + radius preserva clip; outset expande bounds visual;
  combinação fill+stroke+radius+clip+outset).
- 1-3 unit Layouter (emissão Shape antes do body; bounds
  outset; paridade Block ↔ Boxed em estrutura emit).

**Workspace pós-P247**: **2209 → ~2224-2234 verdes** (range
+15-25 paridade M-L magnitude).

### §1.4 Adaptações pre-existentes

Estimativa **N=0-10** adaptações tests pré-existentes:

- Tests construtores `Content::block(body, width, height, inset,
  breakable, outset, radius, clip)` precisam de **+2 args** para
  `fill` + `stroke` → mudança mecânica.
- Tests construtores `Content::boxed(body, width, height, inset,
  baseline, outset, radius, clip)` análogo.
- PartialEq tests com fields explícitos → adicionar fill +
  stroke à comparação.
- E2E tests P231/P242 que verificam Block/Boxed específicos
  preservados literal (output visual baseline preservado se
  fill/stroke default None).

**Cenário `P247.div-1`**: se audit C1 §2.6 revelar que
`extract_color` ou `extract_stroke` já existem em stdlib com
semantic incompatível com paridade vanilla Block/Boxed, criar
divergência formal antes de modificar.

---

## §2 Verificação empírica pré-P247 OBRIGATÓRIA BLOQUEANTE (C1) — lição N=9 → 10 cumulativo

Audit C1 obrigatório bloqueante pós-P236.div-1. Lição refinada
N=9 P246 ("mapear empíricamente distribuição de usos por
sub-módulo antes de fixar arquitectura de migração") expande
para **N=10 cumulativo**: "mapear scope-outs declarados
historicamente vs estado real materializado antes de assumir
ausência" (refino directo do pattern P243→P244 onde scope-outs
declarados estavam factualmente materializados).

### §2.1 Block + Boxed fields actuais (já confirmado 2026-05-14)

8 fields cada, confirmado via `sed`. **Hipótese pré-P247
preservada literal**: ambos não têm `fill` nem `stroke`.

### §2.2 `Stroke` + `Color` + `Paint` tipos existentes (já confirmado)

- `Stroke { paint: Color, thickness: f64 }` em
  `01_core/src/entities/geometry.rs:24` ✓.
- `Color` enum em `layout_types.rs` `{ Rgb, Rgba }` ✓ (ADR-0028
  simplificado preservado).
- **`Paint` enum NÃO existe** — Stroke usa Color directamente.
  Decisão 1 §3 fixa este caminho.

### §2.3 PDF exporter fill+stroke operacional (já confirmado)

4 sítios em `03_infra/src/export.rs` com pattern
`FrameItem::Shape { fill, stroke, .. }` + operadores RGB
+ paint operator selection. **Confirma que P247 é puramente
L1+stdlib; zero trabalho L3**.

### §2.4 Outset semantic actual — VERIFICAÇÃO BLOQUEANTE §2.5

```bash
grep -rn "\.outset\b\|outset:" 01_core/src/rules/layout/ \
  | grep -v "test\|/\\*\\*" \
  | head -40
```

Cenários possíveis:

- **A** — outset apenas armazenado, zero usos em Layouter →
  P247 activa pela primeira vez (M magnitude).
- **B** — outset activo só em arm Grid (parent cell wrap) → P247
  estende a arms Block + Boxed isolados (S+ magnitude).
- **C** — outset activo em todos os arms relevantes → P247
  preserva literal + foco em fill/stroke (S magnitude — escopo
  reduzido).

**Cenário C** seria cenário tipo P243→P244 onde o comentário
"renderização real em layout_grid expande bounds visual" indica
trabalho já completo. Audit empírico decide.

### §2.5 Trabalho real outset diagnosticado

```bash
grep -B2 -A10 "outset" 01_core/src/rules/layout/mod.rs \
  01_core/src/rules/layout/grid.rs 2>/dev/null \
  | head -60
```

Identificar:
1. Arm Block consume outset? Como?
2. Arm Boxed consume outset? Como?
3. Arm Grid (paridade P157A delegação) consume outset via Block
   nested?

### §2.6 Helpers stdlib pré-existentes

```bash
grep -n "fn extract_color\|fn extract_stroke\|fn extract_paint" \
  01_core/src/rules/stdlib/
```

Identificar se `extract_color` + `extract_stroke` já existem
(provavelmente para Table/Grid fill+stroke materializados em
P157A-C). Reuso preservar (sub-padrão "helper privado reuso"
N≥5 cumulativo).

### §2.7 fill + stroke em Table/Grid — semantic atribuição

Pesquisa cómo Table/Grid implementam fill + stroke pode informar
P247 design — paridade ou divergência consciente.

```bash
grep -B2 -A8 "fill.*Color\|stroke.*Stroke" \
  01_core/src/rules/stdlib/structural.rs 2>/dev/null \
  | head -40
```

### §2.8 Tests pré-P247 baseline

```bash
cargo test --workspace
```

Esperado: **2209 verdes** (estado pós-P246).

### §2.9 Decisão arquitectural pós-audit

Após §2.4 + §2.5 + §2.6 + §2.7 completos, fixar empíricamente
Decisão 1 (cenário A/B/C outset) + Decisão 2 (helper reuso vs
duplicação).

### `P247.div-N` antecipadas — possíveis

- **`P247.div-1`** se §2.4-§2.5 revelar outset cenário C
  (totalmente activo) → re-escopo para apenas fill+stroke
  (paridade pattern P243→P244 graded).
- **`P247.div-2`** se §2.6 revelar `extract_stroke` ausente
  E `extract_color` ausente → confirmação não-disruptiva
  (sem div formal; só notar trabalho extra de helpers).
- **`P247.div-3`** se §2.7 revelar fill+stroke em Table/Grid
  com semantic NÃO directamente reutilizável para Block/Boxed
  → divergência arquitectural formalizar.
- **`P247.div-4`** se baseline §2.8 ≠ 2209 → reconciliação prévia.

---

## §3 Decisões fixadas P247 — 9 decisões

### Decisão 0 — Audit C1 lição N=9 → 10 cumulativo

Pattern "spec C1 audit obrigatório bloqueante pós-P236.div-1"
N=9 → **10 cumulativo**. Refino procedural P247: "mapear
scope-outs declarados historicamente vs estado real
materializado antes de assumir ausência" (refino directo do
pattern P243→P244). Anotação em ADR-0080 §"Lição refinada
P247".

### Decisão 1 — `fill: Option<Color>` em Block + Boxed (preliminar)

**Type**: `Option<Color>` (não `Option<Paint>` — `Paint` enum
não existe; `Color` é caminho mínimo per ADR-0029).

**Default**: `None` (sem fill).

**Paridade simétrica Block + Boxed**: idêntica declaração
ambos variants (paridade pattern "refino aditivo paralelo entre
variants irmãos" N=4 → **N=5 cumulativo** P247).

**Justificação**: Stroke já usa `Color` directo
(`geometry.rs:24`). Adicionar `Paint` enum seria refactor
cross-cutting fora de scope P247 (futuro ADR dedicada).

### Decisão 2 — `stroke: Option<Stroke>` em Block + Boxed

**Type**: `Option<Stroke>` (reuso `Stroke` de
`geometry.rs:24`). **Default**: `None`.

**Paridade simétrica** — análoga Decisão 1.

### Decisão 3 — outset semantic real activação (FINAL pós-audit §2.9)

**Cenário preliminar A** (outset zero-uso) — activar pela
primeira vez:
- Layouter arm Block + Boxed avançam cursor pre-body por
  `outset.top` (mais inset.top em sequência).
- Bounds Shape expandidos por `outset` em todos os lados.
- Cursor.y avança `outset.bottom` pós-body (paralelo a
  `inset.bottom`).

**Cenário preliminar B** (outset Grid-only) — estender:
- Preservar comportamento Grid existente literal.
- Adicionar comportamento equivalente em arm Block + Boxed
  isolados.

**Cenário preliminar C** (outset totalmente activo) — escopo
reduzido:
- Preservar comportamento existente.
- Focar P247 em fill + stroke.

**Decisão final fixa pós-audit §2.9.**

### Decisão 4 — Layouter activação ordem precedência visual

Quando Block/Boxed tem `fill.is_some() || stroke.is_some() ||
outset != Sides::ZERO`:

1. Calcular bounds (com outset expansion).
2. Emitir `FrameItem::Shape { pos, kind, width, height, fill,
   stroke }` no início do bloco (antes do body):
   - `kind`: `Rect` se radius == zero; senão `RoundedRect
     { radii: radius }` (reuso P242).
3. Layout body normalmente (P156G/H semantic preservada).
4. Se `clip == true`, wrap body em `FrameItem::Group` com
   `clip_mask` (reuso P242 semantic; preservar literal).

**Ordem importa**: Shape emitido **antes** do body para que
PDF renderize fill+stroke por baixo do body (paridade vanilla;
z-order natural).

### Decisão 5 — Z-order outset vs inset vs body

```
outer bound:  pos.x - outset.left, pos.y - outset.top
shape bounds: outer_bound + (width + outset.left+right,
                              height + outset.top+bottom)
body origin:  pos.x + inset.left, pos.y + inset.top
body bounds:  width - inset.left-right, height - inset.top-bottom
```

Shape fill+stroke abrangem toda área outset+inset+body; cursor
real do body apenas inset (mesma semântica P156G).

### Decisão 6 — Helpers stdlib

`extract_color` + `extract_stroke` — verificar §2.6 audit:

- Se existirem (provável; Table/Grid materializaram) → **reuso**
  directo. Sub-padrão "helper privado reuso" N+1 cumulativo.
- Se faltarem → criar novos em `stdlib/layout.rs` (paralelo a
  `extract_length` N=7+). Pattern emergente sólido.

`Color` parse: aceita `Value::Color` directo OR string `"rgb(...)"`
via fallback (paridade vanilla); rejeitar tipos inválidos com
erro hard.

`Stroke` parse: aceita Dict `{ paint: Color, thickness: Length }`
OR Length puro (constrói Stroke com paint default). Rejeitar
tipos inválidos.

### Decisão 7 — Sem novo `Content::*` variant; sem nova ADR; sem novo entity type

P247 é refino aditivo. **Anti-inflação 39ª aplicação cumulativa**
preservar: Opção β L0 minimal (apenas `entities/content.md`
L0 estendido por field; hashes propagados); Opção α anotação
cumulativa ADRs; sem ADR meta nova.

### Decisão 8 — Anti-inflação 39ª aplicação cumulativa

- Opção β L0 minimal (refino aditivo a content.md; hash propagado).
- Opção α extensão field-by-field (2 fields novos paralelos).
- Opção α activação consumer real (outset semantic + Shape
  emission).
- Opção α anotação cumulativa minimal ADRs (0079 + 0080 + 0061).
- Opção α reuso helpers stdlib (se existirem).
- Opção α scope-outs fechados sem ADR meta prematura.

### Decisão 9 — Padrão emergente "Agregar promoções scope-outs cosméticos visuais" N=1 inaugurado

P247 inaugura sub-padrão **N=1**: "Agregar promoções múltiplas
scope-outs cosméticos visuais num passo único" (3 promoções
agregadas: outset semantic real + fill + stroke). Cumprimento:
- Magnitude controlada M-L (não L+).
- Coesão semantic forte (3 atributos visuais ortogonais).
- Tests cross-multiplicados naturalmente.

Candidato a formalização N=3-4 se outras agregações ocorrerem
(hipóteses futuras: 4 scope-outs restantes Block — spacing +
above + below + sticky; agregar em passo único S-M paridade
P247).

---

## §4 Ficheiros a editar (C2+C3+C4+C5)

| Categoria | Ficheiro | Trabalho |
|-----------|----------|----------|
| L1 entity | `01_core/src/entities/content.rs` | Block: +2 fields (fill, stroke); Boxed: +2 fields; cascata 9 arms cada (declaração, construtor, is_empty, plain_text, PartialEq, map_content, map_text, materialize_time, walk, layout arm) |
| L0 prompt | `00_nucleo/prompts/entities/content.md` | Secção Block + secção Boxed: documentar fill + stroke (2 fields cada); §"Limitações conscientes P156G/H" anotar 3 scope-outs fechados em P247 |
| L1 stdlib | `01_core/src/rules/stdlib/layout.rs` | `native_block` + `native_box` aceitam `fill` + `stroke` named args; helpers `extract_color` + `extract_stroke` (reuso ou novos per §2.6) |
| L1 Layouter | `01_core/src/rules/layout/mod.rs` (ou arm dedicado) | Arm Block + Boxed activam Shape emission + outset semantic real (Decisão 4-5) |
| Tests content | `01_core/src/entities/content.rs` (test module) | 4-6 unit tests + adaptações construtores existentes |
| Tests stdlib | `01_core/src/rules/stdlib/mod.rs` (test module) | 6-10 unit tests native_block + native_box fill/stroke |
| Tests Layouter | `01_core/src/rules/layout/tests.rs` (ou módulo) | 1-3 unit tests Shape emission |
| Tests E2E | `03_infra/tests/` (ou local) | 3-5 E2E layout cross-attribute |
| Inventário 148 | `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md` | §A.5 `block(...)` + `box(...)` reclassificadas — footnotes existentes preservadas; nova footnote ⁶⁵ P247 documenta 3 scope-outs cosméticos fechados; cobertura Layout per metodologia recalculada |
| ADR-0061 | `00_nucleo/adr/typst-adr-0061-layout-fase-x-roadmap.md` | §"Refino futuro" anotação P247: fill + stroke + outset semantic activados; padrão "agregar promoções" inaugurado |
| ADR-0079 | `00_nucleo/adr/typst-adr-0079-fase-5-layout-roadmap.md` | Categoria A.4 §"Sub-categorias materializadas": fill + stroke + outset Block+Boxed P247 |
| ADR-0080 | `00_nucleo/adr/typst-adr-0080-l0-minimal-para-refactors.md` | §"Lição refinada P247" anotada N=10 cumulativo; sub-categoria "promoção real scope-outs cosméticos" N=2 → 3 cumulativo |
| ADR-0054 | `00_nucleo/adr/typst-adr-0054-graded-paridade.md` (ou similar) | §"Promoções reais scope-outs" cumulativo: P242 radius+clip + P247 outset+fill+stroke = 5 promoções cumulativas (limiar ADR meta candidata) |
| DEBT.md | `00_nucleo/DEBT.md` | DEBT-30 P79 sentinela preservada (clip_mask Shape) + nova entrada cumulativa P247 (sem reabertura; anotação) |
| Relatório P247 | `00_nucleo/materialization/typst-passo-247-relatorio.md` | Estrutura canónica passos materialização M-L magnitude |

---

## §5 Critério aceitação P247 (C6+C7)

| Critério | Esperado |
|----------|----------|
| `cargo build --workspace` | **verde** |
| `cargo test --workspace` | **2209 → ~2224-2234 verdes** (+15-25 paridade M-L) |
| `crystalline-lint .` | **0 violations** |
| `crystalline-lint --fix-hashes` | 1 hash propagado (`entities/content.md` extensão) |
| Content variants | **62 preservado** (refino aditivo a variants existentes, sem novos) |
| ShapeKind variants | **5 preservado** |
| Block fields | **8 → 10** (+fill, +stroke) |
| Boxed fields | **8 → 10** (+fill, +stroke) |
| Layouter fields | preservado |
| Regions fields | **4 preservado** |
| Stdlib funcs | **64 preservado** (apenas refino consumer existentes) |
| §A.5 `block(...)` | reclassificação implementado⁺ + footnote ⁶⁵ P247 |
| §A.5 `box(...)` | reclassificação implementado⁺ + footnote ⁶⁵ P247 |
| Cobertura Layout per metodologia | **~93-94% → ~94-95%** (+1pp refino qualitativo) |
| Cobertura user-facing total | **~75-76% preservado** |
| Scope-outs Block originais P156G | 0/9 → **5/9 fechados** (outset+radius+clip+fill+stroke) |
| Scope-outs Boxed originais P156H | 0/6 → **5/6 fechados** (idem + falta stroke-overhang) |
| Promoções reais scope-outs cumulativas | 2 (P242 radius+clip) → **5** (P242 ×2 + P247 ×3) |
| ADR-0079 Categoria A.4 | anotação cumulativa P242+P246+P247 |
| ADR-0080 sub-categoria | "promoção real scope-outs cosméticos" N=2 → 3 cumulativo |
| ADR-0054 §"Promoções reais" | cumulativo N=5 — limiar ADR meta candidata anotado |
| L0 hashes propagados | 1 (`content.md`) |
| Adaptações pre-existentes | **N=0-10** estimadas (construtores Block/Boxed); `P247.div-N` se >10 |
| Regressões reais | **0** |
| Patterns emergentes | "Agregar promoções scope-outs cosméticos visuais" N=1 inaugurado; "Spec C1 audit obrigatório bloqueante" N=9 → 10 cumulativo |

**3 pré-condições obrigatórias verificadas**:

1. **Tests baseline preservados**: 2209 verdes pré-P247 →
   ~2224-2234 pós-P247 (+15-25 novos; N=0-10 adaptações
   construtores documentadas).
2. **Comemo memoization invariants ADR-0073/0074 preservados**.
3. **Backward compat**: Block/Boxed com fill=None, stroke=None,
   outset=Sides::ZERO renderizam **idênticos a P246**
   (output PDF bit-equivalente).

**Promoções ADR esperadas**:

- ADR-0079 Categoria A.4 cumulativa P242+P246+P247.
- ADR-0080 sub-categoria "promoção real scope-outs cosméticos"
  N=3.
- ADR-0061 §"Refino futuro" anotação P247.
- ADR-0054 §"Promoções reais" cumulativo N=5 — limiar ADR meta
  candidata anotado (não materializada em P247; futuro passo
  administrativo XS).
- **Sem novas ADRs criadas**.

---

## §6 Próximo sub-passo pós-P247

P247 fecha 5 dos 9 scope-outs Block + 5 dos 6 Boxed. Restantes
pendentes:

| Caminho | Trabalho | Magnitude | Prioridade |
|---------|----------|-----------|------------|
| **A.4 breakable per-cell activação real** | Materializar semantic Block.breakable + Boxed.height overflow + TableCell overflow | M (~2-4h) | **alta** (desbloqueio arquitectural P246 explorado) |
| **A.4 Block 4 scope-outs restantes** | spacing + above + below + sticky (paridade P247 agregada) | S-M | média |
| **A.4 Boxed 1 scope-out restante** | stroke-overhang | XS | baixa |
| ADR meta admin XS — "promoções reais scope-outs" N=5 | Formalizar pattern cumulativo (limiar sólido pós-P247) | XS | média |
| ADR-0079 → IMPLEMENTADO graded | Scope-out humano C.2 | XS-S | alta se humano decide fechamento |
| Pivot outro módulo | Visualize 54%; Text 52%; Model 50% | varia | baixa |

**Recomendação subjectiva pós-P247**: **A.4 breakable per-cell
activação real** (sequente natural P246 desbloqueio
arquitectural; magnitude M; activa semantic real Block.breakable
+ Boxed.height + TableCell que esperam há muito).

Alternativa: **ADR meta admin XS** formalizar pattern "promoções
reais scope-outs" agora que N=5 atinge limiar sólido (P242 ×2
+ P247 ×3). Patamar conceptual claro para anotação meta.

**Decisão humana fica em aberto literal** pós-P247.

**Estado esperado pós-P247**:
- Tests workspace: **~2224-2234 verdes** (+15-25 P247).
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
- **Patterns emergentes pós-P247** (2):
  - "Agregar promoções scope-outs cosméticos visuais" N=1
    inaugurado.
  - "Spec C1 audit obrigatório bloqueante" N=9 → **10
    cumulativo**.
  - "Promoção real scope-out ADR-0054 graded" N=2 → **3
    cumulativo** (P242 radius + P242 clip + P247 outset +
    P247 fill + P247 stroke = 5 promoções reais cumulativas
    contando granular).
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
  para ADR meta candidata.

---

## §7 Notas operacionais para o executor

1. **Audit C1 BLOQUEANTE prioridade absoluta**. Não materializar
   antes de §2.1-§2.9 completos. **Lição N=10 cumulativa**:
   refino procedural "mapear scope-outs declarados historicamente
   vs estado real materializado antes de assumir ausência".
   Se §2.4-§2.5 revelar cenário C (outset totalmente activo) →
   `P247.div-1` formal + re-escopo para apenas fill+stroke.

2. **Decisão 3 final fixa pós-audit §2.9**. Cenários A/B/C
   outset documentados em §3 são preliminares; decisão final
   baseada em achado empírico §2.4-§2.5.

3. **Ordem de implementação recomendada**:
   1. Audit C1 §2 completo (~20-30 min).
   2. Decisões finais §3 (~5-10 min documentação).
   3. Block + Boxed fields cascata (~60-90 min — 9 arms cada).
   4. stdlib helpers + native_block + native_box (~45-60 min).
   5. Layouter activação Shape + outset (~45-60 min).
   6. Tests cross-multiplicados (~60-90 min).
   7. Anotações ADRs + inventário 148 (~20-30 min).
   8. Relatório P247 (~30-45 min).

   **Total ~4-6h** paridade M-L magnitude.

4. **Paridade simétrica Block ↔ Boxed**: aplicar mudanças
   idênticas em ambos. Sub-padrão "refino aditivo paralelo
   entre variants irmãos" N=4 → **5 cumulativo** (P156D HSpace+
   VSpace; P157C TableHeader+TableFooter; P231 outset Block+
   Boxed; P242 radius+clip Block+Boxed; **P247 fill+stroke
   Block+Boxed**).

5. **Tests construtores existentes adaptação automática**:
   `Content::block(...)` + `Content::boxed(...)` ganham +2 args
   em todas as callsites. Mudança mecânica via search-replace
   em ~10-20 sítios. **Adaptações N esperadas dentro range
   §1.4** (N=0-10).

6. **`fill` + `stroke` ambos None preservam output literal
   P246**. Test E2E baseline P156G/P156H/P231/P242 devem passar
   inalterados quando construtor explícito não passa fill/stroke
   (default `None`).

7. **Outset semantic implementação cuidadosa**: outset != Sides::ZERO
   expande bounds visuais SEM afectar cursor interno do body
   (paralelo a margin CSS). Cursor pre-body avança outset.top +
   inset.top; cursor pós-body avança outset.bottom + inset.bottom.
   Bounds Shape abrangem `[origin - outset, origin + size + outset]`.

8. **`P247.div-N` cenários antecipados em §2.9**. Activar se:
   - Cenário C outset (`P247.div-1`).
   - Table/Grid fill+stroke semantic incompatível com Block+Boxed
     (`P247.div-3`).
   - Baseline ≠ 2209 (`P247.div-4`).

9. **Anti-inflação 39ª aplicação cumulativa** pós-P205D
   preservar: Opção β L0 minimal (content.md hash propagado) +
   Opção α refino aditivo paralelo Block+Boxed + Opção α
   activação consumer real + Opção α reuso helpers stdlib +
   Opção α anotação cumulativa minimal ADRs + Opção α sub-padrão
   N=1 inaugurado anotado + Opção α 3 scope-outs fechados
   simultaneamente (paridade "agregar promoções" pattern N=1).
